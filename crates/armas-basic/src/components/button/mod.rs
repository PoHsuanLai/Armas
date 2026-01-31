//! Button components module
//!
//! This module provides various button styles:
//! - **Base Button**: Material Design 3 variants (Filled, Outlined, Text, etc.)
//! - **Icon Button**: Material Design 3 icon button variants
//! - **Aceternity Buttons**: Specialized button styles inspired by Aceternity UI

pub mod base;
pub mod brutal;
pub mod icon_button;
pub mod shimmer;
pub mod simple;

// Re-export the base button and its variant/size enums
pub use base::{Button, ButtonSize, ButtonVariant};

// Re-export the icon button
pub use icon_button::IconButton;

// Re-export Aceternity button styles
pub use brutal::BrutalButton;
pub use shimmer::ShimmerButton;
pub use simple::SimpleButton;
