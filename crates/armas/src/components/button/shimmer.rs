//! Shimmer button - Button with animated shimmer effect
//!
//! Inspired by Aceternity UI's Shimmer button style

use egui::{Color32, Response, Sense, Ui, Vec2};

/// Button with animated shimmer background effect
pub struct ShimmerButton {
    text: String,
    min_size: Vec2,
    enabled: bool,
}

impl ShimmerButton {
    /// Create a new shimmer button
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            min_size: Vec2::new(100.0, 48.0),
            enabled: true,
        }
    }

    /// Linear interpolation between two colors
    fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
        Color32::from_rgb(
            (a.r() as f32 + t * (b.r() as f32 - a.r() as f32)) as u8,
            (a.g() as f32 + t * (b.g() as f32 - a.g() as f32)) as u8,
            (a.b() as f32 + t * (b.b() as f32 - a.b() as f32)) as u8,
        )
    }

    /// Set minimum size
    pub fn min_size(mut self, size: Vec2) -> Self {
        self.min_size = size;
        self
    }

    /// Set enabled state
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Show the button
    pub fn show(self, ui: &mut Ui) -> Response {
        let ShimmerButton {
            text,
            min_size,
            enabled,
        } = self;

        let sense = if enabled {
            Sense::click()
        } else {
            Sense::hover()
        };

        let (rect, response) = ui.allocate_exact_size(min_size, sense);

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let corner_radius = 6.0;

            // Animated shimmer effect
            // CSS: bg-[length:200%_100%] means the gradient is 200% wide (2x button width)
            // Animation: backgroundPosition goes from "0 0" to "-200% 0" over 2s
            // This slides the gradient from right to left
            let time = ui.input(|i| i.time) as f32;

            // Animation progress: 0.0 to 1.0 over 2 seconds, repeating
            let anim_progress = (time / 2.0).fract();

            // Background position goes from 0 to -200% (meaning gradient slides left)
            // At progress=0: gradient starts at position 0
            // At progress=1: gradient has moved -200% (2x button width to the left)
            let bg_position = -anim_progress * 2.0 * rect.width();

            // Colors from the original: linear-gradient(110deg, #000103 0%, #1e2631 45%, #1e2631 55%, #000103 100%)
            let color_dark = Color32::from_rgb(0, 1, 3); // #000103
            let color_light = Color32::from_rgb(30, 38, 49); // #1e2631

            // Draw the gradient as a series of vertical strips
            // The gradient is 200% wide (2x button width)
            let gradient_width = rect.width() * 2.0;
            let num_strips = rect.width() as i32;

            for i in 0..num_strips {
                let x = rect.min.x + i as f32;

                // Position within the button (0 to button_width)
                let button_x = i as f32;

                // Position within the gradient (accounting for background position offset)
                let gradient_x = button_x - bg_position;

                // Normalize to 0.0-1.0 across the gradient width
                let grad_pos = gradient_x / gradient_width;

                // Apply gradient stops: dark at edges (0%, 100%), light in middle (45%-55%)
                let color = if !(0.0..=1.0).contains(&grad_pos) {
                    color_dark
                } else if grad_pos < 0.45 {
                    // Fade from dark to light (0% to 45%)
                    let t = grad_pos / 0.45;
                    Self::lerp_color(color_dark, color_light, t)
                } else if grad_pos < 0.55 {
                    // Stay light (45% to 55%)
                    color_light
                } else {
                    // Fade from light to dark (55% to 100%)
                    let t = (grad_pos - 0.55) / 0.45;
                    Self::lerp_color(color_light, color_dark, t)
                };

                painter.line_segment(
                    [
                        egui::Pos2::new(x, rect.min.y),
                        egui::Pos2::new(x, rect.max.y),
                    ],
                    egui::Stroke::new(1.0, color),
                );
            }

            // Border
            painter.rect_stroke(
                rect,
                corner_radius,
                egui::Stroke::new(1.0, Color32::from_gray(51)), // slate-800
                egui::StrokeKind::Middle,
            );

            // Draw text - Shimmer uses font-medium (500 weight)
            let font_id = egui::FontId::new(14.0, egui::FontFamily::Name("InterMedium".into()));
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                font_id,
                Color32::from_gray(148), // slate-400
            );

            // Request repaint for animation
            ui.ctx().request_repaint();
        }

        response
    }
}
