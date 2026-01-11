//! Enhanced Input Components
//!
//! Modern text input fields with icons, validation, and animations

use crate::layout::VStack;
use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{vec2, Color32, CornerRadius, Response, Sense, Stroke, TextEdit, Ui, Vec2};

/// Input field variant
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputVariant {
    /// Default input style
    Default,
    /// Outlined input with border
    Outlined,
    /// Filled background style
    Filled,
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
    variant: InputVariant,
    state: InputState,
    label: Option<String>,
    placeholder: String,
    helper_text: Option<String>,
    left_icon: Option<String>,
    right_icon: Option<String>,
    width: Option<f32>,
    password: bool,
}

impl Input {
    /// Create a new input field
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            variant: InputVariant::Default,
            state: InputState::Normal,
            label: None,
            placeholder: placeholder.into(),
            helper_text: None,
            left_icon: None,
            right_icon: None,
            width: None,
            password: false,
        }
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
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set helper text
    pub fn with_helper_text(mut self, text: impl Into<String>) -> Self {
        self.helper_text = Some(text.into());
        self
    }

    /// Set left icon (emoji or character)
    pub fn with_left_icon(mut self, icon: impl Into<String>) -> Self {
        self.left_icon = Some(icon.into());
        self
    }

    /// Set right icon (emoji or character)
    pub fn with_right_icon(mut self, icon: impl Into<String>) -> Self {
        self.right_icon = Some(icon.into());
        self
    }

    /// Set width
    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set as password field
    pub fn password(mut self, enabled: bool) -> Self {
        self.password = enabled;
        self
    }

    /// Show the input field
    pub fn show(self, ui: &mut Ui, text: &mut String) -> Response {
        let theme = ui.ctx().armas_theme();
        let width = self.width.unwrap_or(200.0);

        VStack::new(4.0)
            .show_with_inner(ui, |ui| {
                // Label
                if let Some(label) = &self.label {
                    ui.label(
                        egui::RichText::new(label)
                            .size(14.0)
                            .color(theme.on_surface()),
                    );
                }

                // Input container
                let desired_size = vec2(width, 40.0);
                let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

                if ui.is_rect_visible(rect) {
                    let visuals = ui.style().interact(&response);

                    // Determine colors based on state and variant
                    let (bg_color, border_color) = match self.state {
                        InputState::Normal => match self.variant {
                            InputVariant::Default => (
                                Color32::from_rgba_unmultiplied(40, 40, 40, 180),
                                theme.primary(),
                            ),
                            InputVariant::Outlined => (
                                Color32::TRANSPARENT,
                                if response.has_focus() {
                                    theme.primary()
                                } else {
                                    Color32::from_gray(100)
                                },
                            ),
                            InputVariant::Filled => {
                                let surface = theme.surface_variant();
                                (surface, theme.primary())
                            }
                        },
                        InputState::Success => {
                            let success = theme.success();
                            (
                                Color32::from_rgba_unmultiplied(
                                    success.r(),
                                    success.g(),
                                    success.b(),
                                    40,
                                ),
                                success,
                            )
                        }
                        InputState::Error => {
                            let error = theme.error();
                            (
                                Color32::from_rgba_unmultiplied(
                                    error.r(),
                                    error.g(),
                                    error.b(),
                                    40,
                                ),
                                error,
                            )
                        }
                        InputState::Warning => {
                            let warning = theme.warning();
                            (
                                Color32::from_rgba_unmultiplied(
                                    warning.r(),
                                    warning.g(),
                                    warning.b(),
                                    40,
                                ),
                                warning,
                            )
                        }
                    };

                    // Background
                    ui.painter().rect_filled(
                        rect,
                        CornerRadius::same(theme.spacing.corner_radius_small as u8),
                        bg_color,
                    );

                    // Border (stronger on focus)
                    let border_width = if response.has_focus() { 2.0 } else { 1.0 };
                    ui.painter().rect_stroke(
                        rect,
                        CornerRadius::same(theme.spacing.corner_radius_small as u8),
                        Stroke::new(border_width, border_color),
                        egui::StrokeKind::Outside,
                    );

                    // Calculate text area considering icons
                    let mut text_rect = rect.shrink2(vec2(12.0, 8.0));

                    // Left icon
                    if let Some(icon) = &self.left_icon {
                        let icon_pos = rect.left_center() + vec2(12.0, 0.0);
                        ui.painter().text(
                            icon_pos,
                            egui::Align2::LEFT_CENTER,
                            icon,
                            egui::FontId::proportional(18.0),
                            theme.on_surface(),
                        );
                        text_rect.min.x += 30.0;
                    }

                    // Right icon
                    if let Some(icon) = &self.right_icon {
                        let icon_pos = rect.right_center() - vec2(12.0, 0.0);
                        ui.painter().text(
                            icon_pos,
                            egui::Align2::RIGHT_CENTER,
                            icon,
                            egui::FontId::proportional(18.0),
                            theme.on_surface(),
                        );
                        text_rect.max.x -= 30.0;
                    }

                    // Text input
                    let mut text_edit = TextEdit::singleline(text)
                        .hint_text(&self.placeholder)
                        .desired_width(text_rect.width())
                        .frame(false);

                    if self.password {
                        text_edit = text_edit.password(true);
                    }

                    let text_response = ui.put(text_rect, text_edit);
                    response = response.union(text_response);
                }

                // Helper text
                if let Some(helper) = &self.helper_text {
                    ui.add_space(4.0);
                    let helper_color = match self.state {
                        InputState::Normal => theme.on_surface_variant(),
                        InputState::Success => theme.success(),
                        InputState::Error => theme.error(),
                        InputState::Warning => theme.warning(),
                    };
                    ui.label(egui::RichText::new(helper).size(12.0).color(helper_color));
                }

                response
            })
            .inner
    }
}

/// Search input with built-in search icon
pub struct SearchInput {
    placeholder: String,
    width: Option<f32>,
}

impl SearchInput {
    /// Create a new search input
    pub fn new() -> Self {
        Self {
            placeholder: "Search...".to_string(),
            width: None,
        }
    }

    /// Set placeholder text
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set width
    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Show the search input
    pub fn show(self, ui: &mut Ui, text: &mut String) -> Response {
        Input::new(&self.placeholder)
            .variant(InputVariant::Filled)
            .with_left_icon("🔍")
            .with_width(self.width.unwrap_or(300.0))
            .show(ui, text)
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
            .with_label("Username")
            .with_helper_text("Required field")
            .variant(InputVariant::Outlined)
            .state(InputState::Error);

        assert_eq!(input.label, Some("Username".to_string()));
        assert_eq!(input.helper_text, Some("Required field".to_string()));
        assert_eq!(input.variant, InputVariant::Outlined);
        assert_eq!(input.state, InputState::Error);
    }

    #[test]
    fn test_search_input() {
        let search = SearchInput::new().with_placeholder("Search files...");
        assert_eq!(search.placeholder, "Search files...");
    }
}
