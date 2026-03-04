//! Shared builder traits for audio components.
//!
//! These traits standardize common builder methods across multiple audio components.

// ============================================================================
// Trait Definitions
// ============================================================================

/// Builder methods for components that operate on a musical timeline
/// with beat-based positioning and measure structure.
pub trait AudioTiming: Sized {
    /// Set the width of one beat in pixels (zoom level)
    #[must_use]
    fn beat_width(self, width: f32) -> Self;
    /// Set the number of measures
    #[must_use]
    fn measures(self, measures: u32) -> Self;
    /// Set the number of beats per measure (time signature numerator)
    #[must_use]
    fn beats_per_measure(self, beats: u32) -> Self;
}

/// Builder methods for components with velocity-sensitive drag interaction.
pub trait VelocityControl: Sized {
    /// Enable/disable velocity-based drag mode (faster mouse = larger changes)
    #[must_use]
    fn velocity_mode(self, enabled: bool) -> Self;
    /// Set sensitivity for velocity mode
    #[must_use]
    fn velocity_sensitivity(self, sensitivity: f64) -> Self;
    /// Set default value for double-click reset
    #[must_use]
    fn default_value(self, value: f32) -> Self;
}

/// Builder methods for components with momentum-based scrolling physics.
pub trait MomentumScroll: Sized {
    /// Enable/disable momentum scrolling (inertia after mouse release)
    #[must_use]
    fn momentum_scrolling(self, enabled: bool) -> Self;
    /// Set momentum damping factor (higher = quicker stop, minimum 1.0)
    #[must_use]
    fn momentum_damping(self, damping: f64) -> Self;
}

/// Builder methods for components with LED/neon glow visual effects.
pub trait GlowEffect: Sized {
    /// Set glow intensity (0.0 to 1.0)
    #[must_use]
    fn glow_intensity(self, intensity: f32) -> Self;
}

// ============================================================================
// Implementation Macros
// ============================================================================

macro_rules! impl_audio_timing {
    ($t:ident $(<$($lt:lifetime),+>)?) => {
        impl$(<$($lt),+>)? AudioTiming for $t$(<$($lt),+>)? {
            fn beat_width(mut self, width: f32) -> Self {
                self.beat_width = width;
                self
            }
            fn measures(mut self, measures: u32) -> Self {
                self.measures = measures;
                self
            }
            fn beats_per_measure(mut self, beats: u32) -> Self {
                self.beats_per_measure = beats;
                self
            }
        }
    };
}

macro_rules! impl_velocity_control {
    ($t:ident $(<$($lt:lifetime),+>)?) => {
        impl$(<$($lt),+>)? VelocityControl for $t$(<$($lt),+>)? {
            fn velocity_mode(mut self, enabled: bool) -> Self {
                self.velocity_mode = enabled;
                self
            }
            fn velocity_sensitivity(mut self, sensitivity: f64) -> Self {
                self.velocity_sensitivity = sensitivity;
                self
            }
            fn default_value(mut self, value: f32) -> Self {
                self.default_value = Some(value);
                self
            }
        }
    };
}

macro_rules! impl_momentum_scroll {
    ($t:ident $(<$($lt:lifetime),+>)?) => {
        impl$(<$($lt),+>)? MomentumScroll for $t$(<$($lt),+>)? {
            fn momentum_scrolling(mut self, enabled: bool) -> Self {
                self.momentum_scrolling = enabled;
                self
            }
            fn momentum_damping(mut self, damping: f64) -> Self {
                self.momentum_damping = damping.max(1.0);
                self
            }
        }
    };
}

macro_rules! impl_glow_effect {
    ($t:ident $(<$($lt:lifetime),+>)?) => {
        impl$(<$($lt),+>)? GlowEffect for $t$(<$($lt),+>)? {
            fn glow_intensity(mut self, intensity: f32) -> Self {
                self.glow_intensity = intensity.clamp(0.0, 1.0);
                self
            }
        }
    };
}

// ============================================================================
// Trait Implementations
// ============================================================================

use crate::drum_sequencer::DrumSequencer;
use crate::fader::Fader;
use crate::knob::Knob;
use crate::midi_pad::MidiPad;
use crate::mod_wheel::ModWheel;
use crate::mpe_keyboard::MPEKeyboard;
use crate::piano_roll::PianoRoll;
use crate::time_ruler::TimeRuler;
use crate::xy_pad::XYPad;

// AudioTiming
impl_audio_timing!(PianoRoll);
impl_audio_timing!(TimeRuler);

// VelocityControl
impl_velocity_control!(Fader);
impl_velocity_control!(Knob);
impl_velocity_control!(ModWheel<'a>);

// MomentumScroll
impl_momentum_scroll!(PianoRoll);
impl_momentum_scroll!(MPEKeyboard);
impl_momentum_scroll!(DrumSequencer<'a>);

// GlowEffect
impl_glow_effect!(XYPad<'a>);
impl_glow_effect!(MidiPad);
impl_glow_effect!(DrumSequencer<'a>);
