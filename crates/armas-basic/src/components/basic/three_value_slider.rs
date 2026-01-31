//! Three Value Slider Component
//!
//! Horizontal slider with three thumbs: min bound, current value, and max bound.

use crate::ext::ArmasContextExt;
use egui::{pos2, vec2, Color32, Rect, Response, Sense, Stroke, Ui};

/// Which thumb is being dragged
#[derive(Clone, Copy, Debug, PartialEq, Default)]
enum DragTarget {
    #[default]
    None,
    Min,
    Value,
    Max,
}

/// Persisted drag state
#[derive(Clone, Default)]
struct ThreeValueSliderDragState {
    target: DragTarget,
    drag_start_value: f32,
}

/// Parameters for drawing thumbs
struct ThumbDrawParams<'a> {
    painter: &'a egui::Painter,
    response: &'a Response,
    track_rect: &'a Rect,
    thumb_radius: f32,
    drag_state: &'a ThreeValueSliderDragState,
    hovered_thumb: Option<DragTarget>,
    theme: &'a crate::Theme,
}

/// Style for the center (value) thumb
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ValueThumbStyle {
    /// Same style as min/max thumbs
    Circle,
    /// Diamond shape to differentiate from bounds
    #[default]
    Diamond,
    /// Vertical line indicator
    Line,
}

/// Three value slider with min bound, current value, and max bound
pub struct ThreeValueSlider {
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
    value_thumb_style: ValueThumbStyle,
}

impl ThreeValueSlider {
    /// Create a new three value slider
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
            value_thumb_style: ValueThumbStyle::Diamond,
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

    /// Show or hide the value labels
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Set a label for the slider
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set a suffix for the values
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Set a step value for snapping
    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    /// Set minimum gap between adjacent thumbs
    pub fn min_gap(mut self, gap: f32) -> Self {
        self.min_gap = gap;
        self
    }

    /// Set the style for the center value thumb
    pub fn value_thumb_style(mut self, style: ValueThumbStyle) -> Self {
        self.value_thumb_style = style;
        self
    }

    /// Show the three value slider
    pub fn show(
        self,
        ui: &mut Ui,
        min_bound: &mut f32,
        value: &mut f32,
        max_bound: &mut f32,
    ) -> ThreeValueSliderResponse {
        let theme = ui.ctx().armas_theme();
        let mut changed = false;

        let slider_id = self
            .id
            .unwrap_or_else(|| ui.make_persistent_id("three_value_slider"));
        let drag_state_id = slider_id.with("drag_state");

        // Load state from memory if ID is set (for demos where values reset each frame)
        if let Some(id) = self.id {
            let min_state_id = id.with("min_bound");
            let val_state_id = id.with("value");
            let max_state_id = id.with("max_bound");
            *min_bound = ui
                .ctx()
                .data_mut(|d| d.get_temp(min_state_id).unwrap_or(*min_bound));
            *value = ui
                .ctx()
                .data_mut(|d| d.get_temp(val_state_id).unwrap_or(*value));
            *max_bound = ui
                .ctx()
                .data_mut(|d| d.get_temp(max_state_id).unwrap_or(*max_bound));
        }

        // Clamp and ensure ordering
        self.clamp_values(min_bound, value, max_bound);

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 4.0;

            // Label and values
            self.draw_label(ui, *min_bound, *value, *max_bound);

            // Allocate slider area
            let (rect, response) =
                ui.allocate_exact_size(vec2(self.width, self.height), Sense::click_and_drag());

            // Track and thumb sizes (matching shadcn: h-1.5 track, size-4 thumb)
            let track_height = 6.0;
            let thumb_radius = 8.0;
            let track_rect =
                Rect::from_center_size(rect.center(), vec2(rect.width(), track_height));

            // Calculate thumb positions
            let min_x = self.value_to_x(*min_bound, &track_rect);
            let value_x = self.value_to_x(*value, &track_rect);
            let max_x = self.value_to_x(*max_bound, &track_rect);

            // Handle interactions
            let drag_state = self.handle_interaction(
                ui,
                &response,
                drag_state_id,
                &track_rect,
                min_x,
                value_x,
                max_x,
                min_bound,
                value,
                max_bound,
                &mut changed,
            );

            // Determine which thumb is hovered (for per-thumb hover effect)
            let hovered_thumb = if response.hovered() {
                if let Some(pos) = response.hover_pos() {
                    let dist_to_min = (pos.x - min_x).abs();
                    let dist_to_value = (pos.x - value_x).abs();
                    let dist_to_max = (pos.x - max_x).abs();
                    if dist_to_value <= thumb_radius
                        && dist_to_value <= dist_to_min
                        && dist_to_value <= dist_to_max
                    {
                        Some(DragTarget::Value)
                    } else if dist_to_min <= thumb_radius && dist_to_min <= dist_to_max {
                        Some(DragTarget::Min)
                    } else if dist_to_max <= thumb_radius {
                        Some(DragTarget::Max)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            // Draw the slider
            self.draw(
                ui,
                &response,
                &track_rect,
                track_height,
                thumb_radius,
                min_x,
                value_x,
                max_x,
                &drag_state,
                hovered_thumb,
                &theme,
            );
        });

        // Save state to memory if ID is set
        if let Some(id) = self.id {
            let min_state_id = id.with("min_bound");
            let val_state_id = id.with("value");
            let max_state_id = id.with("max_bound");
            ui.ctx().data_mut(|d| {
                d.insert_temp(min_state_id, *min_bound);
                d.insert_temp(val_state_id, *value);
                d.insert_temp(max_state_id, *max_bound);
            });
        }

        ThreeValueSliderResponse {
            min_bound: *min_bound,
            value: *value,
            max_bound: *max_bound,
            changed,
        }
    }

    fn clamp_values(&self, min_bound: &mut f32, value: &mut f32, max_bound: &mut f32) {
        *min_bound = min_bound.clamp(self.range_min, self.range_max);
        *max_bound = max_bound.clamp(self.range_min, self.range_max);
        if *min_bound > *max_bound {
            std::mem::swap(min_bound, max_bound);
        }
        *value = value.clamp(*min_bound, *max_bound);
    }

    fn draw_label(&self, ui: &mut Ui, min_bound: f32, value: f32, max_bound: f32) {
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
                    "{} | {} | {}",
                    self.format_value(min_bound),
                    self.format_value(value),
                    self.format_value(max_bound)
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

    fn determine_target(&self, pos_x: f32, min_x: f32, value_x: f32, max_x: f32) -> DragTarget {
        let dist_to_min = (pos_x - min_x).abs();
        let dist_to_value = (pos_x - value_x).abs();
        let dist_to_max = (pos_x - max_x).abs();

        if dist_to_value <= dist_to_min && dist_to_value <= dist_to_max {
            DragTarget::Value
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
        min_x: f32,
        value_x: f32,
        max_x: f32,
        min_bound: &mut f32,
        value: &mut f32,
        max_bound: &mut f32,
        changed: &mut bool,
    ) -> ThreeValueSliderDragState {
        let mut drag_state: ThreeValueSliderDragState = ui
            .ctx()
            .data_mut(|d| d.get_temp(drag_state_id).unwrap_or_default());

        // Handle drag start
        if response.drag_started() {
            if let Some(pos) = response.interact_pointer_pos() {
                drag_state.target = self.determine_target(pos.x, min_x, value_x, max_x);
                drag_state.drag_start_value = match drag_state.target {
                    DragTarget::Min => *min_bound,
                    DragTarget::Value => *value,
                    DragTarget::Max => *max_bound,
                    DragTarget::None => *value,
                };
            }
        }

        // Handle dragging
        if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                // Fallback if target wasn't set
                if drag_state.target == DragTarget::None {
                    drag_state.target = self.determine_target(pos.x, min_x, value_x, max_x);
                    drag_state.drag_start_value = match drag_state.target {
                        DragTarget::Min => *min_bound,
                        DragTarget::Value => *value,
                        DragTarget::Max => *max_bound,
                        DragTarget::None => *value,
                    };
                }

                self.update_values_from_drag(
                    pos.x,
                    track_rect,
                    &drag_state,
                    min_bound,
                    value,
                    max_bound,
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

    #[allow(clippy::too_many_arguments)]
    fn update_values_from_drag(
        &self,
        pos_x: f32,
        track_rect: &Rect,
        drag_state: &ThreeValueSliderDragState,
        min_bound: &mut f32,
        value: &mut f32,
        max_bound: &mut f32,
        changed: &mut bool,
    ) {
        let new_value = self.apply_step(self.x_to_value(pos_x, track_rect));

        match drag_state.target {
            DragTarget::Min => {
                let clamped = new_value.clamp(self.range_min, *value - self.min_gap);
                if (clamped - *min_bound).abs() > 0.001 {
                    *min_bound = clamped;
                    *changed = true;
                }
            }
            DragTarget::Value => {
                let clamped = new_value.clamp(*min_bound + self.min_gap, *max_bound - self.min_gap);
                if (clamped - *value).abs() > 0.001 {
                    *value = clamped;
                    *changed = true;
                }
            }
            DragTarget::Max => {
                let clamped = new_value.clamp(*value + self.min_gap, self.range_max);
                if (clamped - *max_bound).abs() > 0.001 {
                    *max_bound = clamped;
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
        track_height: f32,
        thumb_radius: f32,
        min_x: f32,
        value_x: f32,
        max_x: f32,
        drag_state: &ThreeValueSliderDragState,
        hovered_thumb: Option<DragTarget>,
        theme: &crate::Theme,
    ) {
        let painter = ui.painter();

        // Background track
        painter.rect_filled(*track_rect, track_height / 2.0, theme.muted());

        // Filled region (min to max bounds)
        let fill_rect = Rect::from_min_max(
            pos2(min_x, track_rect.top()),
            pos2(max_x, track_rect.bottom()),
        );
        painter.rect_filled(
            fill_rect,
            track_height / 2.0,
            theme.muted().gamma_multiply(1.5),
        );

        // Highlight region from min to current value
        let value_fill_rect = Rect::from_min_max(
            pos2(min_x, track_rect.top()),
            pos2(value_x, track_rect.bottom()),
        );
        painter.rect_filled(value_fill_rect, track_height / 2.0, theme.primary());

        // Draw bound thumbs (min and max)
        self.draw_bound_thumbs(
            painter,
            response,
            track_rect,
            thumb_radius,
            min_x,
            max_x,
            drag_state,
            hovered_thumb,
            theme,
        );

        // Draw value thumb (center)
        let params = ThumbDrawParams {
            painter,
            response,
            track_rect,
            thumb_radius,
            drag_state,
            hovered_thumb,
            theme,
        };
        self.draw_value_thumb(&params, value_x);
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_bound_thumbs(
        &self,
        painter: &egui::Painter,
        response: &Response,
        track_rect: &Rect,
        thumb_radius: f32,
        min_x: f32,
        max_x: f32,
        drag_state: &ThreeValueSliderDragState,
        hovered_thumb: Option<DragTarget>,
        theme: &crate::Theme,
    ) {
        // Bound thumbs are slightly smaller
        let bound_radius = thumb_radius * 0.8;

        for (x, is_min) in [(min_x, true), (max_x, false)] {
            let center = pos2(x, track_rect.center().y);
            let this_target = if is_min {
                DragTarget::Min
            } else {
                DragTarget::Max
            };

            let is_active = response.dragged() && drag_state.target == this_target;
            let is_hovered = hovered_thumb == Some(this_target);

            // Hover ring effect (only for this specific thumb)
            if is_active || is_hovered {
                let ring_color = theme.ring().gamma_multiply(0.5);
                painter.circle_filled(center, bound_radius + 3.0, ring_color);
            }

            // Shadow
            painter.circle_filled(
                center + vec2(0.0, 1.0),
                bound_radius,
                Color32::from_black_alpha(40),
            );

            let handle_color = if is_active {
                theme.muted_foreground()
            } else {
                theme.muted_foreground().gamma_multiply(0.8)
            };

            painter.circle_filled(center, bound_radius, handle_color);
            painter.circle_stroke(center, bound_radius, Stroke::new(1.0, theme.primary()));
        }
    }

    fn draw_value_thumb(&self, params: &ThumbDrawParams, value_x: f32) {
        let value_center = pos2(value_x, params.track_rect.center().y);
        let is_value_active =
            params.response.dragged() && params.drag_state.target == DragTarget::Value;
        let is_value_hovered = params.hovered_thumb == Some(DragTarget::Value);
        let value_color = if is_value_active {
            params.theme.primary()
        } else {
            params.theme.foreground()
        };

        // Hover ring effect for value thumb (only when this thumb is hovered)
        if is_value_active || is_value_hovered {
            let ring_color = params.theme.ring().gamma_multiply(0.5);
            params
                .painter
                .circle_filled(value_center, params.thumb_radius + 4.0, ring_color);
        }

        match self.value_thumb_style {
            ValueThumbStyle::Circle => {
                params.painter.circle_filled(
                    value_center + vec2(0.0, 1.0),
                    params.thumb_radius,
                    Color32::from_black_alpha(40),
                );
                params
                    .painter
                    .circle_filled(value_center, params.thumb_radius, value_color);
                params.painter.circle_stroke(
                    value_center,
                    params.thumb_radius,
                    Stroke::new(1.0, params.theme.primary()),
                );
            }
            ValueThumbStyle::Diamond => {
                let size = params.thumb_radius * 0.9;
                let points = vec![
                    pos2(value_center.x, value_center.y - size),
                    pos2(value_center.x + size, value_center.y),
                    pos2(value_center.x, value_center.y + size),
                    pos2(value_center.x - size, value_center.y),
                ];
                // Shadow
                let shadow_points: Vec<_> = points.iter().map(|p| *p + vec2(0.0, 1.0)).collect();
                params.painter.add(egui::Shape::convex_polygon(
                    shadow_points,
                    Color32::from_black_alpha(40),
                    Stroke::NONE,
                ));
                params.painter.add(egui::Shape::convex_polygon(
                    points,
                    value_color,
                    Stroke::new(1.5, params.theme.card()),
                ));
            }
            ValueThumbStyle::Line => {
                let line_height = self.height * 0.8;
                params.painter.line_segment(
                    [
                        pos2(value_center.x, value_center.y - line_height / 2.0),
                        pos2(value_center.x, value_center.y + line_height / 2.0),
                    ],
                    Stroke::new(3.0, value_color),
                );
            }
        }
    }
}

/// Response from a three value slider
#[derive(Debug, Clone, Copy)]
pub struct ThreeValueSliderResponse {
    /// Current minimum bound
    pub min_bound: f32,
    /// Current value
    pub value: f32,
    /// Current maximum bound
    pub max_bound: f32,
    /// Whether any value changed this frame
    pub changed: bool,
}
