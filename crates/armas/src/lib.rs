//! # Armas
//!
//! UI component library for egui following shadcn/ui design patterns.
//!
//! This is an umbrella crate that re-exports the Armas component sub-crates
//! with feature gates.
//!
//! ## Features
//!
//! - `basic` (default) - Core UI components via [`armas_basic`]
//! - `icon` (default) - SVG icon system via [`armas_icon`]
//! - `audio` - Audio/DAW components via [`armas_audio`]

#[cfg(feature = "basic")]
pub use armas_basic::*;

#[cfg(feature = "icon")]
pub use armas_icon;

#[cfg(feature = "audio")]
pub use armas_audio;
