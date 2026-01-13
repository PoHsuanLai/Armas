//! Chip Component
//!
//! Material Design 3 chips for actions, filters, and selections

use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{Color32, Response, Sense, Stroke, StrokeKind, Ui, Vec2};

/// Material Design 3 chip types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChipType {
    /// Assist chips for common actions (e.g., "Add to cart", "Share")
    Assist,
    /// Filter chips for refinement (e.g., "Size: M", "Color: Red")
    Filter,
    /// Input chips for user-entered content (e.g., tags, recipients)
    Input,
    /// Suggestion chips for recommendations (e.g., "Trending", "Popular")
    Suggestion,
}

/// Chip size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChipSize {
    Small,
    Medium,
    Large,
}

impl ChipSize {
    fn height(&self) -> f32 {
        match self {
            ChipSize::Small => 24.0,
            ChipSize::Medium => 32.0,
            ChipSize::Large => 40.0,
        }
    }

    fn font_size(&self) -> f32 {
        match self {
            ChipSize::Small => 12.0,
            ChipSize::Medium => 14.0,
            ChipSize::Large => 16.0,
        }
    }

    fn padding_x(&self) -> f32 {
        match self {
            ChipSize::Small => 8.0,
            ChipSize::Medium => 12.0,
            ChipSize::Large => 16.0,
        }
    }
}

/// Material Design 3 Chip component
///
/// Compact elements that represent an input, attribute, or action.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::components::{Chip, ChipType};
///
/// // Assist chip
/// if Chip::new("Add to cart")
///     .chip_type(ChipType::Assist)
///     .show(ui).clicked() {
///     // Handle action
/// }
///
/// // Filter chip with selection
/// let mut selected = true;
/// Chip::new("Size: M")
///     .chip_type(ChipType::Filter)
///     .selected(selected)
///     .show(ui);
///
/// // Input chip with remove
/// if Chip::new("user@email.com")
///     .chip_type(ChipType::Input)
///     .removable(true)
///     .show(ui).clicked() {
///     // Handle remove
/// }
/// # }
/// ```
pub struct Chip {
    label: String,
    chip_type: ChipType,
    size: ChipSize,
    icon: Option<String>,
    selected: bool,
    removable: bool,
    disabled: bool,
}

impl Chip {
    /// Create a new chip
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            chip_type: ChipType::Assist,
            size: ChipSize::Medium,
            icon: None,
            selected: false,
            removable: false,
            disabled: false,
        }
    }

    /// Set the chip type
    pub fn chip_type(mut self, chip_type: ChipType) -> Self {
        self.chip_type = chip_type;
        self
    }

    /// Set the chip size
    pub fn size(mut self, size: ChipSize) -> Self {
        self.size = size;
        self
    }

    /// Set an icon (emoji or unicode symbol)
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set selected state (for Filter chips)
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set removable state (for Input chips)
    pub fn removable(mut self, removable: bool) -> Self {
        self.removable = removable;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Show the chip
    pub fn show(self, ui: &mut Ui) -> ChipResponse {
        let theme = ui.ctx().armas_theme();
        let height = self.size.height();
        let font_size = self.size.font_size();
        let padding_x = self.size.padding_x();

        // Calculate content width
        let mut content_width = 0.0;

        // Icon space
        if self.icon.is_some() {
            content_width += font_size + 4.0;
        }

        // Label space
        let label_galley = ui.painter().layout_no_wrap(
            self.label.clone(),
            egui::FontId::proportional(font_size),
            theme.on_surface(),
        );
        content_width += label_galley.size().x;

        // Remove button space (for removable chips)
        if self.removable {
            content_width += font_size + 4.0;
        }

        let total_width = content_width + padding_x * 2.0;
        let desired_size = Vec2::new(total_width, height);

        let (rect, mut response) = ui.allocate_exact_size(
            desired_size,
            if self.disabled {
                Sense::hover()
            } else {
                Sense::click()
            },
        );

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);

            // Determine colors based on chip type and state
            let (bg_color, border_color, text_color) = self.get_colors(&theme, visuals);

            // Draw background
            let rounding = height / 2.0; // Fully rounded ends
            ui.painter().rect_filled(rect, rounding, bg_color);

            // Draw border for outlined variants
            if self.needs_border() {
                ui.painter().rect_stroke(
                    rect,
                    rounding,
                    Stroke::new(1.0, border_color),
                    StrokeKind::Outside,
                );
            }

            // Draw content
            let mut cursor_x = rect.min.x + padding_x;
            let center_y = rect.center().y;

            // Icon
            if let Some(icon) = &self.icon {
                ui.painter().text(
                    egui::pos2(cursor_x, center_y),
                    egui::Align2::LEFT_CENTER,
                    icon,
                    egui::FontId::proportional(font_size),
                    text_color,
                );
                cursor_x += font_size + 4.0;
            }

            // Label
            ui.painter().text(
                egui::pos2(cursor_x, center_y),
                egui::Align2::LEFT_CENTER,
                &self.label,
                egui::FontId::proportional(font_size),
                text_color,
            );
            cursor_x += label_galley.size().x;

            // Remove button (X)
            let remove_clicked = if self.removable {
                cursor_x += 4.0;
                let remove_size = font_size;
                let remove_rect = egui::Rect::from_center_size(
                    egui::pos2(cursor_x + remove_size / 2.0, center_y),
                    Vec2::splat(remove_size),
                );

                let remove_response =
                    ui.interact(remove_rect, response.id.with("remove"), Sense::click());

                // Draw X
                let x_color = if remove_response.hovered() {
                    text_color.linear_multiply(1.2)
                } else {
                    text_color.linear_multiply(0.8)
                };

                ui.painter().text(
                    remove_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "✕",
                    egui::FontId::proportional(font_size * 0.9),
                    x_color,
                );

                if remove_response.clicked() {
                    response.mark_changed();
                }

                remove_response.clicked()
            } else {
                false
            };

            let clicked = response.clicked();

            ChipResponse {
                response,
                clicked,
                remove_clicked,
            }
        } else {
            ChipResponse {
                response,
                clicked: false,
                remove_clicked: false,
            }
        }
    }

    /// Get colors based on chip type and state
    fn get_colors(
        &self,
        theme: &Theme,
        visuals: &egui::style::WidgetVisuals,
    ) -> (Color32, Color32, Color32) {
        if self.disabled {
            return (
                theme.surface_variant().linear_multiply(0.5),
                theme.outline_variant().linear_multiply(0.5),
                theme.on_surface().linear_multiply(0.4),
            );
        }

        match self.chip_type {
            ChipType::Assist => {
                // Elevated style with subtle border
                let bg = if self.selected {
                    theme.primary().linear_multiply(0.2)
                } else if visuals.bg_fill != Color32::TRANSPARENT {
                    theme.surface_variant().linear_multiply(1.1)
                } else {
                    theme.surface_variant()
                };
                let border = theme.outline_variant();
                let text = if self.selected {
                    theme.primary()
                } else {
                    theme.on_surface()
                };
                (bg, border, text)
            }
            ChipType::Filter => {
                // Toggleable style
                if self.selected {
                    (
                        theme.primary().linear_multiply(0.2),
                        theme.primary().linear_multiply(0.2),
                        theme.primary(),
                    )
                } else {
                    (theme.surface(), theme.outline(), theme.on_surface_variant())
                }
            }
            ChipType::Input => {
                // Similar to Assist but with remove button
                let bg = if visuals.bg_fill != Color32::TRANSPARENT {
                    theme.surface_variant().linear_multiply(1.1)
                } else {
                    theme.surface_variant()
                };
                (bg, theme.outline_variant(), theme.on_surface())
            }
            ChipType::Suggestion => {
                // Outlined style
                (theme.surface(), theme.outline(), theme.on_surface_variant())
            }
        }
    }

    /// Check if chip needs a border
    fn needs_border(&self) -> bool {
        matches!(self.chip_type, ChipType::Filter | ChipType::Suggestion) && !self.selected
            || matches!(self.chip_type, ChipType::Assist | ChipType::Input)
    }
}

/// Response from a chip interaction
pub struct ChipResponse {
    /// The underlying egui response
    pub response: Response,
    /// Whether the chip was clicked
    pub clicked: bool,
    /// Whether the remove button was clicked (for removable chips)
    pub remove_clicked: bool,
}

impl ChipResponse {
    /// Check if the chip was clicked
    pub fn clicked(&self) -> bool {
        self.clicked
    }

    /// Check if the remove button was clicked
    pub fn remove_clicked(&self) -> bool {
        self.remove_clicked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip_creation() {
        let chip = Chip::new("Test");
        assert_eq!(chip.label, "Test");
        assert_eq!(chip.chip_type, ChipType::Assist);
        assert!(!chip.selected);
        assert!(!chip.removable);
    }

    #[test]
    fn test_chip_builder() {
        let chip = Chip::new("Filter")
            .chip_type(ChipType::Filter)
            .selected(true)
            .icon("🔍");

        assert_eq!(chip.chip_type, ChipType::Filter);
        assert!(chip.selected);
        assert_eq!(chip.icon, Some("🔍".to_string()));
    }

    #[test]
    fn test_chip_sizes() {
        assert_eq!(ChipSize::Small.height(), 24.0);
        assert_eq!(ChipSize::Medium.height(), 32.0);
        assert_eq!(ChipSize::Large.height(), 40.0);
    }
}
