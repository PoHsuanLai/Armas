use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{Color32, Pos2, Rect, Ui, Vec2};

/// A feature item with icon, title, and description
#[derive(Clone)]
pub struct FeatureItem {
    /// Icon text (emoji or unicode symbol)
    pub icon: String,
    /// Feature title
    pub title: String,
    /// Feature description
    pub description: String,
    /// Optional custom icon color
    pub icon_color: Option<Color32>,
}

impl FeatureItem {
    /// Create a new feature item
    pub fn new(
        icon: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            icon: icon.into(),
            title: title.into(),
            description: description.into(),
            icon_color: None,
        }
    }

    /// Set custom icon color
    pub fn icon_color(mut self, color: Color32) -> Self {
        self.icon_color = Some(color);
        self
    }
}

/// Responsive feature grid with smart borders
///
/// Displays features in a grid layout with icons, titles, and descriptions.
/// Includes smart borders that only appear between items, not on edges.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::layout::FeatureGrid;
///
/// FeatureGrid::new()
///     .columns(3)
///     .show(ui, |grid| {
///         grid.feature("🚀", "Fast", "Lightning fast performance");
///         grid.feature("🎨", "Beautiful", "Gorgeous UI components");
///         grid.feature("🔒", "Secure", "Built with security in mind");
///     });
/// # }
/// ```
pub struct FeatureGrid {
    /// Number of columns (None = auto-calculate)
    columns: Option<usize>,
    /// Gap between items
    gap: f32,
    /// Show borders between items
    show_borders: bool,
    /// Hover effect
    hover_effect: bool,
    /// Icon size
    icon_size: f32,
}

impl Default for FeatureGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureGrid {
    /// Create a new feature grid
    pub fn new() -> Self {
        Self {
            columns: None,
            gap: 20.0,
            show_borders: true,
            hover_effect: true,
            icon_size: 32.0,
        }
    }

    /// Set the number of columns
    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = Some(columns.max(1));
        self
    }

    /// Set the gap between items
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Enable or disable borders
    pub fn show_borders(mut self, show: bool) -> Self {
        self.show_borders = show;
        self
    }

    /// Enable or disable hover effect
    pub fn hover_effect(mut self, hover: bool) -> Self {
        self.hover_effect = hover;
        self
    }

    /// Set the icon size
    pub fn icon_size(mut self, size: f32) -> Self {
        self.icon_size = size;
        self
    }

    /// Show the feature grid
    pub fn show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut GridBuilder) -> R) -> R {
        let theme = ui.ctx().armas_theme();
        let available_width = ui.available_width();

        let mut builder = GridBuilder {
            ui,
            columns: self.columns,
            gap: self.gap,
            show_borders: self.show_borders,
            hover_effect: self.hover_effect,
            icon_size: self.icon_size,
            theme,
            available_width,
            items: Vec::new(),
        };

        let result = content(&mut builder);

        builder.render();
        result
    }
}

/// Builder for adding features to the grid
pub struct GridBuilder<'a> {
    ui: &'a mut Ui,
    columns: Option<usize>,
    gap: f32,
    show_borders: bool,
    hover_effect: bool,
    icon_size: f32,
    theme: Theme,
    available_width: f32,
    items: Vec<FeatureItem>,
}

impl<'a> GridBuilder<'a> {
    /// Add a feature to the grid
    pub fn feature(
        &mut self,
        icon: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) {
        self.items.push(FeatureItem::new(icon, title, description));
    }

    /// Add a feature with custom icon color
    pub fn feature_with_color(
        &mut self,
        icon: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        icon_color: Color32,
    ) {
        self.items
            .push(FeatureItem::new(icon, title, description).icon_color(icon_color));
    }

    /// Render all features
    fn render(&mut self) {
        let columns = self.columns.unwrap_or_else(|| {
            // Auto-calculate based on available width
            let min_item_width = 250.0;
            ((self.available_width / min_item_width).floor() as usize).clamp(1, 4)
        });

        let rows = self.items.len().div_ceil(columns);
        let item_width = (self.available_width - (columns - 1) as f32 * self.gap) / columns as f32;

        // Allocate space for the grid
        let total_height = rows as f32 * 150.0 + (rows - 1) as f32 * self.gap;
        let (rect, _response) = self.ui.allocate_exact_size(
            Vec2::new(self.available_width, total_height),
            egui::Sense::hover(),
        );

        // Draw grid items
        for (index, item) in self.items.iter().enumerate() {
            let row = index / columns;
            let col = index % columns;

            let x = rect.min.x + col as f32 * (item_width + self.gap);
            let y = rect.min.y + row as f32 * (150.0 + self.gap);

            let item_rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(item_width, 150.0));

            // Check hover for this specific item
            let item_hovered = self.ui.rect_contains_pointer(item_rect);

            self.draw_feature_item(item_rect, item, item_hovered);

            // Draw borders (smart: only between items)
            if self.show_borders {
                self.draw_borders(item_rect, (row, col), (rows, columns));
            }
        }
    }

    /// Draw a single feature item
    fn draw_feature_item(&self, rect: Rect, item: &FeatureItem, is_hovered: bool) {
        let painter = self.ui.painter();

        // Background with hover effect
        let bg_color = if self.hover_effect && is_hovered {
            self.theme.accent()
        } else {
            Color32::TRANSPARENT
        };

        if bg_color != Color32::TRANSPARENT {
            painter.rect_filled(rect, 4.0, bg_color);
        }

        // Icon
        let icon_color = item.icon_color.unwrap_or(self.theme.primary());
        let icon_pos = Pos2::new(rect.min.x + 20.0, rect.min.y + 20.0);

        painter.text(
            icon_pos,
            egui::Align2::LEFT_TOP,
            &item.icon,
            egui::FontId::proportional(self.icon_size),
            icon_color,
        );

        // Title
        let title_pos = Pos2::new(rect.min.x + 20.0, rect.min.y + 60.0);
        painter.text(
            title_pos,
            egui::Align2::LEFT_TOP,
            &item.title,
            egui::FontId::proportional(18.0),
            self.theme.foreground(),
        );

        // Description
        let desc_pos = Pos2::new(rect.min.x + 20.0, rect.min.y + 85.0);
        let desc_width = rect.width() - 40.0;

        // Word wrap description
        let galley = painter.layout(
            item.description.clone(),
            egui::FontId::proportional(14.0),
            self.theme.muted_foreground(),
            desc_width,
        );

        painter.galley(desc_pos, galley, self.theme.muted_foreground());
    }

    /// Draw smart borders (only between items)
    fn draw_borders(&self, rect: Rect, position: (usize, usize), grid_size: (usize, usize)) {
        let (row, col) = position;
        let (rows, columns) = grid_size;
        let painter = self.ui.painter();
        let border_color = self.theme.border();
        let stroke = egui::Stroke::new(1.0, border_color);

        // Right border (not on last column)
        if col < columns - 1 {
            painter.line_segment(
                [
                    Pos2::new(rect.max.x + self.gap / 2.0, rect.min.y),
                    Pos2::new(rect.max.x + self.gap / 2.0, rect.max.y),
                ],
                stroke,
            );
        }

        // Bottom border (not on last row)
        if row < rows - 1 {
            painter.line_segment(
                [
                    Pos2::new(rect.min.x, rect.max.y + self.gap / 2.0),
                    Pos2::new(rect.max.x, rect.max.y + self.gap / 2.0),
                ],
                stroke,
            );
        }
    }
}
