//! Visual effects for UI components
//!
//! This module contains visual effects that can be applied to UI components,
//! such as gradient text and moving borders.

pub mod gradient_text;
pub mod moving_border;

// Re-exports
pub use gradient_text::GradientText;
pub use moving_border::MovingBorder;
