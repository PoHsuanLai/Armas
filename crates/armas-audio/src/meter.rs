//! Audio Meter Component
//!
//! DAW-style audio level meter with peak hold
//! and customizable color gradients.

use armas_basic::color::{lerp_color, with_alpha, ColorStop, Gradient};
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Response from the audio meter
#[derive(Debug, Clone)]
pub struct MeterResponse {
    /// The UI response
    pub response: Response,
    /// Current meter level (0.0 to 1.0)
    pub level: f32,
    /// Current peak hold value
    pub peak: f32,
}

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
/// A professional DAW-style vertical meter with instant response,
/// peak hold indicator, and customizable color schemes.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # use armas_basic::Theme;
/// # fn example(ui: &mut Ui, theme: &Theme) {
/// use armas_audio::{AudioMeter, MeterStyle};
///
/// // Simple two-color meter
/// let mut level = 0.75;
/// AudioMeter::new(level)
///     .height(200.0)
///     .width(30.0)
///     .show(ui, theme);
///
/// // Traditional VU meter with green/yellow/red zones
/// AudioMeter::new(level)
///     .vu_colors(theme)
///     .show(ui, theme);
///
/// // Segmented LED style
/// AudioMeter::new(level)
///     .style(MeterStyle::Segmented(24))
///     .show(ui, theme);
///
/// // Monochrome with opacity gradient
/// AudioMeter::new(level)
///     .monochrome(theme.primary())
///     .show_scale()
///     .show(ui, theme);
/// # }
/// ```
pub struct AudioMeter {
    /// Target level (0.0 to 1.0)
    target_level: f32,
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
    #[must_use]
    pub const fn new(level: f32) -> Self {
        let clamped_level = level.clamp(0.0, 1.0);
        Self {
            target_level: clamped_level,
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
    #[must_use]
    pub const fn width(mut self, width: f32) -> Self {
        self.width = width.max(10.0);
        self
    }

    /// Set meter height
    #[must_use]
    pub const fn height(mut self, height: f32) -> Self {
        self.height = height.max(20.0);
        self
    }

    /// Set visual style
    #[must_use]
    pub const fn style(mut self, style: MeterStyle) -> Self {
        self.style = style;
        self
    }

    /// Use a custom gradient for the meter
    #[must_use]
    pub fn gradient(mut self, gradient: Gradient) -> Self {
        self.gradient = Some(gradient);
        self
    }

    /// Set color range (will be interpolated smoothly)
    #[must_use]
    pub fn color_range(mut self, min: Color32, max: Color32) -> Self {
        self.min_color = min;
        self.max_color = max;
        self.gradient = None; // Clear custom gradient
        self
    }

    /// Preset: Traditional VU meter colors (green -> yellow -> red)
    #[must_use]
    pub fn vu_colors(mut self, theme: &armas_basic::Theme) -> Self {
        self.gradient = Some(Gradient::new(vec![
            ColorStop::new(0.0, theme.chart_2()),
            ColorStop::new(0.7, theme.chart_3()),
            ColorStop::new(0.9, theme.destructive()),
        ]));
        self
    }

    /// Preset: Monochrome with opacity gradient
    #[must_use]
    pub fn monochrome(mut self, color: Color32) -> Self {
        self.min_color = with_alpha(color, 50);
        self.max_color = color;
        self.gradient = None;
        self
    }

    /// Set peak hold indicator color
    #[must_use]
    pub const fn peak_color(mut self, color: Color32) -> Self {
        self.peak_color = Some(color);
        self
    }

    /// Set scale position
    #[must_use]
    pub const fn scale_position(mut self, position: ScalePosition) -> Self {
        self.scale_position = position;
        self
    }

    /// Show scale on the right (convenience method)
    #[must_use]
    pub const fn show_scale(mut self) -> Self {
        self.scale_position = ScalePosition::Right;
        self
    }

    /// Show scale on the left
    #[must_use]
    pub const fn scale_left(mut self) -> Self {
        self.scale_position = ScalePosition::Left;
        self
    }

    /// Show scale on the right
    #[must_use]
    pub const fn scale_right(mut self) -> Self {
        self.scale_position = ScalePosition::Right;
        self
    }

    /// Set corner radius for background
    #[must_use]
    pub const fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius.max(0.0);
        self
    }

    /// Set background opacity (0.0 to 1.0)
    #[must_use]
    pub const fn background_opacity(mut self, opacity: f32) -> Self {
        self.background_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Enable/disable glassmorphic background
    #[must_use]
    pub const fn glassmorphic(mut self, enabled: bool) -> Self {
        self.glassmorphic = enabled;
        self
    }

    /// Update the target level (call this when audio level changes)
    pub const fn set_level(&mut self, level: f32) {
        self.target_level = level.clamp(0.0, 1.0);
    }

    /// Show the meter and return the response
    pub fn show(mut self, ui: &mut Ui, theme: &armas_basic::Theme) -> MeterResponse {
        // Width only controls the meter tube, scale is additional space
        let scale_width = if self.scale_position == ScalePosition::None {
            0.0
        } else {
            14.0 // Minimal scale width - just enough for text
        };

        // Total allocation = meter width + scale width
        let total_width = self.width + scale_width;
        let desired_size = Vec2::new(total_width, self.height);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        // Use target level directly — no animation lag for accurate metering
        let dt = ui.input(|i| i.stable_dt);
        let current_level = self.target_level.clamp(0.0, 1.0);

        // Update peak hold
        if current_level > self.peak_hold {
            self.peak_hold = current_level;
            self.peak_hold_time = 0.0;
        } else {
            self.peak_hold_time += dt;
            // Hold for 1.5 seconds, then fade over 1.0 seconds
            if self.peak_hold_time > 1.5 {
                let fade_progress = ((self.peak_hold_time - 1.5) / 1.0).min(1.0);
                self.peak_hold = self
                    .peak_hold
                    .mul_add(1.0 - fade_progress, current_level * fade_progress);
            }
        }

        // Request repaint if peak hold is still fading
        if self.peak_hold_time < 2.5 {
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
                let border_color = with_alpha(theme.border(), 150);
                ui.painter().rect_stroke(
                    meter_rect,
                    self.corner_radius,
                    (1.5, border_color),
                    egui::StrokeKind::Middle,
                );
            }

            // Inner meter area (with padding only on scale side for compact layout)
            let inner_meter_rect = if self.scale_position == ScalePosition::None {
                meter_rect // No padding when no scale
            } else if self.scale_position == ScalePosition::Left {
                // Pad left side only
                Rect::from_min_max(
                    Pos2::new(meter_rect.min.x + 2.0, meter_rect.min.y),
                    meter_rect.max,
                )
            } else {
                // Pad right side only
                Rect::from_min_max(
                    meter_rect.min,
                    Pos2::new(meter_rect.max.x - 2.0, meter_rect.max.y),
                )
            };

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
                let peak_y = Self::level_to_display(self.peak_hold)
                    .mul_add(-inner_meter_rect.height(), inner_meter_rect.max.y);
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
                    (f32::from(peak_color.a()) * fade_alpha) as u8,
                );

                ui.painter().line_segment(
                    [
                        Pos2::new(inner_meter_rect.min.x, peak_y),
                        Pos2::new(inner_meter_rect.max.x, peak_y),
                    ],
                    (2.0, peak_with_fade),
                );
            }

            // Draw scale markings (pass full rect which includes scale area)
            if self.scale_position != ScalePosition::None {
                self.draw_scale(ui, rect, meter_rect, theme);
            }
        }

        MeterResponse {
            response,
            level: current_level,
            peak: self.peak_hold,
        }
    }

    /// Map linear amplitude (0.0–1.0) to display position (0.0–1.0).
    ///
    /// Uses a power curve so that lower dB values get more visual space,
    /// matching standard DAW meter ballistics. Without this, the bottom
    /// quarter of the meter would cover -6 dB to -∞, leaving scale labels
    /// bunched together.
    fn level_to_display(level: f32) -> f32 {
        level.clamp(0.0, 1.0).sqrt()
    }

    /// Draw smooth gradient meter fill
    fn draw_smooth_meter(&self, ui: &mut Ui, meter_rect: Rect, level: f32) {
        let painter = ui.painter();

        if level < 0.01 {
            return;
        }

        let corner_radius = (meter_rect.width() * 0.5).min(6.0);
        let display_level = Self::level_to_display(level);

        // Sample colors from bottom to top
        let steps = 50;
        for i in 0..steps {
            let t = i as f32 / steps as f32;
            let next_t = (i + 1) as f32 / steps as f32;

            if t > display_level {
                break;
            }

            let capped_next = next_t.min(display_level);

            // Map display position back to linear level for color lookup
            let color_t = t * t; // inverse of sqrt
            let base_color = self.get_color_at_level(color_t);

            // Subtle brightness increase for the fill itself
            let brightness = level.powf(1.5).mul_add(0.3, 1.0);

            let color = Color32::from_rgba_unmultiplied(
                ((f32::from(base_color.r()) * brightness).min(255.0)) as u8,
                ((f32::from(base_color.g()) * brightness).min(255.0)) as u8,
                ((f32::from(base_color.b()) * brightness).min(255.0)) as u8,
                base_color.a(),
            );

            let segment_min_y = meter_rect.max.y - (capped_next * meter_rect.height());
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
        let lit_segments = (Self::level_to_display(level) * segment_count as f32).ceil() as usize;
        let corner_radius = (segment_height * 0.5).min(4.0);

        // Glow intensity increases with level
        let glow_intensity = level.powf(1.5);

        for i in 0..segment_count {
            let t = (i as f32 + 0.5) / segment_count as f32;
            let segment_y = ((i + 1) as f32).mul_add(-(segment_height + gap), meter_rect.max.y);

            let segment_rect = Rect::from_min_size(
                Pos2::new(meter_rect.min.x, segment_y),
                Vec2::new(meter_rect.width(), segment_height),
            );

            let is_lit = i < lit_segments;
            let base_color = self.get_color_at_level(t);

            if is_lit {
                // Subtle outer glow for lit segments - consistent with fader and knob
                for layer in 0..3 {
                    let expansion = (3 - layer) as f32 * 0.8;
                    let alpha = ((20 - layer * 5) as f32 * glow_intensity) as u8; // Consistent: 20, 15, 10

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
                let brightness = level.powf(1.5).mul_add(0.3, 1.0);
                let color = Color32::from_rgba_unmultiplied(
                    ((f32::from(base_color.r()) * brightness).min(255.0)) as u8,
                    ((f32::from(base_color.g()) * brightness).min(255.0)) as u8,
                    ((f32::from(base_color.b()) * brightness).min(255.0)) as u8,
                    base_color.a(),
                );

                painter.rect_filled(segment_rect, corner_radius, color);
            } else {
                // Dim unlit segments
                let dim_color = with_alpha(base_color, 20);
                painter.rect_filled(segment_rect, corner_radius, dim_color);
            }
        }
    }

    /// Get color at a specific level (0.0 to 1.0)
    fn get_color_at_level(&self, level: f32) -> Color32 {
        self.gradient.as_ref().map_or_else(
            || lerp_color(self.min_color, self.max_color, level),
            |gradient| gradient.sample(level),
        )
    }

    /// Draw dB scale markings
    /// `full_rect`: the entire allocated space including scale area
    /// `meter_rect`: just the meter bar area (for positioning scale relative to meter)
    fn draw_scale(&self, ui: &mut Ui, full_rect: Rect, meter_rect: Rect, theme: &armas_basic::Theme) {
        let painter = ui.painter();
        let text_color = theme.muted_foreground();

        // dB levels with proper logarithmic scaling
        // Formula: linear = 10^(dB/20)
        let db_marks = [
            (1.0, "0"),     // 0 dB - digital maximum
            (0.708, "-3"),  // -3 dB
            (0.501, "-6"),  // -6 dB - safe headroom
            (0.251, "-12"), // -12 dB - good average
            (0.126, "-18"), // -18 dB - reference level
            (0.063, "-24"), // -24 dB
            (0.0, "-∞"),    // -inf dB - silence
        ];

        let is_left = self.scale_position == ScalePosition::Left;

        for (level, label) in db_marks {
            let y = meter_rect.max.y - Self::level_to_display(level) * meter_rect.height();

            // Position text in the scale area (outside the meter)
            let (text_pos, text_align) = if is_left {
                // Scale is on the left side of full_rect, outside the meter
                (
                    Pos2::new(full_rect.min.x + 1.0, y),
                    egui::Align2::LEFT_CENTER,
                )
            } else {
                // Scale is on the right side of full_rect, outside the meter
                (
                    Pos2::new(full_rect.max.x - 1.0, y),
                    egui::Align2::RIGHT_CENTER,
                )
            };

            // Draw text label only (no tick marks)
            painter.text(
                text_pos,
                text_align,
                label,
                egui::FontId::proportional(9.0),
                text_color,
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
        assert_eq!(meter.width, 22.0);
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
        assert!(!meter.glassmorphic);
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
