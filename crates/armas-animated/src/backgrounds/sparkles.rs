//! Sparkles Effect
//!
//! Animated sparkle particles that overlay content with twinkling stars

use armas::Theme;
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
        let twinkle = f32::midpoint((t * PI * 2.0).sin(), 1.0);
        twinkle * 0.8 + 0.2
    }

    fn draw(&self, painter: &egui::Painter) {
        if self.delay > 0.0 {
            return;
        }

        let opacity = self.opacity();
        let alpha = (f32::from(self.color.a()) * opacity) as u8;
        let color =
            Color32::from_rgba_unmultiplied(self.color.r(), self.color.g(), self.color.b(), alpha);

        // Draw star shape
        let points = 4; // 4-pointed star
        let outer_radius = self.size;
        let inner_radius = self.size * 0.4;

        let mut star_points = Vec::new();
        for i in 0..(points * 2) {
            let angle = ((i as f32 / (points * 2) as f32) * PI).mul_add(2.0, -(PI / 2.0));
            let radius = if i % 2 == 0 {
                outer_radius
            } else {
                inner_radius
            };
            let x = angle.cos().mul_add(radius, self.position.x);
            let y = angle.sin().mul_add(radius, self.position.y);
            star_points.push(Pos2::new(x, y));
        }

        painter.add(egui::Shape::convex_polygon(
            star_points,
            color,
            egui::Stroke::NONE,
        ));

        // Add glow
        for i in 1..4 {
            let glow_radius = (i as f32).mul_add(1.5, self.size);
            let glow_alpha = (f32::from(alpha) * 0.3 / i as f32) as u8;
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

/// Persistent state for sparkles animation
#[derive(Clone)]
struct SparklesState {
    particles: Vec<Sparkle>,
    initialized: bool,
}

/// Sparkles overlay component
///
/// Creates twinkling sparkle particles that can overlay content
pub struct Sparkles {
    id: egui::Id,
    width: f32,
    height: f32,
    particle_count: usize,
    colors: Vec<Color32>,
    min_size: f32,
    max_size: f32,

    // Initial state (used for initialization)
    particles: Vec<Sparkle>,
    initialized: bool,
}

impl Sparkles {
    /// Create a new sparkles effect
    /// Colors will be set from theme context if not customized via `.colors()`
    #[must_use]
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            id: egui::Id::new("sparkles_default"),
            width,
            height,
            particle_count: 30,
            colors: vec![
                Color32::from_rgb(255, 215, 0),   // Temporary default
                Color32::from_rgb(255, 255, 255), // Temporary default
                Color32::from_rgb(135, 206, 250), // Temporary default
            ],
            min_size: 2.0,
            max_size: 4.0,
            particles: Vec::new(),
            initialized: false,
        }
    }

    /// Set a unique ID for this sparkles effect (required for state persistence)
    #[must_use]
    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        self.id = egui::Id::new(id);
        self
    }

    /// Set number of sparkle particles
    #[must_use]
    pub fn particle_count(mut self, count: usize) -> Self {
        self.particle_count = count.max(5);
        self
    }

    /// Set sparkle colors
    #[must_use]
    pub fn colors(mut self, colors: Vec<Color32>) -> Self {
        if !colors.is_empty() {
            self.colors = colors;
        }
        self
    }

    /// Set size range for sparkles
    #[must_use]
    pub const fn size_range(mut self, min: f32, max: f32) -> Self {
        self.min_size = min.max(1.0);
        self.max_size = max.max(min);
        self
    }

    fn initialize_sparkles(&mut self) {
        self.particles.clear();

        // Use a simple pseudo-random approach
        let mut seed = 12345u32;
        let mut random = || {
            seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12345);
            (seed / 65536) % 32768
        };

        for i in 0..self.particle_count {
            let x = (random() as f32 / 32768.0) * self.width;
            let y = (random() as f32 / 32768.0) * self.height;
            let size =
                (random() as f32 / 32768.0).mul_add(self.max_size - self.min_size, self.min_size);
            let delay = (i as f32 / self.particle_count as f32) * 2.0;
            let color_index = random() as usize % self.colors.len();

            self.particles.push(Sparkle::new(
                Pos2::new(x, y),
                size,
                delay,
                self.colors[color_index],
            ));
        }

        self.initialized = true;
    }

    /// Show the sparkles effect, allocating layout space.
    pub fn show(mut self, ui: &mut Ui) -> Response {
        let (response, _painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());

        self.paint_impl(ui, response.rect);
        response
    }

    /// Paint sparkles onto a rect without allocating layout space.
    ///
    /// Use this to render sparkles as a background behind other content.
    pub fn paint(mut self, ui: &mut Ui, rect: egui::Rect) {
        self.paint_impl(ui, rect);
    }

    fn paint_impl(&mut self, ui: &mut Ui, rect: egui::Rect) {
        let mut state = ui.data_mut(|d| {
            d.get_temp::<SparklesState>(self.id)
                .unwrap_or_else(|| SparklesState {
                    particles: self.particles.clone(),
                    initialized: self.initialized,
                })
        });

        if !state.initialized {
            self.initialize_sparkles();
            state.particles.clone_from(&self.particles);
            state.initialized = true;
        }

        let dt = ui.input(|i| i.stable_dt);

        for sparkle in &mut state.particles {
            sparkle.update(dt);
        }

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            for sparkle in &state.particles {
                let mut adjusted_sparkle = sparkle.clone();
                adjusted_sparkle.position = rect.min + sparkle.position.to_vec2();
                adjusted_sparkle.draw(painter);
            }
        }

        ui.data_mut(|d| d.insert_temp(self.id, state));
        ui.ctx().request_repaint();
    }

    /// Show sparkles overlaying content
    pub fn show_with_content<R>(
        mut self,
        ui: &mut Ui,
        _theme: &Theme,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> Response {
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(self.width, self.height), egui::Sense::hover());

        // Render content first
        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            content(ui);
        });

        // Get or initialize state from egui memory
        let mut state = ui.data_mut(|d| {
            d.get_temp::<SparklesState>(self.id)
                .unwrap_or_else(|| SparklesState {
                    particles: self.particles.clone(),
                    initialized: self.initialized,
                })
        });

        // Then overlay sparkles
        if !state.initialized {
            self.initialize_sparkles();
            state.particles.clone_from(&self.particles);
            state.initialized = true;
        }

        let dt = ui.input(|i| i.stable_dt);

        // Update all sparkles
        for sparkle in &mut state.particles {
            sparkle.update(dt);
        }

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Draw all sparkles
            for sparkle in &state.particles {
                let mut adjusted_sparkle = sparkle.clone();
                adjusted_sparkle.position = rect.min + sparkle.position.to_vec2();
                adjusted_sparkle.draw(painter);
            }
        }

        // Store state back
        ui.data_mut(|d| d.insert_temp(self.id, state));

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
