//! Wobble Card
//!
//! Card that wobbles and jiggles on hover for playful interactions

use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{Color32, CornerRadius, Pos2, Response, Sense, Stroke, Ui, Vec2};

/// Wobble card component
///
/// A card that wobbles and jiggles when hovered
pub struct WobbleCard {
    width: f32,
    height: f32,
    background: Color32,
    border: Option<Color32>,
    corner_radius: f32,
    wobble_intensity: f32,
    wobble_speed: f32,

    // Internal state
    is_hovered: bool,
    hover_start_time: f32,
    time: f32,
}

impl WobbleCard {
    /// Create a new wobble card
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            background: Color32::PLACEHOLDER,   // Use theme.card()
            border: Some(Color32::PLACEHOLDER), // Use theme.border()
            corner_radius: 12.0,
            wobble_intensity: 1.0,
            wobble_speed: 8.0,
            is_hovered: false,
            hover_start_time: 0.0,
            time: 0.0,
        }
    }

    /// Set background color
    pub fn background(mut self, color: Color32) -> Self {
        self.background = color;
        self
    }

    /// Set border color
    pub fn border(mut self, color: Option<Color32>) -> Self {
        self.border = color;
        self
    }

    /// Set corner radius
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Set wobble intensity (default: 1.0)
    pub fn wobble_intensity(mut self, intensity: f32) -> Self {
        self.wobble_intensity = intensity.max(0.0);
        self
    }

    /// Set wobble speed (default: 8.0)
    pub fn wobble_speed(mut self, speed: f32) -> Self {
        self.wobble_speed = speed.max(0.1);
        self
    }

    /// Show the wobble card with content
    pub fn show<R>(
        &mut self,
        ui: &mut Ui,
        _theme: &Theme,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> Response {
        let theme = ui.ctx().armas_theme();

        // Use theme colors if not explicitly set
        let background = if self.background == Color32::PLACEHOLDER {
            theme.card()
        } else {
            self.background
        };
        let border = self.border.map(|b| {
            if b == Color32::PLACEHOLDER {
                theme.border()
            } else {
                b
            }
        });

        let dt = ui.input(|i| i.stable_dt);
        self.time += dt;

        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(self.width, self.height),
            Sense::click().union(Sense::hover()),
        );

        // Update hover state
        let was_hovered = self.is_hovered;
        self.is_hovered = response.hovered();

        if self.is_hovered && !was_hovered {
            self.hover_start_time = self.time;
        }

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Calculate wobble transform
            let (offset, rotation) = if self.is_hovered {
                let hover_duration = self.time - self.hover_start_time;
                let wobble_time = hover_duration * self.wobble_speed;

                // Damping factor - wobble decreases over time
                let damping = (-hover_duration * 2.0).exp();

                // Calculate wobble offset using multiple frequencies for natural movement
                let offset_x = (wobble_time * 1.5).sin() * 3.0 * self.wobble_intensity * damping;
                let offset_y = (wobble_time * 2.0).cos() * 2.5 * self.wobble_intensity * damping;

                // Rotation wobble
                let rotation_wobble =
                    (wobble_time * 1.2).sin() * 0.03 * self.wobble_intensity * damping;

                (Vec2::new(offset_x, offset_y), rotation_wobble)
            } else {
                (Vec2::ZERO, 0.0)
            };

            // Apply transforms
            let center = rect.center() + offset;

            // Draw shadow (slightly offset)
            if self.is_hovered {
                let shadow_rect = rect
                    .translate(Vec2::new(theme.spacing.xs / 2.0, theme.spacing.xs / 2.0) + offset);
                painter.rect_filled(
                    shadow_rect,
                    CornerRadius::same(theme.spacing.corner_radius),
                    Color32::from_rgba_unmultiplied(0, 0, 0, 40),
                );
            }

            // Draw card with rotation
            if rotation.abs() > 0.001 {
                // For rotation, we need to draw the card as a rotated shape
                let half_size = rect.size() / 2.0;
                let corners = [
                    Vec2::new(-half_size.x, -half_size.y),
                    Vec2::new(half_size.x, -half_size.y),
                    Vec2::new(half_size.x, half_size.y),
                    Vec2::new(-half_size.x, half_size.y),
                ];

                let rotated_corners: Vec<Pos2> = corners
                    .iter()
                    .map(|&corner| {
                        let cos_r = rotation.cos();
                        let sin_r = rotation.sin();
                        let rotated = Vec2::new(
                            corner.x * cos_r - corner.y * sin_r,
                            corner.x * sin_r + corner.y * cos_r,
                        );
                        center + rotated
                    })
                    .collect();

                // Background
                painter.add(egui::Shape::convex_polygon(
                    rotated_corners.clone(),
                    background,
                    egui::Stroke::NONE,
                ));

                // Border
                if let Some(border_color) = border {
                    painter.add(egui::Shape::closed_line(
                        rotated_corners,
                        Stroke::new(1.0, border_color),
                    ));
                }
            } else {
                // No rotation - draw normally
                let wobble_rect = egui::Rect::from_center_size(center, rect.size());

                painter.rect_filled(
                    wobble_rect,
                    CornerRadius::same(theme.spacing.corner_radius),
                    background,
                );

                if let Some(border_color) = border {
                    painter.rect_stroke(
                        wobble_rect,
                        CornerRadius::same(theme.spacing.corner_radius),
                        Stroke::new(1.0, border_color),
                        egui::StrokeKind::Outside,
                    );
                }
            }

            // Render content
            let content_rect = rect.translate(offset).shrink(theme.spacing.md);
            ui.scope_builder(egui::UiBuilder::new().max_rect(content_rect), |ui| {
                content(ui);
            });

            // Request repaint if wobbling
            if self.is_hovered {
                ui.ctx().request_repaint();
            }
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wobble_card_creation() {
        let card = WobbleCard::new(300.0, 200.0);
        assert_eq!(card.width, 300.0);
        assert_eq!(card.height, 200.0);
        assert_eq!(card.wobble_intensity, 1.0);
    }

    #[test]
    fn test_wobble_card_config() {
        let card = WobbleCard::new(300.0, 200.0)
            .wobble_intensity(1.5)
            .wobble_speed(10.0)
            .corner_radius(16.0);

        assert_eq!(card.wobble_intensity, 1.5);
        assert_eq!(card.wobble_speed, 10.0);
        assert_eq!(card.corner_radius, 16.0);
    }
}
