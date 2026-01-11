use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{Color32, Pos2, Ui, Vec2};
use std::f32::consts::PI;

/// Linear progress bar
///
/// A horizontal progress indicator with determinate and indeterminate modes.
///
/// # Example
///
/// ```rust,no_run
/// use armas::{Theme, components::LinearProgress};
///
/// fn ui(ui: &mut egui::Ui) {
///     let theme = Theme::dark();
///
///     // Determinate progress (0.0 to 1.0)
///     LinearProgress::new(0.65).show(ui, &theme);
///
///     // Indeterminate/loading mode
///     LinearProgress::indeterminate().show(ui, &theme);
/// }
/// ```
pub struct LinearProgress {
    /// Progress value (0.0 to 1.0), None for indeterminate
    progress: Option<f32>,
    /// Bar height
    height: f32,
    /// Bar width (None = fill available)
    width: Option<f32>,
    /// Primary color
    color: Option<Color32>,
    /// Show percentage text
    show_label: bool,
    /// Animation phase for indeterminate mode
    phase: f32,
}

impl LinearProgress {
    /// Create a determinate progress bar
    pub fn new(progress: f32) -> Self {
        Self {
            progress: Some(progress.clamp(0.0, 1.0)),
            height: 4.0,
            width: None,
            color: None,
            show_label: false,
            phase: 0.0,
        }
    }

    /// Create an indeterminate progress bar
    pub fn indeterminate() -> Self {
        Self {
            progress: None,
            height: 4.0,
            width: None,
            color: None,
            show_label: false,
            phase: 0.0,
        }
    }

    /// Set bar height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set bar width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set progress color
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Show percentage label
    pub fn show_label(mut self) -> Self {
        self.show_label = true;
        self
    }

    /// Show the progress bar
    pub fn show(mut self, ui: &mut Ui) -> egui::Response {
        let theme = ui.ctx().armas_theme();
        let desired_width = self.width.unwrap_or(ui.available_width());
        let total_height = if self.show_label && self.progress.is_some() {
            self.height + 20.0
        } else {
            self.height
        };

        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(desired_width, total_height), egui::Sense::hover());

        let bar_rect = egui::Rect::from_min_size(rect.min, Vec2::new(desired_width, self.height));

        let primary_color = self.color.unwrap_or(theme.primary());

        // Background track
        ui.painter()
            .rect_filled(bar_rect, self.height / 2.0, theme.surface_variant());

        if let Some(progress) = self.progress {
            // Determinate mode
            let fill_width = bar_rect.width() * progress;
            let fill_rect =
                egui::Rect::from_min_size(bar_rect.min, Vec2::new(fill_width, self.height));

            ui.painter()
                .rect_filled(fill_rect, self.height / 2.0, primary_color);

            // Label
            if self.show_label {
                let percentage = (progress * 100.0) as u32;
                let label_y = rect.min.y + self.height + 4.0;
                ui.painter().text(
                    Pos2::new(rect.center().x, label_y),
                    egui::Align2::CENTER_TOP,
                    format!("{}%", percentage),
                    egui::FontId::proportional(12.0),
                    theme.on_surface_variant(),
                );
            }
        } else {
            // Indeterminate mode - animated sweep
            let dt = ui.input(|i| i.stable_dt);
            self.phase += dt * 2.0;
            self.phase %= 2.0;

            // Moving segment
            let segment_width = bar_rect.width() * 0.3;
            let x_pos = if self.phase < 1.0 {
                // Move left to right
                bar_rect.min.x + (bar_rect.width() - segment_width) * self.phase
            } else {
                // Move right to left
                bar_rect.max.x
                    - segment_width
                    - (bar_rect.width() - segment_width) * (self.phase - 1.0)
            };

            let fill_rect = egui::Rect::from_min_size(
                Pos2::new(x_pos, bar_rect.min.y),
                Vec2::new(segment_width, self.height),
            );

            ui.painter()
                .rect_filled(fill_rect, self.height / 2.0, primary_color);

            ui.ctx().request_repaint();
        }

        response
    }
}

/// Circular/ring progress indicator
///
/// A circular progress display with optional center label.
///
/// # Example
///
/// ```rust,no_run
/// use armas::{Theme, components::CircularProgressBar};
///
/// fn ui(ui: &mut egui::Ui) {
///     let theme = Theme::dark();
///
///     // Determinate progress (0.0 to 1.0)
///     CircularProgressBar::new(0.75)
///         .size(80.0)
///         .show_percentage(true)
///         .show(ui, &theme);
///
///     // Indeterminate/loading mode
///     CircularProgressBar::indeterminate()
///         .size(60.0)
///         .show(ui, &theme);
/// }
/// ```
pub struct CircularProgressBar {
    /// Progress value (0.0 to 1.0), None for indeterminate
    progress: Option<f32>,
    /// Circle diameter
    size: f32,
    /// Stroke width
    stroke_width: f32,
    /// Primary color
    color: Option<Color32>,
    /// Show percentage in center
    show_percentage: bool,
    /// Animation rotation for indeterminate mode
    rotation: f32,
}

impl CircularProgressBar {
    /// Create a determinate circular progress
    pub fn new(progress: f32) -> Self {
        Self {
            progress: Some(progress.clamp(0.0, 1.0)),
            size: 48.0,
            stroke_width: 4.0,
            color: None,
            show_percentage: false,
            rotation: 0.0,
        }
    }

    /// Create an indeterminate circular progress
    pub fn indeterminate() -> Self {
        Self {
            progress: None,
            size: 48.0,
            stroke_width: 4.0,
            color: None,
            show_percentage: false,
            rotation: 0.0,
        }
    }

    /// Set circle size
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set stroke width
    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    /// Set progress color
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Show percentage in center
    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    /// Show the circular progress
    pub fn show(mut self, ui: &mut Ui) {
        let theme = ui.ctx().armas_theme();
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(self.size), egui::Sense::hover());

        let center = rect.center();
        let radius = (self.size - self.stroke_width) / 2.0;
        let primary_color = self.color.unwrap_or(theme.primary());

        // Background circle
        ui.painter().circle_stroke(
            center,
            radius,
            egui::Stroke::new(self.stroke_width, theme.surface_variant()),
        );

        if let Some(progress) = self.progress {
            // Determinate mode - arc from top
            let arc_angle = progress * 2.0 * PI;
            self.draw_arc(ui, center, radius, -PI / 2.0, arc_angle, primary_color);

            // Percentage text
            if self.show_percentage {
                let percentage = (progress * 100.0) as u32;
                ui.painter().text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    format!("{}%", percentage),
                    egui::FontId::proportional(self.size * 0.25),
                    theme.on_surface(),
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

            self.draw_arc(ui, center, radius, self.rotation, arc_len, primary_color);

            ui.ctx().request_repaint();
        }
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

/// Ring progress with label
pub struct RingProgress {
    /// Progress value (0.0 to 1.0)
    progress: f32,
    /// Ring diameter
    size: f32,
    /// Ring thickness
    thickness: f32,
    /// Label text
    label: Option<String>,
    /// Primary color
    color: Option<Color32>,
}

impl RingProgress {
    /// Create a new ring progress
    pub fn new(progress: f32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            size: 120.0,
            thickness: 12.0,
            label: None,
            color: None,
        }
    }

    /// Set ring size
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set ring thickness
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set label text
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set progress color
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Show the ring progress
    pub fn show(self, ui: &mut Ui) {
        let theme = ui.ctx().armas_theme();
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(self.size), egui::Sense::hover());

        let center = rect.center();
        let outer_radius = self.size / 2.0;
        let inner_radius = outer_radius - self.thickness;
        let mid_radius = (outer_radius + inner_radius) / 2.0;

        let primary_color = self.color.unwrap_or(theme.primary());

        // Background ring
        self.draw_ring(
            ui,
            center,
            mid_radius,
            egui::Stroke::new(self.thickness, theme.surface_variant()),
            0.0,
            2.0 * PI,
        );

        // Progress ring
        let arc_angle = self.progress * 2.0 * PI;
        self.draw_ring(
            ui,
            center,
            mid_radius,
            egui::Stroke::new(self.thickness, primary_color),
            -PI / 2.0,
            arc_angle,
        );

        // Center content
        let percentage = (self.progress * 100.0) as u32;
        ui.painter().text(
            Pos2::new(center.x, center.y - 10.0),
            egui::Align2::CENTER_CENTER,
            format!("{}%", percentage),
            egui::FontId::proportional(self.size * 0.2),
            theme.on_surface(),
        );

        if let Some(label) = &self.label {
            ui.painter().text(
                Pos2::new(center.x, center.y + 12.0),
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::proportional(self.size * 0.12),
                theme.on_surface_variant(),
            );
        }
    }

    /// Draw a ring segment
    fn draw_ring(
        &self,
        ui: &mut Ui,
        center: Pos2,
        radius: f32,
        stroke: egui::Stroke,
        start_angle: f32,
        arc_length: f32,
    ) {
        let segments = 64;
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

            ui.painter().line_segment([p1, p2], stroke);
        }
    }
}
