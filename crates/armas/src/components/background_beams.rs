//! Background Beams
//!
//! Creates diagonal light beams across the background with subtle animation

use crate::Theme;
use egui::{Color32, Pos2, Response, Ui, Vec2};
use std::f32::consts::PI;

/// Background beams component
///
/// Creates diagonal light beams that fill the background with a subtle glow effect
pub struct BackgroundBeams {
    width: f32,
    height: f32,
    beam_count: usize,
    beam_width: f32,
    beam_angle: f32,
    colors: Vec<Color32>,
    opacity: f32,
    animate: bool,
    blur: bool,

    // Internal state
    animation_offset: f32,
}

impl BackgroundBeams {
    /// Create new background beams
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            beam_count: 8,
            beam_width: 100.0,
            beam_angle: 45.0,
            colors: vec![
                Color32::from_rgba_unmultiplied(59, 130, 246, 40), // Blue
                Color32::from_rgba_unmultiplied(147, 51, 234, 35), // Purple
            ],
            opacity: 0.15,
            animate: true,
            blur: true,
            animation_offset: 0.0,
        }
    }

    /// Set number of beams
    pub fn beam_count(mut self, count: usize) -> Self {
        self.beam_count = count.max(1);
        self
    }

    /// Set beam width
    pub fn beam_width(mut self, width: f32) -> Self {
        self.beam_width = width.max(10.0);
        self
    }

    /// Set beam angle in degrees (0-360)
    pub fn beam_angle(mut self, angle: f32) -> Self {
        self.beam_angle = angle;
        self
    }

    /// Set beam colors
    pub fn colors(mut self, colors: Vec<Color32>) -> Self {
        if !colors.is_empty() {
            self.colors = colors;
        }
        self
    }

    /// Set overall opacity (0.0 to 1.0)
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Enable/disable animation
    pub fn animate(mut self, enabled: bool) -> Self {
        self.animate = enabled;
        self
    }

    /// Enable/disable blur effect
    pub fn blur(mut self, enabled: bool) -> Self {
        self.blur = enabled;
        self
    }

    /// Show the background beams
    pub fn show(&mut self, ui: &mut Ui, _theme: &Theme) -> Response {
        if self.animate {
            let dt = ui.input(|i| i.stable_dt);
            self.animation_offset += dt * 20.0; // Slow movement
            if self.animation_offset > self.beam_width * 2.0 {
                self.animation_offset = 0.0;
            }
            ui.ctx().request_repaint();
        }

        let (response, painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());

        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Convert angle to radians
            let angle_rad = self.beam_angle * PI / 180.0;
            let angle_vec = Vec2::new(angle_rad.cos(), angle_rad.sin());
            let perpendicular = Vec2::new(-angle_vec.y, angle_vec.x);

            // Calculate how many beams we need to cover the entire area
            let diagonal = (rect.width().powi(2) + rect.height().powi(2)).sqrt();
            let spacing = diagonal / self.beam_count as f32;

            // Draw beams
            for i in 0..self.beam_count {
                let color_index = i % self.colors.len();
                let base_color = self.colors[color_index];

                // Apply overall opacity
                let color = Color32::from_rgba_unmultiplied(
                    base_color.r(),
                    base_color.g(),
                    base_color.b(),
                    (base_color.a() as f32 * self.opacity) as u8,
                );

                // Calculate beam position with animation offset
                let offset = i as f32 * spacing + self.animation_offset;
                let center_offset = perpendicular * offset;

                // Calculate beam start and end points along the perpendicular
                let beam_start = rect.center().to_vec2() + center_offset - angle_vec * diagonal;
                let beam_end = rect.center().to_vec2() + center_offset + angle_vec * diagonal;

                // Draw beam as a gradient rectangle
                self.draw_beam(
                    &painter,
                    beam_start.to_pos2(),
                    beam_end.to_pos2(),
                    self.beam_width,
                    color,
                    perpendicular,
                );
            }
        }

        response
    }

    /// Draw a single beam with gradient edges
    fn draw_beam(
        &self,
        painter: &egui::Painter,
        start: Pos2,
        end: Pos2,
        width: f32,
        color: Color32,
        perpendicular: Vec2,
    ) {
        let half_width = width / 2.0;

        if self.blur {
            // Draw with gradient falloff on edges
            let gradient_steps = 10;

            for step in 0..gradient_steps {
                let t = step as f32 / gradient_steps as f32;
                let step_width = half_width * (1.0 - t);
                let alpha_multiplier = 1.0 - t;

                let step_color = Color32::from_rgba_unmultiplied(
                    color.r(),
                    color.g(),
                    color.b(),
                    (color.a() as f32 * alpha_multiplier) as u8,
                );

                // Calculate quad corners
                let offset1 = perpendicular * step_width;
                let offset2 = perpendicular * -step_width;

                let p1 = start + offset1;
                let p2 = start + offset2;
                let p3 = end + offset2;
                let p4 = end + offset1;

                // Draw as a filled shape
                painter.add(egui::Shape::convex_polygon(
                    vec![p1, p2, p3, p4],
                    step_color,
                    egui::Stroke::NONE,
                ));
            }
        } else {
            // Draw solid beam
            let offset1 = perpendicular * half_width;
            let offset2 = perpendicular * -half_width;

            let p1 = start + offset1;
            let p2 = start + offset2;
            let p3 = end + offset2;
            let p4 = end + offset1;

            painter.add(egui::Shape::convex_polygon(
                vec![p1, p2, p3, p4],
                color,
                egui::Stroke::NONE,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_background_beams_creation() {
        let beams = BackgroundBeams::new(800.0, 600.0);
        assert_eq!(beams.width, 800.0);
        assert_eq!(beams.height, 600.0);
        assert_eq!(beams.beam_count, 8);
    }

    #[test]
    fn test_background_beams_config() {
        let beams = BackgroundBeams::new(800.0, 600.0)
            .beam_count(12)
            .beam_width(150.0)
            .beam_angle(60.0)
            .opacity(0.3);

        assert_eq!(beams.beam_count, 12);
        assert_eq!(beams.beam_width, 150.0);
        assert_eq!(beams.beam_angle, 60.0);
        assert_eq!(beams.opacity, 0.3);
    }

    #[test]
    fn test_animation_toggle() {
        let beams = BackgroundBeams::new(800.0, 600.0).animate(false);
        assert_eq!(beams.animate, false);
    }
}
