//! Icon Button Component
//!
//! A button variant specifically designed for rendering icons with Material Design 3 styling.

use crate::components::button::ButtonVariant;
use crate::icon::{render_icon, IconData};
use egui::{Color32, Response, Sense, Ui, Vec2};

/// Icon Button component
///
/// A button specifically designed for icons, following Material Design 3 principles.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # use armas_basic::icon::IconData;
/// # static MY_ICON: IconData = IconData {
/// #     name: "test", vertices: &[], indices: &[],
/// #     viewbox_width: 24.0, viewbox_height: 24.0,
/// # };
/// # fn example(ui: &mut Ui) {
/// use armas_basic::components::button::{IconButton, ButtonVariant};
/// use armas_basic::ext::ArmasContextExt;
///
/// let theme = ui.ctx().armas_theme();
/// if IconButton::new(&MY_ICON)
///     .variant(ButtonVariant::Filled)
///     .size(24.0)
///     .show(ui, &theme)
///     .clicked()
/// {
///     // Handle button click
/// }
/// # }
/// ```
pub struct IconButton<'a> {
    icon_data: &'a IconData,
    variant: ButtonVariant,
    size: f32,
    padding: f32,
    enabled: bool,
    icon_color: Option<Color32>,
    hover_icon_color: Option<Color32>,
}

impl<'a> IconButton<'a> {
    /// Create a new icon button
    #[must_use] 
    pub const fn new(icon_data: &'a IconData) -> Self {
        Self {
            icon_data,
            variant: ButtonVariant::Default,
            size: 24.0,
            padding: 8.0,
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

    /// Set the icon size
    #[must_use] 
    pub const fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the padding around the icon
    #[must_use] 
    pub const fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
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
    pub fn show(self, ui: &mut Ui, theme: &crate::Theme) -> Response {
        let total_size = Vec2::splat(self.size + self.padding * 2.0);

        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };

        let (rect, response) = ui.allocate_exact_size(total_size, sense);

        if ui.is_rect_visible(rect) {
            // Determine colors based on variant and state
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
                ButtonVariant::Outline | ButtonVariant::Ghost | ButtonVariant::Link => {
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
            };

            // Apply custom colors if provided
            if response.hovered() {
                if let Some(custom_hover_color) = self.hover_icon_color {
                    icon_color = custom_hover_color;
                }
            } else if let Some(custom_color) = self.icon_color {
                icon_color = custom_color;
            }

            // Apply disabled state
            if !self.enabled {
                icon_color = icon_color.linear_multiply(0.5);
            }

            // Draw background if needed
            if let Some(bg) = bg_color {
                let rounding = match self.variant {
                    ButtonVariant::Default | ButtonVariant::Secondary => total_size.x / 2.0, // Circular
                    _ => 6.0, // rounded-md
                };
                let final_bg = if self.enabled {
                    bg
                } else {
                    bg.linear_multiply(0.5)
                };
                ui.painter().rect_filled(rect, rounding, final_bg);
            }

            // Draw outline for outline variant
            if self.variant == ButtonVariant::Outline {
                let stroke = egui::Stroke::new(1.0, theme.border());
                ui.painter()
                    .rect_stroke(rect, 6.0, stroke, egui::epaint::StrokeKind::Inside);
            }

            // Draw icon
            let icon_rect = egui::Rect::from_center_size(rect.center(), Vec2::splat(self.size));
            render_icon(ui.painter(), icon_rect, self.icon_data, icon_color);
        }

        response
    }
}
