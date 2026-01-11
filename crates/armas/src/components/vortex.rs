//! Vortex Background
//!
//! Creates a swirling vortex effect with rotating particles and circular patterns

use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{Color32, Pos2, Response, Ui, Vec2};
use std::f32::consts::PI;

/// Vortex background component
///
/// Creates animated circular patterns that swirl like a vortex
pub struct VortexBackground {
    width: f32,
    height: f32,
    particle_count: usize,
    ring_count: usize,
    colors: Vec<Color32>,
    rotation_speed: f32,
    radius_variation: f32,
    particle_size: f32,

    // Internal state
    time: f32,
}

impl VortexBackground {
    /// Create a new vortex background
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            particle_count: 50,
            ring_count: 6,
            colors: vec![
                Color32::from_rgba_unmultiplied(59, 130, 246, 80), // Blue
                Color32::from_rgba_unmultiplied(147, 51, 234, 70), // Purple
                Color32::from_rgba_unmultiplied(236, 72, 153, 60), // Pink
            ],
            rotation_speed: 0.3,
            radius_variation: 0.2,
            particle_size: 2.0,
            time: 0.0,
        }
    }

    /// Set number of particles per ring
    pub fn particle_count(mut self, count: usize) -> Self {
        self.particle_count = count.max(10);
        self
    }

    /// Set number of concentric rings
    pub fn ring_count(mut self, count: usize) -> Self {
        self.ring_count = count.max(2);
        self
    }

    /// Set vortex colors
    pub fn colors(mut self, colors: Vec<Color32>) -> Self {
        if !colors.is_empty() {
            self.colors = colors;
        }
        self
    }

    /// Set rotation speed
    pub fn rotation_speed(mut self, speed: f32) -> Self {
        self.rotation_speed = speed;
        self
    }

    /// Set radius variation (wobble effect)
    pub fn radius_variation(mut self, variation: f32) -> Self {
        self.radius_variation = variation.clamp(0.0, 1.0);
        self
    }

    /// Set particle size
    pub fn particle_size(mut self, size: f32) -> Self {
        self.particle_size = size.max(1.0);
        self
    }

    /// Show the vortex background
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let dt = ui.input(|i| i.stable_dt);
        self.time += dt;
        ui.ctx().request_repaint();

        let (response, painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());

        let rect = response.rect;
        let center = rect.center();
        let max_radius = rect.width().min(rect.height()) * 0.45;

        if ui.is_rect_visible(rect) {
            // Draw concentric rings with rotating particles
            for ring in 0..self.ring_count {
                let ring_t = (ring + 1) as f32 / self.ring_count as f32;
                let base_radius = max_radius * ring_t;

                // Each ring rotates at different speeds
                let ring_rotation = self.time * self.rotation_speed * (1.0 + ring as f32 * 0.2);

                // Color for this ring
                let color_index = ring % self.colors.len();
                let color = self.colors[color_index];

                // Draw particles in this ring
                for i in 0..self.particle_count {
                    let angle = (i as f32 / self.particle_count as f32) * 2.0 * PI + ring_rotation;

                    // Add radius variation based on time and angle for wobble effect
                    let radius_wobble =
                        (self.time * 2.0 + angle * 3.0).sin() * self.radius_variation;
                    let radius = base_radius * (1.0 + radius_wobble * 0.3);

                    // Calculate particle position
                    let x = center.x + angle.cos() * radius;
                    let y = center.y + angle.sin() * radius;
                    let pos = Pos2::new(x, y);

                    // Fade particles based on their position in the cycle
                    let fade = ((angle + self.time).sin() * 0.3 + 0.7).clamp(0.3, 1.0);
                    let particle_color = Color32::from_rgba_unmultiplied(
                        color.r(),
                        color.g(),
                        color.b(),
                        (color.a() as f32 * fade) as u8,
                    );

                    // Draw particle
                    painter.circle_filled(pos, self.particle_size, particle_color);

                    // Draw glow around particle
                    for glow_layer in 1..3 {
                        let glow_radius = self.particle_size + glow_layer as f32 * 2.0;
                        let glow_alpha = (color.a() as f32 * fade * 0.3 / glow_layer as f32) as u8;
                        let glow_color = Color32::from_rgba_unmultiplied(
                            color.r(),
                            color.g(),
                            color.b(),
                            glow_alpha,
                        );
                        painter.circle_filled(pos, glow_radius, glow_color);
                    }
                }

                // Optionally draw connecting lines between particles for web effect
                if self.particle_count <= 30 {
                    for i in 0..self.particle_count {
                        let angle1 =
                            (i as f32 / self.particle_count as f32) * 2.0 * PI + ring_rotation;
                        let angle2 = ((i + 1) as f32 / self.particle_count as f32) * 2.0 * PI
                            + ring_rotation;

                        let radius_wobble1 =
                            (self.time * 2.0 + angle1 * 3.0).sin() * self.radius_variation;
                        let radius1 = base_radius * (1.0 + radius_wobble1 * 0.3);

                        let radius_wobble2 =
                            (self.time * 2.0 + angle2 * 3.0).sin() * self.radius_variation;
                        let radius2 = base_radius * (1.0 + radius_wobble2 * 0.3);

                        let pos1 = Pos2::new(
                            center.x + angle1.cos() * radius1,
                            center.y + angle1.sin() * radius1,
                        );
                        let pos2 = Pos2::new(
                            center.x + angle2.cos() * radius2,
                            center.y + angle2.sin() * radius2,
                        );

                        let line_color = Color32::from_rgba_unmultiplied(
                            color.r(),
                            color.g(),
                            color.b(),
                            (color.a() as f32 * 0.2) as u8,
                        );

                        painter.line_segment([pos1, pos2], egui::Stroke::new(1.0, line_color));
                    }
                }
            }

            // Draw center glow
            let center_glow_layers = 8;
            for layer in 0..center_glow_layers {
                let t = layer as f32 / center_glow_layers as f32;
                let radius = max_radius * 0.1 * (1.0 - t);
                let alpha = ((1.0 - t) * 40.0) as u8;

                let color_index = (self.time * 2.0) as usize % self.colors.len();
                let color = self.colors[color_index];

                let glow_color =
                    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);

                painter.circle_filled(center, radius, glow_color);
            }
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vortex_creation() {
        let vortex = VortexBackground::new(800.0, 600.0);
        assert_eq!(vortex.width, 800.0);
        assert_eq!(vortex.height, 600.0);
        assert_eq!(vortex.particle_count, 50);
        assert_eq!(vortex.ring_count, 6);
    }

    #[test]
    fn test_vortex_config() {
        let vortex = VortexBackground::new(800.0, 600.0)
            .particle_count(30)
            .ring_count(8)
            .rotation_speed(0.5)
            .particle_size(3.0);

        assert_eq!(vortex.particle_count, 30);
        assert_eq!(vortex.ring_count, 8);
        assert_eq!(vortex.rotation_speed, 0.5);
        assert_eq!(vortex.particle_size, 3.0);
    }
}
