//! Progress Components
//!
//! Progress indicators styled like shadcn/ui Progress.
//! Includes:
//! - Progress: Simple horizontal progress bar (shadcn style)
//! - `CircularProgressBar`: Circular/spinner progress

use egui::{Color32, Pos2, Ui, Vec2};
use std::f32::consts::PI;

const PROGRESS_HEIGHT: f32 = 8.0; // h-2 (8px)
const PROGRESS_CORNER_RADIUS: f32 = 9999.0; // rounded-full

const CIRCULAR_SIZE: f32 = 48.0;
const CIRCULAR_STROKE: f32 = 4.0;

/// Progress bar styled like shadcn/ui
///
/// A simple horizontal progress indicator.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas_basic::Progress;
/// use armas_basic::ext::ArmasContextExt;
///
/// let theme = ui.ctx().armas_theme();
/// // Basic progress (0-100)
/// Progress::new(65.0).show(ui, &theme);
///
/// // With custom width
/// Progress::new(33.0).width(200.0).show(ui, &theme);
/// # }
/// ```
pub struct Progress {
    /// Progress value (0 to 100)
    value: f32,
    /// Bar width (None = fill available)
    width: Option<f32>,
    /// Bar height
    height: f32,
}

impl Progress {
    /// Create a new progress bar
    ///
    /// # Arguments
    /// * `value` - Progress value from 0 to 100
    #[must_use]
    pub const fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 100.0),
            width: None,
            height: PROGRESS_HEIGHT,
        }
    }

    /// Set the width of the progress bar
    #[must_use]
    pub const fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the height of the progress bar
    #[must_use]
    pub const fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Show the progress bar
    pub fn show(self, ui: &mut Ui, theme: &crate::Theme) -> egui::Response {
        let desired_width = self.width.unwrap_or_else(|| ui.available_width());
        let corner_radius = PROGRESS_CORNER_RADIUS.min(self.height / 2.0);

        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(desired_width, self.height), egui::Sense::hover());

        if ui.is_rect_visible(rect) {
            // Background track: bg-primary/20 (primary at 20% opacity)
            let primary = theme.primary();
            let track_color = Color32::from_rgba_unmultiplied(
                primary.r(),
                primary.g(),
                primary.b(),
                51, // 20% of 255
            );

            ui.painter().rect_filled(rect, corner_radius, track_color);

            // Progress indicator: bg-primary
            let progress_fraction = self.value / 100.0;
            let fill_width = rect.width() * progress_fraction;

            if fill_width > 0.0 {
                let fill_rect =
                    egui::Rect::from_min_size(rect.min, Vec2::new(fill_width, self.height));

                ui.painter().rect_filled(fill_rect, corner_radius, primary);
            }
        }

        response
    }
}

/// Circular progress indicator
///
/// A circular progress display with optional percentage label.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas_basic::CircularProgressBar;
/// use armas_basic::ext::ArmasContextExt;
///
/// let theme = ui.ctx().armas_theme();
/// // Determinate progress (0-100)
/// CircularProgressBar::new(75.0)
///     .size(80.0)
///     .show_percentage(true)
///     .show(ui, &theme);
///
/// // Indeterminate/loading mode
/// CircularProgressBar::indeterminate()
///     .size(60.0)
///     .show(ui, &theme);
/// # }
/// ```
pub struct CircularProgressBar {
    /// Progress value (0 to 100), None for indeterminate
    value: Option<f32>,
    /// Circle diameter
    size: f32,
    /// Stroke width
    stroke_width: f32,
    /// Show percentage in center
    show_percentage: bool,
    /// Animation rotation for indeterminate mode
    rotation: f32,
}

impl CircularProgressBar {
    /// Create a determinate circular progress
    ///
    /// # Arguments
    /// * `value` - Progress value from 0 to 100
    #[must_use]
    pub const fn new(value: f32) -> Self {
        Self {
            value: Some(value.clamp(0.0, 100.0)),
            size: CIRCULAR_SIZE,
            stroke_width: CIRCULAR_STROKE,
            show_percentage: false,
            rotation: 0.0,
        }
    }

    /// Create an indeterminate circular progress (loading spinner)
    #[must_use]
    pub const fn indeterminate() -> Self {
        Self {
            value: None,
            size: CIRCULAR_SIZE,
            stroke_width: CIRCULAR_STROKE,
            show_percentage: false,
            rotation: 0.0,
        }
    }

    /// Set circle size (diameter)
    #[must_use]
    pub const fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set stroke width
    #[must_use]
    pub const fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    /// Show percentage in center (only for determinate mode)
    #[must_use]
    pub const fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    /// Show the circular progress
    pub fn show(mut self, ui: &mut Ui, theme: &crate::Theme) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(self.size), egui::Sense::hover());

        if ui.is_rect_visible(rect) {
            let center = rect.center();
            let radius = (self.size - self.stroke_width) / 2.0;
            let primary = theme.primary();

            // Background track: primary at 20% opacity
            let track_color =
                Color32::from_rgba_unmultiplied(primary.r(), primary.g(), primary.b(), 51);

            ui.painter().circle_stroke(
                center,
                radius,
                egui::Stroke::new(self.stroke_width, track_color),
            );

            if let Some(value) = self.value {
                // Determinate mode - arc from top
                let progress_fraction = value / 100.0;
                let arc_angle = progress_fraction * 2.0 * PI;
                self.draw_arc(ui, center, radius, -PI / 2.0, arc_angle, primary);

                // Percentage text
                if self.show_percentage {
                    let percentage = value as u32;
                    ui.painter().text(
                        center,
                        egui::Align2::CENTER_CENTER,
                        format!("{percentage}%"),
                        egui::FontId::proportional(self.size * 0.25),
                        theme.foreground(),
                    );
                }
            } else {
                // Indeterminate mode - rotating arc
                let dt = ui.input(|i| i.stable_dt);
                self.rotation += dt * 3.0;
                self.rotation %= 2.0 * PI;

                // Breathing arc length
                let breath_phase = (self.rotation * 2.0).sin() * 0.5 + 0.5;
                let arc_len = PI / 4.0 + breath_phase * PI / 2.0;

                self.draw_arc(ui, center, radius, self.rotation, arc_len, primary);

                ui.ctx().request_repaint();
            }
        }

        response
    }

    /// Draw an arc segment
    fn draw_arc(
        &self,
        ui: &mut Ui,
        center: Pos2,
        radius: f32,
        start_angle: f32,
        arc_length: f32,
        color: Color32,
    ) {
        let segments = 32;
        let angle_step = arc_length / segments as f32;

        for i in 0..segments {
            let angle1 = start_angle + i as f32 * angle_step;
            let angle2 = start_angle + (i + 1) as f32 * angle_step;

            let p1 = Pos2::new(
                center.x + radius * angle1.cos(),
                center.y + radius * angle1.sin(),
            );
            let p2 = Pos2::new(
                center.x + radius * angle2.cos(),
                center.y + radius * angle2.sin(),
            );

            ui.painter()
                .line_segment([p1, p2], egui::Stroke::new(self.stroke_width, color));
        }
    }
}
