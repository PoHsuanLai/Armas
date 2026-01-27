//! Radio Button Components
//!
//! Radio buttons styled like shadcn/ui RadioGroup.
//! For single selection from a group of options.
//!
//! # Example
//!
//! ```rust,no_run
//! # use egui::Ui;
//! # fn example(ui: &mut Ui) {
//! use armas::components::{Radio, RadioGroup};
//!
//! // Single radio
//! Radio::new().label("Option").show(ui, true);
//!
//! // Radio group
//! let mut selected = Some("opt1".to_string());
//! RadioGroup::new(&mut selected)
//!     .label("Choose one")
//!     .show(ui, |group| {
//!         group.option("opt1", "First");
//!         group.option("opt2", "Second");
//!     });
//! # }
//! ```

use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{Response, Sense, Stroke, Ui, Vec2};

// shadcn RadioGroup constants
const RADIO_SIZE: f32 = 16.0; // h-4 w-4 (default)
const RADIO_SIZE_SM: f32 = 14.0; // Small variant
const RADIO_SIZE_LG: f32 = 20.0; // Large variant
const BORDER_WIDTH: f32 = 1.0; // border
const INNER_CIRCLE_RATIO: f32 = 0.5; // Inner dot is 50% of outer
const LABEL_FONT_SIZE: f32 = 14.0; // text-sm
const DESCRIPTION_FONT_SIZE: f32 = 12.0; // text-xs

/// Radio button size
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RadioSize {
    /// Small radio button
    Small,
    /// Medium radio button (default)
    Medium,
    /// Large radio button
    Large,
}

impl RadioSize {
    fn diameter(&self) -> f32 {
        match self {
            RadioSize::Small => RADIO_SIZE_SM,
            RadioSize::Medium => RADIO_SIZE,
            RadioSize::Large => RADIO_SIZE_LG,
        }
    }
}

/// Individual radio button
pub struct Radio {
    id: Option<egui::Id>,
    size: RadioSize,
    label: Option<String>,
    description: Option<String>,
    disabled: bool,
}

impl Radio {
    /// Create a new radio button
    pub fn new() -> Self {
        Self {
            id: None,
            size: RadioSize::Medium,
            label: None,
            description: None,
            disabled: false,
        }
    }

    /// Set ID for state persistence
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the size
    pub fn size(mut self, size: RadioSize) -> Self {
        self.size = size;
        self
    }

    /// Set a label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set a description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Show the radio button
    pub fn show(&self, ui: &mut Ui, selected: bool, theme: &crate::Theme) -> RadioResponse {

        let response = ui
            .horizontal(|ui| {
                // Radio control
                let diameter = self.size.diameter();
                let (rect, response) = ui.allocate_exact_size(
                    Vec2::splat(diameter),
                    if self.disabled {
                        Sense::hover()
                    } else {
                        Sense::click()
                    },
                );

                if ui.is_rect_visible(rect) {
                    self.draw_radio(ui, rect, selected, &theme);
                }

                // Label and description
                if self.label.is_some() || self.description.is_some() {
                    ui.add_space(theme.spacing.sm);
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = theme.spacing.xs;
                        if let Some(label) = &self.label {
                            let label_color = if self.disabled {
                                theme.muted_foreground().linear_multiply(0.5)
                            } else {
                                theme.foreground()
                            };

                            ui.label(egui::RichText::new(label).size(LABEL_FONT_SIZE).color(label_color));
                        }

                        if let Some(description) = &self.description {
                            ui.label(
                                egui::RichText::new(description)
                                    .size(DESCRIPTION_FONT_SIZE)
                                    .color(theme.muted_foreground()),
                            );
                        }
                    });
                }

                response
            })
            .inner;

        RadioResponse { response, selected }
    }

    /// Draw the radio button circle (shadcn style)
    fn draw_radio(&self, ui: &mut Ui, rect: egui::Rect, selected: bool, theme: &Theme) {
        let painter = ui.painter();
        let center = rect.center();
        let radius = rect.width() / 2.0;

        // Outer circle border (shadcn uses primary color when selected)
        let border_color = if self.disabled {
            theme.muted_foreground().gamma_multiply(0.5)
        } else if selected {
            theme.primary()
        } else {
            theme.primary() // shadcn uses primary for unselected border too
        };

        painter.circle_stroke(center, radius, Stroke::new(BORDER_WIDTH, border_color));

        // Inner filled circle when selected (shadcn indicator)
        if selected {
            let inner_radius = radius * INNER_CIRCLE_RATIO;
            let fill_color = if self.disabled {
                theme.muted_foreground().gamma_multiply(0.5)
            } else {
                theme.primary()
            };

            painter.circle_filled(center, inner_radius, fill_color);
        }
    }
}

impl Default for Radio {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from radio button interaction
pub struct RadioResponse {
    /// The underlying egui response
    pub response: Response,
    /// Whether this radio is currently selected
    pub selected: bool,
}

// ============================================================================
// RADIO GROUP - CLOSURE-BASED API
// ============================================================================

/// Builder for configuring individual radio options
pub struct RadioOptionBuilder {
    label: String,
    description: Option<String>,
}

impl RadioOptionBuilder {
    fn new(_value: String, label: String) -> Self {
        Self {
            label,
            description: None,
        }
    }

    /// Set option description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Builder for adding radio options to the group
pub struct RadioGroupBuilder<'a> {
    selected_value: &'a mut Option<String>,
    ui: &'a mut Ui,
    changed: &'a mut bool,
    disabled: bool,
}

impl<'a> RadioGroupBuilder<'a> {
    /// Add a radio option to the group
    pub fn option(&mut self, value: &str, label: &str) -> RadioOptionBuilder {
        let builder = RadioOptionBuilder::new(value.to_string(), label.to_string());

        // Check if this option is currently selected
        let is_selected = self
            .selected_value
            .as_ref()
            .map(|v| v == value)
            .unwrap_or(false);

        // Create radio and show it
        let mut radio = Radio::new().label(&builder.label).disabled(self.disabled);

        if let Some(desc) = &builder.description {
            radio = radio.description(desc);
        }

        let theme = self.ui.ctx().armas_theme();
        let response = radio.show(self.ui, is_selected, &theme);

        // Update selection if clicked and not already selected
        if response.response.clicked() && !is_selected {
            *self.selected_value = Some(value.to_string());
            *self.changed = true;
        }

        builder
    }
}

/// Response from radio group
pub struct RadioGroupResponse {
    /// Whether the selection changed
    pub changed: bool,
    /// The currently selected value (if any)
    pub selected: Option<String>,
}

/// Radio group for single selection from multiple options
///
/// # Example
///
/// ```ignore
/// let mut selected = Some("option1".to_string());
/// let response = RadioGroup::new(&mut selected)
///     .label("Choose one")
///     .show(ui, |group| {
///         group.option("option1", "First Option");
///         group.option("option2", "Second Option")
///             .description("This is the second option");
///         group.option("option3", "Third Option");
///     });
///
/// if response.changed {
///     println!("Selected: {:?}", response.selected);
/// }
/// ```
pub struct RadioGroup<'a> {
    selected_value: &'a mut Option<String>,
    label: Option<String>,
    disabled: bool,
}

impl<'a> RadioGroup<'a> {
    /// Create a new radio group
    ///
    /// # Arguments
    /// * `selected_value` - Mutable reference to the currently selected value
    pub fn new(selected_value: &'a mut Option<String>) -> Self {
        Self {
            selected_value,
            label: None,
            disabled: false,
        }
    }

    /// Set a label for the group
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set disabled state for all radios in the group
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Show the radio group with closure-based API
    pub fn show<R>(
        self,
        ui: &mut Ui,
        content: impl FnOnce(&mut RadioGroupBuilder) -> R,
    ) -> RadioGroupResponse {
        let theme = ui.ctx().armas_theme();
        let mut changed = false;

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = theme.spacing.sm;

            // Group label
            if let Some(label) = &self.label {
                ui.label(
                    egui::RichText::new(label)
                        .size(LABEL_FONT_SIZE)
                        .strong()
                        .color(theme.foreground()),
                );
                ui.add_space(theme.spacing.xs);
            }

            // Build radio options from closure
            let mut builder = RadioGroupBuilder {
                selected_value: self.selected_value,
                ui,
                changed: &mut changed,
                disabled: self.disabled,
            };
            content(&mut builder);
        });

        RadioGroupResponse {
            changed,
            selected: self.selected_value.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radio_creation() {
        let radio = Radio::new();
        assert_eq!(radio.size, RadioSize::Medium);
        assert!(!radio.disabled);
    }

    #[test]
    fn test_radio_builder() {
        let radio = Radio::new()
            .size(RadioSize::Large)
            .label("Select me")
            .disabled(true);

        assert_eq!(radio.size, RadioSize::Large);
        assert_eq!(radio.label, Some("Select me".to_string()));
        assert!(radio.disabled);
    }

    #[test]
    fn test_radio_size_dimensions() {
        assert_eq!(RadioSize::Small.diameter(), RADIO_SIZE_SM);
        assert_eq!(RadioSize::Medium.diameter(), RADIO_SIZE);
        assert_eq!(RadioSize::Large.diameter(), RADIO_SIZE_LG);
    }
}
