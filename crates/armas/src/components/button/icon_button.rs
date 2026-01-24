//! Icon Button Component
//!
//! A button variant specifically designed for rendering icons with Material Design 3 styling.

use crate::components::button::ButtonVariant;
use crate::ext::context::ArmasContextExt;
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
/// # use armas::icon::IconData;
/// # static MY_ICON: IconData = IconData {
/// #     name: "test", vertices: &[], indices: &[],
/// #     viewbox_width: 24.0, viewbox_height: 24.0,
/// # };
/// # fn example(ui: &mut Ui) {
/// use armas::components::button::{IconButton, ButtonVariant};
///
/// if IconButton::new(&MY_ICON)
///     .variant(ButtonVariant::Filled)
///     .size(24.0)
///     .show(ui)
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
    pub fn new(icon_data: &'a IconData) -> Self {
        Self {
            icon_data,
            variant: ButtonVariant::Filled,
            size: 24.0,
            padding: 8.0,
            enabled: true,
            icon_color: None,
            hover_icon_color: None,
        }
    }

    /// Set the button variant
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the icon size
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the padding around the icon
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set enabled state
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set custom icon color (overrides default)
    pub fn icon_color(mut self, color: Color32) -> Self {
        self.icon_color = Some(color);
        self
    }

    /// Set custom hover icon color (overrides default)
    pub fn hover_icon_color(mut self, color: Color32) -> Self {
        self.hover_icon_color = Some(color);
        self
    }

    /// Show the icon button
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();
        let total_size = Vec2::splat(self.size + self.padding * 2.0);

        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };

        let (rect, response) = ui.allocate_exact_size(total_size, sense);

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);

            // Determine colors based on variant and state
            let (bg_color, mut icon_color) = match self.variant {
                ButtonVariant::Filled => {
                    let bg = if response.is_pointer_button_down_on() {
                        theme.primary().linear_multiply(0.9)
                    } else if response.hovered() {
                        let mut color = theme.primary();
                        color = color.linear_multiply(1.08);
                        color
                    } else {
                        theme.primary()
                    };
                    (Some(bg), theme.foreground())
                }
                ButtonVariant::FilledTonal => {
                    let bg = if response.is_pointer_button_down_on() {
                        theme.secondary()
                    } else if response.hovered() {
                        let mut color = theme.secondary();
                        color = color.linear_multiply(1.08);
                        color
                    } else {
                        theme.secondary()
                    };
                    (Some(bg), theme.foreground())
                }
                ButtonVariant::Elevated => {
                    let bg = if response.is_pointer_button_down_on() {
                        theme.card()
                    } else if response.hovered() {
                        let mut color = theme.card();
                        color = color.linear_multiply(1.05);
                        color
                    } else {
                        theme.card()
                    };
                    (Some(bg), theme.primary())
                }
                ButtonVariant::Outlined => {
                    let bg = if response.hovered() {
                        Some(theme.muted().linear_multiply(0.5))
                    } else {
                        None
                    };
                    (bg, theme.primary())
                }
                ButtonVariant::Text => {
                    let bg = if response.hovered() {
                        Some(visuals.bg_fill)
                    } else {
                        None
                    };
                    (bg, theme.primary())
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
                    ButtonVariant::Filled
                    | ButtonVariant::FilledTonal
                    | ButtonVariant::Elevated => total_size.x / 2.0, // Circular
                    _ => 12.0, // Rounded corners
                };
                let final_bg = if !self.enabled {
                    bg.linear_multiply(0.5)
                } else {
                    bg
                };
                ui.painter().rect_filled(rect, rounding, final_bg);
            }

            // Draw outline for outlined variant
            if self.variant == ButtonVariant::Outlined {
                let stroke = egui::Stroke::new(1.0, theme.border());
                ui.painter()
                    .rect_stroke(rect, 12.0, stroke, egui::epaint::StrokeKind::Outside);
            }

            // Draw shadow for elevated variant
            if self.variant == ButtonVariant::Elevated
                && !response.is_pointer_button_down_on()
                && self.enabled
            {
                let shadow_rect = rect.translate(egui::vec2(0.0, 2.0));
                ui.painter().rect_filled(
                    shadow_rect,
                    total_size.x / 2.0,
                    egui::Color32::from_black_alpha(20),
                );
            }

            // Draw icon
            let icon_rect = egui::Rect::from_center_size(rect.center(), Vec2::splat(self.size));
            render_icon(ui.painter(), icon_rect, self.icon_data, icon_color);
        }

        response
    }
}
