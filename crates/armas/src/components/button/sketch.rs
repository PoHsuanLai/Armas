//! Sketch button - Button with shadow offset effect on hover
//!
//! Inspired by Aceternity UI's Sketch button style

use egui::{Color32, Response, Sense, Ui, Vec2};

/// Sketch button with offset shadow on hover
pub struct SketchButton {
    text: String,
    min_size: Vec2,
    enabled: bool,
}

impl SketchButton {
    /// Create a new sketch button
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
        let SketchButton {
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

            let bg_color = Color32::WHITE;
            let text_color = Color32::from_gray(60);
            let border_color = Color32::BLACK;

            // Draw offset shadow on hover
            if response.hovered() {
                let shadow_rect = rect.translate(Vec2::new(4.0, 4.0));
                painter.rect_filled(shadow_rect, corner_radius, Color32::BLACK);
            }

            // Draw background
            painter.rect_filled(rect, corner_radius, bg_color);

            // Draw border
            painter.rect_stroke(
                rect,
                corner_radius,
                egui::Stroke::new(1.0, border_color),
                egui::StrokeKind::Middle,
            );

            // Draw text - Sketch uses regular weight
            let font_id = egui::FontId::new(14.0, egui::FontFamily::Name("Inter".into()));
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                font_id,
                text_color,
            );
        }

        response
    }
}
