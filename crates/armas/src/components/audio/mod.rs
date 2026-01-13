//! Audio UI components
//!
//! Components for audio production interfaces.

pub mod fader;
pub mod knob;
pub mod meter;
pub mod midi_controller;
pub mod midi_pad;
pub mod mixer_strip;
pub mod mod_wheel;
pub mod piano;
pub mod piano_roll;
pub mod piano_roll_grid;
pub mod playhead;
pub mod slot;
pub mod step_sequencer;
pub mod time_ruler;
pub mod timeline;
pub mod timeline_track;
pub mod track_header;
pub mod transport;
pub mod xy_pad;

// Re-exports
pub use fader::{Fader, FaderScalePosition, FaderStrip};
pub use knob::Knob;
pub use meter::{AudioMeter, MeterStyle, ScalePosition};
pub use midi_controller::{
    ControllerLayout, ControllerSections, MidiController, MidiControllerResponse,
    MidiControllerState,
};
pub use midi_pad::{MidiPad, MidiPadResponse, PadColorScheme, PadConfig, PadState, PadVariant};
pub use mixer_strip::{Insert, MixerStrip, MixerStripResponse, Route, Send};
pub use mod_wheel::{ModWheel, WheelType, WheelVariant};
pub use piano::{Piano, PianoKey, PianoOrientation, PianoResponse};
pub use piano_roll::{Note, PianoRoll, PianoRollResponse};
pub use piano_roll_grid::{GridDivision, PianoRollGrid};
pub use playhead::Playhead;
pub use slot::Slot;
pub use step_sequencer::{StepSequencer, StepSequencerVariant};
pub use time_ruler::{TimeDisplayMode, TimeRuler};
pub use timeline::{Timeline, TimelineResponse, Track};
pub use timeline_track::{
    AutomationData, AutomationPoint, MidiData, MidiNote, Region, RegionType, TimelineTrack,
    TimelineTrackResponse, WaveformData,
};
pub use track_header::{TrackControls, TrackHeader, TrackHeaderResponse};
pub use transport::{TransportControl, TransportResponse, TransportState};
pub use xy_pad::{XYPad, XYPadVariant};
