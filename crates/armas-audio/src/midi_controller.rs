//! MIDI Controller Component
//!
//! Complete MIDI controller interface combining piano, wheels, pads, and sequencer.
//! Provides a comprehensive MIDI input surface for DAW applications.

use crate::{
    MidiPad, MidiPadResponse, ModWheel, PadColorScheme, PadConfig, PadState, PadVariant, Piano,
    PianoOrientation, PianoResponse, StepSequencer, WheelType, WheelVariant,
    XYPad, XYPadVariant,
};
use armas::components::cards::{Card, CardVariant};
use armas::ext::ArmasContextExt;
use egui::{Response, Ui};
use std::collections::{HashMap, HashSet};

/// MIDI Controller layout variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerLayout {
    /// Full controller with all sections
    Full,
    /// Compact layout with essential controls
    Compact,
    /// Performance layout optimized for live use
    Performance,
}

/// MIDI Controller section visibility
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControllerSections {
    /// Show piano keyboard
    pub show_piano: bool,
    /// Show mod and pitch wheels
    pub show_wheels: bool,
    /// Show XY pad
    pub show_xy_pad: bool,
    /// Show drum pads
    pub show_drum_pads: bool,
    /// Show step sequencer
    pub show_sequencer: bool,
}

impl Default for ControllerSections {
    fn default() -> Self {
        Self {
            show_piano: true,
            show_wheels: true,
            show_xy_pad: true,
            show_drum_pads: true,
            show_sequencer: true,
        }
    }
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
    /// Drum pad states (note -> PadState)
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
/// # fn example(ui: &mut Ui) {
/// use armas::components::audio::{MidiController, MidiControllerState};
///
/// let mut state = MidiControllerState::default();
///
/// let response = MidiController::new(&mut state)
///     .layout(armas::components::audio::ControllerLayout::Full)
///     .show(ui);
///
/// // Handle MIDI events
/// if let Some(piano_response) = response.piano {
///     for note in piano_response.notes_pressed {
///         println!("Note pressed: {}", note);
///     }
/// }
/// # }
/// ```
pub struct MidiController<'a> {
    state: &'a mut MidiControllerState,
    layout: ControllerLayout,
    sections: ControllerSections,
    piano_octaves: u8,
    piano_start_octave: i32,
    drum_pad_rows: usize,
    drum_pad_cols: usize,
    sequencer_steps: usize,
    wheel_variant: WheelVariant,
    pad_variant: PadVariant,
}

impl<'a> MidiController<'a> {
    /// Create a new MIDI controller
    pub fn new(state: &'a mut MidiControllerState) -> Self {
        Self {
            state,
            layout: ControllerLayout::Full,
            sections: ControllerSections::default(),
            piano_octaves: 3,
            piano_start_octave: 3,
            drum_pad_rows: 4,
            drum_pad_cols: 4,
            sequencer_steps: 16,
            wheel_variant: WheelVariant::Filled,
            pad_variant: PadVariant::Filled,
        }
    }

    /// Set controller layout
    pub fn layout(mut self, layout: ControllerLayout) -> Self {
        self.layout = layout;
        // Adjust sections based on layout
        self.sections = match layout {
            ControllerLayout::Full => ControllerSections::default(),
            ControllerLayout::Compact => ControllerSections {
                show_piano: true,
                show_wheels: true,
                show_xy_pad: false,
                show_drum_pads: false,
                show_sequencer: false,
            },
            ControllerLayout::Performance => ControllerSections {
                show_piano: true,
                show_wheels: true,
                show_xy_pad: true,
                show_drum_pads: true,
                show_sequencer: false,
            },
        };
        self
    }

    /// Set visible sections
    pub fn sections(mut self, sections: ControllerSections) -> Self {
        self.sections = sections;
        self
    }

    /// Set piano configuration
    pub fn piano(mut self, octaves: u8, start_octave: i32) -> Self {
        self.piano_octaves = octaves;
        self.piano_start_octave = start_octave;
        self
    }

    /// Set drum pad grid size
    pub fn drum_pads(mut self, rows: usize, cols: usize) -> Self {
        self.drum_pad_rows = rows;
        self.drum_pad_cols = cols;
        self
    }

    /// Set sequencer step count
    pub fn sequencer_steps(mut self, steps: usize) -> Self {
        self.sequencer_steps = steps;
        self
    }

    /// Set visual variant for wheels
    pub fn wheel_variant(mut self, variant: WheelVariant) -> Self {
        self.wheel_variant = variant;
        self
    }

    /// Set visual variant for pads
    pub fn pad_variant(mut self, variant: PadVariant) -> Self {
        self.pad_variant = variant;
        self
    }

    /// Show the MIDI controller
    pub fn show(self, ui: &mut Ui) -> MidiControllerResponse {
        let theme = ui.ctx().armas_theme();

        let mut piano_response = None;
        let mut drum_pad_response = None;
        let mut mod_wheel_changed = false;
        let mut pitch_wheel_changed = false;
        let mut xy_pad_changed = false;
        let mut sequencer_changed = false;

        let card_response = Card::new()
            .variant(CardVariant::Filled)
            .show(ui, &theme, |ui| {
                // Top controls section (wheels, XY pad, drum pads)
                if self.sections.show_wheels
                    || self.sections.show_xy_pad
                    || self.sections.show_drum_pads
                {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = theme.spacing.lg;

                        // Wheels section
                        if self.sections.show_wheels {
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = theme.spacing.sm;
                                ui.add_space(theme.spacing.xs);

                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = theme.spacing.md;

                                    let mod_response = ModWheel::new(&mut self.state.mod_wheel)
                                        .wheel_type(WheelType::Modulation)
                                        .variant(self.wheel_variant)
                                        .label("Mod")
                                        .height(180.0)
                                        .id("midi_controller_mod_wheel")
                                        .show(ui);
                                    mod_wheel_changed = mod_response.changed();

                                    let pitch_response = ModWheel::new(&mut self.state.pitch_wheel)
                                        .wheel_type(WheelType::PitchBend)
                                        .variant(self.wheel_variant)
                                        .label("Pitch")
                                        .height(180.0)
                                        .id("midi_controller_pitch_wheel")
                                        .show(ui);
                                    pitch_wheel_changed = pitch_response.changed();
                                });
                            });
                        }

                        // XY Pad section
                        if self.sections.show_xy_pad {
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = theme.spacing.sm;
                                ui.add_space(theme.spacing.xs);

                                let xy_response =
                                    XYPad::new(&mut self.state.xy_x, &mut self.state.xy_y)
                                        .size(180.0)
                                        .variant(XYPadVariant::Filled)
                                        .x_label("X")
                                        .y_label("Y")
                                        .id("midi_controller_xy_pad")
                                        .show(ui);
                                xy_pad_changed = xy_response.changed;
                            });
                        }

                        // Drum pads section
                        if self.sections.show_drum_pads {
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = theme.spacing.sm;
                                ui.add_space(theme.spacing.xs);

                                // Configure drum pad grid
                                let pad_count = self.drum_pad_rows * self.drum_pad_cols;
                                let mut pad_configs = Vec::new();

                                // Default drum mapping (GM MIDI drum map)
                                let drum_notes = [
                                    36, 37, 38, 39, // Bass drums and snares
                                    40, 41, 42, 43, // Snares and toms
                                    44, 45, 46, 47, // Hi-hats and toms
                                    48, 49, 50, 51, // Cymbals and toms
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
                                    .grid(self.drum_pad_rows, self.drum_pad_cols)
                                    .pads(pad_configs)
                                    .pad_states(self.state.drum_pads.clone())
                                    .variant(self.pad_variant)
                                    .color_scheme(PadColorScheme::Semantic)
                                    .show(ui);

                                // Update state with pressed/released pads
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
                        }
                    });

                    ui.add_space(theme.spacing.lg);
                }

                // Step sequencer section
                if self.sections.show_sequencer {
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = theme.spacing.sm;
                        ui.add_space(theme.spacing.xs);

                        // Ensure steps vector has correct size
                        self.state
                            .sequencer_steps
                            .resize(self.sequencer_steps, false);

                        let seq_response = StepSequencer::new(&mut self.state.sequencer_steps)
                            .steps(self.sequencer_steps)
                            .step_size(theme.spacing.xl, theme.spacing.xl)
                            .gap(theme.spacing.xs) // Tighter gap
                            .show_step_numbers(true)
                            .show(ui);

                        sequencer_changed = seq_response.changed;
                    });

                    ui.add_space(theme.spacing.lg);
                }

                // Piano keyboard section
                if self.sections.show_piano {
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = theme.spacing.sm;
                        ui.add_space(theme.spacing.xs);

                        let start_note = ((self.piano_start_octave + 2) * 12) as u8; // C of the octave
                        let pressed_keys: HashSet<u8> =
                            self.state.active_notes.keys().copied().collect();

                        let piano_response_inner = Piano::new()
                            .octaves(self.piano_octaves)
                            .start_note(start_note)
                            .white_key_width(theme.spacing.lg + theme.spacing.xs) // Smaller keys to fit better
                            .white_key_height(100.0) // Slightly shorter
                            .orientation(PianoOrientation::Horizontal)
                            .pressed_keys(pressed_keys)
                            .show(ui, &theme);

                        // Update active notes state
                        for note in &piano_response_inner.clicked_keys {
                            self.state.active_notes.insert(*note, 100); // Default velocity
                        }
                        for note in &piano_response_inner.released_keys {
                            self.state.active_notes.remove(note);
                        }

                        piano_response = Some(piano_response_inner);
                    });
                }
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
    fn test_controller_layout() {
        let mut state = MidiControllerState::default();
        let controller = MidiController::new(&mut state).layout(ControllerLayout::Compact);

        assert_eq!(controller.layout, ControllerLayout::Compact);
        assert!(controller.sections.show_piano);
        assert!(controller.sections.show_wheels);
        assert!(!controller.sections.show_xy_pad);
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
