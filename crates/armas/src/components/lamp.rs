//! Lamp Effect
//!
//! Creates an animated lamp lighting effect with conic gradients and glow

use crate::animation::{Animation, EasingFunction};
use crate::Theme;
use egui::{Color32, Pos2, Response, Ui, Vec2};
use std::f32::consts::PI;

/// Lamp effect component
///
/// Creates a dramatic lighting effect with animated conic gradients,
/// perfect for section headers and hero sections
pub struct LampEffect {
    width: f32,
    height: f32,
    lamp_color: Color32,
    background_color: Color32,

    // Animation state
    width_animation: Animation<f32>,
    opacity_animation: Animation<f32>,
    started: bool,
}

impl LampEffect {
    /// Create a new lamp effect
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            lamp_color: Color32::from_rgb(6, 182, 212), // cyan-500
            background_color: Color32::from_rgb(2, 6, 23), // slate-950
            width_animation: Animation::new(240.0, 480.0, 0.8).with_easing(EasingFunction::EaseOut),
            opacity_animation: Animation::new(0.5, 1.0, 0.8).with_easing(EasingFunction::EaseOut),
            started: false,
        }
    }

    /// Set the lamp color
    pub fn lamp_color(mut self, color: Color32) -> Self {
        self.lamp_color = color;
        self
    }

    /// Set the background color
    pub fn background_color(mut self, color: Color32) -> Self {
        self.background_color = color;
        self
    }

    /// Set animation duration
    pub fn animation_duration(mut self, duration: f32) -> Self {
        self.width_animation.duration = duration;
        self.opacity_animation.duration = duration;
        self
    }

    /// Draw a conic gradient approximation
    fn draw_conic_gradient(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        opacity: f32,
    ) {
        let segments = 32;
        let angle_range = end_angle - start_angle;

        for i in 0..segments {
            let t1 = i as f32 / segments as f32;
            let t2 = (i + 1) as f32 / segments as f32;

            let angle1 = start_angle + angle_range * t1;
            let angle2 = start_angle + angle_range * t2;

            // Calculate opacity fade across the gradient
            let alpha = ((1.0 - t1.abs()) * opacity * 255.0) as u8;
            let color = Color32::from_rgba_unmultiplied(
                self.lamp_color.r(),
                self.lamp_color.g(),
                self.lamp_color.b(),
                alpha,
            );

            // Create triangle fan from center
            let p1 = center + Vec2::new(angle1.cos(), angle1.sin()) * radius;
            let p2 = center + Vec2::new(angle2.cos(), angle2.sin()) * radius;

            painter.add(egui::Shape::convex_polygon(
                vec![center, p1, p2],
                color,
                egui::Stroke::NONE,
            ));
        }
    }

    /// Show the lamp effect
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        // Start animation on first show
        if !self.started {
            self.width_animation.start();
            self.opacity_animation.start();
            self.started = true;
        }

        let dt = ui.input(|i| i.stable_dt);
        self.width_animation.update(dt);
        self.opacity_animation.update(dt);

        let (response, painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());

        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw dark background
            painter.rect_filled(rect, 0.0, self.background_color);

            // Center point for lamp effect
            let center = Pos2::new(rect.center().x, rect.top() + self.height * 0.3);

            // Get animated values
            let current_radius = self.width_animation.value();
            let current_opacity = self.opacity_animation.value();

            // Draw multiple blur layers for depth
            for i in 0..5 {
                let layer_radius = current_radius + i as f32 * 20.0;
                let layer_opacity = current_opacity * (1.0 - i as f32 * 0.15);

                // Left conic gradient (70 degrees = ~1.22 radians)
                let left_start = PI * 0.6;
                let left_end = PI * 1.4;
                self.draw_conic_gradient(
                    &painter,
                    center,
                    layer_radius,
                    left_start,
                    left_end,
                    layer_opacity * 0.3,
                );

                // Right conic gradient (290 degrees = ~5.06 radians)
                let right_start = PI * 1.4;
                let right_end = PI * 2.0 + PI * 0.6;
                self.draw_conic_gradient(
                    &painter,
                    center,
                    layer_radius,
                    right_start,
                    right_end,
                    layer_opacity * 0.3,
                );
            }

            // Draw central glow (core of lamp)
            let glow_center = Pos2::new(center.x, center.y - 96.0);
            for i in 0..8 {
                let glow_radius = 40.0 - i as f32 * 4.0;
                let glow_alpha = ((current_opacity * 0.4 * (1.0 - i as f32 / 8.0)) * 255.0) as u8;
                let glow_color = Color32::from_rgba_unmultiplied(
                    self.lamp_color.r(),
                    self.lamp_color.g(),
                    self.lamp_color.b(),
                    glow_alpha,
                );
                painter.circle_filled(glow_center, glow_radius, glow_color);
            }

            // Draw bright center point
            painter.circle_filled(
                glow_center,
                6.0,
                Color32::from_rgba_unmultiplied(255, 255, 255, (current_opacity * 255.0) as u8),
            );
        }

        if !self.width_animation.is_complete() || !self.opacity_animation.is_complete() {
            ui.ctx().request_repaint();
        }

        response
    }

    /// Show lamp effect with overlaid content
    pub fn show_with_content<R>(
        &mut self,
        ui: &mut Ui,
        _theme: &Theme,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> Response {
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(self.width, self.height), egui::Sense::hover());

        // Draw lamp effect first
        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            self.show(ui);
        });

        // Overlay content on top
        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            content(ui);
        });

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lamp_creation() {
        let lamp = LampEffect::new(800.0, 600.0);
        assert_eq!(lamp.width, 800.0);
        assert_eq!(lamp.height, 600.0);
    }

    #[test]
    fn test_lamp_config() {
        let lamp = LampEffect::new(800.0, 600.0)
            .lamp_color(Color32::RED)
            .animation_duration(1.0);

        assert_eq!(lamp.lamp_color, Color32::RED);
        assert_eq!(lamp.width_animation.duration, 1.0);
    }
}
