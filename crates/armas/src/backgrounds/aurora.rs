//! Aurora Background Effect
//!
//! Creates a dreamy, atmospheric background with floating gradient blobs
//! that move smoothly using perlin-like motion patterns.

use crate::color::{ColorStop, Gradient};
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
    /// Current phase for smooth motion
    phase_x: f32,
    phase_y: f32,
    /// Frequency multipliers for organic motion
    freq_x: f32,
    freq_y: f32,
}

impl AuroraBlob {
    fn new(pos: Pos2, radius: f32, colors: Vec<Color32>) -> Self {
        use std::collections::hash_map::RandomState;
        use std::hash::BuildHasher;

        // Generate pseudo-random values based on position

        let hash1 = RandomState::new().hash_one(pos.x.to_bits());

        let hash2 = RandomState::new().hash_one(pos.y.to_bits());

        Self {
            pos,
            radius,
            colors,
            phase_x: (hash1 % 360) as f32 * PI / 180.0,
            phase_y: (hash2 % 360) as f32 * PI / 180.0,
            freq_x: 0.5 + ((hash1 % 100) as f32 / 200.0),
            freq_y: 0.5 + ((hash2 % 100) as f32 / 200.0),
        }
    }

    fn update_position(&mut self, time: f32, bounds: Rect) {
        // Smooth sinusoidal motion based on absolute time
        // time already includes speed_multiplier from AuroraBackground
        // Apply blob-specific frequency variation
        let phase_x = time * self.freq_x + self.phase_x;
        let phase_y = time * self.freq_y + self.phase_y;

        // Calculate new position with wrapping
        let center = bounds.center();
        let offset_x = (phase_x.sin() * bounds.width() * 0.4)
            .clamp(-bounds.width() * 0.5, bounds.width() * 0.5);
        let offset_y = (phase_y.sin() * bounds.height() * 0.4)
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

/// Persistent state for aurora animation
#[derive(Clone)]
struct AuroraState {
    blobs: Vec<AuroraBlob>,
}

/// Aurora background effect with floating gradient blobs
///
/// Creates an atmospheric background with multiple colored blobs that
/// move smoothly in organic patterns, perfect for hero sections or
/// ambient backgrounds.
pub struct AuroraBackground {
    id: egui::Id,
    blobs: Vec<AuroraBlob>,
    width: f32,
    height: f32,
    speed_multiplier: f32,
    time_offset: f32,
}

impl AuroraBackground {
    /// Create a new aurora background with default settings
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            id: egui::Id::new("aurora_default"),
            blobs: Vec::new(),
            width,
            height,
            speed_multiplier: 1.0,
            time_offset: 0.0,
        }
    }

    /// Set a unique ID for this aurora (required for state persistence)
    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        self.id = egui::Id::new(id);
        self
    }

    /// Create aurora with cyberpunk color scheme
    pub fn cyberpunk(width: f32, height: f32) -> Self {
        let mut aurora = Self::new(width, height);

        let center = Pos2::new(width / 2.0, height / 2.0);

        // Cyan blob
        aurora.blobs.push(AuroraBlob::new(
            Pos2::new(center.x - 150.0, center.y - 100.0),
            250.0,
            vec![
                Color32::from_rgba_unmultiplied(0, 255, 255, 150),
                Color32::from_rgba_unmultiplied(0, 255, 255, 0),
            ],
        ));

        // Magenta blob
        aurora.blobs.push(AuroraBlob::new(
            Pos2::new(center.x + 150.0, center.y + 100.0),
            300.0,
            vec![
                Color32::from_rgba_unmultiplied(255, 0, 255, 120),
                Color32::from_rgba_unmultiplied(255, 0, 255, 0),
            ],
        ));

        // Blue blob
        aurora.blobs.push(AuroraBlob::new(
            Pos2::new(center.x, center.y),
            220.0,
            vec![
                Color32::from_rgba_unmultiplied(0, 191, 255, 140),
                Color32::from_rgba_unmultiplied(0, 191, 255, 0),
            ],
        ));

        aurora
    }

    /// Create aurora with aurora borealis color scheme
    pub fn borealis(width: f32, height: f32) -> Self {
        let mut aurora = Self::new(width, height);

        let center = Pos2::new(width / 2.0, height / 2.0);

        // Green blob
        aurora.blobs.push(AuroraBlob::new(
            Pos2::new(center.x - 100.0, center.y),
            280.0,
            vec![
                Color32::from_rgba_unmultiplied(0, 255, 127, 160),
                Color32::from_rgba_unmultiplied(0, 255, 127, 0),
            ],
        ));

        // Blue blob
        aurora.blobs.push(AuroraBlob::new(
            Pos2::new(center.x + 100.0, center.y - 80.0),
            250.0,
            vec![
                Color32::from_rgba_unmultiplied(64, 224, 208, 140),
                Color32::from_rgba_unmultiplied(64, 224, 208, 0),
            ],
        ));

        // Purple blob
        aurora.blobs.push(AuroraBlob::new(
            Pos2::new(center.x, center.y + 80.0),
            230.0,
            vec![
                Color32::from_rgba_unmultiplied(138, 43, 226, 130),
                Color32::from_rgba_unmultiplied(138, 43, 226, 0),
            ],
        ));

        aurora
    }

    /// Create aurora with warm sunset colors
    pub fn sunset(width: f32, height: f32) -> Self {
        let mut aurora = Self::new(width, height);

        let center = Pos2::new(width / 2.0, height / 2.0);

        // Orange blob
        aurora.blobs.push(AuroraBlob::new(
            Pos2::new(center.x - 120.0, center.y - 60.0),
            280.0,
            vec![
                Color32::from_rgba_unmultiplied(255, 140, 0, 150),
                Color32::from_rgba_unmultiplied(255, 140, 0, 0),
            ],
        ));

        // Pink blob
        aurora.blobs.push(AuroraBlob::new(
            Pos2::new(center.x + 120.0, center.y + 60.0),
            260.0,
            vec![
                Color32::from_rgba_unmultiplied(255, 105, 180, 135),
                Color32::from_rgba_unmultiplied(255, 105, 180, 0),
            ],
        ));

        // Yellow blob
        aurora.blobs.push(AuroraBlob::new(
            Pos2::new(center.x, center.y),
            240.0,
            vec![
                Color32::from_rgba_unmultiplied(255, 215, 0, 120),
                Color32::from_rgba_unmultiplied(255, 215, 0, 0),
            ],
        ));

        aurora
    }

    /// Set the speed multiplier for all blobs
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed_multiplier = speed;
        self
    }

    /// Set a time offset to desynchronize from other aurora instances
    pub fn time_offset(mut self, offset: f32) -> Self {
        self.time_offset = offset;
        self
    }

    /// Add a custom blob
    pub fn add_blob(mut self, pos: Pos2, radius: f32, colors: Vec<Color32>) -> Self {
        self.blobs.push(AuroraBlob::new(pos, radius, colors));
        self
    }

    /// Show the aurora background
    pub fn show(self, ui: &mut Ui) -> Response {
        let (response, _painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());

        let bounds = response.rect;
        let base_time = ui.input(|i| i.time) as f32;
        let time = (base_time * self.speed_multiplier) + self.time_offset;

        // Get or initialize state from egui memory
        let mut state = ui.data_mut(|d| {
            d.get_temp::<AuroraState>(self.id).unwrap_or(AuroraState {
                blobs: self.blobs.clone(),
            })
        });

        // Update and draw all blobs
        for blob in &mut state.blobs {
            blob.update_position(time, bounds);
            blob.draw(ui);
        }

        // Store state back
        ui.data_mut(|d| d.insert_temp(self.id, state));

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
        let aurora = AuroraBackground::cyberpunk(800.0, 600.0);
        assert_eq!(aurora.blobs.len(), 3);
    }

    #[test]
    fn test_aurora_speed() {
        let aurora = AuroraBackground::borealis(800.0, 600.0).speed(2.0);
        assert_eq!(aurora.speed_multiplier, 2.0);
    }
}
