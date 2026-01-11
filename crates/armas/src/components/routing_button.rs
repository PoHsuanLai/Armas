//! Routing Button Component
//!
//! Input/Output routing buttons (e.g., "Input L+R", "Main")
//! Matches Studio One's I/O display.

use crate::ext::ArmasContextExt;
use egui;

/// Routing button component
pub struct RoutingButton<'a> {
    pub label: &'a str,
    pub width: f32,
    pub height: f32,
    pub is_output: bool, // Output buttons are slightly lighter
}

impl<'a> RoutingButton<'a> {
    pub fn input(label: &'a str, width: f32, height: f32) -> Self {
        Self {
            label,
            width,
            height,
            is_output: false,
        }
    }

    pub fn output(label: &'a str, width: f32, height: f32) -> Self {
        Self {
            label,
            width,
            height,
            is_output: true,
        }
    }

    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = ui.ctx().armas_theme();
        let font_size = ui.spacing().interact_size.y * 0.4;
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(self.width, self.height), egui::Sense::click());

        // Background color (output slightly lighter)
        let bg_color = if self.is_output {
            theme.surface()
        } else {
            theme.surface().gamma_multiply(0.9)
        };

        ui.painter()
            .rect_filled(rect, theme.spacing.corner_radius * 0.5, bg_color);

        ui.painter().rect_stroke(
            rect,
            theme.spacing.corner_radius * 0.5,
            egui::Stroke::new(1.0, theme.outline_variant()),
            egui::StrokeKind::Middle,
        );

        ui.painter().text(
            rect.left_center() + egui::vec2(theme.spacing.spacing_small, 0.0),
            egui::Align2::LEFT_CENTER,
            self.label,
            egui::FontId::proportional(font_size),
            theme.on_surface(),
        );

        response
    }
}
