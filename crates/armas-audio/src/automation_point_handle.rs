//! Point handle for automation point interaction
//!
//! Interactive circle handles for selecting, dragging, and editing automation points.
//!
//! Features:
//! - Normal mode: Direct positioning - drag moves handle by cursor delta
//! - Velocity mode (Ctrl/Cmd + drag): Fine control based on mouse speed

use armas::animation::{VelocityDrag, VelocityDragConfig};
use armas::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

const POINT_RADIUS: f32 = 6.0;
const HOVER_RADIUS: f32 = 10.0;

/// State for point handle drag interaction (stored in egui temp data)
#[derive(Clone)]
struct PointHandleDragState {
    drag_x: VelocityDrag,
    drag_y: VelocityDrag,
}

/// Response from point handle interaction
#[derive(Debug, Clone)]
pub struct PointHandleResponse {
    pub response: Response,
    pub is_selected: bool,
    pub is_hovered: bool,
    pub drag_delta: Option<Vec2>,
}

/// Interactive automation point handle
pub struct PointHandle {
    position: Pos2,
    color: Color32,
    selected: bool,
    show_value: bool,
    value_text: Option<String>,
    /// Enable velocity-based dragging (Ctrl/Cmd for fine control)
    velocity_mode: bool,
    /// Sensitivity for velocity mode
    velocity_sensitivity: f64,
    /// Unique ID for state persistence
    id: Option<egui::Id>,
}

impl PointHandle {
    /// Create a new point handle
    pub fn new(position: Pos2, color: Color32) -> Self {
        Self {
            position,
            color,
            selected: false,
            show_value: false,
            value_text: None,
            velocity_mode: true,
            velocity_sensitivity: 1.0,
            id: None,
        }
    }

    /// Set unique ID for state persistence
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
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

    /// Mark this handle as selected
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Show value label on hover
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Set the value text to display
    pub fn value_text(mut self, text: impl Into<String>) -> Self {
        self.value_text = Some(text.into());
        self
    }

    /// Show the point handle
    pub fn show(self, ui: &mut Ui, theme: &Theme) -> PointHandleResponse {
        let handle_rect = Rect::from_center_size(self.position, Vec2::splat(HOVER_RADIUS * 2.0));

        let response = ui.allocate_rect(handle_rect, Sense::click_and_drag());
        let is_hovered = response.hovered();

        // Get or create drag state for velocity mode
        let drag_state_id = self.id.unwrap_or(response.id).with("point_handle_drag");
        let mut drag_state: PointHandleDragState = ui.ctx().data(|d| {
            d.get_temp(drag_state_id).unwrap_or_else(|| PointHandleDragState {
                drag_x: VelocityDrag::new(
                    VelocityDragConfig::new().sensitivity(self.velocity_sensitivity),
                ),
                drag_y: VelocityDrag::new(
                    VelocityDragConfig::new().sensitivity(self.velocity_sensitivity),
                ),
            })
        });

        // Calculate drag delta based on mode
        let drag_delta = if response.drag_started() {
            let modifiers = ui.ctx().input(|i| i.modifiers);
            // In velocity mode: Ctrl/Cmd switches to absolute mode
            let use_velocity = self.velocity_mode && !modifiers.command && !modifiers.ctrl;

            if let Some(pos) = response.interact_pointer_pos() {
                drag_state.drag_x.begin(self.position.x as f64, pos.x as f64, use_velocity);
                drag_state.drag_y.begin(self.position.y as f64, pos.y as f64, use_velocity);
            }
            None
        } else if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                if drag_state.drag_x.is_velocity_mode() {
                    // Velocity mode: delta based on mouse speed
                    let delta_x = drag_state.drag_x.update_tracked(pos.x as f64, 100.0, 200.0);
                    let delta_y = drag_state.drag_y.update_tracked(pos.y as f64, 100.0, 200.0);
                    Some(Vec2::new(delta_x as f32, delta_y as f32))
                } else {
                    // Absolute mode: use raw drag delta
                    let delta = response.drag_delta();
                    if delta.length() > 0.0 { Some(delta) } else { None }
                }
            } else {
                None
            }
        } else {
            if response.drag_stopped() {
                drag_state.drag_x.end();
                drag_state.drag_y.end();
            }
            None
        };

        // Save drag state
        ui.ctx().data_mut(|d| d.insert_temp(drag_state_id, drag_state));

        // Render the point
        self.render(ui.painter(), is_hovered, theme);

        // Render value label if hovering
        if is_hovered && self.show_value {
            if let Some(value_text) = &self.value_text {
                ui.painter().text(
                    self.position + Vec2::new(15.0, -20.0),
                    egui::Align2::LEFT_BOTTOM,
                    value_text,
                    egui::FontId::monospace(10.0),
                    theme.foreground(),
                );
            }
        }

        PointHandleResponse {
            response,
            is_selected: self.selected,
            is_hovered,
            drag_delta,
        }
    }

    /// Render the point handle
    fn render(&self, painter: &egui::Painter, is_hovered: bool, theme: &Theme) {
        let radius = if is_hovered { HOVER_RADIUS } else { POINT_RADIUS };

        // Draw outer glow rings (enhanced multi-layer glow)
        if self.selected {
            // Strong glow for selected state
            for i in 0..3 {
                let glow_radius = radius + (i + 1) as f32 * 3.5;
                let alpha = ((1.0 - (i as f32 / 3.0)) * 40.0) as u8;
                let glow_color = Color32::from_rgba_unmultiplied(
                    self.color.r(),
                    self.color.g(),
                    self.color.b(),
                    alpha,
                );

                painter.circle_stroke(
                    self.position,
                    glow_radius,
                    egui::Stroke::new(0.8, glow_color),
                );
            }
        } else if is_hovered {
            // Subtle glow for hover state
            for i in 0..2 {
                let glow_radius = radius + (i + 1) as f32 * 2.5;
                let alpha = ((1.0 - (i as f32 / 2.0)) * 25.0) as u8;
                let glow_color = Color32::from_rgba_unmultiplied(
                    self.color.r(),
                    self.color.g(),
                    self.color.b(),
                    alpha,
                );

                painter.circle_stroke(
                    self.position,
                    glow_radius,
                    egui::Stroke::new(0.6, glow_color),
                );
            }
        }

        // Draw main circle
        let fill_color = if is_hovered {
            self.color
        } else {
            self.color.gamma_multiply(0.95)
        };
        painter.circle_filled(self.position, radius, fill_color);

        // Draw border with thickness based on state
        let (border_color, border_width) = if self.selected {
            (theme.primary(), 2.0)
        } else if is_hovered {
            (theme.primary().gamma_multiply(0.8), 1.75)
        } else {
            (Color32::WHITE.gamma_multiply(0.6), 1.25)
        };

        painter.circle_stroke(
            self.position,
            radius,
            egui::Stroke::new(border_width, border_color),
        );

        // Draw concentric inner ring for selected state (flat design accent)
        if self.selected {
            let inner_ring_radius = radius * 0.55;
            let ring_color = self.color.gamma_multiply(0.7);

            painter.circle_stroke(
                self.position,
                inner_ring_radius,
                egui::Stroke::new(1.0, ring_color),
            );
        }
    }
}
