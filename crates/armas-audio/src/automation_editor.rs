//! Automation Editor Component
//!
//! Professional parameter automation curve editor for DAW-style applications.
//! Provides timeline-based editing with multiple curve types and real-time visualization.

use crate::{
    AutomationCanvas, CanvasConfig, CurveTypeSelector, PointHandle, ValueRangeDisplay,
};
use armas::ext::ArmasContextExt;
use armas::theme::Theme;
use audio_automation::{AutomationEnvelope, AutomationPoint, CurveType};
use egui::{Color32, Pos2, Stroke, Ui, Vec2};

/// Main automation editor component
pub struct AutomationEditor<'a, T> {
    envelope: &'a mut AutomationEnvelope<T>,
    canvas_size: Vec2,
    canvas_config: CanvasConfig,
    playhead_pos: Option<f64>,
    show_point_values: bool,
    selected_point: Option<usize>,
    point_color: Color32,
}

impl<'a, T: Clone + std::fmt::Debug> AutomationEditor<'a, T> {
    /// Create a new automation editor
    pub fn new(envelope: &'a mut AutomationEnvelope<T>) -> Self {
        Self {
            envelope,
            canvas_size: Vec2::new(600.0, 300.0),
            canvas_config: CanvasConfig::default(),
            playhead_pos: None,
            show_point_values: true,
            selected_point: None,
            point_color: Color32::from_rgb(100, 150, 255),
        }
    }

    /// Set canvas size
    pub fn canvas_size(mut self, size: Vec2) -> Self {
        self.canvas_size = size;
        self
    }

    /// Set canvas configuration
    pub fn canvas_config(mut self, config: CanvasConfig) -> Self {
        self.canvas_config = config;
        self
    }

    /// Set playhead position in beats
    pub fn playhead(mut self, pos: f64) -> Self {
        self.playhead_pos = Some(pos);
        self
    }

    /// Set point color
    pub fn point_color(mut self, color: Color32) -> Self {
        self.point_color = color;
        self
    }

    /// Show value labels on hover
    pub fn show_values(mut self, show: bool) -> Self {
        self.show_point_values = show;
        self
    }

    /// Set selected point index
    pub fn selected_point(mut self, idx: Option<usize>) -> Self {
        self.selected_point = idx;
        self
    }

    /// Show the automation editor
    pub fn show(mut self, ui: &mut Ui) -> AutomationEditorResponse {
        let theme = ui.ctx().armas_theme();

        let mut response = AutomationEditorResponse::default();

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = theme.spacing.md;

            // Toolbar with curve type selector
            CurveTypeSelector::new(CurveType::Linear).show(ui);

            ui.add_space(theme.spacing.sm);

            // Main editor area with canvas and value display
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = theme.spacing.md;

                // Value range display (Y-axis)
                ValueRangeDisplay::new(
                    self.canvas_config.min_value,
                    self.canvas_config.max_value,
                )
                .markers(5)
                .show(ui, self.canvas_size.y);

                // Canvas with automation curve
                let canvas_response = AutomationCanvas::new(&theme)
                    .config(self.canvas_config.clone())
                    .playhead(self.playhead_pos.unwrap_or(0.0))
                    .show(ui, self.canvas_size);

                // Render automation points and curve
                self.render_points_and_curve(
                    ui,
                    &theme,
                    canvas_response.canvas_rect,
                    &canvas_response,
                    &mut response,
                );
            });
        });

        response
    }

    /// Render automation points and curve visualization
    fn render_points_and_curve(
        &mut self,
        ui: &mut Ui,
        theme: &Theme,
        canvas_rect: egui::Rect,
        canvas_response: &crate::CanvasResponse,
        response: &mut AutomationEditorResponse,
    ) {
        let painter = ui.painter();
        if self.envelope.points.is_empty() {
            return;
        }

        // Sample curve for visualization
        self.render_curve_line(painter, canvas_rect, canvas_response);

        // Collect point data first to avoid borrow conflicts
        let mut point_updates = Vec::new();

        // Render points as interactive handles
        for (idx, point) in self.envelope.points.iter().enumerate() {
            let x = canvas_response.time_to_pixel(point.time);
            let y = canvas_response.value_to_pixel(point.value);
            let pos = Pos2::new(x, y);

            let is_selected = self.selected_point == Some(idx);

            let handle_response = PointHandle::new(pos, self.point_color)
                .selected(is_selected)
                .show_value(self.show_point_values)
                .value_text(format!("{:.2}", point.value))
                .show(ui, &theme);

            if handle_response.response.clicked() {
                self.selected_point = Some(idx);
                response.selected_point = Some(idx);
            }

            // Handle dragging
            if let Some(delta) = handle_response.drag_delta {
                let new_time = canvas_response.pixel_to_time(x + delta.x);
                let new_value = canvas_response.pixel_to_value(y + delta.y);

                // Clamp values
                let clamped_value = new_value
                    .max(self.canvas_config.min_value)
                    .min(self.canvas_config.max_value);

                // Snap time if enabled
                let snapped_time = canvas_response.snap_time(new_time.max(0.0));

                point_updates.push((idx, snapped_time, clamped_value));
            }
        }

        // Apply collected updates
        for (idx, snapped_time, clamped_value) in point_updates {
            self.envelope.points[idx].time = snapped_time;
            self.envelope.points[idx].value = clamped_value;

            response.point_edited = Some((idx, self.envelope.points[idx].clone()));
            response.changed = true;
        }
    }

    /// Render the curve line connecting all points
    fn render_curve_line(
        &self,
        painter: &egui::Painter,
        canvas_rect: egui::Rect,
        canvas_response: &crate::CanvasResponse,
    ) {
        if self.envelope.points.len() < 2 {
            return;
        }

        let num_samples = (self.canvas_size.x as usize).max(100);
        let sample_step = (self.envelope.points.last().unwrap().time) as f32 / num_samples as f32;

        let mut points = Vec::new();

        for i in 0..=num_samples {
            let time = (i as f32 * sample_step) as f64;

            if let Some(value) = self.envelope.get_value_at(time) {
                let x = canvas_response.time_to_pixel(time);
                let y = canvas_response.value_to_pixel(value);

                if x >= canvas_rect.min.x && x <= canvas_rect.max.x {
                    points.push(Pos2::new(x, y));
                }
            }
        }

        // Draw curve line
        if points.len() > 1 {
            // Draw glow effect (outer layers)
            painter.add(egui::Shape::line(
                points.clone(),
                Stroke::new(4.5, self.point_color.gamma_multiply(0.25)),
            ));

            painter.add(egui::Shape::line(
                points.clone(),
                Stroke::new(3.0, self.point_color.gamma_multiply(0.35)),
            ));

            // Draw main curve line
            painter.add(egui::Shape::line(
                points.clone(),
                Stroke::new(2.0, self.point_color),
            ));

            // Draw bright inner line (highlight)
            painter.add(egui::Shape::line(
                points.clone(),
                Stroke::new(0.8, self.point_color.gamma_multiply(1.3)),
            ));

            // Draw filled area under curve with gradient-like effect
            let mut filled_points = points.clone();
            filled_points.push(Pos2::new(filled_points.last().unwrap().x, canvas_rect.max.y));
            filled_points.push(Pos2::new(filled_points.first().unwrap().x, canvas_rect.max.y));

            painter.add(egui::Shape::convex_polygon(
                filled_points,
                self.point_color.gamma_multiply(0.15),
                Stroke::NONE,
            ));
        }
    }
}

/// Response from automation editor interaction
#[derive(Debug, Clone, Default)]
pub struct AutomationEditorResponse {
    /// Whether any changes were made
    pub changed: bool,
    /// Index of currently selected point
    pub selected_point: Option<usize>,
    /// (index, point) if a point was edited
    pub point_edited: Option<(usize, AutomationPoint)>,
    /// Index if a point was added
    pub point_added: Option<usize>,
    /// Index if a point was deleted
    pub point_deleted: Option<usize>,
}
