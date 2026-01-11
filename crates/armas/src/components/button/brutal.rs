//! Brutal button - Brutalist design with stacked shadows
//!
//! Inspired by Aceternity UI's Brutal button style

use egui::{Color32, Response, Sense, Ui, Vec2};

/// Brutalist button with multiple stacked shadows
pub struct BrutalButton {
    text: String,
    min_size: Vec2,
    enabled: bool,
}

impl BrutalButton {
    /// Create a new brutal button
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            min_size: Vec2::new(80.0, 32.0),
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
        let BrutalButton {
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
            let font_id = egui::FontId::new(
                13.0,
                egui::FontFamily::Name("Inter".into()),
            );
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                text.to_uppercase(),
                font_id,
                Color32::from_gray(60),
            );
        }

        response
    }
}
