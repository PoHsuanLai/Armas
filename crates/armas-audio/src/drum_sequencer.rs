//! Drum Sequencer Component
//!
//! Professional multi-row drum sequencer for DAW-style pattern programming.
//! Each row represents a drum sound with independent step patterns and velocity control.
//!
//! Features:
//! - Optional viewport scrolling with momentum physics
//! - Smooth inertia-based scrolling that continues after mouse release

use armas::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};
use std::collections::HashMap;

/// State for momentum scrolling (stored in egui temp data)
#[derive(Clone, Default)]
struct DrumSequencerScrollState {
    /// Current scroll offset
    offset: Vec2,
    /// Current velocity for momentum scrolling
    velocity: Vec2,
    /// Last frame time for delta calculation
    last_frame_time: f64,
    /// Whether momentum animation is active
    is_animating: bool,
}

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
    /// Display name (e.g., "Kick", "Snare", "`HiHat`")
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
    #[must_use]
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
    /// Map of (`row_index`, `step_index`) -> true if clicked
    pub step_toggled: HashMap<(usize, usize), bool>,
    /// Current playback step (from `current_step` parameter)
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
/// # use armas::Theme;
/// # use armas_audio::{DrumSequencer, DrumRow, DrumStep};
/// # fn example(ui: &mut Ui, theme: &Theme) {
/// let mut rows = vec![
///     DrumRow::new("Kick", 16),
///     DrumRow::new("Snare", 16),
///     DrumRow::new("HiHat", 16),
/// ];
///
/// let response = DrumSequencer::new(&mut rows)
///     .steps(16)
///     .current_step(Some(2))
///     .show(ui, theme);
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
    /// Enable scrollable viewport
    scrollable: bool,
    /// Viewport width (content will scroll if larger)
    viewport_width: Option<f32>,
    /// Viewport height (content will scroll if larger)
    viewport_height: Option<f32>,
    /// Enable momentum scrolling
    momentum_scrolling: bool,
    /// Damping factor for momentum (higher = faster decay). Default: 5.0
    momentum_damping: f64,
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
            scrollable: false,
            viewport_width: None,
            viewport_height: None,
            momentum_scrolling: true,
            momentum_damping: 5.0,
        }
    }

    /// Set unique ID for state persistence across frame recreations
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set number of steps per row
    #[must_use]
    pub fn steps(mut self, num_steps: usize) -> Self {
        self.num_steps = num_steps.max(1);
        self
    }

    /// Set current playback step (for visual feedback)
    #[must_use]
    pub fn current_step(mut self, step: Option<usize>) -> Self {
        self.current_step = step;
        self
    }

    /// Set step size (width and height)
    #[must_use]
    pub fn step_size(mut self, width: f32, height: f32) -> Self {
        self.step_width = width.max(20.0);
        self.step_height = height.max(20.0);
        self
    }

    /// Set row label width
    #[must_use]
    pub fn row_label_width(mut self, width: f32) -> Self {
        self.row_label_width = width.max(40.0);
        self
    }

    /// Set row height
    #[must_use]
    pub fn row_height(mut self, height: f32) -> Self {
        self.row_height = height.max(30.0);
        self
    }

    /// Set gap between steps and rows
    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    /// Set glow intensity (0.0-1.0)
    #[must_use]
    pub fn glow_intensity(mut self, intensity: f32) -> Self {
        self.glow_intensity = intensity.clamp(0.0, 1.0);
        self
    }

    /// Set visual variant
    #[must_use]
    pub fn variant(mut self, variant: DrumSequencerVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set color scheme
    #[must_use]
    pub fn color_scheme(mut self, scheme: DrumSequencerColorScheme) -> Self {
        self.color_scheme = scheme;
        self
    }

    /// Show velocity as brightness
    #[must_use]
    pub fn show_velocity(mut self, show: bool) -> Self {
        self.show_velocity = show;
        self
    }

    /// Enable scrollable viewport with specified dimensions
    ///
    /// When content exceeds viewport size, scrolling is enabled.
    #[must_use]
    pub fn scrollable(mut self, width: f32, height: f32) -> Self {
        self.scrollable = true;
        self.viewport_width = Some(width);
        self.viewport_height = Some(height);
        self
    }

    /// Enable or disable momentum scrolling (default: true)
    ///
    /// When enabled, scrolling continues with inertia after mouse release.
    #[must_use]
    pub fn momentum_scrolling(mut self, enabled: bool) -> Self {
        self.momentum_scrolling = enabled;
        self
    }

    /// Set momentum damping factor (default: 5.0)
    ///
    /// Higher values = faster velocity decay. Range: 1.0 to 20.0
    #[must_use]
    pub fn momentum_damping(mut self, damping: f64) -> Self {
        self.momentum_damping = damping.clamp(1.0, 20.0);
        self
    }

    /// Calculate viewport and content dimensions
    fn calculate_dimensions(
        rows: &[DrumRow],
        row_label_width: f32,
        row_height: f32,
        step_width: f32,
        gap: f32,
        num_steps: usize,
        viewport_width: Option<f32>,
        viewport_height: Option<f32>,
    ) -> (f32, f32, f32, f32) {
        let num_visible_rows = rows.iter().filter(|r| r.visible).count();
        let content_width =
            row_label_width + num_steps as f32 * step_width + (num_steps - 1) as f32 * gap;
        let content_height =
            num_visible_rows as f32 * row_height + (num_visible_rows - 1) as f32 * gap;

        let actual_width = viewport_width.unwrap_or(content_width).min(content_width);
        let actual_height = viewport_height
            .unwrap_or(content_height)
            .min(content_height);

        (content_width, content_height, actual_width, actual_height)
    }

    /// Restore state from persistent storage
    fn restore_state(ui: &mut Ui, id: egui::Id, rows: &mut Vec<DrumRow>) {
        let state_id = id.with("drum_sequencer_state");
        if let Some(stored_state) = ui
            .ctx()
            .data(|d| d.get_temp::<Vec<Vec<DrumStep>>>(state_id))
        {
            // Restore stored state to rows
            for (row_idx, stored_steps) in stored_state.iter().enumerate() {
                if row_idx < rows.len() && row_idx < stored_steps.len() {
                    // Copy step data from stored state
                    for (step_idx, stored_step) in stored_steps.iter().enumerate() {
                        if step_idx < rows[row_idx].steps.len() {
                            rows[row_idx].steps[step_idx] = *stored_step;
                        }
                    }
                }
            }
        }
    }

    /// Save state to persistent storage
    fn save_state(ui: &mut Ui, id: egui::Id, rows: &[DrumRow]) {
        let state_id = id.with("drum_sequencer_state");
        let state: Vec<Vec<DrumStep>> = rows.iter().map(|r| r.steps.clone()).collect();
        ui.ctx().data_mut(|d| {
            d.insert_temp(state_id, state);
        });
    }

    /// Render grid of step buttons for a single row
    #[allow(clippy::too_many_arguments)]
    fn render_grid(
        ui: &mut Ui,
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        row: &mut DrumRow,
        row_y: f32,
        scroll_offset: Vec2,
        row_label_width: f32,
        step_width: f32,
        row_height: f32,
        gap: f32,
        num_steps: usize,
        is_dragging: bool,
        mouse_pos: Option<Pos2>,
        glow_intensity: f32,
        variant: DrumSequencerVariant,
        show_velocity: bool,
        scrollable: bool,
        step_toggled: &mut HashMap<(usize, usize), bool>,
        row_idx: usize,
    ) -> bool {
        let mut changed = false;

        for step_idx in 0..num_steps {
            let step_x = rect.min.x
                + scroll_offset.x
                + row_label_width
                + step_idx as f32 * (step_width + gap);

            // Skip steps outside viewport
            if scrollable && (step_x + step_width < rect.min.x || step_x > rect.max.x) {
                continue;
            }

            let step_rect =
                Rect::from_min_size(Pos2::new(step_x, row_y), Vec2::new(step_width, row_height));

            let (step_response, step_changed) = Self::handle_step_interaction(
                ui,
                step_rect,
                rect,
                row,
                step_idx,
                is_dragging,
                mouse_pos,
            );

            if step_changed {
                step_toggled.insert((row_idx, step_idx), row.steps[step_idx].active);
                changed = true;
            }

            let is_active = row.steps[step_idx].active;
            let is_hovered = step_response.hovered();
            let velocity = row.steps[step_idx].velocity;

            Self::draw_step_static(
                painter,
                theme,
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

        changed
    }

    /// Render row header with instrument name
    fn render_header(
        painter: &egui::Painter,
        theme: &Theme,
        rect_min_x: f32,
        row_y: f32,
        scroll_offset: Vec2,
        row_label_width: f32,
        row_height: f32,
        row: &DrumRow,
    ) {
        let label_rect = Rect::from_min_size(
            Pos2::new(rect_min_x + scroll_offset.x, row_y),
            Vec2::new(row_label_width, row_height),
        );

        Self::draw_row_label_static(painter, theme, label_rect, row);
    }

    /// Handle step interaction (clicks and drags)
    fn handle_step_interaction(
        ui: &mut Ui,
        step_rect: Rect,
        viewport_rect: Rect,
        row: &mut DrumRow,
        step_idx: usize,
        is_dragging: bool,
        mouse_pos: Option<Pos2>,
    ) -> (Response, bool) {
        // Only allocate for interaction if step is visible
        let step_response = if step_rect.intersects(viewport_rect) {
            ui.allocate_rect(step_rect.intersect(viewport_rect), Sense::click())
        } else {
            return (ui.allocate_rect(Rect::NOTHING, Sense::hover()), false);
        };

        // Handle click
        let mut changed = if step_response.clicked() {
            row.steps[step_idx].active = !row.steps[step_idx].active;
            if !row.steps[step_idx].active {
                row.steps[step_idx].velocity = 1.0;
            }
            true
        } else {
            false
        };

        // Handle drag - light up steps being dragged over
        if is_dragging {
            if let Some(mouse) = mouse_pos {
                if step_rect.contains(mouse) {
                    // Turn on the step if dragging over it
                    if !row.steps[step_idx].active {
                        row.steps[step_idx].active = true;
                        changed = true;
                    }
                }
            }
        }

        (step_response, changed)
    }

    /// Handle momentum-based scrolling physics
    fn handle_momentum_scrolling(
        ui: &mut Ui,
        response: &Response,
        id: Option<egui::Id>,
        scrollable: bool,
        momentum_scrolling: bool,
        momentum_damping: f64,
        content_width: f32,
        content_height: f32,
        actual_width: f32,
        actual_height: f32,
    ) -> Vec2 {
        if !scrollable {
            return Vec2::ZERO;
        }

        let scroll_state_id = id.unwrap_or(response.id).with("drum_seq_scroll");
        let mut scroll_state: DrumSequencerScrollState = ui
            .ctx()
            .data(|d| d.get_temp(scroll_state_id).unwrap_or_default());

        let current_time = ui.ctx().input(|i| i.time);
        let dt = if scroll_state.last_frame_time > 0.0 {
            (current_time - scroll_state.last_frame_time) as f32
        } else {
            0.016 // ~60fps default
        };
        scroll_state.last_frame_time = current_time;

        // Handle scroll wheel input
        if response.hovered() {
            let scroll_delta = ui.ctx().input(|i| i.raw_scroll_delta);
            if scroll_delta.length() > 0.0 {
                if momentum_scrolling {
                    // Add to velocity for momentum
                    scroll_state.velocity += scroll_delta * 3.0;
                    scroll_state.is_animating = true;
                } else {
                    // Direct scroll without momentum
                    scroll_state.offset += scroll_delta;
                }
            }
        }

        // Handle middle-mouse drag for panning
        let is_panning = ui.ctx().input(|i| i.pointer.middle_down()) && response.hovered();
        if is_panning {
            let drag_delta = response.drag_delta();
            if momentum_scrolling {
                scroll_state.velocity =
                    Vec2::new(drag_delta.x / dt.max(0.001), drag_delta.y / dt.max(0.001)) * 0.3;
                scroll_state.is_animating = true;
            }
            scroll_state.offset += drag_delta;
        }

        // Apply momentum physics
        if momentum_scrolling && scroll_state.is_animating {
            // Apply velocity to offset
            scroll_state.offset += scroll_state.velocity * dt;

            // Apply damping (exponential decay)
            let damping_factor = (-momentum_damping * dt as f64).exp() as f32;
            scroll_state.velocity *= damping_factor;

            // Stop animation when velocity is negligible
            if scroll_state.velocity.length() < 1.0 {
                scroll_state.velocity = Vec2::ZERO;
                scroll_state.is_animating = false;
            } else {
                ui.ctx().request_repaint();
            }
        }

        // Clamp scroll offset to valid range
        let max_scroll_x = (content_width - actual_width).max(0.0);
        let max_scroll_y = (content_height - actual_height).max(0.0);
        scroll_state.offset.x = scroll_state.offset.x.clamp(-max_scroll_x, 0.0);
        scroll_state.offset.y = scroll_state.offset.y.clamp(-max_scroll_y, 0.0);

        // Save scroll state
        ui.ctx()
            .data_mut(|d| d.insert_temp(scroll_state_id, scroll_state.clone()));

        scroll_state.offset
    }

    /// Show the drum sequencer
    pub fn show(self, ui: &mut Ui, theme: &Theme) -> DrumSequencerResponse {
        let mut step_toggled: HashMap<(usize, usize), bool> = HashMap::new();
        let mut changed = false;

        // Restore state from memory if ID is set
        if let Some(id) = self.id {
            Self::restore_state(ui, id, self.rows);
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
        let scrollable = self.scrollable;
        let viewport_width = self.viewport_width;
        let viewport_height = self.viewport_height;
        let momentum_scrolling = self.momentum_scrolling;
        let momentum_damping = self.momentum_damping;

        // Ensure all rows have correct number of steps
        for row in self.rows.iter_mut() {
            row.steps.resize(num_steps, DrumStep::default());
        }

        // Calculate dimensions
        let (content_width, content_height, actual_width, actual_height) =
            Self::calculate_dimensions(
                self.rows,
                row_label_width,
                row_height,
                step_width,
                gap,
                num_steps,
                viewport_width,
                viewport_height,
            );
        let desired_size = Vec2::new(actual_width, actual_height);

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        // Handle scrolling with momentum
        let scroll_offset = Self::handle_momentum_scrolling(
            ui,
            &response,
            id,
            scrollable,
            momentum_scrolling,
            momentum_damping,
            content_width,
            content_height,
            actual_width,
            actual_height,
        );

        // Track drag state - check if primary button is pressed and we're over the sequencer
        let is_dragging = ui.ctx().input(|i| i.pointer.primary_down()) && response.hovered();
        let mouse_pos = ui.ctx().input(|i| i.pointer.latest_pos());

        if ui.is_rect_visible(rect) {
            // Use clipped painter for scrolling
            let painter = if scrollable {
                ui.painter().with_clip_rect(rect)
            } else {
                ui.painter().clone()
            };

            let mut row_y = rect.min.y + scroll_offset.y;

            for (row_idx, row) in self.rows.iter_mut().enumerate() {
                if !row.visible {
                    continue;
                }

                // Skip rows outside viewport
                if scrollable && (row_y + row_height < rect.min.y || row_y > rect.max.y) {
                    row_y += row_height + gap;
                    continue;
                }

                // Draw row header
                Self::render_header(
                    &painter,
                    theme,
                    rect.min.x,
                    row_y,
                    scroll_offset,
                    row_label_width,
                    row_height,
                    row,
                );

                // Draw step grid for this row
                let row_changed = Self::render_grid(
                    ui,
                    &painter,
                    theme,
                    rect,
                    row,
                    row_y,
                    scroll_offset,
                    row_label_width,
                    step_width,
                    row_height,
                    gap,
                    num_steps,
                    is_dragging,
                    mouse_pos,
                    glow_intensity,
                    variant,
                    show_velocity,
                    scrollable,
                    &mut step_toggled,
                    row_idx,
                );

                if row_changed {
                    changed = true;
                }

                row_y += row_height + gap;
            }
        }

        if changed {
            ui.ctx().request_repaint();
        }

        // Save state to memory if ID is set
        if let Some(id) = id {
            Self::save_state(ui, id, self.rows);
        }

        DrumSequencerResponse {
            response,
            step_toggled,
            current_step: self.current_step,
            changed,
        }
    }

    fn draw_row_label_static(painter: &egui::Painter, theme: &Theme, rect: Rect, row: &DrumRow) {
        let corner_radius = theme.spacing.corner_radius_small as f32;

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
            let glow_color =
                Color32::from_rgba_unmultiplied(row.color.r(), row.color.g(), row.color.b(), alpha);
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
        let mut fill_color = if is_active { row_color } else { theme.muted() };

        if is_active && show_velocity {
            let velocity_factor = 1.0 + (velocity * 0.8);
            fill_color = fill_color.gamma_multiply(velocity_factor);
        } else if is_hovered {
            fill_color = fill_color.gamma_multiply(1.2);
        }

        painter.rect_filled(rect, corner_radius, fill_color);

        let border_color = if is_active {
            theme.primary()
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
            Self::draw_glow_effect(
                painter,
                rect,
                corner_radius,
                theme.primary(),
                glow_intensity,
            );
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

        let border_color = if is_active { row_color } else { theme.border() };
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

        let border_color = if is_active {
            theme.primary()
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
            Self::draw_glow_effect(
                painter,
                rect,
                corner_radius,
                theme.primary(),
                glow_intensity,
            );
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
            let layer_color = Color32::from_rgba_unmultiplied(
                glow_color.r(),
                glow_color.g(),
                glow_color.b(),
                alpha,
            );
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
