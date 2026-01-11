//! Retro Grid Background
//!
//! Cyberpunk-style perspective grid with animated lines

use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{Color32, Pos2, Response, Stroke, Ui, Vec2};

/// Retro grid background component
///
/// Creates a perspective grid with optional animation
pub struct RetroGrid {
    width: f32,
    height: f32,
    grid_color: Color32,
    horizon_color: Color32,
    cell_size: f32,
    perspective_depth: f32,
    animate: bool,
    animation_speed: f32,

    // Internal state
    animation_offset: f32,
}

impl RetroGrid {
    /// Create a new retro grid
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            grid_color: Color32::from_rgba_unmultiplied(0, 255, 255, 80), // Cyan
            horizon_color: Color32::from_rgba_unmultiplied(255, 0, 255, 100), // Magenta
            cell_size: 50.0,
            perspective_depth: 0.6,
            animate: true,
            animation_speed: 20.0,
            animation_offset: 0.0,
        }
    }

    /// Set grid line color
    pub fn grid_color(mut self, color: Color32) -> Self {
        self.grid_color = color;
        self
    }

    /// Set horizon glow color
    pub fn horizon_color(mut self, color: Color32) -> Self {
        self.horizon_color = color;
        self
    }

    /// Set cell size
    pub fn cell_size(mut self, size: f32) -> Self {
        self.cell_size = size.max(10.0);
        self
    }

    /// Set perspective depth (0.0 to 1.0)
    pub fn perspective_depth(mut self, depth: f32) -> Self {
        self.perspective_depth = depth.clamp(0.0, 1.0);
        self
    }

    /// Enable/disable animation
    pub fn animate(mut self, enabled: bool) -> Self {
        self.animate = enabled;
        self
    }

    /// Set animation speed
    pub fn animation_speed(mut self, speed: f32) -> Self {
        self.animation_speed = speed;
        self
    }

    /// Show the retro grid
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        if self.animate {
            let dt = ui.input(|i| i.stable_dt);
            self.animation_offset += dt * self.animation_speed;
            if self.animation_offset > self.cell_size {
                self.animation_offset -= self.cell_size;
            }
            ui.ctx().request_repaint();
        }

        let (response, painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());

        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Horizon line is at 60% down the screen
            let horizon_y = rect.top() + rect.height() * 0.6;

            // Draw horizontal lines (perspective grid)
            let num_h_lines = ((rect.bottom() - horizon_y) / (self.cell_size / 4.0)) as usize;

            for i in 0..num_h_lines {
                let t = i as f32 / num_h_lines as f32;

                // Apply animation offset
                let animated_t = if self.animate {
                    (t + self.animation_offset / (self.cell_size * num_h_lines as f32)) % 1.0
                } else {
                    t
                };

                // Y position with perspective (exponential for better depth)
                let y = horizon_y
                    + (rect.bottom() - horizon_y) * animated_t.powf(1.0 - self.perspective_depth);

                // Line gets thicker closer to viewer
                let thickness = 1.0 + animated_t * 2.0;

                // Fade out near horizon
                let alpha = (animated_t.powf(0.5) * self.grid_color.a() as f32) as u8;
                let line_color = Color32::from_rgba_unmultiplied(
                    self.grid_color.r(),
                    self.grid_color.g(),
                    self.grid_color.b(),
                    alpha,
                );

                painter.line_segment(
                    [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
                    Stroke::new(thickness, line_color),
                );
            }

            // Draw vertical lines (converging to vanishing point)
            let vanishing_point = Pos2::new(rect.center().x, horizon_y);
            let num_v_lines = (rect.width() / self.cell_size) as usize;

            for i in 0..=num_v_lines {
                let t = i as f32 / num_v_lines as f32 - 0.5; // -0.5 to 0.5
                let x = rect.left() + (i as f32 * self.cell_size);

                // Bottom point on the viewport
                let bottom_x = x;
                let bottom_point = Pos2::new(bottom_x, rect.bottom());

                // Line thickness based on distance from center
                let distance_from_center = (t.abs() * 2.0).min(1.0);
                let thickness = 1.0 + (1.0 - distance_from_center) * 2.0;

                // Alpha fade for edges
                let alpha = ((1.0 - distance_from_center * 0.5) * self.grid_color.a() as f32) as u8;
                let line_color = Color32::from_rgba_unmultiplied(
                    self.grid_color.r(),
                    self.grid_color.g(),
                    self.grid_color.b(),
                    alpha,
                );

                painter.line_segment(
                    [vanishing_point, bottom_point],
                    Stroke::new(thickness, line_color),
                );
            }

            // Draw horizon glow
            let glow_height = 80.0;
            for i in 0..20 {
                let t = i as f32 / 20.0;
                let alpha = ((1.0 - t) * self.horizon_color.a() as f32 * 0.5) as u8;
                let glow_color = Color32::from_rgba_unmultiplied(
                    self.horizon_color.r(),
                    self.horizon_color.g(),
                    self.horizon_color.b(),
                    alpha,
                );

                let y_offset = t * glow_height;
                painter.line_segment(
                    [
                        Pos2::new(rect.left(), horizon_y - y_offset),
                        Pos2::new(rect.right(), horizon_y - y_offset),
                    ],
                    Stroke::new(2.0, glow_color),
                );
                painter.line_segment(
                    [
                        Pos2::new(rect.left(), horizon_y + y_offset),
                        Pos2::new(rect.right(), horizon_y + y_offset),
                    ],
                    Stroke::new(2.0, glow_color),
                );
            }

            // Draw bright horizon line
            painter.line_segment(
                [
                    Pos2::new(rect.left(), horizon_y),
                    Pos2::new(rect.right(), horizon_y),
                ],
                Stroke::new(3.0, self.horizon_color),
            );
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retro_grid_creation() {
        let grid = RetroGrid::new(800.0, 600.0);
        assert_eq!(grid.width, 800.0);
        assert_eq!(grid.height, 600.0);
        assert_eq!(grid.cell_size, 50.0);
    }

    #[test]
    fn test_retro_grid_config() {
        let grid = RetroGrid::new(800.0, 600.0)
            .cell_size(60.0)
            .perspective_depth(0.7)
            .animate(false);

        assert_eq!(grid.cell_size, 60.0);
        assert_eq!(grid.perspective_depth, 0.7);
        assert_eq!(grid.animate, false);
    }
}
