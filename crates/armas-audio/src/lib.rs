//! Audio UI components for egui
//!
//! Specialized widgets for audio production interfaces including:
//! - Timeline and track editors
//! - MIDI controllers and piano roll
//! - Meters, faders, and knobs
//! - Automation editors
//! - Transport controls

#![warn(missing_docs)]

pub mod icons;
pub mod automation_canvas;
pub mod automation_curve_selector;
pub mod automation_editor;
pub mod automation_point_handle;
pub mod automation_value_display;
pub mod waveform_display;
pub mod waveform_thumbnail;
pub mod waveform_generator;
pub mod fader;
pub mod knob;
pub mod timeline_region;
pub mod timeline_marker;
pub mod meter;
pub mod snap_grid;
pub mod zoom_control;
pub mod midi_controller;
pub mod midi_pad;
pub mod mixer_strip;
pub mod mod_wheel;
pub mod mpe_keyboard;
pub mod piano;
pub mod piano_roll;
pub mod piano_roll_grid;
pub mod playhead;
pub mod slot;
pub mod step_sequencer;
pub mod drum_sequencer;
pub mod time_ruler;
pub mod timeline;
pub mod timeline_track;
pub mod track_header;
pub mod transport;
pub mod xy_pad;

// Re-exports
pub use fader::{Fader, FaderScalePosition, FaderStrip};
pub use knob::Knob;
pub use timeline_region::{TimelineRegion, TimelineRegionResponse, RegionVariant};
pub use timeline_marker::{TimelineMarker, TimelineMarkerResponse, MarkerVariant};
pub use meter::{AudioMeter, MeterStyle, ScalePosition};
pub use snap_grid::SnapGrid;
pub use zoom_control::{ZoomControl, ZoomControlResponse};
pub use midi_controller::{
    ControllerLayout, ControllerSections, MidiController, MidiControllerResponse,
    MidiControllerState,
};
pub use midi_pad::{MidiPad, MidiPadResponse, PadColorScheme, PadConfig, PadState, PadVariant};
pub use mixer_strip::{Insert, MixerStrip, MixerStripResponse, Route, Send};
pub use mod_wheel::{ModWheel, WheelType, WheelVariant};
pub use mpe_keyboard::{MPEKey, MPEKeyboard, MPEKeyboardResponse, MPENote, MPEOrientation};
pub use piano::{Piano, PianoKey, PianoOrientation, PianoResponse};
pub use piano_roll::{Note, PianoRoll, PianoRollResponse};
pub use piano_roll_grid::{GridDivision, PianoRollGrid};
pub use playhead::Playhead;
pub use slot::Slot;
pub use step_sequencer::StepSequencer;
pub use drum_sequencer::{
    DrumSequencer, DrumRow, DrumStep, DrumSequencerResponse, DrumSequencerVariant,
    DrumSequencerColorScheme,
};
pub use time_ruler::{TimeDisplayMode, TimeRuler};
pub use timeline::{
    LoopRegionData, MarkerData, PunchRegionData, SelectionRangeData, Timeline, TimelineResponse,
    Track,
};
pub use timeline_track::{
    AutomationData, AutomationPoint, FadeCurve, FadeHandle, FadeSettings, MidiData, MidiNote,
    PlaybackSettings, Region, RegionEdge, RegionType, TimelineTrack, TimelineTrackResponse,
};
pub use track_header::{TrackControls, TrackHeader, TrackHeaderResponse};
pub use transport::{TransportControl, TransportResponse, TransportState};
pub use icons::TransportIcon;
pub use xy_pad::{XYPad, XYPadVariant};
pub use automation_canvas::{AutomationCanvas, CanvasConfig, CanvasResponse};
pub use automation_editor::{AutomationEditor, AutomationEditorResponse};
pub use automation_point_handle::{PointHandle, PointHandleResponse};
pub use automation_curve_selector::{CurveTypeSelector, CurveTypeSelectorResponse};
pub use automation_value_display::ValueRangeDisplay;
pub use waveform_display::{WaveformDisplay, WaveformConfig, WaveformResponse, MarkerType};
pub use waveform_thumbnail::{AudioThumbnail, ThumbnailLevel};
pub use waveform_generator::{
    ThumbnailGenerator,
    generate_thumbnail_f32_mono, generate_thumbnail_f32_stereo,
    generate_thumbnail_i16_mono, generate_thumbnail_i16_stereo,
};
