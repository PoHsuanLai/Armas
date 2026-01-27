//! Time Ruler Component
//!
//! Horizontal ruler showing measures, beats, and subdivisions for DAW timeline.
//! Designed to align perfectly with PianoRollGrid's vertical grid lines.

use armas::theme::Theme;
use egui::{Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

/// Re-export GridDivision from piano_roll_grid for time subdivisions
pub use super::piano_roll_grid::GridDivision;

/// Response from the time ruler
#[derive(Debug, Clone)]
pub struct TimeRulerResponse {
    /// The UI response
    pub response: Response,
}

/// Time display mode for the ruler
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeDisplayMode {
    /// Display as bars:beats (e.g., "1.1", "1.2", "2.1")
    BarsBeatsSixteenths,
    /// Display as minutes:seconds (e.g., "0:00", "0:15")
    MinutesSeconds,
}

/// Horizontal time ruler for DAW timeline
///
/// Shows measures, beats, and subdivisions with precise alignment to PianoRollGrid.
/// Uses the same measurement system to ensure perfect synchronization.
///
/// # Example
///
/// ```rust,no_run
/// use armas::components::audio::{TimeRuler, GridDivision};
///
/// fn ui(ui: &mut egui::Ui, theme: &armas::Theme) {
///     TimeRuler::new()
///         .measures(8)
///         .beat_width(60.0)
///         .division(GridDivision::Sixteenth)
///         .show(ui, theme);
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
    /// Time display mode
    time_mode: TimeDisplayMode,
    /// Tempo (BPM) for minutes:seconds mode
    tempo: f32,
    /// Optional ID for ScrollArea (to avoid conflicts when multiple rulers exist)
    id: Option<egui::Id>,
}

impl Default for TimeRuler {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeRuler {
    /// Create a new time ruler with default settings
    pub fn new() -> Self {
        Self {
            measures: 8,
            beat_width: 60.0,
            beats_per_measure: 4,
            division: GridDivision::Sixteenth,
            height: 36.0,
            show_beat_numbers: true,
            show_subdivisions: true,
            time_mode: TimeDisplayMode::BarsBeatsSixteenths,
            tempo: 120.0,
            id: None,
        }
    }

    /// Set custom ID for the ScrollArea (prevents conflicts when multiple rulers exist)
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set number of measures to display
    pub fn measures(mut self, measures: u32) -> Self {
        self.measures = measures;
        self
    }

    /// Set width per beat in pixels (zoom level)
    pub fn beat_width(mut self, width: f32) -> Self {
        self.beat_width = width;
        self
    }

    /// Set beats per measure (time signature numerator)
    pub fn beats_per_measure(mut self, beats: u32) -> Self {
        self.beats_per_measure = beats;
        self
    }

    /// Set grid division for subdivisions
    pub fn division(mut self, division: GridDivision) -> Self {
        self.division = division;
        self
    }

    /// Set ruler height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set whether to show beat numbers
    pub fn show_beat_numbers(mut self, show: bool) -> Self {
        self.show_beat_numbers = show;
        self
    }

    /// Set whether to show subdivision tick marks
    pub fn show_subdivisions(mut self, show: bool) -> Self {
        self.show_subdivisions = show;
        self
    }

    /// Set time display mode
    pub fn time_mode(mut self, mode: TimeDisplayMode) -> Self {
        self.time_mode = mode;
        self
    }

    /// Set tempo (BPM) for minutes:seconds display
    pub fn tempo(mut self, tempo: f32) -> Self {
        self.tempo = tempo;
        self
    }

    /// Show the time ruler (allocates full content width)
    ///
    /// Use this when the ruler is the only content and should define its own size.
    /// For use inside a scrollable timeline, use `show_clipped()` instead.
    pub fn show(self, ui: &mut Ui, theme: &Theme) -> TimeRulerResponse {
        TimeRulerResponse {
            response: self.show_inner(ui, theme),
        }
    }

    /// Show the time ruler without ScrollArea wrapper
    pub fn show_no_scroll(self, ui: &mut Ui, theme: &Theme) -> TimeRulerResponse {
        TimeRulerResponse {
            response: self.show_inner(ui, theme),
        }
    }

    /// Show the time ruler within a pre-allocated clipped area
    ///
    /// Use this when the ruler is part of a scrollable timeline.
    /// The ruler paints within `ui.max_rect()` and respects `ui.clip_rect()`.
    /// Does not allocate additional space.
    pub fn show_clipped(self, ui: &mut Ui, theme: &Theme) -> TimeRulerResponse {
        // Use max_rect as our drawing area (set by parent)
        let rect = ui.max_rect();
        let clip = ui.clip_rect();

        // Create a response for interaction
        let response = ui.allocate_rect(rect, Sense::hover());

        if ui.is_rect_visible(clip) {
            let painter = ui.painter();

            // Draw background (only within clip)
            painter.rect_filled(clip, theme.spacing.corner_radius_small as f32, theme.card());

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

        TimeRulerResponse { response }
    }

    /// Internal render function (allocates full width)
    fn show_inner(self, ui: &mut Ui, theme: &Theme) -> Response {
        // Calculate width (MUST match PianoRollGrid calculation)
        let total_beats = self.measures as f32 * self.beats_per_measure as f32;
        let width = total_beats * self.beat_width;

        // Allocate space
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(width, self.height), Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Draw background
            painter.rect_filled(rect, theme.spacing.corner_radius_small as f32, theme.card());

            // Draw bottom border
            painter.line_segment(
                [
                    Pos2::new(rect.min.x, rect.max.y),
                    Pos2::new(rect.max.x, rect.max.y),
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

    /// Draw vertical grid lines matching PianoRollGrid
    fn draw_grid_lines(&self, painter: &egui::Painter, theme: &Theme, rect: Rect) {
        let divisions_per_beat = 1.0 / self.division.beat_fraction();
        let total_beats = self.measures as f32 * self.beats_per_measure as f32;
        let total_divisions = (total_beats * divisions_per_beat) as i32;

        for i in 0..=total_divisions {
            let beat_position = i as f32 * self.division.beat_fraction();
            let x = rect.min.x + beat_position * self.beat_width;

            // Skip if out of bounds
            if x > rect.max.x {
                break;
            }

            // Determine line type
            let is_measure_line = (beat_position % self.beats_per_measure as f32) == 0.0;
            let is_beat_line = (beat_position % 1.0) == 0.0;

            if is_measure_line {
                // Measure line - strong, full height
                painter.line_segment(
                    [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                    Stroke::new(2.0, theme.border()),
                );
            } else if is_beat_line {
                // Beat line - medium, 60% height
                let line_height = rect.height() * 0.6;
                painter.line_segment(
                    [
                        Pos2::new(x, rect.max.y - line_height),
                        Pos2::new(x, rect.max.y),
                    ],
                    Stroke::new(1.5, theme.border()),
                );
            } else if self.show_subdivisions {
                // Subdivision line - subtle, 30% height
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
            let x = rect.min.x + measure as f32 * self.beats_per_measure as f32 * self.beat_width;
            let label_pos = Pos2::new(x + theme.spacing.xs, rect.min.y + theme.spacing.xs);

            let label = match self.time_mode {
                TimeDisplayMode::BarsBeatsSixteenths => {
                    format!("{}", measure + 1)
                }
                TimeDisplayMode::MinutesSeconds => {
                    let total_beats = measure as f32 * self.beats_per_measure as f32;
                    let seconds = (total_beats / self.tempo) * 60.0;
                    let minutes = (seconds / 60.0) as u32;
                    let secs = (seconds % 60.0) as u32;
                    format!("{}:{:02}", minutes, secs)
                }
            };

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

            let x = rect.min.x + beat_idx as f32 * self.beat_width;
            let beat_in_measure = (beat_idx % self.beats_per_measure) + 1;

            // Position beat numbers below measure numbers
            let label_pos = Pos2::new(x + theme.spacing.xs * 0.5, rect.min.y + theme.spacing.md);

            painter.text(
                label_pos,
                egui::Align2::LEFT_TOP,
                format!("{}", beat_in_measure),
                egui::FontId::proportional(9.0),
                theme.muted_foreground(),
            );
        }
    }
}
