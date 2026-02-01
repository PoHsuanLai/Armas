//! Bento Grid Layout
//!
//! Grid layout with variable-sized tiles, inspired by macOS and Japanese bento boxes

use crate::Theme;
use egui::{Color32, Pos2, Rect, Sense, Stroke, Ui, Vec2};

/// Grid item span configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    const fn columns(self) -> usize {
        match self {
            Self::Single | Self::Tall => 1,
            Self::Wide | Self::Large => 2,
        }
    }

    const fn rows(self) -> usize {
        match self {
            Self::Single | Self::Wide => 1,
            Self::Tall | Self::Large => 2,
        }
    }
}

/// Bento grid layout component
pub struct BentoGrid {
    columns: usize,
    cell_size: f32,
    gap: f32,
    corner_radius: f32,
    padding: f32,
}

impl Default for BentoGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl BentoGrid {
    /// Create a new bento grid layout
    #[must_use]
    pub const fn new() -> Self {
        Self {
            columns: 3,
            cell_size: 120.0,
            gap: 12.0,
            corner_radius: 12.0,
            padding: 16.0,
        }
    }

    /// Set the number of columns
    #[must_use]
    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns.max(1);
        self
    }

    /// Set the base cell size
    #[must_use]
    pub const fn cell_size(mut self, size: f32) -> Self {
        self.cell_size = size.max(50.0);
        self
    }

    /// Set the gap between cells
    #[must_use]
    pub const fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Set the corner radius for cells
    #[must_use]
    pub const fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Set the padding around the grid
    #[must_use]
    pub const fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Show the bento grid with the given content
    pub fn show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut GridBuilder) -> R) -> R {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<Theme>(egui::Id::new("armas_theme"))
                .unwrap_or_else(Theme::dark)
        });

        ui.vertical(|ui| {
            // Allocate the full grid area upfront
            let start_pos = ui.cursor().min;

            let mut builder = GridBuilder {
                ui,
                theme: &theme,
                columns: self.columns,
                cell_size: self.cell_size,
                gap: self.gap,
                corner_radius: self.corner_radius,
                padding: self.padding,
                grid_start_pos: start_pos,
                current_col: 0,
                current_row: 0,
                occupied: Vec::new(),
            };

            let result = content(&mut builder);

            // Calculate total height based on occupied rows
            let max_row = builder.occupied.len();
            let total_height = if max_row > 0 {
                max_row as f32 * self.cell_size + (max_row - 1) as f32 * self.gap
            } else {
                0.0
            };
            let grid_width =
                self.columns as f32 * self.cell_size + (self.columns - 1) as f32 * self.gap;
            ui.allocate_space(Vec2::new(grid_width, total_height));

            result
        })
        .inner
    }
}

pub struct GridBuilder<'a> {
    ui: &'a mut Ui,
    theme: &'a Theme,
    columns: usize,
    cell_size: f32,
    gap: f32,
    corner_radius: f32,
    padding: f32,
    grid_start_pos: Pos2,
    current_col: usize,
    current_row: usize,
    // Track occupied cells: (row, col) -> height in rows
    occupied: Vec<Vec<usize>>,
}

impl GridBuilder<'_> {
    /// Check if a cell is occupied
    fn is_occupied(&self, row: usize, col: usize) -> bool {
        if row >= self.occupied.len() {
            return false;
        }
        if col >= self.occupied[row].len() {
            return false;
        }
        self.occupied[row][col] > 0
    }

    /// Mark cells as occupied
    fn mark_occupied(&mut self, row: usize, col: usize, cols: usize, rows: usize) {
        // Ensure we have enough rows
        while self.occupied.len() <= row + rows {
            self.occupied.push(vec![0; self.columns]);
        }

        // Mark all cells occupied by this item
        for r in row..row + rows {
            for c in col..col + cols {
                if c < self.columns {
                    self.occupied[r][c] = rows;
                }
            }
        }
    }

    /// Find next available position
    fn find_next_position(&mut self, cols: usize, rows: usize) {
        loop {
            // Check if current position can fit the item
            let mut can_fit = self.current_col + cols <= self.columns;

            if can_fit {
                // Check if all cells in the span are free
                for r in 0..rows {
                    for c in 0..cols {
                        if self.is_occupied(self.current_row + r, self.current_col + c) {
                            can_fit = false;
                            break;
                        }
                    }
                    if !can_fit {
                        break;
                    }
                }
            }

            if can_fit {
                return;
            }

            // Move to next position
            self.current_col += 1;
            if self.current_col >= self.columns {
                self.current_col = 0;
                self.current_row += 1;
            }
        }
    }

    pub fn item<R>(&mut self, span: GridSpan, content: impl FnOnce(&mut Ui) -> R) -> R {
        self.item_with_style(span, None, None, content)
    }

    pub fn item_with_background<R>(
        &mut self,
        span: GridSpan,
        background: Color32,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> R {
        self.item_with_style(span, Some(background), None, content)
    }

    pub fn item_with_style<R>(
        &mut self,
        span: GridSpan,
        background: Option<Color32>,
        border: Option<Color32>,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> R {
        let cols = span.columns();
        let rows = span.rows();

        // Find next available position that can fit this item
        self.find_next_position(cols, rows);

        // Calculate position and size
        let x = self.grid_start_pos.x + self.current_col as f32 * (self.cell_size + self.gap);
        let y = self.grid_start_pos.y + self.current_row as f32 * (self.cell_size + self.gap);

        let width = cols as f32 * self.cell_size + (cols - 1) as f32 * self.gap;
        let height = rows as f32 * self.cell_size + (rows - 1) as f32 * self.gap;

        let rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(width, height));

        // Draw background and border
        let painter = self.ui.painter();
        let bg_color = background.unwrap_or_else(|| self.theme.card());
        let border_color = border.or_else(|| Some(self.theme.border()));

        painter.rect_filled(rect, self.corner_radius, bg_color);

        if let Some(border) = border_color {
            painter.rect_stroke(
                rect,
                self.corner_radius,
                Stroke::new(1.0, border),
                egui::StrokeKind::Outside,
            );
        }

        // Render content in the padded area
        let content_rect = rect.shrink(self.padding);
        let result = self
            .ui
            .scope_builder(egui::UiBuilder::new().max_rect(content_rect), |ui| {
                content(ui)
            })
            .inner;

        // Register interaction
        let _response = self.ui.interact(
            rect,
            self.ui.id().with((self.current_col, self.current_row)),
            Sense::hover(),
        );

        // Mark cells as occupied
        self.mark_occupied(self.current_row, self.current_col, cols, rows);

        // Update grid position - move to next cell
        self.current_col += cols;
        if self.current_col >= self.columns {
            self.current_col = 0;
            self.current_row += 1;
        }

        result
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
        let grid = BentoGrid::new().columns(3).cell_size(100.0);
        assert_eq!(grid.columns, 3);
        assert_eq!(grid.cell_size, 100.0);
    }

    #[test]
    fn test_bento_grid_config() {
        let grid = BentoGrid::new().gap(16.0).corner_radius(8.0);
        assert_eq!(grid.gap, 16.0);
        assert_eq!(grid.corner_radius, 8.0);
    }
}
