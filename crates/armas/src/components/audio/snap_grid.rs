//! Snap Grid Component
//!
//! Visual grid lines for timeline alignment.
//! Draws vertical lines at beat/subdivision intervals for visual reference.

use crate::ext::ArmasContextExt;
use egui::{Color32, Pos2, Response, Sense, Ui, Vec2};

/// Response from the snap grid
#[derive(Debug, Clone)]
pub struct SnapGridResponse {
    /// The UI response
    pub response: Response,
}

/// Snap grid component
///
/// Displays visual grid lines at regular beat intervals.
/// Useful for visual alignment and spacing reference in DAW timelines.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::components::audio::SnapGrid;
///
/// SnapGrid::new()
///     .beat_width(60.0)
///     .measures(16)
///     .subdivision(4)  // Show 4 lines per beat (16th notes)
///     .show(ui);
/// # }
/// ```
pub struct SnapGrid {
    beat_width: f32,
    measures: u32,
    beats_per_measure: u32,
    subdivision: u32,
    show_beats: bool,
    show_measures: bool,
    show_subdivisions: bool,
    beat_color: Option<Color32>,
    measure_color: Option<Color32>,
    subdivision_color: Option<Color32>,
    beat_opacity: f32,
    measure_opacity: f32,
    subdivision_opacity: f32,
}

impl SnapGrid {
    /// Create a new snap grid
    pub fn new() -> Self {
        Self {
            beat_width: 60.0,
            measures: 16,
            beats_per_measure: 4,
            subdivision: 4,
            show_beats: true,
            show_measures: true,
            show_subdivisions: true,
            beat_color: None,
            measure_color: None,
            subdivision_color: None,
            beat_opacity: 0.3,
            measure_opacity: 0.5,
            subdivision_opacity: 0.15,
        }
    }

    /// Set pixels per beat
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

    /// Set subdivision (lines per beat)
    /// E.g., 4 = 16th notes, 2 = 8th notes
    pub fn subdivision(mut self, subdivision: u32) -> Self {
        self.subdivision = subdivision.max(1);
        self
    }

    /// Show or hide beat lines
    pub fn show_beats(mut self, show: bool) -> Self {
        self.show_beats = show;
        self
    }

    /// Show or hide measure lines (downbeats)
    pub fn show_measures(mut self, show: bool) -> Self {
        self.show_measures = show;
        self
    }

    /// Show or hide subdivision lines
    pub fn show_subdivisions(mut self, show: bool) -> Self {
        self.show_subdivisions = show;
        self
    }

    /// Set custom color for beat lines
    pub fn beat_color(mut self, color: Color32) -> Self {
        self.beat_color = Some(color);
        self
    }

    /// Set custom color for measure lines
    pub fn measure_color(mut self, color: Color32) -> Self {
        self.measure_color = Some(color);
        self
    }

    /// Set custom color for subdivision lines
    pub fn subdivision_color(mut self, color: Color32) -> Self {
        self.subdivision_color = Some(color);
        self
    }

    /// Set opacity for beat lines (0.0 to 1.0)
    pub fn beat_opacity(mut self, opacity: f32) -> Self {
        self.beat_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set opacity for measure lines (0.0 to 1.0)
    pub fn measure_opacity(mut self, opacity: f32) -> Self {
        self.measure_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set opacity for subdivision lines (0.0 to 1.0)
    pub fn subdivision_opacity(mut self, opacity: f32) -> Self {
        self.subdivision_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Show the snap grid
    ///
    /// When used as an overlay (in a separate layer), use `show_overlay()` instead
    /// to avoid allocating space that breaks the parent layout.
    pub fn show(self, ui: &mut Ui) -> SnapGridResponse {
        let theme = ui.ctx().armas_theme();

        let total_beats = self.measures * self.beats_per_measure;
        let timeline_width = total_beats as f32 * self.beat_width;

        let height = ui.available_height().max(100.0);
        let desired_size = Vec2::new(timeline_width, height);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Default colors
            let base_color = theme.outline();
            let measure_col = self.measure_color.unwrap_or(base_color);
            let beat_col = self.beat_color.unwrap_or(base_color);
            let subdiv_col = self.subdivision_color.unwrap_or(base_color);

            let total_subdivisions = total_beats * self.subdivision;
            let subdivision_width = self.beat_width / self.subdivision as f32;

            // Draw all grid lines
            for i in 0..=total_subdivisions {
                let x = rect.min.x + i as f32 * subdivision_width;

                let is_measure = i % (self.beats_per_measure * self.subdivision) == 0;
                let is_beat = i % self.subdivision == 0;

                if is_measure && self.show_measures {
                    // Measure line (strongest)
                    let color = Color32::from_rgba_unmultiplied(
                        measure_col.r(),
                        measure_col.g(),
                        measure_col.b(),
                        (255.0 * self.measure_opacity) as u8,
                    );
                    painter.line_segment(
                        [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                        egui::Stroke::new(1.5, color),
                    );
                } else if is_beat && self.show_beats {
                    // Beat line (medium)
                    let color = Color32::from_rgba_unmultiplied(
                        beat_col.r(),
                        beat_col.g(),
                        beat_col.b(),
                        (255.0 * self.beat_opacity) as u8,
                    );
                    painter.line_segment(
                        [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                        egui::Stroke::new(1.0, color),
                    );
                } else if self.show_subdivisions {
                    // Subdivision line (weakest)
                    let color = Color32::from_rgba_unmultiplied(
                        subdiv_col.r(),
                        subdiv_col.g(),
                        subdiv_col.b(),
                        (255.0 * self.subdivision_opacity) as u8,
                    );
                    painter.line_segment(
                        [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                        egui::Stroke::new(0.5, color),
                    );
                }
            }
        }

        SnapGridResponse { response }
    }

    /// Show the snap grid as an overlay (no space allocation)
    ///
    /// Use this when rendering the grid in a separate layer over other content.
    /// The parent should set up max_rect with scroll offset and clip_rect for visible area.
    /// Grid lines are calculated from max_rect.min.x and clipped to clip_rect.
    pub fn show_overlay(self, ui: &mut Ui) -> SnapGridResponse {
        let theme = ui.ctx().armas_theme();

        // max_rect has the scroll offset baked in (starts at content origin - scroll)
        let content_rect = ui.max_rect();
        // clip_rect is the visible viewport
        let clip = ui.clip_rect();

        // Create a response for the visible area only
        let response = ui.allocate_rect(clip, Sense::hover());

        if ui.is_rect_visible(clip) {
            let painter = ui.painter();

            // Default colors
            let base_color = theme.outline();
            let measure_col = self.measure_color.unwrap_or(base_color);
            let beat_col = self.beat_color.unwrap_or(base_color);
            let subdiv_col = self.subdivision_color.unwrap_or(base_color);

            let total_beats = self.measures * self.beats_per_measure;
            let total_subdivisions = total_beats * self.subdivision;
            let subdivision_width = self.beat_width / self.subdivision as f32;

            // Draw all grid lines - calculate from content_rect, but only draw if in clip
            for i in 0..=total_subdivisions {
                // X position relative to content origin (with scroll offset)
                let x = content_rect.min.x + i as f32 * subdivision_width;

                // Skip if outside visible area (clip rect)
                if x < clip.min.x - 1.0 || x > clip.max.x + 1.0 {
                    continue;
                }

                let is_measure = i % (self.beats_per_measure * self.subdivision) == 0;
                let is_beat = i % self.subdivision == 0;

                // Draw within clip bounds vertically
                let y_min = clip.min.y;
                let y_max = clip.max.y;

                if is_measure && self.show_measures {
                    // Measure line (strongest)
                    let color = Color32::from_rgba_unmultiplied(
                        measure_col.r(),
                        measure_col.g(),
                        measure_col.b(),
                        (255.0 * self.measure_opacity) as u8,
                    );
                    painter.line_segment(
                        [Pos2::new(x, y_min), Pos2::new(x, y_max)],
                        egui::Stroke::new(1.5, color),
                    );
                } else if is_beat && self.show_beats {
                    // Beat line (medium)
                    let color = Color32::from_rgba_unmultiplied(
                        beat_col.r(),
                        beat_col.g(),
                        beat_col.b(),
                        (255.0 * self.beat_opacity) as u8,
                    );
                    painter.line_segment(
                        [Pos2::new(x, y_min), Pos2::new(x, y_max)],
                        egui::Stroke::new(1.0, color),
                    );
                } else if self.show_subdivisions {
                    // Subdivision line (weakest)
                    let color = Color32::from_rgba_unmultiplied(
                        subdiv_col.r(),
                        subdiv_col.g(),
                        subdiv_col.b(),
                        (255.0 * self.subdivision_opacity) as u8,
                    );
                    painter.line_segment(
                        [Pos2::new(x, y_min), Pos2::new(x, y_max)],
                        egui::Stroke::new(0.5, color),
                    );
                }
            }
        }

        SnapGridResponse { response }
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
            .subdivision(8)
            .show_subdivisions(false);

        assert_eq!(grid.beat_width, 80.0);
        assert_eq!(grid.measures, 32);
        assert_eq!(grid.subdivision, 8);
        assert!(!grid.show_subdivisions);
    }
}
