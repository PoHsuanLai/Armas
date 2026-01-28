//! MPE Keyboard Component
//!
//! Interactive piano keyboard with MPE (MIDI Polyphonic Expression) support.
//! Visualizes per-note pitch bend, pressure (aftertouch), and slide using
//! floating circle indicators (JUCE-style visualization).
//!
//! Features:
//! - Per-note velocity visualization (inner circle size)
//! - Per-note pressure visualization (outer circle size)
//! - Per-note pitch bend visualization (circle X position)
//! - Per-note slide visualization (circle Y position)
//! - Same glassmorphic key styling as the standard Piano component
//! - Optional scrollable viewport with momentum physics

use armas::theme::Theme;
use egui::{self, Color32, CornerRadius, Pos2, Rect, Response, Sense, Vec2};
use std::collections::HashMap;

// ============================================================================
// Types and Constants
// ============================================================================

/// State for momentum scrolling (stored in egui temp data)
#[derive(Clone, Default)]
struct MPEScrollState {
    offset: f32,
    velocity: f32,
    last_frame_time: f64,
    is_animating: bool,
}

/// MPE note data with per-note expression parameters
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MPENote {
    /// MIDI note number (0-127)
    pub note: u8,
    /// Initial strike velocity (0.0-1.0)
    pub velocity: f32,
    /// Pressure/aftertouch (0.0-1.0)
    pub pressure: f32,
    /// Pitch bend in semitones (-48.0 to +48.0 typical, but can vary)
    pub pitch_bend: f32,
    /// Slide position (0.0-1.0, where 0.5 is center)
    pub slide: f32,
}

impl MPENote {
    /// Create a new MPE note with default expression values
    #[must_use]
    pub const fn new(note: u8) -> Self {
        Self {
            note,
            velocity: 0.8,
            pressure: 0.0,
            pitch_bend: 0.0,
            slide: 0.5,
        }
    }

    /// Create with specific velocity
    #[must_use]
    pub const fn with_velocity(note: u8, velocity: f32) -> Self {
        Self {
            note,
            velocity: velocity.clamp(0.0, 1.0),
            pressure: 0.0,
            pitch_bend: 0.0,
            slide: 0.5,
        }
    }

    /// Set pressure (aftertouch)
    #[must_use]
    pub const fn pressure(mut self, pressure: f32) -> Self {
        self.pressure = pressure.clamp(0.0, 1.0);
        self
    }

    /// Set pitch bend in semitones
    #[must_use]
    pub const fn pitch_bend(mut self, semitones: f32) -> Self {
        self.pitch_bend = semitones;
        self
    }

    /// Set slide position (0.0-1.0)
    #[must_use]
    pub const fn slide(mut self, slide: f32) -> Self {
        self.slide = slide.clamp(0.0, 1.0);
        self
    }
}

impl Default for MPENote {
    fn default() -> Self {
        Self::new(60)
    }
}

/// A single piano key identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MPEKey {
    /// MIDI note number (0-127)
    pub note: u8,
    /// Whether this is a black key
    pub is_black: bool,
}

impl MPEKey {
    /// Create a new piano key identifier
    #[must_use]
    pub const fn new(note: u8, is_black: bool) -> Self {
        Self { note, is_black }
    }

    /// Get the note name (e.g., "C4", "A#3")
    #[must_use]
    pub fn note_name(&self) -> String {
        const NOTE_NAMES: [&str; 12] = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        let octave = i32::from(self.note / 12) - 1;
        let note_index = (self.note % 12) as usize;
        format!("{}{}", NOTE_NAMES[note_index], octave)
    }

    /// Check if a MIDI note number is a black key
    #[must_use]
    pub const fn is_black_key(note: u8) -> bool {
        matches!(note % 12, 1 | 3 | 6 | 8 | 10)
    }
}

/// Keyboard orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MPEOrientation {
    /// Horizontal with keys extending upward
    Horizontal,
    /// Horizontal with keys extending downward
    HorizontalUp,
    /// Vertical with keys extending rightward
    Vertical,
    /// Vertical with keys extending leftward
    VerticalLeft,
}

/// Internal layout parameters
struct MPELayout {
    total_notes: usize,
    content_size: f32,
    display_size: f32,
    black_key_size: f32,
    black_key_depth: f32,
    is_horizontal: bool,
}

/// Parameters for drawing a single key
struct KeyDrawParams<'a> {
    painter: &'a egui::Painter,
    theme: &'a Theme,
    rect: Rect,
    is_black: bool,
    is_pressed: bool,
    is_hovered: bool,
    base_opacity: f32,
    corner_radius: CornerRadius,
    note_label: Option<(u8, bool)>,
}

// ============================================================================
// MPE Keyboard Component
// ============================================================================

/// MPE Keyboard component with per-note expression visualization
///
/// Uses JUCE-style circle indicators to show MPE expression:
/// - Inner filled circle size = velocity
/// - Outer circle outline size = pressure
/// - Circle X position = pitch bend
/// - Circle Y position = slide
pub struct MPEKeyboard {
    start_note: u8,
    octaves: u8,
    white_key_width: f32,
    white_key_height: f32,
    black_key_height_ratio: f32,
    black_key_width_ratio: f32,
    white_key_opacity: f32,
    black_key_opacity: f32,
    active_notes: HashMap<u8, MPENote>,
    show_labels: bool,
    orientation: MPEOrientation,
    scrollable: bool,
    viewport_size: Option<f32>,
    momentum_scrolling: bool,
    momentum_damping: f64,
    id: Option<egui::Id>,
    /// Pitch bend range in semitones for visualization scaling
    pitch_bend_range: f32,
    /// Circle fill color for velocity
    circle_fill_color: Option<Color32>,
    /// Circle outline color for pressure
    circle_outline_color: Option<Color32>,
    /// Minimum circle radius
    min_circle_radius: f32,
    /// Maximum circle radius scale (multiplied by key width)
    max_circle_radius_scale: f32,
}

impl MPEKeyboard {
    /// Create a new MPE keyboard
    #[must_use]
    pub fn new() -> Self {
        Self {
            start_note: 60,
            octaves: 2,
            white_key_width: 40.0,
            white_key_height: 120.0,
            black_key_height_ratio: 0.6,
            black_key_width_ratio: 0.6,
            white_key_opacity: 0.7,
            black_key_opacity: 0.85,
            active_notes: HashMap::new(),
            show_labels: true,
            orientation: MPEOrientation::Horizontal,
            scrollable: false,
            viewport_size: None,
            momentum_scrolling: true,
            momentum_damping: 5.0,
            id: None,
            pitch_bend_range: 48.0, // ±48 semitones (4 octaves) typical MPE range
            circle_fill_color: None,
            circle_outline_color: None,
            min_circle_radius: 5.0,
            max_circle_radius_scale: 0.4, // Max radius = 40% of key width
        }
    }

    // Builder methods
    /// Set the starting MIDI note number (default: 60, middle C)
    #[must_use]
    pub const fn start_note(mut self, note: u8) -> Self {
        self.start_note = note;
        self
    }

    /// Set the number of octaves to display (default: 2)
    #[must_use]
    pub const fn octaves(mut self, octaves: u8) -> Self {
        self.octaves = octaves;
        self
    }

    /// Set the width of white keys in pixels (default: 40.0)
    #[must_use]
    pub const fn white_key_width(mut self, width: f32) -> Self {
        self.white_key_width = width;
        self
    }

    /// Set the height of white keys in pixels (default: 120.0)
    #[must_use]
    pub const fn white_key_height(mut self, height: f32) -> Self {
        self.white_key_height = height;
        self
    }

    /// Set opacity for white keys (0.0-1.0, default: 0.7)
    #[must_use]
    pub const fn white_key_opacity(mut self, opacity: f32) -> Self {
        self.white_key_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set opacity for black keys (0.0-1.0, default: 0.85)
    #[must_use]
    pub const fn black_key_opacity(mut self, opacity: f32) -> Self {
        self.black_key_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Show or hide note labels on the keys (default: true)
    #[must_use]
    pub const fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// Set the keyboard orientation (default: Horizontal)
    #[must_use]
    pub const fn orientation(mut self, orientation: MPEOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set active MPE notes with their expression data
    #[must_use]
    pub fn active_notes(mut self, notes: HashMap<u8, MPENote>) -> Self {
        self.active_notes = notes;
        self
    }

    /// Add a single active MPE note
    #[must_use]
    pub fn with_note(mut self, note: MPENote) -> Self {
        self.active_notes.insert(note.note, note);
        self
    }

    /// Set unique ID for state persistence (required for scrollable keyboards)
    #[must_use]
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Enable scrollable viewport with specified size in pixels
    #[must_use]
    pub const fn scrollable(mut self, viewport_size: f32) -> Self {
        self.scrollable = true;
        self.viewport_size = Some(viewport_size);
        self
    }

    /// Enable or disable momentum scrolling (default: true)
    #[must_use]
    pub const fn momentum_scrolling(mut self, enabled: bool) -> Self {
        self.momentum_scrolling = enabled;
        self
    }

    /// Set momentum damping factor (1.0-20.0, higher = more damping, default: 5.0)
    #[must_use]
    pub const fn momentum_damping(mut self, damping: f64) -> Self {
        self.momentum_damping = damping.clamp(1.0, 20.0);
        self
    }

    /// Set pitch bend range in semitones for visualization scaling
    #[must_use]
    pub const fn pitch_bend_range(mut self, semitones: f32) -> Self {
        self.pitch_bend_range = semitones.max(1.0);
        self
    }

    /// Set custom fill color for velocity circles
    #[must_use]
    pub const fn circle_fill_color(mut self, color: Color32) -> Self {
        self.circle_fill_color = Some(color);
        self
    }

    /// Set custom outline color for pressure circles
    #[must_use]
    pub const fn circle_outline_color(mut self, color: Color32) -> Self {
        self.circle_outline_color = Some(color);
        self
    }

    /// Show the MPE keyboard
    pub fn show(self, ui: &mut egui::Ui, theme: &Theme) -> MPEKeyboardResponse {
        let mut clicked_keys = Vec::new();
        let mut released_keys = Vec::new();

        if !self.active_notes.is_empty() {
            ui.ctx().request_repaint();
        }

        let layout = self.compute_layout();
        let scroll_offset = self.handle_scrolling(ui, &layout);

        self.render_keys(
            ui,
            theme,
            &layout,
            scroll_offset,
            &mut clicked_keys,
            &mut released_keys,
        );

        MPEKeyboardResponse {
            clicked_keys,
            released_keys,
        }
    }

    // ========================================================================
    // Layout Computation
    // ========================================================================

    fn compute_layout(&self) -> MPELayout {
        let total_notes = self.octaves as usize * 12;
        let white_key_count = (0..total_notes)
            .filter(|i| !MPEKey::is_black_key((self.start_note + *i as u8) % 12))
            .count();

        let is_horizontal = matches!(
            self.orientation,
            MPEOrientation::Horizontal | MPEOrientation::HorizontalUp
        );

        let content_size = white_key_count as f32 * self.white_key_width;
        let display_size = if self.scrollable {
            self.viewport_size.unwrap_or(content_size).min(content_size)
        } else {
            content_size
        };

        let black_key_size = self.white_key_width * self.black_key_width_ratio;
        let black_key_depth = self.white_key_height * self.black_key_height_ratio;

        MPELayout {
            total_notes,
            content_size,
            display_size,
            black_key_size,
            black_key_depth,
            is_horizontal,
        }
    }

    // ========================================================================
    // Scrolling
    // ========================================================================

    fn handle_scrolling(&self, ui: &mut egui::Ui, layout: &MPELayout) -> f32 {
        if !self.scrollable {
            return 0.0;
        }

        let scroll_state_id = self
            .id
            .unwrap_or_else(|| egui::Id::new("mpe_keyboard"))
            .with("scroll");
        let mut state: MPEScrollState = ui
            .ctx()
            .data(|d| d.get_temp(scroll_state_id).unwrap_or_default());

        let current_time = ui.ctx().input(|i| i.time);
        let dt = if state.last_frame_time > 0.0 {
            (current_time - state.last_frame_time) as f32
        } else {
            0.016
        };
        state.last_frame_time = current_time;

        let sense_size = if layout.is_horizontal {
            Vec2::new(layout.display_size, self.white_key_height)
        } else {
            Vec2::new(self.white_key_height, layout.display_size)
        };
        let (_, sense_response) = ui.allocate_exact_size(sense_size, Sense::hover());

        if sense_response.hovered() {
            let scroll_delta = ui.ctx().input(|i| i.raw_scroll_delta);
            let amount = if layout.is_horizontal {
                scroll_delta.x
            } else {
                scroll_delta.y
            };
            if amount.abs() > 0.0 {
                if self.momentum_scrolling {
                    state.velocity += amount * 3.0;
                    state.is_animating = true;
                } else {
                    state.offset += amount;
                }
            }
        }

        if self.momentum_scrolling && state.is_animating {
            state.offset += state.velocity * dt;
            state.velocity *= (-self.momentum_damping * f64::from(dt)).exp() as f32;

            if state.velocity.abs() < 1.0 {
                state.velocity = 0.0;
                state.is_animating = false;
            } else {
                ui.ctx().request_repaint();
            }
        }

        let max_scroll = (layout.content_size - layout.display_size).max(0.0);
        state.offset = state.offset.clamp(-max_scroll, 0.0);

        ui.ctx()
            .data_mut(|d| d.insert_temp(scroll_state_id, state.clone()));
        state.offset
    }

    // ========================================================================
    // Key Rendering
    // ========================================================================

    fn render_keys(
        &self,
        ui: &mut egui::Ui,
        theme: &Theme,
        layout: &MPELayout,
        scroll_offset: f32,
        clicked_keys: &mut Vec<u8>,
        released_keys: &mut Vec<u8>,
    ) {
        let alloc_size = if layout.is_horizontal {
            Vec2::new(layout.display_size, self.white_key_height)
        } else {
            Vec2::new(self.white_key_height, layout.display_size)
        };

        let (rect, _) = ui.allocate_exact_size(alloc_size, Sense::hover());

        if !ui.is_rect_visible(rect) {
            return;
        }

        let painter = if self.scrollable {
            ui.painter().with_clip_rect(rect)
        } else {
            ui.painter().clone()
        };

        let facing_up = matches!(self.orientation, MPEOrientation::HorizontalUp);
        let facing_left = matches!(self.orientation, MPEOrientation::VerticalLeft);

        // Collect key rects for MPE circle drawing later
        let mut key_rects: HashMap<u8, Rect> = HashMap::new();

        // Draw white keys first, then black keys on top
        self.render_white_keys(
            ui,
            &painter,
            theme,
            layout,
            rect,
            scroll_offset,
            facing_up,
            facing_left,
            clicked_keys,
            released_keys,
            &mut key_rects,
        );

        self.render_black_keys(
            ui,
            &painter,
            theme,
            layout,
            rect,
            scroll_offset,
            facing_up,
            facing_left,
            clicked_keys,
            released_keys,
            &mut key_rects,
        );

        // Draw MPE circles on top of all keys
        self.render_mpe_circles(&painter, theme, layout, &key_rects);
    }

    #[allow(clippy::too_many_arguments)]
    fn render_white_keys(
        &self,
        ui: &mut egui::Ui,
        painter: &egui::Painter,
        theme: &Theme,
        layout: &MPELayout,
        rect: Rect,
        scroll_offset: f32,
        facing_up: bool,
        facing_left: bool,
        clicked_keys: &mut Vec<u8>,
        released_keys: &mut Vec<u8>,
        key_rects: &mut HashMap<u8, Rect>,
    ) {
        let mut white_key_index = 0;

        for i in 0..layout.total_notes {
            let note = self.start_note + i as u8;
            if MPEKey::is_black_key(note % 12) {
                continue;
            }

            let key_rect = self.compute_white_key_rect(
                layout,
                rect,
                scroll_offset,
                white_key_index,
                facing_left,
            );

            if self.scrollable && !key_rect.intersects(rect) {
                white_key_index += 1;
                continue;
            }

            // Store rect for MPE circle drawing
            key_rects.insert(note, key_rect);

            let response = ui.allocate_rect(key_rect, Sense::click_and_drag());
            let is_active = self.active_notes.contains_key(&note);
            let is_pressed = is_active || response.is_pointer_button_down_on();

            self.draw_key(&KeyDrawParams {
                painter,
                theme,
                rect: key_rect,
                is_black: false,
                is_pressed,
                is_hovered: response.hovered() && !is_pressed,
                base_opacity: self.white_key_opacity,
                corner_radius: self.white_key_corner_radius(
                    layout.is_horizontal,
                    facing_up,
                    facing_left,
                ),
                note_label: if self.show_labels {
                    Some((note, layout.is_horizontal))
                } else {
                    None
                },
            });

            self.handle_key_interaction(&response, note, clicked_keys, released_keys);
            white_key_index += 1;
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_black_keys(
        &self,
        ui: &mut egui::Ui,
        painter: &egui::Painter,
        theme: &Theme,
        layout: &MPELayout,
        rect: Rect,
        scroll_offset: f32,
        facing_up: bool,
        facing_left: bool,
        clicked_keys: &mut Vec<u8>,
        released_keys: &mut Vec<u8>,
        key_rects: &mut HashMap<u8, Rect>,
    ) {
        let mut white_key_index = 0;

        for i in 0..layout.total_notes {
            let note = self.start_note + i as u8;
            let is_black = MPEKey::is_black_key(note % 12);

            if !is_black {
                white_key_index += 1;
                continue;
            }

            let key_rect = self.compute_black_key_rect(
                layout,
                rect,
                scroll_offset,
                white_key_index,
                facing_up,
                facing_left,
            );

            if self.scrollable && !key_rect.intersects(rect) {
                continue;
            }

            // Store rect for MPE circle drawing
            key_rects.insert(note, key_rect);

            let response = ui.allocate_rect(key_rect, Sense::click_and_drag());
            let is_active = self.active_notes.contains_key(&note);
            let is_pressed = is_active || response.is_pointer_button_down_on();

            self.draw_key(&KeyDrawParams {
                painter,
                theme,
                rect: key_rect,
                is_black: true,
                is_pressed,
                is_hovered: response.hovered() && !is_pressed,
                base_opacity: self.black_key_opacity,
                corner_radius: self.black_key_corner_radius(
                    layout.is_horizontal,
                    facing_up,
                    facing_left,
                ),
                note_label: None,
            });

            self.handle_key_interaction(&response, note, clicked_keys, released_keys);
        }
    }

    /// Render MPE circles on top of active keys (JUCE-style visualization)
    fn render_mpe_circles(
        &self,
        painter: &egui::Painter,
        theme: &Theme,
        layout: &MPELayout,
        key_rects: &HashMap<u8, Rect>,
    ) {
        let fill_color = self.circle_fill_color.unwrap_or_else(|| theme.primary());
        let outline_color = self.circle_outline_color.unwrap_or_else(|| theme.secondary());
        let max_radius = self.white_key_width * self.max_circle_radius_scale;

        for (note, mpe_note) in &self.active_notes {
            // Find the base key rect - handle pitch bend by looking at nearby keys
            let base_note = *note;
            let Some(base_rect) = key_rects.get(&base_note) else {
                continue;
            };

            // Calculate circle position based on pitch bend and slide
            let circle_center =
                self.calculate_circle_position(mpe_note, base_rect, layout, key_rects);

            // Calculate circle radii based on velocity and pressure
            let velocity_radius = mpe_note.velocity.mul_add(max_radius, self.min_circle_radius);
            let pressure_radius = mpe_note.velocity.mul_add(max_radius, self.min_circle_radius)
                + mpe_note.pressure * max_radius * 0.5;

            // Draw outer circle (pressure) - outline only
            painter.circle_stroke(
                circle_center,
                pressure_radius,
                egui::Stroke::new(2.0, outline_color),
            );

            // Draw inner circle (velocity) - filled
            painter.circle_filled(circle_center, velocity_radius, fill_color);

            // Draw a subtle glow around the circle
            for i in 0..3 {
                let glow_radius = ((i + 1) as f32).mul_add(2.0, pressure_radius);
                let alpha = ((1.0 - i as f32 / 3.0) * 30.0) as u8;
                let glow_color = Color32::from_rgba_unmultiplied(
                    fill_color.r(),
                    fill_color.g(),
                    fill_color.b(),
                    alpha,
                );
                painter.circle_stroke(
                    circle_center,
                    glow_radius,
                    egui::Stroke::new(1.0, glow_color),
                );
            }
        }
    }

    /// Calculate the position of the MPE circle based on pitch bend and slide
    fn calculate_circle_position(
        &self,
        mpe_note: &MPENote,
        base_rect: &Rect,
        layout: &MPELayout,
        _key_rects: &HashMap<u8, Rect>,
    ) -> Pos2 {
        // Pitch bend moves the circle horizontally (or vertically for vertical keyboards)
        // Normalized to key width, so ±1 semitone = full key width movement
        let pitch_bend_offset = (mpe_note.pitch_bend / 12.0) * self.white_key_width;

        // Slide moves the circle perpendicular to pitch (Y for horizontal, X for vertical)
        // slide 0.0 = bottom/left edge, 1.0 = top/right edge
        let slide_normalized = mpe_note.slide; // 0.0 to 1.0

        if layout.is_horizontal {
            // Horizontal keyboard: X = pitch bend, Y = slide
            let x = base_rect.center().x + pitch_bend_offset;
            let y = slide_normalized.mul_add(-base_rect.height(), base_rect.max.y);
            Pos2::new(x, y)
        } else {
            // Vertical keyboard: Y = pitch bend, X = slide
            let y = base_rect.center().y - pitch_bend_offset; // Inverted for vertical
            let x = slide_normalized.mul_add(base_rect.width(), base_rect.min.x);
            Pos2::new(x, y)
        }
    }

    // ========================================================================
    // Key Geometry
    // ========================================================================

    fn compute_white_key_rect(
        &self,
        layout: &MPELayout,
        rect: Rect,
        scroll_offset: f32,
        white_key_index: usize,
        facing_left: bool,
    ) -> Rect {
        if layout.is_horizontal {
            let key_x = (white_key_index as f32).mul_add(self.white_key_width, rect.min.x + scroll_offset);
            Rect::from_min_size(
                Pos2::new(key_x, rect.min.y),
                Vec2::new(self.white_key_width, self.white_key_height),
            )
        } else {
            let key_y =
                ((white_key_index + 1) as f32).mul_add(-self.white_key_width, rect.max.y - scroll_offset);
            let key_x = if facing_left {
                rect.max.x - self.white_key_height
            } else {
                rect.min.x
            };
            Rect::from_min_size(
                Pos2::new(key_x, key_y),
                Vec2::new(self.white_key_height, self.white_key_width),
            )
        }
    }

    fn compute_black_key_rect(
        &self,
        layout: &MPELayout,
        rect: Rect,
        scroll_offset: f32,
        white_key_index: usize,
        facing_up: bool,
        facing_left: bool,
    ) -> Rect {
        if layout.is_horizontal {
            let key_x = (white_key_index as f32).mul_add(self.white_key_width, rect.min.x + scroll_offset)
                - layout.black_key_size * 0.5;
            let key_y = if facing_up {
                rect.max.y - layout.black_key_depth
            } else {
                rect.min.y
            };
            Rect::from_min_size(
                Pos2::new(key_x, key_y),
                Vec2::new(layout.black_key_size, layout.black_key_depth),
            )
        } else {
            let key_y = (white_key_index as f32).mul_add(-self.white_key_width, rect.max.y - scroll_offset)
                - layout.black_key_size * 0.5;
            let key_x = if facing_left {
                rect.max.x - layout.black_key_depth
            } else {
                rect.min.x
            };
            Rect::from_min_size(
                Pos2::new(key_x, key_y),
                Vec2::new(layout.black_key_depth, layout.black_key_size),
            )
        }
    }

    const fn white_key_corner_radius(
        &self,
        is_horizontal: bool,
        facing_up: bool,
        facing_left: bool,
    ) -> CornerRadius {
        if is_horizontal {
            if facing_up {
                CornerRadius {
                    nw: 4,
                    ne: 4,
                    sw: 0,
                    se: 0,
                }
            } else {
                CornerRadius {
                    nw: 0,
                    ne: 0,
                    sw: 4,
                    se: 4,
                }
            }
        } else if facing_left {
            CornerRadius {
                nw: 4,
                ne: 0,
                sw: 4,
                se: 0,
            }
        } else {
            CornerRadius {
                nw: 0,
                ne: 4,
                sw: 0,
                se: 4,
            }
        }
    }

    const fn black_key_corner_radius(
        &self,
        is_horizontal: bool,
        facing_up: bool,
        facing_left: bool,
    ) -> CornerRadius {
        self.white_key_corner_radius(is_horizontal, facing_up, facing_left)
    }

    // ========================================================================
    // Key Drawing
    // ========================================================================

    fn draw_key(&self, params: &KeyDrawParams) {
        // Calculate opacity based on state
        let opacity = if params.is_pressed {
            params.base_opacity * 0.85
        } else if params.is_hovered {
            params.base_opacity * 0.95
        } else {
            params.base_opacity
        };

        let base_color = if params.is_black { 20 } else { 255 };
        let glass_color = Color32::from_rgba_unmultiplied(
            base_color,
            base_color,
            base_color,
            (255.0 * opacity) as u8,
        );

        // For black keys, draw an opaque background first to prevent white key lines showing through
        if params.is_black {
            params.painter.rect_filled(
                params.rect,
                params.corner_radius,
                Color32::from_rgb(base_color, base_color, base_color),
            );
        }

        // Background (glass effect)
        params
            .painter
            .rect_filled(params.rect, params.corner_radius, glass_color);

        // Border
        let border_color = if params.is_pressed {
            params.theme.primary()
        } else {
            params.theme.border()
        };
        params.painter.rect_stroke(
            params.rect,
            params.corner_radius,
            egui::Stroke::new(1.0, border_color),
            egui::StrokeKind::Outside,
        );

        // Shimmer
        self.draw_key_shimmer(params.painter, params.rect, params.is_black);

        // Note label (white keys only)
        if let Some((note, is_horizontal)) = params.note_label {
            self.draw_note_label(
                params.painter,
                params.theme,
                params.rect,
                note,
                is_horizontal,
            );
        }
    }

    fn draw_key_shimmer(&self, painter: &egui::Painter, rect: Rect, is_black: bool) {
        let (shimmer_alpha, shimmer_size) = if is_black { (15, 1.5) } else { (30, 2.0) };
        let shimmer_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), shimmer_size));
        painter.rect_filled(
            shimmer_rect,
            CornerRadius::ZERO,
            Color32::from_rgba_unmultiplied(255, 255, 255, shimmer_alpha),
        );
    }

    fn draw_note_label(
        &self,
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        note: u8,
        is_horizontal: bool,
    ) {
        let key = MPEKey::new(note, false);
        let label_pos = if is_horizontal {
            Pos2::new(rect.center().x, rect.max.y - theme.spacing.md)
        } else {
            Pos2::new(rect.max.x - theme.spacing.md, rect.center().y)
        };
        painter.text(
            label_pos,
            egui::Align2::CENTER_CENTER,
            key.note_name(),
            egui::FontId::proportional(10.0),
            Color32::from_rgb(60, 60, 60),
        );
    }

    fn handle_key_interaction(
        &self,
        response: &Response,
        note: u8,
        clicked_keys: &mut Vec<u8>,
        released_keys: &mut Vec<u8>,
    ) {
        if response.clicked() {
            clicked_keys.push(note);
        }
        if response.drag_stopped() {
            released_keys.push(note);
        }
    }
}

impl Default for MPEKeyboard {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Response
// ============================================================================

/// Response from MPE keyboard interaction
pub struct MPEKeyboardResponse {
    /// MIDI note numbers that were clicked this frame
    pub clicked_keys: Vec<u8>,
    /// MIDI note numbers that were released this frame
    pub released_keys: Vec<u8>,
}

impl MPEKeyboardResponse {
    /// Check if any keys were clicked this frame
    #[must_use]
    pub const fn has_clicks(&self) -> bool {
        !self.clicked_keys.is_empty()
    }

    /// Check if any keys were released this frame
    #[must_use]
    pub const fn has_releases(&self) -> bool {
        !self.released_keys.is_empty()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mpe_note_creation() {
        let note = MPENote::new(60);
        assert_eq!(note.note, 60);
        assert_eq!(note.velocity, 0.8);
        assert_eq!(note.pressure, 0.0);
        assert_eq!(note.pitch_bend, 0.0);
        assert_eq!(note.slide, 0.5);
    }

    #[test]
    fn test_mpe_note_builder() {
        let note = MPENote::with_velocity(64, 0.9)
            .pressure(0.5)
            .pitch_bend(2.0)
            .slide(0.75);

        assert_eq!(note.note, 64);
        assert_eq!(note.velocity, 0.9);
        assert_eq!(note.pressure, 0.5);
        assert_eq!(note.pitch_bend, 2.0);
        assert_eq!(note.slide, 0.75);
    }

    #[test]
    fn test_mpe_key_note_name() {
        let key = MPEKey::new(60, false);
        assert_eq!(key.note_name(), "C4");

        let key = MPEKey::new(61, true);
        assert_eq!(key.note_name(), "C#4");
    }

    #[test]
    fn test_is_black_key() {
        assert!(!MPEKey::is_black_key(0)); // C
        assert!(MPEKey::is_black_key(1)); // C#
        assert!(!MPEKey::is_black_key(2)); // D
        assert!(MPEKey::is_black_key(3)); // D#
        assert!(!MPEKey::is_black_key(4)); // E
        assert!(!MPEKey::is_black_key(5)); // F
        assert!(MPEKey::is_black_key(6)); // F#
    }

    #[test]
    fn test_mpe_keyboard_creation() {
        let keyboard = MPEKeyboard::new();
        assert_eq!(keyboard.start_note, 60);
        assert_eq!(keyboard.octaves, 2);
        assert!(keyboard.show_labels);
        assert_eq!(keyboard.pitch_bend_range, 48.0);
    }

    #[test]
    fn test_mpe_keyboard_builder() {
        let keyboard = MPEKeyboard::new()
            .start_note(48)
            .octaves(3)
            .white_key_width(50.0)
            .show_labels(false)
            .pitch_bend_range(24.0);

        assert_eq!(keyboard.start_note, 48);
        assert_eq!(keyboard.octaves, 3);
        assert_eq!(keyboard.white_key_width, 50.0);
        assert!(!keyboard.show_labels);
        assert_eq!(keyboard.pitch_bend_range, 24.0);
    }

    #[test]
    fn test_mpe_keyboard_with_notes() {
        let mut notes = HashMap::new();
        notes.insert(60, MPENote::new(60).pressure(0.5));
        notes.insert(64, MPENote::new(64).pitch_bend(2.0));

        let keyboard = MPEKeyboard::new().active_notes(notes);
        assert_eq!(keyboard.active_notes.len(), 2);
        assert!(keyboard.active_notes.contains_key(&60));
        assert!(keyboard.active_notes.contains_key(&64));
    }
}
