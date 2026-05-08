//! # Armas Audio
//!
//! Audio UI components for [egui](https://github.com/emilk/egui).
//!
//! Provides specialized widgets for audio production interfaces:
//! - Knobs, faders, and meters
//! - MIDI keyboards, pads, and controllers
//! - Piano roll and mixer strip
//!
//! Audio components take `&Theme` as the last parameter to `show()`.

#![warn(missing_docs)]

pub mod drum_sequencer;
pub mod fader;
pub mod knob;
pub mod meter;
pub mod midi_pad;
pub mod mixer_strip;
pub mod mod_wheel;
pub mod mpe_keyboard;
pub mod piano;
pub mod piano_roll;
pub mod time_ruler;
pub mod xy_pad;

pub(crate) mod slot;

// Re-exports
pub use drum_sequencer::{
    DrumRow, DrumSequencer, DrumSequencerColorScheme, DrumSequencerResponse, DrumSequencerVariant,
    DrumStep,
};
pub use fader::{Fader, FaderScalePosition, FaderStrip, FaderStripResponse};
pub use knob::Knob;
pub use meter::{AudioMeter, MeterStyle, ScalePosition};
pub use midi_pad::{MidiPad, MidiPadResponse, PadColorScheme, PadConfig, PadState, PadVariant};
pub use mixer_strip::{Insert, MixerStrip, MixerStripMode, MixerStripResponse, Route, Send};
pub use mod_wheel::{ModWheel, ModWheelResponse, WheelSize, WheelType};
pub use mpe_keyboard::{MPEKey, MPEKeyboard, MPEKeyboardResponse, MPENote, MPEOrientation};
pub use piano::{Piano, PianoKey, PianoOrientation, PianoResponse};
pub use piano_roll::{GridDivision, Note, PianoRoll, PianoRollResponse};
pub use time_ruler::TimeRuler;
pub use xy_pad::{XYPad, XYPadVariant};

mod traits;
pub use traits::{AudioTiming, GlowEffect, MomentumScroll, VelocityControl};

pub mod prelude;
