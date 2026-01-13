//! XY Pad Component
//!
//! 2D touch controller for simultaneous control of two parameters.
//! Perfect for filter controls, spatial effects, and expressive performance.

use crate::ext::ArmasContextExt;
use crate::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Visual style variant for XY pad
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XYPadVariant {
    /// Filled pad with solid background
    Filled,
    /// Outlined pad with transparent background
    Outlined,
    /// Elevated pad with shadow effect
    Elevated,
}

/// XY Pad controller component
///
/// A 2D touch surface for controlling two parameters simultaneously.
/// Common in synthesizers and effects for expressive control.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::components::audio::XYPad;
///
/// let mut x = 0.5;
/// let mut y = 0.5;
///
/// let response = XYPad::new(&mut x, &mut y)
///     .size(200.0)
///     .show(ui);
///
/// if response.changed() {
///     println!("X: {:.2}, Y: {:.2}", x, y);
/// }
/// # }
/// ```
pub struct XYPad<'a> {
    x: &'a mut f32,
    y: &'a mut f32,
    size: f32,
    variant: XYPadVariant,
    x_label: Option<String>,
    y_label: Option<String>,
    show_crosshair: bool,
    show_values: bool,
    handle_size: f32,
    glow_intensity: f32,
}

impl<'a> XYPad<'a> {
    /// Create a new XY pad
    pub fn new(x: &'a mut f32, y: &'a mut f32) -> Self {
        Self {
            x,
            y,
            size: 200.0,
            variant: XYPadVariant::Filled,
            x_label: None,
            y_label: None,
            show_crosshair: true,
            show_values: false,
            handle_size: 16.0,
            glow_intensity: 0.8,
        }
    }

    /// Set pad size (width and height)
    pub fn size(mut self, size: f32) -> Self {
        self.size = size.max(100.0);
        self
    }

    /// Set visual variant
    pub fn variant(mut self, variant: XYPadVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set X axis label
    pub fn x_label(mut self, label: impl Into<String>) -> Self {
        self.x_label = Some(label.into());
        self
    }

    /// Set Y axis label
    pub fn y_label(mut self, label: impl Into<String>) -> Self {
        self.y_label = Some(label.into());
        self
    }

    /// Show crosshair lines
    pub fn show_crosshair(mut self, show: bool) -> Self {
        self.show_crosshair = show;
        self
    }

    /// Show numeric values
    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Set handle size
    pub fn handle_size(mut self, size: f32) -> Self {
        self.handle_size = size.max(8.0);
        self
    }

    /// Set glow intensity
    pub fn glow_intensity(mut self, intensity: f32) -> Self {
        self.glow_intensity = intensity.clamp(0.0, 1.0);
        self
    }

    /// Show the XY pad
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();

        // Clamp values
        *self.x = self.x.clamp(0.0, 1.0);
        *self.y = self.y.clamp(0.0, 1.0);

        let desired_size = Vec2::splat(self.size);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        // Handle interaction
        if response.dragged() || response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                *self.x = ((pos.x - rect.min.x) / rect.width()).clamp(0.0, 1.0);
                *self.y = 1.0 - ((pos.y - rect.min.y) / rect.height()).clamp(0.0, 1.0);
                response.mark_changed();
            }
        }

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let corner_radius = theme.spacing.corner_radius as f32;

            // Draw based on variant
            match self.variant {
                XYPadVariant::Filled => {
                    self.draw_filled(painter, &theme, rect, corner_radius);
                }
                XYPadVariant::Outlined => {
                    self.draw_outlined(painter, &theme, rect, corner_radius);
                }
                XYPadVariant::Elevated => {
                    self.draw_elevated(painter, &theme, rect, corner_radius);
                }
            }

            // Draw crosshair
            if self.show_crosshair {
                let handle_x = rect.min.x + *self.x * rect.width();
                let handle_y = rect.max.y - *self.y * rect.height();

                let crosshair_color = theme.on_surface_variant().gamma_multiply(0.3);
                painter.line_segment(
                    [Pos2::new(rect.min.x, handle_y), Pos2::new(rect.max.x, handle_y)],
                    egui::Stroke::new(1.0, crosshair_color),
                );
                painter.line_segment(
                    [Pos2::new(handle_x, rect.min.y), Pos2::new(handle_x, rect.max.y)],
                    egui::Stroke::new(1.0, crosshair_color),
                );
            }

            // Draw handle
            let handle_x = rect.min.x + *self.x * rect.width();
            let handle_y = rect.max.y - *self.y * rect.height();
            let handle_pos = Pos2::new(handle_x, handle_y);

            // Handle glow
            if response.dragged() || response.is_pointer_button_down_on() {
                for i in 0..4 {
                    let offset = (i + 1) as f32 * 2.0;
                    let alpha = ((1.0 - i as f32 / 4.0) * 50.0 * self.glow_intensity) as u8;
                    let glow_color = Color32::from_rgba_unmultiplied(
                        theme.primary().r(),
                        theme.primary().g(),
                        theme.primary().b(),
                        alpha,
                    );
                    painter.circle_stroke(
                        handle_pos,
                        self.handle_size / 2.0 + offset,
                        egui::Stroke::new(2.0, glow_color),
                    );
                }
            }

            // Handle circle
            painter.circle_filled(handle_pos, self.handle_size / 2.0, theme.primary());
            painter.circle_stroke(
                handle_pos,
                self.handle_size / 2.0,
                egui::Stroke::new(2.0, theme.on_surface().gamma_multiply(0.9)),
            );

            // Draw labels
            if let Some(x_label) = &self.x_label {
                painter.text(
                    Pos2::new(rect.center().x, rect.max.y + theme.spacing.sm),
                    egui::Align2::CENTER_TOP,
                    x_label,
                    egui::FontId::proportional(11.0),
                    theme.on_surface_variant(),
                );
            }

            if let Some(y_label) = &self.y_label {
                painter.text(
                    Pos2::new(rect.min.x - theme.spacing.sm, rect.center().y),
                    egui::Align2::RIGHT_CENTER,
                    y_label,
                    egui::FontId::proportional(11.0),
                    theme.on_surface_variant(),
                );
            }

            // Draw values
            if self.show_values {
                let value_text = format!("X:{:.2} Y:{:.2}", self.x, self.y);
                painter.text(
                    Pos2::new(rect.center().x, rect.min.y + theme.spacing.sm),
                    egui::Align2::CENTER_TOP,
                    value_text,
                    egui::FontId::proportional(10.0),
                    theme.on_surface(),
                );
            }
        }

        response
    }

    fn draw_filled(&self, painter: &egui::Painter, theme: &Theme, rect: Rect, corner_radius: f32) {
        // Background
        painter.rect_filled(rect, corner_radius, theme.surface_variant());

        // Border
        painter.rect_stroke(
            rect,
            corner_radius,
            egui::Stroke::new(1.0, theme.outline_variant()),
            egui::StrokeKind::Outside,
        );
    }

    fn draw_outlined(
        &self,
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        corner_radius: f32,
    ) {
        // Background
        painter.rect_filled(rect, corner_radius, theme.surface());

        // Border
        painter.rect_stroke(
            rect,
            corner_radius,
            egui::Stroke::new(1.5, theme.outline()),
            egui::StrokeKind::Outside,
        );
    }

    fn draw_elevated(
        &self,
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        corner_radius: f32,
    ) {
        // Shadow layers
        for i in 0..3 {
            let offset = (i + 1) as f32 * 0.5;
            let shadow_rect = rect.translate(Vec2::new(0.0, offset));
            let alpha = (20.0 - i as f32 * 5.0) as u8;
            let shadow_color = Color32::from_rgba_unmultiplied(0, 0, 0, alpha);
            painter.rect_filled(shadow_rect, corner_radius, shadow_color);
        }

        // Background
        painter.rect_filled(rect, corner_radius, theme.surface_variant());

        // Border
        painter.rect_stroke(
            rect,
            corner_radius,
            egui::Stroke::new(1.0, theme.outline_variant()),
            egui::StrokeKind::Outside,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xy_pad_creation() {
        let mut x = 0.5;
        let mut y = 0.5;
        let _pad = XYPad::new(&mut x, &mut y);
        assert_eq!(x, 0.5);
        assert_eq!(y, 0.5);
    }

    #[test]
    fn test_xy_pad_builder() {
        let mut x = 0.5;
        let mut y = 0.5;
        let pad = XYPad::new(&mut x, &mut y)
            .size(300.0)
            .x_label("Cutoff")
            .y_label("Resonance")
            .show_crosshair(true);

        assert_eq!(pad.size, 300.0);
        assert_eq!(pad.x_label, Some("Cutoff".to_string()));
        assert_eq!(pad.y_label, Some("Resonance".to_string()));
        assert!(pad.show_crosshair);
    }
}
