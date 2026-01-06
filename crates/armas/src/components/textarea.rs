//! Textarea Component
//!
//! Multi-line text input field

use crate::layout::{HStack, Spacer, VStack};
use crate::{InputState, InputVariant, Theme};
use egui::{vec2, Color32, Response, Sense, Stroke, StrokeKind, TextEdit, Ui, Vec2};

/// Multi-line text input field
pub struct Textarea {
    variant: InputVariant,
    state: InputState,
    label: Option<String>,
    placeholder: String,
    helper_text: Option<String>,
    width: Option<f32>,
    rows: usize,
    max_chars: Option<usize>,
    resizable: bool,
}

impl Textarea {
    /// Create a new textarea
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            variant: InputVariant::Outlined,
            state: InputState::Normal,
            label: None,
            placeholder: placeholder.into(),
            helper_text: None,
            width: None,
            rows: 4,
            max_chars: None,
            resizable: true,
        }
    }

    /// Set the textarea variant
    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the input state (Normal, Success, Error, Warning)
    pub fn state(mut self, state: InputState) -> Self {
        self.state = state;
        self
    }

    /// Set a label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set helper text
    pub fn helper_text(mut self, text: impl Into<String>) -> Self {
        self.helper_text = Some(text.into());
        self
    }

    /// Set fixed width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set number of visible rows
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows.max(1);
        self
    }

    /// Set maximum character count
    pub fn max_chars(mut self, max: usize) -> Self {
        self.max_chars = Some(max);
        self
    }

    /// Set whether the textarea is resizable
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Show the textarea
    pub fn show(self, ui: &mut Ui, theme: &Theme, text: &mut String) -> Response {
        VStack::new(4.0)
            .show_with_inner(ui, |ui| {
                // Label
                if let Some(label) = &self.label {
                    HStack::new(8.0).show(ui, |ui| {
                        ui.label(label);

                        // Character count
                        if let Some(max) = self.max_chars {
                            Spacer::new().show(ui);

                            let count_color = if text.len() > max {
                                theme.error()
                            } else if text.len() as f32 / max as f32 > 0.9 {
                                theme.warning()
                            } else {
                                theme.on_surface_variant()
                            };

                            ui.colored_label(count_color, format!("{}/{}", text.len(), max));
                        }
                    });
                }

                let width = self.width.unwrap_or_else(|| ui.available_width());

                // Calculate height based on rows
                let line_height = ui.text_style_height(&egui::TextStyle::Body);
                let min_height = line_height * self.rows as f32 + 16.0; // 16.0 for padding

                // Background and border colors based on state
                let (bg_color, border_color) = match self.state {
                    InputState::Normal => (theme.surface(), theme.outline()),
                    InputState::Success => (theme.surface(), theme.success()),
                    InputState::Error => (theme.surface(), theme.error()),
                    InputState::Warning => (theme.surface(), theme.warning()),
                };

                let (bg_color, border_color) = match self.variant {
                    InputVariant::Default => (Color32::TRANSPARENT, Color32::TRANSPARENT),
                    InputVariant::Outlined => (Color32::TRANSPARENT, border_color),
                    InputVariant::Filled => (bg_color, Color32::TRANSPARENT),
                };

                // Frame for the textarea
                let frame = egui::Frame::none()
                    .fill(bg_color)
                    .stroke(Stroke::new(1.0, border_color))
                    .rounding(4.0)
                    .inner_margin(8.0);

                let response = frame.show(ui, |ui| {
                    ui.set_width(width - 16.0);
                    ui.set_min_height(min_height);

                    let mut text_edit = TextEdit::multiline(text)
                        .hint_text(&self.placeholder)
                        .desired_width(width - 32.0)
                        .desired_rows(self.rows);

                    // Only allow resize if enabled
                    if !self.resizable {
                        text_edit = text_edit.desired_rows(self.rows);
                    }

                    let response = ui.add(text_edit);

                    // Enforce max characters
                    if let Some(max) = self.max_chars {
                        if text.len() > max {
                            text.truncate(max);
                        }
                    }

                    response
                });

                // Helper text
                if let Some(helper) = &self.helper_text {
                    ui.add_space(4.0);
                    let color = match self.state {
                        InputState::Normal => theme.on_surface_variant(),
                        InputState::Success => theme.success(),
                        InputState::Error => theme.error(),
                        InputState::Warning => theme.warning(),
                    };
                    ui.colored_label(color, helper);
                }

                response.inner
            })
            .inner
    }
}
