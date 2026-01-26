//! Layout helpers
//!
//! Minimal utilities for specialized layout needs:
//! - AspectRatio - Maintain aspect ratio with fit/fill modes
//! - Table - Responsive table with shadcn/ui styling
//! - BentoGrid - Variable-sized tile grid layout
//!
//! ## For everything else, use egui's built-ins:
//! - **Vertical/Horizontal layouts:** `ui.vertical()`, `ui.horizontal()`
//! - **Grids:** `egui::Grid`
//! - **Max width:** `ui.set_max_width()`
//! - **Spacing:** `ui.add_space()`
//! - **Separators:** Use `Separator` component
//!
//! For advanced layout (Flexbox, CSS Grid), consider using `egui_taffy`.

mod aspect_ratio;
mod bento_grid;
mod table;

pub use aspect_ratio::{AspectRatio, ContentMode};
pub use bento_grid::{BentoGrid, GridSpan};
pub use table::{table, header_row, row, cell, cell_ui, TableRows, TableCells};
