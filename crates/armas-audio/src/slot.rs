//! Slot Component
//!
//! Plugin/effect insert slot with activity meter and bypass indicator.
//!
//! A colored box component designed to display audio plugins or effects.
//! Features an activity meter showing processing level and a bypass indicator.
//! Colors are automatically assigned based on effect type (reverb, EQ, compressor, etc.).

use armas_basic::theme::Theme;
use egui;

/// Response from the slot component
#[derive(Debug, Clone)]
pub struct SlotResponse;

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
/// ```rust,ignore
/// use armas_audio::Slot;
/// use armas_basic::Theme;
///
/// fn ui(ui: &mut egui::Ui, theme: &Theme) {
///     Slot::new()
///         .effect("Reverb")
///         .show(ui, theme);
///
///     // Custom size
///     Slot::new()
///         .width(100.0)
///         .height(40.0)
///         .effect("EQ")
///         .show(ui, theme);
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
    /// Custom slot color (overrides auto-detection from effect name)
    pub color: Option<egui::Color32>,
}

impl Default for Slot<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Slot<'a> {
    /// Create a new empty slot with default dimensions (140x28)
    #[must_use]
    pub const fn new() -> Self {
        Self {
            name: None,
            bypassed: false,
            level: 0.0,
            width: 140.0,
            height: 28.0,
            color: None,
        }
    }

    /// Set width
    #[must_use]
    pub const fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set height
    #[must_use]
    pub const fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set the plugin/effect name
    #[must_use]
    pub const fn effect(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    /// Set a custom slot color (overrides auto-detection from effect name)
    #[must_use]
    pub const fn color(mut self, color: egui::Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Show the slot component
    pub fn show(self, ui: &mut egui::Ui, theme: &armas_basic::Theme) -> SlotResponse {
        let font_size = (self.height * 0.55).max(8.0);
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(self.width, self.height), egui::Sense::click());

        let filled = self.name.is_some();

        // Determine colors
        let (mut box_color, mut border_color, mut text_color) = self.name.map_or_else(
            || (theme.card(), theme.border(), theme.muted_foreground()),
            |name| {
                let effect_color = self.color.unwrap_or_else(|| get_effect_color(name, theme));
                let border = if self.bypassed {
                    theme.muted_foreground()
                } else {
                    effect_color.gamma_multiply(1.3)
                };
                // Use luminance to pick contrasting text color
                let lum = 0.299 * effect_color.r() as f32
                    + 0.587 * effect_color.g() as f32
                    + 0.114 * effect_color.b() as f32;
                let text = if lum > 140.0 {
                    egui::Color32::from_gray(20)
                } else {
                    egui::Color32::from_gray(240)
                };
                (effect_color, border, text)
            },
        );

        // Hover effect - brighten box, border, and text (especially for empty slots with "+")
        if response.hovered() {
            box_color = box_color.gamma_multiply(1.2);
            // For empty slots, use theme primary for border glow
            // For filled slots, brighten the existing border color
            if filled {
                border_color = border_color.gamma_multiply(1.4);
            } else {
                border_color = theme.primary();
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
        let label = self.name.map_or_else(
            || "+".to_string(),
            |n| {
                if n.len() > 7 {
                    format!("{}…", &n[..6])
                } else {
                    n.to_string()
                }
            },
        );

        ui.painter().text(
            rect.left_center() + egui::vec2(theme.spacing.sm, 0.0),
            egui::Align2::LEFT_CENTER,
            label,
            egui::FontId::proportional(font_size),
            text_color,
        );

        // Mini meter (if effect is active) - flatter style
        if filled {
            const MINI_METER_WIDTH: f32 = 2.0;
            let meter_rect = egui::Rect::from_min_size(
                egui::pos2(
                    theme
                        .spacing
                        .sm
                        .mul_add(-0.8, rect.max.x - MINI_METER_WIDTH),
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
                ui.painter().circle_filled(bypass_pos, 3.0, theme.chart_3());
            }
        }

        SlotResponse
    }
}

/// Get color for effect based on name using theme colors
fn get_effect_color(name: &str, theme: &Theme) -> egui::Color32 {
    let lower = name.to_lowercase();

    if lower.contains("reverb") || lower.contains("delay") || lower.contains("echo") {
        theme.chart_4() // Blue - spatial/time-based effects
    } else if lower.contains("eq") || lower.contains("filter") {
        theme.chart_2() // Green - corrective/clean effects
    } else if lower.contains("comp") || lower.contains("limit") || lower.contains("gate") {
        theme.chart_3() // Orange - dynamic effects
    } else if lower.contains("chorus") || lower.contains("flanger") || lower.contains("phaser") {
        theme.secondary() // Purple - modulation effects
    } else if lower.contains("dist") || lower.contains("drive") || lower.contains("satur") {
        theme.destructive() // Red - distortion/aggressive effects
    } else {
        theme.primary()
    }
}
