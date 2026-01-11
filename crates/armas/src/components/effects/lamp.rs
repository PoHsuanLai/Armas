//! Lamp Effect
//!
//! Creates a horizontal lamp beam effect with conic gradients on each side,
//! inspired by Aceternity's Lamp component

use crate::animation::{Animation, EasingFunction};
use crate::effects::{blurred_circle, blurred_rect, BlurAmount, GlowConfig, GlowEffect};
use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{Color32, Pos2, Rect, Response, Ui, Vec2};
use std::f32::consts::PI;

/// Lamp effect component
///
/// Creates a dramatic horizontal lighting effect with symmetric conic gradients,
/// perfect for hero sections and landing pages. The lamp consists of:
/// - Two conic gradient beams (left and right)
/// - Central horizontal line that animates in width
/// - Layered glows for depth
/// - Dark background with masks
pub struct LampEffect {
    width: f32,
    height: f32,
    lamp_color: Color32,
    background_color: Color32,

    // Animation state
    line_width_animation: Animation<f32>,
    glow_width_animation: Animation<f32>,
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
            // Line width: 8rem -> 16rem (128px -> 256px)
            line_width_animation: Animation::new(128.0, 256.0, 0.8).easing(EasingFunction::EaseInOut),
            // Glow width: 15rem -> 30rem (240px -> 480px)
            glow_width_animation: Animation::new(240.0, 480.0, 0.8).easing(EasingFunction::EaseInOut),
            opacity_animation: Animation::new(0.5, 1.0, 0.8).easing(EasingFunction::EaseInOut),
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
        self.line_width_animation.duration = duration;
        self.glow_width_animation.duration = duration;
        self.opacity_animation.duration = duration;
        self
    }

    /// Set animation delay
    pub fn animation_delay(mut self, delay: f32) -> Self {
        // Note: Animation struct would need delay support - for now we use duration
        self
    }

    /// Draw a conic gradient (triangle fan approximation)
    fn draw_conic_gradient(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        height: f32,
        width: f32,
        start_angle: f32,
        end_angle: f32,
        opacity: f32,
        flip: bool,
    ) {
        let segments = 32;
        let angle_range = end_angle - start_angle;

        for i in 0..segments {
            let t1 = i as f32 / segments as f32;
            let t2 = (i + 1) as f32 / segments as f32;

            let angle1 = start_angle + angle_range * t1;
            let angle2 = start_angle + angle_range * t2;

            // Opacity fade from center to edge
            let alpha1 = ((1.0 - t1) * opacity * 255.0) as u8;
            let alpha2 = ((1.0 - t2) * opacity * 255.0) as u8;

            let color1 = Color32::from_rgba_unmultiplied(
                self.lamp_color.r(),
                self.lamp_color.g(),
                self.lamp_color.b(),
                alpha1,
            );
            let color2 = Color32::from_rgba_unmultiplied(
                self.lamp_color.r(),
                self.lamp_color.g(),
                self.lamp_color.b(),
                alpha2,
            );

            // Calculate points - conic gradient radiating from center
            let radius1 = height * (1.0 + t1 * 0.5);
            let radius2 = height * (1.0 + t2 * 0.5);

            let x_offset = if flip { -width / 2.0 } else { width / 2.0 };
            let p1 = center + Vec2::new(x_offset + angle1.cos() * radius1, angle1.sin() * radius1);
            let p2 = center + Vec2::new(x_offset + angle2.cos() * radius2, angle2.sin() * radius2);

            // Draw gradient triangle
            let mesh = egui::epaint::Mesh {
                indices: vec![0, 1, 2],
                vertices: vec![
                    egui::epaint::Vertex {
                        pos: center,
                        uv: egui::pos2(0.0, 0.0),
                        color: self.lamp_color,
                    },
                    egui::epaint::Vertex {
                        pos: p1,
                        uv: egui::pos2(0.0, 0.0),
                        color: color1,
                    },
                    egui::epaint::Vertex {
                        pos: p2,
                        uv: egui::pos2(0.0, 0.0),
                        color: color2,
                    },
                ],
                texture_id: egui::TextureId::default(),
            };

            painter.add(egui::Shape::Mesh(std::sync::Arc::new(mesh)));
        }
    }

    /// Show the lamp effect
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        // Start animation on first show
        if !self.started {
            self.line_width_animation.start();
            self.glow_width_animation.start();
            self.opacity_animation.start();
            self.started = true;
        }

        let dt = ui.input(|i| i.stable_dt);
        self.line_width_animation.update(dt);
        self.glow_width_animation.update(dt);
        self.opacity_animation.update(dt);

        let (response, painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());

        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw dark background
            painter.rect_filled(rect, 0.0, self.background_color);

            // Get animated values
            let current_line_width = self.line_width_animation.value();
            let current_glow_width = self.glow_width_animation.value();
            let current_opacity = self.opacity_animation.value();

            // Center position for lamp (about 1/3 down from top)
            let center = Pos2::new(rect.center().x, rect.top() + self.height * 0.35);
            let lamp_height = 224.0; // 56 * 4 = 224px (h-56 in Tailwind)

            // Draw left conic gradient (from 70deg at center top)
            let left_angle_start = PI * 0.389; // 70deg
            let left_angle_end = PI * 1.111;  // ~200deg
            self.draw_conic_gradient(
                &painter,
                center,
                lamp_height,
                current_glow_width,
                left_angle_start,
                left_angle_end,
                current_opacity * 0.6,
                true,
            );

            // Draw right conic gradient (from 290deg at center top)
            let right_angle_start = PI * 1.611; // 290deg
            let right_angle_end = PI * 0.389 + 2.0 * PI; // 70deg + full circle
            self.draw_conic_gradient(
                &painter,
                center,
                lamp_height,
                current_glow_width,
                right_angle_start,
                right_angle_end,
                current_opacity * 0.6,
                false,
            );

            // Match Aceternity's blur implementation exactly:

            // 1. Background blur rectangle (blur-2xl)
            // <div class="absolute top-1/2 h-48 w-full translate-y-12 scale-x-150 bg-slate-950 blur-2xl">
            let bg_blur_center = Pos2::new(center.x, center.y + 48.0); // translate-y-12 = +48px
            let bg_blur_rect = Rect::from_center_size(
                bg_blur_center,
                Vec2::new(rect.width() * 1.5, 192.0), // h-48 = 192px, scale-x-150
            );
            blurred_rect(&painter, bg_blur_rect, self.background_color, BlurAmount::ExtraLarge2.into());

            // 2. Large blurred circle at center (blur-3xl, opacity-50)
            // <div class="... h-36 w-[28rem] -translate-y-1/2 ... bg-cyan-500 opacity-50 blur-3xl">
            // h-36 = 144px radius, w-[28rem] = 448px width (but for circle we use radius)
            let large_blur_color = Color32::from_rgba_unmultiplied(
                self.lamp_color.r(),
                self.lamp_color.g(),
                self.lamp_color.b(),
                128, // opacity-50 = 0.5 * 255 = 128
            );
            // -translate-y-1/2 means move up by half the element height (72px)
            blurred_circle(
                &painter,
                Pos2::new(center.x, center.y - 72.0),
                224.0, // w-[28rem] / 2 = 448px / 2
                large_blur_color,
                BlurAmount::ExtraLarge3.into(), // blur-3xl = 64px
            );

            // 3. Medium blurred circle (blur-2xl, animated)
            // <div class="... h-36 w-64 -translate-y-[6rem] ... bg-cyan-400 blur-2xl">
            blurred_circle(
                &painter,
                Pos2::new(center.x, center.y - 96.0), // -translate-y-[6rem] = -96px
                current_line_width * 0.5, // w-64 -> animated from 8rem to 16rem
                self.lamp_color,
                BlurAmount::ExtraLarge2.into(), // blur-2xl = 40px
            );

            // Central horizontal line (the actual "lamp")
            let line_y = center.y - 112.0; // -translate-y-[7rem]
            let line_rect = Rect::from_center_size(
                Pos2::new(center.x, line_y),
                Vec2::new(current_line_width, 2.0), // h-0.5 = 2px
            );
            painter.rect_filled(line_rect, 0.0, self.lamp_color);

            // Top mask to hide upper part (z-40)
            // <div class="absolute inset-auto z-40 h-44 w-full -translate-y-[12.5rem] bg-slate-950">
            // h-44 = 176px, -translate-y-[12.5rem] = -200px
            let mask_y = center.y - 200.0;
            let mask_rect = Rect::from_min_size(
                Pos2::new(rect.left(), rect.top()),
                Vec2::new(rect.width(), mask_y - rect.top()),
            );
            painter.rect_filled(mask_rect, 0.0, self.background_color);
        }

        if !self.line_width_animation.is_complete()
            || !self.glow_width_animation.is_complete()
            || !self.opacity_animation.is_complete() {
            ui.ctx().request_repaint();
        }

        response
    }

    /// Show lamp effect with overlaid content
    pub fn show_with_content<R>(
        &mut self,
        ui: &mut Ui,
        theme: &Theme,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> (R, Response) {
        let available = ui.available_rect_before_wrap();

        // Draw lamp effect as background
        let response = ui.allocate_ui_at_rect(available, |ui| {
            self.show(ui, theme)
        }).inner;

        // Overlay content on top
        let content_result = ui.allocate_ui_at_rect(
            Rect::from_center_size(
                Pos2::new(available.center().x, available.bottom() - 320.0),
                Vec2::new(available.width() - 40.0, 400.0),
            ),
            |ui| content(ui),
        ).inner;

        (content_result, response)
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
        assert_eq!(lamp.line_width_animation.duration, 1.0);
    }
}
