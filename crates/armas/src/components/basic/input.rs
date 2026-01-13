//! Enhanced Input Components
//!
//! Modern text input fields with icons, validation, and animations

use crate::components::cards::{Card, CardVariant};
use crate::ext::ArmasContextExt;
use egui::{Response, TextEdit, Ui};

/// Input field variant
///
/// Material Design 3 specifies two primary text field variants:
/// - `Filled`: Higher visual emphasis with filled background
/// - `Outlined`: Lower visual emphasis, better for forms
///
/// The `Default` variant is kept for backwards compatibility but deprecated in MD3.
/// Use `Filled` or `Outlined` for new implementations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputVariant {
    /// Default input style
    ///
    /// **Note**: This variant is deprecated in Material Design 3.
    /// Use `Filled` or `Outlined` instead for MD3 compliance.
    Default,
    /// Outlined input with border (MD3 recommended for forms)
    Outlined,
    /// Filled background style (MD3 recommended for higher emphasis)
    Filled,
    /// Inline edit style - looks like a button when not focused, editable when clicked
    Inline,
}

/// Input validation state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputState {
    /// Normal state
    Normal,
    /// Success/valid state
    Success,
    /// Error/invalid state
    Error,
    /// Warning state
    Warning,
}

/// Enhanced text input field
pub struct Input {
    id: Option<egui::Id>,
    variant: InputVariant,
    state: InputState,
    label: Option<String>,
    placeholder: String,
    helper_text: Option<String>,
    left_icon: Option<String>,
    right_icon: Option<String>,
    width: Option<f32>,
    password: bool,
    font_size: f32,
    text_color: Option<egui::Color32>,
}

impl Input {
    /// Create a new input field
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            id: None,
            variant: InputVariant::Default,
            state: InputState::Normal,
            label: None,
            placeholder: placeholder.into(),
            helper_text: None,
            left_icon: None,
            right_icon: None,
            width: None,
            password: false,
            font_size: 14.0,
            text_color: None,
        }
    }

    /// Set ID for state persistence (useful for demos where input is recreated each frame)
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the variant
    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the validation state
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

    /// Set left icon (emoji or character)
    pub fn left_icon(mut self, icon: impl Into<String>) -> Self {
        self.left_icon = Some(icon.into());
        self
    }

    /// Set right icon (emoji or character)
    pub fn right_icon(mut self, icon: impl Into<String>) -> Self {
        self.right_icon = Some(icon.into());
        self
    }

    /// Set width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set as password field
    pub fn password(mut self, enabled: bool) -> Self {
        self.password = enabled;
        self
    }

    /// Set font size
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set text color
    pub fn text_color(mut self, color: egui::Color32) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Show the input field
    pub fn show(self, ui: &mut Ui, text: &mut String) -> Response {
        let theme = ui.ctx().armas_theme();

        // Load state from memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("input_state");
            let stored_text: String = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id).unwrap_or_else(|| text.clone()));
            *text = stored_text;
        }

        // For Inline variant, calculate width based on text content
        let width = if self.variant == InputVariant::Inline && self.width.is_none() {
            // Estimate width: ~8px per character for proportional font at size 14
            let char_count = text.len().max(8); // Minimum 8 chars
            let estimated_width = (char_count as f32 * 8.0) + 24.0; // 24px padding
            estimated_width.min(300.0).max(80.0) // Min 80, max 300
        } else {
            self.width.unwrap_or(200.0)
        };

        // For Inline variant, use button height
        let height = if self.variant == InputVariant::Inline {
            28.0
        } else {
            40.0
        };

        // Only use vertical layout if we have label or helper text
        let has_extras = self.label.is_some() || self.helper_text.is_some();

        let mut render_input = |ui: &mut Ui| {
            if has_extras {
                ui.spacing_mut().item_spacing.y = theme.spacing.xs;
            }

            // Label
            if let Some(label) = &self.label {
                ui.label(
                    egui::RichText::new(label)
                        .size(14.0)
                        .color(theme.on_surface()),
                );
            }

            // For Inline variant, render without card wrapper for minimal visual chrome
            let response = if self.variant == InputVariant::Inline {
                // Left icon
                if let Some(icon) = &self.left_icon {
                    ui.label(
                        egui::RichText::new(icon)
                            .size(16.0)
                            .color(theme.on_surface_variant()),
                    );
                }

                // Apply font size and text color before creating TextEdit
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Body,
                    egui::FontId::proportional(self.font_size),
                );
                if let Some(color) = self.text_color {
                    ui.style_mut().visuals.override_text_color = Some(color);
                }

                // Text input
                let mut text_edit = TextEdit::singleline(text)
                    .hint_text(&self.placeholder)
                    .desired_width(width - if self.left_icon.is_some() { 24.0 } else { 0.0 } - if self.right_icon.is_some() { 24.0 } else { 0.0 })
                    .frame(false)
                    .font(egui::TextStyle::Body)
                    .vertical_align(egui::Align::Center);

                if self.password {
                    text_edit = text_edit.password(true);
                }

                let text_response = ui.add(text_edit);

                // Right icon
                if let Some(icon) = &self.right_icon {
                    ui.label(
                        egui::RichText::new(icon)
                            .size(16.0)
                            .color(theme.on_surface_variant()),
                    );
                }

                text_response
            } else {
                // For Filled and Outlined variants, use Card
                let (card_variant, border_color) = match self.state {
                    InputState::Normal => match self.variant {
                        InputVariant::Default | InputVariant::Filled => (CardVariant::Filled, None),
                        InputVariant::Outlined => (CardVariant::Outlined, None),
                        InputVariant::Inline => unreachable!(), // Already handled above
                    },
                    InputState::Success => (CardVariant::Outlined, Some(theme.success())),
                    InputState::Error => (CardVariant::Outlined, Some(theme.error())),
                    InputState::Warning => (CardVariant::Outlined, Some(theme.warning())),
                };

                // Create card for input with minimal padding
                let mut card = Card::new().variant(card_variant).inner_margin(4.0);

                if let Some(border) = border_color {
                    card = card.stroke(border);
                }

                let card_response = card.show(ui, &theme, |ui| {
                    ui.set_width(width - 8.0); // Account for card padding (4px * 2)
                    ui.set_height(height - 8.0); // Account for card padding (4px * 2)

                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;

                        // Left icon
                        if let Some(icon) = &self.left_icon {
                            ui.label(
                                egui::RichText::new(icon)
                                    .size(16.0)
                                    .color(theme.on_surface_variant()),
                            );
                        }

                        // Text input
                        let mut text_edit = TextEdit::singleline(text)
                            .hint_text(&self.placeholder)
                            .desired_width(
                                ui.available_width()
                                    - if self.right_icon.is_some() { 24.0 } else { 0.0 },
                            )
                            .frame(false)
                            .font(egui::TextStyle::Body)
                            .vertical_align(egui::Align::Center);

                        if self.password {
                            text_edit = text_edit.password(true);
                        }

                        // Apply font size and text color
                        ui.style_mut().text_styles.insert(
                            egui::TextStyle::Body,
                            egui::FontId::proportional(self.font_size),
                        );
                        if let Some(color) = self.text_color {
                            ui.style_mut().visuals.override_text_color = Some(color);
                        }

                        ui.add(text_edit);

                        // Right icon
                        if let Some(icon) = &self.right_icon {
                            ui.label(
                                egui::RichText::new(icon)
                                    .size(16.0)
                                    .color(theme.on_surface_variant()),
                            );
                        }
                    });
                });

                card_response.response
            };

            // Helper text
            if let Some(helper) = &self.helper_text {
                ui.add_space(theme.spacing.xs);
                let helper_color = match self.state {
                    InputState::Normal => theme.on_surface_variant(),
                    InputState::Success => theme.success(),
                    InputState::Error => theme.error(),
                    InputState::Warning => theme.warning(),
                };
                ui.label(egui::RichText::new(helper).size(12.0).color(helper_color));
            }

            // Save state to memory if ID is set
            if let Some(id) = self.id {
                let state_id = id.with("input_state");
                ui.ctx().data_mut(|d| {
                    d.insert_temp(state_id, text.clone());
                });
            }

            response
        };

        if has_extras {
            ui.vertical(render_input).inner
        } else {
            render_input(ui)
        }
    }
}

/// Search input with built-in search icon
pub struct SearchInput {
    id: Option<egui::Id>,
    placeholder: String,
    width: Option<f32>,
}

impl SearchInput {
    /// Create a new search input
    pub fn new() -> Self {
        Self {
            id: None,
            placeholder: "Search...".to_string(),
            width: None,
        }
    }

    /// Set ID for state persistence
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Show the search input
    pub fn show(self, ui: &mut Ui, text: &mut String) -> Response {
        let mut input = Input::new(&self.placeholder)
            .variant(InputVariant::Filled)
            .left_icon("🔍")
            .width(self.width.unwrap_or(300.0));

        if let Some(id) = self.id {
            input = input.id(id);
        }

        input.show(ui, text)
    }
}

impl Default for SearchInput {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_creation() {
        let input = Input::new("Enter text");
        assert_eq!(input.placeholder, "Enter text");
        assert_eq!(input.variant, InputVariant::Default);
        assert_eq!(input.state, InputState::Normal);
    }

    #[test]
    fn test_input_builder() {
        let input = Input::new("Test")
            .label("Username")
            .helper_text("Required field")
            .variant(InputVariant::Outlined)
            .state(InputState::Error);

        assert_eq!(input.label, Some("Username".to_string()));
        assert_eq!(input.helper_text, Some("Required field".to_string()));
        assert_eq!(input.variant, InputVariant::Outlined);
        assert_eq!(input.state, InputState::Error);
    }

    #[test]
    fn test_search_input() {
        let search = SearchInput::new().placeholder("Search files...");
        assert_eq!(search.placeholder, "Search files...");
    }
}
