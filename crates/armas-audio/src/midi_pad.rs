//! MIDI Pad Component
//!
//! Grid-based drum pad controller with velocity-sensitive visual feedback.
//! Perfect for drum machines, samplers, and MPC-style controllers.

use armas::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Visual style variant for MIDI pads
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PadVariant {
    /// Filled pads with solid backgrounds
    Filled,
    /// Outlined pads with transparent backgrounds
    Outlined,
    /// Elevated pads with shadow effect
    Elevated,
}

/// Color scheme for pad grid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PadColorScheme {
    /// Use theme semantic colors (error, warning, success, info, primary, secondary)
    Semantic,
    /// Use single primary color for all pads
    Monochrome,
    /// Use custom colors (set via `pad_colors`)
    Custom,
}

/// Configuration for a single pad in the grid
#[derive(Debug, Clone)]
pub struct PadConfig {
    /// MIDI note number (0-127)
    pub note: u8,
    /// Optional label text
    pub label: Option<String>,
    /// Optional custom color (overrides color scheme)
    pub color: Option<Color32>,
}

impl PadConfig {
    /// Create a new pad configuration
    #[must_use]
    pub fn new(note: u8) -> Self {
        Self {
            note,
            label: None,
            color: None,
        }
    }

    /// Set label text
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set custom color
    #[must_use]
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
}

/// State for a pad with velocity information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PadState {
    /// MIDI note number
    pub note: u8,
    /// Current velocity (0-127), 0 means not pressed
    pub velocity: u8,
}

impl PadState {
    /// Create a new pad state
    #[must_use]
    pub fn new(note: u8, velocity: u8) -> Self {
        Self {
            note,
            velocity: velocity.min(127),
        }
    }

    /// Check if pad is pressed (velocity > 0)
    #[must_use]
    pub fn is_pressed(&self) -> bool {
        self.velocity > 0
    }
}

/// MIDI Pad grid component
///
/// A grid of velocity-sensitive drum pads inspired by hardware controllers
/// like Akai MPC and Native Instruments Maschine.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::components::audio::{MidiPad, PadConfig, PadState, PadVariant};
/// use std::collections::HashMap;
///
/// // Define pad configuration (labels, colors, etc.)
/// let pads = vec![
///     PadConfig::new(36).label("Kick"),
///     PadConfig::new(38).label("Snare"),
///     PadConfig::new(42).label("HH"),
///     PadConfig::new(46).label("Tom"),
/// ];
///
/// // Track pad states (which pads are pressed and their velocities)
/// let mut pad_states = HashMap::new();
/// pad_states.insert(36, PadState::new(36, 100)); // Kick pressed at velocity 100
///
/// let response = MidiPad::new()
///     .grid(2, 2)
///     .pads(pads)
///     .pad_states(pad_states)
///     .variant(PadVariant::Filled)
///     .show(ui);
///
/// // Handle user interaction
/// if let Some((note, velocity)) = response.pressed {
///     println!("Pad pressed: note {}, velocity {}", note, velocity);
/// }
/// if let Some(note) = response.released {
///     println!("Pad released: note {}", note);
/// }
/// # }
/// ```
pub struct MidiPad {
    /// Number of rows
    rows: usize,
    /// Number of columns
    cols: usize,
    /// Pad configurations (row-major order)
    pads: Vec<PadConfig>,
    /// Current pad states (note -> state mapping)
    pad_states: std::collections::HashMap<u8, PadState>,
    /// Visual variant
    variant: PadVariant,
    /// Color scheme
    color_scheme: PadColorScheme,
    /// Pad size (width and height)
    pad_size: f32,
    /// Gap between pads
    gap: f32,
    /// Glow intensity for pressed pads (0.0-1.0)
    glow_intensity: f32,
    /// Show velocity as brightness
    show_velocity: bool,
}

impl MidiPad {
    /// Create a new MIDI pad grid with default 4x4 layout
    #[must_use]
    pub fn new() -> Self {
        Self {
            rows: 4,
            cols: 4,
            pads: Vec::new(),
            pad_states: std::collections::HashMap::new(),
            variant: PadVariant::Filled,
            color_scheme: PadColorScheme::Semantic,
            pad_size: 60.0,
            gap: 8.0,
            glow_intensity: 0.8,
            show_velocity: true,
        }
    }

    /// Set grid dimensions (rows x cols)
    #[must_use]
    pub fn grid(mut self, rows: usize, cols: usize) -> Self {
        self.rows = rows.max(1);
        self.cols = cols.max(1);
        self
    }

    /// Set pad configurations
    #[must_use]
    pub fn pads(mut self, pads: Vec<PadConfig>) -> Self {
        self.pads = pads;
        self
    }

    /// Set pad states (which pads are pressed and their velocities)
    #[must_use]
    pub fn pad_states(mut self, states: std::collections::HashMap<u8, PadState>) -> Self {
        self.pad_states = states;
        self
    }

    /// Set visual variant
    #[must_use]
    pub fn variant(mut self, variant: PadVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set color scheme
    #[must_use]
    pub fn color_scheme(mut self, scheme: PadColorScheme) -> Self {
        self.color_scheme = scheme;
        self
    }

    /// Set pad size (both width and height)
    #[must_use]
    pub fn pad_size(mut self, size: f32) -> Self {
        self.pad_size = size.max(20.0);
        self
    }

    /// Set gap between pads
    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    /// Set glow intensity (0.0-1.0)
    #[must_use]
    pub fn glow_intensity(mut self, intensity: f32) -> Self {
        self.glow_intensity = intensity.clamp(0.0, 1.0);
        self
    }

    /// Show velocity as brightness
    #[must_use]
    pub fn show_velocity(mut self, show: bool) -> Self {
        self.show_velocity = show;
        self
    }

    /// Show the MIDI pad grid
    pub fn show(self, ui: &mut Ui, theme: &armas::Theme) -> MidiPadResponse {
        // Calculate total size
        let total_width = self.cols as f32 * self.pad_size + (self.cols - 1) as f32 * self.gap;
        let total_height = self.rows as f32 * self.pad_size + (self.rows - 1) as f32 * self.gap;
        let desired_size = Vec2::new(total_width, total_height);

        let (rect, _) = ui.allocate_exact_size(desired_size, Sense::hover());

        let mut response = MidiPadResponse {
            pressed: None,
            released: None,
            held: Vec::new(),
        };

        if ui.is_rect_visible(rect) {
            // Draw each pad
            for row in 0..self.rows {
                for col in 0..self.cols {
                    let pad_index = row * self.cols + col;

                    // Get pad config or create default
                    let pad_config = self
                        .pads
                        .get(pad_index)
                        .cloned()
                        .unwrap_or_else(|| PadConfig::new(pad_index as u8));

                    // Get current pad state
                    let velocity = self
                        .pad_states
                        .get(&pad_config.note)
                        .map_or(0, |s| s.velocity);

                    // Calculate pad rect
                    let pad_x = rect.min.x + col as f32 * (self.pad_size + self.gap);
                    let pad_y = rect.min.y + row as f32 * (self.pad_size + self.gap);
                    let pad_rect =
                        Rect::from_min_size(Pos2::new(pad_x, pad_y), Vec2::splat(self.pad_size));

                    // Draw the pad and handle interaction
                    let pad_response =
                        self.draw_pad(ui, theme, pad_rect, &pad_config, pad_index, velocity);

                    // Handle pad interaction
                    if pad_response.clicked() {
                        // Calculate velocity based on click position (simple version)
                        let new_velocity = if self.show_velocity {
                            // Could be enhanced with click force or Y position
                            100
                        } else {
                            127
                        };
                        response.pressed = Some((pad_config.note, new_velocity));
                    }

                    if pad_response.drag_stopped() {
                        response.released = Some(pad_config.note);
                    }

                    if pad_response.is_pointer_button_down_on() {
                        response.held.push(pad_config.note);
                    }
                }
            }
        }

        response
    }

    /// Draw a single pad
    fn draw_pad(
        &self,
        ui: &mut Ui,
        theme: &Theme,
        rect: Rect,
        config: &PadConfig,
        index: usize,
        velocity: u8,
    ) -> Response {
        let pad_response = ui.allocate_rect(rect, Sense::click_and_drag());
        let painter = ui.painter();

        let is_pressed = velocity > 0 || pad_response.is_pointer_button_down_on();
        let is_hovered = pad_response.hovered();

        // Get pad color based on scheme
        let base_color = self.get_pad_color(theme, config, index);

        // Calculate corner radius
        let corner_radius = theme.spacing.corner_radius_small as f32;

        // Draw based on variant
        match self.variant {
            PadVariant::Filled => {
                self.draw_filled_pad(
                    painter,
                    theme,
                    rect,
                    base_color,
                    corner_radius,
                    is_pressed,
                    is_hovered,
                    velocity,
                );
            }
            PadVariant::Outlined => {
                self.draw_outlined_pad(
                    painter,
                    theme,
                    rect,
                    base_color,
                    corner_radius,
                    is_pressed,
                    is_hovered,
                    velocity,
                );
            }
            PadVariant::Elevated => {
                self.draw_elevated_pad(
                    painter,
                    theme,
                    rect,
                    base_color,
                    corner_radius,
                    is_pressed,
                    is_hovered,
                    velocity,
                );
            }
        }

        // Draw label if present
        if let Some(label) = &config.label {
            let label_text = if label.len() > 6 {
                format!("{}…", &label[..5])
            } else {
                label.clone()
            };

            let text_color = if is_pressed {
                theme.foreground()
            } else {
                theme.muted_foreground()
            };

            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                label_text,
                egui::FontId::proportional(10.0),
                text_color,
            );
        }

        pad_response
    }

    /// Draw filled variant pad
    fn draw_filled_pad(
        &self,
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        base_color: Color32,
        corner_radius: f32,
        is_pressed: bool,
        is_hovered: bool,
        velocity: u8,
    ) {
        // Calculate color based on state and velocity
        let mut fill_color = base_color;

        if is_pressed && self.show_velocity {
            // Brighten based on velocity (0-127)
            let velocity_factor = 1.0 + (velocity as f32 / 127.0) * 0.8;
            fill_color = fill_color.gamma_multiply(velocity_factor);
        } else if is_hovered {
            fill_color = fill_color.gamma_multiply(1.2);
        }

        // Draw background
        painter.rect_filled(rect, corner_radius, fill_color);

        // Draw border
        let border_color = if is_pressed {
            theme.primary()
        } else {
            theme.border()
        };

        painter.rect_stroke(
            rect,
            corner_radius,
            egui::Stroke::new(1.0, border_color),
            egui::StrokeKind::Outside,
        );

        // Draw glow effect for pressed pads
        if is_pressed {
            self.draw_glow_effect(painter, rect, corner_radius, theme.primary());
        }
    }

    /// Draw outlined variant pad
    fn draw_outlined_pad(
        &self,
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        base_color: Color32,
        corner_radius: f32,
        is_pressed: bool,
        is_hovered: bool,
        velocity: u8,
    ) {
        // Background is surface color
        let bg_color = if is_pressed && self.show_velocity {
            // Show color with velocity-based alpha
            let alpha = (64.0 + (velocity as f32 / 127.0) * 191.0) as u8;
            Color32::from_rgba_unmultiplied(base_color.r(), base_color.g(), base_color.b(), alpha)
        } else if is_hovered {
            theme.muted()
        } else {
            theme.card()
        };

        painter.rect_filled(rect, corner_radius, bg_color);

        // Draw border (thicker for this variant)
        let border_color = if is_pressed {
            base_color
        } else {
            theme.border()
        };

        let border_width = if is_pressed { 2.0 } else { 1.5 };
        painter.rect_stroke(
            rect,
            corner_radius,
            egui::Stroke::new(border_width, border_color),
            egui::StrokeKind::Outside,
        );

        // Draw glow effect for pressed pads
        if is_pressed {
            self.draw_glow_effect(painter, rect, corner_radius, base_color);
        }
    }

    /// Draw elevated variant pad
    fn draw_elevated_pad(
        &self,
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        base_color: Color32,
        corner_radius: f32,
        is_pressed: bool,
        is_hovered: bool,
        velocity: u8,
    ) {
        // Draw shadow layers (if not pressed)
        if !is_pressed {
            for i in 0..3 {
                let offset = (i + 1) as f32 * 0.5;
                let shadow_rect = rect.translate(Vec2::new(0.0, offset));
                let alpha = (20.0 - i as f32 * 5.0) as u8;
                let shadow_color = Color32::from_rgba_unmultiplied(0, 0, 0, alpha);
                painter.rect_filled(shadow_rect, corner_radius, shadow_color);
            }
        }

        // Draw main pad
        let mut fill_color = base_color;

        if is_pressed && self.show_velocity {
            let velocity_factor = 1.0 + (velocity as f32 / 127.0) * 0.8;
            fill_color = fill_color.gamma_multiply(velocity_factor);
        } else if is_hovered {
            fill_color = fill_color.gamma_multiply(1.15);
        }

        painter.rect_filled(rect, corner_radius, fill_color);

        // Draw subtle border
        let border_color = if is_pressed {
            theme.primary()
        } else {
            theme.border()
        };

        painter.rect_stroke(
            rect,
            corner_radius,
            egui::Stroke::new(1.0, border_color),
            egui::StrokeKind::Outside,
        );

        // Draw glow effect for pressed pads
        if is_pressed {
            self.draw_glow_effect(painter, rect, corner_radius, theme.primary());
        }
    }

    /// Draw multi-layer glow effect (inspired by Knob component)
    fn draw_glow_effect(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        corner_radius: f32,
        glow_color: Color32,
    ) {
        // 4-layer glow effect
        for i in 0..4 {
            let offset = (i + 1) as f32 * 2.0;
            let alpha = ((1.0 - i as f32 / 4.0) * 50.0 * self.glow_intensity) as u8;
            let layer_color = Color32::from_rgba_unmultiplied(
                glow_color.r(),
                glow_color.g(),
                glow_color.b(),
                alpha,
            );
            painter.rect_stroke(
                rect.expand(offset),
                corner_radius,
                egui::Stroke::new(2.0, layer_color),
                egui::StrokeKind::Outside,
            );
        }
    }

    /// Get pad color based on color scheme
    fn get_pad_color(&self, theme: &Theme, config: &PadConfig, index: usize) -> Color32 {
        // Custom color takes precedence
        if let Some(color) = config.color {
            return color;
        }

        match self.color_scheme {
            PadColorScheme::Custom => {
                // Fallback to primary if no custom color set
                theme.primary()
            }
            PadColorScheme::Monochrome => theme.primary(),
            PadColorScheme::Semantic => {
                // Cycle through semantic colors
                let colors = [
                    theme.destructive(),
                    theme.chart_3(),
                    theme.chart_2(),
                    theme.chart_4(),
                    theme.primary(),
                    theme.secondary(),
                ];
                colors[index % colors.len()]
            }
        }
    }
}

impl Default for MidiPad {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from MIDI pad interaction
#[derive(Debug)]
pub struct MidiPadResponse {
    /// Pad that was pressed this frame (note, velocity)
    pub pressed: Option<(u8, u8)>,
    /// Pad that was released this frame (note)
    pub released: Option<u8>,
    /// Pads currently being held (notes)
    pub held: Vec<u8>,
}

impl MidiPadResponse {
    /// Check if any pad was pressed
    #[must_use]
    pub fn has_press(&self) -> bool {
        self.pressed.is_some()
    }

    /// Check if any pad was released
    #[must_use]
    pub fn has_release(&self) -> bool {
        self.released.is_some()
    }

    /// Check if any pads are being held
    #[must_use]
    pub fn has_held(&self) -> bool {
        !self.held.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_config_creation() {
        let pad = PadConfig::new(36).label("Kick");
        assert_eq!(pad.note, 36);
        assert_eq!(pad.label, Some("Kick".to_string()));
    }

    #[test]
    fn test_pad_state() {
        let state = PadState::new(36, 100);
        assert_eq!(state.note, 36);
        assert_eq!(state.velocity, 100);
        assert!(state.is_pressed());

        let state_off = PadState::new(36, 0);
        assert!(!state_off.is_pressed());
    }

    #[test]
    fn test_midi_pad_builder() {
        let midi_pad = MidiPad::new().grid(4, 4).pad_size(50.0).gap(10.0);

        assert_eq!(midi_pad.rows, 4);
        assert_eq!(midi_pad.cols, 4);
        assert_eq!(midi_pad.pad_size, 50.0);
        assert_eq!(midi_pad.gap, 10.0);
    }

    #[test]
    fn test_velocity_clamping() {
        let state = PadState::new(36, 200);
        assert_eq!(state.velocity, 127); // Should be clamped to max
    }
}
