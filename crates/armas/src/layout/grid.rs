use crate::layout::{calc_flex_widths, FlexItem};
use egui::{Response, Ui};

/// Grid column template (similar to CSS Grid template-columns)
#[derive(Debug, Clone, PartialEq)]
pub enum GridTemplate {
    /// Equal columns (e.g., 3 columns)
    Columns(usize),
    /// Custom column widths using flex specification (e.g., [Fixed(200), Flex(1), Flex(2)])
    Template(Vec<FlexItem>),
    /// Repeat pattern (e.g., repeat 3 times with [Flex(1)])
    Repeat(usize, Vec<FlexItem>),
}

impl GridTemplate {
    /// Get the number of columns
    pub fn column_count(&self) -> usize {
        match self {
            GridTemplate::Columns(n) => *n,
            GridTemplate::Template(items) => items.len(),
            GridTemplate::Repeat(count, pattern) => count * pattern.len(),
        }
    }

    /// Get the flex items for column calculation
    pub fn flex_items(&self) -> Vec<FlexItem> {
        match self {
            GridTemplate::Columns(n) => vec![FlexItem::Flex(1.0); *n],
            GridTemplate::Template(items) => items.clone(),
            GridTemplate::Repeat(count, pattern) => pattern
                .iter()
                .cloned()
                .cycle()
                .take(count * pattern.len())
                .collect(),
        }
    }
}

/// Responsive grid layout
///
/// Arranges children in a grid with specified number of columns.
/// Automatically wraps to new rows. Uses egui's Grid internally.
///
/// # Example
///
/// ```rust,no_run
/// use armas::layout::Grid;
///
/// Grid::new(3)  // 3 columns
///     .gap(16.0)
///     .show(ui, |grid| {
///         for i in 0..9 {
///             grid.cell(|ui| {
///                 ui.label(format!("Item {}", i));
///             });
///         }
///     });
/// ```
///
/// Template-based grid:
/// ```rust,no_run
/// use armas::layout::{Grid, GridTemplate, FlexItem};
///
/// Grid::with_template(GridTemplate::Template(vec![
///     FlexItem::Fixed(200.0),  // Sidebar
///     FlexItem::Flex(1.0),     // Main content
///     FlexItem::Fixed(150.0),  // Right panel
/// ]))
/// .show(ui, |grid| {
///     grid.cell(|ui| { ui.label("Sidebar"); });
///     grid.cell(|ui| { ui.label("Content"); });
///     grid.cell(|ui| { ui.label("Panel"); });
/// });
/// ```
pub struct Grid {
    template: GridTemplate,
    gap: f32,
    striped: bool,
    id: Option<egui::Id>,
}

impl Grid {
    /// Create a new grid with the specified number of equal columns
    pub fn new(columns: usize) -> Self {
        Self {
            template: GridTemplate::Columns(columns.max(1)),
            gap: 16.0,
            striped: false,
            id: None,
        }
    }

    /// Create a new grid with a custom template
    pub fn with_template(template: GridTemplate) -> Self {
        Self {
            template,
            gap: 16.0,
            striped: false,
            id: None,
        }
    }

    /// Set the gap between items (both horizontal and vertical)
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Enable striped rows
    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    /// Set a custom ID for the grid (useful when you have multiple grids)
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set a custom ID from a source string
    pub fn id_source(mut self, id_source: impl std::hash::Hash) -> Self {
        self.id = Some(egui::Id::new(id_source));
        self
    }

    /// Show the grid with the given content builder
    pub fn show<R>(self, ui: &mut Ui, builder: impl FnOnce(&mut GridBuilder) -> R) -> Response {
        // Use custom ID if provided, otherwise generate a unique one
        let id = self
            .id
            .unwrap_or_else(|| ui.id().with(("armas_grid", ui.id())));

        let columns = self.template.column_count();
        let flex_items = self.template.flex_items();

        // Calculate column widths using flex system
        let available_width = ui.available_width();
        let col_widths = calc_flex_widths(available_width, self.gap, &flex_items);

        // Use egui's built-in Grid for proper horizontal layout
        egui::Grid::new(id)
            .spacing([self.gap, self.gap])
            .striped(self.striped)
            .show(ui, |egui_ui| {
                let mut grid_builder = GridBuilder {
                    ui: egui_ui,
                    columns,
                    current_col: 0,
                    col_widths: col_widths.clone(),
                    gap: self.gap,
                    striped: self.striped,
                    current_row: 0,
                };

                builder(&mut grid_builder);
            })
            .response
    }
}

/// Builder for adding cells to the grid
pub struct GridBuilder<'a> {
    ui: &'a mut Ui,
    columns: usize,
    current_col: usize,
    col_widths: Vec<f32>,
    gap: f32,
    striped: bool,
    current_row: usize,
}

impl<'a> GridBuilder<'a> {
    /// Add a cell to the grid
    pub fn cell<R>(&mut self, content: impl FnOnce(&mut Ui) -> R) -> R {
        // Get width for current column
        let width = self
            .col_widths
            .get(self.current_col)
            .copied()
            .unwrap_or(100.0);

        // Allocate space for this cell (egui::Grid handles horizontal layout)
        let result = self
            .ui
            .allocate_ui(egui::Vec2::new(width, self.ui.available_height()), |ui| {
                content(ui)
            })
            .inner;

        self.current_col += 1;

        // End row if we've filled all columns
        if self.current_col >= self.columns {
            self.ui.end_row();
            self.current_col = 0;
            self.current_row += 1;
        }

        result
    }

    /// Start a new row (skip to next row)
    pub fn end_row(&mut self) {
        if self.current_col > 0 {
            self.ui.end_row();
            self.current_col = 0;
            self.current_row += 1;
        }
    }
}
