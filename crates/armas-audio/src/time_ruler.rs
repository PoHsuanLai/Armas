//! Time Ruler Component
//!
//! Horizontal ruler showing measures, beats, and subdivisions for DAW timeline.

use armas_basic::theme::Theme;
use egui::{Pos2, Rect, Response, Sense, Stroke, Ui};

/// Re-export `GridDivision` from `piano_roll` for time subdivisions
pub use super::piano_roll::GridDivision;

/// Horizontal time ruler for DAW timeline
///
/// Shows measures, beats, and subdivisions with precise alignment.
///
/// # Example
///
/// ```rust,ignore
/// use armas_audio::TimeRuler;
///
/// fn ui(ui: &mut egui::Ui, theme: &armas_basic::Theme) {
///     TimeRuler::new()
///         .measures(8)
///         .beat_width(60.0)
///         .show_clipped(ui, theme);
/// }
/// ```
pub struct TimeRuler {
    /// Number of measures to display
    measures: u32,
    /// Width per beat in pixels (zoom level)
    beat_width: f32,
    /// Beats per measure (time signature numerator)
    beats_per_measure: u32,
    /// Grid division for subdivisions
    division: GridDivision,
    /// Ruler height in pixels
    height: f32,
    /// Show beat numbers within measures
    show_beat_numbers: bool,
    /// Show subdivision tick marks
    show_subdivisions: bool,
    /// Optional ID for `ScrollArea` (to avoid conflicts when multiple rulers exist)
    id: Option<egui::Id>,
}

impl Default for TimeRuler {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeRuler {
    /// Create a new time ruler with default settings
    #[must_use]
    pub const fn new() -> Self {
        Self {
            measures: 8,
            beat_width: 60.0,
            beats_per_measure: 4,
            division: GridDivision::Sixteenth,
            height: 36.0,
            show_beat_numbers: true,
            show_subdivisions: true,
            id: None,
        }
    }

    /// Set custom ID for the `ScrollArea` (prevents conflicts when multiple rulers exist)
    #[must_use]
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set number of measures to display
    #[must_use]
    pub const fn measures(mut self, measures: u32) -> Self {
        self.measures = measures;
        self
    }

    /// Set width per beat in pixels (zoom level)
    #[must_use]
    pub const fn beat_width(mut self, width: f32) -> Self {
        self.beat_width = width;
        self
    }

    /// Set beats per measure (time signature numerator)
    #[must_use]
    pub const fn beats_per_measure(mut self, beats: u32) -> Self {
        self.beats_per_measure = beats;
        self
    }

    /// Set ruler height
    #[must_use]
    pub const fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Show the time ruler within a pre-allocated clipped area
    ///
    /// Use this when the ruler is part of a scrollable timeline.
    /// The ruler paints within `ui.max_rect()` and respects `ui.clip_rect()`.
    /// Does not allocate additional space.
    pub fn show_clipped(self, ui: &mut Ui, theme: &Theme) -> Response {
        // Use max_rect as our drawing area (set by parent)
        let rect = ui.max_rect();
        let clip = ui.clip_rect();

        // Create a response for interaction
        let response = ui.allocate_rect(rect, Sense::hover());

        if ui.is_rect_visible(clip) {
            let painter = ui.painter();

            // Draw background (only within clip)
            painter.rect_filled(
                clip,
                f32::from(theme.spacing.corner_radius_small),
                theme.card(),
            );

            // Draw bottom border
            painter.line_segment(
                [
                    Pos2::new(clip.min.x, clip.max.y),
                    Pos2::new(clip.max.x, clip.max.y),
                ],
                Stroke::new(1.0, theme.input()),
            );

            // Draw vertical lines and tick marks
            self.draw_grid_lines(painter, theme, rect);

            // Draw measure numbers
            self.draw_measure_numbers(painter, theme, rect);

            // Draw beat numbers if enabled
            if self.show_beat_numbers {
                self.draw_beat_numbers(painter, theme, rect);
            }
        }

        response
    }

    /// Draw vertical grid lines
    fn draw_grid_lines(&self, painter: &egui::Painter, theme: &Theme, rect: Rect) {
        let divisions_per_beat = 1.0 / self.division.beat_fraction();
        let total_beats = self.measures as f32 * self.beats_per_measure as f32;
        let total_divisions = (total_beats * divisions_per_beat) as i32;

        for i in 0..=total_divisions {
            let beat_position = i as f32 * self.division.beat_fraction();
            let x = beat_position.mul_add(self.beat_width, rect.min.x);

            // Skip if out of bounds
            if x > rect.max.x {
                break;
            }

            // Determine line type
            let is_measure_line = (beat_position % self.beats_per_measure as f32) == 0.0;
            let is_beat_line = (beat_position % 1.0) == 0.0;

            if is_measure_line {
                painter.line_segment(
                    [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                    Stroke::new(2.0, theme.border()),
                );
            } else if is_beat_line {
                let line_height = rect.height() * 0.6;
                painter.line_segment(
                    [
                        Pos2::new(x, rect.max.y - line_height),
                        Pos2::new(x, rect.max.y),
                    ],
                    Stroke::new(1.5, theme.border()),
                );
            } else if self.show_subdivisions {
                let line_height = rect.height() * 0.3;
                painter.line_segment(
                    [
                        Pos2::new(x, rect.max.y - line_height),
                        Pos2::new(x, rect.max.y),
                    ],
                    Stroke::new(0.5, theme.input()),
                );
            }
        }
    }

    /// Draw measure numbers at the top
    fn draw_measure_numbers(&self, painter: &egui::Painter, theme: &Theme, rect: Rect) {
        for measure in 0..self.measures {
            let x = (measure as f32 * self.beats_per_measure as f32)
                .mul_add(self.beat_width, rect.min.x);
            let label_pos = Pos2::new(x + theme.spacing.xs, rect.min.y + theme.spacing.xs);

            let label = format!("{}", measure + 1);

            painter.text(
                label_pos,
                egui::Align2::LEFT_TOP,
                label,
                egui::FontId::proportional(11.0),
                theme.foreground(),
            );
        }
    }

    /// Draw beat numbers within measures
    fn draw_beat_numbers(&self, painter: &egui::Painter, theme: &Theme, rect: Rect) {
        let total_beats = self.measures as f32 * self.beats_per_measure as f32;

        for beat_idx in 0..(total_beats as u32) {
            // Skip if this is a measure boundary (already has measure number)
            if beat_idx % self.beats_per_measure == 0 {
                continue;
            }

            let x = (beat_idx as f32).mul_add(self.beat_width, rect.min.x);
            let beat_in_measure = (beat_idx % self.beats_per_measure) + 1;

            let label_pos = Pos2::new(
                theme.spacing.xs.mul_add(0.5, x),
                rect.min.y + theme.spacing.md,
            );

            painter.text(
                label_pos,
                egui::Align2::LEFT_TOP,
                format!("{beat_in_measure}"),
                egui::FontId::proportional(9.0),
                theme.muted_foreground(),
            );
        }
    }
}
