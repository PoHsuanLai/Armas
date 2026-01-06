//! Button component with Material Design 3 styling
//!
//! Provides five variants following Material Design 3 guidelines:
//! - Filled: Highest emphasis, solid background with primary color
//! - FilledTonal: Medium-high emphasis, subtle background
//! - Elevated: Filled tonal with shadow for separation
//! - Outlined: Medium emphasis, transparent with border
//! - Text: Lowest emphasis, minimal styling

use crate::animation::Interpolate;
use crate::theme::Theme;
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
    /// Speaker-style button - modern, sleek plastic aesthetic with subtle depth
    /// Perfect for audio controls (play, pause, mute, solo, record, etc.)
    Speaker,
}

/// Material Design inspired button component
pub struct Button {
    text: String,
    variant: ButtonVariant,
    min_size: Vec2,
    enabled: bool,
}

impl Button {
    /// Create a new button with text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: ButtonVariant::Filled,
            min_size: Vec2::new(80.0, 32.0),
            enabled: true,
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

    /// Set enabled state
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Show the button
    pub fn show(self, ui: &mut Ui, theme: &Theme) -> Response {
        let Button {
            text,
            variant,
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
            // Determine colors and shadow based on variant and state
            let (bg_color, text_color, border_color, draw_shadow) = if !enabled {
                // Disabled state
                let disabled_bg = theme.surface_variant();
                let disabled_text = theme.on_surface_variant();
                (disabled_bg, disabled_text, theme.outline_variant(), false)
            } else if response.hovered() {
                // Hover state
                match variant {
                    ButtonVariant::Filled => {
                        let hover_bg = theme.primary().interpolate(&theme.hover(), 0.2);
                        (hover_bg, theme.on_surface(), theme.primary(), false)
                    }
                    ButtonVariant::FilledTonal => {
                        let hover_bg = theme.secondary().interpolate(&theme.hover(), 0.15);
                        (hover_bg, theme.on_surface(), theme.secondary(), false)
                    }
                    ButtonVariant::Elevated => {
                        let hover_bg = theme.secondary().interpolate(&theme.hover(), 0.15);
                        (hover_bg, theme.on_surface(), theme.secondary(), true)
                    }
                    ButtonVariant::Outlined => {
                        (theme.hover(), theme.primary(), theme.primary(), false)
                    }
                    ButtonVariant::Text => {
                        (theme.hover(), theme.primary(), Color32::TRANSPARENT, false)
                    }
                    ButtonVariant::Speaker => {
                        // Slightly lighter on hover - uses theme surface colors
                        let hover_bg = theme.surface_variant().interpolate(&theme.hover(), 0.3);
                        (hover_bg, theme.on_surface(), theme.outline_variant(), false)
                    }
                }
            } else {
                // Normal state
                match variant {
                    ButtonVariant::Filled => {
                        (theme.primary(), theme.on_surface(), theme.primary(), false)
                    }
                    ButtonVariant::FilledTonal => {
                        let tonal_bg = theme.secondary();
                        (tonal_bg, theme.on_surface(), theme.secondary(), false)
                    }
                    ButtonVariant::Elevated => {
                        let tonal_bg = theme.secondary();
                        (tonal_bg, theme.on_surface(), theme.secondary(), true)
                    }
                    ButtonVariant::Outlined => (
                        Color32::TRANSPARENT,
                        theme.on_surface(),
                        theme.primary(),
                        false,
                    ),
                    ButtonVariant::Text => (
                        Color32::TRANSPARENT,
                        theme.on_surface(),
                        Color32::TRANSPARENT,
                        false,
                    ),
                    ButtonVariant::Speaker => {
                        // Modern speaker button: smooth matte plastic - uses theme surface
                        (
                            theme.surface_variant(),
                            theme.on_surface_variant(),
                            theme.outline_variant(),
                            false,
                        )
                    }
                }
            };

            // Special rendering for Speaker variant
            if variant == ButtonVariant::Speaker {
                let painter = ui.painter();
                let corner_radius = 10.0; // Softer corners for speaker style
                let is_pressed = response.is_pointer_button_down_on();

                // Soft shadow (modern, diffused)
                let shadow_color = Color32::from_black_alpha(50);
                painter.rect_filled(
                    rect.translate(Vec2::new(0.0, 2.0)),
                    corner_radius,
                    shadow_color,
                );

                if is_pressed {
                    // Pressed state: darker with subtle inset shadow - uses theme background
                    let pressed_bg = theme
                        .surface_variant()
                        .interpolate(&theme.background(), 0.5);
                    painter.rect_filled(rect, corner_radius, pressed_bg);

                    // Subtle inset shadow at top
                    painter.rect_stroke(
                        rect.shrink(0.5),
                        corner_radius,
                        egui::Stroke::new(1.0, Color32::from_black_alpha(80)),
                        egui::StrokeKind::Middle,
                    );
                } else {
                    // Normal/Hover: Subtle gradient (top to bottom, very soft) derived from theme
                    let base_color = if response.hovered() {
                        theme.surface_variant().interpolate(&theme.hover(), 0.3)
                    } else {
                        theme.surface_variant()
                    };

                    // Create subtle gradient by darkening bottom slightly
                    let top_color = base_color;
                    let bottom_color = base_color.interpolate(&theme.background(), 0.2);

                    // Draw gradient with 8 steps (subtle enough to look smooth)
                    for i in 0..8 {
                        let t = i as f32 / 7.0;
                        let color = Color32::from_rgb(
                            (top_color.r() as f32
                                + t * (bottom_color.r() as f32 - top_color.r() as f32))
                                as u8,
                            (top_color.g() as f32
                                + t * (bottom_color.g() as f32 - top_color.g() as f32))
                                as u8,
                            (top_color.b() as f32
                                + t * (bottom_color.b() as f32 - top_color.b() as f32))
                                as u8,
                        );

                        let segment_height = rect.height() / 8.0;
                        let y = rect.min.y + i as f32 * segment_height;

                        painter.rect_filled(
                            egui::Rect::from_min_size(
                                egui::Pos2::new(rect.min.x, y),
                                Vec2::new(rect.width(), segment_height + 1.0),
                            ),
                            corner_radius,
                            color,
                        );
                    }

                    // Top highlight (subtle, like light reflection)
                    painter.line_segment(
                        [
                            rect.min + Vec2::new(corner_radius, 1.0),
                            rect.min + Vec2::new(rect.width() - corner_radius, 1.0),
                        ],
                        egui::Stroke::new(0.5, Color32::from_white_alpha(25)),
                    );
                }

                // Clean border (precise edge)
                painter.rect_stroke(
                    rect,
                    corner_radius,
                    egui::Stroke::new(1.0, border_color),
                    egui::StrokeKind::Middle,
                );
            } else {
                // Original rendering for other variants
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
            }

            // Draw text
            let font_id = egui::TextStyle::Button.resolve(ui.style());
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                font_id,
                text_color,
            );
        }

        response
    }
}
