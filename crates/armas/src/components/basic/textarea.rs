//! Textarea Component
//!
//! Multi-line text input field

use crate::ext::ArmasContextExt;
use crate::{InputState, InputVariant};
use egui::{Color32, Response, TextEdit, Ui};

/// Multi-line text input field
///
/// Uses `InputVariant` for styling. See `InputVariant` documentation for MD3 compliance notes.
/// Prefer `Outlined` or `Filled` variants for new implementations.
pub struct Textarea {
    id: Option<egui::Id>,
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
            id: None,
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

    /// Set ID for state persistence (useful for demos where textarea is recreated each frame)
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
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
    pub fn show(self, ui: &mut Ui, text: &mut String) -> Response {
        let theme = ui.ctx().armas_theme();

        // Load state from memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("textarea_state");
            let stored_text: String = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id).unwrap_or_else(|| text.clone()));
            *text = stored_text;
        }

        let response = ui
            .vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 4.0;
                // Label
                if let Some(label) = &self.label {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        ui.label(label);

                        // Character count
                        if let Some(max) = self.max_chars {
                            ui.allocate_space(ui.available_size());

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
                let frame = egui::Frame::NONE
                    .fill(bg_color)
                    .stroke(egui::Stroke::new(1.0, border_color))
                    .corner_radius(4.0)
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
            .inner;

        // Save state to memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("textarea_state");
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id, text.clone());
            });
        }

        response
    }
}
