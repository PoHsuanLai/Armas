//! Base Button component with Material Design 3 styling
//!
//! Provides variants following Material Design 3 guidelines:
//! - Filled: Highest emphasis, solid background with primary color
//! - FilledTonal: Medium-high emphasis, subtle background
//! - Elevated: Filled tonal with shadow for separation
//! - Outlined: Medium emphasis, transparent with border
//! - Text: Lowest emphasis, minimal styling

use crate::animation::Interpolate;
use crate::ext::ArmasContextExt;
use egui::{Color32, Response, Sense, Ui, Vec2};

/// Button style variant following Material Design 3
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonVariant {
    /// Filled button - highest emphasis for primary actions
    Filled,
    /// Filled tonal button - medium-high emphasis, alternative to filled
    FilledTonal,
    /// Elevated button - filled tonal with shadow for visual separation
    Elevated,
    /// Outlined button - medium emphasis for secondary actions
    Outlined,
    /// Text button - lowest emphasis for tertiary actions
    Text,
}

/// Material Design inspired button component
pub struct Button {
    text: String,
    variant: ButtonVariant,
    min_size: Vec2,
    max_width: Option<f32>,
    enabled: bool,
    text_align: egui::Align2,
    text_color: Option<Color32>,
    hover_text_color: Option<Color32>,
}

impl Button {
    /// Calculate contrasting text color based on background brightness
    fn contrasting_text_color(bg_color: Color32) -> Color32 {
        // Calculate relative luminance (perceived brightness)
        let r = bg_color.r() as f32 / 255.0;
        let g = bg_color.g() as f32 / 255.0;
        let b = bg_color.b() as f32 / 255.0;

        let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;

        // If background is bright (luminance > 0.5), use dark text
        // Otherwise use light text
        if luminance > 0.5 {
            Color32::from_gray(20) // Dark text
        } else {
            Color32::from_gray(235) // Light text
        }
    }

    /// Create a new button with text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: ButtonVariant::Filled,
            min_size: Vec2::new(80.0, 32.0),
            max_width: None,
            enabled: true,
            text_align: egui::Align2::CENTER_CENTER,
            text_color: None,
            hover_text_color: None,
        }
    }

    /// Set the button variant
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set minimum size
    pub fn min_size(mut self, size: Vec2) -> Self {
        self.min_size = size;
        self
    }

    /// Set maximum width (text will be truncated with ellipsis if it exceeds this)
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    /// Set enabled state
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set text alignment
    pub fn text_align(mut self, align: egui::Align2) -> Self {
        self.text_align = align;
        self
    }

    /// Set custom text color (overrides default)
    pub fn text_color(mut self, color: Color32) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Set custom hover text color (overrides default)
    pub fn hover_text_color(mut self, color: Color32) -> Self {
        self.hover_text_color = Some(color);
        self
    }

    /// Show the button
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();
        let Button {
            text,
            variant,
            min_size,
            max_width,
            enabled,
            text_align,
            text_color: custom_text_color,
            hover_text_color: custom_hover_text_color,
        } = self;

        let sense = if enabled {
            Sense::click()
        } else {
            Sense::hover()
        };

        // Calculate actual button size based on text
        let font_id = egui::TextStyle::Button.resolve(ui.style());
        let horizontal_padding = 24.0; // 12px on each side

        // Measure text to determine required width
        let text_galley = ui.painter().layout_no_wrap(
            text.clone(),
            font_id.clone(),
            Color32::PLACEHOLDER, // Color doesn't matter for measurement
        );
        let text_width = text_galley.rect.width();

        // Calculate button width: max(min_size.x, text_width + padding)
        let mut button_width = text_width + horizontal_padding;
        button_width = button_width.max(min_size.x);

        // Apply max_width if specified
        if let Some(max_w) = max_width {
            button_width = button_width.min(max_w);
        }

        let button_size = Vec2::new(button_width, min_size.y);
        let (rect, mut response) = ui.allocate_exact_size(button_size, sense);

        // Change cursor to pointer on hover when enabled
        if enabled && response.hovered() {
            response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        }

        if ui.is_rect_visible(rect) {
            // Determine colors and shadow based on variant and state
            let (bg_color, mut text_color, border_color, draw_shadow) = if !enabled {
                // Disabled state
                let disabled_bg = theme.muted();
                let disabled_text = theme.muted_foreground();
                (disabled_bg, disabled_text, theme.border(), false)
            } else if response.hovered() {
                // Hover state
                match variant {
                    ButtonVariant::Filled => {
                        let hover_bg = theme.primary().interpolate(&theme.accent(), 0.2);
                        // Calculate contrasting text color based on primary color brightness
                        let text_color = Self::contrasting_text_color(theme.primary());
                        (hover_bg, text_color, theme.primary(), false)
                    }
                    ButtonVariant::FilledTonal => {
                        let hover_bg = theme.secondary().interpolate(&theme.accent(), 0.15);
                        (hover_bg, theme.foreground(), theme.secondary(), false)
                    }
                    ButtonVariant::Elevated => {
                        let hover_bg = theme.secondary().interpolate(&theme.accent(), 0.15);
                        (hover_bg, theme.foreground(), theme.secondary(), true)
                    }
                    ButtonVariant::Outlined => {
                        (theme.accent(), theme.primary(), theme.primary(), false)
                    }
                    ButtonVariant::Text => {
                        (theme.accent(), theme.primary(), Color32::TRANSPARENT, false)
                    }
                }
            } else {
                // Normal state
                match variant {
                    ButtonVariant::Filled => {
                        // Calculate contrasting text color based on primary color brightness
                        let text_color = Self::contrasting_text_color(theme.primary());
                        (theme.primary(), text_color, theme.primary(), false)
                    }
                    ButtonVariant::FilledTonal => {
                        let tonal_bg = theme.secondary();
                        (tonal_bg, theme.foreground(), theme.secondary(), false)
                    }
                    ButtonVariant::Elevated => {
                        let tonal_bg = theme.secondary();
                        (tonal_bg, theme.foreground(), theme.secondary(), true)
                    }
                    ButtonVariant::Outlined => (
                        Color32::TRANSPARENT,
                        theme.foreground(),
                        theme.primary(),
                        false,
                    ),
                    ButtonVariant::Text => (
                        Color32::TRANSPARENT,
                        theme.foreground(),
                        Color32::TRANSPARENT,
                        false,
                    ),
                }
            };

            // Apply custom text colors if provided
            if response.hovered() {
                if let Some(hover_color) = custom_hover_text_color {
                    text_color = hover_color;
                }
            } else if let Some(normal_color) = custom_text_color {
                text_color = normal_color;
            }

            // Draw shadow for elevated variant
            if draw_shadow {
                let shadow_color = Color32::from_black_alpha(60);
                ui.painter().rect_filled(
                    rect.translate(Vec2::new(0.0, 2.0)),
                    theme.spacing.corner_radius_small,
                    shadow_color,
                );
            }

            // Draw background
            ui.painter()
                .rect_filled(rect, theme.spacing.corner_radius_small, bg_color);

            // Draw border for outlined variant
            if variant == ButtonVariant::Outlined {
                ui.painter().rect_stroke(
                    rect,
                    theme.spacing.corner_radius_small,
                    egui::Stroke::new(1.5, border_color),
                    egui::StrokeKind::Middle,
                );
            }

            // Draw text with proper clipping/truncation
            let available_text_width = rect.width() - horizontal_padding;

            // Create galley with truncation if needed
            let final_galley = if text_width > available_text_width {
                // Text is too long, truncate with ellipsis
                ui.painter()
                    .layout(text, font_id.clone(), text_color, available_text_width)
            } else {
                // Text fits, use normal layout
                text_galley
            };

            // Calculate text position based on alignment
            // galley() uses top-left corner, so we need to adjust for centering
            let galley_height = final_galley.rect.height();
            let galley_width = final_galley.rect.width();

            let text_pos = match text_align {
                egui::Align2::LEFT_CENTER => {
                    egui::pos2(rect.left() + 12.0, rect.center().y - galley_height / 2.0)
                }
                egui::Align2::RIGHT_CENTER => egui::pos2(
                    rect.right() - 12.0 - galley_width,
                    rect.center().y - galley_height / 2.0,
                ),
                _ => {
                    // CENTER_CENTER
                    egui::pos2(
                        rect.center().x - galley_width / 2.0,
                        rect.center().y - galley_height / 2.0,
                    )
                }
            };

            ui.painter().galley(text_pos, final_galley, text_color);
        }

        response
    }
}
