//! Moving Border Button
//!
//! Button with animated gradient border that travels around the edges

use armas::ext::ArmasContextExt;
use egui::{Color32, CornerRadius, Pos2, Response, Sense, Stroke, Ui, Vec2};
use std::f32::consts::PI;

/// Moving border button component
///
/// A button with an animated gradient border that continuously moves around the edges
pub struct MovingBorder {
    text: String,
    width: Option<f32>,
    height: f32,
    border_width: f32,
    border_colors: Vec<Color32>,
    background: Color32,
    text_color: Color32,
    corner_radius: f32,
    animation_speed: f32,
}

impl MovingBorder {
    /// Create a new moving border button
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            width: None,
            height: 40.0,
            border_width: 2.0,
            border_colors: vec![
                Color32::PLACEHOLDER, // Will use theme gradient
                Color32::PLACEHOLDER,
                Color32::PLACEHOLDER,
            ],
            background: Color32::PLACEHOLDER, // Will use theme.card()
            text_color: Color32::PLACEHOLDER, // Will use theme.foreground()
            corner_radius: 8.0,
            animation_speed: 1.0,
        }
    }

    /// Set button width (auto if not set)
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set button height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set border width
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = width.max(1.0);
        self
    }

    /// Set border gradient colors
    pub fn border_colors(mut self, colors: Vec<Color32>) -> Self {
        if !colors.is_empty() {
            self.border_colors = colors;
        }
        self
    }

    /// Set background color
    pub fn background(mut self, color: Color32) -> Self {
        self.background = color;
        self
    }

    /// Set text color
    pub fn text_color(mut self, color: Color32) -> Self {
        self.text_color = color;
        self
    }

    /// Set corner radius
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Set animation speed (default: 1.0)
    pub fn animation_speed(mut self, speed: f32) -> Self {
        self.animation_speed = speed.max(0.1);
        self
    }

    /// Show the button
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();

        // Use theme colors if not explicitly set
        let background = if self.background == Color32::PLACEHOLDER {
            theme.card()
        } else {
            self.background
        };
        let text_color = if self.text_color == Color32::PLACEHOLDER {
            theme.foreground()
        } else {
            self.text_color
        };

        // Setup gradient colors from theme if not set
        let border_colors = if self.border_colors[0] == Color32::PLACEHOLDER {
            vec![theme.primary(), theme.secondary(), theme.primary()]
        } else {
            self.border_colors.clone()
        };

        // Calculate animation offset from time
        let time = ui.input(|i| i.time) as f32;
        let animation_offset = (time * self.animation_speed) % (2.0 * PI);
        ui.ctx().request_repaint();

        // Calculate size
        let text_galley = ui.painter().layout_no_wrap(
            self.text.clone(),
            egui::FontId::proportional(14.0),
            text_color,
        );

        let width = self.width.unwrap_or(text_galley.rect.width() + 40.0);
        let size = Vec2::new(width, self.height);

        let (rect, response) = ui.allocate_exact_size(size, Sense::click());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let _visuals = ui.style().interact(&response);

            // Draw background
            painter.rect_filled(
                rect,
                CornerRadius::same(theme.spacing.corner_radius),
                background,
            );

            // Draw moving gradient border
            // We'll draw multiple gradient segments around the perimeter
            let segments = 20; // Number of segments for smooth gradient
            let perimeter = 2.0 * (rect.width() + rect.height());
            let segment_length = perimeter / segments as f32;

            for i in 0..segments {
                let t = (i as f32 / segments as f32) + (animation_offset / (2.0 * PI));
                let t = t % 1.0;

                // Calculate color based on position
                let color_index = (t * border_colors.len() as f32) as usize % border_colors.len();
                let next_color_index = (color_index + 1) % border_colors.len();
                let blend_factor = (t * border_colors.len() as f32) % 1.0;

                let color1 = border_colors[color_index];
                let color2 = border_colors[next_color_index];

                let blended_color = Color32::from_rgba_unmultiplied(
                    (color1.r() as f32 * (1.0 - blend_factor) + color2.r() as f32 * blend_factor)
                        as u8,
                    (color1.g() as f32 * (1.0 - blend_factor) + color2.g() as f32 * blend_factor)
                        as u8,
                    (color1.b() as f32 * (1.0 - blend_factor) + color2.b() as f32 * blend_factor)
                        as u8,
                    (color1.a() as f32 * (1.0 - blend_factor) + color2.a() as f32 * blend_factor)
                        as u8,
                );

                // Calculate position along perimeter
                let segment_start = i as f32 * segment_length;
                let segment_end = ((i + 1) as f32 * segment_length).min(perimeter);

                let start_pos = self.perimeter_position(rect, segment_start, perimeter);
                let end_pos = self.perimeter_position(rect, segment_end, perimeter);

                painter.line_segment(
                    [start_pos, end_pos],
                    Stroke::new(self.border_width, blended_color),
                );
            }

            // Draw text
            let text_pos = rect.center();
            painter.text(
                text_pos,
                egui::Align2::CENTER_CENTER,
                &self.text,
                egui::FontId::proportional(14.0),
                text_color,
            );

            // Add hover effect
            if response.hovered() {
                let on_surface = theme.foreground();
                painter.rect_filled(
                    rect,
                    CornerRadius::same(theme.spacing.corner_radius),
                    Color32::from_rgba_unmultiplied(
                        on_surface.r(),
                        on_surface.g(),
                        on_surface.b(),
                        10,
                    ),
                );
            }
        }

        response
    }

    /// Calculate position along perimeter
    fn perimeter_position(&self, rect: egui::Rect, distance: f32, _total_perimeter: f32) -> Pos2 {
        let width = rect.width();
        let height = rect.height();

        // Top edge
        if distance < width {
            return Pos2::new(rect.left() + distance, rect.top());
        }

        // Right edge
        let distance = distance - width;
        if distance < height {
            return Pos2::new(rect.right(), rect.top() + distance);
        }

        // Bottom edge
        let distance = distance - height;
        if distance < width {
            return Pos2::new(rect.right() - distance, rect.bottom());
        }

        // Left edge
        let distance = distance - width;
        Pos2::new(rect.left(), rect.bottom() - distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving_border_creation() {
        let button = MovingBorder::new("Click Me");
        assert_eq!(button.text, "Click Me");
        assert_eq!(button.height, 40.0);
    }

    #[test]
    fn test_moving_border_config() {
        let button = MovingBorder::new("Test")
            .width(200.0)
            .height(50.0)
            .border_width(3.0)
            .animation_speed(2.0);

        assert_eq!(button.width, Some(200.0));
        assert_eq!(button.height, 50.0);
        assert_eq!(button.border_width, 3.0);
        assert_eq!(button.animation_speed, 2.0);
    }

    #[test]
    fn test_border_colors() {
        let colors = vec![Color32::RED, Color32::GREEN, Color32::BLUE];
        let button = MovingBorder::new("Test").border_colors(colors.clone());
        assert_eq!(button.border_colors.len(), 3);
    }
}
