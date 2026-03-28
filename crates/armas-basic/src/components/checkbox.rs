//! Checkbox Component
//!
//! A standalone checkbox styled like shadcn/ui Checkbox.
//! 16x16px square with 4px rounded corners, animated checkmark.

use crate::animation::SpringAnimation;
use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{vec2, Color32, CornerRadius, Response, Sense, Stroke, Ui, Vec2};

const SIZE: f32 = 16.0;
const CORNER_RADIUS: u8 = 4;

/// Response from checkbox interaction
pub struct CheckboxResponse {
    /// The underlying egui response
    pub response: Response,
    /// Whether the checkbox state changed
    pub changed: bool,
}

/// A checkbox component (shadcn/ui Checkbox)
///
/// # Example
///
/// ```ignore
/// let mut checked = false;
/// Checkbox::new()
///     .label("Accept terms")
///     .show(ui, &mut checked);
/// ```
pub struct Checkbox {
    id: Option<egui::Id>,
    label: Option<String>,
    description: Option<String>,
    disabled: bool,
    check_spring: SpringAnimation,
}

impl Checkbox {
    /// Create a new checkbox
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: None,
            label: None,
            description: None,
            disabled: false,
            check_spring: SpringAnimation::new(0.0, 0.0).params(800.0, 30.0),
        }
    }

    /// Set ID for state persistence
    #[must_use]
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
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

    /// Show the checkbox
    pub fn show(&mut self, ui: &mut Ui, checked: &mut bool) -> CheckboxResponse {
        let theme = ui.ctx().armas_theme();

        // Load state from memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("checkbox_state");
            let (stored_checked, stored_anim): (bool, f32) = ui.ctx().data_mut(|d| {
                d.get_temp(state_id)
                    .unwrap_or((*checked, if *checked { 1.0 } else { 0.0 }))
            });
            *checked = stored_checked;
            self.check_spring.value = stored_anim;
        }

        let old_checked = *checked;

        // Without an ID, the spring resets to 0.0 every frame.
        // Snap it to match `checked` so it only animates on actual toggles.
        if self.id.is_none() {
            self.check_spring.value = if *checked { 1.0 } else { 0.0 };
        }

        // Update spring animation
        let target = if *checked { 1.0 } else { 0.0 };
        self.check_spring.set_target(target);
        let dt = ui.input(|i| i.stable_dt);
        self.check_spring.update(dt);

        if !self.check_spring.is_settled(0.001, 0.001) {
            ui.ctx().request_repaint();
        }

        let response = ui
            .horizontal(|ui| {
                let (rect, mut response) = ui.allocate_exact_size(
                    Vec2::splat(SIZE),
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
            let state_id = id.with("checkbox_state");
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id, (*checked, self.check_spring.value));
            });
        }

        CheckboxResponse {
            response,
            changed: old_checked != *checked,
        }
    }

    fn draw(&self, ui: &mut Ui, rect: egui::Rect, checked: bool, theme: &Theme) {
        let painter = ui.painter();
        let t = self.check_spring.value;

        // Background — primary when checked, transparent when unchecked
        let bg_color = if self.disabled {
            theme.muted().gamma_multiply(0.5)
        } else if checked {
            theme.primary()
        } else {
            Color32::TRANSPARENT
        };

        painter.rect_filled(rect, CornerRadius::same(CORNER_RADIUS), bg_color);

        // Border
        let border_color = if self.disabled {
            theme.border()
        } else if checked {
            theme.primary()
        } else {
            theme.input()
        };

        painter.rect_stroke(
            rect,
            CornerRadius::same(CORNER_RADIUS),
            Stroke::new(1.0, border_color),
            egui::StrokeKind::Inside,
        );

        // Focus ring on hover
        let hover_response = ui.interact(rect, ui.id().with("cb_hover"), Sense::hover());
        if hover_response.hovered() && !self.disabled {
            painter.rect_stroke(
                rect.expand(2.0),
                CornerRadius::same(CORNER_RADIUS + 2),
                Stroke::new(2.0, theme.ring()),
                egui::StrokeKind::Outside,
            );
        }

        // Checkmark
        if t > 0.0 {
            let center = rect.center();
            let size = rect.height() * 0.5 * t;

            let check_start = center + vec2(-size * 0.35, 0.0);
            let check_middle = center + vec2(-size * 0.05, size * 0.3);
            let check_end = center + vec2(size * 0.35, -size * 0.35);

            let check_color = if self.disabled {
                theme.muted_foreground()
            } else {
                theme.primary_foreground()
            };

            painter.line_segment([check_start, check_middle], Stroke::new(1.5, check_color));
            painter.line_segment([check_middle, check_end], Stroke::new(1.5, check_color));
        }
    }
}

impl Default for Checkbox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_creation() {
        let cb = Checkbox::new();
        assert!(!cb.disabled);
        assert!(cb.label.is_none());
    }

    #[test]
    fn test_checkbox_builder() {
        let cb = Checkbox::new()
            .label("Accept terms")
            .description("Required")
            .disabled(true);

        assert_eq!(cb.label, Some("Accept terms".to_string()));
        assert_eq!(cb.description, Some("Required".to_string()));
        assert!(cb.disabled);
    }
}
