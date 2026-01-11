//! Button components module
//!
//! This module provides various button styles:
//! - **Base Button**: Material Design 3 variants (Filled, Outlined, Text, etc.)
//! - **Aceternity Buttons**: Specialized button styles inspired by Aceternity UI

pub mod base;
pub mod brutal;
pub mod figma;
pub mod invert;
pub mod shimmer;
pub mod simple;
pub mod sketch;
pub mod spotify;

// Re-export the base button and its variant enum
pub use base::{Button, ButtonVariant};

// Re-export Aceternity button styles
pub use brutal::BrutalButton;
pub use figma::FigmaButton;
pub use invert::InvertButton;
pub use shimmer::ShimmerButton;
pub use simple::SimpleButton;
pub use sketch::SketchButton;
pub use spotify::SpotifyButton;
