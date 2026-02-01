//! Input Component
//!
//! Text input field styled like shadcn/ui Input.
//! Provides a clean, accessible text input with support for:
//! - Labels and descriptions
//! - Validation states (error, success, warning)
//! - Icons (left and right)
//! - Password masking

use egui::{Color32, Response, Sense, Stroke, TextEdit, Ui, Vec2};

// shadcn Input constants
const CORNER_RADIUS: f32 = 6.0; // rounded-md
const HEIGHT: f32 = 36.0; // h-9
const PADDING_X: f32 = 12.0; // px-3
const PADDING_Y: f32 = 8.0; // py-2
const FONT_SIZE: f32 = 14.0; // text-sm

/// Response from the input field
#[derive(Debug, Clone)]
pub struct InputResponse {
    /// The UI response
    pub response: Response,
    /// Current text value
    pub text: String,
    /// Whether text changed this frame
    pub changed: bool,
}

/// Input validation state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputState {
    /// Normal state
    #[default]
    Normal,
    /// Success/valid state
    Success,
    /// Error/invalid state
    Error,
    /// Warning state
    Warning,
}

/// Input field variant (for backwards compatibility)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputVariant {
    /// Default input style (standard shadcn input)
    #[default]
    Default,
    /// Outlined input (same as default in shadcn)
    Outlined,
    /// Filled background style
    Filled,
    /// Inline edit style - minimal chrome
    Inline,
}

/// Text input field styled like shadcn/ui
pub struct Input {
    id: Option<egui::Id>,
    variant: InputVariant,
    state: InputState,
    label: Option<String>,
    description: Option<String>,
    placeholder: String,
    left_icon: Option<String>,
    right_icon: Option<String>,
    width: Option<f32>,
    custom_height: Option<f32>,
    password: bool,
    disabled: bool,
}

impl Input {
    /// Create a new input field
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            id: None,
            variant: InputVariant::Default,
            state: InputState::Normal,
            label: None,
            description: None,
            placeholder: placeholder.into(),
            left_icon: None,
            right_icon: None,
            width: None,
            custom_height: None,
            password: false,
            disabled: false,
        }
    }

    /// Set ID for state persistence
    #[must_use]
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the variant (for backwards compatibility)
    #[must_use]
    pub const fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the validation state
    #[must_use]
    pub const fn state(mut self, state: InputState) -> Self {
        self.state = state;
        self
    }

    /// Set a label above the input
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set description/helper text below the input
    #[must_use]
    pub fn description(mut self, text: impl Into<String>) -> Self {
        self.description = Some(text.into());
        self
    }

    /// Alias for description (backwards compatibility)
    #[must_use]
    pub fn helper_text(mut self, text: impl Into<String>) -> Self {
        self.description = Some(text.into());
        self
    }

    /// Set left icon (emoji or character)
    #[must_use]
    pub fn left_icon(mut self, icon: impl Into<String>) -> Self {
        self.left_icon = Some(icon.into());
        self
    }

    /// Set right icon (emoji or character)
    #[must_use]
    pub fn right_icon(mut self, icon: impl Into<String>) -> Self {
        self.right_icon = Some(icon.into());
        self
    }

    /// Set width
    #[must_use]
    pub const fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set explicit height (overrides variant-based height)
    #[must_use]
    pub const fn height(mut self, height: f32) -> Self {
        self.custom_height = Some(height);
        self
    }

    /// Set as password field
    #[must_use]
    pub const fn password(mut self, enabled: bool) -> Self {
        self.password = enabled;
        self
    }

    /// Set disabled state
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Backwards compatibility aliases
    #[must_use]
    pub const fn font_size(self, _size: f32) -> Self {
        // Ignored - use consistent sizing
        self
    }

    /// Set text color (currently ignored - uses theme colors)
    #[must_use]
    pub const fn text_color(self, _color: Color32) -> Self {
        // Ignored - use theme colors
        self
    }

    /// Show the input field
    pub fn show(self, ui: &mut Ui, text: &mut String, theme: &crate::Theme) -> InputResponse {
        // Load state from memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("input_state");
            let stored_text: String = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id).unwrap_or_else(|| text.clone()));
            *text = stored_text;
        }

        let width = self.width.unwrap_or(200.0);

        let response = ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 6.0; // gap-1.5

            // Label
            if let Some(label) = &self.label {
                ui.label(
                    egui::RichText::new(label)
                        .size(14.0)
                        .color(if self.disabled {
                            theme.muted_foreground()
                        } else {
                            theme.foreground()
                        }),
                );
            }

            // Input field
            let input_response = self.render_input(ui, text, width, theme);

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

            input_response
        });

        // Save state to memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("input_state");
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id, text.clone());
            });
        }

        let inner_response = response.inner;
        let changed = inner_response.changed();
        let text_clone = text.clone();

        InputResponse {
            response: inner_response,
            text: text_clone,
            changed,
        }
    }

    fn render_input(
        &self,
        ui: &mut Ui,
        text: &mut String,
        width: f32,
        theme: &crate::Theme,
    ) -> Response {
        let height = self
            .custom_height
            .unwrap_or(if self.variant == InputVariant::Inline {
                28.0
            } else {
                HEIGHT
            });

        // Calculate border color based on state
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

        let placeholder_color = theme.muted_foreground();

        // Allocate space for the input
        let desired_size = Vec2::new(width, height);
        let (rect, response) = ui.allocate_exact_size(
            desired_size,
            if self.disabled {
                Sense::hover()
            } else {
                Sense::click_and_drag()
            },
        );

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Draw background
            painter.rect_filled(rect, CORNER_RADIUS, bg_color);

            // Draw border
            let stroke_width = if response.has_focus() { 2.0 } else { 1.0 };
            let stroke_color = if response.has_focus() {
                theme.ring()
            } else {
                border_color
            };
            painter.rect_stroke(
                rect,
                CORNER_RADIUS,
                Stroke::new(stroke_width, stroke_color),
                egui::StrokeKind::Inside,
            );

            // Scale font and padding to fit custom height
            let font_size = if height < FONT_SIZE + PADDING_Y * 2.0 {
                (height * 0.6).max(8.0)
            } else {
                FONT_SIZE
            };
            let padding_y = ((height - font_size) / 2.0).max(0.0);
            let content_rect = rect.shrink2(Vec2::new(PADDING_X, padding_y));

            // Layout: [left_icon] [text_input] [right_icon]
            let mut x_offset = content_rect.left();

            // Left icon
            if let Some(icon) = &self.left_icon {
                let icon_galley = painter.layout_no_wrap(
                    icon.clone(),
                    egui::FontId::proportional(16.0),
                    placeholder_color,
                );
                let icon_pos = egui::pos2(
                    x_offset,
                    content_rect.center().y - icon_galley.size().y / 2.0,
                );
                painter.galley(icon_pos, icon_galley, placeholder_color);
                x_offset += 24.0; // icon width + spacing
            }

            // Right icon offset calculation
            let right_icon_width = if self.right_icon.is_some() { 24.0 } else { 0.0 };

            // Text input area
            let text_rect = egui::Rect::from_min_max(
                egui::pos2(x_offset, content_rect.top()),
                egui::pos2(
                    content_rect.right() - right_icon_width,
                    content_rect.bottom(),
                ),
            );

            // Right icon
            if let Some(icon) = &self.right_icon {
                let icon_galley = painter.layout_no_wrap(
                    icon.clone(),
                    egui::FontId::proportional(16.0),
                    placeholder_color,
                );
                let icon_x = content_rect.right() - icon_galley.size().x;
                let icon_pos =
                    egui::pos2(icon_x, content_rect.center().y - icon_galley.size().y / 2.0);
                painter.galley(icon_pos, icon_galley, placeholder_color);
            }

            // Render text edit in the allocated space
            let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(text_rect));

            // Style the text edit
            child_ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
            child_ui.style_mut().visuals.widgets.hovered.bg_fill = Color32::TRANSPARENT;
            child_ui.style_mut().visuals.widgets.active.bg_fill = Color32::TRANSPARENT;
            child_ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::NONE;
            child_ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
            child_ui.style_mut().visuals.widgets.active.bg_stroke = Stroke::NONE;
            child_ui.style_mut().visuals.override_text_color = Some(text_color);
            child_ui
                .style_mut()
                .text_styles
                .insert(egui::TextStyle::Body, egui::FontId::proportional(font_size));

            let mut text_edit = TextEdit::singleline(text)
                .hint_text(&self.placeholder)
                .desired_width(text_rect.width())
                .frame(false)
                .font(egui::TextStyle::Body)
                .vertical_align(egui::Align::Center)
                .interactive(!self.disabled);

            if self.password {
                text_edit = text_edit.password(true);
            }

            // Apply ID to TextEdit if provided
            if let Some(id) = self.id {
                text_edit = text_edit.id(id);
            }

            return child_ui.add(text_edit);
        }

        response
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new("")
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
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: None,
            placeholder: "Search...".to_string(),
            width: None,
        }
    }

    /// Set ID for state persistence
    #[must_use]
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set placeholder text
    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set width
    #[must_use]
    pub const fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Show the search input
    pub fn show(self, ui: &mut Ui, text: &mut String, theme: &crate::Theme) -> InputResponse {
        let mut input = Input::new(&self.placeholder)
            .left_icon("🔍")
            .width(self.width.unwrap_or(300.0));

        if let Some(id) = self.id {
            input = input.id(id);
        }

        input.show(ui, text, theme)
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
            .description("Required field")
            .variant(InputVariant::Outlined)
            .state(InputState::Error);

        assert_eq!(input.label, Some("Username".to_string()));
        assert_eq!(input.description, Some("Required field".to_string()));
        assert_eq!(input.variant, InputVariant::Outlined);
        assert_eq!(input.state, InputState::Error);
    }

    #[test]
    fn test_search_input() {
        let search = SearchInput::new().placeholder("Search files...");
        assert_eq!(search.placeholder, "Search files...");
    }
}
