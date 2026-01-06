use egui::Vec2;

/// Calculate widths for flex items in a horizontal layout
///
/// Takes a list of item specs (either Fixed or Flex) and returns calculated widths.
///
/// # Example
///
/// ```rust,no_run
/// use armas::layout::{calc_flex_widths, FlexItem};
///
/// let available = ui.available_width();
/// let widths = calc_flex_widths(available, 8.0, &[
///     FlexItem::Fixed(240.0),  // Sidebar: 240px
///     FlexItem::Flex(1.0),      // Content: remaining space
/// ]);
///
/// // Use widths to layout items
/// ui.horizontal(|ui| {
///     ui.allocate_ui(Vec2::new(widths[0], ui.available_height()), |ui| {
///         // Sidebar content
///     });
///     ui.add_space(8.0);
///     ui.allocate_ui(Vec2::new(widths[1], ui.available_height()), |ui| {
///         // Main content
///     });
/// });
/// ```
pub fn calc_flex_widths(available_width: f32, gap: f32, items: &[FlexItem]) -> Vec<f32> {
    let total_gaps = gap * (items.len().saturating_sub(1)) as f32;
    let mut total_fixed = 0.0;
    let mut total_flex = 0.0;

    // Calculate totals
    for item in items {
        match item {
            FlexItem::Fixed(size) => total_fixed += size,
            FlexItem::Flex(ratio) => total_flex += ratio,
        }
    }

    // Calculate available space for flex items
    let flex_space = (available_width - total_fixed - total_gaps).max(0.0);

    // Calculate individual widths
    items
        .iter()
        .map(|item| match item {
            FlexItem::Fixed(size) => *size,
            FlexItem::Flex(ratio) => {
                if total_flex > 0.0 {
                    flex_space * (ratio / total_flex)
                } else {
                    0.0
                }
            }
        })
        .collect()
}

/// Calculate heights for flex items in a vertical layout
///
/// Takes a list of item specs (either Fixed or Flex) and returns calculated heights.
pub fn calc_flex_heights(available_height: f32, gap: f32, items: &[FlexItem]) -> Vec<f32> {
    // Same logic as widths, just for heights
    let total_gaps = gap * (items.len().saturating_sub(1)) as f32;
    let mut total_fixed = 0.0;
    let mut total_flex = 0.0;

    for item in items {
        match item {
            FlexItem::Fixed(size) => total_fixed += size,
            FlexItem::Flex(ratio) => total_flex += ratio,
        }
    }

    let flex_space = (available_height - total_fixed - total_gaps).max(0.0);

    items
        .iter()
        .map(|item| match item {
            FlexItem::Fixed(size) => *size,
            FlexItem::Flex(ratio) => {
                if total_flex > 0.0 {
                    flex_space * (ratio / total_flex)
                } else {
                    0.0
                }
            }
        })
        .collect()
}

/// Specification for a flex layout item
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexItem {
    /// Fixed size item (in pixels)
    Fixed(f32),
    /// Flexible item with a flex ratio
    ///
    /// The ratio determines proportional space distribution.
    /// For example, Flex(1.0) and Flex(2.0) will distribute space in a 1:2 ratio.
    Flex(f32),
}
