//! Background visual effects and patterns
//!
//! This module contains components designed to be used as background layers,
//! including animated effects and geometric patterns.

pub mod aurora;
pub mod dot_pattern;
pub mod grid_pattern;
pub mod meteor;
pub mod retro_grid;
pub mod sparkles;
pub mod spotlight;

// Re-exports
pub use aurora::AuroraBackground;
pub use dot_pattern::DotPattern;
pub use grid_pattern::GridPattern;
pub use meteor::MeteorShower;
pub use retro_grid::RetroGrid;
pub use sparkles::Sparkles;
pub use spotlight::Spotlight;
