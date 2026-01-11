//! Meteor Shower Effect
//!
//! Creates shooting stars across the screen with trail effects

use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{Color32, Pos2, Rect, Response, Stroke, Ui, Vec2};
use std::f32::consts::PI;
use std::hash::Hash;

/// A single meteor/shooting star
#[derive(Clone, Debug)]
struct Meteor {
    /// Start position
    start: Pos2,
    /// End position
    end: Pos2,
    /// Progress along path (0.0 to 1.0)
    progress: f32,
    /// Tail length in pixels
    tail_length: f32,
    /// Speed (progress per second)
    speed: f32,
    /// Color
    color: Color32,
    /// Thickness
    thickness: f32,
}

impl Meteor {
    fn new(
        start: Pos2,
        end: Pos2,
        speed: f32,
        color: Color32,
        tail_length: f32,
        thickness: f32,
    ) -> Self {
        Self {
            start,
            end,
            progress: 0.0,
            tail_length,
            speed,
            color,
            thickness,
        }
    }

    fn update(&mut self, dt: f32) {
        self.progress += dt * self.speed;
    }

    fn is_finished(&self) -> bool {
        self.progress > 1.0
    }

    fn draw(&self, ui: &mut Ui) {
        if self.progress < 0.0 || self.progress > 1.0 {
            return;
        }

        let painter = ui.painter();
        let segment = self.end - self.start;
        let current_pos = self.start + segment * self.progress;

        // Calculate tail start position
        let tail_t = (self.progress - self.tail_length / segment.length()).max(0.0);
        let tail_start = self.start + segment * tail_t;

        // Draw trail with gradient fade
        let trail_points = 8;
        for i in 0..trail_points {
            let t = i as f32 / trail_points as f32;
            let pos = tail_start + (current_pos - tail_start) * t;
            let next_t = (i + 1) as f32 / trail_points as f32;
            let next_pos = tail_start + (current_pos - tail_start) * next_t;

            let alpha = (t * self.color.a() as f32) as u8;
            let trail_color = Color32::from_rgba_unmultiplied(
                self.color.r(),
                self.color.g(),
                self.color.b(),
                alpha,
            );

            let thickness = self.thickness * (0.5 + 0.5 * t);

            painter.line_segment([pos, next_pos], Stroke::new(thickness, trail_color));
        }

        // Draw bright head
        painter.circle_filled(current_pos, self.thickness * 0.8, self.color);
    }
}

/// Meteor shower effect with shooting stars
///
/// Creates a continuous meteor shower with customizable angle, speed,
/// and spawn rate.
pub struct MeteorShower {
    meteors: Vec<Meteor>,
    spawn_timer: f32,
    spawn_rate: f32,
    angle: f32,
    speed_min: f32,
    speed_max: f32,
    color: Color32,
    width: f32,
    height: f32,
}

impl MeteorShower {
    /// Create a new meteor shower with theme-based defaults
    pub fn new(width: f32, height: f32, theme: &Theme) -> Self {
        Self {
            meteors: Vec::new(),
            spawn_timer: 0.0,
            spawn_rate: 0.5, // spawn every 0.5 seconds
            angle: PI / 4.0, // 45 degrees
            speed_min: 0.8,
            speed_max: 1.2,
            color: theme.primary(),
            width,
            height,
        }
    }

    /// Set the spawn rate (meteors per second)
    pub fn with_spawn_rate(mut self, rate: f32) -> Self {
        self.spawn_rate = 1.0 / rate;
        self
    }

    /// Set the angle in radians (0 = right, PI/2 = down)
    pub fn with_angle(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }

    /// Set the meteor color
    pub fn with_color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    /// Set speed range
    pub fn with_speed_range(mut self, min: f32, max: f32) -> Self {
        self.speed_min = min;
        self.speed_max = max;
        self
    }

    fn spawn_meteor(&mut self, bounds: Rect) {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};

        // Generate pseudo-random values
        let mut hasher = RandomState::new().build_hasher();
        self.spawn_timer.to_bits().hash(&mut hasher);
        let hash = hasher.finish();

        // Random position along spawn edge
        let spawn_t = (hash % 1000) as f32 / 1000.0;

        // Calculate spawn and end positions based on angle
        let dir = Vec2::new(self.angle.cos(), self.angle.sin());

        // Spawn from top or left edge depending on angle
        let start = if self.angle.abs() < PI / 4.0 {
            // Spawn from left
            Pos2::new(
                bounds.left() - 50.0,
                bounds.top() + spawn_t * bounds.height(),
            )
        } else {
            // Spawn from top
            Pos2::new(
                bounds.left() + spawn_t * bounds.width(),
                bounds.top() - 50.0,
            )
        };

        // Calculate end position (shoot across screen)
        let distance = (bounds.width().powi(2) + bounds.height().powi(2)).sqrt() + 100.0;
        let end = start + dir * distance;

        // Random speed
        let speed_t = ((hash >> 10) % 1000) as f32 / 1000.0;
        let speed = self.speed_min + (self.speed_max - self.speed_min) * speed_t;

        // Random tail length
        let tail_length = 80.0 + ((hash >> 20) % 100) as f32;

        self.meteors
            .push(Meteor::new(start, end, speed, self.color, tail_length, 2.5));
    }

    /// Show the meteor shower
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let (response, _painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());

        let bounds = response.rect;
        let dt = ui.input(|i| i.stable_dt);

        // Update spawn timer
        self.spawn_timer += dt;
        if self.spawn_timer >= self.spawn_rate {
            self.spawn_meteor(bounds);
            self.spawn_timer = 0.0;
        }

        // Update and draw all meteors
        for meteor in &mut self.meteors {
            meteor.update(dt);
            meteor.draw(ui);
        }

        // Remove finished meteors
        self.meteors.retain(|m| !m.is_finished());

        // Request repaint for continuous animation
        ui.ctx().request_repaint();

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meteor_creation() {
        let theme = Theme::default();
        let shower = MeteorShower::new(800.0, 600.0, &theme);
        assert_eq!(shower.meteors.len(), 0);
    }

    #[test]
    fn test_meteor_config() {
        let theme = Theme::default();
        let shower = MeteorShower::new(800.0, 600.0, &theme)
            .with_spawn_rate(2.0)
            .with_angle(PI / 6.0);

        assert_eq!(shower.spawn_rate, 0.5);
        assert_eq!(shower.angle, PI / 6.0);
    }
}
