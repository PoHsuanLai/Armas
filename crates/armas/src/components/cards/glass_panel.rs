//! Glass Panel Component
//!
//! Glassmorphic panel with translucent background and border glow.
//! Popular in modern UI design (iOS, Windows 11, etc.)

use crate::theme::Theme;
use egui::{self, CornerRadius};

/// Glassmorphic panel component
pub struct GlassPanel<'a> {
    /// Optional title for the panel
    pub title: Option<&'a str>,
    /// Border glow intensity (0.0 = none, 1.0 = full)
    pub glow_intensity: f32,
    /// Custom width (None = fill available)
    pub width: Option<f32>,
    /// Custom height (None = fill available)
    pub height: Option<f32>,
    /// Blur amount (cosmetic, egui doesn't support actual blur)
    pub blur_amount: f32,
    /// Opacity level (0.0-1.0)
    pub opacity: f32,
    /// Corner radius
    pub corner_radius: Option<f32>,
    /// Inner margin/padding
    pub inner_margin: Option<f32>,
}

impl<'a> GlassPanel<'a> {
    /// Create a new glass panel
    pub fn new() -> Self {
        Self {
            title: None,
            glow_intensity: 0.3,
            width: None,
            height: None,
            blur_amount: 10.0,
            opacity: 0.7,
            corner_radius: None,
            inner_margin: None,
        }
    }

    /// Set the panel title
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// Set border glow intensity (0.0 to 1.0)
    pub fn glow_intensity(mut self, intensity: f32) -> Self {
        self.glow_intensity = intensity.clamp(0.0, 1.0);
        self
    }

    /// Set custom width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set custom height
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set blur amount (cosmetic only, egui doesn't support real blur)
    pub fn blur(mut self, amount: f32) -> Self {
        self.blur_amount = amount;
        self
    }

    /// Set opacity level (0.0 to 1.0)
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set corner radius
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = Some(radius);
        self
    }

    /// Set inner margin/padding
    pub fn inner_margin(mut self, margin: f32) -> Self {
        self.inner_margin = Some(margin);
        self
    }

    /// Show the glass panel with content
    pub fn show<R>(
        self,
        ui: &mut egui::Ui,
        theme: &Theme,
        content: impl FnOnce(&mut egui::Ui) -> R,
    ) -> GlassPanelResponse<R> {
        // Translucent background (glassmorphic effect)
        // Note: egui doesn't support backdrop blur, so we simulate with semi-transparent surface
        let glass_color = {
            let surface = theme.card();
            egui::Color32::from_rgba_unmultiplied(
                surface.r(),
                surface.g(),
                surface.b(),
                (255.0 * self.opacity) as u8,
            )
        };

        // Border glow color (based on primary with intensity)
        let glow_color = {
            let primary = theme.primary();
            egui::Color32::from_rgba_unmultiplied(
                primary.r(),
                primary.g(),
                primary.b(),
                (255.0 * self.glow_intensity * 0.3) as u8,
            )
        };

        let corner_rad = self
            .corner_radius
            .unwrap_or(theme.spacing.corner_radius as f32) as u8;
        let inner_margin_val = self.inner_margin.unwrap_or(theme.spacing.md);
        let mut content_result = None;

        // Create a scope to constrain width and/or height if specified
        let frame_response = if self.width.is_some() || self.height.is_some() {
            let (width, height) = (
                self.width.unwrap_or(ui.available_width()),
                self.height.unwrap_or(ui.available_height()),
            );

            // Generate a unique ID for this scroll area based on the title or position
            let scroll_id = if let Some(title) = self.title {
                egui::Id::new("glass_panel_scroll").with(title)
            } else {
                ui.next_auto_id()
            };

            ui.allocate_ui_with_layout(
                egui::vec2(width, height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    // Use ScrollArea with fixed size to strictly enforce dimensions
                    egui::ScrollArea::both()
                        .id_source(scroll_id)
                        .auto_shrink([false; 2])
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .max_width(width)
                        .max_height(height)
                        .show(ui, |ui| {
                        ui.set_width(width);
                        ui.set_height(height);

                        egui::Frame::new()
                            .fill(glass_color)
                            .corner_radius(CornerRadius::same(corner_rad))
                            .stroke(egui::Stroke::new(1.0, theme.border()))
                            .inner_margin(inner_margin_val)
                            .show(ui, |ui| {
                                // Title if provided
                                if let Some(title) = self.title {
                                    ui.label(
                                        egui::RichText::new(title)
                                            .size(ui.spacing().interact_size.y * 0.7)
                                            .color(theme.foreground())
                                            .strong(),
                                    );
                                    ui.add_space(theme.spacing.sm);
                                }

                                // User content
                                content_result = Some(content(ui));
                            })
                    })
                },
            )
            .inner
            .inner
        } else {
            egui::Frame::new()
                .fill(glass_color)
                .corner_radius(CornerRadius::same(corner_rad))
                .stroke(egui::Stroke::new(1.0, theme.border()))
                .inner_margin(inner_margin_val)
                .show(ui, |ui| {
                    // Title if provided
                    if let Some(title) = self.title {
                        ui.label(
                            egui::RichText::new(title)
                                .size(ui.spacing().interact_size.y * 0.7)
                                .color(theme.foreground())
                                .strong(),
                        );
                        ui.add_space(theme.spacing.sm);
                    }

                    // User content
                    content_result = Some(content(ui));
                })
        };

        let rect = frame_response.response.rect;

        // Draw shimmer on top
        let shimmer_rect =
            egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), theme.spacing.xs / 2.0));
        ui.painter().rect_filled(
            shimmer_rect,
            CornerRadius::same(corner_rad),
            theme.border(),
        );

        // Draw glow border if intensity > 0
        if self.glow_intensity > 0.0 {
            ui.painter().rect_stroke(
                rect,
                CornerRadius::same(corner_rad),
                egui::Stroke::new(1.5, glow_color),
                egui::StrokeKind::Middle,
            );
        }

        GlassPanelResponse {
            response: frame_response.response,
            inner: content_result.unwrap(),
        }
    }
}

impl<'a> Default for GlassPanel<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from showing a glass panel
pub struct GlassPanelResponse<R> {
    /// The interaction response for the panel
    pub response: egui::Response,
    /// The result from the content closure
    pub inner: R,
}

impl<R> GlassPanelResponse<R> {
    /// Whether the panel is hovered
    pub fn hovered(&self) -> bool {
        self.response.hovered()
    }
}
