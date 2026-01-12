//! Audio UI components
//!
//! Components for audio production interfaces.

pub mod fader;
pub mod knob;
pub mod meter;
pub mod mixer_strip;
pub mod piano;
pub mod piano_roll;
pub mod piano_roll_grid;
pub mod slot;

// Re-exports
pub use fader::{Fader, FaderStrip};
pub use knob::Knob;
pub use meter::{AudioMeter, MeterStyle, ScalePosition};
pub use mixer_strip::MixerStrip;
pub use piano::{Piano, PianoKey, PianoOrientation, PianoResponse};
pub use piano_roll::{Note, PianoRoll, PianoRollResponse};
pub use piano_roll_grid::{GridDivision, PianoRollGrid};
pub use slot::Slot;
