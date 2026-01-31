//! Piano Component
//!
//! Interactive piano keyboard with glassmorphic styling.
//! Designed for DAW piano rolls and music applications.
//!
//! Features:
//! - Optional scrollable viewport with momentum physics
//! - Smooth inertia-based scrolling for navigating octaves

use armas_basic::theme::Theme;
use egui::{self, Color32, CornerRadius, Pos2, Rect, Response, Sense, Vec2};
use std::collections::HashSet;

// ============================================================================
// Types and Constants
// ============================================================================

/// State for momentum scrolling (stored in egui temp data)
#[derive(Clone, Default)]
struct PianoScrollState {
    offset: f32,
    velocity: f32,
    last_frame_time: f64,
    is_animating: bool,
}

/// A single piano key identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PianoKey {
    /// MIDI note number (0-127)
    pub note: u8,
    /// Whether this is a black key
    pub is_black: bool,
}

impl PianoKey {
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

/// Piano keyboard orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PianoOrientation {
    /// Horizontal with keys extending upward
    Horizontal,
    /// Horizontal with keys extending downward
    HorizontalUp,
    /// Vertical with keys extending rightward
    Vertical,
    /// Vertical with keys extending leftward
    VerticalLeft,
}

/// Internal layout parameters computed from Piano config
struct PianoLayout {
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
    opacity: f32,
    glow_intensity: f32,
    corner_radius: CornerRadius,
    note: Option<(u8, bool)>, // (note, show_label)
}

// ============================================================================
// Piano Component
// ============================================================================

/// Piano keyboard component with glassmorphic styling
pub struct Piano {
    start_note: u8,
    octaves: u8,
    white_key_width: f32,
    white_key_height: f32,
    black_key_height_ratio: f32,
    black_key_width_ratio: f32,
    white_key_opacity: f32,
    black_key_opacity: f32,
    glow_intensity: f32,
    pressed_keys: HashSet<u8>,
    show_labels: bool,
    orientation: PianoOrientation,
    scrollable: bool,
    viewport_size: Option<f32>,
    momentum_scrolling: bool,
    momentum_damping: f64,
    id: Option<egui::Id>,
}

impl Piano {
    /// Create a new piano keyboard
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
            glow_intensity: 0.8,
            pressed_keys: HashSet::new(),
            show_labels: true,
            orientation: PianoOrientation::Horizontal,
            scrollable: false,
            viewport_size: None,
            momentum_scrolling: true,
            momentum_damping: 5.0,
            id: None,
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

    /// Set glow intensity for pressed keys (0.0-1.0, default: 0.8)
    #[must_use]
    pub const fn glow_intensity(mut self, intensity: f32) -> Self {
        self.glow_intensity = intensity.clamp(0.0, 1.0);
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
    pub const fn orientation(mut self, orientation: PianoOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set which keys are currently pressed
    #[must_use]
    pub fn pressed_keys(mut self, keys: HashSet<u8>) -> Self {
        self.pressed_keys = keys;
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

    /// Show the piano keyboard
    pub fn show(self, ui: &mut egui::Ui, theme: &Theme) -> PianoResponse {
        let mut clicked_keys = Vec::new();
        let mut released_keys = Vec::new();

        if !self.pressed_keys.is_empty() {
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

        PianoResponse {
            clicked_keys,
            released_keys,
        }
    }

    // ========================================================================
    // Layout Computation
    // ========================================================================

    fn compute_layout(&self) -> PianoLayout {
        let total_notes = self.octaves as usize * 12;
        let white_key_count = (0..total_notes)
            .filter(|i| !PianoKey::is_black_key((self.start_note + *i as u8) % 12))
            .count();

        let is_horizontal = matches!(
            self.orientation,
            PianoOrientation::Horizontal | PianoOrientation::HorizontalUp
        );

        let content_size = white_key_count as f32 * self.white_key_width;
        let display_size = if self.scrollable {
            self.viewport_size.unwrap_or(content_size).min(content_size)
        } else {
            content_size
        };

        let black_key_size = self.white_key_width * self.black_key_width_ratio;
        let black_key_depth = self.white_key_height * self.black_key_height_ratio;

        PianoLayout {
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

    fn handle_scrolling(&self, ui: &mut egui::Ui, layout: &PianoLayout) -> f32 {
        if !self.scrollable {
            return 0.0;
        }

        let scroll_state_id = self
            .id
            .unwrap_or_else(|| egui::Id::new("piano"))
            .with("scroll");
        let mut state: PianoScrollState = ui
            .ctx()
            .data(|d| d.get_temp(scroll_state_id).unwrap_or_default());

        let current_time = ui.ctx().input(|i| i.time);
        let dt = if state.last_frame_time > 0.0 {
            (current_time - state.last_frame_time) as f32
        } else {
            0.016
        };
        state.last_frame_time = current_time;

        // Sense area for scroll input
        let sense_size = if layout.is_horizontal {
            Vec2::new(layout.display_size, self.white_key_height)
        } else {
            Vec2::new(self.white_key_height, layout.display_size)
        };
        let (_, sense_response) = ui.allocate_exact_size(sense_size, Sense::hover());

        // Handle scroll wheel
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

        // Apply momentum physics
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

        // Clamp offset
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
        layout: &PianoLayout,
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

        let facing_up = matches!(self.orientation, PianoOrientation::HorizontalUp);
        let facing_left = matches!(self.orientation, PianoOrientation::VerticalLeft);

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
        );
    }

    fn render_white_keys(
        &self,
        ui: &mut egui::Ui,
        painter: &egui::Painter,
        theme: &Theme,
        layout: &PianoLayout,
        rect: Rect,
        scroll_offset: f32,
        facing_up: bool,
        facing_left: bool,
        clicked_keys: &mut Vec<u8>,
        released_keys: &mut Vec<u8>,
    ) {
        let mut white_key_index = 0;

        for i in 0..layout.total_notes {
            let note = self.start_note + i as u8;
            if PianoKey::is_black_key(note % 12) {
                continue;
            }

            let key_rect = self.compute_white_key_rect(
                layout,
                rect,
                scroll_offset,
                white_key_index,
                facing_left,
            );

            // Skip if outside viewport
            if self.scrollable && !key_rect.intersects(rect) {
                white_key_index += 1;
                continue;
            }

            let response = ui.allocate_rect(key_rect, Sense::click_and_drag());
            let is_pressed =
                self.pressed_keys.contains(&note) || response.is_pointer_button_down_on();

            self.draw_key(&KeyDrawParams {
                painter,
                theme,
                rect: key_rect,
                is_black: false,
                is_pressed,
                is_hovered: response.hovered(),
                opacity: self.white_key_opacity,
                glow_intensity: self.glow_intensity,
                corner_radius: self.white_key_corner_radius(
                    layout.is_horizontal,
                    facing_up,
                    facing_left,
                ),
                note: if self.show_labels {
                    Some((note, layout.is_horizontal))
                } else {
                    None
                },
            });

            self.handle_key_interaction(&response, note, clicked_keys, released_keys);
            white_key_index += 1;
        }
    }

    fn render_black_keys(
        &self,
        ui: &mut egui::Ui,
        painter: &egui::Painter,
        theme: &Theme,
        layout: &PianoLayout,
        rect: Rect,
        scroll_offset: f32,
        facing_up: bool,
        facing_left: bool,
        clicked_keys: &mut Vec<u8>,
        released_keys: &mut Vec<u8>,
    ) {
        let mut white_key_index = 0;

        for i in 0..layout.total_notes {
            let note = self.start_note + i as u8;
            let is_black = PianoKey::is_black_key(note % 12);

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

            // Skip if outside viewport
            if self.scrollable && !key_rect.intersects(rect) {
                continue;
            }

            let response = ui.allocate_rect(key_rect, Sense::click_and_drag());
            let is_pressed =
                self.pressed_keys.contains(&note) || response.is_pointer_button_down_on();

            self.draw_key(&KeyDrawParams {
                painter,
                theme,
                rect: key_rect,
                is_black: true,
                is_pressed,
                is_hovered: response.hovered(),
                opacity: self.black_key_opacity,
                glow_intensity: self.glow_intensity,
                corner_radius: self.black_key_corner_radius(
                    layout.is_horizontal,
                    facing_up,
                    facing_left,
                ),
                note: None,
            });

            self.handle_key_interaction(&response, note, clicked_keys, released_keys);
        }
    }

    // ========================================================================
    // Key Geometry
    // ========================================================================

    fn compute_white_key_rect(
        &self,
        layout: &PianoLayout,
        rect: Rect,
        scroll_offset: f32,
        white_key_index: usize,
        facing_left: bool,
    ) -> Rect {
        if layout.is_horizontal {
            let key_x =
                (white_key_index as f32).mul_add(self.white_key_width, rect.min.x + scroll_offset);
            Rect::from_min_size(
                Pos2::new(key_x, rect.min.y),
                Vec2::new(self.white_key_width, self.white_key_height),
            )
        } else {
            let key_y = ((white_key_index + 1) as f32)
                .mul_add(-self.white_key_width, rect.max.y - scroll_offset);
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
        layout: &PianoLayout,
        rect: Rect,
        scroll_offset: f32,
        white_key_index: usize,
        facing_up: bool,
        facing_left: bool,
    ) -> Rect {
        if layout.is_horizontal {
            let key_x = (white_key_index as f32)
                .mul_add(self.white_key_width, rect.min.x + scroll_offset)
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
            let key_y = (white_key_index as f32)
                .mul_add(-self.white_key_width, rect.max.y - scroll_offset)
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
        // Black keys use same corner radius logic as white keys
        self.white_key_corner_radius(is_horizontal, facing_up, facing_left)
    }

    // ========================================================================
    // Key Drawing
    // ========================================================================

    fn draw_key(&self, params: &KeyDrawParams) {
        let opacity = if params.is_pressed {
            params.opacity * 0.85
        } else if params.is_hovered {
            params.opacity * 0.9
        } else {
            params.opacity
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

        // Glow for pressed keys
        if params.is_pressed {
            self.draw_key_glow(
                params.painter,
                params.rect,
                params.corner_radius,
                params.theme.primary(),
                params.glow_intensity,
            );
        }

        // Shimmer
        self.draw_key_shimmer(params.painter, params.rect, params.is_black);

        // Note label (white keys only)
        if let Some((note, is_horizontal)) = params.note {
            self.draw_note_label(
                params.painter,
                params.theme,
                params.rect,
                note,
                is_horizontal,
            );
        }
    }

    fn draw_key_glow(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        corner_radius: CornerRadius,
        color: Color32,
        intensity: f32,
    ) {
        for i in 0..4 {
            let offset = (i + 1) as f32 * 1.5;
            let alpha = ((1.0 - i as f32 / 4.0) * 35.0 * intensity) as u8;
            let glow_color =
                Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);
            painter.rect_stroke(
                rect.expand(offset),
                corner_radius,
                egui::Stroke::new(1.5, glow_color),
                egui::StrokeKind::Outside,
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
        let key = PianoKey::new(note, false);
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

impl Default for Piano {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Response
// ============================================================================

/// Response from piano keyboard interaction
pub struct PianoResponse {
    /// MIDI note numbers that were clicked this frame
    pub clicked_keys: Vec<u8>,
    /// MIDI note numbers that were released this frame
    pub released_keys: Vec<u8>,
}

impl PianoResponse {
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
    fn test_piano_key_note_name() {
        let key = PianoKey::new(60, false);
        assert_eq!(key.note_name(), "C4");

        let key = PianoKey::new(61, true);
        assert_eq!(key.note_name(), "C#4");
    }

    #[test]
    fn test_is_black_key() {
        assert!(!PianoKey::is_black_key(0)); // C
        assert!(PianoKey::is_black_key(1)); // C#
        assert!(!PianoKey::is_black_key(2)); // D
        assert!(PianoKey::is_black_key(3)); // D#
        assert!(!PianoKey::is_black_key(4)); // E
        assert!(!PianoKey::is_black_key(5)); // F
        assert!(PianoKey::is_black_key(6)); // F#
    }

    #[test]
    fn test_piano_creation() {
        let piano = Piano::new();
        assert_eq!(piano.start_note, 60);
        assert_eq!(piano.octaves, 2);
        assert!(piano.show_labels);
    }

    #[test]
    fn test_piano_builder() {
        let piano = Piano::new()
            .start_note(48)
            .octaves(3)
            .white_key_width(50.0)
            .show_labels(false);

        assert_eq!(piano.start_note, 48);
        assert_eq!(piano.octaves, 3);
        assert_eq!(piano.white_key_width, 50.0);
        assert!(!piano.show_labels);
    }
}
