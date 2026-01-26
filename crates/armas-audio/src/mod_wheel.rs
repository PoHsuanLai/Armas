//! Mod Wheel Component
//!
//! Vertical strip controller for modulation, pitch bend, and expression.
//! Essential for expressive MIDI performance.

use armas::animation::{VelocityDrag, VelocityDragConfig};
use armas::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Type of wheel controller
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WheelType {
    /// Modulation wheel (stays at position)
    Modulation,
    /// Pitch bend wheel (springs back to center)
    PitchBend,
    /// Expression wheel (stays at position)
    Expression,
}

/// Visual style variant for wheel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WheelVariant {
    /// Filled wheel with solid background
    Filled,
    /// Outlined wheel with transparent background
    Outlined,
    /// Elevated wheel with shadow effect
    Elevated,
}

/// Internal drag state for ModWheel
#[derive(Clone)]
struct ModWheelDragState {
    drag: VelocityDrag,
}

/// Mod Wheel controller component
///
/// A vertical strip controller for modulation, pitch bend, and expression.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::components::audio::{ModWheel, WheelType};
///
/// let mut value = 0.0;
///
/// let response = ModWheel::new(&mut value)
///     .wheel_type(WheelType::Modulation)
///     .label("Mod")
///     .show(ui);
///
/// if response.changed() {
///     println!("Modulation: {:.2}", value);
/// }
/// # }
/// ```
pub struct ModWheel<'a> {
    value: &'a mut f32,
    wheel_type: WheelType,
    variant: WheelVariant,
    width: f32,
    height: f32,
    label: Option<String>,
    show_value: bool,
    show_center_line: bool,
    glow_intensity: f32,
    id: Option<egui::Id>,
    /// Enable velocity-based dragging (Ctrl/Cmd for fine control)
    velocity_mode: bool,
    /// Sensitivity for velocity mode
    velocity_sensitivity: f64,
    /// Default value for double-click reset
    default_value: Option<f32>,
}

impl<'a> ModWheel<'a> {
    /// Create a new mod wheel
    pub fn new(value: &'a mut f32) -> Self {
        Self {
            value,
            wheel_type: WheelType::Modulation,
            variant: WheelVariant::Filled,
            width: 40.0,
            height: 200.0,
            label: None,
            show_value: false,
            show_center_line: false,
            glow_intensity: 0.8,
            id: None,
            velocity_mode: true,
            velocity_sensitivity: 1.0,
            default_value: None,
        }
    }

    /// Set unique ID for state persistence
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set wheel type
    pub fn wheel_type(mut self, wheel_type: WheelType) -> Self {
        self.wheel_type = wheel_type;
        // Show center line by default for pitch bend
        if wheel_type == WheelType::PitchBend {
            self.show_center_line = true;
        }
        self
    }

    /// Set visual variant
    pub fn variant(mut self, variant: WheelVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(20.0);
        self
    }

    /// Set height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(100.0);
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Show numeric value
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Show center line (for pitch bend)
    pub fn show_center_line(mut self, show: bool) -> Self {
        self.show_center_line = show;
        self
    }

    /// Set glow intensity
    pub fn glow_intensity(mut self, intensity: f32) -> Self {
        self.glow_intensity = intensity.clamp(0.0, 1.0);
        self
    }

    /// Enable velocity-based dragging
    ///
    /// When enabled, holding Ctrl/Cmd while dragging provides fine control
    /// where faster mouse movement creates larger value changes.
    pub fn velocity_mode(mut self, enabled: bool) -> Self {
        self.velocity_mode = enabled;
        self
    }

    /// Set sensitivity for velocity mode
    ///
    /// Higher values make the wheel more responsive to mouse speed.
    /// Default is 1.0.
    pub fn velocity_sensitivity(mut self, sensitivity: f64) -> Self {
        self.velocity_sensitivity = sensitivity.max(0.1);
        self
    }

    /// Set default value for double-click reset
    ///
    /// When set, double-clicking the wheel resets it to this value.
    pub fn default_value(mut self, value: f32) -> Self {
        self.default_value = Some(value);
        self
    }

    /// Show the mod wheel
    pub fn show(self, ui: &mut Ui, theme: &armas::Theme) -> Response {

        // Load previous state if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("mod_wheel_state");
            *self.value = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id).unwrap_or(*self.value));
        }

        // For pitch bend, value ranges from -1.0 to 1.0, otherwise 0.0 to 1.0
        let (min_val, max_val) = match self.wheel_type {
            WheelType::PitchBend => (-1.0, 1.0),
            _ => (0.0, 1.0),
        };

        *self.value = self.value.clamp(min_val, max_val);

        let desired_size = Vec2::new(self.width, self.height);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        // Get or create drag state
        let drag_id = self
            .id
            .unwrap_or_else(|| ui.id())
            .with("mod_wheel_drag");
        let mut drag_state: ModWheelDragState = ui.ctx().data_mut(|d| {
            d.get_temp(drag_id).unwrap_or_else(|| ModWheelDragState {
                drag: VelocityDrag::new(
                    VelocityDragConfig::new().sensitivity(self.velocity_sensitivity),
                ),
            })
        });

        // Handle double-click reset
        if response.double_clicked() {
            if let Some(default) = self.default_value {
                *self.value = default;
                response.mark_changed();
            }
        }

        // Handle drag interaction
        if response.drag_started() {
            let use_velocity = self.velocity_mode
                && ui.input(|i| i.modifiers.command || i.modifiers.ctrl);
            drag_state
                .drag
                .begin(*self.value as f64, response.interact_pointer_pos().map_or(0.0, |p| p.y) as f64, use_velocity);
        }

        if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                if drag_state.drag.is_velocity_mode() {
                    // Velocity mode: value changes based on mouse speed
                    let value_range = (max_val - min_val) as f64;
                    let delta =
                        drag_state
                            .drag
                            .update_tracked(pos.y as f64, value_range, self.height as f64);
                    // Invert delta since moving up should increase value
                    *self.value = (*self.value as f64 - delta).clamp(min_val as f64, max_val as f64) as f32;
                } else {
                    // Absolute mode: Y position to value (inverted: top = max, bottom = min)
                    let normalized = 1.0 - ((pos.y - rect.min.y) / rect.height()).clamp(0.0, 1.0);
                    *self.value = min_val + normalized * (max_val - min_val);
                }
                response.mark_changed();
            }
        }

        if response.drag_stopped() {
            drag_state.drag.end();
        }

        // Pitch bend springs back to center when released
        if self.wheel_type == WheelType::PitchBend && response.drag_stopped() {
            *self.value = 0.0;
            response.mark_changed();
        }

        // Store drag state
        ui.ctx().data_mut(|d| d.insert_temp(drag_id, drag_state));

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let corner_radius = theme.spacing.corner_radius_small as f32;

            // Draw based on variant
            match self.variant {
                WheelVariant::Filled => {
                    self.draw_filled(painter, &theme, rect, corner_radius);
                }
                WheelVariant::Outlined => {
                    self.draw_outlined(painter, &theme, rect, corner_radius);
                }
                WheelVariant::Elevated => {
                    self.draw_elevated(painter, &theme, rect, corner_radius);
                }
            }

            // Draw center line (for pitch bend)
            if self.show_center_line {
                let center_y = rect.center().y;
                painter.line_segment(
                    [
                        Pos2::new(rect.min.x + 4.0, center_y),
                        Pos2::new(rect.max.x - 4.0, center_y),
                    ],
                    egui::Stroke::new(1.0, theme.border()),
                );
            }

            // Draw handle
            let normalized = (*self.value - min_val) / (max_val - min_val);
            let handle_y = rect.max.y - normalized * rect.height();
            let handle_rect = Rect::from_min_size(
                Pos2::new(rect.min.x + 2.0, handle_y - 8.0),
                Vec2::new(self.width - 4.0, 16.0),
            );

            // Draw realistic wheel handle
            self.draw_wheel_handle(
                painter,
                &theme,
                handle_rect,
                response.dragged() || response.is_pointer_button_down_on(),
            );

            // Draw label
            if let Some(label) = &self.label {
                painter.text(
                    Pos2::new(rect.center().x, rect.max.y + theme.spacing.sm),
                    egui::Align2::CENTER_TOP,
                    label,
                    egui::FontId::proportional(10.0),
                    theme.muted_foreground(),
                );
            }

            // Draw value
            if self.show_value {
                let value_text = format!("{:.2}", self.value);
                painter.text(
                    Pos2::new(rect.center().x, rect.min.y - theme.spacing.sm),
                    egui::Align2::CENTER_BOTTOM,
                    value_text,
                    egui::FontId::proportional(10.0),
                    theme.foreground(),
                );
            }
        }

        // Save state to memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("mod_wheel_state");
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id, *self.value);
            });
        }

        response
    }

    fn draw_wheel_handle(
        &self,
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        is_dragging: bool,
    ) {
        let handle_color = theme.primary();

        // Base color components from primary
        let base_r = handle_color.r();
        let base_g = handle_color.g();
        let base_b = handle_color.b();

        // Glow effect when dragging (outer shadow)
        if is_dragging {
            for i in 0..5 {
                let offset = (i + 1) as f32 * 2.0;
                let alpha = ((1.0 - i as f32 / 5.0) * 60.0 * self.glow_intensity) as u8;
                let glow_color = Color32::from_rgba_unmultiplied(base_r, base_g, base_b, alpha);
                painter.rect_filled(rect.expand(offset), 6.0, glow_color);
            }
        }

        // Wheel body rounded corners (more wheel-like)
        let wheel_radius = 6.0;

        // Shadow/depth on bottom and right edges for 3D effect
        let shadow_rect = rect.translate(Vec2::new(0.5, 0.8));
        painter.rect_filled(
            shadow_rect,
            wheel_radius,
            Color32::from_rgba_unmultiplied(0, 0, 0, 50),
        );

        // Main wheel body with vertical gradient for cylindrical depth
        let gradient_steps = rect.height() as usize;
        for i in 0..gradient_steps {
            let t = i as f32 / gradient_steps.max(1) as f32;
            // Create cylindrical effect: darker at edges, lighter at center
            let brightness = if t < 0.5 {
                0.75 + t * 0.5 // 0.75 to 1.0
            } else {
                1.25 - t * 0.5 // 1.0 to 0.75
            };

            let color = Color32::from_rgb(
                (base_r as f32 * brightness).min(255.0) as u8,
                (base_g as f32 * brightness).min(255.0) as u8,
                (base_b as f32 * brightness).min(255.0) as u8,
            );

            let y = rect.min.y + i as f32;
            if y < rect.max.y {
                painter.line_segment(
                    [
                        Pos2::new(rect.min.x + 2.0, y),
                        Pos2::new(rect.max.x - 2.0, y),
                    ],
                    egui::Stroke::new(1.0, color),
                );
            }
        }

        // Top highlight (wheel edge catching light)
        painter.line_segment(
            [
                Pos2::new(rect.min.x + 3.0, rect.min.y + 1.0),
                Pos2::new(rect.max.x - 3.0, rect.min.y + 1.0),
            ],
            egui::Stroke::new(1.5, Color32::from_rgba_unmultiplied(255, 255, 255, 100)),
        );

        // Horizontal grip ridges (suggesting tactile texture)
        let ridge_spacing = 2.5;
        let ridge_count = (rect.height() / ridge_spacing) as usize;
        for i in 0..ridge_count {
            let y = rect.min.y + 2.0 + i as f32 * ridge_spacing;
            if y < rect.max.y - 2.0 {
                painter.line_segment(
                    [
                        Pos2::new(rect.min.x + 4.0, y),
                        Pos2::new(rect.max.x - 4.0, y),
                    ],
                    egui::Stroke::new(0.5, Color32::from_rgba_unmultiplied(0, 0, 0, 70)),
                );
            }
        }

        // Center vertical groove (finger grip indent)
        let groove_x = rect.center().x;
        let groove_top = rect.min.y + 3.0;
        let groove_bottom = rect.max.y - 3.0;

        // Dark line for groove
        painter.line_segment(
            [Pos2::new(groove_x - 0.5, groove_top), Pos2::new(groove_x - 0.5, groove_bottom)],
            egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, 100)),
        );

        // Light highlight on right side of groove
        painter.line_segment(
            [
                Pos2::new(groove_x + 0.5, groove_top),
                Pos2::new(groove_x + 0.5, groove_bottom),
            ],
            egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 50)),
        );

        // Outer border/edge for definition
        painter.rect_stroke(
            rect,
            wheel_radius,
            egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, 140)),
            egui::StrokeKind::Outside,
        );

        // Inner highlight border for depth
        let inner_rect = rect.shrink(1.0);
        painter.rect_stroke(
            inner_rect,
            wheel_radius - 1.0,
            egui::Stroke::new(0.5, Color32::from_rgba_unmultiplied(255, 255, 255, 30)),
            egui::StrokeKind::Inside,
        );
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
            let shadow_rect = rect.translate(Vec2::new(offset * 0.5, offset));
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
    fn test_mod_wheel_creation() {
        let mut value = 0.5;
        let _wheel = ModWheel::new(&mut value);
        assert_eq!(value, 0.5);
    }

    #[test]
    fn test_wheel_type() {
        let mut value = 0.5;
        let wheel = ModWheel::new(&mut value)
            .wheel_type(WheelType::PitchBend)
            .label("Pitch");

        assert_eq!(wheel.wheel_type, WheelType::PitchBend);
        assert!(wheel.show_center_line); // Auto-enabled for pitch bend
    }

    #[test]
    fn test_pitch_bend_range() {
        let mut value = 0.5;
        let wheel = ModWheel::new(&mut value).wheel_type(WheelType::PitchBend);

        // Set to -1.0
        *wheel.value = -1.0;
        assert_eq!(*wheel.value, -1.0);
    }
}
