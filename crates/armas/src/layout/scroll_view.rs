use egui::{Response, Ui};

/// Custom scrollable area with Armas styling
///
/// A themed wrapper around egui's ScrollArea.
///
/// # Example
///
/// ```rust,no_run
/// use armas::layout::ScrollView;
///
/// ScrollView::vertical()
///     .max_height(400.0)
///     .show(ui, |ui| {
///         // Scrollable content
///         for i in 0..100 {
///             ui.label(format!("Item {}", i));
///         }
///     });
/// ```
pub struct ScrollView {
    direction: Direction,
    max_width: Option<f32>,
    max_height: Option<f32>,
    auto_shrink: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Vertical,
    Horizontal,
    Both,
}

impl ScrollView {
    /// Create a vertical scroll view
    pub fn vertical() -> Self {
        Self {
            direction: Direction::Vertical,
            max_width: None,
            max_height: None,
            auto_shrink: true,
        }
    }

    /// Create a horizontal scroll view
    pub fn horizontal() -> Self {
        Self {
            direction: Direction::Horizontal,
            max_width: None,
            max_height: None,
            auto_shrink: true,
        }
    }

    /// Create a scroll view that scrolls both directions
    pub fn both() -> Self {
        Self {
            direction: Direction::Both,
            max_width: None,
            max_height: None,
            auto_shrink: true,
        }
    }

    /// Set maximum width
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Set maximum height
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }

    /// Enable/disable auto-shrinking (default: true)
    pub fn auto_shrink(mut self, shrink: bool) -> Self {
        self.auto_shrink = shrink;
        self
    }

    /// Show the scroll view with the given content
    pub fn show<R>(
        self,
        ui: &mut Ui,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> egui::scroll_area::ScrollAreaOutput<R> {
        let mut scroll_area = match self.direction {
            Direction::Vertical => egui::ScrollArea::vertical(),
            Direction::Horizontal => egui::ScrollArea::horizontal(),
            Direction::Both => egui::ScrollArea::both(),
        };

        if let Some(w) = self.max_width {
            scroll_area = scroll_area.max_width(w);
        }
        if let Some(h) = self.max_height {
            scroll_area = scroll_area.max_height(h);
        }

        scroll_area.auto_shrink(self.auto_shrink).show(ui, content)
    }
}

impl Default for ScrollView {
    fn default() -> Self {
        Self::vertical()
    }
}
