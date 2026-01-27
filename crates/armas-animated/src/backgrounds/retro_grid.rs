//! Retro Grid Background
//!
//! Cyberpunk-style perspective grid with animated lines

use egui::{Color32, Painter, Pos2, Rect, Response, Stroke, Ui, Vec2};

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
}

impl RetroGrid {
    /// Create a new retro grid
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            // More subtle, polished colors
            grid_color: Color32::from_rgba_unmultiplied(100, 200, 255, 60), // Subtle cyan
            horizon_color: Color32::from_rgba_unmultiplied(200, 100, 255, 80), // Subtle purple
            cell_size: 50.0,
            perspective_depth: 0.6,
            animate: true,
            animation_speed: 15.0, // Slightly slower for smoothness
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

    /// Show the retro grid, allocating layout space.
    pub fn show(&self, ui: &mut Ui) -> Response {
        let time = ui.input(|i| i.time) as f32;

        if self.animate {
            ui.ctx().request_repaint();
        }

        let (response, painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());

        if ui.is_rect_visible(response.rect) {
            self.paint_at(&painter, response.rect, time);
        }

        response
    }

    /// Paint the retro grid onto a rect without allocating layout space.
    ///
    /// Use this to render the grid as a background behind other content.
    pub fn paint(&self, ui: &Ui, rect: Rect) {
        let time = ui.input(|i| i.time) as f32;

        if self.animate {
            ui.ctx().request_repaint();
        }

        if ui.is_rect_visible(rect) {
            self.paint_at(ui.painter(), rect, time);
        }
    }

    fn paint_at(&self, painter: &Painter, rect: Rect, time: f32) {
        let animation_offset = if self.animate {
            (time * self.animation_speed) % self.cell_size
        } else {
            0.0
        };

        let horizon_y = rect.top() + rect.height() * 0.6;

        // Draw horizontal lines (perspective grid)
        let num_h_lines = ((rect.bottom() - horizon_y) / (self.cell_size / 4.0)) as usize;

        for i in 0..num_h_lines {
            let t = i as f32 / num_h_lines as f32;

            let animated_t = if self.animate {
                (t + animation_offset / (self.cell_size * num_h_lines as f32)) % 1.0
            } else {
                t
            };

            let y = horizon_y
                + (rect.bottom() - horizon_y) * animated_t.powf(1.0 - self.perspective_depth);

            let thickness = 1.0 + animated_t * 2.0;

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
            let t = i as f32 / num_v_lines as f32 - 0.5;
            let x = rect.left() + (i as f32 * self.cell_size);

            let bottom_point = Pos2::new(x, rect.bottom());

            let distance_from_center = (t.abs() * 2.0).min(1.0);
            let thickness = 1.0 + (1.0 - distance_from_center) * 2.0;

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
        assert!(!grid.animate);
    }
}
