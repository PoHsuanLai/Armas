//! Point handle for automation point interaction
//!
//! Interactive circle handles for selecting, dragging, and editing automation points

use crate::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

const POINT_RADIUS: f32 = 6.0;
const HOVER_RADIUS: f32 = 10.0;

/// Response from point handle interaction
#[derive(Debug, Clone)]
pub struct PointHandleResponse {
    pub response: Response,
    pub is_selected: bool,
    pub is_hovered: bool,
    pub drag_delta: Option<Vec2>,
}

/// Interactive automation point handle
pub struct PointHandle {
    position: Pos2,
    color: Color32,
    selected: bool,
    show_value: bool,
    value_text: Option<String>,
}

impl PointHandle {
    /// Create a new point handle
    pub fn new(position: Pos2, color: Color32) -> Self {
        Self {
            position,
            color,
            selected: false,
            show_value: false,
            value_text: None,
        }
    }

    /// Mark this handle as selected
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Show value label on hover
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Set the value text to display
    pub fn value_text(mut self, text: impl Into<String>) -> Self {
        self.value_text = Some(text.into());
        self
    }

    /// Show the point handle
    pub fn show(self, ui: &mut Ui, theme: &Theme) -> PointHandleResponse {
        let handle_rect = Rect::from_center_size(self.position, Vec2::splat(HOVER_RADIUS * 2.0));

        let response = ui.allocate_rect(handle_rect, Sense::click_and_drag());
        let is_hovered = response.hovered();
        let drag_delta = response.drag_delta();

        // Render the point
        self.render(ui.painter(), is_hovered, theme);

        // Render value label if hovering
        if is_hovered && self.show_value {
            if let Some(value_text) = &self.value_text {
                ui.painter().text(
                    self.position + Vec2::new(15.0, -20.0),
                    egui::Align2::LEFT_BOTTOM,
                    value_text,
                    egui::FontId::monospace(10.0),
                    theme.foreground(),
                );
            }
        }

        PointHandleResponse {
            response,
            is_selected: self.selected,
            is_hovered,
            drag_delta: if drag_delta.length() > 0.0 {
                Some(drag_delta)
            } else {
                None
            },
        }
    }

    /// Render the point handle
    fn render(&self, painter: &egui::Painter, is_hovered: bool, theme: &Theme) {
        let radius = if is_hovered { HOVER_RADIUS } else { POINT_RADIUS };

        // Draw outer glow rings (enhanced multi-layer glow)
        if self.selected {
            // Strong glow for selected state
            for i in 0..3 {
                let glow_radius = radius + (i + 1) as f32 * 3.5;
                let alpha = ((1.0 - (i as f32 / 3.0)) * 40.0) as u8;
                let glow_color = Color32::from_rgba_unmultiplied(
                    self.color.r(),
                    self.color.g(),
                    self.color.b(),
                    alpha,
                );

                painter.circle_stroke(
                    self.position,
                    glow_radius,
                    egui::Stroke::new(0.8, glow_color),
                );
            }
        } else if is_hovered {
            // Subtle glow for hover state
            for i in 0..2 {
                let glow_radius = radius + (i + 1) as f32 * 2.5;
                let alpha = ((1.0 - (i as f32 / 2.0)) * 25.0) as u8;
                let glow_color = Color32::from_rgba_unmultiplied(
                    self.color.r(),
                    self.color.g(),
                    self.color.b(),
                    alpha,
                );

                painter.circle_stroke(
                    self.position,
                    glow_radius,
                    egui::Stroke::new(0.6, glow_color),
                );
            }
        }

        // Draw main circle
        let fill_color = if is_hovered {
            self.color
        } else {
            self.color.gamma_multiply(0.95)
        };
        painter.circle_filled(self.position, radius, fill_color);

        // Draw border with thickness based on state
        let (border_color, border_width) = if self.selected {
            (theme.primary(), 2.0)
        } else if is_hovered {
            (theme.primary().gamma_multiply(0.8), 1.75)
        } else {
            (Color32::WHITE.gamma_multiply(0.6), 1.25)
        };

        painter.circle_stroke(
            self.position,
            radius,
            egui::Stroke::new(border_width, border_color),
        );

        // Draw concentric inner ring for selected state (flat design accent)
        if self.selected {
            let inner_ring_radius = radius * 0.55;
            let ring_color = self.color.gamma_multiply(0.7);

            painter.circle_stroke(
                self.position,
                inner_ring_radius,
                egui::Stroke::new(1.0, ring_color),
            );
        }
    }
}
