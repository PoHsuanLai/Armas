//! Range Slider Component
//!
//! Horizontal slider with two thumbs for selecting a range (min/max).

use crate::ext::ArmasContextExt;
use egui::{pos2, vec2, Color32, Rect, Response, Sense, Stroke, Ui};

/// Which thumb is being dragged
#[derive(Clone, Copy, Debug, PartialEq, Default)]
enum DragTarget {
    #[default]
    None,
    Min,
    Max,
    Both,
}

/// Persisted drag state for range slider
#[derive(Clone, Default)]
struct RangeSliderDragState {
    target: DragTarget,
    drag_start_min: f32,
    drag_start_max: f32,
    drag_start_x: f32,
}

/// Range slider with two thumbs for min/max selection
pub struct RangeSlider {
    id: Option<egui::Id>,
    range_min: f32,
    range_max: f32,
    width: f32,
    height: f32,
    show_value: bool,
    label: Option<String>,
    suffix: Option<String>,
    step: Option<f32>,
    min_gap: f32,
    allow_range_drag: bool,
}

impl RangeSlider {
    /// Create a new range slider
    pub fn new(range_min: f32, range_max: f32) -> Self {
        Self {
            id: None,
            range_min,
            range_max,
            width: 200.0,
            height: 20.0,
            show_value: true,
            label: None,
            suffix: None,
            step: None,
            min_gap: 0.0,
            allow_range_drag: true,
        }
    }

    /// Set ID for state persistence
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the slider width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the slider height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Show or hide the value label
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Set a label for the slider
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set a suffix for the values (e.g., "%", "ms", "Hz")
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Set a step value for snapping
    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    /// Set minimum gap between min and max thumbs
    pub fn min_gap(mut self, gap: f32) -> Self {
        self.min_gap = gap;
        self
    }

    /// Allow dragging the filled region to move both thumbs together
    pub fn allow_range_drag(mut self, allow: bool) -> Self {
        self.allow_range_drag = allow;
        self
    }

    /// Show the range slider
    pub fn show(
        self,
        ui: &mut Ui,
        min_value: &mut f32,
        max_value: &mut f32,
    ) -> RangeSliderResponse {
        let theme = ui.ctx().armas_theme();
        let mut changed = false;

        let slider_id = self
            .id
            .unwrap_or_else(|| ui.make_persistent_id("range_slider"));
        let drag_state_id = slider_id.with("drag_state");

        // Clamp and ensure min <= max
        self.clamp_values(min_value, max_value);

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 4.0;

            // Label and values
            self.draw_label(ui, *min_value, *max_value);

            // Allocate slider area
            let (rect, response) =
                ui.allocate_exact_size(vec2(self.width, self.height), Sense::click_and_drag());

            let track_rect = Rect::from_center_size(rect.center(), vec2(rect.width(), 4.0));
            let handle_radius = self.height / 2.0;

            // Calculate thumb positions
            let min_x = self.value_to_x(*min_value, &track_rect);
            let max_x = self.value_to_x(*max_value, &track_rect);

            // Handle interactions
            let drag_state = self.handle_interaction(
                ui,
                &response,
                drag_state_id,
                &track_rect,
                handle_radius,
                min_x,
                max_x,
                min_value,
                max_value,
                &mut changed,
            );

            // Draw the slider
            self.draw(
                ui,
                &response,
                &track_rect,
                handle_radius,
                min_x,
                max_x,
                &drag_state,
                &theme,
            );
        });

        RangeSliderResponse {
            min_value: *min_value,
            max_value: *max_value,
            changed,
        }
    }

    fn clamp_values(&self, min_value: &mut f32, max_value: &mut f32) {
        *min_value = min_value.clamp(self.range_min, self.range_max);
        *max_value = max_value.clamp(self.range_min, self.range_max);
        if *min_value > *max_value {
            std::mem::swap(min_value, max_value);
        }
    }

    fn draw_label(&self, ui: &mut Ui, min_value: f32, max_value: f32) {
        if self.label.is_none() && !self.show_value {
            return;
        }

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;

            if let Some(label) = &self.label {
                ui.label(label);
            }

            if self.show_value {
                ui.allocate_space(ui.available_size());
                ui.label(format!(
                    "{} - {}",
                    self.format_value(min_value),
                    self.format_value(max_value)
                ));
            }
        });
    }

    fn format_value(&self, value: f32) -> String {
        if let Some(suffix) = &self.suffix {
            format!("{:.1}{}", value, suffix)
        } else {
            format!("{:.1}", value)
        }
    }

    fn apply_step(&self, value: f32) -> f32 {
        if let Some(step) = self.step {
            (value / step).round() * step
        } else {
            value
        }
    }

    fn value_to_x(&self, value: f32, track_rect: &Rect) -> f32 {
        let t = (value - self.range_min) / (self.range_max - self.range_min);
        track_rect.left() + t * track_rect.width()
    }

    fn x_to_value(&self, x: f32, track_rect: &Rect) -> f32 {
        let t = ((x - track_rect.left()) / track_rect.width()).clamp(0.0, 1.0);
        self.range_min + t * (self.range_max - self.range_min)
    }

    fn determine_target(
        &self,
        pos_x: f32,
        min_x: f32,
        max_x: f32,
        handle_radius: f32,
    ) -> DragTarget {
        let dist_to_min = (pos_x - min_x).abs();
        let dist_to_max = (pos_x - max_x).abs();
        let in_range = pos_x > min_x + handle_radius && pos_x < max_x - handle_radius;

        if self.allow_range_drag
            && in_range
            && dist_to_min > handle_radius
            && dist_to_max > handle_radius
        {
            DragTarget::Both
        } else if dist_to_min <= dist_to_max {
            DragTarget::Min
        } else {
            DragTarget::Max
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn handle_interaction(
        &self,
        ui: &mut Ui,
        response: &Response,
        drag_state_id: egui::Id,
        track_rect: &Rect,
        handle_radius: f32,
        min_x: f32,
        max_x: f32,
        min_value: &mut f32,
        max_value: &mut f32,
        changed: &mut bool,
    ) -> RangeSliderDragState {
        let mut drag_state: RangeSliderDragState = ui
            .ctx()
            .data_mut(|d| d.get_temp(drag_state_id).unwrap_or_default());

        // Handle drag start
        if response.drag_started() {
            if let Some(pos) = response.interact_pointer_pos() {
                drag_state.target = self.determine_target(pos.x, min_x, max_x, handle_radius);
                drag_state.drag_start_min = *min_value;
                drag_state.drag_start_max = *max_value;
                drag_state.drag_start_x = pos.x;
            }
        }

        // Handle dragging
        if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                // Fallback if target wasn't set
                if drag_state.target == DragTarget::None {
                    drag_state.target = self.determine_target(pos.x, min_x, max_x, handle_radius);
                    drag_state.drag_start_min = *min_value;
                    drag_state.drag_start_max = *max_value;
                    drag_state.drag_start_x = pos.x;
                }

                self.update_values_from_drag(
                    pos.x,
                    track_rect,
                    &drag_state,
                    min_value,
                    max_value,
                    changed,
                );
            }
        }

        // Handle drag end
        if response.drag_stopped() {
            drag_state.target = DragTarget::None;
        }

        // Save state
        ui.ctx()
            .data_mut(|d| d.insert_temp(drag_state_id, drag_state.clone()));

        drag_state
    }

    fn update_values_from_drag(
        &self,
        pos_x: f32,
        track_rect: &Rect,
        drag_state: &RangeSliderDragState,
        min_value: &mut f32,
        max_value: &mut f32,
        changed: &mut bool,
    ) {
        match drag_state.target {
            DragTarget::Min => {
                let new_value = self
                    .apply_step(self.x_to_value(pos_x, track_rect))
                    .clamp(self.range_min, *max_value - self.min_gap);

                if (new_value - *min_value).abs() > 0.001 {
                    *min_value = new_value;
                    *changed = true;
                }
            }
            DragTarget::Max => {
                let new_value = self
                    .apply_step(self.x_to_value(pos_x, track_rect))
                    .clamp(*min_value + self.min_gap, self.range_max);

                if (new_value - *max_value).abs() > 0.001 {
                    *max_value = new_value;
                    *changed = true;
                }
            }
            DragTarget::Both => {
                let delta_x = pos_x - drag_state.drag_start_x;
                let delta_value =
                    delta_x / track_rect.width() * (self.range_max - self.range_min);

                let range_size = drag_state.drag_start_max - drag_state.drag_start_min;
                let mut new_min = drag_state.drag_start_min + delta_value;
                let mut new_max = drag_state.drag_start_max + delta_value;

                // Clamp to bounds while preserving range size
                if new_min < self.range_min {
                    new_min = self.range_min;
                    new_max = self.range_min + range_size;
                }
                if new_max > self.range_max {
                    new_max = self.range_max;
                    new_min = self.range_max - range_size;
                }

                new_min = self.apply_step(new_min);
                new_max = self.apply_step(new_max);

                if (new_min - *min_value).abs() > 0.001 || (new_max - *max_value).abs() > 0.001 {
                    *min_value = new_min;
                    *max_value = new_max;
                    *changed = true;
                }
            }
            DragTarget::None => {}
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn draw(
        &self,
        ui: &Ui,
        response: &Response,
        track_rect: &Rect,
        handle_radius: f32,
        min_x: f32,
        max_x: f32,
        drag_state: &RangeSliderDragState,
        theme: &crate::Theme,
    ) {
        let painter = ui.painter();

        // Background track
        painter.rect_filled(*track_rect, 2.0, theme.muted());

        // Filled region between thumbs
        let fill_rect = Rect::from_min_max(
            pos2(min_x, track_rect.top()),
            pos2(max_x, track_rect.bottom()),
        );
        painter.rect_filled(fill_rect, 2.0, theme.primary());

        // Draw thumbs
        for (x, is_min) in [(min_x, true), (max_x, false)] {
            let center = pos2(x, track_rect.center().y);

            // Shadow
            painter.circle_filled(
                center + vec2(0.0, 1.0),
                handle_radius,
                Color32::from_black_alpha(40),
            );

            // Determine if this thumb is active
            let is_active = response.dragged()
                && ((is_min && drag_state.target == DragTarget::Min)
                    || (!is_min && drag_state.target == DragTarget::Max)
                    || drag_state.target == DragTarget::Both);

            let handle_color = if is_active || response.hovered() {
                theme.primary()
            } else {
                theme.foreground()
            };

            painter.circle_filled(center, handle_radius, handle_color);
            painter.circle_stroke(center, handle_radius, Stroke::new(2.0, theme.card()));
        }
    }
}

/// Response from a range slider
#[derive(Debug, Clone, Copy)]
pub struct RangeSliderResponse {
    /// Current minimum value
    pub min_value: f32,
    /// Current maximum value
    pub max_value: f32,
    /// Whether either value changed this frame
    pub changed: bool,
}
