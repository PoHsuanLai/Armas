//! Card Spotlight Component
//!
//! A card with an animated spotlight effect that follows the mouse cursor

use crate::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Card with spotlight effect that follows mouse
pub struct CardSpotlight {
    width: Option<f32>,
    height: Option<f32>,
    spotlight_color: Color32,
    spotlight_radius: f32,
    spotlight_intensity: f32,
}

impl CardSpotlight {
    /// Create a new card spotlight
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            spotlight_color: Color32::from_rgba_unmultiplied(100, 150, 255, 80),
            spotlight_radius: 150.0,
            spotlight_intensity: 0.3,
        }
    }

    /// Set fixed width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set fixed height
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set spotlight color
    pub fn spotlight_color(mut self, color: Color32) -> Self {
        self.spotlight_color = color;
        self
    }

    /// Set spotlight radius
    pub fn spotlight_radius(mut self, radius: f32) -> Self {
        self.spotlight_radius = radius;
        self
    }

    /// Set spotlight intensity (0.0 - 1.0)
    pub fn spotlight_intensity(mut self, intensity: f32) -> Self {
        self.spotlight_intensity = intensity.clamp(0.0, 1.0);
        self
    }

    /// Show the card spotlight with content
    pub fn show(self, ui: &mut Ui, theme: &Theme, content: impl FnOnce(&mut Ui)) -> Response {
        let desired_width = self.width.unwrap_or_else(|| ui.available_width());
        let desired_height = self.height.unwrap_or(400.0);

        let (rect, mut response) =
            ui.allocate_exact_size(Vec2::new(desired_width, desired_height), Sense::hover());

        if ui.is_rect_visible(rect) {
            // Draw base card background
            let card_bg = theme.surface();
            ui.painter()
                .rect_filled(rect, theme.spacing.corner_radius, card_bg);

            // Draw card border
            ui.painter().rect_stroke(
                rect,
                theme.spacing.corner_radius,
                egui::Stroke::new(1.0, theme.outline_variant()),
                egui::StrokeKind::Outside,
            );

            // Draw content
            let content_response = ui.scope_builder(
                egui::UiBuilder::new()
                    .max_rect(rect.shrink(theme.spacing.lg))
                    .layout(egui::Layout::top_down(egui::Align::LEFT)),
                |ui| {
                    content(ui);
                },
            );

            // Union the responses so we detect hover on content too
            response = response.union(content_response.response);

            // Get mouse position from input
            let pointer_pos = ui.input(|i| i.pointer.hover_pos());

            // Draw spotlight effect on top of everything if hovering the card area
            if let Some(hover_pos) = pointer_pos {
                if rect.contains(hover_pos) {
                    self.draw_spotlight(ui, rect, hover_pos);
                    // Request repaint for animation
                    ui.ctx().request_repaint();
                }
            }
        }

        response
    }

    fn draw_spotlight(&self, ui: &mut Ui, _card_rect: Rect, mouse_pos: Pos2) {
        let painter = ui.painter();

        // Draw spotlight with grainy texture effect (like aceternity)
        // We'll draw more circles with slight random variations to create grain
        let gradient_steps = 50; // More steps for grainier look
        let base_alpha = (self.spotlight_color.a() as f32 * self.spotlight_intensity) as u8;

        // Use time to create subtle animated grain
        let time = ui.input(|i| i.time) as f32;

        for i in 0..gradient_steps {
            let progress = i as f32 / gradient_steps as f32;
            let radius = self.spotlight_radius * (1.0 - progress);

            // Quadratic ease-out for more natural falloff
            let alpha_multiplier = 1.0 - progress * progress;
            let mut alpha = (base_alpha as f32 * alpha_multiplier) as u8;

            // Add grain/noise effect by randomly varying alpha
            // Use deterministic "random" based on step and time
            let noise_seed = (i as f32 * 12.9898 + time * 0.5).sin() * 43_758.547;
            let noise = (noise_seed.fract() - 0.5) * 0.3; // -0.15 to +0.15 variation
            alpha = ((alpha as f32) * (1.0 + noise)).clamp(0.0, 255.0) as u8;

            let circle_color = Color32::from_rgba_unmultiplied(
                self.spotlight_color.r(),
                self.spotlight_color.g(),
                self.spotlight_color.b(),
                alpha,
            );

            // Draw circle, painter will handle clipping to visible area
            painter.circle_filled(mouse_pos, radius, circle_color);
        }
    }
}

impl Default for CardSpotlight {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_spotlight_creation() {
        let spotlight = CardSpotlight::new();
        assert_eq!(spotlight.spotlight_radius, 150.0);
    }

    #[test]
    fn test_card_spotlight_config() {
        let spotlight = CardSpotlight::new()
            .width(400.0)
            .spotlight_radius(200.0)
            .spotlight_intensity(0.5);

        assert_eq!(spotlight.width, Some(400.0));
        assert_eq!(spotlight.spotlight_radius, 200.0);
        assert_eq!(spotlight.spotlight_intensity, 0.5);
    }
}
