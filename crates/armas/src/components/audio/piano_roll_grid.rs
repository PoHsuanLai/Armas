//! Piano Roll Grid Component
//!
//! Background grid for piano roll editor that aligns with piano keys.
//! Provides horizontal lines for each note and vertical lines for time divisions.

use crate::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

/// Time division for vertical grid lines
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridDivision {
    /// Whole notes (4 beats)
    Whole,
    /// Half notes (2 beats)
    Half,
    /// Quarter notes (1 beat)
    Quarter,
    /// Eighth notes (1/2 beat)
    Eighth,
    /// Sixteenth notes (1/4 beat)
    Sixteenth,
}

impl GridDivision {
    /// Get the beat fraction for this division
    pub fn beat_fraction(&self) -> f32 {
        match self {
            GridDivision::Whole => 4.0,
            GridDivision::Half => 2.0,
            GridDivision::Quarter => 1.0,
            GridDivision::Eighth => 0.5,
            GridDivision::Sixteenth => 0.25,
        }
    }
}

/// Piano roll background grid
pub struct PianoRollGrid {
    /// Starting MIDI note (must match piano)
    start_note: u8,
    /// Number of octaves (must match piano)
    octaves: u8,
    /// Width of each white key (must match piano)
    white_key_width: f32,
    /// Height of white keys (must match piano)
    white_key_height: f32,
    /// Height of black keys as ratio of white keys
    black_key_height_ratio: f32,
    /// Number of measures to display
    measures: u32,
    /// Grid division for vertical lines
    division: GridDivision,
    /// Width per beat in pixels
    beat_width: f32,
    /// Show measure numbers
    show_measure_numbers: bool,
    /// Grid line opacity
    line_opacity: f32,
    /// Emphasize beat lines
    emphasize_beats: bool,
}

impl PianoRollGrid {
    /// Create a new piano roll grid
    pub fn new() -> Self {
        Self {
            start_note: 60, // C4
            octaves: 2,
            white_key_width: 40.0,
            white_key_height: 120.0,
            black_key_height_ratio: 0.6,
            measures: 4,
            division: GridDivision::Quarter,
            beat_width: 50.0,
            show_measure_numbers: true,
            line_opacity: 0.2,
            emphasize_beats: true,
        }
    }

    /// Set the starting MIDI note (should match piano)
    pub fn start_note(mut self, note: u8) -> Self {
        self.start_note = note;
        self
    }

    /// Set number of octaves (should match piano)
    pub fn octaves(mut self, octaves: u8) -> Self {
        self.octaves = octaves;
        self
    }

    /// Set width of white keys (should match piano)
    pub fn white_key_width(mut self, width: f32) -> Self {
        self.white_key_width = width;
        self
    }

    /// Set height of white keys (should match piano)
    pub fn white_key_height(mut self, height: f32) -> Self {
        self.white_key_height = height;
        self
    }

    /// Set black key height ratio (should match piano)
    pub fn black_key_height_ratio(mut self, ratio: f32) -> Self {
        self.black_key_height_ratio = ratio;
        self
    }

    /// Set number of measures to display
    pub fn measures(mut self, measures: u32) -> Self {
        self.measures = measures;
        self
    }

    /// Set grid division
    pub fn division(mut self, division: GridDivision) -> Self {
        self.division = division;
        self
    }

    /// Set width per beat in pixels
    pub fn beat_width(mut self, width: f32) -> Self {
        self.beat_width = width;
        self
    }

    /// Set whether to show measure numbers
    pub fn show_measure_numbers(mut self, show: bool) -> Self {
        self.show_measure_numbers = show;
        self
    }

    /// Set grid line opacity (0.0-1.0)
    pub fn line_opacity(mut self, opacity: f32) -> Self {
        self.line_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set whether to emphasize beat lines
    pub fn emphasize_beats(mut self, emphasize: bool) -> Self {
        self.emphasize_beats = emphasize;
        self
    }

    /// Show the piano roll grid
    pub fn show(self, ui: &mut Ui, theme: &Theme) -> Response {
        // Calculate total keys and white key count
        let total_notes = self.octaves as usize * 12;
        let white_key_count = (0..total_notes)
            .filter(|i| !Self::is_black_key((self.start_note + *i as u8) % 12))
            .count();

        // Calculate dimensions
        let grid_height = white_key_count as f32 * self.white_key_width;
        let beats_per_measure = 4.0; // Assume 4/4 time signature
        let total_beats = self.measures as f32 * beats_per_measure;
        let grid_width = total_beats * self.beat_width;

        // Allocate space
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(grid_width, grid_height), Sense::hover());

        if ui.is_rect_visible(rect) {
            // Draw alternating row backgrounds
            self.draw_alternating_rows(ui, theme, rect);

            // Draw horizontal lines (aligned with piano keys)
            self.draw_horizontal_lines(ui, theme, rect);

            // Draw vertical lines (time divisions)
            self.draw_vertical_lines(ui, theme, rect, beats_per_measure);

            // Draw measure numbers if enabled
            if self.show_measure_numbers {
                self.draw_measure_numbers(ui, theme, rect, beats_per_measure);
            }
        }

        response
    }

    /// Draw alternating row backgrounds for better visibility
    fn draw_alternating_rows(&self, ui: &Ui, theme: &Theme, rect: Rect) {
        let painter = ui.painter();
        let total_notes = self.octaves as usize * 12;

        // Determine if theme is light or dark based on background brightness
        let bg = theme.background();
        let is_light_theme = (bg.r() as u32 + bg.g() as u32 + bg.b() as u32) > 384; // > 128*3

        // Track white key index for positioning
        let mut white_key_index = 0;

        for i in 0..total_notes {
            let note = self.start_note + i as u8;
            let is_black = Self::is_black_key(note % 12);

            if !is_black {
                // Alternate background colors for white key rows
                if white_key_index % 2 == 1 {
                    let y = rect.min.y + white_key_index as f32 * self.white_key_width;
                    let row_rect = Rect::from_min_size(
                        Pos2::new(rect.min.x, y),
                        Vec2::new(rect.width(), self.white_key_width),
                    );

                    // Very subtle glassmorphic alternating color
                    let alt_color = if is_light_theme {
                        // Light theme: slightly darker rows with slight transparency
                        Color32::from_rgba_unmultiplied(0, 0, 0, 8)
                    } else {
                        // Dark theme: slightly lighter rows with slight transparency
                        Color32::from_rgba_unmultiplied(255, 255, 255, 8)
                    };

                    painter.rect_filled(row_rect, 2.0, alt_color);
                }

                white_key_index += 1;
            }
        }
    }

    /// Draw horizontal lines aligned with piano keys
    fn draw_horizontal_lines(&self, ui: &Ui, theme: &Theme, rect: Rect) {
        let painter = ui.painter();
        let total_notes = self.octaves as usize * 12;

        // Determine if theme is light or dark
        let bg = theme.background();
        let is_light_theme = (bg.r() as u32 + bg.g() as u32 + bg.b() as u32) > 384;

        // Use black lines for light theme, white lines for dark theme
        let base_line_color = if is_light_theme {
            Color32::from_rgb(0, 0, 0)
        } else {
            Color32::from_rgb(255, 255, 255)
        };

        // Track white key index for positioning
        let mut white_key_index = 0;

        for i in 0..total_notes {
            let note = self.start_note + i as u8;
            let is_black = Self::is_black_key(note % 12);

            if !is_black {
                // Draw line at white key boundary
                let y = rect.min.y + white_key_index as f32 * self.white_key_width;

                let line_color = Color32::from_rgba_unmultiplied(
                    base_line_color.r(),
                    base_line_color.g(),
                    base_line_color.b(),
                    (255.0 * self.line_opacity) as u8,
                );

                painter.line_segment(
                    [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                    Stroke::new(1.0, line_color),
                );

                white_key_index += 1;
            }
        }

        // Draw final bottom line
        let y = rect.min.y + white_key_index as f32 * self.white_key_width;
        let line_color = Color32::from_rgba_unmultiplied(
            base_line_color.r(),
            base_line_color.g(),
            base_line_color.b(),
            (255.0 * self.line_opacity) as u8,
        );
        painter.line_segment(
            [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
            Stroke::new(1.0, line_color),
        );
    }

    /// Draw vertical lines for time divisions
    fn draw_vertical_lines(&self, ui: &Ui, theme: &Theme, rect: Rect, beats_per_measure: f32) {
        let painter = ui.painter();
        let divisions_per_beat = 1.0 / self.division.beat_fraction();
        let total_beats = self.measures as f32 * beats_per_measure;
        let total_divisions = (total_beats * divisions_per_beat) as i32;

        // Determine if theme is light or dark
        let bg = theme.background();
        let is_light_theme = (bg.r() as u32 + bg.g() as u32 + bg.b() as u32) > 384;

        // Use black lines for light theme, white lines for dark theme
        let base_line_color = if is_light_theme {
            Color32::from_rgb(0, 0, 0)
        } else {
            Color32::from_rgb(255, 255, 255)
        };

        for i in 0..=total_divisions {
            let beat_position = i as f32 * self.division.beat_fraction();
            let x = rect.min.x + beat_position * self.beat_width;

            // Skip if out of bounds
            if x > rect.max.x {
                break;
            }

            // Determine line emphasis
            let is_measure_line = (beat_position % beats_per_measure) == 0.0;
            let is_beat_line = (beat_position % 1.0) == 0.0;

            let (stroke_width, opacity_multiplier) = if is_measure_line {
                (2.0, 2.5) // Measure lines are strongest
            } else if is_beat_line && self.emphasize_beats {
                (1.5, 1.8) // Beat lines are medium
            } else {
                (1.0, 1.0) // Division lines are subtle
            };

            let line_color = Color32::from_rgba_unmultiplied(
                base_line_color.r(),
                base_line_color.g(),
                base_line_color.b(),
                (255.0 * self.line_opacity * opacity_multiplier).min(255.0) as u8,
            );

            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                Stroke::new(stroke_width, line_color),
            );
        }
    }

    /// Draw measure numbers at the top of the grid
    fn draw_measure_numbers(&self, ui: &Ui, theme: &Theme, rect: Rect, beats_per_measure: f32) {
        let painter = ui.painter();

        for measure in 0..self.measures {
            let x = rect.min.x + measure as f32 * beats_per_measure * self.beat_width;
            let label_pos = Pos2::new(x + 4.0, rect.min.y + 4.0);

            painter.text(
                label_pos,
                egui::Align2::LEFT_TOP,
                format!("{}", measure + 1),
                egui::FontId::proportional(10.0),
                theme.muted_foreground(),
            );
        }
    }

    /// Check if a note is a black key
    fn is_black_key(note: u8) -> bool {
        matches!(note % 12, 1 | 3 | 6 | 8 | 10) // C#, D#, F#, G#, A#
    }
}

impl Default for PianoRollGrid {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from the piano roll grid
pub struct PianoRollGridResponse {
    /// The interaction response
    pub response: Response,
}
