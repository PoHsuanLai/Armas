//! Snap Grid Component
//!
//! Visual grid lines for timeline alignment.
//! Draws vertical lines at beat/subdivision intervals for visual reference.

use armas_basic::ext::ArmasContextExt;
use egui::{Color32, Pos2, Sense, Ui};

/// Snap grid component
///
/// Displays visual grid lines at regular beat intervals.
/// Useful for visual alignment and spacing reference in DAW timelines.
///
/// # Example
///
/// ```rust,ignore
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas_audio::SnapGrid;
///
/// SnapGrid::new()
///     .beat_width(60.0)
///     .measures(16)
///     .subdivision(4)
///     .show_overlay(ui);
/// # }
/// ```
pub struct SnapGrid {
    beat_width: f32,
    measures: u32,
    beats_per_measure: u32,
    subdivision: u32,
}

impl SnapGrid {
    /// Create a new snap grid
    #[must_use]
    pub const fn new() -> Self {
        Self {
            beat_width: 60.0,
            measures: 16,
            beats_per_measure: 4,
            subdivision: 4,
        }
    }

    /// Set pixels per beat
    #[must_use]
    pub const fn beat_width(mut self, width: f32) -> Self {
        self.beat_width = width.max(1.0);
        self
    }

    /// Set number of measures
    #[must_use]
    pub const fn measures(mut self, measures: u32) -> Self {
        self.measures = measures;
        self
    }

    /// Set beats per measure
    #[must_use]
    pub const fn beats_per_measure(mut self, beats: u32) -> Self {
        self.beats_per_measure = beats;
        self
    }

    /// Set subdivision (lines per beat)
    /// E.g., 4 = 16th notes, 2 = 8th notes
    #[must_use]
    pub fn subdivision(mut self, subdivision: u32) -> Self {
        self.subdivision = subdivision.max(1);
        self
    }

    /// Show the snap grid as an overlay (no space allocation)
    ///
    /// Use this when rendering the grid in a separate layer over other content.
    /// The parent should set up `max_rect` with scroll offset and `clip_rect` for visible area.
    /// Grid lines are calculated from `max_rect.min.x` and clipped to `clip_rect`.
    pub fn show_overlay(self, ui: &mut Ui) {
        let theme = ui.ctx().armas_theme();

        // max_rect has the scroll offset baked in (starts at content origin - scroll)
        let content_rect = ui.max_rect();
        // clip_rect is the visible viewport
        let clip = ui.clip_rect();

        // Create a response for the visible area only
        let _response = ui.allocate_rect(clip, Sense::hover());

        if ui.is_rect_visible(clip) {
            let painter = ui.painter();

            let base_color = theme.border();
            let measure_opacity: f32 = 0.5;
            let beat_opacity: f32 = 0.3;
            let subdivision_opacity: f32 = 0.15;

            let total_beats = self.measures * self.beats_per_measure;
            let total_subdivisions = total_beats * self.subdivision;
            let subdivision_width = self.beat_width / self.subdivision as f32;

            // Draw all grid lines - calculate from content_rect, but only draw if in clip
            for i in 0..=total_subdivisions {
                // X position relative to content origin (with scroll offset)
                let x = (i as f32).mul_add(subdivision_width, content_rect.min.x);

                // Skip if outside visible area (clip rect)
                if x < clip.min.x - 1.0 || x > clip.max.x + 1.0 {
                    continue;
                }

                let is_measure = i % (self.beats_per_measure * self.subdivision) == 0;
                let is_beat = i % self.subdivision == 0;

                // Draw within clip bounds vertically
                let y_min = clip.min.y;
                let y_max = clip.max.y;

                if is_measure {
                    let color = Color32::from_rgba_unmultiplied(
                        base_color.r(),
                        base_color.g(),
                        base_color.b(),
                        (255.0 * measure_opacity) as u8,
                    );
                    painter.line_segment(
                        [Pos2::new(x, y_min), Pos2::new(x, y_max)],
                        egui::Stroke::new(1.5, color),
                    );
                } else if is_beat {
                    let color = Color32::from_rgba_unmultiplied(
                        base_color.r(),
                        base_color.g(),
                        base_color.b(),
                        (255.0 * beat_opacity) as u8,
                    );
                    painter.line_segment(
                        [Pos2::new(x, y_min), Pos2::new(x, y_max)],
                        egui::Stroke::new(1.0, color),
                    );
                } else {
                    let color = Color32::from_rgba_unmultiplied(
                        base_color.r(),
                        base_color.g(),
                        base_color.b(),
                        (255.0 * subdivision_opacity) as u8,
                    );
                    painter.line_segment(
                        [Pos2::new(x, y_min), Pos2::new(x, y_max)],
                        egui::Stroke::new(0.5, color),
                    );
                }
            }
        }
    }
}

impl Default for SnapGrid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snap_grid_creation() {
        let grid = SnapGrid::new();
        assert_eq!(grid.beat_width, 60.0);
        assert_eq!(grid.subdivision, 4);
    }

    #[test]
    fn test_snap_grid_builder() {
        let grid = SnapGrid::new()
            .beat_width(80.0)
            .measures(32)
            .subdivision(8);

        assert_eq!(grid.beat_width, 80.0);
        assert_eq!(grid.measures, 32);
        assert_eq!(grid.subdivision, 8);
    }
}
