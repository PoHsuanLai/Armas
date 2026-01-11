//! Table component with multiple visual styles
//!
//! Provides a simple table layout with support for different visual styles:
//! - Default: Minimal styling with spacing
//! - Striped: Alternating row backgrounds
//! - Bordered: Full grid with borders
//! - Lined: Horizontal dividers only

use crate::Theme;
use egui;

/// Visual style for the table
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableStyle {
    /// Minimal styling with spacing only
    Default,
    /// Alternating row backgrounds (zebra striping)
    Striped,
    /// Full grid with borders around all cells
    Bordered,
    /// Horizontal dividers between rows only
    Lined,
}

impl Default for TableStyle {
    fn default() -> Self {
        Self::Default
    }
}

/// A table component with customizable styling
///
/// # Example
///
/// ```rust,ignore
/// Table::new()
///     .style(TableStyle::Striped)
///     .compact(true)
///     .show(ui, |table| {
///         table.header_row(|row| {
///             row.cell("Name");
///             row.cell("Age");
///             row.cell("City");
///         });
///
///         table.row(|row| {
///             row.cell("Alice");
///             row.cell("30");
///             row.cell("NYC");
///         });
///
///         table.row(|row| {
///             row.cell("Bob");
///             row.cell("25");
///             row.cell("SF");
///         });
///     });
/// ```
pub struct Table {
    style: TableStyle,
    compact: bool,
    hoverable: bool,
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    /// Create a new table with default styling
    pub fn new() -> Self {
        Self {
            style: TableStyle::Default,
            compact: false,
            hoverable: false,
        }
    }

    /// Set the visual style
    pub fn style(mut self, style: TableStyle) -> Self {
        self.style = style;
        self
    }

    /// Enable compact mode with reduced padding
    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    /// Enable row hover highlighting
    pub fn hoverable(mut self, hoverable: bool) -> Self {
        self.hoverable = hoverable;
        self
    }

    /// Show the table with the given content
    pub fn show<R>(
        self,
        ui: &mut egui::Ui,
        content: impl FnOnce(&mut TableBuilder) -> R,
    ) -> R {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<Theme>(egui::Id::new("armas_theme"))
                .unwrap_or_else(Theme::dark)
        });

        let padding = if self.compact { 8.0 } else { 12.0 };
        let spacing = if self.compact { 8.0 } else { 12.0 };

        let grid_id = ui.id().with("table_grid");
        let available_width = ui.available_width();

        egui::Grid::new(grid_id)
            .spacing([spacing, 0.0])
            .striped(self.style == TableStyle::Striped)
            .min_col_width(0.0)
            .show(ui, |ui| {
                let mut builder = TableBuilder {
                    ui,
                    style: self.style,
                    _compact: self.compact,
                    _hoverable: self.hoverable,
                    theme: &theme,
                    row_index: 0,
                    padding,
                    available_width,
                    num_columns: 0,
                };

                content(&mut builder)
            })
            .inner
    }
}

/// Builder for constructing table rows
pub struct TableBuilder<'a> {
    ui: &'a mut egui::Ui,
    style: TableStyle,
    _compact: bool,
    _hoverable: bool,
    theme: &'a Theme,
    row_index: usize,
    padding: f32,
    available_width: f32,
    num_columns: usize,
}

impl<'a> TableBuilder<'a> {
    /// Add a header row
    pub fn header_row<R>(&mut self, content: impl FnOnce(&mut RowBuilder) -> R) -> R {
        let mut row_builder = RowBuilder {
            ui: self.ui,
            style: self.style,
            theme: self.theme,
            is_header: true,
            cell_index: 0,
            _padding: self.padding,
            col_width: None,
        };

        let result = content(&mut row_builder);

        // Track number of columns from header
        if self.num_columns == 0 {
            self.num_columns = row_builder.cell_index;
        }

        self.ui.end_row();

        // Add horizontal separator after header for Lined and Bordered styles
        if matches!(self.style, TableStyle::Lined | TableStyle::Bordered) {
            for _ in 0..self.num_columns {
                self.ui.add(egui::Separator::default().horizontal());
            }
            self.ui.end_row();
        }

        result
    }

    /// Add a regular data row
    pub fn row<R>(&mut self, content: impl FnOnce(&mut RowBuilder) -> R) -> R {
        // Calculate column width if we know the number of columns
        let col_width = if self.num_columns > 0 {
            Some((self.available_width - (self.num_columns - 1) as f32 * 12.0) / self.num_columns as f32)
        } else {
            None
        };

        let mut row_builder = RowBuilder {
            ui: self.ui,
            style: self.style,
            theme: self.theme,
            is_header: false,
            cell_index: 0,
            _padding: self.padding,
            col_width,
        };

        let result = content(&mut row_builder);
        self.ui.end_row();

        // Add horizontal separator for Lined and Bordered styles
        if matches!(self.style, TableStyle::Lined | TableStyle::Bordered) {
            // Draw a full-width separator across all columns
            for _ in 0..self.num_columns {
                self.ui.add(egui::Separator::default().horizontal());
            }
            self.ui.end_row();
        }

        self.row_index += 1;
        result
    }
}

/// Builder for constructing table cells within a row
pub struct RowBuilder<'a> {
    ui: &'a mut egui::Ui,
    style: TableStyle,
    theme: &'a Theme,
    is_header: bool,
    cell_index: usize,
    _padding: f32,
    col_width: Option<f32>,
}

impl<'a> RowBuilder<'a> {
    /// Add a cell with text content
    pub fn cell(&mut self, text: impl Into<String>) -> egui::Response {
        let text = text.into();

        // Add vertical separator before cell for Bordered style (except first column)
        if self.style == TableStyle::Bordered && self.cell_index > 0 {
            self.ui.add(egui::Separator::default().vertical());
        }

        let response = if let Some(width) = self.col_width {
            // Use vertical layout to allow height to expand
            self.ui.vertical(|ui| {
                ui.set_width(width);

                let label = if self.is_header {
                    egui::Label::new(
                        egui::RichText::new(&text)
                            .strong()
                            .color(self.theme.on_surface())
                    ).wrap()
                } else {
                    egui::Label::new(
                        egui::RichText::new(&text)
                            .color(self.theme.on_surface_variant())
                    ).wrap()
                };
                ui.add(label)
            }).inner
        } else {
            let label = if self.is_header {
                egui::Label::new(
                    egui::RichText::new(&text)
                        .strong()
                        .color(self.theme.on_surface())
                ).wrap()
            } else {
                egui::Label::new(
                    egui::RichText::new(&text)
                        .color(self.theme.on_surface_variant())
                ).wrap()
            };
            self.ui.add(label)
        };

        self.cell_index += 1;
        response
    }

    /// Add a custom cell with a closure
    pub fn cell_ui<R>(&mut self, content: impl FnOnce(&mut egui::Ui) -> R) -> R {
        // Add vertical separator before cell for Bordered style (except first column)
        if self.style == TableStyle::Bordered && self.cell_index > 0 {
            self.ui.add(egui::Separator::default().vertical());
        }

        let result = if let Some(width) = self.col_width {
            self.ui.vertical(|ui| {
                ui.set_width(width);
                content(ui)
            }).inner
        } else {
            content(self.ui)
        };

        self.cell_index += 1;
        result
    }
}
