//! # Armas
//!
//! A theme-aware component library for [egui](https://github.com/emilk/egui),
//! inspired by [shadcn/ui](https://ui.shadcn.com).
//!
//! This is the umbrella crate that re-exports the Armas sub-crates
//! with feature gates.
//!
//! ## Features
//!
//! - `basic` (default) — Core UI components via [`armas_basic`]
//! - `icon` (default) — SVG icon system via [`armas_icon`]
//! - `audio` — Audio/DAW components via `armas_audio`
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use armas::prelude::*;
//!
//! // Set the theme once at startup
//! // ctx.set_armas_theme(Theme::dark());
//!
//! // Then use components in your UI code
//! // Button::new("Click me").variant(ButtonVariant::Default).show(ui);
//! ```

#[cfg(feature = "basic")]
pub use armas_basic::*;

#[cfg(feature = "icon")]
pub use armas_icon;

#[cfg(feature = "audio")]
pub use armas_audio;
