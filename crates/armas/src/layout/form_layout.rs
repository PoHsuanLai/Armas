use egui::{Response, Ui};

/// Two-column form layout for label-value pairs
///
/// Optimized for forms, settings, and profile displays where you have
/// labels on the left and values on the right.
///
/// # Example
///
/// ```rust,no_run
/// use armas::layout::FormLayout;
///
/// FormLayout::new()
///     .label_width(100.0)
///     .gap(16.0)
///     .show(ui, |form| {
///         form.row("Name:", |ui| {
///             ui.text_edit_singleline(&mut name);
///         });
///         form.row("Email:", |ui| {
///             ui.label(email);
///         });
///     });
/// ```
pub struct FormLayout {
    label_width: Option<f32>,
    gap: f32,
    id: Option<egui::Id>,
}

impl FormLayout {
    /// Create a new form layout
    pub fn new() -> Self {
        Self {
            label_width: None,
            gap: 16.0,
            id: None,
        }
    }

    /// Set the width of the label column (default: auto-calculated as 30% of available width)
    pub fn label_width(mut self, width: f32) -> Self {
        self.label_width = Some(width);
        self
    }

    /// Set the gap between label and value columns
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Set a custom ID for the form (useful when you have multiple forms)
    pub fn id_source(mut self, id_source: impl std::hash::Hash) -> Self {
        self.id = Some(egui::Id::new(id_source));
        self
    }

    /// Show the form layout with the given content builder
    pub fn show<R>(self, ui: &mut Ui, builder: impl FnOnce(&mut FormBuilder) -> R) -> Response {
        let id = self
            .id
            .unwrap_or_else(|| ui.id().with(("armas_form", ui.id())));

        let available = ui.available_width();
        let label_width = self.label_width.unwrap_or_else(|| {
            // Default: 30% for labels, 70% for values
            (available * 0.3).min(200.0).max(80.0)
        });

        let grid = egui::Grid::new(id)
            .spacing([self.gap, self.gap])
            .min_col_width(label_width);

        grid.show(ui, |ui| {
            let mut form_builder = FormBuilder { ui, label_width };

            builder(&mut form_builder);
        })
        .response
    }
}

impl Default for FormLayout {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for adding rows to the form
pub struct FormBuilder<'a> {
    ui: &'a mut Ui,
    label_width: f32,
}

impl<'a> FormBuilder<'a> {
    /// Add a row with a label and value content
    pub fn row<R>(&mut self, label: &str, content: impl FnOnce(&mut Ui) -> R) -> R {
        // Label column
        self.ui.label(label);

        // Value column
        let result = content(self.ui);

        self.ui.end_row();

        result
    }

    /// Add a row with a strong (bold) label
    pub fn row_strong<R>(&mut self, label: &str, content: impl FnOnce(&mut Ui) -> R) -> R {
        // Label column with strong text
        self.ui.strong(label);

        // Value column
        let result = content(self.ui);

        self.ui.end_row();

        result
    }

    /// Add a full-width row (spans both columns)
    pub fn full_row<R>(&mut self, content: impl FnOnce(&mut Ui) -> R) -> R {
        // Skip label column
        self.ui.label("");

        // Full-width content
        let result = content(self.ui);

        self.ui.end_row();

        result
    }
}
