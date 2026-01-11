//! Wavy Background
//!
//! Animated wave effects for backgrounds and hero sections

use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{Color32, Pos2, Response, Stroke, Ui, Vec2};
use std::f32::consts::PI;

/// Wavy background component
///
/// Creates animated sine waves that move across the background
pub struct WavyBackground {
    width: f32,
    height: f32,
    wave_count: usize,
    colors: Vec<Color32>,
    amplitude: f32,
    frequency: f32,
    speed: f32,
    blur: bool,

    // Internal state
    time: f32,
}

impl WavyBackground {
    /// Create a new wavy background
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            wave_count: 5,
            colors: vec![
                Color32::from_rgba_unmultiplied(59, 130, 246, 30), // Blue
                Color32::from_rgba_unmultiplied(147, 51, 234, 25), // Purple
                Color32::from_rgba_unmultiplied(236, 72, 153, 20), // Pink
            ],
            amplitude: 40.0,
            frequency: 0.02,
            speed: 0.5,
            blur: false,
            time: 0.0,
        }
    }

    /// Set number of waves
    pub fn wave_count(mut self, count: usize) -> Self {
        self.wave_count = count.max(1);
        self
    }

    /// Set wave colors
    pub fn colors(mut self, colors: Vec<Color32>) -> Self {
        if !colors.is_empty() {
            self.colors = colors;
        }
        self
    }

    /// Set wave amplitude (height)
    pub fn amplitude(mut self, amplitude: f32) -> Self {
        self.amplitude = amplitude.max(5.0);
        self
    }

    /// Set wave frequency (how wavy)
    pub fn frequency(mut self, freq: f32) -> Self {
        self.frequency = freq.max(0.001);
        self
    }

    /// Set animation speed
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Enable blur effect (uses gradient)
    pub fn with_blur(mut self, enabled: bool) -> Self {
        self.blur = enabled;
        self
    }

    /// Show the wavy background
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let dt = ui.input(|i| i.stable_dt);
        self.time += dt * self.speed;
        ui.ctx().request_repaint();

        let (response, painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());

        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw waves from bottom to top (layered effect)
            for i in 0..self.wave_count {
                let color_index = i % self.colors.len();
                let color = self.colors[color_index];

                // Vary wave parameters for each wave
                let wave_offset = i as f32 * 0.5;
                let phase_shift = i as f32 * PI / 4.0;
                let y_offset = rect.bottom() - (i as f32 * 30.0);

                // Draw wave as a series of line segments
                let segments = 200;
                let mut points = Vec::new();

                for seg in 0..=segments {
                    let x = rect.left() + (seg as f32 / segments as f32) * rect.width();
                    let norm_x = (x - rect.left()) / rect.width();

                    // Calculate wave height using sine
                    let wave_y = (norm_x * 2.0 * PI / self.frequency + self.time + phase_shift)
                        .sin()
                        * self.amplitude
                        * (1.0 + wave_offset * 0.2);

                    points.push(Pos2::new(x, y_offset + wave_y));
                }

                // Draw the wave
                if self.blur {
                    // Draw multiple offset waves for blur effect
                    for offset in 0..3 {
                        let offset_y = offset as f32 * 2.0;
                        let alpha = color.a() as f32 * (0.3 + offset as f32 * 0.1);
                        let blur_color = Color32::from_rgba_unmultiplied(
                            color.r(),
                            color.g(),
                            color.b(),
                            alpha as u8,
                        );

                        let offset_points: Vec<Pos2> = points
                            .iter()
                            .map(|p| Pos2::new(p.x, p.y + offset_y))
                            .collect();

                        painter.add(egui::Shape::line(
                            offset_points,
                            Stroke::new(2.0 + offset as f32, blur_color),
                        ));
                    }
                } else {
                    painter.add(egui::Shape::line(points, Stroke::new(2.0, color)));
                }
            }
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wavy_background_creation() {
        let waves = WavyBackground::new(800.0, 600.0);
        assert_eq!(waves.width, 800.0);
        assert_eq!(waves.height, 600.0);
        assert_eq!(waves.wave_count, 5);
    }

    #[test]
    fn test_wavy_background_config() {
        let waves = WavyBackground::new(800.0, 600.0)
            .wave_count(3)
            .amplitude(60.0)
            .speed(1.0);

        assert_eq!(waves.wave_count, 3);
        assert_eq!(waves.amplitude, 60.0);
        assert_eq!(waves.speed, 1.0);
    }

    #[test]
    fn test_custom_colors() {
        let colors = vec![Color32::RED, Color32::GREEN];
        let waves = WavyBackground::new(800.0, 600.0).colors(colors.clone());
        assert_eq!(waves.colors.len(), 2);
    }
}
