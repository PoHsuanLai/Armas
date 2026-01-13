//! Knob Component
//!
//! Polished metallic knob with 3D appearance and inner glow level indicator.
//! Perfect for audio mixing, synthesizers, and effect controls.

use crate::theme::Theme;
use egui::{Color32, Pos2, Response, Sense, Stroke, Ui, Vec2};

/// Polished metallic knob with inner glow indicator
pub struct Knob {
    /// Knob diameter
    diameter: f32,
    /// Label text
    label: Option<String>,
    /// Show value text
    show_value: bool,
    /// Knob color (default: metallic white/silver)
    color: Option<Color32>,
    /// Glow color for level indicator
    glow_color: Option<Color32>,
    /// Minimum angle in radians (default: -2.5)
    min_angle: f32,
    /// Maximum angle in radians (default: 2.5)
    max_angle: f32,
    /// Sensitivity multiplier
    sensitivity: f32,
}

impl Knob {
    /// Create a new knob
    pub fn new(_value: f32) -> Self {
        Self {
            diameter: 60.0,
            label: None,
            show_value: true,
            color: None,
            glow_color: None,
            min_angle: -2.5,
            max_angle: 2.5,
            sensitivity: 0.01, // Increased from 0.005 for better control
        }
    }

    /// Set the knob diameter
    pub fn diameter(mut self, diameter: f32) -> Self {
        self.diameter = diameter;
        self
    }

    /// Set label text
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Show value text below knob
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Set knob color (default: metallic silver)
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Set glow color for level indicator
    pub fn glow_color(mut self, color: Color32) -> Self {
        self.glow_color = Some(color);
        self
    }

    /// Set angle range in radians
    pub fn angle_range(mut self, min: f32, max: f32) -> Self {
        self.min_angle = min;
        self.max_angle = max;
        self
    }

    /// Set drag sensitivity
    pub fn sensitivity(mut self, sensitivity: f32) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    /// Show the knob
    pub fn show(self, ui: &mut Ui, value: &mut f32, theme: &Theme) -> Response {
        let desired_size = Vec2::splat(self.diameter);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        // Handle drag interaction with both vertical and horizontal movement
        if response.dragged() {
            let drag_delta = response.drag_delta();

            // Support both vertical and horizontal drag
            // Horizontal drag is more intuitive for knobs
            let delta_y = -drag_delta.y;
            let delta_x = drag_delta.x;

            // Use whichever axis has more movement
            let primary_delta = if delta_x.abs() > delta_y.abs() {
                delta_x
            } else {
                delta_y
            };

            // Fine control with Shift modifier (10x slower)
            let sensitivity = if ui.input(|i| i.modifiers.shift) {
                self.sensitivity * 0.1
            } else {
                self.sensitivity
            };

            let delta = primary_delta * sensitivity;
            *value = (*value + delta).clamp(0.0, 1.0);
            response.mark_changed();
        }

        // Mouse wheel support for fine adjustment
        // Consume scroll events when hovering to prevent page scrolling
        if response.hovered() {
            let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll_delta.abs() > 0.0 {
                let wheel_sensitivity = 0.01;
                let delta = scroll_delta * wheel_sensitivity;
                *value = (*value + delta).clamp(0.0, 1.0);
                response.mark_changed();

                // Consume the scroll event to prevent page scrolling
                ui.ctx().input_mut(|i| {
                    i.smooth_scroll_delta = Vec2::ZERO;
                });
            }
        }

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Calculate knob center - no padding
            let center = rect.center();
            let radius = self.diameter / 2.0;

            // Base knob color (glazed ceramic silver-grey)
            let base_color = self.color.unwrap_or_else(|| {
                Color32::from_rgb(210, 215, 222)
            });

            // === Layer 1: Outer Shadow (Depth) ===
            for i in 0..6 {
                let shadow_radius = radius + (6 - i) as f32 * 1.0;
                let shadow_alpha = 20 - i * 3;
                painter.circle_filled(
                    center,
                    shadow_radius,
                    Color32::from_rgba_unmultiplied(0, 0, 0, shadow_alpha as u8),
                );
            }

            // === Layer 2: Base Ceramic Body ===
            // Gradient from darker (bottom) to lighter (top)
            let dark_base = Color32::from_rgb(185, 190, 200);
            let light_base = Color32::from_rgb(225, 230, 238);

            // Bottom half darker
            painter.circle_filled(center, radius, dark_base);

            // Top half lighter with gradient
            let gradient_center = Pos2::new(center.x, center.y - radius * 0.2);
            for i in 0..8 {
                let grad_radius = radius * (1.0 - i as f32 * 0.09);
                let alpha = 25 + i * 8;
                painter.circle_filled(
                    gradient_center,
                    grad_radius,
                    Color32::from_rgba_unmultiplied(
                        light_base.r(),
                        light_base.g(),
                        light_base.b(),
                        alpha as u8,
                    ),
                );
            }

            // === Layer 3: Ceramic Body Main Color ===
            painter.circle_filled(center, radius * 0.98, base_color);

            // === Layer 4: Bottom Shadow Arc (Depth) ===
            let shadow_arc_start = 0.85;
            let shadow_arc_end = 2.35;
            for i in 0..6 {
                let grad_radius = radius * 0.98 - i as f32 * 0.3;
                self.draw_gradient_arc(
                    painter,
                    center,
                    grad_radius,
                    shadow_arc_start,
                    shadow_arc_end,
                    Color32::from_rgba_unmultiplied(145, 150, 165, 45 - i * 6),
                );
            }

            // === Layer 5: Side Ambient Occlusion ===
            // Left side darker
            for i in 0..4 {
                let ao_radius = radius * 0.96 - i as f32 * 0.5;
                self.draw_gradient_arc(
                    painter,
                    center,
                    ao_radius,
                    -3.3,
                    -2.5,
                    Color32::from_rgba_unmultiplied(170, 175, 185, 30 - i * 6),
                );
            }

            // === Layer 6: Glass Glaze Base Layer ===
            // Large diffuse area where glaze is thickest
            let glaze_center = Pos2::new(center.x - radius * 0.1, center.y - radius * 0.25);
            for i in 0..10 {
                let glaze_radius = radius * (0.75 - i as f32 * 0.04);
                let alpha = 20 + i * 3;
                painter.circle_filled(
                    glaze_center,
                    glaze_radius,
                    Color32::from_rgba_unmultiplied(255, 255, 255, alpha as u8),
                );
            }

            // === Layer 7: Glass Glaze Mid Layer ===
            let mid_glaze_center = Pos2::new(center.x - radius * 0.15, center.y - radius * 0.32);
            for i in 0..6 {
                let mid_radius = radius * (0.5 - i as f32 * 0.05);
                let alpha = 40 + i * 8;
                painter.circle_filled(
                    mid_glaze_center,
                    mid_radius,
                    Color32::from_rgba_unmultiplied(255, 255, 255, alpha as u8),
                );
            }

            // === Layer 8: Glass Fresnel Effect ===
            // Edge brightness (where light catches the curved glass edge)
            for i in 0..5 {
                let fresnel_radius = radius * (0.96 - i as f32 * 0.02);
                painter.circle_stroke(
                    center,
                    fresnel_radius,
                    Stroke::new(0.8, Color32::from_rgba_unmultiplied(255, 255, 255, 30 - i * 5)),
                );
            }

            // === Layer 9: Subsurface Scattering Simulation ===
            // Rim glow where light penetrates the glaze
            let rim_glow_start = -1.8;
            let rim_glow_end = -0.3;
            for i in 0..4 {
                let rim_radius = radius * 0.94 - i as f32 * 0.4;
                self.draw_gradient_arc(
                    painter,
                    center,
                    rim_radius,
                    rim_glow_start,
                    rim_glow_end,
                    Color32::from_rgba_unmultiplied(245, 248, 255, 35 - i * 7),
                );
            }

            // === Layer 10: Specular Highlights ===
            // Primary specular (main light source)
            let specular_primary = Pos2::new(center.x - radius * 0.2, center.y - radius * 0.36);
            for i in 0..5 {
                let spec_radius = radius * (0.18 - i as f32 * 0.025);
                let alpha = 200 - i * 30;
                painter.circle_filled(
                    specular_primary,
                    spec_radius,
                    Color32::from_rgba_unmultiplied(255, 255, 255, alpha as u8),
                );
            }

            // === Layer 11: Secondary Specular ===
            let specular_secondary = Pos2::new(center.x + radius * 0.15, center.y - radius * 0.28);
            for i in 0..3 {
                let spec_radius = radius * (0.1 - i as f32 * 0.02);
                let alpha = 120 - i * 25;
                painter.circle_filled(
                    specular_secondary,
                    spec_radius,
                    Color32::from_rgba_unmultiplied(255, 255, 255, alpha as u8),
                );
            }

            // === Layer 12: Sharp Highlights (Intense Reflections) ===
            // Brightest point - wet glaze look
            let bright_spot_1 = Pos2::new(center.x - radius * 0.25, center.y - radius * 0.41);
            painter.circle_filled(
                bright_spot_1,
                radius * 0.08,
                Color32::from_rgba_unmultiplied(255, 255, 255, 255),
            );
            painter.circle_filled(
                bright_spot_1,
                radius * 0.05,
                Color32::from_rgba_unmultiplied(255, 255, 255, 255),
            );

            // Secondary bright spot
            let bright_spot_2 = Pos2::new(center.x - radius * 0.15, center.y - radius * 0.35);
            painter.circle_filled(
                bright_spot_2,
                radius * 0.04,
                Color32::from_rgba_unmultiplied(255, 255, 255, 240),
            );

            // === Layer 13: Glass Edge Refraction ===
            // Subtle color shift at edges where light refracts
            for i in 0..3 {
                let refract_radius = radius * (0.97 - i as f32 * 0.01);
                painter.circle_stroke(
                    center,
                    refract_radius,
                    Stroke::new(0.5, Color32::from_rgba_unmultiplied(235, 240, 255, 20 - i * 5)),
                );
            }

            // Draw level indicator on the rim (bright white arc)
            let glow_color = self.glow_color.unwrap_or(theme.primary());
            self.draw_rim_indicator(
                painter,
                center,
                radius,
                *value,
                glow_color,
                self.min_angle,
                self.max_angle,
            );

            // Very bright white rim highlight
            painter.circle_stroke(
                center,
                radius - 1.0,
                Stroke::new(2.0, Color32::from_rgba_unmultiplied(255, 255, 255, 200)),
            );

            // Bottom edge shadow for depth
            painter.circle_stroke(
                center,
                radius + 0.5,
                Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, 50)),
            );

        }

        response
    }

    /// Draw gradient arc for ceramic depth
    fn draw_gradient_arc(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        color: Color32,
    ) {
        let segments = 24;
        for i in 0..segments {
            let t = i as f32 / segments as f32;
            let angle = start_angle + t * (end_angle - start_angle);
            let next_angle = start_angle + ((i + 1) as f32 / segments as f32) * (end_angle - start_angle);

            let p1 = Pos2::new(
                center.x + angle.cos() * radius,
                center.y + angle.sin() * radius,
            );
            let p2 = Pos2::new(
                center.x + next_angle.cos() * radius,
                center.y + next_angle.sin() * radius,
            );

            painter.line_segment([p1, p2], Stroke::new(2.0, color));
        }
    }

    /// Draw level indicator on the rim
    fn draw_rim_indicator(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        value: f32,
        color: Color32,
        min_angle: f32,
        max_angle: f32,
    ) {
        let current_angle = min_angle + value * (max_angle - min_angle);
        let segments = 48;

        // Draw very subtle glow layers first (behind the solid rim)
        for glow_layer in 0..3 {
            let glow_offset = (3 - glow_layer) as f32 * 0.8; // Reduced offset for subtlety
            let glow_alpha = (20 - glow_layer * 5) as u8; // Consistent: 20, 15, 10
            let glow_color = Color32::from_rgba_unmultiplied(
                color.r(),
                color.g(),
                color.b(),
                glow_alpha,
            );

            for i in 0..segments {
                let t = i as f32 / segments as f32;
                let angle = min_angle + t * (current_angle - min_angle);

                if angle > current_angle {
                    break;
                }

                let next_angle = min_angle + ((i + 1) as f32 / segments as f32) * (current_angle - min_angle);

                let glow_radius = radius - 0.5 + glow_offset;
                let p1 = Pos2::new(
                    center.x + angle.cos() * glow_radius,
                    center.y + angle.sin() * glow_radius,
                );
                let p2 = Pos2::new(
                    center.x + next_angle.cos() * glow_radius,
                    center.y + next_angle.sin() * glow_radius,
                );

                painter.line_segment([p1, p2], Stroke::new(2.5 + glow_offset, glow_color)); // Thinner stroke
            }
        }

        // Draw solid colored rim on top
        for i in 0..segments {
            let t = i as f32 / segments as f32;
            let angle = min_angle + t * (current_angle - min_angle);

            if angle > current_angle {
                break;
            }

            let next_angle = min_angle + ((i + 1) as f32 / segments as f32) * (current_angle - min_angle);

            let outer_radius = radius - 0.5;
            let p1 = Pos2::new(
                center.x + angle.cos() * outer_radius,
                center.y + angle.sin() * outer_radius,
            );
            let p2 = Pos2::new(
                center.x + next_angle.cos() * outer_radius,
                center.y + next_angle.sin() * outer_radius,
            );

            // Solid colored rim
            painter.line_segment([p1, p2], Stroke::new(4.5, color));
        }
    }
}

impl Default for Knob {
    fn default() -> Self {
        Self::new(0.5)
    }
}

