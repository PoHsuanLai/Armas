//! Aurora Background Effect
//!
//! Creates a dreamy, atmospheric background with floating gradient blobs
//! that move smoothly using perlin-like motion patterns.

use crate::ext::ArmasContextExt;
use crate::color::{ColorStop, Gradient};
use crate::context::ArmasContextExt;
use egui::{Color32, Pos2, Rect, Response, Ui, Vec2};
use std::f32::consts::PI;

/// Configuration for an aurora blob
#[derive(Clone, Debug)]
struct AuroraBlob {
    /// Current position
    pos: Pos2,
    /// Current radius
    radius: f32,
    /// Color gradient stops
    colors: Vec<Color32>,
    /// Movement speed (pixels per second)
    speed: f32,
    /// Current phase for smooth motion
    phase_x: f32,
    phase_y: f32,
    /// Frequency multipliers for organic motion
    freq_x: f32,
    freq_y: f32,
}

impl AuroraBlob {
    fn new(pos: Pos2, radius: f32, colors: Vec<Color32>, speed: f32) -> Self {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hash, Hasher};

        // Generate pseudo-random values based on position
        let mut hasher = RandomState::new().build_hasher();
        pos.x.to_bits().hash(&mut hasher);
        let hash1 = hasher.finish();

        let mut hasher = RandomState::new().build_hasher();
        pos.y.to_bits().hash(&mut hasher);
        let hash2 = hasher.finish();

        Self {
            pos,
            radius,
            colors,
            speed,
            phase_x: (hash1 % 360) as f32 * PI / 180.0,
            phase_y: (hash2 % 360) as f32 * PI / 180.0,
            freq_x: 0.5 + ((hash1 % 100) as f32 / 200.0),
            freq_y: 0.5 + ((hash2 % 100) as f32 / 200.0),
        }
    }

    fn update(&mut self, dt: f32, bounds: Rect) {
        // Smooth sinusoidal motion
        self.phase_x += dt * self.speed * self.freq_x;
        self.phase_y += dt * self.speed * self.freq_y;

        // Calculate new position with wrapping
        let center = bounds.center();
        let offset_x = (self.phase_x.sin() * bounds.width() * 0.4)
            .clamp(-bounds.width() * 0.5, bounds.width() * 0.5);
        let offset_y = (self.phase_y.sin() * bounds.height() * 0.4)
            .clamp(-bounds.height() * 0.5, bounds.height() * 0.5);

        self.pos = Pos2::new(center.x + offset_x, center.y + offset_y);
    }

    fn draw(&self, ui: &mut Ui) {
        let painter = ui.painter();

        // Create gradient from colors
        let stops: Vec<ColorStop> = self
            .colors
            .iter()
            .enumerate()
            .map(|(i, &color)| {
                ColorStop::new(i as f32 / (self.colors.len() - 1).max(1) as f32, color)
            })
            .collect();

        let gradient = Gradient::new(stops);
        let mesh = gradient.radial_mesh(self.pos, self.radius, 32);
        painter.add(egui::Shape::Mesh(std::sync::Arc::new(mesh)));
    }
}

/// Aurora background effect with floating gradient blobs
///
/// Creates an atmospheric background with multiple colored blobs that
/// move smoothly in organic patterns, perfect for hero sections or
/// ambient backgrounds.
pub struct AuroraBackground {
    blobs: Vec<AuroraBlob>,
    width: f32,
    height: f32,
    speed_multiplier: f32,
    time_elapsed: f32,
}

impl AuroraBackground {
    /// Create a new aurora background with default settings
    /// Colors will be derived from theme when shown
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            blobs: Vec::new(),
            width,
            height,
            speed_multiplier: 1.0,
            time_elapsed: 0.0,
        }
    }

    /// Set the speed multiplier for all blobs
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed_multiplier = speed;
        self
    }

    /// Add a custom blob
    pub fn add_blob(mut self, pos: Pos2, radius: f32, colors: Vec<Color32>, speed: f32) -> Self {
        self.blobs.push(AuroraBlob::new(pos, radius, colors, speed));
        self
    }

    /// Show the aurora background
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();
        let (response, _painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());

        let bounds = response.rect;
        let dt = ui.input(|i| i.stable_dt);
        self.time_elapsed += dt;

        // If no custom blobs, create from theme gradient
        if self.blobs.is_empty() {
            let center = Pos2::new(self.width / 2.0, self.height / 2.0);
            let [c1, c2, c3] = theme.gradient();

            // Primary blob
            self.blobs.push(AuroraBlob::new(
                Pos2::new(center.x - 150.0, center.y - 100.0),
                250.0,
                vec![
                    Color32::from_rgba_unmultiplied(c1.r(), c1.g(), c1.b(), 150),
                    Color32::TRANSPARENT,
                ],
                0.3,
            ));

            // Secondary blob
            self.blobs.push(AuroraBlob::new(
                Pos2::new(center.x + 150.0, center.y + 100.0),
                300.0,
                vec![
                    Color32::from_rgba_unmultiplied(c2.r(), c2.g(), c2.b(), 120),
                    Color32::TRANSPARENT,
                ],
                0.25,
            ));

            // Tertiary blob
            self.blobs.push(AuroraBlob::new(
                Pos2::new(center.x, center.y),
                220.0,
                vec![
                    Color32::from_rgba_unmultiplied(c3.r(), c3.g(), c3.b(), 140),
                    Color32::TRANSPARENT,
                ],
                0.35,
            ));
        }

        // Update and draw all blobs
        for blob in &mut self.blobs {
            blob.update(dt * self.speed_multiplier, bounds);
            blob.draw(ui);
        }

        // Request repaint for continuous animation
        ui.ctx().request_repaint();

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aurora_creation() {
        let aurora = AuroraBackground::new(800.0, 600.0);
        assert_eq!(aurora.blobs.len(), 0); // Blobs created on first show()
        assert_eq!(aurora.width, 800.0);
        assert_eq!(aurora.height, 600.0);
    }

    #[test]
    fn test_aurora_speed() {
        let aurora = AuroraBackground::new(800.0, 600.0).speed(2.0);
        assert_eq!(aurora.speed_multiplier, 2.0);
    }

    #[test]
    fn test_aurora_custom_blob() {
        let aurora = AuroraBackground::new(800.0, 600.0).add_blob(
            Pos2::new(100.0, 100.0),
            150.0,
            vec![Color32::RED, Color32::TRANSPARENT],
            0.5,
        );
        assert_eq!(aurora.blobs.len(), 1);
    }
}
