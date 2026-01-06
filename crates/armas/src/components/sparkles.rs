//! Sparkles Effect
//!
//! Animated sparkle particles that overlay content with twinkling stars

use crate::Theme;
use egui::{Color32, Pos2, Response, Ui, Vec2};
use std::f32::consts::PI;

/// A single sparkle particle
#[derive(Clone)]
struct Sparkle {
    position: Pos2,
    size: f32,
    lifetime: f32,
    max_lifetime: f32,
    delay: f32,
    color: Color32,
}

impl Sparkle {
    fn new(position: Pos2, size: f32, delay: f32, color: Color32) -> Self {
        Self {
            position,
            size,
            lifetime: 0.0,
            max_lifetime: 1.0 + (delay % 0.5),
            delay,
            color,
        }
    }

    fn update(&mut self, dt: f32) {
        if self.delay > 0.0 {
            self.delay -= dt;
        } else {
            self.lifetime += dt;
            if self.lifetime > self.max_lifetime {
                self.lifetime = 0.0;
            }
        }
    }

    fn opacity(&self) -> f32 {
        if self.delay > 0.0 {
            return 0.0;
        }

        let t = self.lifetime / self.max_lifetime;
        // Sine wave for smooth twinkling
        let twinkle = ((t * PI * 2.0).sin() + 1.0) / 2.0;
        twinkle * 0.8 + 0.2
    }

    fn draw(&self, painter: &egui::Painter) {
        if self.delay > 0.0 {
            return;
        }

        let opacity = self.opacity();
        let alpha = (self.color.a() as f32 * opacity) as u8;
        let color =
            Color32::from_rgba_unmultiplied(self.color.r(), self.color.g(), self.color.b(), alpha);

        // Draw star shape
        let points = 4; // 4-pointed star
        let outer_radius = self.size;
        let inner_radius = self.size * 0.4;

        let mut star_points = Vec::new();
        for i in 0..(points * 2) {
            let angle = (i as f32 / (points * 2) as f32) * PI * 2.0 - PI / 2.0;
            let radius = if i % 2 == 0 {
                outer_radius
            } else {
                inner_radius
            };
            let x = self.position.x + angle.cos() * radius;
            let y = self.position.y + angle.sin() * radius;
            star_points.push(Pos2::new(x, y));
        }

        painter.add(egui::Shape::convex_polygon(
            star_points,
            color,
            egui::Stroke::NONE,
        ));

        // Add glow
        for i in 1..4 {
            let glow_radius = self.size + i as f32 * 1.5;
            let glow_alpha = (alpha as f32 * 0.3 / i as f32) as u8;
            let glow_color = Color32::from_rgba_unmultiplied(
                self.color.r(),
                self.color.g(),
                self.color.b(),
                glow_alpha,
            );
            painter.circle_filled(self.position, glow_radius, glow_color);
        }
    }
}

/// Sparkles overlay component
///
/// Creates twinkling sparkle particles that can overlay content
pub struct Sparkles {
    width: f32,
    height: f32,
    particle_count: usize,
    colors: Vec<Color32>,
    min_size: f32,
    max_size: f32,

    // Internal state
    sparkles: Vec<Sparkle>,
    initialized: bool,
}

impl Sparkles {
    /// Create a new sparkles effect
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            particle_count: 30,
            colors: vec![
                Color32::from_rgb(255, 215, 0),   // Gold
                Color32::from_rgb(255, 255, 255), // White
                Color32::from_rgb(135, 206, 250), // Sky blue
            ],
            min_size: 2.0,
            max_size: 4.0,
            sparkles: Vec::new(),
            initialized: false,
        }
    }

    /// Set number of sparkle particles
    pub fn particle_count(mut self, count: usize) -> Self {
        self.particle_count = count.max(5);
        self
    }

    /// Set sparkle colors
    pub fn colors(mut self, colors: Vec<Color32>) -> Self {
        if !colors.is_empty() {
            self.colors = colors;
        }
        self
    }

    /// Set size range for sparkles
    pub fn size_range(mut self, min: f32, max: f32) -> Self {
        self.min_size = min.max(1.0);
        self.max_size = max.max(min);
        self
    }

    fn initialize_sparkles(&mut self) {
        self.sparkles.clear();

        // Use a simple pseudo-random approach
        let mut seed = 12345u32;
        let mut random = || {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            (seed / 65536) % 32768
        };

        for i in 0..self.particle_count {
            let x = (random() as f32 / 32768.0) * self.width;
            let y = (random() as f32 / 32768.0) * self.height;
            let size =
                self.min_size + (random() as f32 / 32768.0) * (self.max_size - self.min_size);
            let delay = (i as f32 / self.particle_count as f32) * 2.0;
            let color_index = random() as usize % self.colors.len();

            self.sparkles.push(Sparkle::new(
                Pos2::new(x, y),
                size,
                delay,
                self.colors[color_index],
            ));
        }

        self.initialized = true;
    }

    /// Show the sparkles effect
    pub fn show(&mut self, ui: &mut Ui, _theme: &Theme) -> Response {
        if !self.initialized {
            self.initialize_sparkles();
        }

        let dt = ui.input(|i| i.stable_dt);

        // Update all sparkles
        for sparkle in &mut self.sparkles {
            sparkle.update(dt);
        }

        let (response, painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());

        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw all sparkles
            for sparkle in &self.sparkles {
                let mut adjusted_sparkle = sparkle.clone();
                adjusted_sparkle.position = rect.min + (sparkle.position.to_vec2());
                adjusted_sparkle.draw(&painter);
            }
        }

        ui.ctx().request_repaint();
        response
    }

    /// Show sparkles overlaying content
    pub fn show_with_content<R>(
        &mut self,
        ui: &mut Ui,
        theme: &Theme,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> Response {
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(self.width, self.height), egui::Sense::hover());

        // Render content first
        ui.allocate_ui_at_rect(rect, |ui| {
            content(ui);
        });

        // Then overlay sparkles
        if !self.initialized {
            self.initialize_sparkles();
        }

        let dt = ui.input(|i| i.stable_dt);

        // Update all sparkles
        for sparkle in &mut self.sparkles {
            sparkle.update(dt);
        }

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Draw all sparkles
            for sparkle in &self.sparkles {
                let mut adjusted_sparkle = sparkle.clone();
                adjusted_sparkle.position = rect.min + sparkle.position.to_vec2();
                adjusted_sparkle.draw(&painter);
            }
        }

        ui.ctx().request_repaint();
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparkles_creation() {
        let sparkles = Sparkles::new(800.0, 600.0);
        assert_eq!(sparkles.width, 800.0);
        assert_eq!(sparkles.height, 600.0);
        assert_eq!(sparkles.particle_count, 30);
    }

    #[test]
    fn test_sparkles_config() {
        let sparkles = Sparkles::new(800.0, 600.0)
            .particle_count(50)
            .size_range(3.0, 6.0);

        assert_eq!(sparkles.particle_count, 50);
        assert_eq!(sparkles.min_size, 3.0);
        assert_eq!(sparkles.max_size, 6.0);
    }

    #[test]
    fn test_sparkle_opacity() {
        let sparkle = Sparkle::new(Pos2::ZERO, 3.0, 0.0, Color32::WHITE);
        assert!(sparkle.opacity() >= 0.0 && sparkle.opacity() <= 1.0);
    }
}
