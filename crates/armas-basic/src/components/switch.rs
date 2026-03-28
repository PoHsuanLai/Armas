//! Switch Component
//!
//! Animated toggle switch styled like shadcn/ui Switch.
//! Provides smooth spring animations and supports:
//! - Multiple sizes
//! - Labels and descriptions
//! - Disabled state

use crate::animation::SpringAnimation;
use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{pos2, vec2, Color32, CornerRadius, Response, Sense, Stroke, Ui, Vec2};

// shadcn Switch dimensions
const SWITCH_WIDTH: f32 = 44.0; // w-11
const SWITCH_HEIGHT: f32 = 24.0; // h-6
const SWITCH_THUMB_SIZE: f32 = 20.0; // h-5 w-5

/// Switch size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchSize {
    /// Small switch
    Small,
    /// Medium switch (default)
    Medium,
    /// Large switch
    Large,
}

impl SwitchSize {
    const fn dimensions(self) -> (f32, f32) {
        match self {
            Self::Small => (36.0, 20.0),
            Self::Medium => (SWITCH_WIDTH, SWITCH_HEIGHT),
            Self::Large => (52.0, 28.0),
        }
    }

    const fn thumb_size(self) -> f32 {
        match self {
            Self::Small => 16.0,
            Self::Medium => SWITCH_THUMB_SIZE,
            Self::Large => 24.0,
        }
    }
}

/// Animated switch component (shadcn/ui Switch)
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas_basic::components::Switch;
///
/// let mut checked = false;
/// let mut switch = Switch::new().label("Dark mode");
/// switch.show(ui, &mut checked);
/// # }
/// ```
pub struct Switch {
    id: Option<egui::Id>,
    size: SwitchSize,
    label: Option<String>,
    description: Option<String>,
    disabled: bool,
    toggle_spring: SpringAnimation,
}

impl Switch {
    /// Create a new switch
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: None,
            size: SwitchSize::Medium,
            label: None,
            description: None,
            disabled: false,
            toggle_spring: SpringAnimation::new(0.0, 0.0).params(800.0, 30.0),
        }
    }

    /// Set ID for state persistence
    #[must_use]
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the size
    #[must_use]
    pub const fn size(mut self, size: SwitchSize) -> Self {
        self.size = size;
        self
    }

    /// Set a label
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set a description
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set disabled state
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Show the switch and return whether it changed
    pub fn show(&mut self, ui: &mut Ui, checked: &mut bool) -> SwitchResponse {
        let theme = ui.ctx().armas_theme();

        // Load state from memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("switch_state");
            let (stored_checked, stored_anim): (bool, f32) = ui.ctx().data_mut(|d| {
                d.get_temp(state_id)
                    .unwrap_or((*checked, if *checked { 1.0 } else { 0.0 }))
            });
            *checked = stored_checked;
            self.toggle_spring.value = stored_anim;
        }

        let old_checked = *checked;

        // Without an ID, the spring resets to 0.0 every frame (no stored state).
        // Snap it to match `checked` so it only animates on actual toggles.
        if self.id.is_none() {
            self.toggle_spring.value = if *checked { 1.0 } else { 0.0 };
        }

        let target = if *checked { 1.0 } else { 0.0 };
        self.toggle_spring.set_target(target);
        let dt = ui.input(|i| i.stable_dt);
        self.toggle_spring.update(dt);

        if !self.toggle_spring.is_settled(0.001, 0.001) {
            ui.ctx().request_repaint();
        }

        let response = ui
            .horizontal(|ui| {
                let (width, height) = self.size.dimensions();
                let (rect, mut response) = ui.allocate_exact_size(
                    Vec2::new(width, height),
                    if self.disabled {
                        Sense::hover()
                    } else {
                        Sense::click()
                    },
                );

                if ui.is_rect_visible(rect) {
                    self.draw(ui, rect, *checked, &theme);
                }

                if response.clicked() && !self.disabled {
                    *checked = !*checked;
                    response.mark_changed();
                }

                // Label and description
                if self.label.is_some() || self.description.is_some() {
                    ui.add_space(theme.spacing.sm);
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = theme.spacing.xs;
                        if let Some(label) = &self.label {
                            let color = if self.disabled {
                                theme.muted_foreground().linear_multiply(0.5)
                            } else {
                                theme.foreground()
                            };
                            ui.label(
                                egui::RichText::new(label)
                                    .size(theme.typography.base)
                                    .color(color),
                            );
                        }
                        if let Some(desc) = &self.description {
                            ui.label(
                                egui::RichText::new(desc)
                                    .size(theme.typography.sm)
                                    .color(theme.muted_foreground()),
                            );
                        }
                    });
                }

                response
            })
            .inner;

        // Save state
        if let Some(id) = self.id {
            let state_id = id.with("switch_state");
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id, (*checked, self.toggle_spring.value));
            });
        }

        SwitchResponse {
            response,
            changed: old_checked != *checked,
        }
    }

    fn draw(&self, ui: &mut Ui, rect: egui::Rect, checked: bool, theme: &Theme) {
        let painter = ui.painter();
        let t = self.toggle_spring.value;

        // Background track
        let bg_color = if self.disabled {
            theme.muted().gamma_multiply(0.5)
        } else if checked {
            theme.primary()
        } else {
            theme.input()
        };

        let track_radius = rect.height() / 2.0;
        painter.rect_filled(rect, CornerRadius::same(track_radius as u8), bg_color);

        // Focus ring on hover
        let response = ui.interact(rect, ui.id().with("switch_hover"), Sense::hover());
        if response.hovered() && !self.disabled {
            painter.rect_stroke(
                rect.expand(2.0),
                CornerRadius::same((track_radius + 2.0) as u8),
                Stroke::new(2.0, theme.ring()),
                egui::StrokeKind::Outside,
            );
        }

        // Thumb
        let thumb_size = self.size.thumb_size();
        let thumb_radius = thumb_size / 2.0;
        let thumb_padding = 2.0;
        let thumb_travel = rect.width() - thumb_size - thumb_padding * 2.0;
        let thumb_x = rect.min.x + thumb_padding + thumb_radius + thumb_travel * t;
        let thumb_center = pos2(thumb_x, rect.center().y);

        let thumb_color = if self.disabled {
            theme.muted_foreground()
        } else {
            theme.foreground()
        };

        // Shadow under thumb
        if !self.disabled {
            painter.circle_filled(
                thumb_center + vec2(0.0, 1.0),
                thumb_radius,
                Color32::from_rgba_unmultiplied(0, 0, 0, 20),
            );
        }

        painter.circle_filled(thumb_center, thumb_radius, thumb_color);
    }
}

impl Default for Switch {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from switch interaction
pub struct SwitchResponse {
    /// The underlying egui response
    pub response: Response,
    /// Whether the switch state changed
    pub changed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_creation() {
        let switch = Switch::new();
        assert_eq!(switch.size, SwitchSize::Medium);
        assert!(!switch.disabled);
    }

    #[test]
    fn test_switch_builder() {
        let switch = Switch::new()
            .size(SwitchSize::Large)
            .label("Enable feature")
            .disabled(true);

        assert_eq!(switch.size, SwitchSize::Large);
        assert_eq!(switch.label, Some("Enable feature".to_string()));
        assert!(switch.disabled);
    }

    #[test]
    fn test_switch_size_dimensions() {
        assert_eq!(SwitchSize::Small.dimensions(), (36.0, 20.0));
        assert_eq!(SwitchSize::Medium.dimensions(), (44.0, 24.0));
        assert_eq!(SwitchSize::Large.dimensions(), (52.0, 28.0));
    }
}
