//! Card Component
//!
//! Material Design card - a surface for displaying grouped content.
//! Features elevation with shadow, rounded corners, and solid background.

use crate::theme::Theme;
use egui::{self, Color32, CornerRadius};

/// Material Design card component
pub struct Card<'a> {
    /// Optional title for the card
    pub title: Option<&'a str>,
    /// Elevation level (0-5, affects shadow intensity)
    pub elevation: u8,
    /// Whether the card is clickable (adds hover effect)
    pub clickable: bool,
    /// Custom width (None = fill available)
    pub width: Option<f32>,
    /// Custom inner margin (None = use theme default)
    pub inner_margin: Option<f32>,
    /// Custom background color (None = use theme default)
    pub fill_color: Option<Color32>,
    /// Custom border color (None = use theme default)
    pub stroke_color: Option<Color32>,
    /// Custom corner radius (None = use theme default)
    pub corner_radius: Option<f32>,
}

impl<'a> Card<'a> {
    /// Create a new card
    pub fn new() -> Self {
        Self {
            title: None,
            elevation: 1,
            clickable: false,
            width: None,
            inner_margin: None,
            fill_color: None,
            stroke_color: None,
            corner_radius: None,
        }
    }

    /// Set the card title
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// Set the elevation level (0-5)
    /// Higher elevation = stronger shadow
    pub fn elevation(mut self, level: u8) -> Self {
        self.elevation = level.min(5);
        self
    }

    /// Make the card clickable (adds hover effect)
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    /// Set custom width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set custom inner margin (overrides theme default)
    pub fn inner_margin(mut self, margin: f32) -> Self {
        self.inner_margin = Some(margin);
        self
    }

    /// Set custom fill/background color (overrides theme default)
    pub fn fill(mut self, color: Color32) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set custom stroke/border color (overrides theme default)
    pub fn stroke(mut self, color: Color32) -> Self {
        self.stroke_color = Some(color);
        self
    }

    /// Set custom corner radius (overrides theme default)
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = Some(radius);
        self
    }

    /// Alias for corner_radius for backwards compatibility
    pub fn rounding(mut self, radius: f32) -> Self {
        self.corner_radius = Some(radius);
        self
    }

    /// Enable hover effect (same as clickable)
    pub fn hover_effect(mut self, enable: bool) -> Self {
        self.clickable = enable;
        self
    }

    /// Set stroke width (overrides elevation-based width)
    pub fn stroke_width(mut self, width: f32) -> Self {
        self.elevation = 0; // Custom width overrides elevation
        self.stroke_color = self.stroke_color.or(Some(Color32::TRANSPARENT));
        self
    }

    /// Show the card with content
    pub fn show<R>(
        self,
        ui: &mut egui::Ui,
        theme: &Theme,
        content: impl FnOnce(&mut egui::Ui) -> R,
    ) -> CardResponse<R> {
        // Border - thicker for higher elevation (simulates shadow without actual shadow)
        let border_width = match self.elevation {
            0 => 0.0,
            1 => 1.0,
            2 => 1.5,
            3 => 2.0,
            4 => 2.5,
            5 => 3.0,
            _ => 1.0,
        };

        let border_color = self.stroke_color.unwrap_or_else(|| {
            if self.elevation > 0 {
                theme.outline()
            } else {
                Color32::TRANSPARENT
            }
        });

        let fill_color = self.fill_color.unwrap_or_else(|| theme.surface());
        let corner_rad = self.corner_radius.unwrap_or(theme.spacing.corner_radius);

        let sense = if self.clickable {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        };

        let inner_margin = self.inner_margin.unwrap_or(theme.spacing.spacing_medium);
        let mut content_result = None;

        // Create a vertical scope to constrain width if specified
        let outer_response = if let Some(width) = self.width {
            ui.vertical(|ui| {
                ui.set_max_width(width);

                let frame_response = egui::Frame::new()
                    .fill(fill_color)
                    .corner_radius(CornerRadius::same(corner_rad as u8))
                    .stroke(egui::Stroke::new(border_width, border_color))
                    .inner_margin(inner_margin)
                    .show(ui, |ui| {
                        // Title if provided
                        if let Some(title) = self.title {
                            ui.label(
                                egui::RichText::new(title)
                                    .size(ui.spacing().interact_size.y * 0.7)
                                    .color(theme.on_surface())
                                    .strong(),
                            );
                            ui.add_space(theme.spacing.spacing_small);
                        }

                        // User content
                        content_result = Some(content(ui));
                    });

                frame_response
            })
            .inner
        } else {
            egui::Frame::new()
                .fill(fill_color)
                .corner_radius(CornerRadius::same(corner_rad as u8))
                .stroke(egui::Stroke::new(border_width, border_color))
                .inner_margin(inner_margin)
                .show(ui, |ui| {
                    // Title if provided
                    if let Some(title) = self.title {
                        ui.label(
                            egui::RichText::new(title)
                                .size(ui.spacing().interact_size.y * 0.7)
                                .color(theme.on_surface())
                                .strong(),
                        );
                        ui.add_space(theme.spacing.spacing_small);
                    }

                    // User content
                    content_result = Some(content(ui));
                })
        };

        // Make the entire frame interactive if clickable
        let rect = outer_response.response.rect;
        let response = if self.clickable {
            ui.interact(rect, ui.id().with("card"), sense)
        } else {
            outer_response.response
        };

        // Apply hover background if clickable and hovered
        if self.clickable && response.hovered() {
            ui.painter()
                .rect_filled(rect, CornerRadius::same(corner_rad as u8), theme.hover());
        }

        CardResponse {
            response,
            inner: content_result.unwrap(),
        }
    }
}

impl<'a> Default for Card<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from showing a card
pub struct CardResponse<R> {
    /// The interaction response for the card
    pub response: egui::Response,
    /// The result from the content closure
    pub inner: R,
}

impl<R> CardResponse<R> {
    /// Whether the card was clicked (if clickable)
    pub fn clicked(&self) -> bool {
        self.response.clicked()
    }

    /// Whether the card is hovered
    pub fn hovered(&self) -> bool {
        self.response.hovered()
    }
}
