//! Invert button - Button that inverts colors on hover
//!
//! Inspired by Aceternity UI's Invert button style

use egui::{Color32, Response, Sense, Ui, Vec2};

/// Button that inverts background and text colors on hover
pub struct InvertButton {
    text: String,
    min_size: Vec2,
    enabled: bool,
    max_width: Option<f32>,
}

impl InvertButton {
    /// Create a new invert button
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            min_size: Vec2::new(100.0, 36.0),
            enabled: true,
            max_width: None,
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

    /// Set maximum width
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    /// Show the button
    pub fn show(self, ui: &mut Ui) -> Response {
        let InvertButton {
            text,
            min_size,
            enabled,
            max_width,
        } = self;

        // Measure text to calculate button width
        let font_id = egui::FontId::new(14.0, egui::FontFamily::Name("InterBold".into()));
        let text_galley =
            ui.painter()
                .layout_no_wrap(text.clone(), font_id.clone(), Color32::PLACEHOLDER);
        let text_width = text_galley.rect.width();
        let mut button_width = text_width + 24.0;
        button_width = button_width.max(min_size.x);

        // Apply max_width if specified
        if let Some(max_w) = max_width {
            button_width = button_width.min(max_w);
        }

        let button_size = Vec2::new(button_width, min_size.y);

        let sense = if enabled {
            Sense::click()
        } else {
            Sense::hover()
        };

        let (rect, response) = ui.allocate_exact_size(button_size, sense);

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
            let font_id = egui::FontId::new(14.0, egui::FontFamily::Name("InterBold".into()));
            let available_text_width = rect.width() - 24.0;
            let final_galley = if text_width > available_text_width {
                painter.layout(text, font_id, text_color, available_text_width)
            } else {
                text_galley
            };
            let text_pos = egui::pos2(
                rect.center().x - final_galley.size().x / 2.0,
                rect.center().y - final_galley.size().y / 2.0,
            );
            painter.galley(text_pos, final_galley, text_color);
        }

        response
    }
}
