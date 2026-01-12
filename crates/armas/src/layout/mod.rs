//! Layout helpers
//!
//! Minimal utilities for specialized layout needs:
//! - AspectRatio - Maintain aspect ratio with fit/fill modes
//! - Table - Styled tables with striped, bordered, and lined variants
//! - BentoGrid - Variable-sized tile grid layout
//!
//! ## For everything else, use egui's built-ins:
//! - **Vertical/Horizontal layouts:** `ui.vertical()`, `ui.horizontal()`
//! - **Grids:** `egui::Grid`
//! - **Max width:** `ui.set_max_width()`
//! - **Spacing:** `ui.add_space()`
//! - **Separators:** `ui.separator()`
//!
//! For advanced layout (Flexbox, CSS Grid), consider using `egui_taffy`.

mod aspect_ratio;
mod bento_grid;
mod divider;
mod feature_grid;
mod table;

pub use aspect_ratio::{AspectRatio, ContentMode};
pub use bento_grid::{BentoGrid, GridSpan};
pub use divider::{DividerOrientation, DividerStyle, GlowingDivider};
pub use feature_grid::{FeatureGrid, FeatureItem};
pub use table::{Table, TableStyle};
