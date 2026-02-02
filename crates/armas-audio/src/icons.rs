//! Transport control icons.
//!
//! Lazy-parsed SVG icons for DAW transport controls.
//! Each icon is parsed once on first access via `OnceLock`.
//!
//! # Example
//!
//! ```rust,no_run
//! use armas_audio::icons;
//! use armas_icon::Icon;
//!
//! fn ui(ui: &mut egui::Ui) {
//!     Icon::from_owned(icons::play())
//!         .size(32.0)
//!         .color(egui::Color32::WHITE)
//!         .show(ui);
//! }
//! ```

use armas_icon::OwnedIconData;
use std::sync::OnceLock;

fn parse(svg: &str, name: &str) -> OwnedIconData {
    armas_icon::runtime::parse_svg_named(svg, name).expect("embedded SVG should parse")
}

static PLAY: OnceLock<OwnedIconData> = OnceLock::new();
static PAUSE: OnceLock<OwnedIconData> = OnceLock::new();
static STOP: OnceLock<OwnedIconData> = OnceLock::new();
static RECORD: OnceLock<OwnedIconData> = OnceLock::new();
static REWIND: OnceLock<OwnedIconData> = OnceLock::new();
static FORWARD: OnceLock<OwnedIconData> = OnceLock::new();
static LOOP: OnceLock<OwnedIconData> = OnceLock::new();
static METRONOME: OnceLock<OwnedIconData> = OnceLock::new();

/// Play button icon
pub fn play() -> &'static OwnedIconData {
    PLAY.get_or_init(|| parse(include_str!("../icons/transport/play.svg"), "play"))
}

/// Pause button icon
pub fn pause() -> &'static OwnedIconData {
    PAUSE.get_or_init(|| parse(include_str!("../icons/transport/pause.svg"), "pause"))
}

/// Stop button icon
pub fn stop() -> &'static OwnedIconData {
    STOP.get_or_init(|| parse(include_str!("../icons/transport/stop.svg"), "stop"))
}

/// Record button icon
pub fn record() -> &'static OwnedIconData {
    RECORD.get_or_init(|| parse(include_str!("../icons/transport/fad-record.svg"), "record"))
}

/// Rewind/back button icon
pub fn rewind() -> &'static OwnedIconData {
    REWIND.get_or_init(|| parse(include_str!("../icons/transport/back.svg"), "rewind"))
}

/// Fast forward button icon
pub fn forward() -> &'static OwnedIconData {
    FORWARD.get_or_init(|| parse(include_str!("../icons/transport/forward.svg"), "forward"))
}

/// Loop button icon
pub fn loop_icon() -> &'static OwnedIconData {
    LOOP.get_or_init(|| parse(include_str!("../icons/transport/loop.svg"), "loop"))
}

/// Metronome button icon
pub fn metronome() -> &'static OwnedIconData {
    METRONOME.get_or_init(|| parse(include_str!("../icons/transport/fad-metronome.svg"), "metronome"))
}
