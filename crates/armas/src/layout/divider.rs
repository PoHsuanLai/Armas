use crate::Theme;
use egui::{Response, Sense, Stroke, Ui, Vec2};

/// Horizontal or vertical divider line
///
/// A themed separator line, inspired by SwiftUI's Divider.
///
/// # Example
///
/// ```rust,no_run
/// use armas::layout::Divider;
/// use armas::Theme;
///
/// let theme = Theme::dark();
///
/// // Horizontal divider
/// Divider::horizontal().show(ui, &theme);
///
/// // Vertical divider with custom height
/// Divider::vertical().height(100.0).show(ui, &theme);
/// ```
pub struct Divider {
    orientation: Orientation,
    thickness: f32,
    length: Option<f32>,
    color: Option<egui::Color32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Orientation {
    Horizontal,
    Vertical,
}

impl Divider {
    /// Create a horizontal divider
    pub fn horizontal() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            thickness: 1.0,
            length: None,
            color: None,
        }
    }

    /// Create a vertical divider
    pub fn vertical() -> Self {
        Self {
            orientation: Orientation::Vertical,
            thickness: 1.0,
            length: None,
            color: None,
        }
    }

    /// Set the thickness of the divider (default: 1.0)
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set the width (for horizontal) or height (for vertical)
    pub fn length(mut self, length: f32) -> Self {
        self.length = Some(length);
        self
    }

    /// Set width for horizontal dividers
    pub fn width(self, width: f32) -> Self {
        self.length(width)
    }

    /// Set height for vertical dividers
    pub fn height(self, height: f32) -> Self {
        self.length(height)
    }

    /// Set custom color (overrides theme)
    pub fn color(mut self, color: egui::Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Show the divider
    pub fn show(self, ui: &mut Ui, theme: &Theme) -> Response {
        let color = self.color.unwrap_or_else(|| theme.outline());

        let (width, height) = match self.orientation {
            Orientation::Horizontal => {
                let w = self.length.unwrap_or(ui.available_width());
                (w, self.thickness)
            }
            Orientation::Vertical => {
                let h = self.length.unwrap_or(ui.available_height().max(20.0));
                (self.thickness, h)
            }
        };

        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

        if ui.is_rect_visible(rect) {
            let stroke = Stroke::new(self.thickness, color);

            match self.orientation {
                Orientation::Horizontal => {
                    ui.painter()
                        .line_segment([rect.left_center(), rect.right_center()], stroke);
                }
                Orientation::Vertical => {
                    ui.painter()
                        .line_segment([rect.center_top(), rect.center_bottom()], stroke);
                }
            }
        }

        response
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::horizontal()
    }
}
