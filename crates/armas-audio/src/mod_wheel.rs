//! Mod Wheel Component
//!
//! Rotating cylinder controller for modulation, pitch bend, and expression.
//! Renders as a 3D cylinder visible through a recessed slot, with scrolling
//! grip ridges that simulate rotation as the value changes.

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

/// Size preset for wheel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WheelSize {
    /// Small - 30x120
    Small,
    /// Default - 40x180
    #[default]
    Default,
    /// Large - 50x220
    Large,
}

impl WheelSize {
    /// Width in pixels
    const fn width(self) -> f32 {
        match self {
            Self::Small => 30.0,
            Self::Default => 40.0,
            Self::Large => 50.0,
        }
    }

    /// Height in pixels
    const fn height(self) -> f32 {
        match self {
            Self::Small => 120.0,
            Self::Default => 180.0,
            Self::Large => 220.0,
        }
    }
}

/// Internal drag state for `ModWheel`
#[derive(Clone)]
struct ModWheelDragState {
    drag: VelocityDrag,
}

/// Mod Wheel controller component
///
/// A rotating cylinder controller for modulation, pitch bend, and expression.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # use armas::Theme;
/// # fn example(ui: &mut Ui, theme: &Theme) {
/// use armas_audio::{ModWheel, WheelType};
///
/// let mut value = 0.0;
///
/// let response = ModWheel::new(&mut value)
///     .wheel_type(WheelType::Modulation)
///     .label("Mod")
///     .show(ui, theme);
///
/// if response.changed() {
///     println!("Modulation: {:.2}", value);
/// }
/// # }
/// ```
pub struct ModWheel<'a> {
    value: &'a mut f32,
    wheel_type: WheelType,
    size: WheelSize,
    label: Option<String>,
    show_value: bool,
    show_center_line: bool,
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
    pub const fn new(value: &'a mut f32) -> Self {
        Self {
            value,
            wheel_type: WheelType::Modulation,
            size: WheelSize::Default,
            label: None,
            show_value: false,
            show_center_line: false,
            id: None,
            velocity_mode: true,
            velocity_sensitivity: 1.0,
            default_value: None,
        }
    }

    /// Set unique ID for state persistence
    #[must_use]
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set wheel type
    #[must_use]
    pub fn wheel_type(mut self, wheel_type: WheelType) -> Self {
        self.wheel_type = wheel_type;
        // Show center line by default for pitch bend
        if wheel_type == WheelType::PitchBend {
            self.show_center_line = true;
        }
        self
    }

    /// Set size preset
    #[must_use]
    pub const fn size(mut self, size: WheelSize) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Show numeric value
    #[must_use]
    pub const fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Show center line (for pitch bend)
    #[must_use]
    pub const fn show_center_line(mut self, show: bool) -> Self {
        self.show_center_line = show;
        self
    }

    /// Enable velocity-based dragging
    ///
    /// When enabled, holding Ctrl/Cmd while dragging provides fine control
    /// where faster mouse movement creates larger value changes.
    #[must_use]
    pub const fn velocity_mode(mut self, enabled: bool) -> Self {
        self.velocity_mode = enabled;
        self
    }

    /// Set sensitivity for velocity mode
    ///
    /// Higher values make the wheel more responsive to mouse speed.
    /// Default is 1.0.
    #[must_use]
    pub const fn velocity_sensitivity(mut self, sensitivity: f64) -> Self {
        self.velocity_sensitivity = sensitivity.max(0.1);
        self
    }

    /// Set default value for double-click reset
    ///
    /// When set, double-clicking the wheel resets it to this value.
    #[must_use]
    pub const fn default_value(mut self, value: f32) -> Self {
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

        let desired_size = Vec2::new(self.size.width(), self.size.height());
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        // Get or create drag state
        let drag_id = self.id.unwrap_or_else(|| ui.id()).with("mod_wheel_drag");
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
            let use_velocity =
                self.velocity_mode && ui.input(|i| i.modifiers.command || i.modifiers.ctrl);
            drag_state.drag.begin(
                f64::from(*self.value),
                f64::from(response.interact_pointer_pos().map_or(0.0, |p| p.y)),
                use_velocity,
            );
        }

        if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                if drag_state.drag.is_velocity_mode() {
                    // Velocity mode: value changes based on mouse speed
                    let value_range = f64::from(max_val - min_val);
                    let delta = drag_state.drag.update_tracked(
                        f64::from(pos.y),
                        value_range,
                        f64::from(self.size.height()),
                    );
                    // Invert delta since moving up should increase value
                    *self.value = (f64::from(*self.value) - delta)
                        .clamp(f64::from(min_val), f64::from(max_val))
                        as f32;
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

        // Handle mouse wheel scroll for fine adjustment
        if response.hovered() {
            let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll_delta.abs() > 0.0 {
                let sensitivity = 0.005;
                let delta = scroll_delta * sensitivity;
                *self.value = (*self.value + delta).clamp(min_val, max_val);
                response.mark_changed();
                ui.ctx().input_mut(|i| i.smooth_scroll_delta = Vec2::ZERO);
            }
        }

        // Store drag state
        ui.ctx().data_mut(|d| d.insert_temp(drag_id, drag_state));

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let corner_radius = f32::from(theme.spacing.corner_radius_small);

            // Draw housing
            painter.rect_filled(rect, corner_radius, theme.muted());
            painter.rect_stroke(
                rect,
                corner_radius,
                egui::Stroke::new(1.0, theme.border()),
                egui::StrokeKind::Outside,
            );

            // Calculate cylinder surface area (inset from housing)
            let inset = 3.0;
            let cylinder_rect = rect.shrink2(Vec2::new(inset, inset));

            // Calculate ridge offset from current value (scrolling texture)
            let normalized = (*self.value - min_val) / (max_val - min_val);
            let ridge_offset = normalized * cylinder_rect.height();

            // Draw the rotating cylinder surface
            let is_active = response.dragged() || response.is_pointer_button_down_on();
            self.draw_cylinder_surface(painter, theme, cylinder_rect, ridge_offset, is_active);

            // Draw center line (for pitch bend) on top of cylinder
            if self.show_center_line {
                let center_y = cylinder_rect.center().y;
                painter.line_segment(
                    [
                        Pos2::new(cylinder_rect.min.x + 2.0, center_y),
                        Pos2::new(cylinder_rect.max.x - 2.0, center_y),
                    ],
                    egui::Stroke::new(1.0, theme.border()),
                );
            }

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

    /// Draw the cylinder surface visible through the slot.
    ///
    /// The cylinder stays in place — only the grip ridges scroll vertically
    /// to simulate rotation, creating the illusion of a real mod wheel.
    fn draw_cylinder_surface(
        &self,
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        ridge_offset: f32,
        is_active: bool,
    ) {
        // --- Layer 1: Dark recessed slot background ---
        painter.rect_filled(rect, 2.0, Color32::from_rgb(15, 15, 18));

        // --- Layer 2: Cylinder body with horizontal cylindrical shading ---
        // Shade column-by-column: darkest at left/right edges, brightest at center
        // This simulates a convex cylinder lit from the front.
        let bg = theme.muted();
        let bg_lum = (u16::from(bg.r()) + u16::from(bg.g()) + u16::from(bg.b())) / 3;
        let is_dark_theme = bg_lum < 128;
        let base_gray: u8 = if is_dark_theme { 85 } else { 160 };
        let brightness_boost: f32 = if is_active { 0.08 } else { 0.0 };

        let width = rect.width();
        let steps = width.max(1.0) as usize;

        for i in 0..steps {
            let t = i as f32 / (steps.max(2) - 1) as f32;
            // Cosine falloff: 1.0 at center (t=0.5), 0.0 at edges
            let angle = (t - 0.5) * std::f32::consts::PI;
            let cylinder_brightness = angle.cos();
            // Map to brightness range: ~0.55 at edges, ~1.0 at center
            let brightness = (0.55 + 0.45 * cylinder_brightness + brightness_boost).min(1.15);

            let gray = (f32::from(base_gray) * brightness).min(255.0) as u8;
            let color = Color32::from_rgb(gray, gray, gray);

            let x = rect.min.x + i as f32;
            if x < rect.max.x {
                painter.line_segment(
                    [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                    egui::Stroke::new(1.0, color),
                );
            }
        }

        // --- Layer 3: Scrolling grip ridges (clipped to slot) ---
        let clipped = painter.with_clip_rect(rect);
        let ridge_spacing = 4.0;
        // Enough ridges to cover the slot plus full scroll range
        let total_extent = rect.height() * 2.0;
        let ridge_count = (total_extent / ridge_spacing) as usize + 2;
        let base_y = rect.min.y - rect.height();

        for i in 0..ridge_count {
            let y = base_y + (i as f32 * ridge_spacing) + (ridge_offset % ridge_spacing);
            if y > rect.min.y - ridge_spacing && y < rect.max.y + ridge_spacing {
                // Dark ridge line
                clipped.line_segment(
                    [
                        Pos2::new(rect.min.x + 3.0, y),
                        Pos2::new(rect.max.x - 3.0, y),
                    ],
                    egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, 80)),
                );
                // Light highlight below each ridge (3D groove effect)
                clipped.line_segment(
                    [
                        Pos2::new(rect.min.x + 3.0, y + 1.0),
                        Pos2::new(rect.max.x - 3.0, y + 1.0),
                    ],
                    egui::Stroke::new(0.5, Color32::from_rgba_unmultiplied(255, 255, 255, 30)),
                );
            }
        }

        // --- Layer 4: Top edge shadow (slot rim casts shadow downward) ---
        for i in 0..6u8 {
            let alpha = 40u8.saturating_sub(i * 6);
            let y = rect.min.y + f32::from(i);
            painter.line_segment(
                [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, alpha)),
            );
        }

        // --- Layer 5: Bottom edge shadow (slot rim casts shadow upward) ---
        for i in 0..6u8 {
            let alpha = 40u8.saturating_sub(i * 6);
            let y = rect.max.y - f32::from(i);
            painter.line_segment(
                [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, alpha)),
            );
        }

        // --- Layer 6: Left/right edge darkening (curvature reinforcement) ---
        for i in 0..3u8 {
            let alpha = 50u8.saturating_sub(i * 15);
            // Left edge
            let lx = rect.min.x + f32::from(i);
            painter.line_segment(
                [Pos2::new(lx, rect.min.y), Pos2::new(lx, rect.max.y)],
                egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, alpha)),
            );
            // Right edge
            let rx = rect.max.x - f32::from(i);
            painter.line_segment(
                [Pos2::new(rx, rect.min.y), Pos2::new(rx, rect.max.y)],
                egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, alpha)),
            );
        }

        // --- Layer 7: Center specular highlight ---
        let cx = rect.center().x;
        for offset in -1..=1_i32 {
            let alpha = if offset == 0 { 35u8 } else { 15u8 };
            painter.line_segment(
                [
                    Pos2::new(cx + offset as f32, rect.min.y + 6.0),
                    Pos2::new(cx + offset as f32, rect.max.y - 6.0),
                ],
                egui::Stroke::new(
                    1.0,
                    Color32::from_rgba_unmultiplied(255, 255, 255, alpha),
                ),
            );
        }

        // --- Layer 8: Slot border (crisp edge definition) ---
        painter.rect_stroke(
            rect,
            2.0,
            egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, 120)),
            egui::StrokeKind::Inside,
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
