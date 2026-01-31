//! Simple button with subtle lift on hover
//!
//! Inspired by Aceternity UI's Simple button style

use egui::{Color32, Response, Sense, Ui, Vec2};

/// Simple button with subtle lift effect
pub struct SimpleButton {
    text: String,
    min_size: Vec2,
    enabled: bool,
    max_width: Option<f32>,
}

impl SimpleButton {
    /// Create a new simple button
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            min_size: Vec2::new(80.0, 36.0),
            enabled: true,
            max_width: None,
        }
    }

    /// Set minimum size
    #[must_use] 
    pub const fn min_size(mut self, size: Vec2) -> Self {
        self.min_size = size;
        self
    }

    /// Set enabled state
    #[must_use] 
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set maximum width
    #[must_use] 
    pub const fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    /// Show the button
    pub fn show(self, ui: &mut Ui, _theme: &crate::Theme) -> Response {
        let Self {
            text,
            min_size,
            enabled,
            max_width,
        } = self;

        // Measure text to calculate button width
        let font_id = egui::FontId::new(14.0, egui::FontFamily::Name("Inter".into()));
        let text_galley =
            ui.painter()
                .layout_no_wrap(text.clone(), font_id, Color32::PLACEHOLDER);
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
                painter.rect_filled(shadow_rect, corner_radius, Color32::from_black_alpha(20));
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
            let font_id = egui::FontId::new(14.0, egui::FontFamily::Name("Inter".into()));
            let available_text_width = lifted_rect.width() - 24.0;
            let final_galley = if text_width > available_text_width {
                painter.layout(text, font_id, text_color, available_text_width)
            } else {
                text_galley
            };
            let text_pos = egui::pos2(
                lifted_rect.center().x - final_galley.size().x / 2.0,
                lifted_rect.center().y - final_galley.size().y / 2.0,
            );
            painter.galley(text_pos, final_galley, text_color);
        }

        response
    }
}
