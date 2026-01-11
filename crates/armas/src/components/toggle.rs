//! Toggle/Switch Components
//!
//! Animated toggle switches and checkboxes

use crate::animation::SpringAnimation;
use crate::layout::VStack;
use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{pos2, vec2, Color32, CornerRadius, Response, Sense, Stroke, Ui, Vec2};

/// Toggle switch variant
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToggleVariant {
    /// Standard toggle switch
    Switch,
    /// Checkbox style
    Checkbox,
}

/// Toggle size
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToggleSize {
    Small,
    Medium,
    Large,
}

impl ToggleSize {
    fn dimensions(&self, variant: ToggleVariant) -> (f32, f32) {
        match variant {
            ToggleVariant::Switch => match self {
                ToggleSize::Small => (36.0, 20.0),
                ToggleSize::Medium => (44.0, 24.0),
                ToggleSize::Large => (52.0, 28.0),
            },
            ToggleVariant::Checkbox => match self {
                ToggleSize::Small => (16.0, 16.0),
                ToggleSize::Medium => (20.0, 20.0),
                ToggleSize::Large => (24.0, 24.0),
            },
        }
    }
}

/// Animated toggle switch component
pub struct Toggle {
    variant: ToggleVariant,
    size: ToggleSize,
    label: Option<String>,
    description: Option<String>,
    disabled: bool,
    // Use spring animation for smooth, physics-based toggle animation
    toggle_spring: SpringAnimation,
}

impl Toggle {
    /// Create a new toggle
    pub fn new() -> Self {
        Self {
            variant: ToggleVariant::Switch,
            size: ToggleSize::Medium,
            label: None,
            description: None,
            disabled: false,
            // Smooth spring animation for natural toggle feel
            toggle_spring: SpringAnimation::new(0.0, 0.0).with_params(300.0, 30.0),
        }
    }

    /// Set the variant
    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the size
    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    /// Set a label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set a description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Show the toggle and return whether it changed
    pub fn show(&mut self, ui: &mut Ui, checked: &mut bool) -> ToggleResponse {
        let theme = ui.ctx().armas_theme();
        let old_checked = *checked;

        // Update spring animation to match checked state
        let target = if *checked { 1.0 } else { 0.0 };
        self.toggle_spring.set_target(target);

        let dt = ui.input(|i| i.stable_dt);
        self.toggle_spring.update(dt);

        // Request repaint while animating
        if !self.toggle_spring.is_settled(0.001, 0.001) {
            ui.ctx().request_repaint();
        }

        let response = ui
            .horizontal(|ui| {
                // Toggle control
                let (width, height) = self.size.dimensions(self.variant);
                let (rect, mut response) = ui.allocate_exact_size(
                    Vec2::new(width, height),
                    if self.disabled {
                        Sense::hover()
                    } else {
                        Sense::click()
                    },
                );

                if ui.is_rect_visible(rect) {
                    match self.variant {
                        ToggleVariant::Switch => {
                            self.draw_switch(ui, rect, *checked, &theme);
                        }
                        ToggleVariant::Checkbox => {
                            self.draw_checkbox(ui, rect, *checked, &theme);
                        }
                    }
                }

                // Handle click
                if response.clicked() && !self.disabled {
                    *checked = !*checked;
                    response.mark_changed();
                }

                // Label and description
                if self.label.is_some() || self.description.is_some() {
                    ui.add_space(theme.spacing.sm);
                    VStack::new(2.0).show(ui, |ui| {
                        if let Some(label) = &self.label {
                            let label_color = if self.disabled {
                                Color32::from_gray(100)
                            } else {
                                theme.on_surface()
                            };

                            ui.label(egui::RichText::new(label).size(14.0).color(label_color));
                        }

                        if let Some(description) = &self.description {
                            ui.label(
                                egui::RichText::new(description)
                                    .size(12.0)
                                    .color(Color32::from_gray(150)),
                            );
                        }
                    });
                }

                response
            })
            .inner;

        ToggleResponse {
            response,
            changed: old_checked != *checked,
        }
    }

    /// Draw a switch-style toggle
    fn draw_switch(&self, ui: &mut Ui, rect: egui::Rect, checked: bool, theme: &Theme) {
        let painter = ui.painter();
        let t = self.toggle_spring.value;

        // Background track
        let bg_color = if self.disabled {
            Color32::from_gray(60)
        } else if checked {
            let primary = theme.primary();
            Color32::from_rgba_unmultiplied(
                primary.r(),
                primary.g(),
                primary.b(),
                (200.0 + 55.0 * t) as u8,
            )
        } else {
            Color32::from_gray(80)
        };

        let track_radius = rect.height() / 2.0;
        painter.rect_filled(rect, CornerRadius::same(track_radius as u8), bg_color);

        // Border
        if !checked && !self.disabled {
            painter.rect_stroke(
                rect,
                CornerRadius::same(track_radius as u8),
                Stroke::new(1.0, Color32::from_gray(100)),
                egui::StrokeKind::Outside,
            );
        }

        // Thumb (sliding circle)
        let thumb_radius = (rect.height() - 4.0) / 2.0;
        let thumb_padding = 2.0;
        let thumb_travel = rect.width() - rect.height();
        let thumb_x = rect.min.x + thumb_padding + thumb_radius + thumb_travel * t;
        let thumb_center = pos2(thumb_x, rect.center().y);

        let thumb_color = if self.disabled {
            Color32::from_gray(150)
        } else {
            Color32::WHITE
        };

        painter.circle_filled(thumb_center, thumb_radius, thumb_color);

        // Thumb shadow
        if !self.disabled {
            painter.circle_filled(
                thumb_center + vec2(0.0, 1.0),
                thumb_radius,
                Color32::from_rgba_unmultiplied(0, 0, 0, 30),
            );
        }
    }

    /// Draw a checkbox-style toggle
    fn draw_checkbox(&self, ui: &mut Ui, rect: egui::Rect, checked: bool, theme: &Theme) {
        let painter = ui.painter();
        let t = self.toggle_spring.value;

        // Background
        let bg_color = if self.disabled {
            Color32::from_gray(60)
        } else if checked {
            let primary = theme.primary();
            Color32::from_rgba_unmultiplied(
                primary.r(),
                primary.g(),
                primary.b(),
                (200.0 + 55.0 * t) as u8,
            )
        } else {
            Color32::from_gray(40)
        };

        painter.rect_filled(rect, CornerRadius::same(theme.spacing.xs as u8), bg_color);

        // Border
        let border_color = if self.disabled {
            Color32::from_gray(80)
        } else if checked {
            theme.primary()
        } else {
            Color32::from_gray(100)
        };

        painter.rect_stroke(
            rect,
            CornerRadius::same(theme.spacing.xs as u8),
            Stroke::new(1.5, border_color),
            egui::StrokeKind::Outside,
        );

        // Checkmark
        if t > 0.0 {
            let scale = t; // Animate the checkmark appearance
            let center = rect.center();
            let size = rect.height() * 0.6 * scale;

            // Draw checkmark as two lines
            let check_start = center + vec2(-size * 0.3, 0.0);
            let check_middle = center + vec2(-size * 0.1, size * 0.3);
            let check_end = center + vec2(size * 0.4, -size * 0.4);

            let check_color = if self.disabled {
                Color32::from_gray(150)
            } else {
                Color32::WHITE
            };

            painter.line_segment([check_start, check_middle], Stroke::new(2.0, check_color));
            painter.line_segment([check_middle, check_end], Stroke::new(2.0, check_color));
        }
    }
}

impl Default for Toggle {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from toggle interaction
pub struct ToggleResponse {
    /// The underlying egui response
    pub response: Response,
    /// Whether the toggle state changed
    pub changed: bool,
}

/// Toggle group for managing multiple related toggles
pub struct ToggleGroup {
    label: Option<String>,
    items: Vec<ToggleGroupItem>,
}

struct ToggleGroupItem {
    id: String,
    toggle: Toggle,
    checked: bool,
}

impl ToggleGroup {
    /// Create a new toggle group
    pub fn new() -> Self {
        Self {
            label: None,
            items: Vec::new(),
        }
    }

    /// Set a label for the group
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Add a toggle to the group
    pub fn add_toggle(
        mut self,
        id: impl Into<String>,
        toggle: Toggle,
        default_checked: bool,
    ) -> Self {
        self.items.push(ToggleGroupItem {
            id: id.into(),
            toggle,
            checked: default_checked,
        });
        self
    }

    /// Show the toggle group
    pub fn show(&mut self, ui: &mut Ui) -> ToggleGroupResponse {
        let theme = ui.ctx().armas_theme();
        let mut changed = Vec::new();

        VStack::new(theme.spacing.sm).show(ui, |ui| {
            // Group label
            if let Some(label) = &self.label {
                ui.label(
                    egui::RichText::new(label)
                        .size(14.0)
                        .strong()
                        .color(theme.on_surface()),
                );
            }

            // Show each toggle
            for item in &mut self.items {
                let old_checked = item.checked;
                item.toggle.show(ui, &mut item.checked);

                if old_checked != item.checked {
                    changed.push((item.id.clone(), item.checked));
                }

                ui.add_space(theme.spacing.xs);
            }
        });

        ToggleGroupResponse { changed }
    }

    /// Get the current state of all toggles
    pub fn get_state(&self) -> Vec<(String, bool)> {
        self.items
            .iter()
            .map(|item| (item.id.clone(), item.checked))
            .collect()
    }

    /// Set the state of a specific toggle
    pub fn set_state(&mut self, id: &str, checked: bool) {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.checked = checked;
        }
    }
}

impl Default for ToggleGroup {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from toggle group
pub struct ToggleGroupResponse {
    /// List of toggles that changed: (id, new_state)
    pub changed: Vec<(String, bool)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_creation() {
        let toggle = Toggle::new();
        assert_eq!(toggle.variant, ToggleVariant::Switch);
        assert_eq!(toggle.size, ToggleSize::Medium);
        assert!(!toggle.disabled);
    }

    #[test]
    fn test_toggle_builder() {
        let toggle = Toggle::new()
            .variant(ToggleVariant::Checkbox)
            .size(ToggleSize::Large)
            .with_label("Enable feature")
            .disabled(true);

        assert_eq!(toggle.variant, ToggleVariant::Checkbox);
        assert_eq!(toggle.size, ToggleSize::Large);
        assert_eq!(toggle.label, Some("Enable feature".to_string()));
        assert!(toggle.disabled);
    }

    #[test]
    fn test_toggle_size_dimensions() {
        assert_eq!(
            ToggleSize::Small.dimensions(ToggleVariant::Switch),
            (36.0, 20.0)
        );
        assert_eq!(
            ToggleSize::Medium.dimensions(ToggleVariant::Checkbox),
            (20.0, 20.0)
        );
    }

    #[test]
    fn test_toggle_group() {
        let mut group = ToggleGroup::new()
            .with_label("Settings")
            .add_toggle("option1", Toggle::new().with_label("Option 1"), false)
            .add_toggle("option2", Toggle::new().with_label("Option 2"), true);

        let state = group.get_state();
        assert_eq!(state.len(), 2);
        assert_eq!(state[0], ("option1".to_string(), false));
        assert_eq!(state[1], ("option2".to_string(), true));

        group.set_state("option1", true);
        let state = group.get_state();
        assert_eq!(state[0], ("option1".to_string(), true));
    }
}
