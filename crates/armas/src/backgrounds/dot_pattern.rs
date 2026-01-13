//! Dot Pattern Background
//!
//! Creates a simple dot grid pattern background effect

use crate::ext::ArmasContextExt;
use egui::{Color32, Pos2, Response, Ui, Vec2};

/// Dot pattern background effect
///
/// Creates a regular grid of dots with optional fade and glow effects
pub struct DotPattern {
    /// Spacing between dots
    spacing: f32,
    /// Dot radius
    dot_radius: f32,
    /// Dot color
    color: Option<Color32>,
    /// Fade distance (0.0 = no fade, 1.0 = full fade at edges)
    fade_distance: f32,
    /// Glow effect (adds larger, more transparent dots behind)
    glow: bool,
    /// Width and height
    width: Option<f32>,
    height: Option<f32>,
}

impl Default for DotPattern {
    fn default() -> Self {
        Self::new()
    }
}

impl DotPattern {
    /// Create a new dot pattern with default settings
    pub fn new() -> Self {
        Self {
            spacing: 20.0,
            dot_radius: 1.5,
            color: None,
            fade_distance: 0.0,
            glow: false,
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

    /// Set dot spacing
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing.max(1.0);
        self
    }

    /// Set dot radius
    pub fn dot_radius(mut self, radius: f32) -> Self {
        self.dot_radius = radius.max(0.5);
        self
    }

    /// Set dot color
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Enable fade effect at edges (0.0 to 1.0)
    pub fn fade(mut self, fade: f32) -> Self {
        self.fade_distance = fade.clamp(0.0, 1.0);
        self
    }

    /// Enable glow effect
    pub fn glow(mut self, enabled: bool) -> Self {
        self.glow = enabled;
        self
    }

    /// Show the dot pattern
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();
        // Apply theme defaults - use outline color for subtle dot pattern
        let color = self.color.unwrap_or_else(|| theme.outline());

        // Determine size
        let size = if self.width.is_some() || self.height.is_some() {
            Vec2::new(self.width.unwrap_or(400.0), self.height.unwrap_or(300.0))
        } else {
            ui.available_size()
        };

        let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());

        let bounds = response.rect;
        let center = bounds.center();

        // Calculate starting position to center the grid
        let start_x = bounds.left() + (bounds.width() % self.spacing) / 2.0;
        let start_y = bounds.top() + (bounds.height() % self.spacing) / 2.0;

        // Draw dots
        let mut y = 0.0;
        while start_y + y <= bounds.bottom() {
            let mut x = 0.0;
            while start_x + x <= bounds.right() {
                let dot_pos = Pos2::new(start_x + x, start_y + y);

                // Calculate fade based on distance from center
                let alpha = if self.fade_distance > 0.0 {
                    let distance_x = (dot_pos.x - center.x).abs() / (bounds.width() / 2.0);
                    let distance_y = (dot_pos.y - center.y).abs() / (bounds.height() / 2.0);
                    let distance = (distance_x.powi(2) + distance_y.powi(2)).sqrt();
                    let fade_factor = 1.0 - (distance / self.fade_distance).min(1.0);
                    (color.a() as f32 * fade_factor) as u8
                } else {
                    color.a()
                };

                let dot_color =
                    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);

                // Draw glow if enabled
                if self.glow && alpha > 0 {
                    let glow_color = Color32::from_rgba_unmultiplied(
                        color.r(),
                        color.g(),
                        color.b(),
                        (alpha as f32 * 0.3) as u8,
                    );
                    painter.circle_filled(dot_pos, self.dot_radius * 2.5, glow_color);
                }

                // Draw main dot
                painter.circle_filled(dot_pos, self.dot_radius, dot_color);

                x += self.spacing;
            }
            y += self.spacing;
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_pattern_creation() {
        let pattern = DotPattern::new();
        assert_eq!(pattern.spacing, 20.0);
        assert_eq!(pattern.dot_radius, 1.5);
        assert!(pattern.color.is_none());
    }

    #[test]
    fn test_dot_pattern_config() {
        let pattern = DotPattern::new()
            .width(800.0)
            .height(600.0)
            .spacing(30.0)
            .dot_radius(2.0)
            .fade(0.5)
            .glow(true);

        assert_eq!(pattern.width, Some(800.0));
        assert_eq!(pattern.height, Some(600.0));
        assert_eq!(pattern.spacing, 30.0);
        assert_eq!(pattern.dot_radius, 2.0);
        assert_eq!(pattern.fade_distance, 0.5);
        assert!(pattern.glow);
    }

    #[test]
    fn test_dot_pattern_bounds() {
        let pattern = DotPattern::new()
            .spacing(0.5) // Should clamp to 1.0
            .dot_radius(0.3); // Should clamp to 0.5

        assert_eq!(pattern.spacing, 1.0);
        assert_eq!(pattern.dot_radius, 0.5);
    }
}
