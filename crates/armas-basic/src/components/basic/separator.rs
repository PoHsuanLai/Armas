//! Separator Component (shadcn/ui style)
//!
//! Simple horizontal or vertical divider line.

use egui::{Response, Ui, Vec2};

/// Separator orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SeparatorOrientation {
    #[default]
    /// Horizontal separator
    Horizontal,
    /// Vertical separator
    Vertical,
}

/// Simple separator/divider component
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas_basic::Separator;
/// use armas_basic::ext::ArmasContextExt;
///
/// let theme = ui.ctx().armas_theme();
/// // Horizontal separator (default)
/// Separator::new().show(ui, &theme);
///
/// // Vertical separator
/// Separator::new().vertical().show(ui, &theme);
/// # }
/// ```
pub struct Separator {
    orientation: SeparatorOrientation,
    length: Option<f32>,
}

impl Separator {
    /// Create a new horizontal separator
    #[must_use] 
    pub const fn new() -> Self {
        Self {
            orientation: SeparatorOrientation::Horizontal,
            length: None,
        }
    }

    /// Set horizontal orientation
    #[must_use] 
    pub const fn horizontal(mut self) -> Self {
        self.orientation = SeparatorOrientation::Horizontal;
        self
    }

    /// Set vertical orientation
    #[must_use] 
    pub const fn vertical(mut self) -> Self {
        self.orientation = SeparatorOrientation::Vertical;
        self
    }

    /// Set custom length (width for horizontal, height for vertical)
    #[must_use] 
    pub const fn length(mut self, length: f32) -> Self {
        self.length = Some(length);
        self
    }

    /// Show the separator
    pub fn show(self, ui: &mut Ui, theme: &crate::Theme) -> Response {
        let color = theme.border();

        let size = match self.orientation {
            SeparatorOrientation::Horizontal => {
                let width = self.length.unwrap_or_else(|| ui.available_width());
                Vec2::new(width, 1.0)
            }
            SeparatorOrientation::Vertical => {
                let height = self.length.unwrap_or_else(|| ui.available_height());
                Vec2::new(1.0, height)
            }
        };

        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.painter().rect_filled(rect, 0.0, color);
        }

        response
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self::new()
    }
}
