//! Piano Roll Component
//!
//! Complete DAW-style piano roll with vertical piano keyboard, grid, and interactive note blocks.
//! Supports clicking to place notes, dragging to resize, and beautiful glassmorphic styling.

use crate::{Piano, PianoOrientation};
use armas_basic::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

/// Time division for vertical grid lines
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    #[must_use]
    pub const fn beat_fraction(&self) -> f32 {
        match self {
            Self::Whole => 4.0,
            Self::Half => 2.0,
            Self::Quarter => 1.0,
            Self::Eighth => 0.5,
            Self::Sixteenth => 0.25,
        }
    }
}

/// Momentum scroll state stored in egui temp data
#[derive(Clone, Default)]
struct PianoRollScrollState {
    /// Current scroll offset
    offset: Vec2,
    /// Current velocity (pixels per second)
    velocity: Vec2,
    /// Last frame time for delta calculation
    last_frame_time: f64,
    /// Whether momentum is active
    is_animating: bool,
}

/// Grid line style for piano roll
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridLineStyle {
    /// Color of measure lines (darkest)
    pub measure_color: Color32,
    /// Color of beat lines
    pub beat_color: Color32,
    /// Color of subdivision lines
    pub subdivision_color: Color32,
    /// Width of measure lines
    pub measure_width: f32,
    /// Width of beat lines
    pub beat_width: f32,
    /// Width of subdivision lines
    pub subdivision_width: f32,
}

/// A single note in the piano roll
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
    /// MIDI note number (0-127)
    pub note: u8,
    /// Start position in beats
    pub start_beat: f64,
    /// Duration in beats
    pub duration: f64,
    /// Velocity (0-127, MIDI standard)
    pub velocity: u8,
}

impl Note {
    /// Create a new note
    #[must_use]
    pub const fn new(note: u8, start_beat: f64, duration: f64) -> Self {
        Self {
            note,
            start_beat,
            duration,
            velocity: 100,
        }
    }

    /// Create a new note with velocity
    #[must_use]
    pub const fn with_velocity(note: u8, start_beat: f64, duration: f64, velocity: u8) -> Self {
        Self {
            note,
            start_beat,
            duration,
            velocity,
        }
    }
}

/// Response from the piano roll
pub struct PianoRollResponse {
    /// The UI response
    pub response: Response,
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
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # use armas_basic::Theme;
/// # fn example(ui: &mut Ui, theme: &Theme) {
/// use armas_audio::{PianoRoll, Note, AudioTiming};
///
/// let notes = vec![
///     Note::new(60, 0.0, 1.0),  // C4
///     Note::new(64, 1.0, 1.0),  // E4
///     Note::new(67, 2.0, 1.0),  // G4
/// ];
/// let response = PianoRoll::new()
///     .notes(notes)
///     .measures(4)
///     .show(ui, theme);
/// # }
/// ```
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
    pub(crate) measures: u32,
    /// Beats per measure (time signature numerator)
    pub(crate) beats_per_measure: u32,
    /// Grid division
    division: GridDivision,
    /// Width per beat in pixels
    pub(crate) beat_width: f32,
    /// Default note duration when placing
    default_note_duration: f64,
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
    /// Black key color
    black_key_color: Option<Color32>,
    /// Note block color
    note_color: Option<Color32>,
    /// Selected note color
    selected_note_color: Option<Color32>,
    /// Grid line style configuration
    grid_style: Option<GridLineStyle>,
    /// Snap to grid enabled
    snap_to_grid: bool,
    /// Enable scrolling
    scrollable: bool,
    /// Viewport width (if scrollable)
    viewport_width: Option<f32>,
    /// Viewport height (if scrollable)
    viewport_height: Option<f32>,
    /// Enable momentum scrolling
    pub(crate) momentum_scrolling: bool,
    /// Momentum damping factor
    pub(crate) momentum_damping: f64,
    /// Optional ID for state persistence
    id: Option<egui::Id>,
    /// Whether the piano roll is disabled (non-interactive)
    disabled: bool,
}

impl PianoRoll {
    /// Create a new piano roll
    #[must_use]
    pub const fn new() -> Self {
        Self {
            start_note: 60, // C4
            octaves: 2,
            white_key_width: 40.0,
            white_key_height: 120.0,
            measures: 4,
            beats_per_measure: 4,
            division: GridDivision::Quarter,
            beat_width: 50.0,
            default_note_duration: 1.0, // Quarter note
            notes: Vec::new(),
            show_grid: true,
            show_piano: true,
            note_opacity: 0.85,
            editable: true,
            black_key_color: None,
            note_color: None,
            selected_note_color: None,
            grid_style: None,
            snap_to_grid: false,
            scrollable: false,
            viewport_width: None,
            viewport_height: None,
            momentum_scrolling: true,
            momentum_damping: 5.0,
            id: None,
            disabled: false,
        }
    }

    /// Set the starting MIDI note
    #[must_use]
    pub const fn start_note(mut self, note: u8) -> Self {
        self.start_note = note;
        self
    }

    /// Set number of octaves
    #[must_use]
    pub const fn octaves(mut self, octaves: u8) -> Self {
        self.octaves = octaves;
        self
    }

    /// Set width of white keys
    #[must_use]
    pub const fn white_key_width(mut self, width: f32) -> Self {
        self.white_key_width = width;
        self
    }

    /// Set height of white keys
    #[must_use]
    pub const fn white_key_height(mut self, height: f32) -> Self {
        self.white_key_height = height;
        self
    }

    /// Set grid division
    #[must_use]
    pub const fn division(mut self, division: GridDivision) -> Self {
        self.division = division;
        self
    }

    /// Set default note duration when placing
    #[must_use]
    pub const fn default_note_duration(mut self, duration: f64) -> Self {
        self.default_note_duration = duration;
        self
    }

    /// Set notes to display
    #[must_use]
    pub fn notes(mut self, notes: Vec<Note>) -> Self {
        self.notes = notes;
        self
    }

    /// Set whether to show grid
    #[must_use]
    pub const fn show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Set whether to show piano
    #[must_use]
    pub const fn show_piano(mut self, show: bool) -> Self {
        self.show_piano = show;
        self
    }

    /// Set note opacity
    #[must_use]
    pub const fn note_opacity(mut self, opacity: f32) -> Self {
        self.note_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set whether notes are editable
    #[must_use]
    pub const fn editable(mut self, editable: bool) -> Self {
        self.editable = editable;
        self
    }

    /// Set black key color
    #[must_use]
    pub const fn black_key_color(mut self, color: Color32) -> Self {
        self.black_key_color = Some(color);
        self
    }

    /// Set note block color
    #[must_use]
    pub const fn note_color(mut self, color: Color32) -> Self {
        self.note_color = Some(color);
        self
    }

    /// Set selected note color
    #[must_use]
    pub const fn selected_note_color(mut self, color: Color32) -> Self {
        self.selected_note_color = Some(color);
        self
    }

    /// Set grid line style for customizing line appearance
    #[must_use]
    pub const fn grid_style(mut self, style: GridLineStyle) -> Self {
        self.grid_style = Some(style);
        self
    }

    /// Set whether the piano roll is disabled (non-interactive)
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Enable snap-to-grid for note placement and resizing
    #[must_use]
    pub const fn snap_to_grid(mut self, enabled: bool) -> Self {
        self.snap_to_grid = enabled;
        self
    }

    /// Set custom ID for state persistence
    #[must_use]
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Enable scrolling with specified viewport size
    ///
    /// When enabled, the piano roll content can be scrolled within
    /// the specified viewport dimensions.
    #[must_use]
    pub const fn scrollable(mut self, width: f32, height: f32) -> Self {
        self.scrollable = true;
        self.viewport_width = Some(width);
        self.viewport_height = Some(height);
        self
    }

    /// Show the piano roll
    pub fn show(mut self, ui: &mut Ui, theme: &Theme) -> PianoRollResponse {
        let mut modified = false;
        let mut added_notes = Vec::new();
        let mut removed_notes = Vec::new();

        let outer_response = ui.horizontal(|ui| {
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
                if self.editable && !self.disabled {
                    for key in piano_response.clicked_keys {
                        // Place note at the start of the first measure
                        let new_note = Note::new(key, 0.0_f64, self.default_note_duration);
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
            response: outer_response.response,
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
        // Calculate content dimensions
        let total_notes = self.octaves as usize * 12;
        let white_key_count = (0..total_notes)
            .filter(|i| !Self::is_black_key((self.start_note + *i as u8) % 12))
            .count();

        let content_height = white_key_count as f32 * self.white_key_width;
        let measures = self.measures as f32;
        let beats_per_measure = self.beats_per_measure as f32;
        let total_beats = measures * beats_per_measure;
        let content_width = total_beats * self.beat_width;

        // Determine viewport size
        let (viewport_width, viewport_height) = if self.scrollable {
            (
                self.viewport_width.unwrap_or(content_width),
                self.viewport_height.unwrap_or(content_height),
            )
        } else {
            (content_width, content_height)
        };

        // Allocate viewport space
        let sense = if self.disabled {
            Sense::hover()
        } else if self.editable {
            Sense::click_and_drag()
        } else {
            Sense::hover()
        };
        let (viewport_rect, response) =
            ui.allocate_exact_size(Vec2::new(viewport_width, viewport_height), sense);

        // Get or create scroll state
        let scroll_id = self.id.unwrap_or_else(|| ui.id()).with("piano_roll_scroll");
        let mut scroll_state: PianoRollScrollState = ui
            .ctx()
            .data_mut(|d| d.get_temp(scroll_id).unwrap_or_default());

        // Calculate max scroll
        let max_scroll_x = (content_width - viewport_width).max(0.0);
        let max_scroll_y = (content_height - viewport_height).max(0.0);

        // Handle scroll input if scrollable
        if self.scrollable && response.hovered() {
            let current_time = ui.ctx().input(|i| i.time);
            let dt = if scroll_state.last_frame_time > 0.0 {
                (current_time - scroll_state.last_frame_time) as f32
            } else {
                0.016
            };
            scroll_state.last_frame_time = current_time;

            // Handle scroll wheel
            ui.ctx().input(|i| {
                let delta = i.smooth_scroll_delta;
                if delta != Vec2::ZERO {
                    scroll_state.offset.x -= delta.x;
                    scroll_state.offset.y -= delta.y;

                    if self.momentum_scrolling {
                        let velocity_scale = 8.0;
                        scroll_state.velocity.x = -delta.x * velocity_scale / dt.max(0.001);
                        scroll_state.velocity.y = -delta.y * velocity_scale / dt.max(0.001);
                        scroll_state.is_animating = true;
                    }
                }
            });

            // Apply momentum
            if self.momentum_scrolling && scroll_state.is_animating {
                let momentum_delta = scroll_state.velocity * dt;

                if momentum_delta.x.abs() > 0.01 || momentum_delta.y.abs() > 0.01 {
                    scroll_state.offset += momentum_delta;

                    let damping = (-self.momentum_damping as f32 * dt).exp();
                    scroll_state.velocity *= damping;

                    ui.ctx().request_repaint();
                }

                let min_velocity = 5.0;
                if scroll_state.velocity.x.abs() < min_velocity
                    && scroll_state.velocity.y.abs() < min_velocity
                {
                    scroll_state.velocity = Vec2::ZERO;
                    scroll_state.is_animating = false;
                }
            }
        }

        // Clamp scroll offset
        scroll_state.offset.x = scroll_state.offset.x.clamp(0.0, max_scroll_x);
        scroll_state.offset.y = scroll_state.offset.y.clamp(0.0, max_scroll_y);

        // Store scroll state
        ui.ctx()
            .data_mut(|d| d.insert_temp(scroll_id, scroll_state.clone()));

        if ui.is_rect_visible(viewport_rect) {
            // Calculate the content rect (offset by scroll)
            let content_rect = if self.scrollable {
                Rect::from_min_size(
                    viewport_rect.min - scroll_state.offset,
                    Vec2::new(content_width, content_height),
                )
            } else {
                viewport_rect
            };

            // Set clip rect for scrollable content
            let painter = ui.painter().with_clip_rect(viewport_rect);

            // Draw grid background
            if self.show_grid {
                self.draw_grid_background_clipped(&painter, theme, content_rect, viewport_rect);
            }

            // Draw existing notes
            self.draw_notes_clipped(&painter, theme, content_rect, viewport_rect);

            // Draw hover preview if editable and not disabled
            let note_interactions = if self.editable && !self.disabled {
                Some(self.handle_interactions_scrolled(
                    ui,
                    theme,
                    content_rect,
                    viewport_rect,
                    &response,
                    scroll_state.offset,
                ))
            } else {
                None
            };

            return (response, note_interactions);
        }

        (response, None)
    }

    /// Draw grid background with clipping support
    fn draw_grid_background_clipped(
        &self,
        painter: &egui::Painter,
        theme: &Theme,
        content_rect: Rect,
        _viewport_rect: Rect,
    ) {
        // Draw alternating rows
        self.draw_alternating_rows_clipped(painter, theme, content_rect);

        // Draw grid lines
        self.draw_horizontal_lines_clipped(painter, theme, content_rect);
        self.draw_vertical_lines_clipped(painter, theme, content_rect);
    }

    /// Draw alternating rows with clipping
    fn draw_alternating_rows_clipped(&self, painter: &egui::Painter, theme: &Theme, rect: Rect) {
        let total_notes = self.octaves as usize * 12;
        let bg = theme.background();
        let is_light_theme = (u32::from(bg.r()) + u32::from(bg.g()) + u32::from(bg.b())) > 384;

        let mut white_key_index = 0;
        for i in 0..total_notes {
            let note = self.start_note + i as u8;
            let is_black = Self::is_black_key(note % 12);

            if !is_black {
                if white_key_index % 2 == 1 {
                    let y = (white_key_index as f32).mul_add(self.white_key_width, rect.min.y);
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

    /// Draw horizontal lines with clipping
    fn draw_horizontal_lines_clipped(&self, painter: &egui::Painter, theme: &Theme, rect: Rect) {
        let total_notes = self.octaves as usize * 12;
        let bg = theme.background();
        let is_light_theme = (u32::from(bg.r()) + u32::from(bg.g()) + u32::from(bg.b())) > 384;

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
                let y = (white_key_index as f32).mul_add(self.white_key_width, rect.min.y);
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

    /// Draw vertical lines with clipping
    fn draw_vertical_lines_clipped(&self, painter: &egui::Painter, theme: &Theme, rect: Rect) {
        let bg = theme.background();
        let is_light_theme = (u32::from(bg.r()) + u32::from(bg.g()) + u32::from(bg.b())) > 384;

        let base_line_color = if is_light_theme {
            Color32::from_rgb(0, 0, 0)
        } else {
            Color32::from_rgb(255, 255, 255)
        };

        let divisions_per_beat = 1.0 / self.division.beat_fraction();
        let measures = self.measures as f32;
        let beats_per_measure = self.beats_per_measure as f32;
        let total_beats = measures * beats_per_measure;
        let total_divisions = (total_beats * divisions_per_beat) as i32;

        for i in 0..=total_divisions {
            let beat_position = i as f32 * self.division.beat_fraction();
            let x = beat_position.mul_add(self.beat_width, rect.min.x);

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

    /// Draw notes with clipping support
    fn draw_notes_clipped(
        &self,
        painter: &egui::Painter,
        theme: &Theme,
        content_rect: Rect,
        _viewport_rect: Rect,
    ) {
        let dim = if self.disabled { 0.5 } else { 1.0 };
        for note in &self.notes {
            if let Some(note_rect) = self.get_note_rect_in_content(note, content_rect) {
                let primary = theme.primary().gamma_multiply(dim);
                let velocity_f = f32::from(note.velocity) / 127.0;
                let intensity = (velocity_f * 255.0) as u8;
                let note_color = Color32::from_rgba_unmultiplied(
                    primary.r(),
                    primary.g(),
                    primary.b(),
                    (f32::from(intensity) * self.note_opacity) as u8,
                );

                painter.rect_filled(note_rect, 4.0, note_color);

                let border_color = Color32::from_rgba_unmultiplied(
                    primary.r(),
                    primary.g(),
                    primary.b(),
                    ((f32::from(intensity) * 1.3).min(255.0)) as u8,
                );
                painter.rect_stroke(
                    note_rect,
                    4.0,
                    Stroke::new(1.0, border_color),
                    egui::StrokeKind::Outside,
                );

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

    /// Get note rect in content space
    fn get_note_rect_in_content(&self, note: &Note, content_rect: Rect) -> Option<Rect> {
        let row = self.note_to_row(note.note)?;
        let start = note.start_beat as f32;
        let dur = note.duration as f32;
        let x = start.mul_add(self.beat_width, content_rect.min.x);
        let y = (row as f32).mul_add(self.white_key_width, content_rect.min.y);
        let width = dur * self.beat_width;
        let height = self.white_key_width;

        Some(Rect::from_min_size(
            Pos2::new(x, y),
            Vec2::new(width, height),
        ))
    }

    /// Handle interactions with scroll offset support
    fn handle_interactions_scrolled(
        &self,
        ui: &Ui,
        theme: &Theme,
        content_rect: Rect,
        viewport_rect: Rect,
        response: &Response,
        scroll_offset: Vec2,
    ) -> NoteInteractions {
        let mut interactions = NoteInteractions::default();

        // Convert screen position to content position
        let to_content_pos = |screen_pos: Pos2| -> Pos2 {
            Pos2::new(
                screen_pos.x + scroll_offset.x - (viewport_rect.min.x - content_rect.min.x),
                screen_pos.y + scroll_offset.y - (viewport_rect.min.y - content_rect.min.y),
            )
        };

        // Handle click to remove notes
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let content_pos = to_content_pos(pos);
                if let Some(clicked_idx) = self.find_note_at_content_pos(content_pos, content_rect)
                {
                    interactions.removed_indices.push(clicked_idx);
                }
            }
        }

        // Handle drag to place notes
        if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                let content_pos = to_content_pos(pos);
                if self
                    .find_note_at_content_pos(content_pos, content_rect)
                    .is_none()
                {
                    if let Some(new_note) = self.content_pos_to_note(content_pos, content_rect) {
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

        // Handle drag start
        if response.drag_started() {
            if let Some(pos) = response.interact_pointer_pos() {
                let content_pos = to_content_pos(pos);
                if self
                    .find_note_at_content_pos(content_pos, content_rect)
                    .is_none()
                {
                    if let Some(new_note) = self.content_pos_to_note(content_pos, content_rect) {
                        interactions.added_note = Some(new_note);
                    }
                }
            }
        }

        // Draw hover preview
        if let Some(hover_pos) = response.hover_pos() {
            let content_pos = to_content_pos(hover_pos);
            if let Some(preview_note) = self.content_pos_to_note(content_pos, content_rect) {
                if self
                    .find_note_at_content_pos(content_pos, content_rect)
                    .is_none()
                {
                    if let Some(note_rect) =
                        self.get_note_rect_in_content(&preview_note, content_rect)
                    {
                        let painter = ui.painter().with_clip_rect(viewport_rect);
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

        interactions
    }

    /// Find note at content position
    fn find_note_at_content_pos(&self, pos: Pos2, content_rect: Rect) -> Option<usize> {
        for (idx, note) in self.notes.iter().enumerate() {
            if let Some(note_rect) = self.get_note_rect_in_content(note, content_rect) {
                if note_rect.contains(pos) {
                    return Some(idx);
                }
            }
        }
        None
    }

    /// Convert content position to note
    fn content_pos_to_note(&self, pos: Pos2, content_rect: Rect) -> Option<Note> {
        if pos.x < content_rect.min.x || pos.y < content_rect.min.y {
            return None;
        }

        let row = ((pos.y - content_rect.min.y) / self.white_key_width).floor() as usize;
        let note_num = self.row_to_note(row)?;

        let beat_pos = (pos.x - content_rect.min.x) / self.beat_width;
        let snap_amount = self.division.beat_fraction();
        let snapped_beat = (beat_pos / snap_amount).floor() * snap_amount;

        if snapped_beat < 0.0 {
            return None;
        }

        Some(Note::new(
            note_num,
            f64::from(snapped_beat),
            self.default_note_duration,
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

    /// Check if a note is a black key
    const fn is_black_key(note: u8) -> bool {
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
