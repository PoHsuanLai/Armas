//! Dot Pattern Background
//!
//! Creates a simple dot grid pattern background effect

use crate::ext::ArmasContextExt;
use crate::Theme;
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
    color: Color32,
    /// Fade distance (0.0 = no fade, 1.0 = full fade at edges)
    fade_distance: f32,
    /// Glow effect (adds larger, more transparent dots behind)
    glow: bool,
    /// Width and height
    width: f32,
    height: f32,
}

impl DotPattern {
    /// Create a new dot pattern with theme-based defaults
    pub fn new(width: f32, height: f32, theme: &Theme) -> Self {
        Self {
            spacing: 20.0,
            dot_radius: 1.5,
            color: Color32::from_gray(120), // Use gray instead of theme color for visibility
            fade_distance: 0.0,
            glow: false,
            width,
            height,
        }
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
        self.color = color;
        self
    }

    /// Enable fade effect at edges (0.0 to 1.0)
    pub fn fade(mut self, fade: f32) -> Self {
        self.fade_distance = fade.clamp(0.0, 1.0);
        self
    }

    /// Enable glow effect
    pub fn with_glow(mut self, enabled: bool) -> Self {
        self.glow = enabled;
        self
    }

    /// Show the dot pattern
    pub fn show(&self, ui: &mut Ui) -> Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());

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
                    (self.color.a() as f32 * fade_factor) as u8
                } else {
                    self.color.a()
                };

                let dot_color = Color32::from_rgba_unmultiplied(
                    self.color.r(),
                    self.color.g(),
                    self.color.b(),
                    alpha,
                );

                // Draw glow if enabled
                if self.glow && alpha > 0 {
                    let glow_color = Color32::from_rgba_unmultiplied(
                        self.color.r(),
                        self.color.g(),
                        self.color.b(),
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
        let theme = Theme::default();
        let pattern = DotPattern::new(800.0, 600.0, &theme);
        assert_eq!(pattern.spacing, 20.0);
        assert_eq!(pattern.dot_radius, 1.5);
    }

    #[test]
    fn test_dot_pattern_config() {
        let theme = Theme::default();
        let pattern = DotPattern::new(800.0, 600.0, &theme)
            .spacing(30.0)
            .dot_radius(2.0)
            .fade(0.5)
            .with_glow(true);

        assert_eq!(pattern.spacing, 30.0);
        assert_eq!(pattern.dot_radius, 2.0);
        assert_eq!(pattern.fade_distance, 0.5);
        assert!(pattern.glow);
    }

    #[test]
    fn test_dot_pattern_bounds() {
        let theme = Theme::default();
        let pattern = DotPattern::new(800.0, 600.0, &theme)
            .spacing(0.5) // Should clamp to 1.0
            .dot_radius(0.3); // Should clamp to 0.5

        assert_eq!(pattern.spacing, 1.0);
        assert_eq!(pattern.dot_radius, 0.5);
    }
}
