//! Punch Marker Component
//!
//! Visual markers for recording punch-in/punch-out points with draggable handles.
//! Shows highlighted punch region background in the timeline for defining recording regions.

use crate::ext::ArmasContextExt;
use crate::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Punch marker component
///
/// Displays punch-in and punch-out markers with draggable handles and highlighted region.
/// Useful for defining recording regions in DAW timelines (auto punch-in/out recording).
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::components::audio::PunchMarker;
///
/// let mut punch_in = 8.0;   // beats
/// let mut punch_out = 16.0; // beats
///
/// let response = PunchMarker::new(&mut punch_in, &mut punch_out)
///     .beat_width(60.0)  // pixels per beat (must match Timeline)
///     .measures(16)
///     .enabled(true)
///     .snap_to_grid(true)
///     .grid_division(1.0)  // snap to whole beats
///     .show(ui);
///
/// if response.punch_in_changed {
///     println!("Punch in: {}", punch_in);
/// }
/// if response.punch_out_changed {
///     println!("Punch out: {}", punch_out);
/// }
/// # }
/// ```
pub struct PunchMarker<'a> {
    punch_in: &'a mut f32,
    punch_out: &'a mut f32,
    beat_width: f32,
    measures: u32,
    beats_per_measure: u32,
    height: f32,
    enabled: bool,
    snap_to_grid: bool,
    grid_division: f32,
    color: Option<Color32>,
    handle_width: f32,
    show_labels: bool,
    id: Option<egui::Id>,
    clip_rect: Option<Rect>,
    vertical_range: (f32, f32), // (top_percent, bottom_percent) from 0.0 to 1.0
}

/// Response from punch marker interaction
#[derive(Debug, Clone)]
pub struct PunchMarkerResponse {
    /// The egui response
    pub response: Response,
    /// Punch in handle was dragged
    pub punch_in_changed: bool,
    /// Punch out handle was dragged
    pub punch_out_changed: bool,
    /// Punch region was clicked
    pub region_clicked: bool,
}

impl<'a> PunchMarker<'a> {
    /// Create a new punch marker
    pub fn new(punch_in: &'a mut f32, punch_out: &'a mut f32) -> Self {
        Self {
            punch_in,
            punch_out,
            beat_width: 60.0,
            measures: 16,
            beats_per_measure: 4,
            height: 70.0,
            enabled: true,
            snap_to_grid: false,
            grid_division: 1.0,
            color: None,
            handle_width: 8.0,
            show_labels: true,
            id: None,
            clip_rect: None,
            vertical_range: (0.0, 1.0), // Full height by default
        }
    }

    /// Set unique ID for state persistence
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set pixels per beat (must match Timeline/TimelineTrack)
    pub fn beat_width(mut self, width: f32) -> Self {
        self.beat_width = width.max(1.0);
        self
    }

    /// Set number of measures
    pub fn measures(mut self, measures: u32) -> Self {
        self.measures = measures;
        self
    }

    /// Set beats per measure
    pub fn beats_per_measure(mut self, beats: u32) -> Self {
        self.beats_per_measure = beats;
        self
    }

    /// Set height of the marker component
    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(20.0);
        self
    }

    /// Enable or disable the punch region
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Enable snap to grid
    pub fn snap_to_grid(mut self, snap: bool) -> Self {
        self.snap_to_grid = snap;
        self
    }

    /// Set grid division for snapping (e.g., 1.0 = whole beats, 0.25 = 16th notes)
    pub fn grid_division(mut self, division: f32) -> Self {
        self.grid_division = division.max(0.0625); // Minimum 1/16 of a beat
        self
    }

    /// Set custom color for the punch region
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Set handle width
    pub fn handle_width(mut self, width: f32) -> Self {
        self.handle_width = width.max(4.0);
        self
    }

    /// Show or hide time labels on handles
    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// Set clip rect for rendering (to prevent overflow)
    pub fn clip_rect(mut self, clip_rect: Rect) -> Self {
        self.clip_rect = Some(clip_rect);
        self
    }

    /// Set vertical range as percentages (0.0 to 1.0)
    /// For example: (0.0, 0.5) = top half, (0.5, 1.0) = bottom half, (0.33, 0.66) = middle third
    pub fn vertical_range(mut self, top_percent: f32, bottom_percent: f32) -> Self {
        self.vertical_range = (top_percent.clamp(0.0, 1.0), bottom_percent.clamp(0.0, 1.0));
        self
    }

    /// Show the punch marker
    pub fn show(self, ui: &mut Ui) -> PunchMarkerResponse {
        let theme = ui.ctx().armas_theme();

        // Calculate timeline width from measures and beat_width (same as TimelineTrack)
        let total_beats = self.measures * self.beats_per_measure;
        let timeline_width = total_beats as f32 * self.beat_width;

        // Load previous state if ID is set
        if let Some(id) = self.id {
            let state_id_in = id.with("punch_in");
            let state_id_out = id.with("punch_out");

            // Load stored values
            let stored_in: f32 = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id_in).unwrap_or(*self.punch_in));
            let stored_out: f32 = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id_out).unwrap_or(*self.punch_out));

            // Update the references
            *self.punch_in = stored_in;
            *self.punch_out = stored_out;
        }

        // Ensure punch_in is before punch_out
        if *self.punch_in > *self.punch_out {
            std::mem::swap(self.punch_in, self.punch_out);
        }

        let mut punch_in_changed = false;
        let mut punch_out_changed = false;
        let mut region_clicked = false;

        if !self.enabled {
            // Return early if disabled, just allocate space
            let (_rect, response) = ui.allocate_exact_size(
                Vec2::new(timeline_width, 0.0),
                Sense::hover(),
            );
            return PunchMarkerResponse {
                response,
                punch_in_changed: false,
                punch_out_changed: false,
                region_clicked: false,
            };
        }

        // Don't allocate the full space - only interact with specific regions (handles and area)
        let full_rect = ui.available_rect_before_wrap();
        let full_height = self.height;

        // Apply vertical range to position within the available height
        let (top_percent, bottom_percent) = self.vertical_range;
        let y_offset = full_height * top_percent;
        let actual_height = full_height * (bottom_percent - top_percent);

        let rect = Rect::from_min_size(
            Pos2::new(full_rect.min.x, full_rect.min.y + y_offset),
            Vec2::new(timeline_width, actual_height)
        );

        if ui.is_rect_visible(rect) {
            // Punch uses recording red color by default
            let punch_color = self.color.unwrap_or_else(|| {
                Color32::from_rgb(220, 50, 50) // Recording red
            });

            // Get painter - apply clip rect only for painting, not for interactions
            let base_painter = ui.painter();
            let painter = if let Some(clip) = self.clip_rect {
                base_painter.with_clip_rect(clip)
            } else {
                base_painter.clone()
            };

            // Calculate pixel positions
            let in_x = rect.min.x + (*self.punch_in * self.beat_width).max(0.0).min(timeline_width);
            let out_x = rect.min.x + (*self.punch_out * self.beat_width).max(0.0).min(timeline_width);

            // Draw punch region background
            if out_x > in_x {
                let region_rect = Rect::from_min_max(
                    Pos2::new(in_x, rect.min.y),
                    Pos2::new(out_x, rect.max.y),
                );

                // Semi-transparent fill with recording red
                painter.rect_filled(
                    region_rect,
                    0.0,
                    Color32::from_rgba_unmultiplied(
                        punch_color.r(),
                        punch_color.g(),
                        punch_color.b(),
                        80, // Increased from 35 for better visibility
                    ),
                );

                // Dashed border to distinguish from loop/selection
                let stroke_color = punch_color;
                self.draw_dashed_border(&painter, region_rect, stroke_color);

                // Check if region was clicked
                let region_id = self.id.unwrap_or_else(|| ui.id()).with("punch_region_area");
                let region_response = ui.interact(region_rect, region_id, Sense::click());
                if region_response.clicked() {
                    region_clicked = true;
                }
            }

            // Draw punch in handle
            let in_handle_rect = Rect::from_min_size(
                Pos2::new(in_x - self.handle_width / 2.0, rect.min.y),
                Vec2::new(self.handle_width, actual_height),
            );
            let punch_in_id = self.id.unwrap_or_else(|| ui.id()).with("punch_in_handle");
            let in_handle_response = ui.interact(
                in_handle_rect,
                punch_in_id,
                Sense::click_and_drag(),
            );

            if in_handle_response.dragged() {
                if let Some(pos) = in_handle_response.interact_pointer_pos() {
                    let new_beat = ((pos.x - rect.min.x) / self.beat_width).max(0.0);
                    *self.punch_in = if self.snap_to_grid {
                        (new_beat / self.grid_division).round() * self.grid_division
                    } else {
                        new_beat
                    };
                    punch_in_changed = true;
                }
            }

            self.draw_handle(
                &painter,
                &theme,
                in_handle_rect,
                punch_color,
                in_handle_response.hovered() || in_handle_response.dragged(),
                true, // is_in
            );

            // Draw punch out handle
            let out_handle_rect = Rect::from_min_size(
                Pos2::new(out_x - self.handle_width / 2.0, rect.min.y),
                Vec2::new(self.handle_width, actual_height),
            );
            let punch_out_id = self.id.unwrap_or_else(|| ui.id()).with("punch_out_handle");
            let out_handle_response = ui.interact(
                out_handle_rect,
                punch_out_id,
                Sense::click_and_drag(),
            );

            if out_handle_response.dragged() {
                if let Some(pos) = out_handle_response.interact_pointer_pos() {
                    let new_beat = ((pos.x - rect.min.x) / self.beat_width).max(0.0);
                    *self.punch_out = if self.snap_to_grid {
                        (new_beat / self.grid_division).round() * self.grid_division
                    } else {
                        new_beat
                    };
                    punch_out_changed = true;
                }
            }

            self.draw_handle(
                &painter,
                &theme,
                out_handle_rect,
                punch_color,
                out_handle_response.hovered() || out_handle_response.dragged(),
                false, // is_out
            );

            // Draw labels
            if self.show_labels {
                // Punch in label
                let in_text = format!("{:.1}", self.punch_in);
                painter.text(
                    Pos2::new(in_x, rect.min.y - theme.spacing.xs),
                    egui::Align2::CENTER_BOTTOM,
                    in_text,
                    egui::FontId::proportional(10.0),
                    theme.on_surface(),
                );

                // Punch out label
                let out_text = format!("{:.1}", self.punch_out);
                painter.text(
                    Pos2::new(out_x, rect.min.y - theme.spacing.xs),
                    egui::Align2::CENTER_BOTTOM,
                    out_text,
                    egui::FontId::proportional(10.0),
                    theme.on_surface(),
                );
            }
        }

        // Save state to memory if ID is set
        if let Some(id) = self.id {
            let state_id_in = id.with("punch_in");
            let state_id_out = id.with("punch_out");
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id_in, *self.punch_in);
                d.insert_temp(state_id_out, *self.punch_out);
            });
        }

        // Create a dummy response (we handle all interactions manually)
        let response = ui.allocate_response(Vec2::ZERO, Sense::hover());

        PunchMarkerResponse {
            response,
            punch_in_changed,
            punch_out_changed,
            region_clicked,
        }
    }

    fn draw_dashed_border(&self, painter: &egui::Painter, rect: Rect, color: Color32) {
        let dash_len = 4.0;
        let gap_len = 3.0;
        let stroke_width = 1.5;

        // Top edge
        let mut x = rect.min.x;
        while x < rect.max.x {
            let end_x = (x + dash_len).min(rect.max.x);
            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(end_x, rect.min.y)],
                egui::Stroke::new(stroke_width, color),
            );
            x += dash_len + gap_len;
        }

        // Bottom edge
        x = rect.min.x;
        while x < rect.max.x {
            let end_x = (x + dash_len).min(rect.max.x);
            painter.line_segment(
                [Pos2::new(x, rect.max.y), Pos2::new(end_x, rect.max.y)],
                egui::Stroke::new(stroke_width, color),
            );
            x += dash_len + gap_len;
        }
    }

    fn draw_handle(
        &self,
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        color: Color32,
        is_hovered: bool,
        is_in: bool,
    ) {
        // Handle background - darker for punch markers
        let bg_color = if is_hovered {
            color.gamma_multiply(1.3)
        } else {
            color.gamma_multiply(0.9)
        };

        painter.rect_filled(rect, theme.spacing.corner_radius_small as f32, bg_color);

        // Handle border - thicker for punch markers
        painter.rect_stroke(
            rect,
            theme.spacing.corner_radius_small as f32,
            egui::Stroke::new(2.0, Color32::from_rgb(180, 40, 40)),
            egui::StrokeKind::Outside,
        );

        // Draw record button icon (filled circle)
        let center = rect.center();
        let icon_radius = 5.0;
        let icon_color = Color32::WHITE;

        // Filled circle for record button
        painter.circle_filled(center, icon_radius, icon_color);

        // Optional: Add inner circle for more detail
        painter.circle_stroke(
            center,
            icon_radius,
            egui::Stroke::new(1.0, Color32::from_rgb(200, 200, 200)),
        );

        // Small "IN" or "OUT" label below the icon
        let label_y = center.y + 12.0;
        let text = if is_in { "IN" } else { "OUT" };
        let text_color = Color32::WHITE;

        painter.text(
            Pos2::new(center.x, label_y),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(7.0),
            text_color,
        );

        // Glow effect when hovered - brighter for punch markers
        if is_hovered {
            for i in 0..4 {
                let offset = (i + 1) as f32 * 2.0;
                let alpha = ((1.0 - i as f32 / 4.0) * 60.0) as u8;
                let glow_color = Color32::from_rgba_unmultiplied(
                    color.r(),
                    color.g(),
                    color.b(),
                    alpha,
                );
                painter.rect_stroke(
                    rect.expand(offset),
                    theme.spacing.corner_radius_small as f32,
                    egui::Stroke::new(2.0, glow_color),
                    egui::StrokeKind::Outside,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_punch_marker_creation() {
        let mut punch_in = 8.0;
        let mut punch_out = 16.0;
        let _marker = PunchMarker::new(&mut punch_in, &mut punch_out);
        assert_eq!(punch_in, 8.0);
        assert_eq!(punch_out, 16.0);
    }

    #[test]
    fn test_punch_marker_builder() {
        let mut punch_in = 4.0;
        let mut punch_out = 12.0;
        let marker = PunchMarker::new(&mut punch_in, &mut punch_out)
            .enabled(true)
            .snap_to_grid(true)
            .grid_division(0.25)
            .show_labels(false);

        assert!(marker.enabled);
        assert!(marker.snap_to_grid);
        assert_eq!(marker.grid_division, 0.25);
        assert!(!marker.show_labels);
    }

    #[test]
    fn test_swap_if_inverted() {
        let mut punch_in = 16.0;
        let mut punch_out = 4.0;
        let _marker = PunchMarker::new(&mut punch_in, &mut punch_out);

        // In real usage, the show() method would swap these
        // Here we just verify they're set up for swapping
        assert!(punch_in > punch_out);
    }
}
