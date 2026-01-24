//! Zoom Control Component
//!
//! Interactive zoom control for timeline scaling.
//! Provides buttons and/or slider for adjusting timeline zoom level.

use armas::components::basic::Slider;
use armas::components::button::{Button, ButtonVariant};
use armas::ext::ArmasContextExt;
use egui::{Response, Ui};

/// Zoom control component
///
/// Displays zoom controls for timeline scaling.
/// Provides buttons (+/-) and optional slider for precise control.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::components::audio::ZoomControl;
///
/// let mut zoom_level = 1.0;
///
/// let response = ZoomControl::new(&mut zoom_level)
///     .min_zoom(0.1)
///     .max_zoom(10.0)
///     .show(ui);
///
/// if response.changed {
///     println!("New zoom: {}", zoom_level);
/// }
/// # }
/// ```
pub struct ZoomControl<'a> {
    zoom_level: &'a mut f32,
    min_zoom: f32,
    max_zoom: f32,
    show_slider: bool,
    show_buttons: bool,
    show_label: bool,
    button_step: f32,
    slider_width: f32,
    id: Option<egui::Id>,
}

/// Response from zoom control interaction
#[derive(Debug, Clone)]
pub struct ZoomControlResponse {
    /// The egui response
    pub response: Response,
    /// Zoom level was changed
    pub changed: bool,
    /// Zoom in button was clicked
    pub zoomed_in: bool,
    /// Zoom out button was clicked
    pub zoomed_out: bool,
    /// Reset button was clicked (if shown)
    pub reset: bool,
}

impl<'a> ZoomControl<'a> {
    /// Create a new zoom control
    pub fn new(zoom_level: &'a mut f32) -> Self {
        Self {
            zoom_level,
            min_zoom: 0.1,
            max_zoom: 10.0,
            show_slider: true,
            show_buttons: true,
            show_label: true,
            button_step: 0.2,
            slider_width: 150.0,
            id: None,
        }
    }

    /// Set ID for state persistence (useful when zoom control is recreated each frame)
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set minimum zoom level
    pub fn min_zoom(mut self, min: f32) -> Self {
        self.min_zoom = min.max(0.01);
        self
    }

    /// Set maximum zoom level
    pub fn max_zoom(mut self, max: f32) -> Self {
        self.max_zoom = max;
        self
    }

    /// Show or hide the slider
    pub fn show_slider(mut self, show: bool) -> Self {
        self.show_slider = show;
        self
    }

    /// Show or hide +/- buttons
    pub fn show_buttons(mut self, show: bool) -> Self {
        self.show_buttons = show;
        self
    }

    /// Show or hide zoom level label
    pub fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    /// Set zoom step for buttons
    pub fn button_step(mut self, step: f32) -> Self {
        self.button_step = step.max(0.01);
        self
    }

    /// Set slider width in pixels
    pub fn slider_width(mut self, width: f32) -> Self {
        self.slider_width = width.max(50.0);
        self
    }

    /// Show the zoom control
    pub fn show(self, ui: &mut Ui) -> ZoomControlResponse {
        let theme = ui.ctx().armas_theme();

        // Load state from memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("zoom_state");
            let stored_value: f32 = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id).unwrap_or(*self.zoom_level));
            *self.zoom_level = stored_value;
        }

        let old_zoom = *self.zoom_level;
        let mut changed = false;
        let mut zoomed_in = false;
        let mut zoomed_out = false;
        let mut reset = false;

        let response = ui
            .horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = theme.spacing.sm;

                // Vertically center all items
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    // Label
                    if self.show_label {
                        ui.label("Zoom:");
                    }

                    // Zoom out button
                    if self.show_buttons {
                        if Button::new("−")
                            .variant(ButtonVariant::FilledTonal)
                            .min_size(egui::vec2(28.0, 28.0))
                            .show(ui)
                            .clicked()
                        {
                            *self.zoom_level =
                                (*self.zoom_level - self.button_step).max(self.min_zoom);
                            zoomed_out = true;
                            changed = true;
                        }
                    }

                    // Slider
                    if self.show_slider {
                        // Note: Armas Slider doesn't support logarithmic yet, but we can still use it
                        let slider_response = Slider::new(self.min_zoom, self.max_zoom)
                            .width(self.slider_width)
                            .show_value(false)
                            .show(ui, self.zoom_level);

                        if slider_response.changed {
                            changed = true;
                        }
                    }

                    // Zoom in button
                    if self.show_buttons {
                        if Button::new("+")
                            .variant(ButtonVariant::FilledTonal)
                            .min_size(egui::vec2(28.0, 28.0))
                            .show(ui)
                            .clicked()
                        {
                            *self.zoom_level =
                                (*self.zoom_level + self.button_step).min(self.max_zoom);
                            zoomed_in = true;
                            changed = true;
                        }
                    }

                    // Zoom level display
                    if self.show_label {
                        ui.label(format!("{:.1}x", self.zoom_level));
                    }

                    // Reset button (1:1)
                    if Button::new("1:1")
                        .variant(ButtonVariant::Text)
                        .min_size(egui::vec2(36.0, 28.0))
                        .show(ui)
                        .clicked()
                    {
                        *self.zoom_level = 1.0;
                        reset = true;
                        changed = true;
                    }

                    // Fit button (could zoom to fit content)
                    if Button::new("Fit")
                        .variant(ButtonVariant::Text)
                        .min_size(egui::vec2(36.0, 28.0))
                        .show(ui)
                        .clicked()
                    {
                        *self.zoom_level = 1.0; // Placeholder - would calculate based on content
                        reset = true;
                        changed = true;
                    }
                });
            })
            .response;

        // Ensure zoom is within bounds
        *self.zoom_level = self.zoom_level.clamp(self.min_zoom, self.max_zoom);

        // Check if zoom actually changed
        changed = changed || (old_zoom != *self.zoom_level);

        // Save state to memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("zoom_state");
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id, *self.zoom_level);
            });
        }

        ZoomControlResponse {
            response,
            changed,
            zoomed_in,
            zoomed_out,
            reset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zoom_control_creation() {
        let mut zoom = 1.0;
        let _control = ZoomControl::new(&mut zoom);
        assert_eq!(zoom, 1.0);
    }

    #[test]
    fn test_zoom_control_builder() {
        let mut zoom = 1.0;
        let control = ZoomControl::new(&mut zoom)
            .min_zoom(0.5)
            .max_zoom(5.0)
            .show_slider(false)
            .button_step(0.5);

        assert_eq!(control.min_zoom, 0.5);
        assert_eq!(control.max_zoom, 5.0);
        assert!(!control.show_slider);
        assert_eq!(control.button_step, 0.5);
    }

    #[test]
    fn test_zoom_bounds() {
        let mut zoom = 1.0;
        let control = ZoomControl::new(&mut zoom).min_zoom(0.5).max_zoom(2.0);

        // Simulate setting out of bounds
        let mut zoom: f32 = 10.0;
        let clamped = zoom.clamp(control.min_zoom, control.max_zoom);
        assert_eq!(clamped, 2.0);

        zoom = 0.1;
        let clamped = zoom.clamp(control.min_zoom, control.max_zoom);
        assert_eq!(clamped, 0.5);
    }
}
