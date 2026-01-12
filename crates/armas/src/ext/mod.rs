//! Extension traits for egui types
//!
//! This module provides extension traits that add Armas-specific
//! functionality to egui types.

pub mod context;
pub mod painter;

pub use context::ArmasContextExt;
pub use painter::{neon_circle, neon_line, PainterExt};
