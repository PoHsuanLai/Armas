//! Glowing Divider
//!
//! Animated separator with glow effect and optional gradient

use crate::theme::Theme;
use egui::{self, Color32, Pos2, Rect, Stroke, Vec2};

/// Style for the divider
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DividerStyle {
    /// Simple line with glow
    Simple,
    /// Gradient fade from center
    Gradient,
    /// Pulsing glow animation
    Pulsing,
}

impl Default for DividerStyle {
    fn default() -> Self {
        Self::Simple
    }
}

/// Orientation of the divider
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DividerOrientation {
    /// Horizontal divider
    Horizontal,
    /// Vertical divider
    Vertical,
}

impl Default for DividerOrientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

/// Glowing divider/separator component
///
/// A separator line with customizable glow effect, gradient, and animation.
///
/// # Example
///
/// ```rust,ignore
/// use armas::{GlowingDivider, DividerStyle};
///
/// // Simple glowing divider
/// GlowingDivider::new()
///     .show(ui, &theme);
///
/// // Gradient divider with custom color
/// GlowingDivider::new()
///     .style(DividerStyle::Gradient)
///     .glow_color(Color32::from_rgb(255, 100, 200))
///     .show(ui, &theme);
///
/// // Pulsing divider
/// GlowingDivider::new()
///     .style(DividerStyle::Pulsing)
///     .show(ui, &theme);
/// ```
pub struct GlowingDivider {
    style: DividerStyle,
    orientation: DividerOrientation,
    thickness: f32,
    glow_intensity: f32,
    glow_color: Option<Color32>,
    length: Option<f32>,
}

impl GlowingDivider {
    /// Create a new glowing divider
    pub fn new() -> Self {
        Self {
            style: DividerStyle::Simple,
            orientation: DividerOrientation::Horizontal,
            thickness: 1.0,
            glow_intensity: 0.5,
            glow_color: None,
            length: None,
        }
    }

    /// Set the divider style
    pub fn style(mut self, style: DividerStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the orientation
    pub fn orientation(mut self, orientation: DividerOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set the line thickness
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set the glow intensity (0.0 to 1.0)
    pub fn glow_intensity(mut self, intensity: f32) -> Self {
        self.glow_intensity = intensity.clamp(0.0, 1.0);
        self
    }

    /// Set custom glow color (defaults to primary color)
    pub fn glow_color(mut self, color: Color32) -> Self {
        self.glow_color = Some(color);
        self
    }

    /// Set custom length (defaults to full available width/height)
    pub fn length(mut self, length: f32) -> Self {
        self.length = Some(length);
        self
    }

    /// Alias for horizontal orientation
    pub fn horizontal(mut self) -> Self {
        self.orientation = DividerOrientation::Horizontal;
        self
    }

    /// Alias for vertical orientation
    pub fn vertical(mut self) -> Self {
        self.orientation = DividerOrientation::Vertical;
        self
    }

    /// Show the glowing divider
    pub fn show(self, ui: &mut egui::Ui, theme: &Theme) -> egui::Response {
        let glow_color = self.glow_color.unwrap_or_else(|| theme.primary());

        // Get animation time for pulsing effect
        let time = ui.input(|i| i.time) as f32;

        // Calculate pulse factor for pulsing style
        let pulse_factor = if self.style == DividerStyle::Pulsing {
            ((time * 2.0).sin() * 0.5 + 0.5) * 0.3 + 0.7 // Oscillate between 0.7 and 1.0
        } else {
            1.0
        };

        let size = match self.orientation {
            DividerOrientation::Horizontal => {
                let width = self.length.unwrap_or(ui.available_width());
                let height = self.thickness + self.glow_intensity * 20.0;
                Vec2::new(width, height)
            }
            DividerOrientation::Vertical => {
                let height = self.length.unwrap_or(ui.available_height());
                let width = self.thickness + self.glow_intensity * 20.0;
                Vec2::new(width, height)
            }
        };

        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            match self.style {
                DividerStyle::Simple => {
                    self.draw_simple(painter, rect, glow_color, pulse_factor);
                }
                DividerStyle::Gradient => {
                    self.draw_gradient(painter, rect, glow_color, theme);
                }
                DividerStyle::Pulsing => {
                    self.draw_simple(painter, rect, glow_color, pulse_factor);
                }
            }
        }

        // Request repaint for pulsing animation
        if self.style == DividerStyle::Pulsing {
            ui.ctx().request_repaint();
        }

        response
    }

    fn draw_simple(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        glow_color: Color32,
        pulse_factor: f32,
    ) {
        let (start, end) = match self.orientation {
            DividerOrientation::Horizontal => {
                let y = rect.center().y;
                (Pos2::new(rect.left(), y), Pos2::new(rect.right(), y))
            }
            DividerOrientation::Vertical => {
                let x = rect.center().x;
                (Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom()))
            }
        };

        // Draw glow layers
        let glow_layers = 5;
        for i in 0..glow_layers {
            let layer_width =
                self.thickness + (i as f32 * 2.0 * self.glow_intensity * pulse_factor);
            let alpha = ((glow_layers - i) as f32 / glow_layers as f32
                * 60.0
                * self.glow_intensity
                * pulse_factor) as u8;
            let glow = Color32::from_rgba_unmultiplied(
                glow_color.r(),
                glow_color.g(),
                glow_color.b(),
                alpha,
            );
            painter.line_segment([start, end], Stroke::new(layer_width, glow));
        }

        // Draw center line
        painter.line_segment([start, end], Stroke::new(self.thickness, glow_color));
    }

    fn draw_gradient(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        glow_color: Color32,
        _theme: &Theme,
    ) {
        match self.orientation {
            DividerOrientation::Horizontal => {
                let y = rect.center().y;
                let center_x = rect.center().x;
                let segments = 50;
                let segment_width = rect.width() / segments as f32;

                for i in 0..segments {
                    let x1 = rect.left() + i as f32 * segment_width;
                    let x2 = x1 + segment_width;
                    let start = Pos2::new(x1, y);
                    let end = Pos2::new(x2, y);

                    // Calculate distance from center (0.0 to 1.0)
                    let dist_from_center =
                        ((x1 + x2) / 2.0 - center_x).abs() / (rect.width() / 2.0);

                    // Fade alpha based on distance from center
                    let alpha = ((1.0 - dist_from_center) * 255.0) as u8;
                    let color = Color32::from_rgba_unmultiplied(
                        glow_color.r(),
                        glow_color.g(),
                        glow_color.b(),
                        alpha,
                    );

                    // Draw glow
                    let glow_width =
                        self.thickness + self.glow_intensity * 10.0 * (1.0 - dist_from_center);
                    let glow_alpha = (alpha as f32 * 0.3) as u8;
                    let glow = Color32::from_rgba_unmultiplied(
                        glow_color.r(),
                        glow_color.g(),
                        glow_color.b(),
                        glow_alpha,
                    );
                    painter.line_segment([start, end], Stroke::new(glow_width, glow));

                    // Draw center line
                    painter.line_segment([start, end], Stroke::new(self.thickness, color));
                }
            }
            DividerOrientation::Vertical => {
                let x = rect.center().x;
                let center_y = rect.center().y;
                let segments = 50;
                let segment_height = rect.height() / segments as f32;

                for i in 0..segments {
                    let y1 = rect.top() + i as f32 * segment_height;
                    let y2 = y1 + segment_height;
                    let start = Pos2::new(x, y1);
                    let end = Pos2::new(x, y2);

                    // Calculate distance from center (0.0 to 1.0)
                    let dist_from_center =
                        ((y1 + y2) / 2.0 - center_y).abs() / (rect.height() / 2.0);

                    // Fade alpha based on distance from center
                    let alpha = ((1.0 - dist_from_center) * 255.0) as u8;
                    let color = Color32::from_rgba_unmultiplied(
                        glow_color.r(),
                        glow_color.g(),
                        glow_color.b(),
                        alpha,
                    );

                    // Draw glow
                    let glow_width =
                        self.thickness + self.glow_intensity * 10.0 * (1.0 - dist_from_center);
                    let glow_alpha = (alpha as f32 * 0.3) as u8;
                    let glow = Color32::from_rgba_unmultiplied(
                        glow_color.r(),
                        glow_color.g(),
                        glow_color.b(),
                        glow_alpha,
                    );
                    painter.line_segment([start, end], Stroke::new(glow_width, glow));

                    // Draw center line
                    painter.line_segment([start, end], Stroke::new(self.thickness, color));
                }
            }
        }
    }
}

impl Default for GlowingDivider {
    fn default() -> Self {
        Self::new()
    }
}
