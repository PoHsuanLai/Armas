//! Grid Pattern Background
//!
//! Creates an infinite grid with optional perspective and fade effects

use armas::ext::ArmasContextExt;
use egui::{Color32, Painter, Pos2, Rect, Response, Stroke, Ui, Vec2};

/// Grid pattern background effect
///
/// Creates an infinite grid with optional perspective distortion,
/// fade at distance, and dots at intersections.
pub struct GridPattern {
    /// Grid cell spacing
    spacing: f32,
    /// Line color
    color: Option<Color32>,
    /// Dot color (if enabled)
    dot_color: Option<Color32>,
    /// Fade distance (0.0 = no fade, 1.0 = full fade at edges)
    fade_distance: f32,
    /// Enable perspective effect
    perspective: bool,
    /// Line thickness
    thickness: f32,
    /// Dot radius (if dots enabled)
    dot_radius: f32,
    /// Width and height
    width: Option<f32>,
    height: Option<f32>,
}

impl Default for GridPattern {
    fn default() -> Self {
        Self::new()
    }
}

impl GridPattern {
    /// Create a new grid pattern with default settings
    pub fn new() -> Self {
        Self {
            spacing: 50.0,
            color: None,
            dot_color: None,
            fade_distance: 0.3,
            perspective: false,
            thickness: 1.0,
            dot_radius: 2.0,
            width: None,
            height: None,
        }
    }

    /// Set the width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the height
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set the grid spacing
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the grid line color
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Enable dots at intersections
    pub fn dots(mut self, color: Color32, radius: f32) -> Self {
        self.dot_color = Some(color);
        self.dot_radius = radius;
        self
    }

    /// Set fade distance (0.0 to 1.0)
    pub fn fade(mut self, fade: f32) -> Self {
        self.fade_distance = fade.clamp(0.0, 1.0);
        self
    }

    /// Enable perspective effect (3D-like grid)
    pub fn perspective(mut self, enabled: bool) -> Self {
        self.perspective = enabled;
        self
    }

    /// Set line thickness
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Show the grid pattern, allocating layout space.
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();
        let outline = theme.border();
        let color = self.color.unwrap_or_else(|| {
            Color32::from_rgba_unmultiplied(outline.r(), outline.g(), outline.b(), 50)
        });

        let size = if self.width.is_some() || self.height.is_some() {
            Vec2::new(self.width.unwrap_or(400.0), self.height.unwrap_or(300.0))
        } else {
            ui.available_size()
        };

        let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
        self.paint_at(&painter, response.rect, color);
        response
    }

    /// Paint the grid pattern onto a rect without allocating layout space.
    ///
    /// Use this to render the pattern as a background behind other content.
    pub fn paint(&self, ui: &Ui, rect: Rect) {
        let theme = ui.ctx().armas_theme();
        let outline = theme.border();
        let color = self.color.unwrap_or_else(|| {
            Color32::from_rgba_unmultiplied(outline.r(), outline.g(), outline.b(), 50)
        });
        self.paint_at(ui.painter(), rect, color);
    }

    fn paint_at(&self, painter: &Painter, bounds: Rect, color: Color32) {
        let center = bounds.center();
        let left = bounds.left();
        let right = bounds.right();
        let top = bounds.top();
        let bottom = bounds.bottom();

        // Draw vertical lines
        let mut x = center.x % self.spacing;
        while x < bounds.width() {
            let line_x = left + x;

            let alpha = if self.fade_distance > 0.0 {
                let distance_from_center = (line_x - center.x).abs() / (bounds.width() / 2.0);
                let fade_factor = 1.0 - (distance_from_center / self.fade_distance).min(1.0);
                (color.a() as f32 * fade_factor) as u8
            } else {
                color.a()
            };

            let line_color =
                Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);

            if self.perspective {
                let convergence = 0.1;
                let top_offset = (line_x - center.x) * convergence * 0.3;
                let bottom_offset = (line_x - center.x) * convergence;

                painter.line_segment(
                    [
                        Pos2::new(line_x - top_offset, top),
                        Pos2::new(line_x - bottom_offset, bottom),
                    ],
                    Stroke::new(self.thickness, line_color),
                );
            } else {
                painter.line_segment(
                    [Pos2::new(line_x, top), Pos2::new(line_x, bottom)],
                    Stroke::new(self.thickness, line_color),
                );
            }

            x += self.spacing;
        }

        // Draw horizontal lines
        let mut y = center.y % self.spacing;
        while y < bounds.height() {
            let line_y = top + y;

            let alpha = if self.fade_distance > 0.0 {
                let distance_from_center = (line_y - center.y).abs() / (bounds.height() / 2.0);
                let fade_factor = 1.0 - (distance_from_center / self.fade_distance).min(1.0);
                (color.a() as f32 * fade_factor) as u8
            } else {
                color.a()
            };

            let line_color =
                Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);

            if self.perspective {
                let scale = 0.5 + 0.5 * (line_y - top) / bounds.height();
                let thickness = self.thickness * scale;

                painter.line_segment(
                    [Pos2::new(left, line_y), Pos2::new(right, line_y)],
                    Stroke::new(thickness, line_color),
                );
            } else {
                painter.line_segment(
                    [Pos2::new(left, line_y), Pos2::new(right, line_y)],
                    Stroke::new(self.thickness, line_color),
                );
            }

            y += self.spacing;
        }

        // Draw dots at intersections if enabled
        if let Some(dot_color) = self.dot_color {
            let mut x = center.x % self.spacing;
            while x < bounds.width() {
                let mut y = center.y % self.spacing;
                while y < bounds.height() {
                    let dot_x = left + x;
                    let dot_y = top + y;

                    let alpha = if self.fade_distance > 0.0 {
                        let distance_x = (dot_x - center.x).abs() / (bounds.width() / 2.0);
                        let distance_y = (dot_y - center.y).abs() / (bounds.height() / 2.0);
                        let distance = (distance_x.powi(2) + distance_y.powi(2)).sqrt();
                        let fade_factor = 1.0 - (distance / self.fade_distance).min(1.0);
                        (dot_color.a() as f32 * fade_factor) as u8
                    } else {
                        dot_color.a()
                    };

                    let faded_dot_color = Color32::from_rgba_unmultiplied(
                        dot_color.r(),
                        dot_color.g(),
                        dot_color.b(),
                        alpha,
                    );

                    if self.perspective {
                        let bottom_offset = (dot_x - center.x) * 0.1;
                        painter.circle_filled(
                            Pos2::new(dot_x - bottom_offset, dot_y),
                            self.dot_radius,
                            faded_dot_color,
                        );
                    } else {
                        painter.circle_filled(
                            Pos2::new(dot_x, dot_y),
                            self.dot_radius,
                            faded_dot_color,
                        );
                    }

                    y += self.spacing;
                }
                x += self.spacing;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = GridPattern::new();
        assert_eq!(grid.spacing, 50.0);
        assert!(grid.color.is_none());
    }

    #[test]
    fn test_grid_config() {
        let grid = GridPattern::new()
            .width(800.0)
            .height(600.0)
            .spacing(50.0)
            .perspective(true)
            .fade(0.5);

        assert_eq!(grid.width, Some(800.0));
        assert_eq!(grid.height, Some(600.0));
        assert_eq!(grid.spacing, 50.0);
        assert!(grid.perspective);
        assert_eq!(grid.fade_distance, 0.5);
    }
}
