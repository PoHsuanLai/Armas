//! MIDI Controller Component
//!
//! Complete MIDI controller interface combining piano, wheels, pads, and sequencer.
//! Provides a comprehensive MIDI input surface for DAW applications.

use crate::{
    MidiPad, MidiPadResponse, ModWheel, PadColorScheme, PadConfig, PadState, Piano,
    PianoOrientation, PianoResponse, StepSequencer, WheelSize, WheelType, XYPad,
};
use armas_basic::components::cards::{Card, CardVariant};
use egui::{Response, ScrollArea, Ui};
use std::collections::{HashMap, HashSet};

/// Draw a section panel with a subtle background
fn section_panel(ui: &mut Ui, theme: &armas_basic::Theme, add_contents: impl FnOnce(&mut Ui)) {
    let corner_radius = f32::from(theme.spacing.corner_radius);
    egui::Frame::NONE
        .fill(theme.background().gamma_multiply(0.5))
        .corner_radius(corner_radius)
        .inner_margin(theme.spacing.md)
        .show(ui, |ui| {
            add_contents(ui);
        });
}

/// MIDI Controller state
#[derive(Debug, Clone)]
pub struct MidiControllerState {
    /// Modulation wheel value (0.0 to 1.0)
    pub mod_wheel: f32,
    /// Pitch bend wheel value (-1.0 to 1.0)
    pub pitch_wheel: f32,
    /// XY pad X value (0.0 to 1.0)
    pub xy_x: f32,
    /// XY pad Y value (0.0 to 1.0)
    pub xy_y: f32,
    /// Active piano keys (note -> velocity)
    pub active_notes: HashMap<u8, u8>,
    /// Drum pad states (note -> `PadState`)
    pub drum_pads: HashMap<u8, PadState>,
    /// Step sequencer pattern
    pub sequencer_steps: Vec<bool>,
}

impl Default for MidiControllerState {
    fn default() -> Self {
        Self {
            mod_wheel: 0.0,
            pitch_wheel: 0.0,
            xy_x: 0.5,
            xy_y: 0.5,
            active_notes: HashMap::new(),
            drum_pads: HashMap::new(),
            sequencer_steps: vec![false; 16],
        }
    }
}

/// Response from MIDI Controller interaction
pub struct MidiControllerResponse {
    /// Inner response
    pub response: Response,
    /// Piano response (if shown)
    pub piano: Option<PianoResponse>,
    /// Drum pad response (if shown)
    pub drum_pads: Option<MidiPadResponse>,
    /// Whether mod wheel changed
    pub mod_wheel_changed: bool,
    /// Whether pitch wheel changed
    pub pitch_wheel_changed: bool,
    /// Whether XY pad changed
    pub xy_pad_changed: bool,
    /// Whether sequencer pattern changed
    pub sequencer_changed: bool,
}

/// Complete MIDI Controller component
///
/// Combines multiple MIDI input components into a unified interface.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # use armas_basic::Theme;
/// # fn example(ui: &mut Ui, theme: &Theme) {
/// use armas_audio::{MidiController, MidiControllerState};
///
/// let mut state = MidiControllerState::default();
///
/// let response = MidiController::new(&mut state)
///     .show(ui, theme);
///
/// if let Some(piano_response) = response.piano {
///     for note in piano_response.clicked_keys {
///         println!("Note pressed: {}", note);
///     }
/// }
/// # }
/// ```
pub struct MidiController<'a> {
    state: &'a mut MidiControllerState,
    wheel_size: WheelSize,
    id: Option<egui::Id>,
}

impl<'a> MidiController<'a> {
    /// Create a new MIDI controller
    pub const fn new(state: &'a mut MidiControllerState) -> Self {
        Self {
            state,
            wheel_size: WheelSize::Default,
            id: None,
        }
    }

    /// Set unique ID for state persistence
    #[must_use]
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set size preset for wheels
    #[must_use]
    pub const fn wheel_size(mut self, size: WheelSize) -> Self {
        self.wheel_size = size;
        self
    }

    /// Show the MIDI controller
    pub fn show(self, ui: &mut Ui, theme: &armas_basic::Theme) -> MidiControllerResponse {
        let mut piano_response = None;
        let mut drum_pad_response = None;
        let mut mod_wheel_changed = false;
        let mut pitch_wheel_changed = false;
        let mut xy_pad_changed = false;
        let mut sequencer_changed = false;

        let base_id = self.id.unwrap_or_else(|| egui::Id::new("midi_controller"));
        let mod_wheel_id = base_id.with("mod_wheel");
        let pitch_wheel_id = base_id.with("pitch_wheel");
        let xy_pad_id = base_id.with("xy_pad");

        let card_response = Card::new()
            .variant(CardVariant::Filled)
            .show(ui, theme, |ui| {
                // Top controls section (wheels, XY pad, drum pads)
                section_panel(ui, theme, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = theme.spacing.lg;

                        // Wheels
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = theme.spacing.md;

                            let mod_response = ModWheel::new(&mut self.state.mod_wheel)
                                .wheel_type(WheelType::Modulation)
                                .size(self.wheel_size)
                                .label("Mod")
                                .id(mod_wheel_id)
                                .show(ui, theme);
                            mod_wheel_changed = mod_response.changed();

                            let pitch_response = ModWheel::new(&mut self.state.pitch_wheel)
                                .wheel_type(WheelType::PitchBend)
                                .size(self.wheel_size)
                                .label("Pitch")
                                .id(pitch_wheel_id)
                                .show(ui, theme);
                            pitch_wheel_changed = pitch_response.changed();
                        });

                        // XY Pad
                        let xy_response = XYPad::new(&mut self.state.xy_x, &mut self.state.xy_y)
                            .size(180.0)
                            .x_label("X")
                            .y_label("Y")
                            .id(xy_pad_id)
                            .show(ui, theme);
                        xy_pad_changed = xy_response.changed;

                        // Drum pads
                        ui.vertical(|ui| {
                            let pad_count = 4 * 4;
                            let mut pad_configs = Vec::new();

                            let drum_notes = [
                                36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
                            ];

                            for i in 0..pad_count {
                                let note = drum_notes.get(i).copied().unwrap_or(36 + i as u8);
                                pad_configs.push(PadConfig {
                                    note,
                                    label: Some(format!("{}", i + 1)),
                                    color: None,
                                });
                            }

                            let pad_response_inner = MidiPad::new()
                                .grid(4, 4)
                                .pads(pad_configs)
                                .pad_states(self.state.drum_pads.clone())
                                .color_scheme(PadColorScheme::Semantic)
                                .show(ui, theme);

                            if let Some((note, velocity)) = pad_response_inner.pressed {
                                self.state
                                    .drum_pads
                                    .insert(note, PadState { note, velocity });
                            }
                            if let Some(note) = pad_response_inner.released {
                                self.state.drum_pads.remove(&note);
                            }

                            drum_pad_response = Some(pad_response_inner);
                        });
                    });
                });

                ui.add_space(theme.spacing.sm);

                // Step sequencer section
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = theme.spacing.sm;
                    ui.add_space(theme.spacing.xs);

                    self.state.sequencer_steps.resize(16, false);

                    let seq_response = StepSequencer::new(&mut self.state.sequencer_steps)
                        .steps(16)
                        .step_size(theme.spacing.xl, theme.spacing.xl)
                        .gap(theme.spacing.xs)
                        .show_step_numbers(true)
                        .show(ui, theme);

                    sequencer_changed = seq_response.changed;
                });

                ui.add_space(theme.spacing.sm);

                // Piano keyboard section
                ScrollArea::horizontal()
                    .id_salt(base_id.with("piano_scroll"))
                    .show(ui, |ui| {
                        let start_note = ((3 + 2) * 12) as u8;
                        let pressed_keys: HashSet<u8> =
                            self.state.active_notes.keys().copied().collect();

                        let piano_response_inner = Piano::new()
                            .octaves(3)
                            .start_note(start_note)
                            .white_key_width(theme.spacing.lg + theme.spacing.xs)
                            .white_key_height(100.0)
                            .orientation(PianoOrientation::Horizontal)
                            .pressed_keys(pressed_keys)
                            .show(ui, theme);

                        for note in &piano_response_inner.clicked_keys {
                            self.state.active_notes.insert(*note, 100);
                        }
                        for note in &piano_response_inner.released_keys {
                            self.state.active_notes.remove(note);
                        }

                        piano_response = Some(piano_response_inner);
                    });
            });

        MidiControllerResponse {
            response: card_response.response,
            piano: piano_response,
            drum_pads: drum_pad_response,
            mod_wheel_changed,
            pitch_wheel_changed,
            xy_pad_changed,
            sequencer_changed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midi_controller_creation() {
        let mut state = MidiControllerState::default();
        let _controller = MidiController::new(&mut state);
        assert_eq!(state.mod_wheel, 0.0);
        assert_eq!(state.pitch_wheel, 0.0);
    }

    #[test]
    fn test_state_default() {
        let state = MidiControllerState::default();
        assert_eq!(state.mod_wheel, 0.0);
        assert_eq!(state.pitch_wheel, 0.0);
        assert_eq!(state.xy_x, 0.5);
        assert_eq!(state.xy_y, 0.5);
        assert_eq!(state.sequencer_steps.len(), 16);
    }
}
