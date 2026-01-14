//! Selection Range Component
//!
//! Visual markers for selection start/end points with draggable handles.
//! Shows highlighted selection region background in the timeline for editing operations.

use crate::ext::ArmasContextExt;
use crate::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Selection range marker component
///
/// Displays selection start and end markers with draggable handles and highlighted region.
/// Useful for defining editing ranges in DAW timelines (copy, cut, paste, delete).
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::components::audio::SelectionRange;
///
/// let mut selection_start = 4.0;  // beats
/// let mut selection_end = 16.0;   // beats
///
/// let response = SelectionRange::new(&mut selection_start, &mut selection_end)
///     .beat_width(60.0)  // pixels per beat (must match Timeline)
///     .measures(16)
///     .enabled(true)
///     .snap_to_grid(true)
///     .grid_division(1.0)  // snap to whole beats
///     .show(ui);
///
/// if response.selection_start_changed {
///     println!("Selection start: {}", selection_start);
/// }
/// if response.selection_end_changed {
///     println!("Selection end: {}", selection_end);
/// }
/// # }
/// ```
pub struct SelectionRange<'a> {
    selection_start: &'a mut f32,
    selection_end: &'a mut f32,
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

/// Response from selection range interaction
#[derive(Debug, Clone)]
pub struct SelectionRangeResponse {
    /// The egui response
    pub response: Response,
    /// Selection start handle was dragged
    pub selection_start_changed: bool,
    /// Selection end handle was dragged
    pub selection_end_changed: bool,
    /// Selection region was clicked
    pub region_clicked: bool,
}

impl<'a> SelectionRange<'a> {
    /// Create a new selection range marker
    pub fn new(selection_start: &'a mut f32, selection_end: &'a mut f32) -> Self {
        Self {
            selection_start,
            selection_end,
            beat_width: 60.0,
            measures: 16,
            beats_per_measure: 4,
            height: 60.0,
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

    /// Enable or disable the selection region
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

    /// Set custom color for the selection region
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

    /// Show the selection range marker
    pub fn show(self, ui: &mut Ui) -> SelectionRangeResponse {
        let theme = ui.ctx().armas_theme();

        // Calculate timeline width from measures and beat_width (same as TimelineTrack)
        let total_beats = self.measures * self.beats_per_measure;
        let timeline_width = total_beats as f32 * self.beat_width;

        // Load previous state if ID is set
        if let Some(id) = self.id {
            let state_id_start = id.with("selection_start");
            let state_id_end = id.with("selection_end");

            // Load stored values
            let stored_start: f32 = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id_start).unwrap_or(*self.selection_start));
            let stored_end: f32 = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id_end).unwrap_or(*self.selection_end));

            // Update the references
            *self.selection_start = stored_start;
            *self.selection_end = stored_end;
        }

        // Ensure start is before end
        if *self.selection_start > *self.selection_end {
            std::mem::swap(self.selection_start, self.selection_end);
        }

        let mut selection_start_changed = false;
        let mut selection_end_changed = false;
        let mut region_clicked = false;

        if !self.enabled {
            // Return early if disabled, just allocate space
            let (_rect, response) = ui.allocate_exact_size(
                Vec2::new(timeline_width, 0.0),
                Sense::hover(),
            );
            return SelectionRangeResponse {
                response,
                selection_start_changed: false,
                selection_end_changed: false,
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
            // Selection uses a different color scheme - lighter/more neutral
            let selection_color = self.color.unwrap_or_else(|| {
                Color32::from_rgb(150, 150, 150) // Neutral gray for selection
            });

            // Get painter - apply clip rect only for painting, not for interactions
            let base_painter = ui.painter();
            let painter = if let Some(clip) = self.clip_rect {
                base_painter.with_clip_rect(clip)
            } else {
                base_painter.clone()
            };

            // Calculate pixel positions
            let start_x = rect.min.x + (*self.selection_start * self.beat_width).max(0.0).min(timeline_width);
            let end_x = rect.min.x + (*self.selection_end * self.beat_width).max(0.0).min(timeline_width);

            // Draw selection region background
            if end_x > start_x {
                let region_rect = Rect::from_min_max(
                    Pos2::new(start_x, rect.min.y),
                    Pos2::new(end_x, rect.max.y),
                );

                // Semi-transparent fill (lighter than loop region)
                painter.rect_filled(
                    region_rect,
                    0.0,
                    Color32::from_rgba_unmultiplied(
                        selection_color.r(),
                        selection_color.g(),
                        selection_color.b(),
                        80, // Increased from 40 for better visibility
                    ),
                );

                // Border around selection
                painter.rect_stroke(
                    region_rect,
                    0.0,
                    egui::Stroke::new(2.0, selection_color),  // Increased from 1.0
                    egui::StrokeKind::Inside,
                );

                // Check if region was clicked
                let region_id = self.id.unwrap_or_else(|| ui.id()).with("selection_region_area");
                let region_response = ui.interact(region_rect, region_id, Sense::click());
                if region_response.clicked() {
                    region_clicked = true;
                }
            }

            // Draw start handle
            let start_handle_rect = Rect::from_min_size(
                Pos2::new(start_x - self.handle_width / 2.0, rect.min.y),
                Vec2::new(self.handle_width, actual_height),
            );
            let start_handle_id = self.id.unwrap_or_else(|| ui.id()).with("selection_start_handle");
            let start_handle_response = ui.interact(
                start_handle_rect,
                start_handle_id,
                Sense::click_and_drag(),
            );

            if start_handle_response.dragged() {
                if let Some(pos) = start_handle_response.interact_pointer_pos() {
                    let new_beat = ((pos.x - rect.min.x) / self.beat_width).max(0.0);
                    *self.selection_start = if self.snap_to_grid {
                        (new_beat / self.grid_division).round() * self.grid_division
                    } else {
                        new_beat
                    };
                    selection_start_changed = true;
                }
            }

            self.draw_handle(
                &painter,
                &theme,
                start_handle_rect,
                selection_color,
                start_handle_response.hovered() || start_handle_response.dragged(),
                true, // is_start
            );

            // Draw end handle
            let end_handle_rect = Rect::from_min_size(
                Pos2::new(end_x - self.handle_width / 2.0, rect.min.y),
                Vec2::new(self.handle_width, actual_height),
            );
            let end_handle_id = self.id.unwrap_or_else(|| ui.id()).with("selection_end_handle");
            let end_handle_response = ui.interact(
                end_handle_rect,
                end_handle_id,
                Sense::click_and_drag(),
            );

            if end_handle_response.dragged() {
                if let Some(pos) = end_handle_response.interact_pointer_pos() {
                    let new_beat = ((pos.x - rect.min.x) / self.beat_width).max(0.0);
                    *self.selection_end = if self.snap_to_grid {
                        (new_beat / self.grid_division).round() * self.grid_division
                    } else {
                        new_beat
                    };
                    selection_end_changed = true;
                }
            }

            self.draw_handle(
                &painter,
                &theme,
                end_handle_rect,
                selection_color,
                end_handle_response.hovered() || end_handle_response.dragged(),
                false, // is_end
            );

            // Draw labels
            if self.show_labels {
                // Start label
                let start_text = format!("{:.1}", self.selection_start);
                painter.text(
                    Pos2::new(start_x, rect.min.y - theme.spacing.xs),
                    egui::Align2::CENTER_BOTTOM,
                    start_text,
                    egui::FontId::proportional(10.0),
                    theme.on_surface(),
                );

                // End label
                let end_text = format!("{:.1}", self.selection_end);
                painter.text(
                    Pos2::new(end_x, rect.min.y - theme.spacing.xs),
                    egui::Align2::CENTER_BOTTOM,
                    end_text,
                    egui::FontId::proportional(10.0),
                    theme.on_surface(),
                );
            }
        }

        // Save state to memory if ID is set
        if let Some(id) = self.id {
            let state_id_start = id.with("selection_start");
            let state_id_end = id.with("selection_end");
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id_start, *self.selection_start);
                d.insert_temp(state_id_end, *self.selection_end);
            });
        }

        // Create a dummy response (we handle all interactions manually)
        let response = ui.allocate_response(Vec2::ZERO, Sense::hover());

        SelectionRangeResponse {
            response,
            selection_start_changed,
            selection_end_changed,
            region_clicked,
        }
    }

    fn draw_handle(
        &self,
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        color: Color32,
        is_hovered: bool,
        is_start: bool,
    ) {
        // Handle background
        let bg_color = if is_hovered {
            color.gamma_multiply(1.3)
        } else {
            color
        };

        painter.rect_filled(rect, theme.spacing.corner_radius_small as f32, bg_color);

        // Handle border
        painter.rect_stroke(
            rect,
            theme.spacing.corner_radius_small as f32,
            egui::Stroke::new(1.0, theme.on_surface().gamma_multiply(0.8)),
            egui::StrokeKind::Outside,
        );

        // Draw bracket indicator (different from loop arrows)
        let center = rect.center();
        let bracket_size = 6.0;
        let bracket_color = theme.on_surface();

        if is_start {
            // Left bracket for start handle: [
            painter.line_segment(
                [
                    Pos2::new(center.x + bracket_size / 2.0, center.y - bracket_size),
                    Pos2::new(center.x - bracket_size / 2.0, center.y - bracket_size),
                ],
                egui::Stroke::new(1.5, bracket_color),
            );
            painter.line_segment(
                [
                    Pos2::new(center.x - bracket_size / 2.0, center.y - bracket_size),
                    Pos2::new(center.x - bracket_size / 2.0, center.y + bracket_size),
                ],
                egui::Stroke::new(1.5, bracket_color),
            );
            painter.line_segment(
                [
                    Pos2::new(center.x - bracket_size / 2.0, center.y + bracket_size),
                    Pos2::new(center.x + bracket_size / 2.0, center.y + bracket_size),
                ],
                egui::Stroke::new(1.5, bracket_color),
            );
        } else {
            // Right bracket for end handle: ]
            painter.line_segment(
                [
                    Pos2::new(center.x - bracket_size / 2.0, center.y - bracket_size),
                    Pos2::new(center.x + bracket_size / 2.0, center.y - bracket_size),
                ],
                egui::Stroke::new(1.5, bracket_color),
            );
            painter.line_segment(
                [
                    Pos2::new(center.x + bracket_size / 2.0, center.y - bracket_size),
                    Pos2::new(center.x + bracket_size / 2.0, center.y + bracket_size),
                ],
                egui::Stroke::new(1.5, bracket_color),
            );
            painter.line_segment(
                [
                    Pos2::new(center.x + bracket_size / 2.0, center.y + bracket_size),
                    Pos2::new(center.x - bracket_size / 2.0, center.y + bracket_size),
                ],
                egui::Stroke::new(1.5, bracket_color),
            );
        }

        // Glow effect when hovered
        if is_hovered {
            for i in 0..3 {
                let offset = (i + 1) as f32 * 1.5;
                let alpha = ((1.0 - i as f32 / 3.0) * 40.0) as u8;
                let glow_color = Color32::from_rgba_unmultiplied(
                    color.r(),
                    color.g(),
                    color.b(),
                    alpha,
                );
                painter.rect_stroke(
                    rect.expand(offset),
                    theme.spacing.corner_radius_small as f32,
                    egui::Stroke::new(1.5, glow_color),
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
    fn test_selection_range_creation() {
        let mut start = 0.0;
        let mut end = 16.0;
        let _marker = SelectionRange::new(&mut start, &mut end);
        assert_eq!(start, 0.0);
        assert_eq!(end, 16.0);
    }

    #[test]
    fn test_selection_range_builder() {
        let mut start = 4.0;
        let mut end = 12.0;
        let marker = SelectionRange::new(&mut start, &mut end)
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
        let mut start = 16.0;
        let mut end = 4.0;
        let _marker = SelectionRange::new(&mut start, &mut end);

        // In real usage, the show() method would swap these
        // Here we just verify they're set up for swapping
        assert!(start > end);
    }
}
