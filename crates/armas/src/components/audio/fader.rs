//! Vertical fader component inspired by DAW mixer faders
//!
//! This module provides two components:
//! - `Fader`: Minimal interactive slider (track + channel + thumb)
//! - `FaderStrip`: Complete fader with housing and optional LED (batteries included)

use crate::ext::ArmasContextExt;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Scale position for dB markings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaderScalePosition {
    /// Scale on the left side
    Left,
    /// Scale on the right side
    Right,
    /// No scale
    None,
}

// Fader (minimal) default dimensions - just the track
const FADER_DEFAULT_WIDTH: f32 = 30.0; // Track width from original
const FADER_DEFAULT_HEIGHT: f32 = 240.0; // Track height from original

// FaderStrip (complete) default dimensions - includes housing
const STRIP_DEFAULT_WIDTH: f32 = 39.0;
const STRIP_DEFAULT_HEIGHT: f32 = 254.0;

// Housing (outer grey box) dimensions - only used by FaderStrip
const HOUSING_MARGIN_TOP: f32 = 7.0;
const HOUSING_MARGIN_BOTTOM: f32 = 7.0;
const HOUSING_CORNER_RADIUS: f32 = 4.0;
const HOUSING_GRADIENT_STEPS: usize = 20;

// Channel (slider crack) dimensions
const CHANNEL_WIDTH: f32 = 8.0;
const CHANNEL_CORNER_RADIUS: f32 = 4.0;

// Thumb dimensions
const THUMB_WIDTH: f32 = 20.0;
const THUMB_HEIGHT: f32 = 54.0;

/// Minimal vertical fader component (track + channel + thumb only)
///
/// This is the core interactive slider without any container/housing.
/// Use this when you want to place the fader in your own container or card.
pub struct Fader {
    id: Option<egui::Id>,
    width: f32,
    height: f32,
    value: f32,
    scale_position: FaderScalePosition,
}

impl Fader {
    /// Create a new minimal fader with default dimensions (track only)
    pub fn new(value: f32) -> Self {
        Self {
            id: None,
            width: FADER_DEFAULT_WIDTH,
            height: FADER_DEFAULT_HEIGHT,
            value: value.clamp(0.0, 1.0),
            scale_position: FaderScalePosition::None,
        }
    }

    /// Set ID for state persistence (useful when fader is recreated each frame)
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set custom size
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Show scale on the right (convenience method)
    pub fn show_scale(mut self) -> Self {
        self.scale_position = FaderScalePosition::Right;
        self
    }

    /// Show scale on the left
    pub fn scale_left(mut self) -> Self {
        self.scale_position = FaderScalePosition::Left;
        self
    }

    /// Show scale on the right
    pub fn scale_right(mut self) -> Self {
        self.scale_position = FaderScalePosition::Right;
        self
    }

    /// Show the fader and return the new value
    pub fn show(mut self, ui: &mut Ui) -> (Response, f32) {
        let theme = ui.ctx().armas_theme();

        // Load state from memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("fader_state");
            let stored_value: f32 = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id).unwrap_or(self.value));
            self.value = stored_value;
        }

        // Clamp value to valid range
        self.value = self.value.clamp(0.0, 1.0);

        // Width controls the fader track, scale is additional space
        let scale_width = if self.scale_position != FaderScalePosition::None {
            10.0 // Minimal scale width - just enough for text
        } else {
            0.0
        };

        // Total allocation = fader width + scale width
        let total_width = self.width + scale_width;
        let desired_size = Vec2::new(total_width, self.height);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        // Calculate fader rect (the actual fader area, always self.width wide)
        let fader_rect = if self.scale_position == FaderScalePosition::Left {
            // Scale on left, fader on right
            Rect::from_min_size(
                Pos2::new(rect.min.x + scale_width, rect.min.y),
                Vec2::new(self.width, rect.height()),
            )
        } else if self.scale_position == FaderScalePosition::Right {
            // Scale on right, fader on left
            Rect::from_min_size(rect.min, Vec2::new(self.width, rect.height()))
        } else {
            // No scale, fader uses full allocated width
            rect
        };

        // Handle interaction
        if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                // Calculate scale for interaction bounds
                let scale_y = self.height / FADER_DEFAULT_HEIGHT;

                // Channel bounds (where the thumb actually moves) - now full height
                let channel_top = fader_rect.min.y;
                let channel_bottom = fader_rect.max.y;
                let channel_height = channel_bottom - channel_top;

                // Account for thumb height to keep it fully inside the channel
                let thumb_height = THUMB_HEIGHT * scale_y;
                let thumb_travel_range = channel_height - thumb_height;

                // Calculate value from Y position (inverted: top = 1.0, bottom = 0.0)
                // Thumb top edge ranges from channel_top to channel_top + thumb_travel_range
                let thumb_top_pos = (pos.y - channel_top).clamp(0.0, thumb_travel_range);
                let normalized = 1.0 - (thumb_top_pos / thumb_travel_range);
                self.value = normalized;
                response.mark_changed();
            }
        }

        if ui.is_rect_visible(fader_rect) {
            let painter = ui.painter();

            // Calculate scale factor based on default size
            let scale_x = self.width / FADER_DEFAULT_WIDTH;
            let scale_y = self.height / FADER_DEFAULT_HEIGHT;
            let scale = scale_x.min(scale_y); // Use uniform scale to maintain proportions

            // Track plate dimensions
            let track_rect = fader_rect;

            // Draw the slider channel/crack (vertical dark gradient track)
            // Channel fills the full height (no internal padding)
            // Shift channel position based on scale side
            let channel_width = CHANNEL_WIDTH * scale_x;
            let channel_x = if self.scale_position == FaderScalePosition::Left {
                // Pad left side when scale is on left
                track_rect.min.x + 2.0 + (self.width - 2.0 - channel_width) / 2.0
            } else if self.scale_position == FaderScalePosition::Right {
                // Pad right side when scale is on right
                track_rect.min.x + (self.width - 2.0 - channel_width) / 2.0
            } else {
                // Center when no scale
                track_rect.min.x + (self.width - channel_width) / 2.0
            };
            let channel_y = track_rect.min.y;
            let channel_height = self.height;

            let channel_rect = Rect::from_min_size(
                Pos2::new(channel_x, channel_y),
                Vec2::new(channel_width, channel_height),
            );

            // Draw channel background
            painter.rect_filled(
                channel_rect,
                CHANNEL_CORNER_RADIUS * scale,
                theme.background(),
            );

            // Add border to channel
            painter.rect_stroke(
                channel_rect,
                CHANNEL_CORNER_RADIUS * scale,
                (1.0 * scale, theme.outline()),
                egui::StrokeKind::Middle,
            );

            // Draw fader thumb (scaled)
            let thumb_height = THUMB_HEIGHT * scale_y;
            let thumb_width = THUMB_WIDTH * scale_x;

            // Calculate thumb travel range (keep thumb fully inside channel)
            let thumb_travel_range = channel_height - thumb_height;
            let thumb_y = channel_y + (1.0 - self.value) * thumb_travel_range;

            // Center thumb horizontally in track
            let thumb_x = track_rect.min.x + (self.width - thumb_width) / 2.0;

            self.draw_fader_thumb(
                painter,
                Pos2::new(thumb_x, thumb_y),
                thumb_width,
                thumb_height,
                scale,
                &theme,
            );

            // Draw scale markings
            if self.scale_position != FaderScalePosition::None {
                self.draw_scale(ui, fader_rect, rect, &theme);
            }
        }

        // Save state to memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("fader_state");
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id, self.value);
            });
        }

        (response, self.value)
    }

    /// Draw dB scale markings for fader
    fn draw_scale(&self, ui: &mut Ui, fader_rect: Rect, full_rect: Rect, theme: &crate::Theme) {
        let painter = ui.painter();
        let text_color = theme.on_surface_variant();

        // Fader dB scale (fader represents gain/volume control)
        // 0 dB at 75% (unity gain), with boost above and attenuation below
        let db_marks = [
            (1.0, "+6"),   // +6 dB - boost
            (0.87, "+3"),  // +3 dB
            (0.75, "0"),   // 0 dB - unity gain (most important!)
            (0.59, "-3"),  // -3 dB
            (0.44, "-6"),  // -6 dB
            (0.25, "-12"), // -12 dB
            (0.13, "-18"), // -18 dB
            (0.0, "-∞"),   // -inf dB - fully muted
        ];

        let is_left = self.scale_position == FaderScalePosition::Left;

        for (level, label) in db_marks {
            let y = fader_rect.max.y - (level * fader_rect.height());

            // Position text based on scale position (no tick marks, tight spacing)
            let (text_pos, text_align) = if is_left {
                (
                    Pos2::new(full_rect.min.x + 1.0, y),
                    egui::Align2::LEFT_CENTER,
                )
            } else {
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
                egui::FontId::monospace(9.0),
                text_color,
            );
        }
    }

    fn draw_fader_thumb(
        &self,
        painter: &egui::Painter,
        pos: Pos2,
        width: f32,
        height: f32,
        scale: f32,
        theme: &crate::theme::Theme,
    ) {
        let rect = Rect::from_min_size(pos, Vec2::new(width, height));

        // Calculate relative dimensions based on scaled width/height
        let w_scale = width / THUMB_WIDTH;
        let h_scale = height / THUMB_HEIGHT;

        // Determine brightness based on theme - brighter in dark mode
        let bg = theme.background();
        let bg_luminance = (bg.r() as u16 + bg.g() as u16 + bg.b() as u16) / 3;
        let is_dark = bg_luminance < 128;

        // Base gray - brighter in dark mode, darker in light mode
        let base_gray = if is_dark { 120u8 } else { 80u8 };

        // Glow effect (outer shadow) - subtle
        for i in 0..3 {
            let glow_offset = (3 - i) as f32 * scale;
            let glow_alpha = (20 - i * 5) as u8; // More subtle: 20, 15, 10
            painter.rect_filled(
                rect.expand(glow_offset),
                2.0,
                Color32::from_rgba_unmultiplied(255, 255, 255, glow_alpha),
            );
        }

        // Top cap
        let top_cap = Rect::from_min_size(
            pos + Vec2::new(1.0 * w_scale, 0.0),
            Vec2::new((THUMB_WIDTH - 2.0) * w_scale, 4.0 * h_scale),
        );
        painter.rect_filled(
            top_cap,
            0.0,
            Color32::from_rgb(base_gray + 20, base_gray + 20, base_gray + 20),
        );

        // Top cap highlight
        painter.line_segment(
            [
                pos + Vec2::new(1.0 * w_scale, 0.0),
                pos + Vec2::new((THUMB_WIDTH - 1.0) * w_scale, 0.0),
            ],
            (
                1.0 * scale,
                Color32::from_rgb(base_gray + 50, base_gray + 50, base_gray + 50),
            ),
        );

        // Top slanted face
        let top_slant = Rect::from_min_size(
            pos + Vec2::new(2.0 * w_scale, 5.0 * h_scale),
            Vec2::new((THUMB_WIDTH - 4.0) * w_scale, 3.0 * h_scale),
        );
        painter.rect_filled(
            top_slant,
            0.0,
            Color32::from_rgb(base_gray - 30, base_gray - 30, base_gray - 30),
        );

        // Main body (gradient)
        let gradient_lines = (39.0 * h_scale) as usize;
        for i in 0..gradient_lines {
            let t = i as f32 / (gradient_lines - 1) as f32;
            let gray_value = ((base_gray - 10) as f32 + t * 40.0) as u8;
            let color = Color32::from_rgb(gray_value, gray_value, gray_value);
            painter.line_segment(
                [
                    pos + Vec2::new(2.0 * w_scale, 8.0 * h_scale + i as f32 * h_scale),
                    pos + Vec2::new(
                        (THUMB_WIDTH - 2.0) * w_scale,
                        8.0 * h_scale + i as f32 * h_scale,
                    ),
                ],
                (1.0 * scale, color),
            );
        }

        // Draw ridges (horizontal dark lines) (scaled)
        let ridge_count = (20.0 * h_scale) as usize;
        for i in 0..ridge_count {
            let y = pos.y + 9.0 * h_scale + i as f32 * 2.0 * h_scale;
            painter.line_segment(
                [
                    Pos2::new(pos.x + 2.0 * w_scale, y),
                    Pos2::new(pos.x + (THUMB_WIDTH - 2.0) * w_scale, y),
                ],
                (1.0 * scale, Color32::from_rgba_unmultiplied(0, 0, 0, 80)),
            );
        }

        // Finger groove (indent in the middle) (scaled)
        let groove = Rect::from_min_size(
            pos + Vec2::new(1.0 * w_scale, 24.0 * h_scale),
            Vec2::new((THUMB_WIDTH - 2.0) * w_scale, 5.0 * h_scale),
        );
        painter.rect_filled(
            groove,
            0.0,
            Color32::from_rgb(base_gray + 60, base_gray + 60, base_gray + 60),
        );
        painter.rect_stroke(
            groove,
            0.0,
            (
                1.0 * scale,
                Color32::from_rgb(base_gray - 40, base_gray - 40, base_gray - 40),
            ),
            egui::StrokeKind::Middle,
        );

        // Bottom cap
        let bottom_cap = Rect::from_min_size(
            pos + Vec2::new(1.0 * w_scale, height - 6.0 * h_scale),
            Vec2::new((THUMB_WIDTH - 2.0) * w_scale, 6.0 * h_scale),
        );
        painter.rect_filled(
            bottom_cap,
            0.0,
            Color32::from_rgb(base_gray - 35, base_gray - 35, base_gray - 35),
        );

        // Side borders (scaled)
        painter.line_segment(
            [
                pos + Vec2::new(1.0 * w_scale, 5.0 * h_scale),
                pos + Vec2::new(1.0 * w_scale, height - 6.0 * h_scale),
            ],
            (
                1.0 * scale,
                Color32::from_rgb(base_gray - 45, base_gray - 45, base_gray - 45),
            ),
        );
        painter.line_segment(
            [
                pos + Vec2::new((THUMB_WIDTH - 1.0) * w_scale, 5.0 * h_scale),
                pos + Vec2::new((THUMB_WIDTH - 1.0) * w_scale, height - 6.0 * h_scale),
            ],
            (
                1.0 * scale,
                Color32::from_rgb(base_gray - 40, base_gray - 40, base_gray - 40),
            ),
        );

        // Drop shadow (scaled)
        painter.rect_filled(
            rect.translate(Vec2::new(0.0, 2.0 * scale)),
            0.0,
            Color32::from_rgba_unmultiplied(0, 0, 0, 80),
        );
    }
}

/// Complete fader strip with housing (grey box container)
///
/// This is the "batteries included" version that includes:
/// - Grey gradient housing box
/// - Inner fader track with channel and thumb
///
/// Use this when you want a complete, ready-to-use fader component.
pub struct FaderStrip {
    width: f32,
    height: f32,
    value: f32,
}

impl FaderStrip {
    /// Create a new fader strip with default dimensions (includes housing)
    pub fn new(value: f32) -> Self {
        Self {
            width: STRIP_DEFAULT_WIDTH,
            height: STRIP_DEFAULT_HEIGHT,
            value: value.clamp(0.0, 1.0),
        }
    }

    /// Set custom size
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Show the fader strip and return the new value
    pub fn show(mut self, ui: &mut Ui) -> (Response, f32) {
        let theme = ui.ctx().armas_theme();
        let desired_size = Vec2::new(self.width, self.height);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Calculate scale factor
            let scale_x = self.width / STRIP_DEFAULT_WIDTH;
            let scale_y = self.height / STRIP_DEFAULT_HEIGHT;
            let scale = scale_x.min(scale_y);

            // Housing gradient colors from theme
            let housing_top = theme.surface_variant();
            let housing_bottom = theme.surface();

            // Draw outer housing with gradient
            // First draw a base rounded rect for the shape
            painter.rect_filled(rect, HOUSING_CORNER_RADIUS * scale, housing_top);

            // Then draw gradient layers on top (no corner radius to avoid waves)
            for i in 0..HOUSING_GRADIENT_STEPS {
                let t = i as f32 / (HOUSING_GRADIENT_STEPS - 1) as f32;
                // Interpolate from top to bottom color
                let color = Color32::from_rgb(
                    (housing_top.r() as f32 * (1.0 - t) + housing_bottom.r() as f32 * t) as u8,
                    (housing_top.g() as f32 * (1.0 - t) + housing_bottom.g() as f32 * t) as u8,
                    (housing_top.b() as f32 * (1.0 - t) + housing_bottom.b() as f32 * t) as u8,
                );

                let segment_height = self.height / HOUSING_GRADIENT_STEPS as f32;
                let y = rect.min.y + i as f32 * segment_height;

                painter.rect_filled(
                    Rect::from_min_size(
                        Pos2::new(rect.min.x, y),
                        Vec2::new(self.width, segment_height + 1.0),
                    ),
                    0.0, // No corner radius for gradient layers
                    color,
                );
            }

            // Re-draw the outer border with corner radius to clip the gradient
            painter.rect_stroke(
                rect,
                HOUSING_CORNER_RADIUS * scale,
                (0.5 * scale, theme.outline()),
                egui::StrokeKind::Middle,
            );

            // Calculate fader dimensions (inside housing with margins)
            let housing_margin_top = HOUSING_MARGIN_TOP * scale_y;
            let housing_margin_bottom = HOUSING_MARGIN_BOTTOM * scale_y;
            let fader_width = FADER_DEFAULT_WIDTH * scale_x;
            let fader_height = self.height - housing_margin_top - housing_margin_bottom;

            // Center fader horizontally in housing
            let fader_x = rect.min.x + (self.width - fader_width) / 2.0;
            let fader_y = rect.min.y + housing_margin_top;

            // Create a child UI region for the fader
            let fader_rect = Rect::from_min_size(
                Pos2::new(fader_x, fader_y),
                Vec2::new(fader_width, fader_height),
            );

            let mut fader_ui = ui.new_child(egui::UiBuilder::new().max_rect(fader_rect));

            // Show the inner fader
            let (fader_response, new_value) = Fader::new(self.value)
                .size(fader_width, fader_height)
                .show(&mut fader_ui);

            self.value = new_value;

            // Return the fader's response (which handles interaction)
            return (fader_response, self.value);
        }

        (response, self.value)
    }
}
