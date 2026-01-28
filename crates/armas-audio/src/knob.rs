//! Knob Component
//!
//! Metallic knob with 3D appearance and inner glow level indicator.
//! Designed for audio mixing, synthesizers, and effect controls.
//!
//! Supports velocity-based dragging for fine control (hold Ctrl/Cmd).

use armas::animation::{DragMode, VelocityDrag, VelocityDragConfig};
use armas::theme::Theme;
use egui::{Color32, Pos2, Response, Sense, Stroke, Ui, Vec2};

/// Persisted drag state for knob
#[derive(Clone)]
struct KnobDragState {
    drag: VelocityDrag,
    drag_start_value: f32,
}

impl Default for KnobDragState {
    fn default() -> Self {
        Self {
            drag: VelocityDrag::new(VelocityDragConfig::new().sensitivity(1.0)),
            drag_start_value: 0.0,
        }
    }
}

/// Response from the knob control
#[derive(Debug, Clone)]
pub struct KnobResponse {
    /// The UI response
    pub response: Response,
    /// New knob value (0.0 to 1.0)
    pub value: f32,
    /// Whether the value changed this frame
    pub changed: bool,
}

impl KnobResponse {
    /// Check if the value changed this frame
    #[must_use]
    pub const fn changed(&self) -> bool {
        self.changed
    }
}

/// Knob response curve for different control types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnobCurve {
    /// Linear response (0.0 to 1.0 mapped directly)
    Linear,
    /// Logarithmic response (useful for frequencies, more sensitive at lower values)
    Logarithmic,
    /// Exponential response (useful for amplitude, more sensitive at higher values)
    Exponential,
}

/// Metallic knob with inner glow indicator
pub struct Knob {
    /// Knob diameter
    diameter: f32,
    /// Label text
    label: Option<String>,
    /// Show value text
    show_value: bool,
    /// Knob color (default: metallic white/silver)
    color: Option<Color32>,
    /// Glow color for level indicator
    glow_color: Option<Color32>,
    /// Minimum angle in radians (default: -2.5)
    min_angle: f32,
    /// Maximum angle in radians (default: 2.5)
    max_angle: f32,
    /// Sensitivity multiplier for normal drag
    sensitivity: f32,
    /// Sensitivity for velocity mode
    velocity_sensitivity: f64,
    /// Response curve for value mapping
    curve: KnobCurve,
    /// Value range (min, max)
    value_range: (f32, f32),
    /// Show tick marks
    show_ticks: bool,
    /// Default value for double-click reset
    default_value: Option<f32>,
    /// Enable velocity-based drag mode
    velocity_mode: bool,
}

impl Knob {
    /// Create a new knob
    #[must_use]
    pub const fn new(_value: f32) -> Self {
        Self {
            diameter: 60.0,
            label: None,
            show_value: true,
            color: None,
            glow_color: None,
            min_angle: -2.5,
            max_angle: 2.5,
            sensitivity: 0.01,
            velocity_sensitivity: 1.0,
            curve: KnobCurve::Linear,
            value_range: (0.0, 1.0),
            show_ticks: false,
            default_value: None,
            velocity_mode: true, // Enabled by default for knobs
        }
    }

    /// Set the knob diameter
    #[must_use]
    pub const fn diameter(mut self, diameter: f32) -> Self {
        self.diameter = diameter;
        self
    }

    /// Set label text
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Show value text below knob
    #[must_use]
    pub const fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Set knob color (default: metallic silver)
    #[must_use]
    pub const fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Set glow color for level indicator
    #[must_use]
    pub const fn glow_color(mut self, color: Color32) -> Self {
        self.glow_color = Some(color);
        self
    }

    /// Set angle range in radians
    #[must_use]
    pub const fn angle_range(mut self, min: f32, max: f32) -> Self {
        self.min_angle = min;
        self.max_angle = max;
        self
    }

    /// Set drag sensitivity for normal (absolute) mode
    #[must_use]
    pub const fn sensitivity(mut self, sensitivity: f32) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    /// Set sensitivity for velocity-based drag mode
    #[must_use]
    pub const fn velocity_sensitivity(mut self, sensitivity: f64) -> Self {
        self.velocity_sensitivity = sensitivity;
        self
    }

    /// Set knob response curve for different control types
    #[must_use]
    pub const fn response_curve(mut self, curve: KnobCurve) -> Self {
        self.curve = curve;
        self
    }

    /// Set value range (min, max) for display purposes
    #[must_use]
    pub const fn value_range(mut self, min: f32, max: f32) -> Self {
        self.value_range = (min.min(max), min.max(max));
        self
    }

    /// Show tick marks at regular intervals
    #[must_use]
    pub const fn show_ticks(mut self, show: bool) -> Self {
        self.show_ticks = show;
        self
    }

    /// Set default value for double-click reset
    #[must_use]
    pub const fn default_value(mut self, value: f32) -> Self {
        self.default_value = Some(value);
        self
    }

    /// Enable/disable velocity-based drag mode (default: enabled)
    ///
    /// When enabled, holding Ctrl/Cmd while dragging uses velocity mode
    /// where faster mouse movement = larger value changes.
    #[must_use]
    pub const fn velocity_mode(mut self, enabled: bool) -> Self {
        self.velocity_mode = enabled;
        self
    }

    /// Show the knob
    pub fn show(self, ui: &mut Ui, value: &mut f32, theme: &Theme) -> KnobResponse {
        let desired_size = Vec2::splat(self.diameter);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        let mut changed = false;

        // Generate stable ID for drag state
        let knob_id = ui.make_persistent_id(format!("knob_{:?}", rect.min));
        let drag_state_id = knob_id.with("drag_state");

        // Handle input interactions
        changed |= self.handle_double_click(&mut response, value);
        self.handle_drag_start(ui, &response, *value, drag_state_id);
        changed |= self.handle_dragging(ui, &mut response, value, drag_state_id);
        self.handle_drag_end(ui, &response, drag_state_id);
        changed |= self.handle_mouse_wheel(ui, &mut response, value);

        // Render knob
        if ui.is_rect_visible(rect) {
            let center = rect.center();
            let radius = self.diameter / 2.0;
            let base_color = self
                .color
                .unwrap_or_else(|| Color32::from_rgb(210, 215, 222));
            let glow_color = self.glow_color.unwrap_or_else(|| theme.primary());

            self.render_knob(ui.painter(), center, radius, base_color, glow_color, *value);
        }

        KnobResponse {
            response,
            value: *value,
            changed,
        }
    }

    /// Handle double-click to reset value
    fn handle_double_click(&self, response: &mut Response, value: &mut f32) -> bool {
        if response.double_clicked() {
            if let Some(default) = self.default_value {
                if (*value - default).abs() > 0.001 {
                    *value = default.clamp(0.0, 1.0);
                    response.mark_changed();
                    return true;
                }
            }
        }
        false
    }

    /// Initialize drag state on drag start
    fn handle_drag_start(
        &self,
        ui: &mut Ui,
        response: &Response,
        value: f32,
        drag_state_id: egui::Id,
    ) {
        if response.drag_started() {
            let mut drag_state = KnobDragState {
                drag: VelocityDrag::new(
                    VelocityDragConfig::new().sensitivity(self.velocity_sensitivity),
                ),
                drag_start_value: value,
            };

            if let Some(pos) = response.interact_pointer_pos() {
                let use_velocity =
                    self.velocity_mode && ui.input(|i| i.modifiers.command || i.modifiers.ctrl);
                drag_state
                    .drag
                    .begin(f64::from(value), f64::from(pos.y), use_velocity);
            }

            ui.ctx()
                .data_mut(|d| d.insert_temp(drag_state_id, drag_state));
        }
    }

    /// Handle dragging with velocity or absolute mode
    fn handle_dragging(
        &self,
        ui: &mut Ui,
        response: &mut Response,
        value: &mut f32,
        drag_state_id: egui::Id,
    ) -> bool {
        if !response.dragged() {
            return false;
        }

        let drag_delta = response.drag_delta();
        let mut drag_state: KnobDragState = ui
            .ctx()
            .data_mut(|d| d.get_temp(drag_state_id).unwrap_or_default());

        let changed = if drag_state.drag.mode() == DragMode::Velocity {
            // Velocity mode: faster movement = larger change
            if let Some(pos) = response.interact_pointer_pos() {
                let delta = drag_state.drag.update_tracked(f64::from(pos.y), 1.0, 200.0);
                let new_value = drag_state.drag_start_value - delta as f32;
                if (new_value - *value).abs() > 0.0001 {
                    *value = new_value.clamp(0.0, 1.0);
                    response.mark_changed();
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            // Absolute mode: direct delta mapping
            let delta_y = -drag_delta.y;
            let delta_x = drag_delta.x;
            let primary_delta = if delta_x.abs() > delta_y.abs() {
                delta_x
            } else {
                delta_y
            };

            let sensitivity = if ui.input(|i| i.modifiers.shift) {
                self.sensitivity * 0.1
            } else {
                self.sensitivity
            };

            let delta = primary_delta * sensitivity;
            *value = (*value + delta).clamp(0.0, 1.0);
            response.mark_changed();
            true
        };

        ui.ctx()
            .data_mut(|d| d.insert_temp(drag_state_id, drag_state));
        changed
    }

    /// Clean up drag state on drag end
    fn handle_drag_end(&self, ui: &mut Ui, response: &Response, drag_state_id: egui::Id) {
        if response.drag_stopped() {
            ui.ctx().data_mut(|d| {
                let mut drag_state: KnobDragState = d.get_temp(drag_state_id).unwrap_or_default();
                drag_state.drag.end();
                d.insert_temp(drag_state_id, drag_state);
            });
        }
    }

    /// Handle mouse wheel for fine adjustment
    fn handle_mouse_wheel(&self, ui: &mut Ui, response: &mut Response, value: &mut f32) -> bool {
        if response.hovered() {
            let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll_delta.abs() > 0.0 {
                let wheel_sensitivity = 0.01;
                let delta = scroll_delta * wheel_sensitivity;
                *value = (*value + delta).clamp(0.0, 1.0);
                response.mark_changed();

                ui.ctx().input_mut(|i| i.smooth_scroll_delta = Vec2::ZERO);
                return true;
            }
        }
        false
    }

    /// Orchestrate all rendering layers
    fn render_knob(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        base_color: Color32,
        glow_color: Color32,
        value: f32,
    ) {
        self.render_outer_shadow(painter, center, radius);
        self.render_ceramic_body(painter, center, radius, base_color);
        self.render_shadow_arcs(painter, center, radius);
        self.render_glass_glaze(painter, center, radius);
        self.render_fresnel_effects(painter, center, radius);
        self.render_specular_highlights(painter, center, radius);
        self.render_sharp_highlights(painter, center, radius);
        self.render_edge_refraction(painter, center, radius);
        self.render_rim_effects(painter, center, radius, value, glow_color);
    }

    /// Render outer shadow for depth
    fn render_outer_shadow(&self, painter: &egui::Painter, center: Pos2, radius: f32) {
        for i in 0..6 {
            let shadow_radius = ((6 - i) as f32).mul_add(1.0, radius);
            let shadow_alpha = 20 - i * 3;
            painter.circle_filled(
                center,
                shadow_radius,
                Color32::from_rgba_unmultiplied(0, 0, 0, shadow_alpha as u8),
            );
        }
    }

    /// Render ceramic body with gradient
    fn render_ceramic_body(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        base_color: Color32,
    ) {
        let dark_base = Color32::from_rgb(185, 190, 200);
        let light_base = Color32::from_rgb(225, 230, 238);

        // Bottom half darker
        painter.circle_filled(center, radius, dark_base);

        // Top half lighter with gradient
        let gradient_center = Pos2::new(center.x, radius.mul_add(-0.2, center.y));
        for i in 0..8 {
            let grad_radius = radius * (i as f32).mul_add(-0.09, 1.0);
            let alpha = 25 + i * 8;
            painter.circle_filled(
                gradient_center,
                grad_radius,
                Color32::from_rgba_unmultiplied(
                    light_base.r(),
                    light_base.g(),
                    light_base.b(),
                    alpha as u8,
                ),
            );
        }

        // Main color
        painter.circle_filled(center, radius * 0.98, base_color);
    }

    /// Render shadow arcs for depth and ambient occlusion
    fn render_shadow_arcs(&self, painter: &egui::Painter, center: Pos2, radius: f32) {
        // Bottom shadow arc
        let shadow_arc_start = 0.85;
        let shadow_arc_end = 2.35;
        for i in 0..6 {
            let grad_radius = radius.mul_add(0.98, -(f32::from(i) * 0.3));
            self.draw_gradient_arc(
                painter,
                center,
                grad_radius,
                shadow_arc_start,
                shadow_arc_end,
                Color32::from_rgba_unmultiplied(145, 150, 165, 45 - i * 6),
            );
        }

        // Side ambient occlusion
        for i in 0..4 {
            let ao_radius = radius.mul_add(0.96, -(f32::from(i) * 0.5));
            self.draw_gradient_arc(
                painter,
                center,
                ao_radius,
                -3.3,
                -2.5,
                Color32::from_rgba_unmultiplied(170, 175, 185, 30 - i * 6),
            );
        }
    }

    /// Render glass glaze layers
    fn render_glass_glaze(&self, painter: &egui::Painter, center: Pos2, radius: f32) {
        // Base layer
        let glaze_center = Pos2::new(radius.mul_add(-0.1, center.x), radius.mul_add(-0.25, center.y));
        for i in 0..10 {
            let glaze_radius = radius * (i as f32).mul_add(-0.04, 0.75);
            let alpha = 20 + i * 3;
            painter.circle_filled(
                glaze_center,
                glaze_radius,
                Color32::from_rgba_unmultiplied(255, 255, 255, alpha as u8),
            );
        }

        // Mid layer
        let mid_glaze_center = Pos2::new(radius.mul_add(-0.15, center.x), radius.mul_add(-0.32, center.y));
        for i in 0..6 {
            let mid_radius = radius * (i as f32).mul_add(-0.05, 0.5);
            let alpha = 40 + i * 8;
            painter.circle_filled(
                mid_glaze_center,
                mid_radius,
                Color32::from_rgba_unmultiplied(255, 255, 255, alpha as u8),
            );
        }

        // Subsurface scattering
        let rim_glow_start = -1.8;
        let rim_glow_end = -0.3;
        for i in 0..4 {
            let rim_radius = radius.mul_add(0.94, -(f32::from(i) * 0.4));
            self.draw_gradient_arc(
                painter,
                center,
                rim_radius,
                rim_glow_start,
                rim_glow_end,
                Color32::from_rgba_unmultiplied(245, 248, 255, 35 - i * 7),
            );
        }
    }

    /// Render fresnel effects at edges
    fn render_fresnel_effects(&self, painter: &egui::Painter, center: Pos2, radius: f32) {
        for i in 0..5 {
            let fresnel_radius = radius * f32::from(i).mul_add(-0.02, 0.96);
            painter.circle_stroke(
                center,
                fresnel_radius,
                Stroke::new(
                    0.8,
                    Color32::from_rgba_unmultiplied(255, 255, 255, 30 - i * 5),
                ),
            );
        }
    }

    /// Render specular highlights
    fn render_specular_highlights(&self, painter: &egui::Painter, center: Pos2, radius: f32) {
        // Primary specular
        let specular_primary = Pos2::new(radius.mul_add(-0.2, center.x), radius.mul_add(-0.36, center.y));
        for i in 0..5 {
            let spec_radius = radius * (i as f32).mul_add(-0.025, 0.18);
            let alpha = 200 - i * 30;
            painter.circle_filled(
                specular_primary,
                spec_radius,
                Color32::from_rgba_unmultiplied(255, 255, 255, alpha as u8),
            );
        }

        // Secondary specular
        let specular_secondary = Pos2::new(radius.mul_add(0.15, center.x), radius.mul_add(-0.28, center.y));
        for i in 0..3 {
            let spec_radius = radius * (i as f32).mul_add(-0.02, 0.1);
            let alpha = 120 - i * 25;
            painter.circle_filled(
                specular_secondary,
                spec_radius,
                Color32::from_rgba_unmultiplied(255, 255, 255, alpha as u8),
            );
        }
    }

    /// Render sharp bright highlights
    fn render_sharp_highlights(&self, painter: &egui::Painter, center: Pos2, radius: f32) {
        // Primary bright spot
        let bright_spot_1 = Pos2::new(radius.mul_add(-0.25, center.x), radius.mul_add(-0.41, center.y));
        painter.circle_filled(
            bright_spot_1,
            radius * 0.08,
            Color32::from_rgba_unmultiplied(255, 255, 255, 255),
        );
        painter.circle_filled(
            bright_spot_1,
            radius * 0.05,
            Color32::from_rgba_unmultiplied(255, 255, 255, 255),
        );

        // Secondary bright spot
        let bright_spot_2 = Pos2::new(radius.mul_add(-0.15, center.x), radius.mul_add(-0.35, center.y));
        painter.circle_filled(
            bright_spot_2,
            radius * 0.04,
            Color32::from_rgba_unmultiplied(255, 255, 255, 240),
        );
    }

    /// Render edge refraction effects
    fn render_edge_refraction(&self, painter: &egui::Painter, center: Pos2, radius: f32) {
        for i in 0..3 {
            let refract_radius = radius * f32::from(i).mul_add(-0.01, 0.97);
            painter.circle_stroke(
                center,
                refract_radius,
                Stroke::new(
                    0.5,
                    Color32::from_rgba_unmultiplied(235, 240, 255, 20 - i * 5),
                ),
            );
        }
    }

    /// Render rim highlights and level indicator
    fn render_rim_effects(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        value: f32,
        glow_color: Color32,
    ) {
        // Draw level indicator
        self.draw_rim_indicator(
            painter,
            center,
            radius,
            value,
            glow_color,
            self.min_angle,
            self.max_angle,
        );

        // White rim highlight
        painter.circle_stroke(
            center,
            radius - 1.0,
            Stroke::new(2.0, Color32::from_rgba_unmultiplied(255, 255, 255, 200)),
        );

        // Bottom edge shadow
        painter.circle_stroke(
            center,
            radius + 0.5,
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, 50)),
        );
    }

    /// Draw gradient arc for ceramic depth
    fn draw_gradient_arc(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        color: Color32,
    ) {
        let segments = 24;
        for i in 0..segments {
            let t = i as f32 / segments as f32;
            let angle = start_angle + t * (end_angle - start_angle);
            let next_angle =
                ((i + 1) as f32 / segments as f32).mul_add(end_angle - start_angle, start_angle);

            let p1 = Pos2::new(
                angle.cos().mul_add(radius, center.x),
                angle.sin().mul_add(radius, center.y),
            );
            let p2 = Pos2::new(
                next_angle.cos().mul_add(radius, center.x),
                next_angle.sin().mul_add(radius, center.y),
            );

            painter.line_segment([p1, p2], Stroke::new(2.0, color));
        }
    }

    /// Draw level indicator on the rim
    #[allow(clippy::too_many_arguments)]
    fn draw_rim_indicator(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        value: f32,
        color: Color32,
        min_angle: f32,
        max_angle: f32,
    ) {
        let current_angle = value.mul_add(max_angle - min_angle, min_angle);
        let segments = 48;

        // Draw very subtle glow layers first (behind the solid rim)
        for glow_layer in 0..3 {
            let glow_offset = (3 - glow_layer) as f32 * 0.8; // Reduced offset for subtlety
            let glow_alpha = (20 - glow_layer * 5) as u8; // Consistent: 20, 15, 10
            let glow_color =
                Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), glow_alpha);

            for i in 0..segments {
                let t = i as f32 / segments as f32;
                let angle = min_angle + t * (current_angle - min_angle);

                if angle > current_angle {
                    break;
                }

                let next_angle =
                    ((i + 1) as f32 / segments as f32).mul_add(current_angle - min_angle, min_angle);

                let glow_radius = radius - 0.5 + glow_offset;
                let p1 = Pos2::new(
                    angle.cos().mul_add(glow_radius, center.x),
                    angle.sin().mul_add(glow_radius, center.y),
                );
                let p2 = Pos2::new(
                    next_angle.cos().mul_add(glow_radius, center.x),
                    next_angle.sin().mul_add(glow_radius, center.y),
                );

                painter.line_segment([p1, p2], Stroke::new(2.5 + glow_offset, glow_color));
                // Thinner stroke
            }
        }

        // Draw solid colored rim on top
        for i in 0..segments {
            let t = i as f32 / segments as f32;
            let angle = min_angle + t * (current_angle - min_angle);

            if angle > current_angle {
                break;
            }

            let next_angle =
                ((i + 1) as f32 / segments as f32).mul_add(current_angle - min_angle, min_angle);

            let outer_radius = radius - 0.5;
            let p1 = Pos2::new(
                angle.cos().mul_add(outer_radius, center.x),
                angle.sin().mul_add(outer_radius, center.y),
            );
            let p2 = Pos2::new(
                next_angle.cos().mul_add(outer_radius, center.x),
                next_angle.sin().mul_add(outer_radius, center.y),
            );

            // Solid colored rim
            painter.line_segment([p1, p2], Stroke::new(4.5, color));
        }
    }
}

impl Default for Knob {
    fn default() -> Self {
        Self::new(0.5)
    }
}
