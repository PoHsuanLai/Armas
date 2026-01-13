//! 3D Tilt Card Effect
//!
//! Card that tilts in 3D based on mouse position with glare effect

use crate::animation::SpringAnimation;
use crate::{PainterExt, Theme};
use egui::{Color32, Pos2, Response, Sense, Ui, Vec2};

/// 3D tilt card with mouse-tracking perspective
///
/// Creates a card that tilts based on mouse position, with optional
/// glare effect that moves with the tilt.
#[derive(Clone)]
pub struct TiltCard {
    width: f32,
    height: f32,
    tilt_strength: f32,
    glare_enabled: bool,
    corner_radius: f32,
    background: Color32,
    border_color: Option<Color32>,
    elevation: f32,

    // Internal state - using spring animations for smooth tilt
    mouse_pos: Option<Pos2>,
    tilt_x_spring: SpringAnimation,
    tilt_y_spring: SpringAnimation,
}

impl TiltCard {
    /// Create a new tilt card with theme-based defaults
    pub fn new(width: f32, height: f32, theme: &Theme) -> Self {
        let outline = theme.outline_variant();
        let primary = theme.primary();
        // Use primary color with low alpha over surface for a lighter, themed background
        let surface = theme.surface();
        let background = Color32::from_rgba_unmultiplied(
            ((surface.r() as u16 * 200 + primary.r() as u16 * 55) / 255) as u8,
            ((surface.g() as u16 * 200 + primary.g() as u16 * 55) / 255) as u8,
            ((surface.b() as u16 * 200 + primary.b() as u16 * 55) / 255) as u8,
            255,
        );
        Self {
            width,
            height,
            tilt_strength: 0.15,
            glare_enabled: true,
            corner_radius: 12.0,
            background,
            border_color: Some(Color32::from_rgba_unmultiplied(
                outline.r(),
                outline.g(),
                outline.b(),
                100,
            )),
            elevation: 8.0,
            mouse_pos: None,
            // Smooth spring animations for natural tilt motion
            tilt_x_spring: SpringAnimation::new(0.0, 0.0).params(150.0, 15.0),
            tilt_y_spring: SpringAnimation::new(0.0, 0.0).params(150.0, 15.0),
        }
    }

    /// Set tilt strength (0.0 to 1.0)
    pub fn tilt_strength(mut self, strength: f32) -> Self {
        self.tilt_strength = strength.clamp(0.0, 1.0);
        self
    }

    /// Enable/disable glare effect
    pub fn glare(mut self, enabled: bool) -> Self {
        self.glare_enabled = enabled;
        self
    }

    /// Set corner radius
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Set background color
    pub fn background(mut self, color: Color32) -> Self {
        self.background = color;
        self
    }

    /// Set border color
    pub fn border(mut self, color: Color32) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Set elevation (shadow depth)
    pub fn elevation(mut self, elevation: f32) -> Self {
        self.elevation = elevation;
        self
    }

    /// Show the tilt card with content
    pub fn show<R>(
        &mut self,
        ui: &mut Ui,
        theme: &Theme,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> Response {
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(self.width, self.height), Sense::hover());

        // Update mouse position
        if let Some(hover_pos) = response.hover_pos() {
            self.mouse_pos = Some(hover_pos);
        } else if !response.hovered() {
            self.mouse_pos = None;
        }

        // Calculate tilt targets based on mouse position
        let (target_tilt_x, target_tilt_y) = if let Some(mouse_pos) = self.mouse_pos {
            let center = rect.center();
            let dx = (mouse_pos.x - center.x) / (self.width / 2.0);
            let dy = (mouse_pos.y - center.y) / (self.height / 2.0);
            (
                dy * self.tilt_strength * 20.0,
                -dx * self.tilt_strength * 20.0,
            )
        } else {
            (0.0, 0.0)
        };

        // Update spring animations for smooth, physics-based tilt
        self.tilt_x_spring.set_target(target_tilt_x);
        self.tilt_y_spring.set_target(target_tilt_y);

        let dt = ui.input(|i| i.stable_dt);
        self.tilt_x_spring.update(dt);
        self.tilt_y_spring.update(dt);

        let tilt_x = self.tilt_x_spring.value;
        let tilt_y = self.tilt_y_spring.value;

        let painter = ui.painter();

        // Draw shadow with tilt offset
        let shadow_offset = Vec2::new(-tilt_y * 0.5, tilt_x * 0.5 + self.elevation * 0.5);

        painter.shadow(
            rect,
            self.corner_radius.into(),
            shadow_offset,
            self.elevation,
            Color32::from_black_alpha(100),
        );

        // For now, draw a simple transformed rect (egui doesn't support full 3D transforms)
        // We simulate tilt with offset and shape
        let tilt_offset = Vec2::new(tilt_y * 0.3, -tilt_x * 0.3);
        let tilted_rect = rect.translate(tilt_offset);

        // Draw card background
        painter.rect_filled(tilted_rect, theme.spacing.corner_radius, self.background);

        // Draw border if enabled
        if let Some(border_color) = self.border_color {
            painter.rect_stroke(
                tilted_rect,
                theme.spacing.corner_radius,
                egui::Stroke::new(1.0, border_color),
                egui::epaint::StrokeKind::Middle,
            );
        }

        // Draw glare effect
        if self.glare_enabled && self.mouse_pos.is_some() {
            let glare_strength = (tilt_x.abs() + tilt_y.abs()) / 30.0;
            if glare_strength > 0.01 {
                let glare_center = Pos2::new(
                    tilted_rect.center().x + tilt_y * 3.0,
                    tilted_rect.center().y - tilt_x * 3.0,
                );

                let on_surface = theme.on_surface();
                // Draw multiple glare layers for more visibility
                painter.circle_filled(
                    glare_center,
                    100.0,
                    Color32::from_rgba_unmultiplied(
                        on_surface.r(),
                        on_surface.g(),
                        on_surface.b(),
                        (glare_strength * 50.0) as u8,
                    ),
                );
                painter.circle_filled(
                    glare_center,
                    60.0,
                    Color32::from_rgba_unmultiplied(
                        on_surface.r(),
                        on_surface.g(),
                        on_surface.b(),
                        (glare_strength * 80.0) as u8,
                    ),
                );
            }
        }

        // Draw content in a child UI
        let content_rect = tilted_rect.shrink(theme.spacing.md);
        let mut child_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(content_rect)
                .layout(*ui.layout()),
        );
        content(&mut child_ui);

        // Request repaint for smooth animation (while springs are settling)
        if !self.tilt_x_spring.is_settled(0.01, 0.01) || !self.tilt_y_spring.is_settled(0.01, 0.01)
        {
            ui.ctx().request_repaint();
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tilt_card_creation() {
        let theme = Theme::default();
        let card = TiltCard::new(300.0, 200.0, &theme);
        assert_eq!(card.width, 300.0);
        assert_eq!(card.height, 200.0);
    }

    #[test]
    fn test_tilt_card_config() {
        let theme = Theme::default();
        let card = TiltCard::new(300.0, 200.0, &theme)
            .tilt_strength(0.5)
            .glare(false);

        assert_eq!(card.tilt_strength, 0.5);
        assert!(!card.glare_enabled);
    }
}
