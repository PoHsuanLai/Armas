//! XY Pad Component
//!
//! 2D touch controller for simultaneous control of two parameters.
//! Designed for filter controls, spatial effects, and expressive performance.
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

/// Trail history state (stored in egui temp data)
#[derive(Clone, Default)]
struct XYPadTrailState {
    points: Vec<(f32, f32)>,
}

impl XYPadTrailState {
    const MAX_POINTS: usize = 32;

    fn push(&mut self, x: f32, y: f32) {
        if self.points.len() >= Self::MAX_POINTS {
            self.points.remove(0);
        }
        self.points.push((x, y));
    }
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
    #[must_use]
    pub const fn changed(&self) -> bool {
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
/// # use armas::Theme;
/// # fn example(ui: &mut Ui, theme: &Theme) {
/// use armas_audio::XYPad;
///
/// let mut x = 0.5;
/// let mut y = 0.5;
///
/// let response = XYPad::new(&mut x, &mut y)
///     .size(200.0)
///     .show(ui, theme);
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
    show_trail: bool,
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
    pub const fn new(x: &'a mut f32, y: &'a mut f32) -> Self {
        Self {
            x,
            y,
            size: 200.0,
            variant: XYPadVariant::Filled,
            x_label: None,
            y_label: None,
            show_crosshair: true,
            show_values: false,
            show_trail: true,
            handle_size: 16.0,
            glow_intensity: 0.8,
            id: None,
            velocity_mode: true,
            velocity_sensitivity: 0.4,
            default_x: None,
            default_y: None,
        }
    }

    /// Set unique ID for state persistence
    #[must_use]
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set pad size (width and height)
    #[must_use]
    pub const fn size(mut self, size: f32) -> Self {
        self.size = size.max(100.0);
        self
    }

    /// Set visual variant
    #[must_use]
    pub const fn variant(mut self, variant: XYPadVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set X axis label
    #[must_use]
    pub fn x_label(mut self, label: impl Into<String>) -> Self {
        self.x_label = Some(label.into());
        self
    }

    /// Set Y axis label
    #[must_use]
    pub fn y_label(mut self, label: impl Into<String>) -> Self {
        self.y_label = Some(label.into());
        self
    }

    /// Show crosshair lines
    #[must_use]
    pub const fn show_crosshair(mut self, show: bool) -> Self {
        self.show_crosshair = show;
        self
    }

    /// Show numeric values
    #[must_use]
    pub const fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Show movement trail
    #[must_use]
    pub const fn show_trail(mut self, show: bool) -> Self {
        self.show_trail = show;
        self
    }

    /// Set handle size
    #[must_use]
    pub const fn handle_size(mut self, size: f32) -> Self {
        self.handle_size = size.max(8.0);
        self
    }

    /// Set glow intensity
    #[must_use]
    pub const fn glow_intensity(mut self, intensity: f32) -> Self {
        self.glow_intensity = intensity.clamp(0.0, 1.0);
        self
    }

    /// Enable velocity mode (Ctrl/Cmd for fine control). Default: true
    #[must_use]
    pub const fn velocity_mode(mut self, enabled: bool) -> Self {
        self.velocity_mode = enabled;
        self
    }

    /// Set sensitivity for velocity mode. Default: 1.0
    #[must_use]
    pub const fn velocity_sensitivity(mut self, sensitivity: f64) -> Self {
        self.velocity_sensitivity = sensitivity.max(0.1);
        self
    }

    /// Set default X value for double-click reset
    #[must_use]
    pub const fn default_x(mut self, value: f32) -> Self {
        self.default_x = Some(value.clamp(0.0, 1.0));
        self
    }

    /// Set default Y value for double-click reset
    #[must_use]
    pub const fn default_y(mut self, value: f32) -> Self {
        self.default_y = Some(value.clamp(0.0, 1.0));
        self
    }

    /// Set default values for both axes (convenience method)
    #[must_use]
    pub const fn default_values(mut self, x: f32, y: f32) -> Self {
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
                    .begin(f64::from(*self.x), f64::from(pos.x), use_velocity);
                drag_state
                    .drag_y
                    .begin(f64::from(*self.y), f64::from(pos.y), use_velocity);
            }
        }

        if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                if drag_state.drag_x.is_velocity_mode() {
                    // Velocity mode: incremental changes based on mouse speed
                    let delta_x = drag_state.drag_x.update_tracked(
                        f64::from(pos.x),
                        1.0,
                        f64::from(self.size),
                    );
                    let delta_y = drag_state.drag_y.update_tracked(
                        f64::from(pos.y),
                        1.0,
                        f64::from(self.size),
                    );

                    *self.x = (*self.x + delta_x as f32).clamp(0.0, 1.0);
                    // Y is inverted (up = higher value)
                    *self.y = (*self.y - delta_y as f32).clamp(0.0, 1.0);
                } else {
                    // Absolute mode: jump to cursor position
                    *self.x = ((pos.x - rect.min.x) / rect.width()).clamp(0.0, 1.0);
                    *self.y = 1.0 - ((pos.y - rect.min.y) / rect.height()).clamp(0.0, 1.0);
                }
                response.mark_changed();
            }
        }

        if response.drag_stopped() {
            drag_state.drag_x.end();
            drag_state.drag_y.end();
        }

        // Save drag state
        ui.ctx()
            .data_mut(|d| d.insert_temp(drag_state_id, drag_state));

        // Update trail state
        let trail_id = self
            .id
            .unwrap_or(response.id)
            .with("xy_pad_trail");
        let mut trail_state: XYPadTrailState = ui
            .ctx()
            .data_mut(|d| d.get_temp(trail_id).unwrap_or_default());
        if response.changed() {
            trail_state.push(*self.x, *self.y);
        }
        // Decay trail when not interacting
        if !response.dragged() && !trail_state.points.is_empty() {
            // Remove oldest point each frame to fade out
            trail_state.points.remove(0);
        }
        ui.ctx()
            .data_mut(|d| d.insert_temp(trail_id, trail_state.clone()));

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let corner_radius = f32::from(theme.spacing.corner_radius);
            let is_active = response.dragged() || response.is_pointer_button_down_on();

            // Handle position in pixel space
            let handle_x = (*self.x).mul_add(rect.width(), rect.min.x);
            let handle_y = (*self.y).mul_add(-rect.height(), rect.max.y);
            let handle_pos = Pos2::new(handle_x, handle_y);

            // Draw based on variant
            match self.variant {
                XYPadVariant::Filled => self.draw_filled(painter, theme, rect, corner_radius),
                XYPadVariant::Outlined => self.draw_outlined(painter, theme, rect, corner_radius),
                XYPadVariant::Elevated => self.draw_elevated(painter, theme, rect, corner_radius),
            }

            Self::draw_grid(painter, theme, rect);
            Self::draw_tick_marks(painter, theme, rect);

            if self.show_trail {
                Self::draw_trail(painter, theme, rect, &trail_state);
            }
            if self.show_crosshair {
                Self::draw_crosshair_lines(painter, theme, rect, handle_pos);
            }

            self.draw_handle(painter, theme, handle_pos, is_active);

            if is_active {
                Self::draw_coordinate_readout(painter, theme, rect, handle_pos, *self.x, *self.y, self.handle_size);
            }

            self.draw_labels(painter, theme, rect);
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

    /// Draw 4x4 grid lines with brighter center lines
    fn draw_grid(painter: &egui::Painter, theme: &Theme, rect: Rect) {
        let grid_color = theme.border();
        for i in 1..4u8 {
            let t = f32::from(i) / 4.0;
            let alpha = if i == 2 { 0.3 } else { 0.15 };
            let color = grid_color.gamma_multiply(alpha);
            let stroke = egui::Stroke::new(1.0, color);

            let gx = rect.min.x + t * rect.width();
            painter.line_segment(
                [Pos2::new(gx, rect.min.y), Pos2::new(gx, rect.max.y)],
                stroke,
            );
            let gy = rect.min.y + t * rect.height();
            painter.line_segment(
                [Pos2::new(rect.min.x, gy), Pos2::new(rect.max.x, gy)],
                stroke,
            );
        }
    }

    /// Draw axis tick marks on all four edges
    fn draw_tick_marks(painter: &egui::Painter, theme: &Theme, rect: Rect) {
        let tick_color = theme.border().gamma_multiply(0.25);
        let tick_len = 3.0;
        let stroke = egui::Stroke::new(1.0, tick_color);

        for i in 0..5u8 {
            let t = f32::from(i) / 4.0;

            let tx = rect.min.x + t * rect.width();
            painter.line_segment(
                [Pos2::new(tx, rect.max.y), Pos2::new(tx, rect.max.y - tick_len)],
                stroke,
            );
            painter.line_segment(
                [Pos2::new(tx, rect.min.y), Pos2::new(tx, rect.min.y + tick_len)],
                stroke,
            );

            let ty = rect.min.y + t * rect.height();
            painter.line_segment(
                [Pos2::new(rect.min.x, ty), Pos2::new(rect.min.x + tick_len, ty)],
                stroke,
            );
            painter.line_segment(
                [Pos2::new(rect.max.x, ty), Pos2::new(rect.max.x - tick_len, ty)],
                stroke,
            );
        }
    }

    /// Draw fading movement trail from recent handle positions
    fn draw_trail(
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        trail_state: &XYPadTrailState,
    ) {
        if trail_state.points.len() < 2 {
            return;
        }

        let primary = theme.primary();
        let (pr, pg, pb) = (primary.r(), primary.g(), primary.b());
        let total = trail_state.points.len();

        for i in 1..total {
            let t = i as f32 / total as f32;
            let alpha = (t * 120.0) as u8;
            let color = Color32::from_rgba_unmultiplied(pr, pg, pb, alpha);

            let (x0, y0) = trail_state.points[i - 1];
            let (x1, y1) = trail_state.points[i];

            let p0 = Pos2::new(
                x0.mul_add(rect.width(), rect.min.x),
                y0.mul_add(-rect.height(), rect.max.y),
            );
            let p1 = Pos2::new(
                x1.mul_add(rect.width(), rect.min.x),
                y1.mul_add(-rect.height(), rect.max.y),
            );

            painter.line_segment([p0, p1], egui::Stroke::new(2.0, color));
        }
    }

    /// Draw crosshair lines through the handle position
    fn draw_crosshair_lines(
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        handle_pos: Pos2,
    ) {
        let color = theme.muted_foreground().gamma_multiply(0.3);
        let stroke = egui::Stroke::new(1.0, color);

        painter.line_segment(
            [Pos2::new(rect.min.x, handle_pos.y), Pos2::new(rect.max.x, handle_pos.y)],
            stroke,
        );
        painter.line_segment(
            [Pos2::new(handle_pos.x, rect.min.y), Pos2::new(handle_pos.x, rect.max.y)],
            stroke,
        );
    }

    /// Draw the handle with glow, fill, highlight, specular dot, and border
    fn draw_handle(
        &self,
        painter: &egui::Painter,
        theme: &Theme,
        handle_pos: Pos2,
        is_active: bool,
    ) {
        let radius = self.handle_size / 2.0;

        // Glow rings when active
        if is_active {
            let primary = theme.primary();
            let (pr, pg, pb) = (primary.r(), primary.g(), primary.b());
            for i in 0..4u8 {
                let offset = f32::from(i + 1) * 2.0;
                let alpha = ((1.0 - f32::from(i) / 4.0) * 50.0 * self.glow_intensity) as u8;
                let glow_color = Color32::from_rgba_unmultiplied(pr, pg, pb, alpha);
                painter.circle_stroke(
                    handle_pos,
                    radius + offset,
                    egui::Stroke::new(2.0, glow_color),
                );
            }
        }

        // Main fill
        painter.circle_filled(handle_pos, radius, theme.primary());

        // Inner highlight (lighter center for depth)
        painter.circle_filled(
            handle_pos,
            radius * 0.6,
            Color32::from_rgba_unmultiplied(255, 255, 255, 40),
        );

        // Specular dot (top-left light catch)
        let spec_pos = Pos2::new(handle_pos.x - radius * 0.25, handle_pos.y - radius * 0.25);
        painter.circle_filled(
            spec_pos,
            radius * 0.2,
            Color32::from_rgba_unmultiplied(255, 255, 255, 70),
        );

        // Border stroke
        painter.circle_stroke(
            handle_pos,
            radius,
            egui::Stroke::new(1.5, theme.foreground().gamma_multiply(0.9)),
        );
    }

    /// Draw floating coordinate readout near the handle when dragging
    fn draw_coordinate_readout(
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        handle_pos: Pos2,
        x: f32,
        y: f32,
        handle_size: f32,
    ) {
        let radius = handle_size / 2.0;
        let coord_text = format!("{x:.2}, {y:.2}");
        let text_x = (handle_pos.x + radius + 6.0).min(rect.max.x - 40.0);
        let text_y = (handle_pos.y - radius - 14.0).max(rect.min.y + 2.0);
        painter.text(
            Pos2::new(text_x, text_y),
            egui::Align2::LEFT_BOTTOM,
            coord_text,
            egui::FontId::proportional(9.0),
            theme.foreground().gamma_multiply(0.7),
        );
    }

    /// Draw axis labels and value display
    fn draw_labels(&self, painter: &egui::Painter, theme: &Theme, rect: Rect) {
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
            let alpha = (i as f32).mul_add(-5.0, 20.0) as u8;
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
