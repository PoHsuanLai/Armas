//! Table component following shadcn/ui design patterns
//!
//! A responsive table with minimal styling, borders, and hover states.

use crate::Theme;
use egui;

// Constants matching shadcn/ui spacing
const CELL_PADDING: f32 = 8.0; // p-2 = 0.5rem = 8px
const HEADER_HEIGHT: f32 = 40.0; // h-10 = 2.5rem = 40px
const CELL_SPACING: f32 = 0.0;

/// Get the current theme from UI context
fn get_theme(ui: &egui::Ui) -> Theme {
    ui.ctx()
        .data(|d| d.get_temp::<Theme>(egui::Id::new("armas_theme")).unwrap_or_else(Theme::dark))
}

/// Draw a horizontal border line spanning all columns
fn draw_full_width_border(ui: &mut egui::Ui, _theme: &Theme, num_columns: usize) {
    for _ in 0..num_columns {
        ui.add(egui::Separator::default().horizontal().spacing(0.0));
    }
    ui.end_row();
}

/// A table component following shadcn/ui design
///
/// # Example
///
/// ```rust,ignore
/// table(ui, |mut rows| {
///     header_row(&mut rows, |cells| {
///         cell(cells, "Name");
///         cell(cells, "Status");
///     });
///     row(&mut rows, |cells| {
///         cell(cells, "Alice");
///         cell(cells, "Active");
///     });
/// });
/// ```
pub fn table<R>(ui: &mut egui::Ui, content: impl FnOnce(&mut TableRows) -> R) -> R {
    let theme = get_theme(ui);

    egui::Grid::new(ui.id().with("table"))
        .spacing([CELL_SPACING, CELL_SPACING])
        .min_col_width(0.0)
        .show(ui, |ui| {
            let mut table_state = TableState {
                theme,
                num_columns: 0,
            };

            let mut rows = TableRows {
                ui,
                state: &mut table_state,
            };

            content(&mut rows)
        })
        .inner
}

/// Table state tracking
struct TableState {
    theme: Theme,
    num_columns: usize,
}

/// Builder for table rows
pub struct TableRows<'a> {
    ui: &'a mut egui::Ui,
    state: &'a mut TableState,
}

/// Builder for table cells
pub struct TableCells<'a> {
    ui: &'a mut egui::Ui,
    theme: &'a Theme,
    is_header: bool,
    cell_index: usize,
}

/// Add a header row to the table
pub fn header_row<R>(rows: &mut TableRows, content: impl FnOnce(&mut TableCells) -> R) -> R {
    let result = render_row(rows, true, content);

    rows.ui.end_row();

    // Draw border after header across full width
    draw_full_width_border(rows.ui, &rows.state.theme, rows.state.num_columns);

    result
}

/// Add a data row to the table
pub fn row<R>(rows: &mut TableRows, content: impl FnOnce(&mut TableCells) -> R) -> R {
    let result = render_row(rows, false, content);

    rows.ui.end_row();

    // Draw border after row across full width
    draw_full_width_border(rows.ui, &rows.state.theme, rows.state.num_columns);

    result
}

/// Render a single row (header or data)
fn render_row<R>(
    rows: &mut TableRows,
    is_header: bool,
    content: impl FnOnce(&mut TableCells) -> R,
) -> R {
    let mut cells = TableCells {
        ui: rows.ui,
        theme: &rows.state.theme,
        is_header,
        cell_index: 0,
    };

    let result = content(&mut cells);

    // Update column count from first row
    if rows.state.num_columns == 0 {
        rows.state.num_columns = cells.cell_index;
    }

    result
}


/// Add a text cell to the current row
pub fn cell(cells: &mut TableCells, text: impl Into<String>) {
    render_cell(cells, |ui, theme, is_header| {
        let text = text.into();
        let label = create_label(&text, theme, is_header);
        ui.add(label);
    });
}

/// Add a custom cell with custom content
pub fn cell_ui<R>(cells: &mut TableCells, content: impl FnOnce(&mut egui::Ui) -> R) -> R {
    render_cell(cells, |ui, _theme, _is_header| content(ui))
}

/// Render a single cell with padding and styling
fn render_cell<R>(
    cells: &mut TableCells,
    content: impl FnOnce(&mut egui::Ui, &Theme, bool) -> R,
) -> R {
    let frame = egui::Frame::new()
        .inner_margin(egui::Margin::same(CELL_PADDING as i8));

    let result = frame.show(cells.ui, |ui| {
        // Set consistent min height for all cells in the row
        let min_height = if cells.is_header {
            HEADER_HEIGHT - CELL_PADDING * 2.0
        } else {
            0.0
        };

        if min_height > 0.0 {
            ui.set_min_height(min_height);
        }

        // Use horizontal layout to center content vertically
        ui.horizontal_centered(|ui| {
            content(ui, cells.theme, cells.is_header)
        }).inner
    }).inner;

    cells.cell_index += 1;
    result
}

/// Create a styled label for table cell
fn create_label(text: &str, theme: &Theme, is_header: bool) -> egui::Label {
    if is_header {
        egui::Label::new(
            egui::RichText::new(text)
                .strong()
                .color(theme.foreground())
        )
    } else {
        egui::Label::new(
            egui::RichText::new(text)
                .color(theme.muted_foreground())
        )
    }
}

