//! Audio UI components for egui
//!
//! Specialized widgets for audio production interfaces including:
//! - Timeline and track editors
//! - MIDI controllers and piano roll
//! - Meters, faders, and knobs

#![warn(missing_docs)]

pub mod drum_sequencer;
pub mod fader;
pub mod knob;
pub mod meter;
pub mod midi_controller;
pub mod midi_pad;
pub mod mixer_strip;
pub mod mod_wheel;
pub mod mpe_keyboard;
pub mod piano_roll;
pub mod step_sequencer;
pub mod timeline;
pub mod timeline_marker;
pub mod timeline_region;
pub mod xy_pad;

// Icon module - transport icons used by documentation
pub mod icons;
pub(crate) mod piano;
pub(crate) mod playhead;
pub(crate) mod slot;
pub(crate) mod snap_grid;
pub(crate) mod time_ruler;
pub(crate) mod timeline_track;
pub(crate) mod track_header;

// Re-exports
pub use drum_sequencer::{
    DrumRow, DrumSequencer, DrumSequencerColorScheme, DrumSequencerResponse, DrumSequencerVariant,
    DrumStep,
};
pub use fader::{Fader, FaderScalePosition, FaderStrip};
pub use knob::Knob;
pub use meter::{AudioMeter, MeterStyle, ScalePosition};
pub use midi_controller::{MidiController, MidiControllerResponse, MidiControllerState};
pub use midi_pad::{MidiPad, MidiPadResponse, PadColorScheme, PadConfig, PadState, PadVariant};
pub use mixer_strip::{Insert, MixerStrip, MixerStripMode, MixerStripResponse, Route, Send};
pub use mod_wheel::{ModWheel, WheelSize, WheelType};
pub use mpe_keyboard::{MPEKey, MPEKeyboard, MPEKeyboardResponse, MPENote, MPEOrientation};
pub use piano_roll::{Note, PianoRoll, PianoRollResponse};
pub use step_sequencer::StepSequencer;
pub use timeline::{
    LoopRegionData, MarkerData, PunchRegionData, SelectionRangeData, Timeline, TimelineResponse,
    Track,
};
pub use timeline_marker::{MarkerVariant, TimelineMarker, TimelineMarkerResponse};
pub use timeline_region::{RegionVariant, TimelineRegion, TimelineRegionResponse};
pub use icons::TransportIcon;
pub use xy_pad::{XYPad, XYPadVariant};

// Types from internal modules that are exposed through public API structs
pub use piano_roll::GridDivision;
pub use timeline_track::{
    AutomationData, AutomationPoint, FadeCurve, FadeHandle, FadeSettings, MidiData, MidiNote,
    PlaybackSettings, Region, RegionEdge, RegionType,
};
pub use track_header::TrackControls;

// Internal re-exports for cross-module use within the crate
pub(crate) use piano::{Piano, PianoOrientation, PianoResponse};
pub(crate) use playhead::Playhead;
pub(crate) use slot::Slot;
pub(crate) use snap_grid::SnapGrid;
pub(crate) use time_ruler::TimeRuler;
pub(crate) use timeline_track::TimelineTrack;
pub(crate) use track_header::TrackHeader;
