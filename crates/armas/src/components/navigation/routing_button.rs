//! Routing Button Component
//!
//! Input/Output routing button for audio/MIDI channel selection.
//!
//! A compact button component designed to display routing information
//! like "Input L+R", "Main", or other channel labels. Output buttons
//! are styled slightly lighter than input buttons for visual distinction.

use crate::ext::ArmasContextExt;
use egui;

/// Audio/MIDI routing button component
///
/// Displays routing information with automatic styling for input vs output.
///
/// # Example
///
/// ```rust,no_run
/// use armas::components::RoutingButton;
///
/// fn ui(ui: &mut egui::Ui) {
///     RoutingButton::input("Input L+R").show(ui);
///     RoutingButton::output("Main").show(ui);
///
///     // Custom size
///     RoutingButton::input("Stereo")
///         .size(100.0, 30.0)
///         .show(ui);
/// }
/// ```
pub struct RoutingButton<'a> {
    pub label: &'a str,
    pub width: f32,
    pub height: f32,
    pub is_output: bool,
}

impl<'a> RoutingButton<'a> {
    /// Create an input routing button with default size (100x32)
    pub fn input(label: &'a str) -> Self {
        Self {
            label,
            width: 100.0,
            height: 32.0,
            is_output: false,
        }
    }

    /// Create an output routing button with default size (100x32, styled slightly lighter)
    pub fn output(label: &'a str) -> Self {
        Self {
            label,
            width: 100.0,
            height: 32.0,
            is_output: true,
        }
    }

    /// Set custom width and height
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Show the routing button
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = ui.ctx().armas_theme();
        let font_size = ui.spacing().interact_size.y * 0.4;
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(self.width, self.height), egui::Sense::click());

        // Background color (output slightly lighter)
        let bg_color = if self.is_output {
            theme.surface()
        } else {
            theme.surface().gamma_multiply(0.9)
        };

        ui.painter()
            .rect_filled(rect, theme.spacing.corner_radius as f32 * 0.5, bg_color);

        ui.painter().rect_stroke(
            rect,
            theme.spacing.corner_radius as f32 * 0.5,
            egui::Stroke::new(1.0, theme.outline_variant()),
            egui::StrokeKind::Middle,
        );

        ui.painter().text(
            rect.left_center() + egui::vec2(theme.spacing.sm, 0.0),
            egui::Align2::LEFT_CENTER,
            self.label,
            egui::FontId::proportional(font_size),
            theme.on_surface(),
        );

        response
    }
}
