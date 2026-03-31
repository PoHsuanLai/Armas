//! Number Field Component
//!
//! Numeric input with increment/decrement stepper buttons, styled like shadcn/vue `NumberField`.
//! Features:
//! - +/- stepper buttons
//! - Min/max value constraints
//! - Configurable step size
//! - Direct text editing
//! - Labels and descriptions
//! - Disabled state

use crate::ext::ArmasContextExt;
use egui::{pos2, vec2, Color32, CornerRadius, Response, Sense, Stroke, Ui, Vec2};

// shadcn NumberField constants
const HEIGHT: f32 = 36.0;
const BUTTON_WIDTH: f32 = 36.0;
const CORNER_RADIUS: f32 = 6.0;
// Font size resolved from theme.typography.base at show-time

/// Response from the number field
pub struct NumberFieldResponse {
    /// The underlying egui response
    pub response: Response,
    /// Whether the value changed this frame
    pub changed: bool,
}

/// Number field with +/- stepper buttons
///
/// # Example
///
/// ```ignore
/// let mut value = 5.0;
/// NumberField::new()
///     .min(0.0)
///     .max(100.0)
///     .step(1.0)
///     .label("Quantity")
///     .show(ui, &mut value);
/// ```
pub struct NumberField {
    id: Option<egui::Id>,
    min: Option<f32>,
    max: Option<f32>,
    step: f32,
    height: f32,
    label: Option<String>,
    description: Option<String>,
    width: Option<f32>,
    disabled: bool,
    bg_color: Option<Color32>,
}

impl NumberField {
    /// Create a new number field with step of 1.0
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: None,
            min: None,
            max: None,
            step: 1.0,
            height: HEIGHT,
            label: None,
            description: None,
            width: None,
            disabled: false,
            bg_color: None,
        }
    }

    /// Set ID for state persistence
    #[must_use]
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set minimum value
    #[must_use]
    pub const fn min(mut self, min: f32) -> Self {
        self.min = Some(min);
        self
    }

    /// Set maximum value
    #[must_use]
    pub const fn max(mut self, max: f32) -> Self {
        self.max = Some(max);
        self
    }

    /// Set step increment
    #[must_use]
    pub const fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    /// Set the height of the field
    #[must_use]
    pub const fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set a label above the field
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set description/helper text below the field
    #[must_use]
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set width of the entire component
    #[must_use]
    pub const fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set disabled state
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Override the background color (default: theme.background())
    #[must_use]
    pub const fn bg_color(mut self, color: Color32) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Clamp value to min/max bounds
    fn clamp_value(&self, value: f32) -> f32 {
        let min = self.min.unwrap_or(f32::NEG_INFINITY);
        let max = self.max.unwrap_or(f32::INFINITY);
        value.clamp(min, max)
    }

    /// Show the number field
    pub fn show(self, ui: &mut Ui, value: &mut f32) -> NumberFieldResponse {
        let theme = ui.ctx().armas_theme();
        let total_width = self.width.unwrap_or(180.0);

        // Load state from memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("number_field_state");
            let stored: f32 = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id).unwrap_or(*value));
            *value = stored;
        }

        let old_value = *value;

        let response = ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 6.0;

            // Label
            if let Some(label) = &self.label {
                ui.label(
                    egui::RichText::new(label)
                        .size(theme.typography.base)
                        .color(if self.disabled {
                            theme.muted_foreground()
                        } else {
                            theme.foreground()
                        }),
                );
            }

            // Number field container
            let inner_response = self.render_field(ui, value, total_width, &theme);

            // Description
            if let Some(desc) = &self.description {
                ui.label(
                    egui::RichText::new(desc)
                        .size(theme.typography.sm)
                        .color(theme.muted_foreground()),
                );
            }

            inner_response
        });

        let changed = (*value - old_value).abs() > f32::EPSILON;

        // Save state to memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("number_field_state");
            ui.ctx().data_mut(|d| d.insert_temp(state_id, *value));
        }

        NumberFieldResponse {
            response: response.inner,
            changed,
        }
    }

    fn render_field(
        &self,
        ui: &mut Ui,
        value: &mut f32,
        total_width: f32,
        theme: &crate::Theme,
    ) -> Response {
        let input_width = total_width - BUTTON_WIDTH * 2.0;
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(total_width, self.height), Sense::hover());

        if !ui.is_rect_visible(rect) {
            return response;
        }

        let painter = ui.painter();
        let disabled_alpha = if self.disabled { 0.5 } else { 1.0 };

        // Outer container background and border
        let bg_color = if self.disabled {
            theme.muted()
        } else {
            self.bg_color.unwrap_or_else(|| theme.background())
        };
        let border_color = theme.input().linear_multiply(disabled_alpha);

        painter.rect_filled(rect, CORNER_RADIUS, bg_color);
        painter.rect_stroke(
            rect,
            CORNER_RADIUS,
            Stroke::new(1.0, border_color),
            egui::StrokeKind::Inside,
        );

        // Three sections: [-] | value | [+]
        let decrement_rect = egui::Rect::from_min_size(rect.min, vec2(BUTTON_WIDTH, self.height));
        let input_rect = egui::Rect::from_min_size(
            rect.min + vec2(BUTTON_WIDTH, 0.0),
            vec2(input_width, self.height),
        );
        let increment_rect = egui::Rect::from_min_size(
            rect.min + vec2(BUTTON_WIDTH + input_width, 0.0),
            vec2(BUTTON_WIDTH, self.height),
        );

        // Vertical dividers
        painter.line_segment(
            [
                pos2(decrement_rect.right(), rect.top() + 1.0),
                pos2(decrement_rect.right(), rect.bottom() - 1.0),
            ],
            Stroke::new(1.0, border_color),
        );
        painter.line_segment(
            [
                pos2(increment_rect.left(), rect.top() + 1.0),
                pos2(increment_rect.left(), rect.bottom() - 1.0),
            ],
            Stroke::new(1.0, border_color),
        );

        // Decrement button
        let can_decrement = !self.disabled && self.min.is_none_or(|min| *value > min);
        let dec_response = ui.interact(
            decrement_rect,
            ui.id().with("dec"),
            if can_decrement {
                Sense::click()
            } else {
                Sense::hover()
            },
        );

        if dec_response.hovered() && can_decrement {
            painter.rect_filled(
                decrement_rect.shrink(1.0),
                CornerRadius {
                    nw: (CORNER_RADIUS - 1.0) as u8,
                    sw: (CORNER_RADIUS - 1.0) as u8,
                    ne: 0,
                    se: 0,
                },
                theme.muted(),
            );
        }

        let minus_color = if can_decrement {
            theme.foreground()
        } else {
            theme.muted_foreground().linear_multiply(0.5)
        };
        let minus_galley = painter.layout_no_wrap(
            "\u{2212}".to_string(), // −
            egui::FontId::proportional(16.0),
            minus_color,
        );
        let minus_pos = decrement_rect.center() - minus_galley.size() / 2.0;
        painter.galley(pos2(minus_pos.x, minus_pos.y), minus_galley, minus_color);

        if dec_response.clicked() && can_decrement {
            *value = self.clamp_value(*value - self.step);
        }

        // Increment button
        let can_increment = !self.disabled && self.max.is_none_or(|max| *value < max);
        let inc_response = ui.interact(
            increment_rect,
            ui.id().with("inc"),
            if can_increment {
                Sense::click()
            } else {
                Sense::hover()
            },
        );

        if inc_response.hovered() && can_increment {
            painter.rect_filled(
                increment_rect.shrink(1.0),
                CornerRadius {
                    nw: 0,
                    sw: 0,
                    ne: (CORNER_RADIUS - 1.0) as u8,
                    se: (CORNER_RADIUS - 1.0) as u8,
                },
                theme.muted(),
            );
        }

        let plus_color = if can_increment {
            theme.foreground()
        } else {
            theme.muted_foreground().linear_multiply(0.5)
        };
        let plus_galley = painter.layout_no_wrap(
            "+".to_string(),
            egui::FontId::proportional(16.0),
            plus_color,
        );
        let plus_pos = increment_rect.center() - plus_galley.size() / 2.0;
        painter.galley(pos2(plus_pos.x, plus_pos.y), plus_galley, plus_color);

        if inc_response.clicked() && can_increment {
            *value = self.clamp_value(*value + self.step);
        }

        // Center value display — editable text
        let text_id = ui.id().with("number_text");
        let is_editing: bool = ui.ctx().data_mut(|d| d.get_temp(text_id).unwrap_or(false));

        if is_editing && !self.disabled {
            // Text edit mode
            let mut text_buf: String = ui.ctx().data_mut(|d| {
                d.get_temp(text_id.with("buf"))
                    .unwrap_or_else(|| format_value(*value))
            });

            let text_rect = input_rect.shrink2(vec2(4.0, 4.0));
            let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(text_rect));

            child_ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
            child_ui.style_mut().visuals.widgets.hovered.bg_fill = Color32::TRANSPARENT;
            child_ui.style_mut().visuals.widgets.active.bg_fill = Color32::TRANSPARENT;
            child_ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::NONE;
            child_ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
            child_ui.style_mut().visuals.widgets.active.bg_stroke = Stroke::NONE;
            child_ui.style_mut().visuals.override_text_color = Some(theme.foreground());

            let text_edit = egui::TextEdit::singleline(&mut text_buf)
                .desired_width(text_rect.width())
                .frame(false)
                .font(egui::FontId::proportional(theme.typography.base))
                .horizontal_align(egui::Align::Center)
                .vertical_align(egui::Align::Center)
                .id(text_id.with("edit"));

            let edit_response = child_ui.add(text_edit);

            // Request focus on first frame of editing
            if !edit_response.has_focus() {
                let first_frame: bool = ui.ctx().data_mut(|d| {
                    let first = d.get_temp(text_id.with("first")).unwrap_or(true);
                    if first {
                        d.insert_temp(text_id.with("first"), false);
                    }
                    first
                });
                if first_frame {
                    edit_response.request_focus();
                }
            }

            // Save buffer
            ui.ctx()
                .data_mut(|d| d.insert_temp(text_id.with("buf"), text_buf.clone()));

            // Commit on Enter or lost focus
            if edit_response.lost_focus() {
                if let Ok(parsed) = text_buf.parse::<f32>() {
                    *value = self.clamp_value(parsed);
                }
                ui.ctx().data_mut(|d| {
                    d.insert_temp(text_id, false);
                    d.remove_by_type::<String>(); // clean up buf
                });
            }
        } else {
            // Display mode — show value, click to edit
            let display_sense = if self.disabled {
                Sense::hover()
            } else {
                Sense::click()
            };
            let input_response = ui.interact(input_rect, text_id.with("display"), display_sense);

            let text_color = if self.disabled {
                theme.muted_foreground()
            } else {
                theme.foreground()
            };

            let value_text = format_value(*value);
            let value_galley = painter.layout_no_wrap(
                value_text.clone(),
                egui::FontId::proportional(theme.typography.base),
                text_color,
            );
            let value_pos = input_rect.center() - value_galley.size() / 2.0;
            painter.galley(pos2(value_pos.x, value_pos.y), value_galley, text_color);

            // Click to enter edit mode
            if input_response.clicked() && !self.disabled {
                ui.ctx().data_mut(|d| {
                    d.insert_temp(text_id, true);
                    d.insert_temp(text_id.with("buf"), value_text);
                    d.insert_temp(text_id.with("first"), true);
                });
            }
        }

        response
    }
}

impl Default for NumberField {
    fn default() -> Self {
        Self::new()
    }
}

/// Format a float value for display (strips trailing zeros)
fn format_value(value: f32) -> String {
    if value.fract().abs() < f32::EPSILON {
        format!("{}", value as i64)
    } else {
        // Show up to 3 decimal places, strip trailing zeros
        let s = format!("{value:.3}");
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_field_creation() {
        let field = NumberField::new();
        assert_eq!(field.step, 1.0);
        assert!(field.min.is_none());
        assert!(field.max.is_none());
        assert!(!field.disabled);
    }

    #[test]
    fn test_number_field_builder() {
        let field = NumberField::new()
            .min(0.0)
            .max(100.0)
            .step(5.0)
            .label("Count")
            .description("Enter a number")
            .width(200.0)
            .disabled(true);

        assert_eq!(field.min, Some(0.0));
        assert_eq!(field.max, Some(100.0));
        assert_eq!(field.step, 5.0);
        assert_eq!(field.label, Some("Count".to_string()));
        assert_eq!(field.description, Some("Enter a number".to_string()));
        assert_eq!(field.width, Some(200.0));
        assert!(field.disabled);
    }

    #[test]
    fn test_clamp_value() {
        let field = NumberField::new().min(0.0).max(10.0);
        assert_eq!(field.clamp_value(-5.0), 0.0);
        assert_eq!(field.clamp_value(5.0), 5.0);
        assert_eq!(field.clamp_value(15.0), 10.0);
    }

    #[test]
    fn test_clamp_no_bounds() {
        let field = NumberField::new();
        assert_eq!(field.clamp_value(-100.0), -100.0);
        assert_eq!(field.clamp_value(100.0), 100.0);
    }

    #[test]
    fn test_format_value() {
        assert_eq!(format_value(5.0), "5");
        assert_eq!(format_value(3.125), "3.125");
        assert_eq!(format_value(0.0), "0");
        assert_eq!(format_value(-1.0), "-1");
        assert_eq!(format_value(2.5), "2.5");
        assert_eq!(format_value(1.100), "1.1");
    }
}
