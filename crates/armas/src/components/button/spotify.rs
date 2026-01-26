//! Spotify button - Spotify-style green button
//!
//! Inspired by Aceternity UI's Spotify button style

use egui::{Color32, Response, Sense, Ui, Vec2};

/// Spotify-style button with brand green color
pub struct SpotifyButton {
    text: String,
    min_size: Vec2,
    enabled: bool,
    max_width: Option<f32>,
}

impl SpotifyButton {
    /// Create a new Spotify button
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            min_size: Vec2::new(120.0, 48.0),
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
        let SpotifyButton {
            text,
            min_size,
            enabled,
            max_width,
        } = self;

        // Measure text to calculate button width (using uppercase version)
        let display_text = text.to_uppercase();
        let font_id = egui::FontId::new(16.0, egui::FontFamily::Name("InterBold".into()));
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
            let corner_radius = min_size.y / 2.0; // Fully rounded

            // Spotify brand colors
            let spotify_green = Color32::from_rgb(30, 215, 96);
            let spotify_green_hover = Color32::from_rgb(33, 224, 101);

            let bg_color = if response.hovered() {
                spotify_green_hover
            } else {
                spotify_green
            };

            // Draw background
            painter.rect_filled(rect, corner_radius, bg_color);

            // Draw text - Spotify uses font-bold (700 weight), uppercase, tracking-widest
            let font_id = egui::FontId::new(16.0, egui::FontFamily::Name("InterBold".into()));
            let available_text_width = rect.width() - 24.0;
            let final_galley = if text_width > available_text_width {
                painter.layout(display_text, font_id, Color32::WHITE, available_text_width)
            } else {
                text_galley
            };
            let text_pos = egui::pos2(
                rect.center().x - final_galley.size().x / 2.0,
                rect.center().y - final_galley.size().y / 2.0,
            );
            painter.galley(text_pos, final_galley, Color32::WHITE);
        }

        response
    }
}
