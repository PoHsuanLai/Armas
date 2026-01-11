//! Glowing Border
//!
//! Container with a pulsing glow border effect

use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{Color32, CornerRadius, Response, Sense, Stroke, Ui, Vec2};

/// Glowing border container
///
/// Wraps content with a pulsing glow border effect
pub struct GlowingBorder {
    width: Option<f32>,
    height: Option<f32>,
    glow_color: Color32,
    background: Color32,
    corner_radius: f32,
    border_width: f32,
    glow_intensity: f32,
    pulse_speed: f32,
    pulse: bool,

    // Internal state
    time: f32,
}

impl GlowingBorder {
    /// Create a new glowing border container
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            glow_color: Color32::from_rgb(59, 130, 246),
            background: Color32::from_gray(20),
            corner_radius: 12.0,
            border_width: 2.0,
            glow_intensity: 1.0,
            pulse_speed: 1.0,
            pulse: true,
            time: 0.0,
        }
    }

    /// Set container width (auto if not set)
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set container height (auto if not set)
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set the glow color
    pub fn glow_color(mut self, color: Color32) -> Self {
        self.glow_color = color;
        self
    }

    /// Set background color
    pub fn background(mut self, color: Color32) -> Self {
        self.background = color;
        self
    }

    /// Set corner radius
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Set border width
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = width.max(1.0);
        self
    }

    /// Set glow intensity (0.0 to 2.0)
    pub fn glow_intensity(mut self, intensity: f32) -> Self {
        self.glow_intensity = intensity.max(0.0);
        self
    }

    /// Set pulse speed
    pub fn pulse_speed(mut self, speed: f32) -> Self {
        self.pulse_speed = speed.max(0.0);
        self
    }

    /// Enable/disable pulsing
    pub fn pulse(mut self, enabled: bool) -> Self {
        self.pulse = enabled;
        self
    }

    /// Show the glowing border with content
    pub fn show<R>(
        &mut self,
        ui: &mut Ui,
        _theme: &Theme,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> Response {
        if self.pulse {
            let dt = ui.input(|i| i.stable_dt);
            self.time += dt * self.pulse_speed;
            ui.ctx().request_repaint();
        }

        // Calculate pulse multiplier
        let pulse_multiplier = if self.pulse {
            0.6 + (self.time * 2.0).sin() * 0.4
        } else {
            1.0
        };

        // Allocate space for the container
        let available = ui.available_size();
        let desired_size = Vec2::new(
            self.width.unwrap_or(available.x),
            self.height.unwrap_or(available.y),
        );

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Draw glow layers (outside to inside)
            let glow_layers = 8;
            for i in 0..glow_layers {
                let t = i as f32 / glow_layers as f32;
                let glow_offset =
                    (glow_layers - i) as f32 * 2.0 * self.glow_intensity * pulse_multiplier;
                let alpha = ((1.0 - t) * 60.0 * self.glow_intensity * pulse_multiplier) as u8;

                let glow_rect = rect.expand(glow_offset);
                let glow_color = Color32::from_rgba_unmultiplied(
                    self.glow_color.r(),
                    self.glow_color.g(),
                    self.glow_color.b(),
                    alpha,
                );

                painter.rect_stroke(
                    glow_rect,
                    CornerRadius::same((self.corner_radius + glow_offset) as u8),
                    Stroke::new(2.0, glow_color),
                    egui::StrokeKind::Outside,
                );
            }

            // Draw background
            painter.rect_filled(
                rect,
                CornerRadius::same(self.corner_radius as u8),
                self.background,
            );

            // Draw main border
            let border_alpha = (255.0 * pulse_multiplier) as u8;
            let border_color = Color32::from_rgba_unmultiplied(
                self.glow_color.r(),
                self.glow_color.g(),
                self.glow_color.b(),
                border_alpha,
            );

            painter.rect_stroke(
                rect,
                CornerRadius::same(self.corner_radius as u8),
                Stroke::new(self.border_width, border_color),
                egui::StrokeKind::Outside,
            );

            // Render content inside
            let content_rect = rect.shrink(16.0); // Padding
            ui.allocate_ui_at_rect(content_rect, |ui| {
                content(ui);
            });
        }

        response
    }
}

impl Default for GlowingBorder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glowing_border_creation() {
        let border = GlowingBorder::new();
        assert_eq!(border.corner_radius, 12.0);
        assert_eq!(border.pulse, true);
    }

    #[test]
    fn test_glowing_border_config() {
        let border = GlowingBorder::new()
            .width(400.0)
            .glow_intensity(1.5)
            .pulse_speed(2.0)
            .pulse(false);

        assert_eq!(border.width, Some(400.0));
        assert_eq!(border.glow_intensity, 1.5);
        assert_eq!(border.pulse_speed, 2.0);
        assert_eq!(border.pulse, false);
    }
}
