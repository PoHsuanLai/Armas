use crate::Theme;
use egui::{Response, Ui};

/// Table layout with headers and rows
///
/// Displays tabular data with column headers and automatic sizing.
/// Supports sorting indicators, striped rows, and custom column widths.
///
/// # Example
///
/// ```rust,no_run
/// use armas::layout::Table;
///
/// Table::new()
///     .striped(true)
///     .show(ui, &theme, |table| {
///         table.headers(&["Name", "Email", "Role"]);
///
///         table.row(|row| {
///             row.col(|ui| { ui.label("Alice"); });
///             row.col(|ui| { ui.label("alice@example.com"); });
///             row.col(|ui| { ui.label("Admin"); });
///         });
///
///         table.row(|row| {
///             row.col(|ui| { ui.label("Bob"); });
///             row.col(|ui| { ui.label("bob@example.com"); });
///             row.col(|ui| { ui.label("User"); });
///         });
///     });
/// ```
pub struct Table {
    striped: bool,
    gap: f32,
    min_col_width: Option<f32>,
    auto_width: bool,
    id: Option<egui::Id>,
}

impl Table {
    /// Create a new table
    pub fn new() -> Self {
        Self {
            striped: true,
            gap: 8.0,
            min_col_width: None,
            auto_width: true,
            id: None,
        }
    }

    /// Enable or disable striped rows (alternating background colors)
    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    /// Set the gap between columns
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Set minimum column width
    pub fn min_col_width(mut self, width: f32) -> Self {
        self.min_col_width = Some(width);
        self.auto_width = false;
        self
    }

    /// Enable auto width distribution (default: true)
    pub fn auto_width(mut self, enabled: bool) -> Self {
        self.auto_width = enabled;
        self
    }

    /// Set a custom ID for the table
    pub fn id_source(mut self, id_source: impl std::hash::Hash) -> Self {
        self.id = Some(egui::Id::new(id_source));
        self
    }

    /// Show the table with the given content builder
    pub fn show<R>(
        self,
        ui: &mut Ui,
        theme: &Theme,
        builder: impl FnOnce(&mut TableBuilder) -> R,
    ) -> Response {
        let id = self
            .id
            .unwrap_or_else(|| ui.id().with(("armas_table", ui.id())));

        ui.vertical(|ui| {
            let mut table_builder = TableBuilder {
                ui,
                theme,
                id,
                striped: self.striped,
                gap: self.gap,
                min_col_width: self.min_col_width,
                auto_width: self.auto_width,
                num_columns: 0,
                current_row: 0,
            };

            builder(&mut table_builder);
        })
        .response
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for adding headers and rows to the table
pub struct TableBuilder<'a> {
    ui: &'a mut Ui,
    theme: &'a Theme,
    id: egui::Id,
    striped: bool,
    gap: f32,
    min_col_width: Option<f32>,
    auto_width: bool,
    num_columns: usize,
    current_row: usize,
}

impl<'a> TableBuilder<'a> {
    /// Add header row
    pub fn headers(&mut self, headers: &[&str]) {
        self.num_columns = headers.len();

        // Calculate column width
        let min_width = self.calculate_min_col_width();

        let grid = egui::Grid::new(self.id.with("headers"))
            .spacing([self.gap, self.gap])
            .striped(false);

        let grid = if min_width > 0.0 {
            grid.min_col_width(min_width)
        } else {
            grid
        };

        grid.show(self.ui, |ui| {
            for header in headers {
                ui.strong(*header);
            }
        });

        // Add separator line
        self.ui.separator();
    }

    /// Add a data row
    pub fn row<R>(&mut self, content: impl FnOnce(&mut RowBuilder) -> R) -> R {
        let min_width = self.calculate_min_col_width();

        let grid = egui::Grid::new(self.id.with(("row", self.current_row)))
            .spacing([self.gap, self.gap])
            .striped(self.striped);

        let grid = if min_width > 0.0 {
            grid.min_col_width(min_width)
        } else {
            grid
        };

        let result = grid.show(self.ui, |ui| {
            let mut row_builder = RowBuilder { ui, current_col: 0 };

            content(&mut row_builder)
        });

        self.current_row += 1;

        result.inner
    }

    fn calculate_min_col_width(&self) -> f32 {
        if let Some(explicit_width) = self.min_col_width {
            explicit_width
        } else if self.auto_width && self.num_columns > 0 {
            let available = self.ui.available_width();
            let total_gap = self.gap * (self.num_columns - 1) as f32;
            let col_width = (available - total_gap) / self.num_columns as f32;
            col_width.max(60.0) // Minimum 60px per column for tables
        } else {
            0.0
        }
    }
}

/// Builder for adding columns to a table row
pub struct RowBuilder<'a> {
    ui: &'a mut Ui,
    current_col: usize,
}

impl<'a> RowBuilder<'a> {
    /// Add a column to the current row
    pub fn col<R>(&mut self, content: impl FnOnce(&mut Ui) -> R) -> R {
        let result = content(self.ui);
        self.current_col += 1;
        result
    }

    /// Add a column with strong (bold) text
    pub fn col_strong(&mut self, text: &str) {
        self.ui.strong(text);
        self.current_col += 1;
    }

    /// Add a column with a label
    pub fn col_label(&mut self, text: &str) {
        self.ui.label(text);
        self.current_col += 1;
    }
}
