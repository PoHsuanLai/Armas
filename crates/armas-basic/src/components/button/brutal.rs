//! Brutal button - Brutalist design with stacked shadows
//!
//! Inspired by Aceternity UI's Brutal button style

use egui::{Color32, Response, Sense, Ui, Vec2};

/// Brutalist button with multiple stacked shadows
pub struct BrutalButton {
    text: String,
    min_size: Vec2,
    enabled: bool,
    max_width: Option<f32>,
}

impl BrutalButton {
    /// Create a new brutal button
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            min_size: Vec2::new(80.0, 32.0),
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
    pub fn show(self, ui: &mut Ui, _theme: &crate::Theme) -> Response {
        let BrutalButton {
            text,
            min_size,
            enabled,
            max_width,
        } = self;

        // Measure text to calculate button width (using uppercase version)
        let display_text = text.to_uppercase();
        let font_id = egui::FontId::new(13.0, egui::FontFamily::Name("Inter".into()));
        let text_galley = ui.painter().layout_no_wrap(
            display_text.clone(),
            font_id.clone(),
            Color32::PLACEHOLDER,
        );
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

            // Draw stacked shadows (brutal aesthetic)
            for i in 1..=5 {
                let shadow_rect = rect.translate(Vec2::new(i as f32, i as f32));
                painter.rect_stroke(
                    shadow_rect,
                    0.0,
                    egui::Stroke::new(1.0, Color32::BLACK),
                    egui::StrokeKind::Middle,
                );
            }

            // Draw main button
            painter.rect_filled(rect, 0.0, Color32::WHITE);

            // Draw border
            painter.rect_stroke(
                rect,
                0.0,
                egui::Stroke::new(2.0, Color32::BLACK),
                egui::StrokeKind::Middle,
            );

            // Draw text - Brutal uses regular weight, uppercase, smaller size
            let font_id = egui::FontId::new(13.0, egui::FontFamily::Name("Inter".into()));
            let text_color = Color32::from_gray(60);
            let available_text_width = rect.width() - 24.0;
            let final_galley = if text_width > available_text_width {
                painter.layout(display_text, font_id, text_color, available_text_width)
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
