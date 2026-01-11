//! Spotify button - Spotify-style green button
//!
//! Inspired by Aceternity UI's Spotify button style

use egui::{Color32, Response, Sense, Ui, Vec2};

/// Spotify-style button with brand green color
pub struct SpotifyButton {
    text: String,
    min_size: Vec2,
    enabled: bool,
}

impl SpotifyButton {
    /// Create a new Spotify button
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            min_size: Vec2::new(120.0, 48.0),
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
        let SpotifyButton {
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
            let font_id = egui::FontId::new(
                16.0, // Slightly larger for Spotify
                egui::FontFamily::Name("InterBold".into()),
            );
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                text.to_uppercase(),
                font_id,
                Color32::WHITE,
            );
        }

        response
    }
}
