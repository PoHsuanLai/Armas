//! Audio Meter Component
//!
//! Professional DAW-style audio level meter with smooth animations,
//! peak hold, and customizable color gradients.

use crate::animation::SpringAnimation;
use crate::color::{lerp_color, with_alpha, ColorStop, Gradient};
use crate::ext::ArmasContextExt;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Visual style for the meter display
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeterStyle {
    /// Smooth gradient fill
    Smooth,
    /// LED segment display with specified number of segments
    Segmented(u8),
}

/// Scale position for dB markings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalePosition {
    /// Scale on the left side
    Left,
    /// Scale on the right side
    Right,
    /// No scale
    None,
}

/// Audio level meter component
///
/// A professional DAW-style vertical meter with smooth spring-based animation,
/// peak hold indicator, and customizable color schemes.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # use armas::Theme;
/// # fn example(ui: &mut Ui, theme: &Theme) {
/// use armas::components::{AudioMeter, MeterStyle};
///
/// // Simple two-color meter
/// let mut level = 0.75;
/// AudioMeter::new(level)
///     .height(200.0)
///     .width(30.0)
///     .show(ui);
///
/// // Traditional VU meter with green/yellow/red zones
/// AudioMeter::new(level)
///     .vu_colors(theme)
///     .show(ui);
///
/// // Segmented LED style
/// AudioMeter::new(level)
///     .style(MeterStyle::Segmented(24))
///     .show(ui);
///
/// // Monochrome with opacity gradient
/// AudioMeter::new(level)
///     .monochrome(theme.primary())
///     .show_scale(true)
///     .show(ui);
/// # }
/// ```
pub struct AudioMeter {
    /// Target level (0.0 to 1.0)
    target_level: f32,
    /// Spring animation for smooth level changes
    level_animation: SpringAnimation,
    /// Peak hold value
    peak_hold: f32,
    /// Time since peak was hit (for fade out)
    peak_hold_time: f32,
    /// Custom gradient (takes precedence over color range)
    gradient: Option<Gradient>,
    /// Minimum level color (used when gradient is None)
    min_color: Color32,
    /// Maximum level color (used when gradient is None)
    max_color: Color32,
    /// Peak hold indicator color
    peak_color: Option<Color32>,
    /// Meter width
    width: f32,
    /// Meter height
    height: f32,
    /// Visual style
    style: MeterStyle,
    /// Scale position
    scale_position: ScalePosition,
    /// Corner radius for glassmorphic background
    corner_radius: f32,
    /// Background opacity (0.0 to 1.0)
    background_opacity: f32,
    /// Enable glassmorphic background
    glassmorphic: bool,
}

impl AudioMeter {
    /// Create a new audio meter with default green-to-red gradient
    pub fn new(level: f32) -> Self {
        let clamped_level = level.clamp(0.0, 1.0);
        Self {
            target_level: clamped_level,
            level_animation: SpringAnimation::new(clamped_level, clamped_level).params(250.0, 18.0), // Fast response, light damping
            peak_hold: clamped_level,
            peak_hold_time: 0.0,
            gradient: None,
            min_color: Color32::from_rgb(0, 150, 0), // Dark green
            max_color: Color32::from_rgb(255, 0, 0), // Red
            peak_color: None,                        // Will use theme primary by default
            width: 22.0,
            height: 200.0,
            style: MeterStyle::Smooth,
            scale_position: ScalePosition::None,
            corner_radius: 16.0,
            background_opacity: 0.3,
            glassmorphic: true,
        }
    }

    /// Set meter width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(10.0);
        self
    }

    /// Set meter height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(20.0);
        self
    }

    /// Set visual style
    pub fn style(mut self, style: MeterStyle) -> Self {
        self.style = style;
        self
    }

    /// Use a custom gradient for the meter
    pub fn gradient(mut self, gradient: Gradient) -> Self {
        self.gradient = Some(gradient);
        self
    }

    /// Set color range (will be interpolated smoothly)
    pub fn color_range(mut self, min: Color32, max: Color32) -> Self {
        self.min_color = min;
        self.max_color = max;
        self.gradient = None; // Clear custom gradient
        self
    }

    /// Preset: Traditional VU meter colors (green -> yellow -> red)
    pub fn vu_colors(mut self, theme: &crate::Theme) -> Self {
        self.gradient = Some(Gradient::new(vec![
            ColorStop::new(0.0, theme.success()),
            ColorStop::new(0.7, theme.warning()),
            ColorStop::new(0.9, theme.error()),
        ]));
        self
    }

    /// Preset: Monochrome with opacity gradient
    pub fn monochrome(mut self, color: Color32) -> Self {
        self.min_color = with_alpha(color, 50);
        self.max_color = color;
        self.gradient = None;
        self
    }

    /// Set peak hold indicator color
    pub fn peak_color(mut self, color: Color32) -> Self {
        self.peak_color = Some(color);
        self
    }

    /// Set scale position
    pub fn scale_position(mut self, position: ScalePosition) -> Self {
        self.scale_position = position;
        self
    }

    /// Show scale on the right (convenience method)
    pub fn show_scale(mut self) -> Self {
        self.scale_position = ScalePosition::Right;
        self
    }

    /// Show scale on the left
    pub fn scale_left(mut self) -> Self {
        self.scale_position = ScalePosition::Left;
        self
    }

    /// Show scale on the right
    pub fn scale_right(mut self) -> Self {
        self.scale_position = ScalePosition::Right;
        self
    }

    /// Set corner radius for background
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius.max(0.0);
        self
    }

    /// Set background opacity (0.0 to 1.0)
    pub fn background_opacity(mut self, opacity: f32) -> Self {
        self.background_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Enable/disable glassmorphic background
    pub fn glassmorphic(mut self, enabled: bool) -> Self {
        self.glassmorphic = enabled;
        self
    }

    /// Update the target level (call this when audio level changes)
    pub fn set_level(&mut self, level: f32) {
        self.target_level = level.clamp(0.0, 1.0);
        self.level_animation.set_target(self.target_level);
    }

    /// Show the meter and return the response
    pub fn show(mut self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();

        // Width only controls the meter tube, scale is additional space
        let scale_width = if self.scale_position != ScalePosition::None {
            25.0 // Space for scale text and tick marks
        } else {
            0.0
        };

        // Total allocation = meter width + scale width
        let total_width = self.width + scale_width;
        let desired_size = Vec2::new(total_width, self.height);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        // Update animation
        let dt = ui.input(|i| i.stable_dt);
        self.level_animation.update(dt);
        let current_level = self.level_animation.value.clamp(0.0, 1.0);

        // Update peak hold
        if current_level > self.peak_hold {
            self.peak_hold = current_level;
            self.peak_hold_time = 0.0;
        } else {
            self.peak_hold_time += dt;
            // Hold for 1.5 seconds, then fade over 1.0 seconds
            if self.peak_hold_time > 1.5 {
                let fade_progress = ((self.peak_hold_time - 1.5) / 1.0).min(1.0);
                self.peak_hold =
                    self.peak_hold * (1.0 - fade_progress) + current_level * fade_progress;
            }
        }

        // Request repaint if animating
        if !self.level_animation.is_settled(0.001, 0.001) || self.peak_hold_time < 2.5 {
            ui.ctx().request_repaint();
        }

        if ui.is_rect_visible(rect) {
            // Calculate meter rect (the actual meter bar area, always self.width wide)
            let meter_rect = if self.scale_position == ScalePosition::Left {
                // Scale on left, meter on right
                Rect::from_min_size(
                    Pos2::new(rect.min.x + scale_width, rect.min.y),
                    Vec2::new(self.width, rect.height()),
                )
            } else if self.scale_position == ScalePosition::Right {
                // Scale on right, meter on left
                Rect::from_min_size(rect.min, Vec2::new(self.width, rect.height()))
            } else {
                // No scale, meter uses full allocated width
                rect
            };

            // Draw glassmorphic background
            if self.glassmorphic {
                // Brighter border for glass edge
                let border_color = with_alpha(theme.outline_variant(), 150);
                ui.painter().rect_stroke(
                    meter_rect,
                    self.corner_radius,
                    (1.5, border_color),
                    egui::StrokeKind::Middle,
                );
            }

            // Inner meter area (with padding)
            let padding = 6.0;
            let inner_meter_rect = meter_rect.shrink(padding);

            // Draw meter fill based on style
            match self.style {
                MeterStyle::Smooth => {
                    self.draw_smooth_meter(ui, inner_meter_rect, current_level);
                }
                MeterStyle::Segmented(segment_count) => {
                    self.draw_segmented_meter(ui, inner_meter_rect, current_level, segment_count);
                }
            }

            // Draw peak hold indicator
            if self.peak_hold > 0.01 && self.peak_hold_time < 2.5 {
                let peak_y = inner_meter_rect.max.y - (self.peak_hold * inner_meter_rect.height());
                let peak_color = self.peak_color.unwrap_or_else(|| theme.primary());

                // Fade out after hold period
                let fade_alpha = if self.peak_hold_time > 1.5 {
                    1.0 - ((self.peak_hold_time - 1.5) / 1.0).min(1.0)
                } else {
                    1.0
                };

                let peak_with_fade = Color32::from_rgba_unmultiplied(
                    peak_color.r(),
                    peak_color.g(),
                    peak_color.b(),
                    (peak_color.a() as f32 * fade_alpha) as u8,
                );

                ui.painter().line_segment(
                    [
                        Pos2::new(inner_meter_rect.min.x, peak_y),
                        Pos2::new(inner_meter_rect.max.x, peak_y),
                    ],
                    (2.0, peak_with_fade),
                );
            }

            // Draw scale markings
            if self.scale_position != ScalePosition::None {
                self.draw_scale(ui, meter_rect, &theme);
            }
        }

        response
    }

    /// Draw smooth gradient meter fill
    fn draw_smooth_meter(&self, ui: &mut Ui, meter_rect: Rect, level: f32) {
        let painter = ui.painter();

        if level < 0.01 {
            return;
        }

        let corner_radius = (meter_rect.width() * 0.5).min(6.0);
        let fill_height = meter_rect.height() * level;

        // Calculate fill rect (the actual colored area)
        let fill_rect = Rect::from_min_max(
            Pos2::new(meter_rect.min.x, meter_rect.max.y - fill_height),
            meter_rect.max,
        );

        // Draw outer glow for the colored fill (more obvious)
        let glow_intensity = level.powf(1.5);
        let fill_color = self.get_color_at_level(level);

        for layer in 0..5 {
            let expansion = (layer + 1) as f32 * 1.2;
            let alpha = (35.0 * glow_intensity * (1.0 - layer as f32 / 5.0)) as u8;

            let glow_rect = fill_rect.expand(expansion);
            let glow_with_alpha = Color32::from_rgba_unmultiplied(
                fill_color.r(),
                fill_color.g(),
                fill_color.b(),
                alpha,
            );

            painter.rect_filled(glow_rect, corner_radius + expansion, glow_with_alpha);
        }

        // Sample colors from bottom to top for the actual fill
        let steps = 50;
        for i in 0..steps {
            let t = i as f32 / steps as f32;
            let next_t = (i + 1) as f32 / steps as f32;

            if next_t > level {
                break;
            }

            // Get base color and brighten it slightly based on level
            let base_color = self.get_color_at_level(t);

            // Subtle brightness increase for the fill itself
            let brightness = 1.0 + (level.powf(1.5) * 0.3);

            let color = Color32::from_rgba_unmultiplied(
                ((base_color.r() as f32 * brightness).min(255.0)) as u8,
                ((base_color.g() as f32 * brightness).min(255.0)) as u8,
                ((base_color.b() as f32 * brightness).min(255.0)) as u8,
                base_color.a(),
            );

            let segment_min_y = meter_rect.max.y - (next_t * meter_rect.height());
            let segment_max_y = meter_rect.max.y - (t * meter_rect.height());

            let segment_rect = Rect::from_min_max(
                Pos2::new(meter_rect.min.x, segment_min_y),
                Pos2::new(meter_rect.max.x, segment_max_y),
            );

            // Round bottom corners only
            let rounding = if t < 0.1 { corner_radius } else { 0.0 };
            painter.rect_filled(segment_rect, rounding, color);
        }
    }

    /// Draw segmented LED-style meter
    fn draw_segmented_meter(&self, ui: &mut Ui, meter_rect: Rect, level: f32, segment_count: u8) {
        let painter = ui.painter();
        let segment_count = segment_count.max(1) as usize;
        let gap = 2.0;
        let segment_height =
            (meter_rect.height() - (gap * (segment_count - 1) as f32)) / segment_count as f32;
        let lit_segments = (level * segment_count as f32).ceil() as usize;
        let corner_radius = (segment_height * 0.5).min(4.0);

        // Glow intensity increases with level
        let glow_intensity = level.powf(1.5);

        for i in 0..segment_count {
            let t = (i as f32 + 0.5) / segment_count as f32;
            let segment_y = meter_rect.max.y - ((i + 1) as f32 * (segment_height + gap));

            let segment_rect = Rect::from_min_size(
                Pos2::new(meter_rect.min.x, segment_y),
                Vec2::new(meter_rect.width(), segment_height),
            );

            let is_lit = i < lit_segments;

            if is_lit {
                let base_color = self.get_color_at_level(t);

                // More obvious outer glow for lit segments
                for layer in 0..4 {
                    let expansion = (layer + 1) as f32 * 1.0;
                    let alpha = (30.0 * glow_intensity * (1.0 - layer as f32 / 4.0)) as u8;

                    let glow_rect = segment_rect.expand(expansion);
                    let glow_with_alpha = Color32::from_rgba_unmultiplied(
                        base_color.r(),
                        base_color.g(),
                        base_color.b(),
                        alpha,
                    );

                    painter.rect_filled(glow_rect, corner_radius + expansion, glow_with_alpha);
                }

                // Subtle brightness increase for the segment itself
                let brightness = 1.0 + (level.powf(1.5) * 0.3);
                let color = Color32::from_rgba_unmultiplied(
                    ((base_color.r() as f32 * brightness).min(255.0)) as u8,
                    ((base_color.g() as f32 * brightness).min(255.0)) as u8,
                    ((base_color.b() as f32 * brightness).min(255.0)) as u8,
                    base_color.a(),
                );

                painter.rect_filled(segment_rect, corner_radius, color);
            } else {
                // Dim unlit segments
                let base_color = self.get_color_at_level(t);
                let dim_color = with_alpha(base_color, 20);
                painter.rect_filled(segment_rect, corner_radius, dim_color);
            }
        }
    }

    /// Get color at a specific level (0.0 to 1.0)
    fn get_color_at_level(&self, level: f32) -> Color32 {
        if let Some(ref gradient) = self.gradient {
            gradient.sample(level)
        } else {
            lerp_color(self.min_color, self.max_color, level)
        }
    }

    /// Draw dB scale markings
    fn draw_scale(&self, ui: &mut Ui, rect: Rect, theme: &crate::Theme) {
        let painter = ui.painter();
        let text_color = theme.on_surface_variant();

        // dB levels: 0, -6, -12, -18, -24, -inf
        let db_marks = [
            (1.0, "0"),
            (0.5, "-6"),
            (0.25, "-12"),
            (0.125, "-18"),
            (0.063, "-24"),
            (0.0, "-∞"),
        ];

        let meter_rect = rect.shrink(6.0);
        let is_left = self.scale_position == ScalePosition::Left;

        for (level, label) in db_marks {
            let y = meter_rect.max.y - (level * meter_rect.height());

            // Position text and tick based on scale position
            let (text_pos, text_align, tick_start_x, tick_end_x) = if is_left {
                (
                    Pos2::new(rect.min.x - 4.0, y),
                    egui::Align2::RIGHT_CENTER,
                    meter_rect.min.x - 3.0,
                    meter_rect.min.x - 1.0,
                )
            } else {
                (
                    Pos2::new(rect.max.x + 4.0, y),
                    egui::Align2::LEFT_CENTER,
                    meter_rect.max.x + 1.0,
                    meter_rect.max.x + 3.0,
                )
            };

            painter.text(
                text_pos,
                text_align,
                label,
                egui::FontId::proportional(9.0),
                text_color,
            );

            // Small tick mark
            painter.line_segment(
                [Pos2::new(tick_start_x, y), Pos2::new(tick_end_x, y)],
                (1.0, text_color),
            );
        }
    }
}

impl Default for AudioMeter {
    fn default() -> Self {
        Self::new(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_meter_creation() {
        let meter = AudioMeter::new(0.5);
        assert_eq!(meter.target_level, 0.5);
        assert_eq!(meter.width, 30.0);
        assert_eq!(meter.height, 200.0);
    }

    #[test]
    fn test_audio_meter_config() {
        let meter = AudioMeter::new(0.0)
            .width(40.0)
            .height(300.0)
            .show_scale()
            .glassmorphic(false);

        assert_eq!(meter.width, 40.0);
        assert_eq!(meter.height, 300.0);
        assert_eq!(meter.scale_position, ScalePosition::Right);
        assert_eq!(meter.glassmorphic, false);
    }

    #[test]
    fn test_meter_style() {
        let meter = AudioMeter::new(0.5).style(MeterStyle::Segmented(16));
        assert_eq!(meter.style, MeterStyle::Segmented(16));
    }

    #[test]
    fn test_level_clamping() {
        let mut meter = AudioMeter::new(1.5);
        assert_eq!(meter.target_level, 1.0);

        meter.set_level(-0.5);
        assert_eq!(meter.target_level, 0.0);
    }

    #[test]
    fn test_color_range() {
        let meter = AudioMeter::new(0.5).color_range(Color32::BLUE, Color32::RED);

        assert_eq!(meter.min_color, Color32::BLUE);
        assert_eq!(meter.max_color, Color32::RED);
        assert!(meter.gradient.is_none());
    }
}
