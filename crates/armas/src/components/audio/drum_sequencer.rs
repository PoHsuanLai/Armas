//! Drum Sequencer Component
//!
//! Professional multi-row drum sequencer for DAW-style pattern programming.
//! Each row represents a drum sound with independent step patterns and velocity control.

use crate::ext::ArmasContextExt;
use crate::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};
use std::collections::HashMap;

/// Visual style variant for drum sequencer steps
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrumSequencerVariant {
    /// Filled steps with solid backgrounds
    Filled,
    /// Outlined steps with transparent backgrounds
    Outlined,
    /// Elevated steps with shadow effect
    Elevated,
}

/// Color scheme for drum sequencer rows
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrumSequencerColorScheme {
    /// Use theme semantic colors for each row
    Semantic,
    /// Use a single primary color for all rows
    Monochrome,
}

/// Individual drum step data
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrumStep {
    /// Whether this step is active (on/off)
    pub active: bool,
    /// Velocity (0.0-1.0), only used if active
    pub velocity: f32,
}

impl Default for DrumStep {
    fn default() -> Self {
        Self {
            active: false,
            velocity: 1.0,
        }
    }
}

/// Drum sequencer row configuration
#[derive(Debug, Clone)]
pub struct DrumRow {
    /// Display name (e.g., "Kick", "Snare", "HiHat")
    pub name: String,
    /// Row color for visual identification
    pub color: Color32,
    /// Steps for this row
    pub steps: Vec<DrumStep>,
    /// Whether row is visible
    pub visible: bool,
    /// Whether row is muted
    pub muted: bool,
    /// Whether row is soloed
    pub soloed: bool,
}

impl DrumRow {
    /// Create a new drum row with the given name
    pub fn new(name: impl Into<String>, num_steps: usize) -> Self {
        Self {
            name: name.into(),
            color: Color32::WHITE,
            steps: vec![DrumStep::default(); num_steps],
            visible: true,
            muted: false,
            soloed: false,
        }
    }

    /// Set the row color
    pub fn with_color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }
}

/// Response from the drum sequencer
#[derive(Debug, Clone)]
pub struct DrumSequencerResponse {
    /// Overall UI response
    pub response: Response,
    /// Map of (row_index, step_index) -> true if clicked
    pub step_toggled: HashMap<(usize, usize), bool>,
    /// Current playback step (from current_step parameter)
    pub current_step: Option<usize>,
    /// Whether any step data changed
    pub changed: bool,
}

/// Professional drum sequencer component
///
/// Multi-row step sequencer designed for drum programming in DAW applications.
/// Each row represents a different drum sound with independent patterns.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # use armas::components::audio::{DrumSequencer, DrumRow, DrumStep};
/// # fn example(ui: &mut Ui) {
/// let mut rows = vec![
///     DrumRow::new("Kick", 16),
///     DrumRow::new("Snare", 16),
///     DrumRow::new("HiHat", 16),
/// ];
///
/// let response = DrumSequencer::new(&mut rows)
///     .steps(16)
///     .current_step(Some(2))
///     .show(ui);
/// # }
/// ```
pub struct DrumSequencer<'a> {
    rows: &'a mut Vec<DrumRow>,
    num_steps: usize,
    current_step: Option<usize>,
    step_width: f32,
    step_height: f32,
    row_label_width: f32,
    row_height: f32,
    gap: f32,
    glow_intensity: f32,
    variant: DrumSequencerVariant,
    color_scheme: DrumSequencerColorScheme,
    show_velocity: bool,
    id: Option<egui::Id>,
}

impl<'a> DrumSequencer<'a> {
    /// Create a new drum sequencer
    pub fn new(rows: &'a mut Vec<DrumRow>) -> Self {
        Self {
            rows,
            num_steps: 16,
            current_step: None,
            step_width: 40.0,
            step_height: 32.0,
            row_label_width: 80.0,
            row_height: 48.0,
            gap: 4.0,
            glow_intensity: 0.8,
            variant: DrumSequencerVariant::Filled,
            color_scheme: DrumSequencerColorScheme::Semantic,
            show_velocity: true,
            id: None,
        }
    }

    /// Set unique ID for state persistence across frame recreations
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set number of steps per row
    pub fn steps(mut self, num_steps: usize) -> Self {
        self.num_steps = num_steps.max(1);
        self
    }

    /// Set current playback step (for visual feedback)
    pub fn current_step(mut self, step: Option<usize>) -> Self {
        self.current_step = step;
        self
    }

    /// Set step size (width and height)
    pub fn step_size(mut self, width: f32, height: f32) -> Self {
        self.step_width = width.max(20.0);
        self.step_height = height.max(20.0);
        self
    }

    /// Set row label width
    pub fn row_label_width(mut self, width: f32) -> Self {
        self.row_label_width = width.max(40.0);
        self
    }

    /// Set row height
    pub fn row_height(mut self, height: f32) -> Self {
        self.row_height = height.max(30.0);
        self
    }

    /// Set gap between steps and rows
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    /// Set glow intensity (0.0-1.0)
    pub fn glow_intensity(mut self, intensity: f32) -> Self {
        self.glow_intensity = intensity.clamp(0.0, 1.0);
        self
    }

    /// Set visual variant
    pub fn variant(mut self, variant: DrumSequencerVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set color scheme
    pub fn color_scheme(mut self, scheme: DrumSequencerColorScheme) -> Self {
        self.color_scheme = scheme;
        self
    }

    /// Show velocity as brightness
    pub fn show_velocity(mut self, show: bool) -> Self {
        self.show_velocity = show;
        self
    }

    /// Show the drum sequencer
    pub fn show(self, ui: &mut Ui) -> DrumSequencerResponse {
        let theme = ui.ctx().armas_theme();
        let mut step_toggled: HashMap<(usize, usize), bool> = HashMap::new();
        let mut changed = false;

        // Restore state from memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("drum_sequencer_state");
            if let Some(stored_state) = ui.ctx().data(|d| d.get_temp::<Vec<Vec<DrumStep>>>(state_id)) {
                // Restore stored state to rows
                for (row_idx, stored_steps) in stored_state.iter().enumerate() {
                    if row_idx < self.rows.len() && row_idx < stored_steps.len() {
                        // Copy step data from stored state
                        for (step_idx, stored_step) in stored_steps.iter().enumerate() {
                            if step_idx < self.rows[row_idx].steps.len() {
                                self.rows[row_idx].steps[step_idx] = *stored_step;
                            }
                        }
                    }
                }
            }
        }

        // Cache rendering parameters before borrowing
        let glow_intensity = self.glow_intensity;
        let row_label_width = self.row_label_width;
        let row_height = self.row_height;
        let step_width = self.step_width;
        let gap = self.gap;
        let num_steps = self.num_steps;
        let _current_step = self.current_step;
        let variant = self.variant;
        let _color_scheme = self.color_scheme;
        let show_velocity = self.show_velocity;
        let id = self.id;

        // Ensure all rows have correct number of steps
        for row in self.rows.iter_mut() {
            row.steps.resize(num_steps, DrumStep::default());
        }

        // Calculate total size
        let num_visible_rows = self.rows.iter().filter(|r| r.visible).count();
        let total_width = row_label_width
            + num_steps as f32 * step_width
            + (num_steps - 1) as f32 * gap;
        let total_height =
            num_visible_rows as f32 * row_height + (num_visible_rows - 1) as f32 * gap;

        let desired_size = Vec2::new(total_width, total_height);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        // Track drag state - check if primary button is pressed and we're over the sequencer
        let is_dragging = ui.ctx().input(|i| i.pointer.primary_down()) && response.hovered();
        let mouse_pos = ui.ctx().input(|i| i.pointer.latest_pos());

        if ui.is_rect_visible(rect) {
            let mut row_y = rect.min.y;

            for (row_idx, row) in self.rows.iter_mut().enumerate() {
                if !row.visible {
                    continue;
                }

                // Draw row label/controls
                let label_rect = Rect::from_min_size(
                    Pos2::new(rect.min.x, row_y),
                    Vec2::new(row_label_width, row_height),
                );

                Self::draw_row_label_static(ui, &theme, label_rect, row);

                // Draw steps for this row
                for step_idx in 0..num_steps {
                    let step_x =
                        rect.min.x + row_label_width + step_idx as f32 * (step_width + gap);
                    let step_rect = Rect::from_min_size(
                        Pos2::new(step_x, row_y),
                        Vec2::new(step_width, row_height),
                    );

                    let step_response = ui.allocate_rect(step_rect, Sense::click());

                    // Handle click
                    if step_response.clicked() {
                        row.steps[step_idx].active = !row.steps[step_idx].active;
                        if !row.steps[step_idx].active {
                            row.steps[step_idx].velocity = 1.0;
                        }
                        step_toggled.insert((row_idx, step_idx), row.steps[step_idx].active);
                        changed = true;
                    }

                    // Handle drag - light up steps being dragged over
                    if is_dragging {
                        if let Some(mouse) = mouse_pos {
                            if step_rect.contains(mouse) {
                                // Turn on the step if dragging over it
                                if !row.steps[step_idx].active {
                                    row.steps[step_idx].active = true;
                                    step_toggled.insert((row_idx, step_idx), true);
                                    changed = true;
                                }
                            }
                        }
                    }

                    let is_active = row.steps[step_idx].active;
                    let is_hovered = step_response.hovered();
                    let velocity = row.steps[step_idx].velocity;

                    Self::draw_step_static(
                        ui.painter(),
                        &theme,
                        step_rect,
                        row.color,
                        is_active,
                        is_hovered,
                        velocity,
                        glow_intensity,
                        variant,
                        show_velocity,
                    );
                }

                row_y += row_height + gap;
            }

        }

        if changed {
            ui.ctx().request_repaint();
        }

        // Save state to memory if ID is set
        if let Some(id) = id {
            let state_id = id.with("drum_sequencer_state");
            let state: Vec<Vec<DrumStep>> = self.rows.iter().map(|r| r.steps.clone()).collect();
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id, state);
            });
        }

        DrumSequencerResponse {
            response,
            step_toggled,
            current_step: self.current_step,
            changed,
        }
    }

    fn draw_row_label_static(
        ui: &mut Ui,
        theme: &Theme,
        rect: Rect,
        row: &DrumRow,
    ) {
        let corner_radius = theme.spacing.corner_radius_small as f32;
        let painter = ui.painter();

        // Background - use row color with brightness adjustment
        let bg_color = if row.muted {
            row.color.gamma_multiply(0.4)
        } else if row.soloed {
            row.color.gamma_multiply(0.7)
        } else {
            row.color.gamma_multiply(0.6)
        };

        painter.rect_filled(rect, corner_radius, bg_color);

        // Subtle glow effect around header (2 layers like steps)
        for i in 0..2 {
            let offset = (i + 1) as f32 * 1.5;
            let alpha = ((1.0 - i as f32 / 2.0) * 15.0) as u8;
            let glow_color = Color32::from_rgba_unmultiplied(
                row.color.r(),
                row.color.g(),
                row.color.b(),
                alpha,
            );
            painter.rect_stroke(
                rect.expand(offset),
                corner_radius,
                egui::Stroke::new(1.0, glow_color),
                egui::StrokeKind::Outside,
            );
        }

        // Row name text - white for contrast
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            &row.name,
            egui::FontId::proportional(12.0),
            Color32::WHITE,
        );
    }

    fn draw_step_static(
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        row_color: Color32,
        is_active: bool,
        is_hovered: bool,
        velocity: f32,
        glow_intensity: f32,
        variant: DrumSequencerVariant,
        show_velocity: bool,
    ) {
        let corner_radius = theme.spacing.corner_radius_small as f32;

        // Draw based on variant
        match variant {
            DrumSequencerVariant::Filled => {
                Self::draw_filled_step(
                    painter,
                    theme,
                    rect,
                    row_color,
                    corner_radius,
                    is_active,
                    is_hovered,
                    velocity,
                    show_velocity,
                    glow_intensity,
                );
            }
            DrumSequencerVariant::Outlined => {
                Self::draw_outlined_step(
                    painter,
                    theme,
                    rect,
                    row_color,
                    corner_radius,
                    is_active,
                    is_hovered,
                    velocity,
                    show_velocity,
                    glow_intensity,
                );
            }
            DrumSequencerVariant::Elevated => {
                Self::draw_elevated_step(
                    painter,
                    theme,
                    rect,
                    row_color,
                    corner_radius,
                    is_active,
                    is_hovered,
                    velocity,
                    show_velocity,
                    glow_intensity,
                );
            }
        }
    }

    fn draw_filled_step(
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        row_color: Color32,
        corner_radius: f32,
        is_active: bool,
        is_hovered: bool,
        velocity: f32,
        show_velocity: bool,
        glow_intensity: f32,
    ) {
        let mut fill_color = if is_active {
            row_color
        } else {
            theme.muted()
        };

        if is_active && show_velocity {
            let velocity_factor = 1.0 + (velocity * 0.8);
            fill_color = fill_color.gamma_multiply(velocity_factor);
        } else if is_hovered {
            fill_color = fill_color.gamma_multiply(1.2);
        }

        painter.rect_filled(rect, corner_radius, fill_color);

        let border_color = if is_active {
            theme.primary()
        } else if is_hovered {
            theme.border()
        } else {
            theme.border()
        };

        painter.rect_stroke(
            rect,
            corner_radius,
            egui::Stroke::new(1.0, border_color),
            egui::StrokeKind::Outside,
        );

        if is_active {
            Self::draw_glow_effect(painter, rect, corner_radius, theme.primary(), glow_intensity);
        }
    }

    fn draw_outlined_step(
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        row_color: Color32,
        corner_radius: f32,
        is_active: bool,
        is_hovered: bool,
        velocity: f32,
        show_velocity: bool,
        glow_intensity: f32,
    ) {
        let bg_color = if is_active && show_velocity {
            let alpha = (64.0 + (velocity * 191.0)) as u8;
            Color32::from_rgba_unmultiplied(row_color.r(), row_color.g(), row_color.b(), alpha)
        } else if is_hovered {
            theme.muted()
        } else {
            theme.card()
        };

        painter.rect_filled(rect, corner_radius, bg_color);

        let border_color = if is_active { row_color } else if is_hovered { theme.border() } else { theme.border() };
        let border_width = if is_active { 2.0 } else { 1.5 };

        painter.rect_stroke(
            rect,
            corner_radius,
            egui::Stroke::new(border_width, border_color),
            egui::StrokeKind::Outside,
        );

        if is_active {
            Self::draw_glow_effect(painter, rect, corner_radius, row_color, glow_intensity);
        }
    }

    fn draw_elevated_step(
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        row_color: Color32,
        corner_radius: f32,
        is_active: bool,
        is_hovered: bool,
        velocity: f32,
        show_velocity: bool,
        glow_intensity: f32,
    ) {
        if !is_active {
            for i in 0..3 {
                let offset = (i + 1) as f32 * 0.5;
                let shadow_rect = rect.translate(Vec2::new(0.0, offset));
                let alpha = (20.0 - i as f32 * 5.0) as u8;
                let shadow_color = Color32::from_rgba_unmultiplied(0, 0, 0, alpha);
                painter.rect_filled(shadow_rect, corner_radius, shadow_color);
            }
        }

        let mut fill_color = row_color;
        if is_active && show_velocity {
            let velocity_factor = 1.0 + (velocity * 0.8);
            fill_color = fill_color.gamma_multiply(velocity_factor);
        } else if is_hovered {
            fill_color = fill_color.gamma_multiply(1.15);
        }

        painter.rect_filled(rect, corner_radius, fill_color);

        let border_color = if is_active { theme.primary() } else { theme.border() };
        painter.rect_stroke(
            rect,
            corner_radius,
            egui::Stroke::new(1.0, border_color),
            egui::StrokeKind::Outside,
        );

        if is_active {
            Self::draw_glow_effect(painter, rect, corner_radius, theme.primary(), glow_intensity);
        }
    }

    fn draw_glow_effect(
        painter: &egui::Painter,
        rect: Rect,
        corner_radius: f32,
        glow_color: Color32,
        glow_intensity: f32,
    ) {
        // Subtle glow: only 2 layers with reduced alpha
        for i in 0..2 {
            let offset = (i + 1) as f32 * 1.5;
            let alpha = ((1.0 - i as f32 / 2.0) * 15.0 * glow_intensity) as u8;
            let layer_color = Color32::from_rgba_unmultiplied(glow_color.r(), glow_color.g(), glow_color.b(), alpha);
            painter.rect_stroke(
                rect.expand(offset),
                corner_radius,
                egui::Stroke::new(1.0, layer_color),
                egui::StrokeKind::Outside,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drum_row_creation() {
        let row = DrumRow::new("Kick", 16);
        assert_eq!(row.name, "Kick");
        assert_eq!(row.steps.len(), 16);
        assert!(!row.muted);
        assert!(row.visible);
    }

    #[test]
    fn test_drum_step_default() {
        let step = DrumStep::default();
        assert!(!step.active);
        assert_eq!(step.velocity, 1.0);
    }

    #[test]
    fn test_drum_sequencer_step_resize() {
        let mut rows = vec![DrumRow::new("Kick", 8), DrumRow::new("Snare", 8)];
        let seq = DrumSequencer::new(&mut rows).steps(16);
        assert_eq!(seq.num_steps, 16);
    }
}
