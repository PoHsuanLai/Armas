//! Animated Beam Effect
//!
//! Creates animated beams that follow paths with glow effects

use egui::{Color32, Pos2, Response, Stroke, Ui, Vec2};

/// A path point with optional curve control
#[derive(Clone, Debug)]
pub struct PathPoint {
    pub pos: Pos2,
}

impl PathPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Pos2::new(x, y),
        }
    }
}

/// Animated beam that follows a path
pub struct AnimatedBeam {
    /// Path points
    path: Vec<PathPoint>,
    /// Current progress along path (0.0 to 1.0)
    progress: f32,
    /// Speed (progress per second)
    speed: f32,
    /// Color
    color: Color32,
    /// Thickness
    thickness: f32,
    /// Glow intensity
    glow_intensity: f32,
    /// Loop mode
    loop_mode: BeamLoopMode,
    /// Gradient along beam
    gradient: bool,
}

/// How the beam should loop
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BeamLoopMode {
    /// Play once and stop
    Once,
    /// Loop back to start
    Loop,
    /// Bounce back and forth
    PingPong,
}

impl AnimatedBeam {
    /// Create a new animated beam with default color
    pub fn new(path: Vec<PathPoint>) -> Self {
        Self {
            path,
            progress: 0.0,
            speed: 0.5,
            color: Color32::from_rgb(99, 102, 241), // Default primary color
            thickness: 3.0,
            glow_intensity: 0.8,
            loop_mode: BeamLoopMode::Loop,
            gradient: true,
        }
    }

    /// Set the beam speed
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Set the beam color
    pub fn with_color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    /// Set the beam thickness
    pub fn with_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set glow intensity (0.0 to 1.0)
    pub fn with_glow(mut self, intensity: f32) -> Self {
        self.glow_intensity = intensity.clamp(0.0, 1.0);
        self
    }

    /// Set loop mode
    pub fn with_loop_mode(mut self, mode: BeamLoopMode) -> Self {
        self.loop_mode = mode;
        self
    }

    /// Enable/disable gradient along beam
    pub fn with_gradient(mut self, enabled: bool) -> Self {
        self.gradient = enabled;
        self
    }

    /// Update the beam animation
    pub fn update(&mut self, dt: f32) {
        self.progress += dt * self.speed;

        match self.loop_mode {
            BeamLoopMode::Once => {
                if self.progress > 1.0 {
                    self.progress = 1.0;
                }
            }
            BeamLoopMode::Loop => {
                if self.progress > 1.0 {
                    self.progress = 0.0;
                }
            }
            BeamLoopMode::PingPong => {
                if self.progress > 1.0 {
                    self.progress = 2.0 - self.progress;
                    self.speed = -self.speed;
                } else if self.progress < 0.0 {
                    self.progress = -self.progress;
                    self.speed = -self.speed;
                }
            }
        }
    }

    /// Draw the beam
    pub fn draw(&self, ui: &mut Ui) {
        if self.path.len() < 2 {
            return;
        }

        let painter = ui.painter();

        // Calculate total path length
        let mut total_length = 0.0;
        let mut segment_lengths = Vec::new();

        for i in 0..self.path.len() - 1 {
            let length = (self.path[i + 1].pos - self.path[i].pos).length();
            segment_lengths.push(length);
            total_length += length;
        }

        // Find current position on path
        let target_length = total_length * self.progress;
        let mut current_length = 0.0;
        let mut current_pos = self.path[0].pos;

        for (i, &segment_length) in segment_lengths.iter().enumerate() {
            if current_length + segment_length >= target_length {
                let t = (target_length - current_length) / segment_length;
                current_pos = self.path[i].pos + (self.path[i + 1].pos - self.path[i].pos) * t;
                break;
            }
            current_length += segment_length;
        }

        // Draw full path with glow
        if self.glow_intensity > 0.0 {
            let glow_layers = 5;
            for layer in 0..glow_layers {
                let t = layer as f32 / glow_layers as f32;
                let layer_thickness = self.thickness + (self.glow_intensity * 8.0 * t);
                let alpha = ((1.0 - t) * self.glow_intensity * 255.0) as u8;

                let glow_color = Color32::from_rgba_unmultiplied(
                    self.color.r(),
                    self.color.g(),
                    self.color.b(),
                    alpha.min(80),
                );

                for i in 0..self.path.len() - 1 {
                    painter.line_segment(
                        [self.path[i].pos, self.path[i + 1].pos],
                        Stroke::new(layer_thickness, glow_color),
                    );
                }
            }
        }

        // Draw main path with optional gradient
        for i in 0..self.path.len() - 1 {
            let color = if self.gradient {
                let seg_t = i as f32 / (self.path.len() - 1) as f32;
                let alpha = (self.color.a() as f32 * (0.3 + 0.7 * seg_t)) as u8;
                Color32::from_rgba_unmultiplied(
                    self.color.r(),
                    self.color.g(),
                    self.color.b(),
                    alpha,
                )
            } else {
                self.color
            };

            painter.line_segment(
                [self.path[i].pos, self.path[i + 1].pos],
                Stroke::new(self.thickness, color),
            );
        }

        // Draw moving bright point
        painter.circle_filled(current_pos, self.thickness * 1.5, self.color);

        // Draw glow around moving point
        if self.glow_intensity > 0.0 {
            for i in 0..3 {
                let radius = self.thickness * 1.5 + i as f32 * 3.0;
                let alpha = (self.glow_intensity * 100.0 / (i + 1) as f32) as u8;
                let glow = Color32::from_rgba_unmultiplied(
                    self.color.r(),
                    self.color.g(),
                    self.color.b(),
                    alpha,
                );
                painter.circle_filled(current_pos, radius, glow);
            }
        }
    }
}

/// Container for multiple animated beams
pub struct AnimatedBeams {
    beams: Vec<AnimatedBeam>,
    width: f32,
    height: f32,
}

impl AnimatedBeams {
    /// Create a new animated beams container
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            beams: Vec::new(),
            width,
            height,
        }
    }

    /// Add a beam to the container
    pub fn add_beam(mut self, beam: AnimatedBeam) -> Self {
        self.beams.push(beam);
        self
    }

    /// Show all beams
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let (response, _painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());

        let dt = ui.input(|i| i.stable_dt);

        // Update and draw all beams
        for beam in &mut self.beams {
            beam.update(dt);
            beam.draw(ui);
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
    fn test_beam_creation() {
        let path = vec![PathPoint::new(0.0, 0.0), PathPoint::new(100.0, 100.0)];
        let beam = AnimatedBeam::new(path);
        assert_eq!(beam.progress, 0.0);
    }

    #[test]
    fn test_beam_config() {
        let path = vec![PathPoint::new(0.0, 0.0), PathPoint::new(100.0, 100.0)];
        let beam = AnimatedBeam::new(path)
            .with_speed(2.0)
            .with_loop_mode(BeamLoopMode::PingPong);

        assert_eq!(beam.speed, 2.0);
        assert_eq!(beam.loop_mode, BeamLoopMode::PingPong);
    }
}
