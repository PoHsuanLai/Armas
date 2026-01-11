//! Invert button - Button that inverts colors on hover
//!
//! Inspired by Aceternity UI's Invert button style

use egui::{Color32, Response, Sense, Ui, Vec2};

/// Button that inverts background and text colors on hover
pub struct InvertButton {
    text: String,
    min_size: Vec2,
    enabled: bool,
}

impl InvertButton {
    /// Create a new invert button
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            min_size: Vec2::new(100.0, 36.0),
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
        let InvertButton {
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

            // Teal accent color
            let teal = Color32::from_rgb(20, 184, 166);

            let (bg_color, text_color, border_color) = if response.hovered() {
                // Inverted: white bg, black text
                (Color32::WHITE, Color32::BLACK, teal)
            } else {
                // Normal: teal bg, white text
                (teal, Color32::WHITE, teal)
            };

            // Draw background
            painter.rect_filled(rect, corner_radius, bg_color);

            // Draw border
            painter.rect_stroke(
                rect,
                corner_radius,
                egui::Stroke::new(2.0, border_color),
                egui::StrokeKind::Middle,
            );

            // Draw text - Invert uses font-bold (700 weight)
            let font_id = egui::FontId::new(
                14.0,
                egui::FontFamily::Name("InterBold".into()),
            );
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
