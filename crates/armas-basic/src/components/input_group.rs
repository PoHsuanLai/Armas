//! Input Group Component (shadcn/ui style)
//!
//! A container that wraps a text input with leading/trailing addon slots
//! for icons, text labels, or buttons.
//!
//! ```rust,no_run
//! # use egui::Ui;
//! # fn example(ui: &mut Ui) {
//! use armas_basic::prelude::*;
//!
//! let mut text = String::new();
//! InputGroup::new("search_input")
//!     .leading(|ui| { ui.label("🔍"); })
//!     .show(ui, &mut text);
//! # }
//! ```

use crate::ext::ArmasContextExt;
use egui::{vec2, Id, Sense, Stroke, Ui};

// Constants
const HEIGHT: f32 = 40.0;
const CORNER_RADIUS: f32 = 6.0;
const ADDON_PADDING: f32 = 10.0;
const INPUT_PADDING: f32 = 12.0;
// Font size resolved from theme.typography.base at show-time

/// Boxed closure for addon content.
type AddonFn = Box<dyn FnOnce(&mut Ui)>;

/// Input group — a text input with leading/trailing addons.
pub struct InputGroup {
    id: Id,
    width: Option<f32>,
    placeholder: String,
    leading: Option<AddonFn>,
    trailing: Option<AddonFn>,
}

/// Response from an input group.
pub struct InputGroupResponse {
    /// The UI response.
    pub response: egui::Response,
    /// Whether the text changed this frame.
    pub changed: bool,
}

impl InputGroup {
    /// Create a new input group with a unique ID.
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            width: None,
            placeholder: String::new(),
            leading: None,
            trailing: None,
        }
    }

    /// Set the input group width.
    #[must_use]
    pub const fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }

    /// Set placeholder text.
    #[must_use]
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set the leading (left) addon content.
    #[must_use]
    pub fn leading(mut self, content: impl FnOnce(&mut Ui) + 'static) -> Self {
        self.leading = Some(Box::new(content));
        self
    }

    /// Set the trailing (right) addon content.
    #[must_use]
    pub fn trailing(mut self, content: impl FnOnce(&mut Ui) + 'static) -> Self {
        self.trailing = Some(Box::new(content));
        self
    }

    /// Show the input group.
    pub fn show(self, ui: &mut Ui, text: &mut String) -> InputGroupResponse {
        let theme = ui.ctx().armas_theme();
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().min(300.0));

        // Load persisted text state
        let state_id = self.id.with("input_state");
        let stored: Option<String> = ui.ctx().data_mut(|d| d.get_temp(state_id));
        if let Some(stored) = stored {
            *text = stored;
        }

        // Allocate outer rect
        let (outer_rect, _) = ui.allocate_exact_size(vec2(width, HEIGHT), Sense::hover());

        // Draw outer border
        ui.painter()
            .rect_filled(outer_rect, CORNER_RADIUS, theme.background());
        ui.painter().rect_stroke(
            outer_rect,
            CORNER_RADIUS,
            Stroke::new(1.0, theme.input()),
            egui::epaint::StrokeKind::Inside,
        );

        // Track widths for layout
        let mut leading_width = 0.0;
        let mut trailing_width = 0.0;

        // Render leading addon
        if let Some(leading) = self.leading {
            let addon_rect = egui::Rect::from_min_size(
                outer_rect.left_top(),
                vec2(outer_rect.width() * 0.3, HEIGHT), // max 30% for measurement
            );

            let mut child_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(addon_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            );
            child_ui.set_clip_rect(outer_rect);
            child_ui.add_space(ADDON_PADDING);
            child_ui.style_mut().visuals.override_text_color = Some(theme.muted_foreground());
            leading(&mut child_ui);
            child_ui.add_space(ADDON_PADDING);
            leading_width = child_ui.min_rect().width();

            // Draw separator between leading and input
            let sep_x = outer_rect.left() + leading_width;
            ui.painter().line_segment(
                [
                    egui::Pos2::new(sep_x, outer_rect.top() + 1.0),
                    egui::Pos2::new(sep_x, outer_rect.bottom() - 1.0),
                ],
                Stroke::new(1.0, theme.border()),
            );
        }

        // Render trailing addon (measure first)
        if let Some(trailing) = self.trailing {
            // Render from the right side
            let addon_rect = egui::Rect::from_min_size(
                egui::Pos2::new(
                    outer_rect.right() - outer_rect.width() * 0.3,
                    outer_rect.top(),
                ),
                vec2(outer_rect.width() * 0.3, HEIGHT),
            );

            let mut child_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(addon_rect)
                    .layout(egui::Layout::right_to_left(egui::Align::Center)),
            );
            child_ui.set_clip_rect(outer_rect);
            child_ui.add_space(ADDON_PADDING);
            child_ui.style_mut().visuals.override_text_color = Some(theme.muted_foreground());
            trailing(&mut child_ui);
            child_ui.add_space(ADDON_PADDING);
            trailing_width = child_ui.min_rect().width();

            // Draw separator between input and trailing
            let sep_x = outer_rect.right() - trailing_width;
            ui.painter().line_segment(
                [
                    egui::Pos2::new(sep_x, outer_rect.top() + 1.0),
                    egui::Pos2::new(sep_x, outer_rect.bottom() - 1.0),
                ],
                Stroke::new(1.0, theme.border()),
            );
        }

        // Render input in the remaining space
        let input_left = outer_rect.left() + leading_width;
        let input_right = outer_rect.right() - trailing_width;
        let input_rect = egui::Rect::from_min_max(
            egui::Pos2::new(input_left + INPUT_PADDING, outer_rect.top()),
            egui::Pos2::new(input_right - INPUT_PADDING, outer_rect.bottom()),
        );

        let prev_text = text.clone();

        let mut child_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt(self.id.with("input_area"))
                .max_rect(input_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        child_ui.set_clip_rect(egui::Rect::from_min_max(
            egui::Pos2::new(input_left, outer_rect.top()),
            egui::Pos2::new(input_right, outer_rect.bottom()),
        ));

        // Style the text edit to be borderless
        child_ui.style_mut().visuals.extreme_bg_color = theme.background();
        child_ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::NONE;
        child_ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
        child_ui.style_mut().visuals.widgets.active.bg_stroke = Stroke::NONE;
        child_ui.style_mut().visuals.widgets.inactive.bg_fill = theme.background();
        child_ui.style_mut().visuals.widgets.hovered.bg_fill = theme.background();
        child_ui.style_mut().visuals.widgets.active.bg_fill = theme.background();
        child_ui.style_mut().visuals.selection.bg_fill = theme.primary();

        let te = egui::TextEdit::singleline(text)
            .id(self.id.with("input"))
            .font(egui::TextStyle::Body)
            .text_color(theme.foreground())
            .desired_width(input_rect.width())
            .hint_text(
                egui::RichText::new(&self.placeholder)
                    .size(theme.typography.base)
                    .color(theme.muted_foreground()),
            )
            .frame(false);

        let te_response = child_ui.add(te);
        let changed = *text != prev_text;

        // Focus ring on the outer border when input is focused
        if te_response.has_focus() {
            ui.painter().rect_stroke(
                outer_rect,
                CORNER_RADIUS,
                Stroke::new(2.0, theme.ring()),
                egui::epaint::StrokeKind::Inside,
            );
        }

        // Persist text state
        ui.ctx().data_mut(|d| d.insert_temp(state_id, text.clone()));

        InputGroupResponse {
            response: te_response,
            changed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_group_builder() {
        let ig = InputGroup::new("test")
            .width(400.0)
            .placeholder("Search...");
        assert_eq!(ig.width, Some(400.0));
        assert_eq!(ig.placeholder, "Search...");
    }
}
