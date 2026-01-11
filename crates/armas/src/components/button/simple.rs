//! Simple button - Elegant button with subtle lift on hover
//!
//! Inspired by Aceternity UI's Simple button style

use egui::{Color32, Response, Sense, Ui, Vec2};

/// Simple elegant button with subtle lift effect
pub struct SimpleButton {
    text: String,
    min_size: Vec2,
    enabled: bool,
}

impl SimpleButton {
    /// Create a new simple button
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            min_size: Vec2::new(80.0, 36.0),
            enabled: true,
        }
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
        let SimpleButton {
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

            let (bg_color, text_color, border_color) = if response.hovered() {
                (
                    Color32::from_gray(230),
                    Color32::from_gray(140),
                    Color32::from_gray(200),
                )
            } else {
                (
                    Color32::from_gray(241),
                    Color32::from_gray(128),
                    Color32::from_gray(212),
                )
            };

            // Apply lift effect
            let lift_offset = if response.hovered() { -1.0 } else { 0.0 };
            let lifted_rect = rect.translate(Vec2::new(0.0, lift_offset));

            // Draw shadow on hover
            if response.hovered() {
                let shadow_rect = rect.translate(Vec2::new(0.0, 2.0));
                painter.rect_filled(
                    shadow_rect,
                    corner_radius,
                    Color32::from_black_alpha(20),
                );
            }

            // Draw background
            painter.rect_filled(lifted_rect, corner_radius, bg_color);

            // Draw border
            painter.rect_stroke(
                lifted_rect,
                corner_radius,
                egui::Stroke::new(1.0, border_color),
                egui::StrokeKind::Middle,
            );

            // Draw text - Simple uses regular weight
            let font_id = egui::FontId::new(
                14.0,
                egui::FontFamily::Name("Inter".into()),
            );
            painter.text(
                lifted_rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                font_id,
                text_color,
            );
        }

        response
    }
}
