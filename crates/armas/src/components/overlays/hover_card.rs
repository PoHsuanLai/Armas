//! Hover Card Effect
//!
//! Card with content reveal on hover with smooth transitions

use crate::ext::ArmasContextExt;
use crate::animation::{Interpolate, SpringAnimation};
use crate::context::ArmasContextExt;
use crate::effects::{GlowConfig, GlowEffect, ShadowEffect};
use crate::effects::presets::ShadowPresets;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Hover card with content reveal
///
/// Displays a card that reveals additional content when hovered,
/// with smooth scale and opacity transitions.
pub struct HoverCard {
    width: Option<f32>,
    height: Option<f32>,
    corner_radius: f32,
    background: Option<Color32>,
    hover_background: Option<Color32>,
    border_color: Option<Color32>,
    scale_on_hover: f32,
    elevation: f32,

    // Internal state - using spring animation for smooth transitions
    hover_spring: SpringAnimation,
}

impl HoverCard {
    /// Create a new hover card
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            corner_radius: 12.0,
            background: None,
            hover_background: None,
            border_color: None,
            scale_on_hover: 1.05,
            elevation: 4.0,
            hover_spring: SpringAnimation::new(0.0, 0.0).params(250.0, 25.0),
        }
    }

    /// Set width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set height
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set corner radius
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Set background color
    pub fn background(mut self, color: Color32) -> Self {
        self.background = Some(color);
        self
    }

    /// Set hover background color
    pub fn hover_background(mut self, color: Color32) -> Self {
        self.hover_background = Some(color);
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
        base_content: impl FnOnce(&mut Ui, f32) -> R,
        hover_content: impl FnOnce(&mut Ui, f32) -> R,
    ) -> Response {
        let theme = ui.ctx().armas_theme();

        // Apply theme defaults
        let width = self.width.unwrap_or(300.0);
        let height = self.height.unwrap_or(200.0);
        let background = self.background.unwrap_or_else(|| theme.surface());
        let hover_background = self.hover_background.unwrap_or_else(|| theme.surface_variant());
        let border_color = self.border_color.or_else(|| {
            let outline = theme.outline_variant();
            Some(Color32::from_rgba_unmultiplied(
                outline.r(),
                outline.g(),
                outline.b(),
                80,
            ))
        });

        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

        // Update hover spring animation
        // Use rect_contains_pointer to ensure hover is detected even over child widgets
        let is_hovered = ui.rect_contains_pointer(rect);
        let target = if is_hovered { 1.0 } else { 0.0 };
        self.hover_spring.set_target(target);

        let dt = ui.input(|i| i.stable_dt);
        self.hover_spring.update(dt);

        // Get smooth transition value
        let t = self.hover_spring.value;

        // Calculate scale and position
        let current_scale = 1.0 + (self.scale_on_hover - 1.0) * t;
        let scaled_width = width * current_scale;
        let scaled_height = height * current_scale;
        let offset_x = (scaled_width - width) / 2.0;
        let offset_y = (scaled_height - height) / 2.0;

        let scaled_rect = Rect::from_min_size(
            Pos2::new(rect.min.x - offset_x, rect.min.y - offset_y),
            Vec2::new(scaled_width, scaled_height),
        );

        let painter = ui.painter();

        // Draw shadow with dynamic elevation using unified ShadowEffect
        let shadow_elevation = self.elevation * (1.0 + t * 2.0);
        let shadow = ShadowEffect::new(ShadowPresets::card());
        shadow.render_with_elevation(&painter, scaled_rect, self.corner_radius, shadow_elevation);

        // Interpolate background color using animation system
        let current_bg = Color32::interpolate(background, hover_background, t);

        // Draw card background
        painter.rect_filled(scaled_rect, self.corner_radius, current_bg);

        // Draw border if enabled
        if let Some(border_col) = border_color {
            let border_alpha = (border_col.a() as f32 * (1.0 + t * 0.5)) as u8;
            let current_border = Color32::from_rgba_unmultiplied(
                border_col.r(),
                border_col.g(),
                border_col.b(),
                border_alpha,
            );

            painter.rect_stroke(
                scaled_rect,
                self.corner_radius,
                egui::Stroke::new(1.0 + t, current_border),
                egui::epaint::StrokeKind::Middle,
            );
        }

        // Draw accent glow on hover using unified GlowEffect
        if t > 0.01 {
            let glow_alpha = (t * 30.0) as u8;
            let glow_color = Color32::from_rgba_unmultiplied(
                theme.primary().r(),
                theme.primary().g(),
                theme.primary().b(),
                glow_alpha,
            );

            let glow = GlowEffect::new(
                GlowConfig::new(glow_color)
                    .layers(4)
                    .expansion(2.0)
                    .intensity(t),
            );
            glow.render_filled_rect(&painter, scaled_rect, self.corner_radius);
        }

        // Draw base content
        let base_rect = scaled_rect.shrink(16.0);
        let mut base_ui = ui.child_ui(base_rect, *ui.layout(), None);
        base_content(&mut base_ui, 1.0 - t);

        // Draw hover content with fade-in
        if t > 0.01 {
            let hover_rect = scaled_rect.shrink(16.0);
            let mut hover_ui = ui.child_ui(hover_rect, *ui.layout(), None);
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
        let card = HoverCard::new().width(300.0).height(200.0);
        assert_eq!(card.width, Some(300.0));
        assert_eq!(card.height, Some(200.0));
        assert_eq!(card.hover_spring.value, 0.0);
    }

    #[test]
    fn test_hover_card_config() {
        let card = HoverCard::new()
            .width(300.0)
            .height(200.0)
            .scale(1.1)
            .elevation(8.0);

        assert_eq!(card.scale_on_hover, 1.1);
        assert_eq!(card.elevation, 8.0);
    }
}
