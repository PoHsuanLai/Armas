//! Automation curve canvas with grid rendering
//!
//! Handles the main canvas area where curves are visualized and edited

use crate::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

/// Configuration for the automation canvas
#[derive(Debug, Clone)]
pub struct CanvasConfig {
    /// Horizontal pixels per beat
    pub pixels_per_beat: f32,
    /// Vertical pixels per unit value
    pub pixels_per_value: f32,
    /// Minimum value on Y axis
    pub min_value: f32,
    /// Maximum value on Y axis
    pub max_value: f32,
    /// Number of beat subdivisions to show grid for
    pub grid_subdivisions: usize,
    /// Enable grid snapping
    pub snap_enabled: bool,
    /// Snap interval in beats
    pub snap_interval: f64,
}

impl Default for CanvasConfig {
    fn default() -> Self {
        Self {
            pixels_per_beat: 60.0,
            pixels_per_value: 40.0,
            min_value: 0.0,
            max_value: 1.0,
            grid_subdivisions: 4,
            snap_enabled: true,
            snap_interval: 0.25,
        }
    }
}

/// Canvas for rendering and interacting with automation curves
pub struct AutomationCanvas<'a> {
    config: CanvasConfig,
    theme: &'a Theme,
    playhead_pos: Option<f64>,
}

impl<'a> AutomationCanvas<'a> {
    /// Create a new automation canvas
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            config: CanvasConfig::default(),
            theme,
            playhead_pos: None,
        }
    }

    /// Set canvas configuration
    pub fn config(mut self, config: CanvasConfig) -> Self {
        self.config = config;
        self
    }

    /// Set playhead position in beats
    pub fn playhead(mut self, pos: f64) -> Self {
        self.playhead_pos = Some(pos);
        self
    }

    /// Show the canvas and handle rendering
    pub fn show(self, ui: &mut Ui, size: Vec2) -> CanvasResponse {
        let (rect, response) = ui.allocate_exact_size(size, Sense::hover());

        if ui.is_rect_visible(rect) {
            self.render_canvas(ui.painter(), rect);
        }

        CanvasResponse {
            response,
            canvas_rect: rect,
            config: self.config,
        }
    }

    /// Render the canvas background and grid
    fn render_canvas(&self, painter: &egui::Painter, rect: Rect) {
        let corner_radius = 8u8;
        let corner_radius_f32 = corner_radius as f32;

        // Background with rounded corners
        painter.rect_filled(rect, corner_radius_f32, self.theme.card());

        // Border glow effect (inner) - using multiple circles for glow
        let rounding = egui::epaint::CornerRadius::same(corner_radius);
        let border_glow = self.theme.primary().gamma_multiply(0.3);
        for offset in &[2.0, 3.0, 4.0] {
            let glow_rect = rect.expand(*offset);
            painter.rect_stroke(
                glow_rect,
                rounding,
                egui::Stroke::new(0.5, border_glow.gamma_multiply(0.5 / offset)),
                egui::epaint::StrokeKind::Outside,
            );
        }

        // Main border stroke
        painter.rect_stroke(
            rect,
            rounding,
            egui::Stroke::new(1.5, self.theme.primary().gamma_multiply(0.5)),
            egui::epaint::StrokeKind::Outside,
        );

        // Draw grid
        self.draw_grid(painter, rect);

        // Draw playhead if set
        if let Some(pos) = self.playhead_pos {
            self.draw_playhead(painter, rect, pos);
        }

        // Vignette effect (faded edges)
        self.draw_vignette(painter, rect, corner_radius_f32);
    }

    /// Draw vignette effect at canvas edges
    fn draw_vignette(&self, painter: &egui::Painter, rect: Rect, _corner_radius: f32) {
        let edge_width = 30.0;
        let num_steps = 5;

        // Top vignette
        for i in 0..num_steps {
            let progress = i as f32 / num_steps as f32;
            let alpha = (progress * 30.0) as u8;
            let color = self.theme.card().gamma_multiply(0.5);
            let color = Color32::from_rgba_unmultiplied(
                color.r(),
                color.g(),
                color.b(),
                alpha,
            );

            let y = rect.min.y + (progress * edge_width);
            painter.hline(
                rect.min.x..=rect.max.x,
                y,
                egui::Stroke::new(1.0, color),
            );
        }

        // Bottom vignette
        for i in 0..num_steps {
            let progress = i as f32 / num_steps as f32;
            let alpha = (progress * 30.0) as u8;
            let color = self.theme.card().gamma_multiply(0.5);
            let color = Color32::from_rgba_unmultiplied(
                color.r(),
                color.g(),
                color.b(),
                alpha,
            );

            let y = rect.max.y - (progress * edge_width);
            painter.hline(
                rect.min.x..=rect.max.x,
                y,
                egui::Stroke::new(1.0, color),
            );
        }
    }

    /// Draw grid lines and beats
    fn draw_grid(&self, painter: &egui::Painter, rect: Rect) {
        let mut x = rect.min.x;
        let beat_width = self.config.pixels_per_beat;
        let sub_width = beat_width / self.config.grid_subdivisions as f32;

        // Draw vertical lines (time)
        while x <= rect.max.x {
            // Determine if this is a beat or subdivision
            let progress = (x - rect.min.x) / beat_width;
            let is_beat = (progress.fract() * self.config.grid_subdivisions as f32).abs() < 0.01;

            let color = if is_beat {
                self.theme.primary().gamma_multiply(0.3)
            } else {
                self.theme.border().gamma_multiply(0.2)
            };

            let stroke_width = if is_beat { 1.0 } else { 0.5 };

            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                Stroke::new(stroke_width, color),
            );

            x += sub_width;
        }

        // Draw horizontal lines (values)
        let num_lines = 5; // Major divisions

        for i in 0..=num_lines {
            let y = rect.max.y - (i as f32 * rect.height() / num_lines as f32);

            painter.line_segment(
                [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                Stroke::new(0.5, self.theme.border().gamma_multiply(0.15)),
            );
        }
    }

    /// Draw the playhead indicator
    fn draw_playhead(&self, painter: &egui::Painter, rect: Rect, pos: f64) {
        let x = rect.min.x + (pos as f32 * self.config.pixels_per_beat);

        if x >= rect.min.x && x <= rect.max.x {
            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                Stroke::new(2.0, self.theme.secondary()),
            );

            // Add glow effect
            for i in 0..2 {
                let offset = (i + 1) as f32 * 1.5;
                let alpha = ((1.0 - i as f32 / 2.0) * 15.0) as u8;
                let glow_color =
                    Color32::from_rgba_unmultiplied(
                        self.theme.secondary().r(),
                        self.theme.secondary().g(),
                        self.theme.secondary().b(),
                        alpha,
                    );

                painter.line_segment(
                    [Pos2::new(x - offset, rect.min.y), Pos2::new(x - offset, rect.max.y)],
                    Stroke::new(1.0, glow_color),
                );
                painter.line_segment(
                    [Pos2::new(x + offset, rect.min.y), Pos2::new(x + offset, rect.max.y)],
                    Stroke::new(1.0, glow_color),
                );
            }
        }
    }
}

/// Response from canvas rendering
pub struct CanvasResponse {
    pub response: Response,
    pub canvas_rect: Rect,
    pub config: CanvasConfig,
}

impl CanvasResponse {
    /// Convert time coordinate to pixel position
    pub fn time_to_pixel(&self, time: f64) -> f32 {
        self.canvas_rect.min.x + (time as f32 * self.config.pixels_per_beat)
    }

    /// Convert pixel position to time coordinate
    pub fn pixel_to_time(&self, pixel: f32) -> f64 {
        ((pixel - self.canvas_rect.min.x) / self.config.pixels_per_beat) as f64
    }

    /// Convert value to pixel position (Y axis)
    pub fn value_to_pixel(&self, value: f32) -> f32 {
        let normalized = (value - self.config.min_value) / (self.config.max_value - self.config.min_value);
        self.canvas_rect.max.y - (normalized * self.canvas_rect.height())
    }

    /// Convert pixel position to value
    pub fn pixel_to_value(&self, pixel: f32) -> f32 {
        let normalized = (self.canvas_rect.max.y - pixel) / self.canvas_rect.height();
        self.config.min_value + (normalized * (self.config.max_value - self.config.min_value))
    }

    /// Snap time to grid if enabled
    pub fn snap_time(&self, time: f64) -> f64 {
        if self.config.snap_enabled {
            (time / self.config.snap_interval).round() * self.config.snap_interval
        } else {
            time
        }
    }
}
