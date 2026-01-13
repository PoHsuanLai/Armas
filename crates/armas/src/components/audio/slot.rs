//! Slot Component
//!
//! Plugin/effect insert slot with activity meter and bypass indicator.
//!
//! A colored box component designed to display audio plugins or effects.
//! Features an activity meter showing processing level and a bypass indicator.
//! Colors are automatically assigned based on effect type (reverb, EQ, compressor, etc.).

use crate::ext::ArmasContextExt;
use crate::theme::Theme;
use egui;

/// Plugin/effect insert slot component
///
/// Displays a plugin or effect insert with:
/// - Color-coded background based on effect type
/// - Activity meter showing processing level
/// - Bypass indicator when effect is bypassed
/// - Clickable for interaction
///
/// # Example
///
/// ```rust,no_run
/// use armas::components::Slot;
///
/// fn ui(ui: &mut egui::Ui) {
///     Slot::new()
///         .effect("Reverb")
///         .level(0.75)
///         .show(ui);
///
///     // Custom size
///     Slot::new()
///         .size(100.0, 40.0)
///         .effect("EQ")
///         .show(ui);
/// }
/// ```
pub struct Slot<'a> {
    /// Plugin/effect name (or None for empty slot)
    pub name: Option<&'a str>,
    /// Whether the effect is bypassed
    pub bypassed: bool,
    /// Activity level (0.0 - 1.0) for mini meter
    pub level: f32,
    /// Width of the insert box
    pub width: f32,
    /// Height of the insert box
    pub height: f32,
}

impl<'a> Default for Slot<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Slot<'a> {
    /// Create a new empty slot with default dimensions (140x28)
    pub fn new() -> Self {
        Self {
            name: None,
            bypassed: false,
            level: 0.0,
            width: 140.0,
            height: 28.0,
        }
    }

    /// Set custom width and height
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set the plugin/effect name
    pub fn effect(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    /// Set whether the effect is bypassed
    pub fn bypassed(mut self, bypassed: bool) -> Self {
        self.bypassed = bypassed;
        self
    }

    /// Set the activity level (0.0 to 1.0)
    pub fn level(mut self, level: f32) -> Self {
        self.level = level.clamp(0.0, 1.0);
        self
    }

    /// Show the slot component
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = ui.ctx().armas_theme();
        let font_size = ui.spacing().interact_size.y * 0.4;
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(self.width, self.height), egui::Sense::click());

        let filled = self.name.is_some();

        // Determine colors
        let (mut box_color, mut border_color, mut text_color) = if let Some(name) = self.name {
            let effect_color = get_effect_color(name, &theme);
            let border = if self.bypassed {
                theme.on_surface_variant()
            } else {
                effect_color.gamma_multiply(1.3)
            };
            (effect_color, border, theme.on_surface())
        } else {
            (
                theme.surface(),
                theme.outline_variant(),
                theme.on_surface_variant(),
            )
        };

        // Hover effect - brighten box, border, and text (especially for empty slots with "+")
        if response.hovered() {
            box_color = box_color.gamma_multiply(1.2);
            // For empty slots, use theme primary for border glow
            // For filled slots, brighten the existing border color
            if !filled {
                border_color = theme.primary();
            } else {
                border_color = border_color.gamma_multiply(1.4);
            }
            text_color = text_color.gamma_multiply(1.5); // Make text (including "+") brighter
        }

        // Background - flatter style with smaller corner radius
        ui.painter().rect_filled(rect, 3.0, box_color);

        // Border - thinner for flatter appearance
        let border_width = if filled { 1.0 } else { 0.8 };
        ui.painter().rect_stroke(
            rect,
            3.0,
            egui::Stroke::new(border_width, border_color),
            egui::StrokeKind::Middle,
        );

        // Text label
        let label = self
            .name
            .map(|n| {
                if n.len() > 7 {
                    format!("{}…", &n[..6])
                } else {
                    n.to_string()
                }
            })
            .unwrap_or_else(|| "+".to_string());

        ui.painter().text(
            rect.left_center() + egui::vec2(theme.spacing.sm, 0.0),
            egui::Align2::LEFT_CENTER,
            label,
            egui::FontId::proportional(font_size * 0.9),
            text_color,
        );

        // Mini meter (if effect is active) - flatter style
        if filled {
            const MINI_METER_WIDTH: f32 = 2.0;
            let meter_rect = egui::Rect::from_min_size(
                egui::pos2(
                    rect.max.x - MINI_METER_WIDTH - theme.spacing.sm * 0.8,
                    rect.min.y + 4.0,
                ),
                egui::vec2(MINI_METER_WIDTH, self.height - 8.0),
            );

            // Meter fill - no border for cleaner look
            let meter_height = meter_rect.height() * self.level;
            let meter_fill = egui::Rect::from_min_size(
                egui::pos2(meter_rect.min.x, meter_rect.max.y - meter_height),
                egui::vec2(MINI_METER_WIDTH, meter_height),
            );

            ui.painter().rect_filled(meter_fill, 1.0, theme.primary());

            // Bypass indicator
            if self.bypassed {
                let bypass_pos =
                    rect.right_center() - egui::vec2(MINI_METER_WIDTH + theme.spacing.md, 0.0);
                ui.painter().circle_filled(bypass_pos, 3.0, theme.warning());
            }
        }

        response
    }
}

/// Get color for effect based on name using theme colors
fn get_effect_color(name: &str, theme: &Theme) -> egui::Color32 {
    let lower = name.to_lowercase();

    if lower.contains("reverb") || lower.contains("delay") || lower.contains("echo") {
        theme.info() // Blue - spatial/time-based effects
    } else if lower.contains("eq") || lower.contains("filter") {
        theme.success() // Green - corrective/clean effects
    } else if lower.contains("comp") || lower.contains("limit") || lower.contains("gate") {
        theme.warning() // Orange - dynamic effects
    } else if lower.contains("chorus") || lower.contains("flanger") || lower.contains("phaser") {
        theme.secondary() // Purple - modulation effects
    } else if lower.contains("dist") || lower.contains("drive") || lower.contains("satur") {
        theme.error() // Red - distortion/aggressive effects
    } else {
        theme.primary()
    }
}
