//! Hover Card Effect
//!
//! Card with content reveal on hover with smooth transitions

use crate::animation::{Interpolate, SpringAnimation};
use crate::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Hover card with content reveal
///
/// Displays a card that reveals additional content when hovered,
/// with smooth scale and opacity transitions.
pub struct HoverCard {
    width: f32,
    height: f32,
    corner_radius: f32,
    background: Color32,
    hover_background: Color32,
    border_color: Option<Color32>,
    scale_on_hover: f32,
    elevation: f32,

    // Internal state - using spring animation for smooth transitions
    hover_spring: SpringAnimation,
}

impl HoverCard {
    /// Create a new hover card with theme-based defaults
    pub fn new(width: f32, height: f32, theme: &Theme) -> Self {
        let outline = theme.outline_variant();
        Self {
            width,
            height,
            corner_radius: 12.0,
            background: theme.surface(),
            hover_background: theme.surface_variant(),
            border_color: Some(Color32::from_rgba_unmultiplied(
                outline.r(),
                outline.g(),
                outline.b(),
                80,
            )),
            scale_on_hover: 1.05,
            elevation: 4.0,
            hover_spring: SpringAnimation::new(0.0, 0.0).params(250.0, 25.0),
        }
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

    /// Set hover background color
    pub fn hover_background(mut self, color: Color32) -> Self {
        self.hover_background = color;
        self
    }

    /// Set border color
    pub fn border(mut self, color: Color32) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Set scale multiplier on hover
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale_on_hover = scale.max(1.0);
        self
    }

    /// Set elevation (shadow depth)
    pub fn elevation(mut self, elevation: f32) -> Self {
        self.elevation = elevation;
        self
    }

    /// Show the hover card with two content layers
    pub fn show<R>(
        &mut self,
        ui: &mut Ui,
        theme: &Theme,
        base_content: impl FnOnce(&mut Ui, f32) -> R,
        hover_content: impl FnOnce(&mut Ui, f32) -> R,
    ) -> Response {
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(self.width, self.height), Sense::hover());

        // Update hover spring animation
        let target = if response.hovered() { 1.0 } else { 0.0 };
        self.hover_spring.set_target(target);

        let dt = ui.input(|i| i.stable_dt);
        self.hover_spring.update(dt);

        // Get smooth transition value
        let t = self.hover_spring.value;

        // Calculate scale and position
        let current_scale = 1.0 + (self.scale_on_hover - 1.0) * t;
        let scaled_width = self.width * current_scale;
        let scaled_height = self.height * current_scale;
        let offset_x = (scaled_width - self.width) / 2.0;
        let offset_y = (scaled_height - self.height) / 2.0;

        let scaled_rect = Rect::from_min_size(
            Pos2::new(rect.min.x - offset_x, rect.min.y - offset_y),
            Vec2::new(scaled_width, scaled_height),
        );

        let painter = ui.painter();

        // Draw shadow with dynamic elevation
        let shadow_elevation = self.elevation * (1.0 + t * 2.0);
        let shadow_offset = Vec2::new(0.0, shadow_elevation * 0.4);

        painter.rect_filled(
            scaled_rect.translate(shadow_offset),
            theme.spacing.corner_radius,
            Color32::from_black_alpha((50.0 + t * 50.0) as u8),
        );

        // Interpolate background color using animation system
        let current_bg = self.background.interpolate(&self.hover_background, t);

        // Draw card background
        painter.rect_filled(scaled_rect, theme.spacing.corner_radius, current_bg);

        // Draw border if enabled
        if let Some(border_color) = self.border_color {
            let border_alpha = (border_color.a() as f32 * (1.0 + t * 0.5)) as u8;
            let current_border = Color32::from_rgba_unmultiplied(
                border_color.r(),
                border_color.g(),
                border_color.b(),
                border_alpha,
            );

            painter.rect_stroke(
                scaled_rect,
                theme.spacing.corner_radius,
                egui::Stroke::new(1.0 + t, current_border),
                egui::epaint::StrokeKind::Middle,
            );
        }

        // Draw accent glow on hover
        if t > 0.01 {
            let glow_alpha = (t * 30.0) as u8;
            let glow_color = Color32::from_rgba_unmultiplied(
                theme.primary().r(),
                theme.primary().g(),
                theme.primary().b(),
                glow_alpha,
            );

            painter.rect_filled(
                scaled_rect.expand(theme.spacing.xs / 2.0),
                theme.spacing.corner_radius as f32 + theme.spacing.xs / 2.0,
                glow_color,
            );
        }

        // Draw base content
        let base_rect = scaled_rect.shrink(theme.spacing.md);
        let mut base_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(base_rect)
                .layout(*ui.layout()),
        );
        base_content(&mut base_ui, 1.0 - t);

        // Draw hover content with fade-in
        if t > 0.01 {
            let hover_rect = scaled_rect.shrink(theme.spacing.md);
            let mut hover_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(hover_rect)
                    .layout(*ui.layout()),
            );
            hover_content(&mut hover_ui, t);
        }

        // Request repaint during transitions
        if !self.hover_spring.is_settled(0.001, 0.001) {
            ui.ctx().request_repaint();
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hover_card_creation() {
        let theme = Theme::default();
        let card = HoverCard::new(300.0, 200.0, &theme);
        assert_eq!(card.width, 300.0);
        assert_eq!(card.height, 200.0);
        assert_eq!(card.hover_spring.value, 0.0);
    }

    #[test]
    fn test_hover_card_config() {
        let theme = Theme::default();
        let card = HoverCard::new(300.0, 200.0, &theme)
            .scale(1.1)
            .elevation(8.0);

        assert_eq!(card.scale_on_hover, 1.1);
        assert_eq!(card.elevation, 8.0);
    }
}
