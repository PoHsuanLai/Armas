//! Slot Component
//!
//! Colored box showing plugin/effect with mini meter and bypass indicator.
//! Matches Studio One's insert design.

use crate::theme::Theme;
use egui;

/// Insert slot component (Studio One style)
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

impl<'a> Slot<'a> {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            name: None,
            bypassed: false,
            level: 0.0,
            width,
            height,
        }
    }

    pub fn with_effect(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn bypassed(mut self, bypassed: bool) -> Self {
        self.bypassed = bypassed;
        self
    }

    pub fn level(mut self, level: f32) -> Self {
        self.level = level;
        self
    }

    pub fn show(self, ui: &mut egui::Ui, theme: &Theme) -> egui::Response {
        let font_size = ui.spacing().interact_size.y * 0.4;
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(self.width, self.height), egui::Sense::click());

        let filled = self.name.is_some();

        // Determine colors
        let (box_color, border_color, text_color) = if let Some(name) = self.name {
            let effect_color = get_effect_color(name, theme);
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

        // Background
        ui.painter()
            .rect_filled(rect, theme.spacing.corner_radius * 0.5, box_color);

        // Border
        let border_width = if filled { 1.5 } else { 1.0 };
        ui.painter().rect_stroke(
            rect,
            theme.spacing.corner_radius * 0.5,
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
            rect.left_center() + egui::vec2(theme.spacing.spacing_small, 0.0),
            egui::Align2::LEFT_CENTER,
            label,
            egui::FontId::proportional(font_size * 0.9),
            text_color,
        );

        // Mini meter (if effect is active)
        if filled {
            const MINI_METER_WIDTH: f32 = 2.5;
            let meter_rect = egui::Rect::from_min_size(
                egui::pos2(
                    rect.max.x - MINI_METER_WIDTH - theme.spacing.spacing_small,
                    rect.min.y + theme.spacing.spacing_small * 0.5,
                ),
                egui::vec2(MINI_METER_WIDTH, self.height - theme.spacing.spacing_small),
            );

            // Meter fill
            let meter_height = meter_rect.height() * self.level;
            let meter_fill = egui::Rect::from_min_size(
                egui::pos2(meter_rect.min.x, meter_rect.max.y - meter_height),
                egui::vec2(MINI_METER_WIDTH, meter_height),
            );

            ui.painter()
                .rect_filled(meter_fill, 0.0, theme.primary().gamma_multiply(0.8));

            // Meter border
            ui.painter().rect_stroke(
                meter_rect,
                0.0,
                egui::Stroke::new(0.5, theme.outline_variant()),
                egui::StrokeKind::Middle,
            );

            // Bypass indicator
            if self.bypassed {
                let bypass_pos = rect.right_center()
                    - egui::vec2(MINI_METER_WIDTH + theme.spacing.spacing_medium, 0.0);
                ui.painter().circle_filled(bypass_pos, 3.0, theme.warning());
            }
        }

        response
    }
}

/// Get color for effect based on name
fn get_effect_color(name: &str, theme: &Theme) -> egui::Color32 {
    let lower = name.to_lowercase();

    if lower.contains("reverb") || lower.contains("delay") || lower.contains("echo") {
        egui::Color32::from_rgb(70, 130, 180) // Blue
    } else if lower.contains("eq") || lower.contains("filter") {
        egui::Color32::from_rgb(60, 179, 113) // Green
    } else if lower.contains("comp") || lower.contains("limit") || lower.contains("gate") {
        egui::Color32::from_rgb(255, 140, 60) // Orange
    } else if lower.contains("chorus") || lower.contains("flanger") || lower.contains("phaser") {
        egui::Color32::from_rgb(147, 112, 219) // Purple
    } else if lower.contains("dist") || lower.contains("drive") || lower.contains("satur") {
        egui::Color32::from_rgb(220, 80, 80) // Red
    } else {
        theme.primary()
    }
}
