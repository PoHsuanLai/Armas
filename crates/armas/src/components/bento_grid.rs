//! Bento Grid Layout
//!
//! Modern grid layout with variable-sized tiles, inspired by macOS and Japanese bento boxes

use crate::Theme;
use egui::{Color32, CornerRadius, Stroke, Ui, Vec2};

/// Grid item span configuration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridSpan {
    /// Single cell (1x1)
    Single,
    /// Wide (2x1)
    Wide,
    /// Tall (1x2)
    Tall,
    /// Large (2x2)
    Large,
}

impl GridSpan {
    fn columns(&self) -> usize {
        match self {
            GridSpan::Single => 1,
            GridSpan::Wide => 2,
            GridSpan::Tall => 1,
            GridSpan::Large => 2,
        }
    }

    fn rows(&self) -> usize {
        match self {
            GridSpan::Single => 1,
            GridSpan::Wide => 1,
            GridSpan::Tall => 2,
            GridSpan::Large => 2,
        }
    }
}

/// A single item in the bento grid
pub struct BentoItem {
    span: GridSpan,
    background: Option<Color32>,
    border: Option<Color32>,
    corner_radius: f32,
    padding: f32,
    content: Box<dyn FnOnce(&mut Ui)>,
}

impl BentoItem {
    /// Create a new bento grid item
    pub fn new(content: impl FnOnce(&mut Ui) + 'static) -> Self {
        Self {
            span: GridSpan::Single,
            background: None,
            border: None,
            corner_radius: 12.0,
            padding: 16.0,
            content: Box::new(content),
        }
    }

    /// Set the grid span
    pub fn span(mut self, span: GridSpan) -> Self {
        self.span = span;
        self
    }

    /// Set background color
    pub fn background(mut self, color: Color32) -> Self {
        self.background = Some(color);
        self
    }

    /// Set border color
    pub fn border(mut self, color: Color32) -> Self {
        self.border = Some(color);
        self
    }

    /// Set corner radius
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Set padding
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }
}

/// Bento grid layout component
///
/// Creates a responsive grid layout with variable-sized tiles
pub struct BentoGrid {
    /// Number of columns in the grid
    columns: usize,
    /// Cell base size (width and height)
    cell_size: f32,
    /// Gap between cells
    gap: f32,
    /// Default background color
    default_background: Color32,
    /// Default border color
    default_border: Option<Color32>,
}

impl BentoGrid {
    /// Create a new bento grid
    pub fn new(columns: usize, cell_size: f32) -> Self {
        Self {
            columns: columns.max(1),
            cell_size: cell_size.max(50.0),
            gap: 12.0,
            default_background: Color32::from_gray(30),
            default_border: Some(Color32::from_gray(60)),
        }
    }

    /// Set gap between cells
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Set default background color
    pub fn default_background(mut self, color: Color32) -> Self {
        self.default_background = color;
        self
    }

    /// Set default border color
    pub fn default_border(mut self, color: Option<Color32>) -> Self {
        self.default_border = color;
        self
    }

    /// Show the bento grid with items
    pub fn show(&self, ui: &mut Ui, _theme: &Theme, items: impl IntoIterator<Item = BentoItem>) {
        let mut current_col = 0;
        let mut current_row = 0;
        let mut max_row = 0;

        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = Vec2::splat(self.gap);

            for item in items {
                let cols = item.span.columns();
                let rows = item.span.rows();

                // Check if item fits in current row
                if current_col + cols > self.columns {
                    current_col = 0;
                    current_row = max_row;
                }

                // Calculate size
                let width = cols as f32 * self.cell_size + (cols - 1) as f32 * self.gap;
                let height = rows as f32 * self.cell_size + (rows - 1) as f32 * self.gap;

                // Draw the item
                let bg_color = item.background.unwrap_or(self.default_background);
                let border_color = item.border.or(self.default_border);

                let (rect, _response) =
                    ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());

                if ui.is_rect_visible(rect) {
                    let painter = ui.painter();

                    // Background
                    painter.rect_filled(
                        rect,
                        CornerRadius::same(item.corner_radius as u8),
                        bg_color,
                    );

                    // Border
                    if let Some(border) = border_color {
                        painter.rect_stroke(
                            rect,
                            CornerRadius::same(item.corner_radius as u8),
                            Stroke::new(1.0, border),
                            egui::StrokeKind::Outside,
                        );
                    }

                    // Content area with padding
                    let content_rect = rect.shrink(item.padding);
                    ui.scope_builder(egui::UiBuilder::new().max_rect(content_rect), |ui| {
                        (item.content)(ui);
                    });
                }

                // Update position
                current_col += cols;
                max_row = max_row.max(current_row + rows);

                // Move to next row if we've filled this one
                if current_col >= self.columns {
                    current_col = 0;
                    current_row = max_row;
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_span() {
        assert_eq!(GridSpan::Single.columns(), 1);
        assert_eq!(GridSpan::Single.rows(), 1);
        assert_eq!(GridSpan::Wide.columns(), 2);
        assert_eq!(GridSpan::Wide.rows(), 1);
        assert_eq!(GridSpan::Tall.columns(), 1);
        assert_eq!(GridSpan::Tall.rows(), 2);
        assert_eq!(GridSpan::Large.columns(), 2);
        assert_eq!(GridSpan::Large.rows(), 2);
    }

    #[test]
    fn test_bento_grid_creation() {
        let grid = BentoGrid::new(3, 100.0);
        assert_eq!(grid.columns, 3);
        assert_eq!(grid.cell_size, 100.0);
    }

    #[test]
    fn test_bento_grid_config() {
        let grid = BentoGrid::new(4, 150.0)
            .gap(16.0)
            .default_background(Color32::BLACK);

        assert_eq!(grid.gap, 16.0);
        assert_eq!(grid.default_background, Color32::BLACK);
    }
}
