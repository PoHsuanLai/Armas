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
    max_width: Option<f32>,
}

impl FigmaButton {
    /// Create a new Figma button
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            min_size: Vec2::new(80.0, 36.0),
            enabled: true,
            outlined: false,
            max_width: None,
        }
    }

    /// Create an outlined Figma button
    pub fn outlined(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            min_size: Vec2::new(80.0, 36.0),
            enabled: true,
            outlined: true,
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
        let FigmaButton {
            text,
            min_size,
            enabled,
            outlined,
            max_width,
        } = self;

        // Measure text to calculate button width
        let font_id = egui::FontId::new(14.0, egui::FontFamily::Name("InterBold".into()));
        let text_galley = ui.painter().layout_no_wrap(
            text.clone(),
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

        // Apply lift effect on hover
        let lift_offset = if response.hovered() { -2.0 } else { 0.0 };
        let lifted_rect = rect.translate(Vec2::new(0.0, lift_offset));

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let corner_radius = 8.0;

            let font_id = egui::FontId::new(14.0, egui::FontFamily::Name("InterBold".into()));
            let available_text_width = lifted_rect.width() - 24.0;

            if outlined {
                // Outlined version
                painter.rect_stroke(
                    lifted_rect,
                    corner_radius,
                    egui::Stroke::new(3.0, Color32::BLACK),
                    egui::StrokeKind::Middle,
                );

                // Figma uses font-bold (700 weight)
                let final_galley = if text_width > available_text_width {
                    painter.layout(text, font_id, Color32::BLACK, available_text_width)
                } else {
                    text_galley
                };
                let text_pos = egui::pos2(
                    lifted_rect.center().x - final_galley.size().x / 2.0,
                    lifted_rect.center().y - final_galley.size().y / 2.0,
                );
                painter.galley(text_pos, final_galley, Color32::BLACK);
            } else {
                // Filled version
                painter.rect_filled(lifted_rect, corner_radius, Color32::BLACK);

                // Figma uses font-bold (700 weight)
                let final_galley = if text_width > available_text_width {
                    painter.layout(text, font_id, Color32::WHITE, available_text_width)
                } else {
                    text_galley
                };
                let text_pos = egui::pos2(
                    lifted_rect.center().x - final_galley.size().x / 2.0,
                    lifted_rect.center().y - final_galley.size().y / 2.0,
                );
                painter.galley(text_pos, final_galley, Color32::WHITE);
            }
        }

        response
    }
}
