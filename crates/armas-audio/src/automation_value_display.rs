//! Y-axis value range display for automation canvas
//!
//! Shows min/max values and grid markers on the vertical axis

use armas::ext::ArmasContextExt;
use armas::theme::Theme;
use egui::{FontId, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Y-axis value range display
pub struct ValueRangeDisplay {
    min_value: f32,
    max_value: f32,
    num_markers: usize,
}

impl ValueRangeDisplay {
    /// Create a new value range display
    pub fn new(min_value: f32, max_value: f32) -> Self {
        Self {
            min_value,
            max_value,
            num_markers: 5,
        }
    }

    /// Set number of value markers to show
    pub fn markers(mut self, count: usize) -> Self {
        self.num_markers = count;
        self
    }

    /// Show the value range display
    pub fn show(self, ui: &mut Ui, height: f32) -> Response {
        let width = 50.0;
        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

        if ui.is_rect_visible(rect) {
            let theme = ui.ctx().armas_theme();
            self.render(ui.painter(), &theme, rect);
        }

        response
    }

    /// Render the value labels
    fn render(&self, painter: &egui::Painter, theme: &Theme, rect: Rect) {
        let value_range = self.max_value - self.min_value;
        let step = value_range / (self.num_markers - 1) as f32;

        for i in 0..self.num_markers {
            let value = self.max_value - (i as f32 * step);
            let y = rect.min.y + (i as f32 * rect.height() / (self.num_markers - 1) as f32);

            // Draw value text
            let text = format!("{:.2}", value);
            painter.text(
                Pos2::new(rect.max.x - 5.0, y),
                egui::Align2::RIGHT_CENTER,
                &text,
                FontId::monospace(9.0),
                theme.muted_foreground(),
            );

            // Draw small tick mark
            painter.line_segment(
                [
                    Pos2::new(rect.max.x - 12.0, y),
                    Pos2::new(rect.max.x - 8.0, y),
                ],
                egui::Stroke::new(0.5, theme.border()),
            );
        }
    }
}
