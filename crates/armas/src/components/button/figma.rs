//! Figma button - Simple black button with lift effect
//!
//! Inspired by Aceternity UI's Figma button style

use egui::{Color32, Response, Sense, Ui, Vec2};

/// Figma-style button with subtle lift on hover
pub struct FigmaButton {
    text: String,
    min_size: Vec2,
    enabled: bool,
    outlined: bool,
}

impl FigmaButton {
    /// Create a new Figma button
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            min_size: Vec2::new(80.0, 36.0),
            enabled: true,
            outlined: false,
        }
    }

    /// Create an outlined Figma button
    pub fn outlined(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            min_size: Vec2::new(80.0, 36.0),
            enabled: true,
            outlined: true,
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
        let FigmaButton {
            text,
            min_size,
            enabled,
            outlined,
        } = self;

        let sense = if enabled {
            Sense::click()
        } else {
            Sense::hover()
        };

        let (rect, response) = ui.allocate_exact_size(min_size, sense);

        // Apply lift effect on hover
        let lift_offset = if response.hovered() { -2.0 } else { 0.0 };
        let lifted_rect = rect.translate(Vec2::new(0.0, lift_offset));

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let corner_radius = 8.0;

            if outlined {
                // Outlined version
                painter.rect_stroke(
                    lifted_rect,
                    corner_radius,
                    egui::Stroke::new(3.0, Color32::BLACK),
                    egui::StrokeKind::Middle,
                );

                // Figma uses font-bold (700 weight)
                let font_id = egui::FontId::new(14.0, egui::FontFamily::Name("InterBold".into()));
                painter.text(
                    lifted_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    text,
                    font_id,
                    Color32::BLACK,
                );
            } else {
                // Filled version
                painter.rect_filled(lifted_rect, corner_radius, Color32::BLACK);

                // Figma uses font-bold (700 weight)
                let font_id = egui::FontId::new(14.0, egui::FontFamily::Name("InterBold".into()));
                painter.text(
                    lifted_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    text,
                    font_id,
                    Color32::WHITE,
                );
            }
        }

        response
    }
}
