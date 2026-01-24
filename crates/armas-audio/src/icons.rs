//! Transport control icons for audio UI
//!
//! Pre-tessellated SVG icons for DAW transport controls.
//! Generated at compile time from SVG files in `icons/transport/`.

use armas_icon::IconData;

// Include the generated icon data
include!(concat!(env!("OUT_DIR"), "/transport_icons.rs"));

/// Transport control icons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransportIcon {
    /// Play button
    Play,
    /// Pause button
    Pause,
    /// Stop button
    Stop,
    /// Record button
    Record,
    /// Rewind button
    Rewind,
    /// Fast forward button
    Forward,
    /// Loop button
    Loop,
    /// Metronome button
    Metronome,
}

impl TransportIcon {
    /// Get the icon data for this transport icon
    pub fn data(self) -> &'static IconData {
        match self {
            Self::Play => &PLAY,
            Self::Pause => &PAUSE,
            Self::Stop => &STOP,
            Self::Record => &FAD_RECORD,
            Self::Rewind => &BACK,
            Self::Forward => &FORWARD,
            Self::Loop => &LOOP,
            Self::Metronome => &FAD_METRONOME,
        }
    }

    /// Create an Icon widget from this TransportIcon
    pub fn icon(self) -> armas_icon::Icon<'static> {
        armas_icon::Icon::new(self.data())
    }
}
