//! XY Pad Component
//!
//! 2D touch controller for simultaneous control of two parameters.
//! Perfect for filter controls, spatial effects, and expressive performance.
//!
//! Features:
//! - Normal mode: Direct positioning - click/drag moves handle to cursor position
//! - Velocity mode (Ctrl/Cmd + drag): Fine control based on mouse speed
//! - Double-click to reset to default values

use armas::animation::{VelocityDrag, VelocityDragConfig};
use armas::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// State for XY pad drag interaction (stored in egui temp data)
#[derive(Clone)]
struct XYPadDragState {
    drag_x: VelocityDrag,
    drag_y: VelocityDrag,
}

/// Response from the XY pad
#[derive(Debug, Clone)]
pub struct XYPadResponse {
    /// The UI response
    pub response: Response,
    /// X value (0.0 to 1.0)
    pub x: f32,
    /// Y value (0.0 to 1.0)
    pub y: f32,
    /// Whether values changed this frame
    pub changed: bool,
}

impl XYPadResponse {
    /// Check if values changed this frame
    pub fn changed(&self) -> bool {
        self.changed
    }
}

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
    id: Option<egui::Id>,
    /// Enable velocity-based dragging (Ctrl/Cmd for fine control)
    velocity_mode: bool,
    /// Sensitivity for velocity mode
    velocity_sensitivity: f64,
    /// Default X value for double-click reset
    default_x: Option<f32>,
    /// Default Y value for double-click reset
    default_y: Option<f32>,
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
            id: None,
            velocity_mode: true,
            velocity_sensitivity: 1.0,
            default_x: None,
            default_y: None,
        }
    }

    /// Set unique ID for state persistence
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
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

    /// Enable velocity mode (Ctrl/Cmd for fine control). Default: true
    pub fn velocity_mode(mut self, enabled: bool) -> Self {
        self.velocity_mode = enabled;
        self
    }

    /// Set sensitivity for velocity mode. Default: 1.0
    pub fn velocity_sensitivity(mut self, sensitivity: f64) -> Self {
        self.velocity_sensitivity = sensitivity.max(0.1);
        self
    }

    /// Set default X value for double-click reset
    pub fn default_x(mut self, value: f32) -> Self {
        self.default_x = Some(value.clamp(0.0, 1.0));
        self
    }

    /// Set default Y value for double-click reset
    pub fn default_y(mut self, value: f32) -> Self {
        self.default_y = Some(value.clamp(0.0, 1.0));
        self
    }

    /// Set default values for both axes (convenience method)
    pub fn default_values(mut self, x: f32, y: f32) -> Self {
        self.default_x = Some(x.clamp(0.0, 1.0));
        self.default_y = Some(y.clamp(0.0, 1.0));
        self
    }

    /// Show the XY pad
    pub fn show(self, ui: &mut Ui, theme: &Theme) -> XYPadResponse {
        // Load previous state if ID is set
        if let Some(id) = self.id {
            let state_id_x = id.with("xy_pad_x");
            let state_id_y = id.with("xy_pad_y");
            *self.x = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id_x).unwrap_or(*self.x));
            *self.y = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id_y).unwrap_or(*self.y));
        }

        // Clamp values
        *self.x = self.x.clamp(0.0, 1.0);
        *self.y = self.y.clamp(0.0, 1.0);

        let desired_size = Vec2::splat(self.size);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        // Get or create drag state
        let drag_state_id = self.id.unwrap_or(response.id).with("xy_pad_drag_state");
        let mut drag_state: XYPadDragState = ui.ctx().data(|d| {
            d.get_temp(drag_state_id).unwrap_or_else(|| XYPadDragState {
                drag_x: VelocityDrag::new(
                    VelocityDragConfig::new().sensitivity(self.velocity_sensitivity),
                ),
                drag_y: VelocityDrag::new(
                    VelocityDragConfig::new().sensitivity(self.velocity_sensitivity),
                ),
            })
        });

        // Handle double-click to reset
        if response.double_clicked() {
            if let Some(default_x) = self.default_x {
                *self.x = default_x;
            }
            if let Some(default_y) = self.default_y {
                *self.y = default_y;
            }
            if self.default_x.is_some() || self.default_y.is_some() {
                response.mark_changed();
            }
        }

        // Handle drag interaction
        if response.drag_started() {
            let modifiers = ui.ctx().input(|i| i.modifiers);
            // In velocity mode: Ctrl/Cmd switches to absolute mode
            // Without velocity mode: always absolute
            let use_velocity = self.velocity_mode && !modifiers.command && !modifiers.ctrl;

            if let Some(pos) = response.interact_pointer_pos() {
                drag_state
                    .drag_x
                    .begin(*self.x as f64, pos.x as f64, use_velocity);
                drag_state
                    .drag_y
                    .begin(*self.y as f64, pos.y as f64, use_velocity);
            }
        }

        if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                if drag_state.drag_x.is_velocity_mode() {
                    // Velocity mode: incremental changes based on mouse speed
                    let delta_x =
                        drag_state
                            .drag_x
                            .update_tracked(pos.x as f64, 1.0, self.size as f64);
                    let delta_y =
                        drag_state
                            .drag_y
                            .update_tracked(pos.y as f64, 1.0, self.size as f64);

                    *self.x = (*self.x + delta_x as f32).clamp(0.0, 1.0);
                    // Y is inverted (up = higher value)
                    *self.y = (*self.y - delta_y as f32).clamp(0.0, 1.0);
                    response.mark_changed();
                } else {
                    // Absolute mode: jump to cursor position
                    *self.x = ((pos.x - rect.min.x) / rect.width()).clamp(0.0, 1.0);
                    *self.y = 1.0 - ((pos.y - rect.min.y) / rect.height()).clamp(0.0, 1.0);
                    response.mark_changed();
                }
            }
        }

        if response.drag_stopped() {
            drag_state.drag_x.end();
            drag_state.drag_y.end();
        }

        // Save drag state
        ui.ctx()
            .data_mut(|d| d.insert_temp(drag_state_id, drag_state));

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let corner_radius = theme.spacing.corner_radius as f32;

            // Draw based on variant
            match self.variant {
                XYPadVariant::Filled => {
                    self.draw_filled(painter, theme, rect, corner_radius);
                }
                XYPadVariant::Outlined => {
                    self.draw_outlined(painter, theme, rect, corner_radius);
                }
                XYPadVariant::Elevated => {
                    self.draw_elevated(painter, theme, rect, corner_radius);
                }
            }

            // Draw crosshair
            if self.show_crosshair {
                let handle_x = rect.min.x + *self.x * rect.width();
                let handle_y = rect.max.y - *self.y * rect.height();

                let crosshair_color = theme.muted_foreground().gamma_multiply(0.3);
                painter.line_segment(
                    [
                        Pos2::new(rect.min.x, handle_y),
                        Pos2::new(rect.max.x, handle_y),
                    ],
                    egui::Stroke::new(1.0, crosshair_color),
                );
                painter.line_segment(
                    [
                        Pos2::new(handle_x, rect.min.y),
                        Pos2::new(handle_x, rect.max.y),
                    ],
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
                egui::Stroke::new(2.0, theme.foreground().gamma_multiply(0.9)),
            );

            // Draw labels
            if let Some(x_label) = &self.x_label {
                painter.text(
                    Pos2::new(rect.center().x, rect.max.y + theme.spacing.sm),
                    egui::Align2::CENTER_TOP,
                    x_label,
                    egui::FontId::proportional(11.0),
                    theme.muted_foreground(),
                );
            }

            if let Some(y_label) = &self.y_label {
                painter.text(
                    Pos2::new(rect.min.x - theme.spacing.sm, rect.center().y),
                    egui::Align2::RIGHT_CENTER,
                    y_label,
                    egui::FontId::proportional(11.0),
                    theme.muted_foreground(),
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
                    theme.foreground(),
                );
            }
        }

        // Save state to memory if ID is set
        if let Some(id) = self.id {
            let state_id_x = id.with("xy_pad_x");
            let state_id_y = id.with("xy_pad_y");
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id_x, *self.x);
                d.insert_temp(state_id_y, *self.y);
            });
        }

        let changed = response.changed();
        XYPadResponse {
            response,
            x: *self.x,
            y: *self.y,
            changed,
        }
    }

    fn draw_filled(&self, painter: &egui::Painter, theme: &Theme, rect: Rect, corner_radius: f32) {
        // Background
        painter.rect_filled(rect, corner_radius, theme.muted());

        // Border
        painter.rect_stroke(
            rect,
            corner_radius,
            egui::Stroke::new(1.0, theme.border()),
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
        painter.rect_filled(rect, corner_radius, theme.card());

        // Border
        painter.rect_stroke(
            rect,
            corner_radius,
            egui::Stroke::new(1.5, theme.border()),
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
        painter.rect_filled(rect, corner_radius, theme.muted());

        // Border
        painter.rect_stroke(
            rect,
            corner_radius,
            egui::Stroke::new(1.0, theme.border()),
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
            .show_crosshair(true)
            .velocity_mode(true)
            .velocity_sensitivity(1.5)
            .default_values(0.5, 0.5);

        assert_eq!(pad.size, 300.0);
        assert_eq!(pad.x_label, Some("Cutoff".to_string()));
        assert_eq!(pad.y_label, Some("Resonance".to_string()));
        assert!(pad.show_crosshair);
        assert!(pad.velocity_mode);
        assert_eq!(pad.velocity_sensitivity, 1.5);
        assert_eq!(pad.default_x, Some(0.5));
        assert_eq!(pad.default_y, Some(0.5));
    }
}
