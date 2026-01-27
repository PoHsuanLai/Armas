//! Textarea Component
//!
//! Multi-line text input field styled like shadcn/ui Textarea.
//! Provides a clean, accessible textarea with support for:
//! - Labels and descriptions
//! - Validation states (error, success, warning)
//! - Character count limits
//! - Resizable option

use crate::ext::ArmasContextExt;
use crate::{InputState, InputVariant};
use egui::{Color32, Response, Stroke, TextEdit, Ui};

// shadcn Textarea constants
const CORNER_RADIUS: f32 = 6.0; // rounded-md
const MIN_HEIGHT: f32 = 80.0; // Minimum height
const PADDING: f32 = 12.0; // px-3 py-2
const FONT_SIZE: f32 = 14.0; // text-sm

/// Response from the textarea
#[derive(Debug, Clone)]
pub struct TextareaResponse {
    /// The UI response
    pub response: Response,
    /// Current text value
    pub text: String,
    /// Whether text changed this frame
    pub changed: bool,
}

/// Multi-line text input field styled like shadcn/ui
pub struct Textarea {
    id: Option<egui::Id>,
    variant: InputVariant,
    state: InputState,
    label: Option<String>,
    description: Option<String>,
    placeholder: String,
    width: Option<f32>,
    rows: usize,
    max_chars: Option<usize>,
    resizable: bool,
    disabled: bool,
}

impl Textarea {
    /// Create a new textarea
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            id: None,
            variant: InputVariant::Default,
            state: InputState::Normal,
            label: None,
            description: None,
            placeholder: placeholder.into(),
            width: None,
            rows: 4,
            max_chars: None,
            resizable: true,
            disabled: false,
        }
    }

    /// Set ID for state persistence
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the textarea variant (for backwards compatibility)
    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the validation state
    pub fn state(mut self, state: InputState) -> Self {
        self.state = state;
        self
    }

    /// Set a label above the textarea
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set description/helper text below the textarea
    pub fn description(mut self, text: impl Into<String>) -> Self {
        self.description = Some(text.into());
        self
    }

    /// Alias for description (backwards compatibility)
    pub fn helper_text(mut self, text: impl Into<String>) -> Self {
        self.description = Some(text.into());
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

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Show the textarea
    pub fn show(self, ui: &mut Ui, text: &mut String) -> TextareaResponse {
        let theme = ui.ctx().armas_theme();

        // Load state from memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("textarea_state");
            let stored_text: String = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id).unwrap_or_else(|| text.clone()));
            *text = stored_text;
        }

        let width = self.width.unwrap_or(300.0);

        let response = ui
            .vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 6.0; // gap-1.5

                // Label with optional character count
                if let Some(label) = &self.label {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(label)
                                .size(14.0)
                                .color(if self.disabled {
                                    theme.muted_foreground()
                                } else {
                                    theme.foreground()
                                }),
                        );

                        // Character count on the right
                        if let Some(max) = self.max_chars {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    let count_color = if text.len() > max {
                                        theme.destructive()
                                    } else if text.len() as f32 / max as f32 > 0.9 {
                                        theme.chart_3()
                                    } else {
                                        theme.muted_foreground()
                                    };
                                    ui.label(
                                        egui::RichText::new(format!("{}/{}", text.len(), max))
                                            .size(12.0)
                                            .color(count_color),
                                    );
                                },
                            );
                        }
                    });
                }

                // Calculate height based on rows
                let line_height = ui.text_style_height(&egui::TextStyle::Body);
                let min_height = (line_height * self.rows as f32 + PADDING * 2.0).max(MIN_HEIGHT);

                // Border color based on state
                let border_color = match self.state {
                    InputState::Normal => theme.input(),
                    InputState::Success => theme.chart_2(),
                    InputState::Error => theme.destructive(),
                    InputState::Warning => theme.chart_3(),
                };

                // Background color
                let bg_color = if self.disabled || self.variant == InputVariant::Filled {
                    theme.muted()
                } else {
                    theme.background()
                };

                // Text color
                let text_color = if self.disabled {
                    theme.muted_foreground()
                } else {
                    theme.foreground()
                };

                // Frame for the textarea
                let frame = egui::Frame::NONE
                    .fill(bg_color)
                    .stroke(Stroke::new(1.0, border_color))
                    .corner_radius(CORNER_RADIUS)
                    .inner_margin(PADDING);

                let response = frame.show(ui, |ui| {
                    ui.set_width(width - PADDING * 2.0);
                    ui.set_min_height(min_height - PADDING * 2.0);

                    // Style the text edit
                    ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
                    ui.style_mut().visuals.widgets.hovered.bg_fill = Color32::TRANSPARENT;
                    ui.style_mut().visuals.widgets.active.bg_fill = Color32::TRANSPARENT;
                    ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::NONE;
                    ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
                    ui.style_mut().visuals.widgets.active.bg_stroke = Stroke::NONE;
                    ui.style_mut().visuals.override_text_color = Some(text_color);
                    ui.style_mut()
                        .text_styles
                        .insert(egui::TextStyle::Body, egui::FontId::proportional(FONT_SIZE));

                    let mut text_edit = TextEdit::multiline(text)
                        .hint_text(&self.placeholder)
                        .desired_width(width - PADDING * 4.0)
                        .desired_rows(self.rows)
                        .frame(false)
                        .interactive(!self.disabled);

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

                // Description/helper text
                if let Some(desc) = &self.description {
                    let desc_color = match self.state {
                        InputState::Normal => theme.muted_foreground(),
                        InputState::Success => theme.chart_2(),
                        InputState::Error => theme.destructive(),
                        InputState::Warning => theme.chart_3(),
                    };
                    ui.label(egui::RichText::new(desc).size(12.0).color(desc_color));
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

        let changed = response.changed();
        let text_clone = text.clone();

        TextareaResponse {
            response,
            text: text_clone,
            changed,
        }
    }
}

impl Default for Textarea {
    fn default() -> Self {
        Self::new("")
    }
}
