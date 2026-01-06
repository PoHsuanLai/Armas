//! Layout components inspired by SwiftUI
//!
//! Provides composable layout primitives for building UIs:
//! - VStack/HStack/ZStack - Stack layouts with spacing and layering
//! - Spacer - Flexible and fixed spacing
//! - Container - Max-width wrapper with padding
//! - Divider - Horizontal/vertical separator lines
//! - Grid - Responsive grid layout
//! - FormLayout - Two-column form layout for label-value pairs
//! - Table - Tabular data with headers and rows
//! - ScrollView - Themed scrollable area
//! - AspectRatio - Maintain aspect ratio wrapper
//! - Flex utilities - Calculate proportional sizes for responsive layouts

mod aspect_ratio;
mod container;
mod divider;
mod flex;
mod form_layout;
mod grid;
mod hstack;
mod scroll_view;
mod spacer;
mod table;
mod vstack;
mod zstack;

pub use aspect_ratio::{AspectRatio, ContentMode};
pub use container::{Container, ContainerSize};
pub use divider::Divider;
pub use flex::{calc_flex_heights, calc_flex_widths, FlexItem};
pub use form_layout::{FormBuilder, FormLayout};
pub use grid::{Grid, GridBuilder, GridTemplate};
pub use hstack::{HStack, HStackBuilder};
pub use scroll_view::ScrollView;
pub use spacer::Spacer;
pub use table::{RowBuilder, Table, TableBuilder};
pub use vstack::{VStack, VStackBuilder};
pub use zstack::{ZStack, ZStackBuilder};

/// Alignment for layout components
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// Leading edge (left in LTR, right in RTL)
    Leading,
    /// Center
    Center,
    /// Trailing edge (right in LTR, left in RTL)
    Trailing,
    /// Top edge
    Top,
    /// Bottom edge
    Bottom,
}

impl Alignment {
    /// Convert to egui Align
    pub(crate) fn to_egui_align(self) -> egui::Align {
        match self {
            Alignment::Leading => egui::Align::Min,
            Alignment::Center => egui::Align::Center,
            Alignment::Trailing => egui::Align::Max,
            Alignment::Top => egui::Align::Min,
            Alignment::Bottom => egui::Align::Max,
        }
    }
}
