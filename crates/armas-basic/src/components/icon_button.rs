//! Icon Button Component
//!
//! A button variant specifically designed for rendering icons.

use crate::components::ButtonVariant;
use crate::ext::ArmasContextExt;
use crate::icon::{render_icon_data, IconData, OwnedIconData};
use egui::{Color32, Response, Sense, Ui, Vec2};

/// Icon Button component
///
/// `size` controls the **total widget size** (allocation), not the icon drawing area.
/// The icon is drawn inside after subtracting padding.
/// For Ghost/Link variants, padding defaults to 0 so `size` ≈ icon size.
/// For other variants, padding defaults to `ui.spacing().button_padding`.
pub struct IconButton<'a> {
    vertices: &'a [(f32, f32)],
    indices: &'a [u32],
    viewbox_width: f32,
    viewbox_height: f32,
    variant: ButtonVariant,
    size: f32,
    padding: Option<f32>,
    enabled: bool,
    icon_color: Option<Color32>,
    hover_icon_color: Option<Color32>,
}

impl<'a> IconButton<'a> {
    /// Create a new icon button from static [`IconData`].
    #[must_use]
    pub const fn new(icon_data: &'a IconData) -> Self {
        Self {
            vertices: icon_data.vertices,
            indices: icon_data.indices,
            viewbox_width: icon_data.viewbox_width,
            viewbox_height: icon_data.viewbox_height,
            variant: ButtonVariant::Default,
            size: 16.0,
            padding: None,
            enabled: true,
            icon_color: None,
            hover_icon_color: None,
        }
    }

    /// Create a new icon button from [`OwnedIconData`].
    #[must_use]
    pub fn from_owned(data: &'a OwnedIconData) -> Self {
        Self {
            vertices: &data.vertices,
            indices: &data.indices,
            viewbox_width: data.viewbox_width,
            viewbox_height: data.viewbox_height,
            variant: ButtonVariant::Default,
            size: 16.0,
            padding: None,
            enabled: true,
            icon_color: None,
            hover_icon_color: None,
        }
    }

    /// Set the button variant
    #[must_use]
    pub const fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the total widget size (icon + padding). Default 16.
    #[must_use]
    pub const fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set padding between widget edge and icon. Default: 0 for Ghost/Link, button_padding for others.
    #[must_use]
    pub const fn padding(mut self, padding: f32) -> Self {
        self.padding = Some(padding);
        self
    }

    /// Set enabled state
    #[must_use]
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set custom icon color (overrides default)
    #[must_use]
    pub const fn icon_color(mut self, color: Color32) -> Self {
        self.icon_color = Some(color);
        self
    }

    /// Set custom hover icon color (overrides default)
    #[must_use]
    pub const fn hover_icon_color(mut self, color: Color32) -> Self {
        self.hover_icon_color = Some(color);
        self
    }

    /// Show the icon button
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();

        // Ghost/Link: no background, so default padding = 0 (size ≈ icon).
        // Others: use egui's button_padding so icon has breathing room inside bg.
        let padding = self.padding.unwrap_or_else(|| match self.variant {
            ButtonVariant::Ghost | ButtonVariant::Link => 0.0,
            _ => {
                let bp = ui.spacing().button_padding;
                bp.x.max(bp.y)
            }
        });

        // `size` is the total widget allocation.
        let total_size = Vec2::splat(self.size);
        // Icon draws in the inner rect after subtracting padding.
        let icon_draw_size = (self.size - padding * 2.0).max(1.0);

        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };

        let (rect, response) = ui.allocate_exact_size(total_size, sense);

        if ui.is_rect_visible(rect) {
            let (bg_color, mut icon_color) = match self.variant {
                ButtonVariant::Default => {
                    let bg = if response.is_pointer_button_down_on() {
                        theme.primary().linear_multiply(0.9)
                    } else if response.hovered() {
                        theme.primary().linear_multiply(1.08)
                    } else {
                        theme.primary()
                    };
                    (Some(bg), theme.primary_foreground())
                }
                ButtonVariant::Secondary => {
                    let bg = if response.is_pointer_button_down_on() {
                        theme.secondary()
                    } else if response.hovered() {
                        theme.secondary().linear_multiply(1.08)
                    } else {
                        theme.secondary()
                    };
                    (Some(bg), theme.secondary_foreground())
                }
                ButtonVariant::Outline => {
                    let bg = if response.hovered() {
                        Some(theme.accent())
                    } else {
                        None
                    };
                    let icon = if response.hovered() {
                        theme.accent_foreground()
                    } else {
                        theme.foreground()
                    };
                    (bg, icon)
                }
                ButtonVariant::Ghost | ButtonVariant::Link => {
                    let bg = if response.hovered() {
                        Some(theme.muted())
                    } else {
                        None
                    };
                    let icon = if response.hovered() {
                        theme.foreground()
                    } else {
                        theme.muted_foreground()
                    };
                    (bg, icon)
                }
            };

            if response.hovered() {
                if let Some(custom_hover_color) = self.hover_icon_color {
                    icon_color = custom_hover_color;
                }
            } else if let Some(custom_color) = self.icon_color {
                icon_color = custom_color;
            }

            if !self.enabled {
                icon_color = icon_color.linear_multiply(0.5);
            }

            if let Some(bg) = bg_color {
                let rounding = match self.variant {
                    ButtonVariant::Default | ButtonVariant::Secondary => total_size.x / 2.0,
                    ButtonVariant::Ghost | ButtonVariant::Link => 2.0,
                    _ => 6.0,
                };
                let final_bg = if self.enabled {
                    bg
                } else {
                    bg.linear_multiply(0.5)
                };
                ui.painter().rect_filled(rect, rounding, final_bg);
            }

            if self.variant == ButtonVariant::Outline {
                let stroke = egui::Stroke::new(1.0, theme.border());
                ui.painter()
                    .rect_stroke(rect, 6.0, stroke, egui::epaint::StrokeKind::Inside);
            }

            let icon_rect =
                egui::Rect::from_center_size(rect.center(), Vec2::splat(icon_draw_size));
            render_icon_data(
                ui.painter(),
                icon_rect,
                self.vertices,
                self.indices,
                self.viewbox_width,
                self.viewbox_height,
                icon_color,
            );
        }

        response
    }
}
