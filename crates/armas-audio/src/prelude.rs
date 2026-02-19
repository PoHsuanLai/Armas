//! Convenience re-exports for `use armas_audio::prelude::*`.

pub use crate::automation_editor::{
    AutomationEditor, AutomationEditorResponse, AutomationPoint as AutoPoint,
};
pub use crate::drum_sequencer::{
    DrumRow, DrumSequencer, DrumSequencerColorScheme, DrumSequencerResponse, DrumSequencerVariant,
    DrumStep,
};
pub use crate::fader::{Fader, FaderScalePosition, FaderStrip, FaderStripResponse};
pub use crate::icons;
pub use crate::knob::Knob;
pub use crate::meter::{AudioMeter, MeterStyle, ScalePosition};
pub use crate::midi_controller::{MidiController, MidiControllerResponse, MidiControllerState};
pub use crate::midi_pad::{
    MidiPad, MidiPadResponse, PadColorScheme, PadConfig, PadState, PadVariant,
};
pub use crate::mixer_strip::{Insert, MixerStrip, MixerStripMode, MixerStripResponse, Route, Send};
pub use crate::mod_wheel::{ModWheel, WheelSize, WheelType};
pub use crate::mpe_keyboard::{MPEKey, MPEKeyboard, MPEKeyboardResponse, MPENote, MPEOrientation};
pub use crate::piano::{Piano, PianoKey, PianoOrientation, PianoResponse};
pub use crate::piano_roll::{GridDivision, Note, PianoRoll, PianoRollResponse};
pub use crate::step_sequencer::StepSequencer;
pub use crate::timeline::{
    LoopRegionData, MarkerData, PunchRegionData, SelectionRangeData, Timeline, TimelineResponse,
    Track,
};
pub use crate::timeline_marker::{MarkerVariant, TimelineMarker, TimelineMarkerResponse};
pub use crate::timeline_region::{RegionVariant, TimelineRegion, TimelineRegionResponse};
pub use crate::timeline_track::{
    AudioData, AutomationData, AutomationPoint, FadeCurve, FadeHandle, FadeSettings, MidiData,
    MidiNote, PlaybackSettings, Region, RegionEdge, RegionType,
};
pub use crate::track_header::TrackControls;
pub use crate::traits::{AudioTiming, GlowEffect, MomentumScroll, VelocityControl};
pub use crate::xy_pad::{XYPad, XYPadVariant};
