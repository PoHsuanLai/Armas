//! Piano Component
//!
//! Interactive piano keyboard with glassmorphic styling.
//! Perfect for DAW piano rolls and music applications.

use crate::theme::Theme;
use egui::{self, Color32, CornerRadius, Pos2, Rect, Response, Sense, Vec2};
use std::collections::HashSet;

/// A single piano key identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PianoKey {
    /// MIDI note number (0-127)
    pub note: u8,
    /// Whether this is a black key
    pub is_black: bool,
}

impl PianoKey {
    /// Create a new piano key
    pub fn new(note: u8, is_black: bool) -> Self {
        Self { note, is_black }
    }

    /// Get the note name (e.g., "C4", "C#4")
    pub fn note_name(&self) -> String {
        const NOTE_NAMES: [&str; 12] = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        let octave = (self.note / 12) as i32 - 1;
        let note_index = (self.note % 12) as usize;
        format!("{}{}", NOTE_NAMES[note_index], octave)
    }

    /// Check if this note is a black key based on note number
    pub fn is_black_key(note: u8) -> bool {
        matches!(note % 12, 1 | 3 | 6 | 8 | 10) // C#, D#, F#, G#, A#
    }
}

/// Piano orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PianoOrientation {
    /// Horizontal layout (default) - keys go left to right, facing down
    Horizontal,
    /// Horizontal layout - keys go left to right, facing up
    HorizontalUp,
    /// Vertical layout - keys go bottom to top, facing right
    Vertical,
    /// Vertical layout - keys go bottom to top, facing left
    VerticalLeft,
}

/// Piano keyboard component with glassmorphic styling
pub struct Piano {
    /// Starting MIDI note (default: 60 = C4)
    start_note: u8,
    /// Number of octaves to display (default: 2)
    octaves: u8,
    /// Width of each white key in pixels (default: 40.0)
    white_key_width: f32,
    /// Height of white keys (default: 120.0)
    white_key_height: f32,
    /// Height of black keys as ratio of white keys (default: 0.6)
    black_key_height_ratio: f32,
    /// Width of black keys as ratio of white keys (default: 0.6)
    black_key_width_ratio: f32,
    /// Glass opacity for white keys (default: 0.7)
    white_key_opacity: f32,
    /// Glass opacity for black keys (default: 0.85)
    black_key_opacity: f32,
    /// Glow intensity for pressed keys (default: 0.8)
    glow_intensity: f32,
    /// Currently pressed keys
    pressed_keys: HashSet<u8>,
    /// Show note labels on keys
    show_labels: bool,
    /// Piano orientation
    orientation: PianoOrientation,
}

impl Piano {
    /// Create a new piano starting at middle C (C4 = MIDI 60)
    pub fn new() -> Self {
        Self {
            start_note: 60, // C4
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
        }
    }

    /// Set the starting MIDI note
    pub fn start_note(mut self, note: u8) -> Self {
        self.start_note = note;
        self
    }

    /// Set number of octaves to display
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

    /// Set opacity for white keys (0.0-1.0)
    pub fn white_key_opacity(mut self, opacity: f32) -> Self {
        self.white_key_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set opacity for black keys (0.0-1.0)
    pub fn black_key_opacity(mut self, opacity: f32) -> Self {
        self.black_key_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set glow intensity for pressed keys (0.0-1.0)
    pub fn glow_intensity(mut self, intensity: f32) -> Self {
        self.glow_intensity = intensity.clamp(0.0, 1.0);
        self
    }

    /// Set whether to show note labels
    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// Set piano orientation (horizontal or vertical)
    pub fn orientation(mut self, orientation: PianoOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set pressed keys
    pub fn pressed_keys(mut self, keys: HashSet<u8>) -> Self {
        self.pressed_keys = keys;
        self
    }

    /// Show the piano keyboard
    pub fn show(self, ui: &mut egui::Ui, theme: &Theme) -> PianoResponse {
        let mut clicked_keys = Vec::new();
        let mut released_keys = Vec::new();

        // Request repaint if any keys are pressed (to show glow effect)
        if !self.pressed_keys.is_empty() {
            ui.ctx().request_repaint();
        }

        match self.orientation {
            PianoOrientation::Horizontal => {
                self.show_horizontal(ui, theme, &mut clicked_keys, &mut released_keys, false)
            }
            PianoOrientation::HorizontalUp => {
                self.show_horizontal(ui, theme, &mut clicked_keys, &mut released_keys, true)
            }
            PianoOrientation::Vertical => {
                self.show_vertical(ui, theme, &mut clicked_keys, &mut released_keys, false)
            }
            PianoOrientation::VerticalLeft => {
                self.show_vertical(ui, theme, &mut clicked_keys, &mut released_keys, true)
            }
        }

        PianoResponse {
            clicked_keys,
            released_keys,
        }
    }

    /// Show horizontal piano layout
    fn show_horizontal(
        &self,
        ui: &mut egui::Ui,
        theme: &Theme,
        clicked_keys: &mut Vec<u8>,
        released_keys: &mut Vec<u8>,
        facing_up: bool,
    ) {
        // Calculate total keys (12 notes per octave)
        let total_notes = self.octaves as usize * 12;

        // Count white keys for layout
        let white_key_count = (0..total_notes)
            .filter(|i| !PianoKey::is_black_key((self.start_note + *i as u8) % 12))
            .count();

        // Calculate total size
        let total_width = white_key_count as f32 * self.white_key_width;
        let total_height = self.white_key_height;
        let black_key_height = self.white_key_height * self.black_key_height_ratio;
        let black_key_width = self.white_key_width * self.black_key_width_ratio;

        // Allocate space
        let (rect, _) =
            ui.allocate_exact_size(Vec2::new(total_width, total_height), Sense::hover());

        if ui.is_rect_visible(rect) {
            // Draw white keys first
            let mut white_key_index = 0;
            for i in 0..total_notes {
                let note = self.start_note + i as u8;
                let is_black = PianoKey::is_black_key(note % 12);

                if !is_black {
                    let key_x = rect.min.x + white_key_index as f32 * self.white_key_width;
                    let key_rect = Rect::from_min_size(
                        Pos2::new(key_x, rect.min.y),
                        Vec2::new(self.white_key_width, self.white_key_height),
                    );

                    let is_pressed = self.pressed_keys.contains(&note);
                    let key_response =
                        self.draw_white_key(ui, theme, key_rect, note, is_pressed, facing_up);

                    // Handle interaction
                    if key_response.clicked() {
                        clicked_keys.push(note);
                    }
                    if key_response.drag_stopped() {
                        released_keys.push(note);
                    }

                    white_key_index += 1;
                }
            }

            // Draw black keys on top
            white_key_index = 0;
            for i in 0..total_notes {
                let note = self.start_note + i as u8;
                let is_black = PianoKey::is_black_key(note % 12);

                // Update white key index for positioning black keys
                if !is_black {
                    white_key_index += 1;
                } else {
                    // Position black key between white keys
                    let key_x = rect.min.x + white_key_index as f32 * self.white_key_width
                        - black_key_width * 0.5;
                    let key_y = if facing_up {
                        rect.max.y - black_key_height // Position at top for facing up
                    } else {
                        rect.min.y // Position at bottom for facing down
                    };
                    let key_rect = Rect::from_min_size(
                        Pos2::new(key_x, key_y),
                        Vec2::new(black_key_width, black_key_height),
                    );

                    let is_pressed = self.pressed_keys.contains(&note);
                    let key_response =
                        self.draw_black_key(ui, theme, key_rect, note, is_pressed, facing_up);

                    // Handle interaction
                    if key_response.clicked() {
                        clicked_keys.push(note);
                    }
                    if key_response.drag_stopped() {
                        released_keys.push(note);
                    }
                }
            }
        }
    }

    /// Show vertical piano layout (rotated 90 degrees - keys go bottom to top)
    fn show_vertical(
        &self,
        ui: &mut egui::Ui,
        theme: &Theme,
        clicked_keys: &mut Vec<u8>,
        released_keys: &mut Vec<u8>,
        facing_left: bool,
    ) {
        // Calculate total keys (12 notes per octave)
        let total_notes = self.octaves as usize * 12;

        // Count white keys for layout
        let white_key_count = (0..total_notes)
            .filter(|i| !PianoKey::is_black_key((self.start_note + *i as u8) % 12))
            .count();

        // In vertical mode, width and height are swapped
        let total_width = self.white_key_height; // Width is now the key height
        let total_height = white_key_count as f32 * self.white_key_width; // Height is now key count * key width
        let black_key_width = self.white_key_height * self.black_key_height_ratio;
        let black_key_height = self.white_key_width * self.black_key_width_ratio;

        // Allocate space
        let (rect, _) =
            ui.allocate_exact_size(Vec2::new(total_width, total_height), Sense::hover());

        if ui.is_rect_visible(rect) {
            // Draw white keys first (bottom to top)
            let mut white_key_index = 0;
            for i in 0..total_notes {
                let note = self.start_note + i as u8;
                let is_black = PianoKey::is_black_key(note % 12);

                if !is_black {
                    // Position from bottom (max.y) going up (subtracting)
                    let key_y = rect.max.y - (white_key_index + 1) as f32 * self.white_key_width;
                    let key_rect = Rect::from_min_size(
                        Pos2::new(rect.min.x, key_y),
                        Vec2::new(self.white_key_height, self.white_key_width),
                    );

                    let is_pressed = self.pressed_keys.contains(&note);
                    let key_response = self.draw_white_key_vertical(
                        ui,
                        theme,
                        key_rect,
                        note,
                        is_pressed,
                        facing_left,
                    );

                    // Handle interaction
                    if key_response.clicked() {
                        clicked_keys.push(note);
                    }
                    if key_response.drag_stopped() {
                        released_keys.push(note);
                    }

                    white_key_index += 1;
                }
            }

            // Draw black keys on top
            white_key_index = 0;
            for i in 0..total_notes {
                let note = self.start_note + i as u8;
                let is_black = PianoKey::is_black_key(note % 12);

                // Update white key index for positioning black keys
                if !is_black {
                    white_key_index += 1;
                } else {
                    // Position black key between white keys
                    let key_y = rect.max.y
                        - white_key_index as f32 * self.white_key_width
                        - black_key_height * 0.5;
                    let key_x = if facing_left {
                        rect.max.x - black_key_width // Position at left for facing left
                    } else {
                        rect.min.x // Position at right for facing right
                    };
                    let key_rect = Rect::from_min_size(
                        Pos2::new(key_x, key_y),
                        Vec2::new(black_key_width, black_key_height),
                    );

                    let is_pressed = self.pressed_keys.contains(&note);
                    let key_response = self.draw_black_key_vertical(
                        ui,
                        theme,
                        key_rect,
                        note,
                        is_pressed,
                        facing_left,
                    );

                    // Handle interaction
                    if key_response.clicked() {
                        clicked_keys.push(note);
                    }
                    if key_response.drag_stopped() {
                        released_keys.push(note);
                    }
                }
            }
        }
    }

    /// Draw a white piano key with glass effect
    fn draw_white_key(
        &self,
        ui: &mut egui::Ui,
        theme: &Theme,
        rect: Rect,
        note: u8,
        is_pressed: bool,
        facing_up: bool,
    ) -> Response {
        let response = ui.allocate_rect(rect, Sense::click_and_drag());
        let painter = ui.painter();

        // Check if actively being clicked (provides instant feedback)
        let is_actively_pressed = is_pressed || response.is_pointer_button_down_on();

        // Glass effect colors - pure white with transparency
        let opacity = if is_actively_pressed {
            self.white_key_opacity * 0.85 // Slightly more transparent when pressed
        } else if response.hovered() {
            self.white_key_opacity * 0.9
        } else {
            self.white_key_opacity
        };

        // Pure white with glass transparency
        let glass_color = Color32::from_rgba_unmultiplied(
            255, // Pure white
            255,
            255,
            (255.0 * opacity) as u8,
        );

        let corner_radius = if facing_up {
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
        };

        // Draw key background
        painter.rect_filled(rect, corner_radius, glass_color);

        // Draw border
        let border_color = if is_actively_pressed {
            theme.primary()
        } else if response.hovered() {
            theme.border()
        } else {
            theme.border()
        };
        painter.rect_stroke(
            rect,
            corner_radius,
            egui::Stroke::new(1.0, border_color),
            egui::StrokeKind::Outside,
        );

        // Draw subtle glow for pressed keys
        if is_actively_pressed {
            let glow_color = theme.primary();
            for i in 0..4 {
                let offset = (i + 1) as f32 * 1.5;
                let alpha = ((1.0 - i as f32 / 4.0) * 35.0 * self.glow_intensity) as u8;
                let layer_color = Color32::from_rgba_unmultiplied(
                    glow_color.r(),
                    glow_color.g(),
                    glow_color.b(),
                    alpha,
                );
                painter.rect_stroke(
                    rect.expand(offset),
                    corner_radius,
                    egui::Stroke::new(1.5, layer_color),
                    egui::StrokeKind::Outside,
                );
            }
        }

        // Draw shimmer on top edge
        let shimmer_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), 2.0));
        painter.rect_filled(
            shimmer_rect,
            CornerRadius::ZERO,
            Color32::from_rgba_unmultiplied(255, 255, 255, 30),
        );

        // Draw note label if enabled
        if self.show_labels {
            let key = PianoKey::new(note, false);
            let label_pos = if facing_up {
                Pos2::new(
                    rect.center().x,
                    rect.min.y + theme.spacing.md, // Top for facing up
                )
            } else {
                Pos2::new(
                    rect.center().x,
                    rect.max.y - theme.spacing.md, // Bottom for facing down
                )
            };
            painter.text(
                label_pos,
                egui::Align2::CENTER_CENTER,
                key.note_name(),
                egui::FontId::proportional(10.0),
                Color32::from_rgb(60, 60, 60), // Dark gray text on white keys
            );
        }

        response
    }

    /// Draw a black piano key with glass effect
    fn draw_black_key(
        &self,
        ui: &mut egui::Ui,
        theme: &Theme,
        rect: Rect,
        _note: u8,
        is_pressed: bool,
        facing_up: bool,
    ) -> Response {
        let response = ui.allocate_rect(rect, Sense::click_and_drag());
        let painter = ui.painter();

        // Check if actively being clicked (provides instant feedback)
        let is_actively_pressed = is_pressed || response.is_pointer_button_down_on();

        // Glass effect colors - pure black with transparency
        let opacity = if is_actively_pressed {
            self.black_key_opacity * 0.85
        } else if response.hovered() {
            self.black_key_opacity * 0.9
        } else {
            self.black_key_opacity
        };

        // Pure black with glass transparency
        let glass_color = Color32::from_rgba_unmultiplied(
            20, // Very dark, not quite pure black for some depth
            20,
            20,
            (255.0 * opacity) as u8,
        );

        let corner_radius = if facing_up {
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
        };

        // Draw key background
        painter.rect_filled(rect, corner_radius, glass_color);

        // Draw border
        let border_color = if is_actively_pressed {
            theme.primary()
        } else if response.hovered() {
            theme.border()
        } else {
            theme.border()
        };
        painter.rect_stroke(
            rect,
            corner_radius,
            egui::Stroke::new(1.0, border_color),
            egui::StrokeKind::Outside,
        );

        // Draw subtle glow for pressed keys
        if is_actively_pressed {
            let glow_color = theme.primary();
            for i in 0..4 {
                let offset = (i + 1) as f32 * 1.5;
                let alpha = ((1.0 - i as f32 / 4.0) * 35.0 * self.glow_intensity) as u8;
                let layer_color = Color32::from_rgba_unmultiplied(
                    glow_color.r(),
                    glow_color.g(),
                    glow_color.b(),
                    alpha,
                );
                painter.rect_stroke(
                    rect.expand(offset),
                    corner_radius,
                    egui::Stroke::new(1.5, layer_color),
                    egui::StrokeKind::Outside,
                );
            }
        }

        // Draw subtle shimmer on top edge
        let shimmer_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), 1.5));
        painter.rect_filled(
            shimmer_rect,
            CornerRadius::ZERO,
            Color32::from_rgba_unmultiplied(255, 255, 255, 15),
        );

        response
    }

    /// Draw a white piano key with glass effect (vertical orientation)
    fn draw_white_key_vertical(
        &self,
        ui: &mut egui::Ui,
        theme: &Theme,
        rect: Rect,
        note: u8,
        is_pressed: bool,
        facing_left: bool,
    ) -> Response {
        let response = ui.allocate_rect(rect, Sense::click_and_drag());
        let painter = ui.painter();

        // Check if actively being clicked (provides instant feedback)
        let is_actively_pressed = is_pressed || response.is_pointer_button_down_on();

        // Glass effect colors - pure white with transparency
        let opacity = if is_actively_pressed {
            self.white_key_opacity * 0.85
        } else if response.hovered() {
            self.white_key_opacity * 0.9
        } else {
            self.white_key_opacity
        };

        // Pure white with glass transparency
        let glass_color = Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * opacity) as u8);

        let corner_radius = if facing_left {
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
        };

        // Draw key background
        painter.rect_filled(rect, corner_radius, glass_color);

        // Draw border
        let border_color = if is_actively_pressed {
            theme.primary()
        } else if response.hovered() {
            theme.border()
        } else {
            theme.border()
        };
        painter.rect_stroke(
            rect,
            corner_radius,
            egui::Stroke::new(1.0, border_color),
            egui::StrokeKind::Outside,
        );

        // Draw subtle glow for pressed keys
        if is_actively_pressed {
            let glow_color = theme.primary();
            for i in 0..4 {
                let offset = (i + 1) as f32 * 1.5;
                let alpha = ((1.0 - i as f32 / 4.0) * 35.0 * self.glow_intensity) as u8;
                let layer_color = Color32::from_rgba_unmultiplied(
                    glow_color.r(),
                    glow_color.g(),
                    glow_color.b(),
                    alpha,
                );
                painter.rect_stroke(
                    rect.expand(offset),
                    corner_radius,
                    egui::Stroke::new(1.5, layer_color),
                    egui::StrokeKind::Outside,
                );
            }
        }

        // Draw shimmer on right edge
        let shimmer_rect = Rect::from_min_size(
            Pos2::new(rect.max.x - 2.0, rect.min.y),
            Vec2::new(2.0, rect.height()),
        );
        painter.rect_filled(
            shimmer_rect,
            CornerRadius::ZERO,
            Color32::from_rgba_unmultiplied(255, 255, 255, 30),
        );

        // Draw note label if enabled (rotated position)
        if self.show_labels {
            let key = PianoKey::new(note, false);
            let label_pos = if facing_left {
                Pos2::new(
                    rect.min.x + theme.spacing.md, // Left side for facing left
                    rect.center().y,
                )
            } else {
                Pos2::new(
                    rect.max.x - theme.spacing.md, // Right side for facing right
                    rect.center().y,
                )
            };
            painter.text(
                label_pos,
                egui::Align2::CENTER_CENTER,
                key.note_name(),
                egui::FontId::proportional(10.0),
                Color32::from_rgb(60, 60, 60),
            );
        }

        response
    }

    /// Draw a black piano key with glass effect (vertical orientation)
    fn draw_black_key_vertical(
        &self,
        ui: &mut egui::Ui,
        theme: &Theme,
        rect: Rect,
        _note: u8,
        is_pressed: bool,
        facing_left: bool,
    ) -> Response {
        let response = ui.allocate_rect(rect, Sense::click_and_drag());
        let painter = ui.painter();

        // Check if actively being clicked (provides instant feedback)
        let is_actively_pressed = is_pressed || response.is_pointer_button_down_on();

        // Glass effect colors - pure black with transparency
        let opacity = if is_actively_pressed {
            self.black_key_opacity * 0.85
        } else if response.hovered() {
            self.black_key_opacity * 0.9
        } else {
            self.black_key_opacity
        };

        // Pure black with glass transparency
        let glass_color = Color32::from_rgba_unmultiplied(20, 20, 20, (255.0 * opacity) as u8);

        let corner_radius = if facing_left {
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
        };

        // Draw key background
        painter.rect_filled(rect, corner_radius, glass_color);

        // Draw border
        let border_color = if is_actively_pressed {
            theme.primary()
        } else if response.hovered() {
            theme.border()
        } else {
            theme.border()
        };
        painter.rect_stroke(
            rect,
            corner_radius,
            egui::Stroke::new(1.0, border_color),
            egui::StrokeKind::Outside,
        );

        // Draw subtle glow for pressed keys
        if is_actively_pressed {
            let glow_color = theme.primary();
            for i in 0..4 {
                let offset = (i + 1) as f32 * 1.5;
                let alpha = ((1.0 - i as f32 / 4.0) * 35.0 * self.glow_intensity) as u8;
                let layer_color = Color32::from_rgba_unmultiplied(
                    glow_color.r(),
                    glow_color.g(),
                    glow_color.b(),
                    alpha,
                );
                painter.rect_stroke(
                    rect.expand(offset),
                    corner_radius,
                    egui::Stroke::new(1.5, layer_color),
                    egui::StrokeKind::Outside,
                );
            }
        }

        // Draw subtle shimmer on right edge
        let shimmer_rect = Rect::from_min_size(
            Pos2::new(rect.max.x - 1.5, rect.min.y),
            Vec2::new(1.5, rect.height()),
        );
        painter.rect_filled(
            shimmer_rect,
            CornerRadius::ZERO,
            Color32::from_rgba_unmultiplied(255, 255, 255, 15),
        );

        response
    }
}

impl Default for Piano {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from the piano component
pub struct PianoResponse {
    /// Keys that were clicked this frame
    pub clicked_keys: Vec<u8>,
    /// Keys that were released this frame
    pub released_keys: Vec<u8>,
}

impl PianoResponse {
    /// Check if any keys were clicked
    pub fn has_clicks(&self) -> bool {
        !self.clicked_keys.is_empty()
    }

    /// Check if any keys were released
    pub fn has_releases(&self) -> bool {
        !self.released_keys.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piano_key_note_name() {
        let key = PianoKey::new(60, false); // Middle C
        assert_eq!(key.note_name(), "C4");

        let key = PianoKey::new(61, true); // C#4
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
        assert_eq!(piano.start_note, 60); // Middle C
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
