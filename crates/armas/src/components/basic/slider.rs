//! Slider Component
//!
//! Horizontal slider for value selection with optional velocity-based dragging.

use crate::animation::{DragMode, VelocityDrag, VelocityDragConfig};
use crate::ext::ArmasContextExt;
use egui::{pos2, vec2, Color32, Rect, Sense, Stroke, Ui};

/// Persisted drag state for slider
#[derive(Clone)]
struct SliderDragState {
    drag: VelocityDrag,
    drag_start_value: f32,
}

impl Default for SliderDragState {
    fn default() -> Self {
        Self {
            drag: VelocityDrag::new(VelocityDragConfig::new().sensitivity(1.0)),
            drag_start_value: 0.0,
        }
    }
}

/// Slider component
pub struct Slider {
    id: Option<egui::Id>,
    min: f32,
    max: f32,
    width: f32,
    height: f32,
    show_value: bool,
    label: Option<String>,
    suffix: Option<String>,
    step: Option<f32>,
    default_value: Option<f32>,
    velocity_mode: bool,
    sensitivity: f64,
}

impl Slider {
    /// Create a new slider
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            id: None,
            min,
            max,
            width: 200.0,
            height: 20.0,
            show_value: true,
            label: None,
            suffix: None,
            step: None,
            default_value: None,
            velocity_mode: false,
            sensitivity: 1.0,
        }
    }

    /// Set ID for state persistence (useful for demos where slider is recreated each frame)
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

    /// Set a suffix for the value (e.g., "%", "ms", "dB")
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Set a step value for snapping
    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    /// Set a default value for double-click reset
    pub fn default_value(mut self, value: f32) -> Self {
        self.default_value = Some(value);
        self
    }

    /// Enable velocity-based dragging mode
    ///
    /// When enabled, holding Ctrl/Cmd while dragging uses velocity mode
    /// where faster mouse movement = larger value changes.
    /// This allows for fine-grained control.
    pub fn velocity_mode(mut self, enabled: bool) -> Self {
        self.velocity_mode = enabled;
        self
    }

    /// Set the sensitivity for velocity mode (default: 1.0)
    ///
    /// Higher values = more responsive to mouse speed
    pub fn sensitivity(mut self, sensitivity: f64) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    /// Show the slider
    pub fn show(self, ui: &mut Ui, value: &mut f32) -> SliderResponse {
        let theme = ui.ctx().armas_theme();
        let mut changed = false;

        // Generate a stable ID for drag state
        let slider_id = self
            .id
            .unwrap_or_else(|| ui.make_persistent_id("slider"));
        let drag_state_id = slider_id.with("drag_state");

        // Load state from memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("slider_state");
            let stored_value: f32 = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id).unwrap_or(*value));
            *value = stored_value;
        }

        // Clamp value to range
        *value = value.clamp(self.min, self.max);

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 4.0;
            // Label
            if let Some(label) = &self.label {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;
                    ui.label(label);

                    if self.show_value {
                        ui.allocate_space(ui.available_size());

                        let value_text = if let Some(suffix) = &self.suffix {
                            format!("{:.1}{}", value, suffix)
                        } else {
                            format!("{:.1}", value)
                        };
                        ui.label(value_text);
                    }
                });
            }

            // Slider track and handle
            let slider_width = self.width;
            let (rect, response) =
                ui.allocate_exact_size(vec2(slider_width, self.height), Sense::click_and_drag());

            // Handle double-click to reset
            if response.double_clicked() {
                if let Some(default) = self.default_value {
                    if (*value - default).abs() > 0.001 {
                        *value = default;
                        changed = true;
                    }
                }
            }
            // Handle drag interaction
            else if response.drag_started() {
                let mut drag_state = SliderDragState {
                    drag: VelocityDrag::new(
                        VelocityDragConfig::new().sensitivity(self.sensitivity),
                    ),
                    drag_start_value: *value,
                };

                if let Some(pos) = response.interact_pointer_pos() {
                    let use_velocity = self.velocity_mode
                        && ui.input(|i| {
                            i.modifiers.command || i.modifiers.ctrl
                        });
                    drag_state.drag.begin(*value as f64, pos.x as f64, use_velocity);
                }

                ui.ctx()
                    .data_mut(|d| d.insert_temp(drag_state_id, drag_state));
            } else if response.dragged() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let mut drag_state: SliderDragState = ui
                        .ctx()
                        .data_mut(|d| d.get_temp(drag_state_id).unwrap_or_default());

                    let range = (self.max - self.min) as f64;

                    if drag_state.drag.mode() == DragMode::Velocity {
                        // Velocity mode: use drag helper
                        let delta =
                            drag_state
                                .drag
                                .update_tracked(pos.x as f64, range, rect.width() as f64);
                        let mut new_value = drag_state.drag_start_value + delta as f32;

                        // Apply step if specified
                        if let Some(step) = self.step {
                            new_value = (new_value / step).round() * step;
                        }

                        if (new_value - *value).abs() > 0.001 {
                            *value = new_value.clamp(self.min, self.max);
                            changed = true;
                        }
                    } else {
                        // Absolute mode: position maps directly to value
                        let t = ((pos.x - rect.left()) / rect.width()).clamp(0.0, 1.0);
                        let mut new_value = self.min + t * (self.max - self.min);

                        // Apply step if specified
                        if let Some(step) = self.step {
                            new_value = (new_value / step).round() * step;
                        }

                        if (new_value - *value).abs() > 0.001 {
                            *value = new_value.clamp(self.min, self.max);
                            changed = true;
                        }
                    }

                    ui.ctx()
                        .data_mut(|d| d.insert_temp(drag_state_id, drag_state));
                }
            } else if response.drag_stopped() {
                ui.ctx().data_mut(|d| {
                    let mut drag_state: SliderDragState =
                        d.get_temp(drag_state_id).unwrap_or_default();
                    drag_state.drag.end();
                    d.insert_temp(drag_state_id, drag_state);
                });
            }
            // Handle click (not drag)
            else if response.clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let t = ((pos.x - rect.left()) / rect.width()).clamp(0.0, 1.0);
                    let mut new_value = self.min + t * (self.max - self.min);

                    // Apply step if specified
                    if let Some(step) = self.step {
                        new_value = (new_value / step).round() * step;
                    }

                    if (new_value - *value).abs() > 0.001 {
                        *value = new_value.clamp(self.min, self.max);
                        changed = true;
                    }
                }
            }

            if ui.is_rect_visible(rect) {
                let painter = ui.painter();

                // Track and thumb sizes (matching shadcn: h-1.5 track, size-4 thumb)
                let track_height = 6.0;
                let thumb_radius = 8.0;

                // Background track
                let track_rect = Rect::from_center_size(rect.center(), vec2(rect.width(), track_height));

                painter.rect_filled(track_rect, track_height / 2.0, theme.muted());

                // Filled track (progress)
                let t = (*value - self.min) / (self.max - self.min);
                let fill_width = track_rect.width() * t;
                let fill_rect = Rect::from_min_size(track_rect.min, vec2(fill_width, track_height));

                painter.rect_filled(fill_rect, track_height / 2.0, theme.primary());

                // Handle (thumb)
                let handle_x = track_rect.left() + fill_width;
                let handle_center = pos2(handle_x, track_rect.center().y);

                // Hover ring effect (like shadcn ring-4)
                if response.hovered() || response.dragged() {
                    let ring_color = theme.ring().gamma_multiply(0.5);
                    painter.circle_filled(handle_center, thumb_radius + 4.0, ring_color);
                }

                // Handle shadow
                painter.circle_filled(
                    handle_center + vec2(0.0, 1.0),
                    thumb_radius,
                    Color32::from_black_alpha(40),
                );

                // Handle
                let handle_color = if response.dragged() {
                    theme.primary()
                } else {
                    theme.foreground()
                };

                painter.circle_filled(handle_center, thumb_radius, handle_color);

                // Handle border
                painter.circle_stroke(
                    handle_center,
                    thumb_radius,
                    Stroke::new(1.0, theme.primary()),
                );
            }
        });

        // Save state to memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("slider_state");
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id, *value);
            });
        }

        SliderResponse {
            value: *value,
            changed,
        }
    }
}

/// Response from a slider
#[derive(Debug, Clone, Copy)]
pub struct SliderResponse {
    /// Current value
    pub value: f32,
    /// Whether the value changed this frame
    pub changed: bool,
}
