//! Piano Roll Component
//!
//! Complete DAW-style piano roll with vertical piano keyboard, grid, and interactive note blocks.
//! Supports clicking to place notes, dragging to resize, and beautiful glassmorphic styling.

use crate::components::audio::{GridDivision, Piano, PianoOrientation};
use crate::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

/// A single note in the piano roll
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
    /// MIDI note number (0-127)
    pub note: u8,
    /// Start position in beats
    pub start_beat: f32,
    /// Duration in beats
    pub duration: f32,
    /// Velocity (0.0-1.0)
    pub velocity: f32,
}

impl Note {
    /// Create a new note
    pub fn new(note: u8, start_beat: f32, duration: f32) -> Self {
        Self {
            note,
            start_beat,
            duration,
            velocity: 0.8,
        }
    }

    /// Create a new note with velocity
    pub fn with_velocity(note: u8, start_beat: f32, duration: f32, velocity: f32) -> Self {
        Self {
            note,
            start_beat,
            duration,
            velocity: velocity.clamp(0.0, 1.0),
        }
    }
}

/// Response from the piano roll
#[derive(Debug, Clone)]
pub struct PianoRollResponse {
    /// The notes after user interaction
    pub notes: Vec<Note>,
    /// Whether notes were modified this frame
    pub modified: bool,
    /// Newly added notes
    pub added_notes: Vec<Note>,
    /// Removed notes
    pub removed_notes: Vec<Note>,
}

/// Complete piano roll editor
pub struct PianoRoll {
    /// Starting MIDI note
    start_note: u8,
    /// Number of octaves
    octaves: u8,
    /// Width of white keys
    white_key_width: f32,
    /// Height of white keys
    white_key_height: f32,
    /// Number of measures
    measures: u32,
    /// Grid division
    division: GridDivision,
    /// Width per beat in pixels
    beat_width: f32,
    /// Default note duration when placing
    default_note_duration: f32,
    /// Notes to display
    notes: Vec<Note>,
    /// Show grid
    show_grid: bool,
    /// Show piano
    show_piano: bool,
    /// Note opacity
    note_opacity: f32,
    /// Enable note editing
    editable: bool,
}

impl PianoRoll {
    /// Create a new piano roll
    pub fn new() -> Self {
        Self {
            start_note: 60, // C4
            octaves: 2,
            white_key_width: 40.0,
            white_key_height: 120.0,
            measures: 4,
            division: GridDivision::Quarter,
            beat_width: 50.0,
            default_note_duration: 1.0, // Quarter note
            notes: Vec::new(),
            show_grid: true,
            show_piano: true,
            note_opacity: 0.85,
            editable: true,
        }
    }

    /// Set the starting MIDI note
    pub fn start_note(mut self, note: u8) -> Self {
        self.start_note = note;
        self
    }

    /// Set number of octaves
    pub fn octaves(mut self, octaves: u8) -> Self {
        self.octaves = octaves;
        self
    }

    /// Set width of white keys
    pub fn white_key_width(mut self, width: f32) -> Self {
        self.white_key_width = width;
        self
    }

    /// Set height of white keys
    pub fn white_key_height(mut self, height: f32) -> Self {
        self.white_key_height = height;
        self
    }

    /// Set number of measures
    pub fn measures(mut self, measures: u32) -> Self {
        self.measures = measures;
        self
    }

    /// Set grid division
    pub fn division(mut self, division: GridDivision) -> Self {
        self.division = division;
        self
    }

    /// Set width per beat
    pub fn beat_width(mut self, width: f32) -> Self {
        self.beat_width = width;
        self
    }

    /// Set default note duration when placing
    pub fn default_note_duration(mut self, duration: f32) -> Self {
        self.default_note_duration = duration;
        self
    }

    /// Set notes to display
    pub fn notes(mut self, notes: Vec<Note>) -> Self {
        self.notes = notes;
        self
    }

    /// Set whether to show grid
    pub fn show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Set whether to show piano
    pub fn show_piano(mut self, show: bool) -> Self {
        self.show_piano = show;
        self
    }

    /// Set note opacity
    pub fn note_opacity(mut self, opacity: f32) -> Self {
        self.note_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set whether notes are editable
    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = editable;
        self
    }

    /// Show the piano roll
    pub fn show(mut self, ui: &mut Ui, theme: &Theme) -> PianoRollResponse {
        let mut modified = false;
        let mut added_notes = Vec::new();
        let mut removed_notes = Vec::new();

        ui.horizontal(|ui| {
            // Vertical piano on the left
            if self.show_piano {
                let piano_response = Piano::new()
                    .start_note(self.start_note)
                    .octaves(self.octaves)
                    .white_key_width(self.white_key_width)
                    .white_key_height(self.white_key_height)
                    .orientation(PianoOrientation::Vertical)
                    .show(ui, theme);

                // Auto-add notes when piano keys are clicked
                if self.editable {
                    for key in piano_response.clicked_keys {
                        // Place note at the start of the first measure
                        let new_note = Note::new(key, 0.0, self.default_note_duration);
                        self.notes.push(new_note);
                        added_notes.push(new_note);
                        modified = true;
                    }
                }
            }

            // Grid and notes area
            let (_grid_response, note_interactions) = self.show_grid_and_notes(ui, theme);

            // Process note interactions
            if let Some(interactions) = note_interactions {
                if let Some(note) = interactions.added_note {
                    self.notes.push(note);
                    added_notes.push(note);
                    modified = true;
                }

                for &idx in &interactions.removed_indices {
                    if idx < self.notes.len() {
                        removed_notes.push(self.notes[idx]);
                    }
                }

                // Remove notes (in reverse order to maintain indices)
                for &idx in interactions.removed_indices.iter().rev() {
                    if idx < self.notes.len() {
                        self.notes.remove(idx);
                        modified = true;
                    }
                }
            }
        });

        PianoRollResponse {
            notes: self.notes.clone(),
            modified,
            added_notes,
            removed_notes,
        }
    }

    /// Show the grid and notes area
    fn show_grid_and_notes(
        &self,
        ui: &mut Ui,
        theme: &Theme,
    ) -> (Response, Option<NoteInteractions>) {
        // Calculate dimensions
        let total_notes = self.octaves as usize * 12;
        let white_key_count = (0..total_notes)
            .filter(|i| !Self::is_black_key((self.start_note + *i as u8) % 12))
            .count();

        let grid_height = white_key_count as f32 * self.white_key_width;
        let beats_per_measure = 4.0;
        let total_beats = self.measures as f32 * beats_per_measure;
        let grid_width = total_beats * self.beat_width;

        // Allocate space for the grid - use drag for note placement
        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(grid_width, grid_height),
            if self.editable {
                Sense::click_and_drag()
            } else {
                Sense::hover()
            },
        );

        if ui.is_rect_visible(rect) {
            // Draw grid background
            if self.show_grid {
                self.draw_grid_background(ui, theme, rect);
            }

            // Draw existing notes
            self.draw_notes(ui, theme, rect);

            // Draw hover preview if editable
            let note_interactions = if self.editable {
                self.handle_interactions(ui, theme, rect, &response)
            } else {
                None
            };

            return (response, note_interactions);
        }

        (response, None)
    }

    /// Draw the grid background
    fn draw_grid_background(&self, ui: &Ui, theme: &Theme, rect: Rect) {
        // Draw alternating rows
        self.draw_alternating_rows(ui, theme, rect);

        // Draw grid lines
        self.draw_horizontal_lines(ui, theme, rect);
        self.draw_vertical_lines(ui, theme, rect);
    }

    /// Draw alternating row backgrounds
    fn draw_alternating_rows(&self, ui: &Ui, theme: &Theme, rect: Rect) {
        let painter = ui.painter();
        let total_notes = self.octaves as usize * 12;
        let bg = theme.background();
        let is_light_theme = (bg.r() as u32 + bg.g() as u32 + bg.b() as u32) > 384;

        let mut white_key_index = 0;
        for i in 0..total_notes {
            let note = self.start_note + i as u8;
            let is_black = Self::is_black_key(note % 12);

            if !is_black {
                if white_key_index % 2 == 1 {
                    let y = rect.min.y + white_key_index as f32 * self.white_key_width;
                    let row_rect = Rect::from_min_size(
                        Pos2::new(rect.min.x, y),
                        Vec2::new(rect.width(), self.white_key_width),
                    );

                    let alt_color = if is_light_theme {
                        Color32::from_rgba_unmultiplied(0, 0, 0, 8)
                    } else {
                        Color32::from_rgba_unmultiplied(255, 255, 255, 8)
                    };

                    painter.rect_filled(row_rect, 2.0, alt_color);
                }
                white_key_index += 1;
            }
        }
    }

    /// Draw horizontal grid lines
    fn draw_horizontal_lines(&self, ui: &Ui, theme: &Theme, rect: Rect) {
        let painter = ui.painter();
        let total_notes = self.octaves as usize * 12;
        let bg = theme.background();
        let is_light_theme = (bg.r() as u32 + bg.g() as u32 + bg.b() as u32) > 384;

        let base_line_color = if is_light_theme {
            Color32::from_rgb(0, 0, 0)
        } else {
            Color32::from_rgb(255, 255, 255)
        };

        let mut white_key_index = 0;
        for i in 0..total_notes {
            let note = self.start_note + i as u8;
            let is_black = Self::is_black_key(note % 12);

            if !is_black {
                let y = rect.min.y + white_key_index as f32 * self.white_key_width;
                let line_color = Color32::from_rgba_unmultiplied(
                    base_line_color.r(),
                    base_line_color.g(),
                    base_line_color.b(),
                    (255.0 * 0.2) as u8,
                );

                painter.line_segment(
                    [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                    Stroke::new(1.0, line_color),
                );

                white_key_index += 1;
            }
        }
    }

    /// Draw vertical grid lines
    fn draw_vertical_lines(&self, ui: &Ui, theme: &Theme, rect: Rect) {
        let painter = ui.painter();
        let bg = theme.background();
        let is_light_theme = (bg.r() as u32 + bg.g() as u32 + bg.b() as u32) > 384;

        let base_line_color = if is_light_theme {
            Color32::from_rgb(0, 0, 0)
        } else {
            Color32::from_rgb(255, 255, 255)
        };

        let divisions_per_beat = 1.0 / self.division.beat_fraction();
        let beats_per_measure = 4.0;
        let total_beats = self.measures as f32 * beats_per_measure;
        let total_divisions = (total_beats * divisions_per_beat) as i32;

        for i in 0..=total_divisions {
            let beat_position = i as f32 * self.division.beat_fraction();
            let x = rect.min.x + beat_position * self.beat_width;

            if x > rect.max.x {
                break;
            }

            let is_measure_line = (beat_position % beats_per_measure) == 0.0;
            let is_beat_line = (beat_position % 1.0) == 0.0;

            let (stroke_width, opacity_multiplier) = if is_measure_line {
                (2.0, 2.5)
            } else if is_beat_line {
                (1.5, 1.8)
            } else {
                (1.0, 1.0)
            };

            let alpha_val = 255.0 * 0.2 * opacity_multiplier;
            let alpha = if alpha_val > 255.0 {
                255
            } else {
                alpha_val as u8
            };
            let line_color = Color32::from_rgba_unmultiplied(
                base_line_color.r(),
                base_line_color.g(),
                base_line_color.b(),
                alpha,
            );

            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                Stroke::new(stroke_width, line_color),
            );
        }
    }

    /// Draw note blocks
    fn draw_notes(&self, ui: &Ui, theme: &Theme, rect: Rect) {
        let painter = ui.painter();

        for note in &self.notes {
            if let Some(note_rect) = self.get_note_rect(note, rect) {
                // Glassmorphic note block with gradient
                let primary = theme.primary();

                // Base color with velocity-based intensity
                let intensity = (note.velocity * 255.0) as u8;
                let note_color = Color32::from_rgba_unmultiplied(
                    primary.r(),
                    primary.g(),
                    primary.b(),
                    (intensity as f32 * self.note_opacity) as u8,
                );

                // Draw note block with rounded corners
                painter.rect_filled(note_rect, 4.0, note_color);

                // Subtle border for definition
                let border_color = Color32::from_rgba_unmultiplied(
                    primary.r(),
                    primary.g(),
                    primary.b(),
                    ((intensity as f32 * 1.3).min(255.0)) as u8,
                );
                painter.rect_stroke(
                    note_rect,
                    4.0,
                    Stroke::new(1.0, border_color),
                    egui::StrokeKind::Outside,
                );

                // Top highlight for glass effect
                let highlight_rect = Rect::from_min_size(
                    note_rect.min,
                    Vec2::new(note_rect.width(), note_rect.height() * 0.3),
                );
                let highlight_color = Color32::from_rgba_unmultiplied(255, 255, 255, 20);
                let highlight_rounding = egui::CornerRadius {
                    nw: 4,
                    ne: 4,
                    sw: 0,
                    se: 0,
                };
                painter.rect_filled(highlight_rect, highlight_rounding, highlight_color);
            }
        }
    }

    /// Get the rectangle for a note
    fn get_note_rect(&self, note: &Note, grid_rect: Rect) -> Option<Rect> {
        // Convert note number to row position
        let row = self.note_to_row(note.note)?;

        // Calculate position
        let x = grid_rect.min.x + note.start_beat * self.beat_width;
        let y = grid_rect.min.y + row as f32 * self.white_key_width;
        let width = note.duration * self.beat_width;
        let height = self.white_key_width;

        Some(Rect::from_min_size(
            Pos2::new(x, y),
            Vec2::new(width, height),
        ))
    }

    /// Convert MIDI note to row index
    fn note_to_row(&self, note: u8) -> Option<usize> {
        if note < self.start_note {
            return None;
        }

        let note_offset = (note - self.start_note) as usize;
        let total_notes = self.octaves as usize * 12;

        if note_offset >= total_notes {
            return None;
        }

        // Count white keys up to this note
        let mut white_key_count = 0;
        for i in 0..=note_offset {
            let current_note = self.start_note + i as u8;
            if !Self::is_black_key(current_note % 12) {
                if i == note_offset {
                    return Some(white_key_count);
                }
                white_key_count += 1;
            }
        }

        None
    }

    /// Handle mouse interactions for note placement/removal
    fn handle_interactions(
        &self,
        ui: &Ui,
        theme: &Theme,
        rect: Rect,
        response: &Response,
    ) -> Option<NoteInteractions> {
        let mut interactions = NoteInteractions::default();

        // Handle click to remove notes
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(clicked_idx) = self.find_note_at_pos(pos, rect) {
                    // Remove note
                    interactions.removed_indices.push(clicked_idx);
                }
            }
        }

        // Handle drag to place notes
        if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                // Only place note if not over existing note
                if self.find_note_at_pos(pos, rect).is_none() {
                    if let Some(new_note) = self.pos_to_note(pos, rect) {
                        // Check if we already have a note at this exact position
                        let note_exists = self.notes.iter().any(|n| {
                            n.note == new_note.note
                                && (n.start_beat - new_note.start_beat).abs() < 0.01
                        });

                        if !note_exists {
                            interactions.added_note = Some(new_note);
                        }
                    }
                }
            }
        }

        // Handle drag start to place initial note
        if response.drag_started() {
            if let Some(pos) = response.interact_pointer_pos() {
                if self.find_note_at_pos(pos, rect).is_none() {
                    if let Some(new_note) = self.pos_to_note(pos, rect) {
                        interactions.added_note = Some(new_note);
                    }
                }
            }
        }

        // Draw hover preview
        if let Some(hover_pos) = response.hover_pos() {
            if let Some(preview_note) = self.pos_to_note(hover_pos, rect) {
                if self.find_note_at_pos(hover_pos, rect).is_none() {
                    // Draw ghost preview
                    if let Some(note_rect) = self.get_note_rect(&preview_note, rect) {
                        let painter = ui.painter();
                        let primary = theme.primary();
                        let ghost_color = Color32::from_rgba_unmultiplied(
                            primary.r(),
                            primary.g(),
                            primary.b(),
                            40,
                        );
                        painter.rect_filled(note_rect, 4.0, ghost_color);
                        painter.rect_stroke(
                            note_rect,
                            4.0,
                            Stroke::new(1.0, primary),
                            egui::StrokeKind::Outside,
                        );
                    }
                }
            }
        }

        Some(interactions)
    }

    /// Convert position to note
    fn pos_to_note(&self, pos: Pos2, grid_rect: Rect) -> Option<Note> {
        if !grid_rect.contains(pos) {
            return None;
        }

        // Get row
        let row = ((pos.y - grid_rect.min.y) / self.white_key_width).floor() as usize;
        let note_num = self.row_to_note(row)?;

        // Get beat position (snap to grid)
        let beat_pos = (pos.x - grid_rect.min.x) / self.beat_width;
        let snap_amount = self.division.beat_fraction();
        let snapped_beat = (beat_pos / snap_amount).floor() * snap_amount;

        Some(Note::new(
            note_num,
            snapped_beat,
            self.default_note_duration,
        ))
    }

    /// Convert row index to MIDI note
    fn row_to_note(&self, row: usize) -> Option<u8> {
        let total_notes = self.octaves as usize * 12;
        let mut white_key_count = 0;

        for i in 0..total_notes {
            let note = self.start_note + i as u8;
            if !Self::is_black_key(note % 12) {
                if white_key_count == row {
                    return Some(note);
                }
                white_key_count += 1;
            }
        }

        None
    }

    /// Find note at position
    fn find_note_at_pos(&self, pos: Pos2, grid_rect: Rect) -> Option<usize> {
        for (idx, note) in self.notes.iter().enumerate() {
            if let Some(note_rect) = self.get_note_rect(note, grid_rect) {
                if note_rect.contains(pos) {
                    return Some(idx);
                }
            }
        }
        None
    }

    /// Check if a note is a black key
    fn is_black_key(note: u8) -> bool {
        matches!(note % 12, 1 | 3 | 6 | 8 | 10)
    }
}

impl Default for PianoRoll {
    fn default() -> Self {
        Self::new()
    }
}

/// Note interaction results
#[derive(Debug, Default)]
struct NoteInteractions {
    added_note: Option<Note>,
    removed_indices: Vec<usize>,
}
