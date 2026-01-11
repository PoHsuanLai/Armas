//! Slider Component
//!
//! Horizontal slider for value selection

use crate::layout::{HStack, Spacer, VStack};
use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{pos2, vec2, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

/// Slider component
pub struct Slider {
    value: f32,
    min: f32,
    max: f32,
    width: f32,
    height: f32,
    show_value: bool,
    label: Option<String>,
    suffix: Option<String>,
    step: Option<f32>,
}

impl Slider {
    /// Create a new slider
    pub fn new(value: f32, min: f32, max: f32) -> Self {
        Self {
            value: value.clamp(min, max),
            min,
            max,
            width: 200.0,
            height: 20.0,
            show_value: true,
            label: None,
            suffix: None,
            step: None,
        }
    }

    /// Set the slider width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the slider height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Show or hide the value label
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Set a label for the slider
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set a suffix for the value (e.g., "%", "ms", "dB")
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Set a step value for snapping
    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    /// Show the slider
    pub fn show(mut self, ui: &mut Ui) -> SliderResponse {
        let theme = ui.ctx().armas_theme();
        let mut changed = false;

        VStack::new(4.0).show(ui, |ui| {
            // Label
            if let Some(label) = &self.label {
                HStack::new(8.0).show(ui, |ui| {
                    ui.label(label);

                    if self.show_value {
                        Spacer::new().show(ui);

                        let value_text = if let Some(suffix) = &self.suffix {
                            format!("{:.1}{}", self.value, suffix)
                        } else {
                            format!("{:.1}", self.value)
                        };
                        ui.label(value_text);
                    }
                });
            }

            // Slider track and handle
            let (rect, response) =
                ui.allocate_exact_size(vec2(self.width, self.height), Sense::click_and_drag());

            if response.clicked() || response.dragged() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let t = ((pos.x - rect.left()) / rect.width()).clamp(0.0, 1.0);
                    let mut new_value = self.min + t * (self.max - self.min);

                    // Apply step if specified
                    if let Some(step) = self.step {
                        new_value = (new_value / step).round() * step;
                    }

                    if (new_value - self.value).abs() > 0.001 {
                        self.value = new_value.clamp(self.min, self.max);
                        changed = true;
                    }
                }
            }

            if ui.is_rect_visible(rect) {
                let painter = ui.painter();

                // Background track
                let track_rect = Rect::from_center_size(rect.center(), vec2(rect.width(), 4.0));

                painter.rect_filled(track_rect, 2.0, theme.surface_variant());

                // Filled track (progress)
                let t = (self.value - self.min) / (self.max - self.min);
                let fill_width = track_rect.width() * t;
                let fill_rect = Rect::from_min_size(track_rect.min, vec2(fill_width, 4.0));

                painter.rect_filled(fill_rect, 2.0, theme.primary());

                // Handle (thumb)
                let handle_x = track_rect.left() + fill_width;
                let handle_center = pos2(handle_x, track_rect.center().y);
                let handle_radius = self.height / 2.0;

                // Handle shadow
                painter.circle_filled(
                    handle_center + vec2(0.0, 1.0),
                    handle_radius,
                    Color32::from_black_alpha(40),
                );

                // Handle
                let handle_color = if response.hovered() || response.dragged() {
                    theme.primary()
                } else {
                    theme.on_surface()
                };

                painter.circle_filled(handle_center, handle_radius, handle_color);

                // Handle border
                painter.circle_stroke(
                    handle_center,
                    handle_radius,
                    Stroke::new(2.0, theme.surface()),
                );
            }
        });

        SliderResponse {
            value: self.value,
            changed,
        }
    }
}

/// Response from a slider
#[derive(Debug, Clone, Copy)]
pub struct SliderResponse {
    /// Current value
    pub value: f32,
    /// Whether the value changed this frame
    pub changed: bool,
}
