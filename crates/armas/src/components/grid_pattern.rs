//! Grid Pattern Background
//!
//! Creates an infinite grid with optional perspective and fade effects

use crate::Theme;
use egui::{Color32, Pos2, Response, Stroke, Ui, Vec2};

/// Grid pattern background effect
///
/// Creates an infinite grid with optional perspective distortion,
/// fade at distance, and dots at intersections.
pub struct GridPattern {
    /// Grid cell spacing
    spacing: f32,
    /// Line color
    color: Color32,
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
    width: f32,
    height: f32,
}

impl GridPattern {
    /// Create a new grid pattern with theme-based defaults
    pub fn new(width: f32, height: f32, spacing: f32, _theme: &Theme) -> Self {
        // Subtle but visible: slightly lighter than Aceternity for better visibility
        Self {
            spacing,
            color: Color32::from_rgb(50, 50, 50), // Slightly lighter than #262626
            dot_color: None,
            fade_distance: 0.3,
            perspective: false,
            thickness: 0.8, // Thin but visible
            dot_radius: 2.0,
            width,
            height,
        }
    }

    /// Set the grid line color
    pub fn with_color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    /// Enable dots at intersections
    pub fn with_dots(mut self, color: Color32, radius: f32) -> Self {
        self.dot_color = Some(color);
        self.dot_radius = radius;
        self
    }

    /// Set fade distance (0.0 to 1.0)
    pub fn with_fade(mut self, fade: f32) -> Self {
        self.fade_distance = fade.clamp(0.0, 1.0);
        self
    }

    /// Enable perspective effect (3D-like grid)
    pub fn with_perspective(mut self, enabled: bool) -> Self {
        self.perspective = enabled;
        self
    }

    /// Set line thickness
    pub fn with_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Show the grid pattern
    pub fn show(&self, ui: &mut Ui) -> Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());

        let bounds = response.rect;
        let center = bounds.center();

        // Calculate grid bounds
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
                (self.color.a() as f32 * fade_factor) as u8
            } else {
                self.color.a()
            };

            let line_color = Color32::from_rgba_unmultiplied(
                self.color.r(),
                self.color.g(),
                self.color.b(),
                alpha,
            );

            if self.perspective {
                // Perspective effect: lines converge slightly at bottom
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
                (self.color.a() as f32 * fade_factor) as u8
            } else {
                self.color.a()
            };

            let line_color = Color32::from_rgba_unmultiplied(
                self.color.r(),
                self.color.g(),
                self.color.b(),
                alpha,
            );

            if self.perspective {
                // Perspective effect: horizontal lines get closer together at bottom
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

                    // Calculate fade for dot
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

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let theme = Theme::default();
        let grid = GridPattern::new(800.0, 600.0, 50.0, &theme);
        assert_eq!(grid.spacing, 50.0);
    }

    #[test]
    fn test_grid_config() {
        let theme = Theme::default();
        let grid = GridPattern::new(800.0, 600.0, 50.0, &theme)
            .with_perspective(true)
            .with_fade(0.5);

        assert!(grid.perspective);
        assert_eq!(grid.fade_distance, 0.5);
    }
}
