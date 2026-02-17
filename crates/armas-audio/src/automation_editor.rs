//! Automation Editor Component
//!
//! Glass-forward automation curve editor with emphasis on negative space.
//! Control points and lines are minimal; the filled area dominates.

use armas_basic::theme::Theme;
use egui::{Color32, Pos2, Rect, Sense, Stroke, Ui, Vec2};

/// A single automation point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutomationPoint {
    /// Time position (0.0 to 1.0, normalized)
    pub time: f32,
    /// Value (0.0 to 1.0, normalized)
    pub value: f32,
}

impl AutomationPoint {
    /// Create a new automation point
    #[must_use]
    pub const fn new(time: f32, value: f32) -> Self {
        Self { time, value }
    }
}

/// Response from automation editor interaction
pub struct AutomationEditorResponse {
    /// The UI response
    pub response: egui::Response,
    /// Points after user interaction
    pub points: Vec<AutomationPoint>,
    /// Whether points were modified this frame
    pub modified: bool,
    /// Index of currently selected point (if any)
    pub selected: Option<usize>,
    /// Index of currently hovered point (if any)
    pub hovered: Option<usize>,
}

/// Automation curve editor with glassmorphic styling
pub struct AutomationEditor {
    /// Editor width
    width: f32,
    /// Editor height
    height: f32,
    /// Curve line width
    line_width: f32,
    /// Control point radius
    point_radius: f32,
    /// Show grid lines
    show_grid: bool,
    /// Number of measures
    measures: u32,
    /// Beats per measure (time signature)
    beats_per_measure: u32,
    /// Subdivision per beat (4 = 16th notes, 2 = 8th notes)
    subdivision: u32,
    /// Show center line (50% value reference)
    show_center_line: bool,
    /// Show simulated waveform background
    show_waveform: bool,
    /// Waveform opacity (0.0-1.0)
    waveform_opacity: f32,
    /// Allow editing
    editable: bool,
    /// Currently selected point index
    selected_point: Option<usize>,
    /// Whether the editor is disabled (non-interactive)
    disabled: bool,
}

impl AutomationEditor {
    /// Create a new automation editor
    #[must_use]
    pub const fn new() -> Self {
        Self {
            width: 400.0,
            height: 120.0,
            line_width: 1.0,
            point_radius: 5.0,
            show_grid: true,
            measures: 4,
            beats_per_measure: 4,
            subdivision: 4,
            show_center_line: true,
            show_waveform: true,
            waveform_opacity: 0.15,
            editable: true,
            selected_point: None,
            disabled: false,
        }
    }

    /// Set editor width
    #[must_use]
    pub const fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set editor height
    #[must_use]
    pub const fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Show simulated waveform background
    #[must_use]
    pub const fn show_waveform(mut self, show: bool) -> Self {
        self.show_waveform = show;
        self
    }

    /// Set waveform opacity (0.0-1.0)
    #[must_use]
    pub const fn waveform_opacity(mut self, opacity: f32) -> Self {
        self.waveform_opacity = opacity;
        self
    }

    /// Set curve line width
    #[must_use]
    pub const fn line_width(mut self, width: f32) -> Self {
        self.line_width = width;
        self
    }

    /// Set control point radius
    #[must_use]
    pub const fn point_radius(mut self, radius: f32) -> Self {
        self.point_radius = radius;
        self
    }

    /// Show grid lines
    #[must_use]
    pub const fn show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Set number of measures
    #[must_use]
    pub const fn measures(mut self, measures: u32) -> Self {
        self.measures = measures;
        self
    }

    /// Set beats per measure (time signature, default: 4)
    #[must_use]
    pub const fn beats_per_measure(mut self, beats: u32) -> Self {
        self.beats_per_measure = beats;
        self
    }

    /// Set subdivision per beat (4 = 16th notes, 2 = 8th notes, 1 = quarter notes)
    #[must_use]
    pub fn subdivision(mut self, subdivision: u32) -> Self {
        self.subdivision = subdivision.max(1);
        self
    }

    /// Show center line (50% value reference)
    #[must_use]
    pub const fn show_center_line(mut self, show: bool) -> Self {
        self.show_center_line = show;
        self
    }

    /// Enable/disable editing
    #[must_use]
    pub const fn editable(mut self, editable: bool) -> Self {
        self.editable = editable;
        self
    }

    /// Set whether the editor is disabled (non-interactive)
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set selected point index
    #[must_use]
    pub const fn selected(mut self, index: Option<usize>) -> Self {
        self.selected_point = index;
        self
    }

    /// Show the automation editor
    pub fn show(
        self,
        ui: &mut Ui,
        points: &mut Vec<AutomationPoint>,
        theme: &Theme,
    ) -> AutomationEditorResponse {
        let desired_size = Vec2::new(self.width, self.height);
        let sense = if self.disabled {
            Sense::hover()
        } else if self.editable {
            Sense::click_and_drag()
        } else {
            Sense::hover()
        };
        let (rect, response) = ui.allocate_exact_size(desired_size, sense);

        let mut modified = false;
        let mut selected = self.selected_point;
        let mut hovered: Option<usize> = None;

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // 1. Draw glass background panel
            self.draw_glass_background(painter, rect, theme);

            // 2. Draw simulated waveform (if enabled)
            if self.show_waveform {
                self.draw_waveform(painter, rect, theme);
            }

            // 3. Draw subtle grid (if enabled)
            if self.show_grid {
                self.draw_grid(painter, rect, theme);
            }

            // 4. Draw the thin curve line
            self.draw_curve_line(painter, rect, points, theme);

            // 5. Draw control points (small, minimal)
            hovered = self.find_hovered_point(ui, rect, points);
            self.draw_control_points(painter, rect, points, selected, hovered, theme);

            // 6. Draw top shimmer for glass effect
            self.draw_shimmer(painter, rect);
        }

        // Handle interactions
        if self.editable && !self.disabled {
            let interaction =
                self.handle_interaction(ui, &response, rect, points, &mut selected, hovered);
            modified = interaction;
        }

        AutomationEditorResponse {
            response,
            points: points.clone(),
            modified,
            selected,
            hovered,
        }
    }

    /// Draw glass background panel
    fn draw_glass_background(&self, painter: &egui::Painter, rect: Rect, theme: &Theme) {
        let corner_radius = f32::from(theme.spacing.corner_radius_small);

        // Glass surface
        let glass_color = Color32::from_rgba_unmultiplied(
            theme.card().r(),
            theme.card().g(),
            theme.card().b(),
            (255.0 * 0.7) as u8,
        );
        painter.rect_filled(rect, corner_radius, glass_color);

        // Subtle border
        painter.rect_stroke(
            rect,
            corner_radius,
            Stroke::new(1.0, theme.border()),
            egui::StrokeKind::Inside,
        );
    }

    /// Draw beat-aware grid lines (matching `SnapGrid` visual hierarchy)
    fn draw_grid(&self, painter: &egui::Painter, rect: Rect, theme: &Theme) {
        let base_color = theme.border();

        // Three-tier opacity hierarchy (matching SnapGrid)
        let measure_opacity: f32 = 0.5;
        let beat_opacity: f32 = 0.3;
        let subdivision_opacity: f32 = 0.15;

        let total_beats = self.measures * self.beats_per_measure;
        let total_subdivisions = total_beats * self.subdivision;
        let subdivision_width = rect.width() / total_subdivisions as f32;

        // Draw vertical grid lines
        for i in 0..=total_subdivisions {
            let x = (i as f32).mul_add(subdivision_width, rect.min.x);

            // Skip if at edges
            if i == 0 || i == total_subdivisions {
                continue;
            }

            let is_measure = i % (self.beats_per_measure * self.subdivision) == 0;
            let is_beat = i % self.subdivision == 0;

            let (opacity, width) = if is_measure {
                (measure_opacity, 1.5)
            } else if is_beat {
                (beat_opacity, 1.0)
            } else {
                (subdivision_opacity, 0.5)
            };

            let color = Color32::from_rgba_unmultiplied(
                base_color.r(),
                base_color.g(),
                base_color.b(),
                (255.0 * opacity) as u8,
            );

            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                Stroke::new(width, color),
            );
        }

        // Draw center line (50% value reference) if enabled
        if self.show_center_line {
            let y = rect.min.y + rect.height() * 0.5;
            let color = Color32::from_rgba_unmultiplied(
                base_color.r(),
                base_color.g(),
                base_color.b(),
                (255.0 * subdivision_opacity) as u8,
            );
            painter.line_segment(
                [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                Stroke::new(0.5, color),
            );
        }
    }

    /// Draw simulated waveform background (Logic Pro style - bars growing up from bottom)
    fn draw_waveform(&self, painter: &egui::Painter, rect: Rect, theme: &Theme) {
        let dim = if self.disabled { 0.5 } else { 1.0 };
        let primary = theme.primary().gamma_multiply(dim);
        let wave_color = Color32::from_rgba_unmultiplied(
            primary.r(),
            primary.g(),
            primary.b(),
            (255.0 * self.waveform_opacity) as u8,
        );

        // Number of vertical bars to draw
        let num_bars = (rect.width() / 2.0) as usize; // One bar every 2 pixels
        let bar_width = rect.width() / num_bars as f32;
        let bottom_y = rect.max.y;
        let max_amplitude = rect.height() * 0.7; // Max 70% of height

        // Use a simple deterministic pseudo-random pattern based on position
        // This creates a convincing waveform look without actual audio data
        for i in 0..num_bars {
            let t = i as f32 / num_bars as f32;
            let x = (i as f32).mul_add(bar_width, rect.min.x);

            // Generate amplitude using multiple sine waves for organic look
            let wave1 = (t * 23.7).sin();
            let wave2 = (t * 47.3).sin() * 0.5;
            let wave3 = (t * 97.1).sin() * 0.25;
            let envelope = (t * std::f32::consts::PI).sin().abs(); // Fade in/out at edges

            let amplitude = ((wave1 + wave2 + wave3).abs() * envelope * max_amplitude).max(1.0);

            // Draw vertical bar from bottom up (like drums hitting water)
            painter.line_segment(
                [Pos2::new(x, bottom_y), Pos2::new(x, bottom_y - amplitude)],
                Stroke::new(1.0, wave_color),
            );
        }
    }

    /// Draw the thin curve line
    fn draw_curve_line(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        points: &[AutomationPoint],
        theme: &Theme,
    ) {
        if points.len() < 2 {
            return;
        }

        let mut sorted_points = points.to_vec();
        sorted_points.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        let dim = if self.disabled { 0.5 } else { 1.0 };
        let line_color = theme.primary().gamma_multiply(dim);

        // Draw line segments between points
        for i in 0..sorted_points.len() - 1 {
            let p1 = &sorted_points[i];
            let p2 = &sorted_points[i + 1];

            let x1 = p1.time.mul_add(rect.width(), rect.min.x);
            let y1 = (1.0 - p1.value).mul_add(rect.height(), rect.min.y);
            let x2 = p2.time.mul_add(rect.width(), rect.min.x);
            let y2 = (1.0 - p2.value).mul_add(rect.height(), rect.min.y);

            painter.line_segment(
                [Pos2::new(x1, y1), Pos2::new(x2, y2)],
                Stroke::new(self.line_width, line_color),
            );
        }
    }

    /// Find which point is being hovered
    fn find_hovered_point(&self, ui: &Ui, rect: Rect, points: &[AutomationPoint]) -> Option<usize> {
        let pointer_pos = ui.input(|i| i.pointer.hover_pos())?;

        if !rect.contains(pointer_pos) {
            return None;
        }

        let hit_radius = self.point_radius * 2.5; // Generous hit area

        for (i, point) in points.iter().enumerate() {
            let x = point.time.mul_add(rect.width(), rect.min.x);
            let y = (1.0 - point.value).mul_add(rect.height(), rect.min.y);
            let point_pos = Pos2::new(x, y);

            if pointer_pos.distance(point_pos) < hit_radius {
                return Some(i);
            }
        }

        None
    }

    /// Draw control points - slider thumb style
    fn draw_control_points(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        points: &[AutomationPoint],
        selected: Option<usize>,
        hovered: Option<usize>,
        theme: &Theme,
    ) {
        let dim = if self.disabled { 0.5 } else { 1.0 };
        for (i, point) in points.iter().enumerate() {
            let x = point.time.mul_add(rect.width(), rect.min.x);
            let y = (1.0 - point.value).mul_add(rect.height(), rect.min.y);
            let center = Pos2::new(x, y);

            let is_selected = selected == Some(i);
            let is_hovered = hovered == Some(i);

            // Hover/select ring effect (like shadcn ring-4)
            if is_hovered || is_selected {
                let ring_color = theme.ring().gamma_multiply(0.5 * dim);
                painter.circle_filled(center, self.point_radius + 4.0, ring_color);
            }

            // Shadow
            painter.circle_filled(
                Pos2::new(center.x, center.y + 1.0),
                self.point_radius,
                Color32::from_black_alpha(40),
            );

            // Handle fill
            let handle_color = if is_selected {
                theme.primary().gamma_multiply(dim)
            } else {
                theme.foreground().gamma_multiply(dim)
            };
            painter.circle_filled(center, self.point_radius, handle_color);

            // Border
            painter.circle_stroke(
                center,
                self.point_radius,
                Stroke::new(1.0, theme.primary().gamma_multiply(dim)),
            );
        }
    }

    /// Draw top shimmer for glass effect
    fn draw_shimmer(&self, painter: &egui::Painter, rect: Rect) {
        let shimmer_height = 2.0;
        let shimmer_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), shimmer_height));
        let corner_radius = 4.0;

        painter.rect_filled(
            shimmer_rect,
            egui::CornerRadius {
                nw: corner_radius as u8,
                ne: corner_radius as u8,
                sw: 0,
                se: 0,
            },
            Color32::from_rgba_unmultiplied(255, 255, 255, 25),
        );
    }

    /// Handle user interactions (click to add, drag to move, etc.)
    fn handle_interaction(
        &self,
        ui: &Ui,
        response: &egui::Response,
        rect: Rect,
        points: &mut Vec<AutomationPoint>,
        selected: &mut Option<usize>,
        hovered: Option<usize>,
    ) -> bool {
        let mut modified = false;

        // Click to select or add point
        if response.clicked() {
            if let Some(hover_idx) = hovered {
                *selected = Some(hover_idx);
            } else if let Some(pos) = response.interact_pointer_pos() {
                // Add new point
                let time = ((pos.x - rect.min.x) / rect.width()).clamp(0.0, 1.0);
                let value = (1.0 - (pos.y - rect.min.y) / rect.height()).clamp(0.0, 1.0);
                points.push(AutomationPoint::new(time, value));
                *selected = Some(points.len() - 1);
                modified = true;
            }
        }

        // Double-click to delete point
        if response.double_clicked() {
            if let Some(hover_idx) = hovered {
                if points.len() > 2 {
                    points.remove(hover_idx);
                    *selected = None;
                    modified = true;
                }
            }
        }

        // Drag to move selected point
        if response.dragged() {
            if let Some(sel_idx) = *selected {
                if let Some(pos) = response.interact_pointer_pos() {
                    let time = ((pos.x - rect.min.x) / rect.width()).clamp(0.0, 1.0);
                    let value = (1.0 - (pos.y - rect.min.y) / rect.height()).clamp(0.0, 1.0);

                    if sel_idx < points.len() {
                        points[sel_idx] = AutomationPoint::new(time, value);
                        modified = true;
                    }
                }
            }
        }

        // Request repaint if hovering for smooth feedback
        if hovered.is_some() || selected.is_some() {
            ui.ctx().request_repaint();
        }

        modified
    }
}

impl Default for AutomationEditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automation_point_creation() {
        let point = AutomationPoint::new(0.5, 0.75);
        assert_eq!(point.time, 0.5);
        assert_eq!(point.value, 0.75);
    }

    #[test]
    fn test_automation_editor_builder() {
        let editor = AutomationEditor::new()
            .width(400.0)
            .height(200.0)
            .waveform_opacity(0.2);

        assert_eq!(editor.width, 400.0);
        assert_eq!(editor.height, 200.0);
        assert_eq!(editor.waveform_opacity, 0.2);
    }
}
