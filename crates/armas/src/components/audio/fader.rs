//! Vertical fader component inspired by DAW mixer faders
//!
//! This module provides two components:
//! - `Fader`: Minimal interactive slider (track + channel + thumb)
//! - `FaderStrip`: Complete fader with housing and optional LED (batteries included)

use crate::ext::ArmasContextExt;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

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

// Track (inner black plate) dimensions
const TRACK_CORNER_RADIUS: f32 = 3.0;
const TRACK_INTERNAL_PADDING: f32 = 15.0;

// Channel (slider crack) dimensions
const CHANNEL_WIDTH: f32 = 8.0;
const CHANNEL_CORNER_RADIUS: f32 = 4.0;

// Thumb dimensions
const THUMB_WIDTH: f32 = 25.0;
const THUMB_HEIGHT: f32 = 54.0;

/// Minimal vertical fader component (track + channel + thumb only)
///
/// This is the core interactive slider without any container/housing.
/// Use this when you want to place the fader in your own container or card.
pub struct Fader {
    width: f32,
    height: f32,
    value: f32,
}

impl Fader {
    /// Create a new minimal fader with default dimensions (track only)
    pub fn new(value: f32) -> Self {
        Self {
            width: FADER_DEFAULT_WIDTH,
            height: FADER_DEFAULT_HEIGHT,
            value: value.clamp(0.0, 1.0),
        }
    }

    /// Set custom size
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Show the fader and return the new value
    pub fn show(mut self, ui: &mut Ui) -> (Response, f32) {
        let desired_size = Vec2::new(self.width, self.height);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        // Handle interaction
        if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                // Calculate scale for interaction bounds
                let scale_y = self.height / FADER_DEFAULT_HEIGHT;

                // Channel bounds (where the thumb actually moves)
                let track_internal_padding = TRACK_INTERNAL_PADDING * scale_y;

                let channel_top = rect.min.y + track_internal_padding;
                let channel_bottom = rect.max.y - track_internal_padding;
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

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Calculate scale factor based on default size
            let scale_x = self.width / FADER_DEFAULT_WIDTH;
            let scale_y = self.height / FADER_DEFAULT_HEIGHT;
            let scale = scale_x.min(scale_y); // Use uniform scale to maintain proportions

            // Track plate dimensions (the black inner plate) - this is the entire fader now
            let track_rect = rect;

            // Track background (dark inset with shadow for depth)
            painter.rect_filled(
                track_rect,
                TRACK_CORNER_RADIUS * scale,
                Color32::from_rgb(8, 8, 8),
            );

            // Add inset shadow for 3D recessed effect
            // box-shadow: inset 1px 1px 5px 3px rgba(0, 0, 0, 0.5)
            let shadow_size = 5.0 * scale;

            // Top-left shadow (darker at edges, fading inward)
            for i in 0..3 {
                let alpha = ((3 - i) as f32 / 3.0 * 127.0) as u8; // 0.5 max alpha = 127
                let offset = (i as f32 + 1.0) * scale;

                painter.rect_stroke(
                    track_rect.shrink(offset),
                    TRACK_CORNER_RADIUS * scale,
                    (
                        (shadow_size - offset).max(0.5),
                        Color32::from_rgba_unmultiplied(0, 0, 0, alpha),
                    ),
                    egui::StrokeKind::Middle,
                );
            }

            // Draw tick marks with varying widths
            self.draw_tick_marks(painter, track_rect, scale);

            // Draw the slider channel/crack (vertical dark gradient track)
            // Channel has internal padding within the track plate
            let track_internal_padding = TRACK_INTERNAL_PADDING * scale_y;
            let channel_width = CHANNEL_WIDTH * scale_x;
            let channel_x = track_rect.min.x + (self.width - channel_width) / 2.0;
            let channel_y = track_rect.min.y + track_internal_padding;
            let channel_height = self.height - (track_internal_padding * 2.0);

            let channel_rect = Rect::from_min_size(
                Pos2::new(channel_x, channel_y),
                Vec2::new(channel_width, channel_height),
            );

            // Draw solid black channel to hide tick marks underneath
            painter.rect_filled(
                channel_rect,
                CHANNEL_CORNER_RADIUS * scale,
                Color32::from_rgb(0, 0, 0), // Solid black
            );

            // Add border to channel
            painter.rect_stroke(
                channel_rect,
                CHANNEL_CORNER_RADIUS * scale,
                (1.0 * scale, Color32::from_rgb(14, 14, 14)),
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
            );
        }

        (response, self.value)
    }

    fn draw_tick_marks(&self, painter: &egui::Painter, track_rect: Rect, scale: f32) {
        // Tick mark pattern from the SVG: varying widths (scaled)
        // Draw tick marks from center extending outward to left and right
        let tick_widths = [0.0, 8.0, 9.0, 8.0, 10.0, 8.0, 10.0, 8.0, 10.0, 8.0, 0.0];
        let track_center_x = track_rect.center().x;
        let track_start_y = track_rect.min.y + 15.0 * scale;
        let track_usable_height = track_rect.height() - 30.0 * scale;

        for (i, &width) in tick_widths.iter().enumerate() {
            // Skip first and last tick marks to avoid rounded corners
            if i == 0 || i == tick_widths.len() - 1 {
                continue;
            }

            let t = i as f32 / (tick_widths.len() - 1) as f32;
            let y = track_start_y + t * track_usable_height;
            let tick_length = width * scale;

            // Left tick mark (from center extending leftward)
            painter.line_segment(
                [
                    Pos2::new(track_center_x - tick_length, y),
                    Pos2::new(track_center_x, y),
                ],
                (1.0 * scale, Color32::from_rgb(51, 51, 51)), // #333 from SVG
            );

            // Right tick mark (from center extending rightward)
            painter.line_segment(
                [
                    Pos2::new(track_center_x, y),
                    Pos2::new(track_center_x + tick_length, y),
                ],
                (1.0 * scale, Color32::from_rgb(51, 51, 51)), // #333 from SVG
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
    ) {
        let rect = Rect::from_min_size(pos, Vec2::new(width, height));

        // Calculate relative dimensions based on scaled width/height
        let w_scale = width / THUMB_WIDTH;
        let h_scale = height / THUMB_HEIGHT;

        // Top cap (gradient from #3D3D3D to #4E4E4E)
        let top_cap = Rect::from_min_size(
            pos + Vec2::new(1.0 * w_scale, 0.0),
            Vec2::new(23.0 * w_scale, 4.0 * h_scale),
        );
        painter.rect_filled(top_cap, 0.0, Color32::from_rgb(69, 69, 69));

        // Top cap highlight
        painter.line_segment(
            [
                pos + Vec2::new(1.0 * w_scale, 0.0),
                pos + Vec2::new(24.0 * w_scale, 0.0),
            ],
            (1.0 * scale, Color32::from_rgb(106, 106, 106)),
        );

        // Top slanted face
        let top_slant = Rect::from_min_size(
            pos + Vec2::new(2.0 * w_scale, 5.0 * h_scale),
            Vec2::new(21.0 * w_scale, 3.0 * h_scale),
        );
        painter.rect_filled(top_slant, 0.0, Color32::from_rgb(28, 28, 28));

        // Main body (gradient from #2E2E2E to #646464)
        // Approximate gradient with multiple horizontal lines (scaled)
        let gradient_lines = (39.0 * h_scale) as usize;
        for i in 0..gradient_lines {
            let t = i as f32 / (gradient_lines - 1) as f32;
            let color = Color32::from_rgb(
                (46.0 + t * (100.0 - 46.0)) as u8,
                (46.0 + t * (100.0 - 46.0)) as u8,
                (46.0 + t * (100.0 - 46.0)) as u8,
            );
            painter.line_segment(
                [
                    pos + Vec2::new(2.0 * w_scale, 8.0 * h_scale + i as f32 * h_scale),
                    pos + Vec2::new(23.0 * w_scale, 8.0 * h_scale + i as f32 * h_scale),
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
                    Pos2::new(pos.x + 23.0 * w_scale, y),
                ],
                (1.0 * scale, Color32::from_rgba_unmultiplied(0, 0, 0, 100)),
            );
        }

        // Finger groove (indent in the middle) (scaled)
        let groove = Rect::from_min_size(
            pos + Vec2::new(1.0 * w_scale, 24.0 * h_scale),
            Vec2::new(23.0 * w_scale, 5.0 * h_scale),
        );
        painter.rect_filled(groove, 0.0, Color32::from_rgb(145, 145, 145));
        painter.rect_stroke(
            groove,
            0.0,
            (1.0 * scale, Color32::from_rgb(27, 27, 27)),
            egui::StrokeKind::Middle,
        );

        // Bottom cap (gradient from #1E1E1E to #0B0B0B) (scaled)
        let bottom_cap = Rect::from_min_size(
            pos + Vec2::new(1.0 * w_scale, height - 6.0 * h_scale),
            Vec2::new(23.0 * w_scale, 6.0 * h_scale),
        );
        painter.rect_filled(bottom_cap, 0.0, Color32::from_rgb(20, 20, 20));

        // Side borders (scaled)
        painter.line_segment(
            [
                pos + Vec2::new(1.0 * w_scale, 5.0 * h_scale),
                pos + Vec2::new(1.0 * w_scale, height - 6.0 * h_scale),
            ],
            (1.0 * scale, Color32::from_rgb(17, 17, 17)),
        );
        painter.line_segment(
            [
                pos + Vec2::new(24.0 * w_scale, 5.0 * h_scale),
                pos + Vec2::new(24.0 * w_scale, height - 6.0 * h_scale),
            ],
            (1.0 * scale, Color32::from_rgb(21, 21, 21)),
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
        let _theme = ui.ctx().armas_theme();
        let desired_size = Vec2::new(self.width, self.height);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Calculate scale factor
            let scale_x = self.width / STRIP_DEFAULT_WIDTH;
            let scale_y = self.height / STRIP_DEFAULT_HEIGHT;
            let scale = scale_x.min(scale_y);

            // Draw outer housing with gradient (from #252525 to #121212)
            // First draw a base rounded rect for the shape
            painter.rect_filled(
                rect,
                HOUSING_CORNER_RADIUS * scale,
                Color32::from_rgb(37, 37, 37), // Base color
            );

            // Then draw gradient layers on top (no corner radius to avoid waves)
            for i in 0..HOUSING_GRADIENT_STEPS {
                let t = i as f32 / (HOUSING_GRADIENT_STEPS - 1) as f32;
                // Interpolate from #252525 to #121212
                let color_value = (37.0 - t * (37.0 - 18.0)) as u8;
                let color = Color32::from_rgb(color_value, color_value, color_value);

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
                (0.5 * scale, Color32::from_rgb(18, 18, 18)), // Subtle border
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
