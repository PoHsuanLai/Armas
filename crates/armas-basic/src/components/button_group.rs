//! Button Group Component (shadcn/ui style)
//!
//! Groups buttons with connected borders, removing redundant inner borders
//! between adjacent buttons.
//!
//! ```rust,no_run
//! # use egui::Ui;
//! # fn example(ui: &mut Ui) {
//! use armas_basic::prelude::*;
//!
//! ButtonGroup::new("actions").show(ui, |ui| {
//!     Button::new("Bold").variant(ButtonVariant::Outline).show(ui);
//!     Button::new("Italic").variant(ButtonVariant::Outline).show(ui);
//!     Button::new("Underline").variant(ButtonVariant::Outline).show(ui);
//! });
//! # }
//! ```

use egui::{Id, Sense, Ui};

/// Button group orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonGroupOrientation {
    /// Buttons arranged left to right.
    Horizontal,
    /// Buttons stacked top to bottom.
    Vertical,
}

/// Button Group — groups buttons with connected borders.
pub struct ButtonGroup {
    id: Id,
    orientation: ButtonGroupOrientation,
}

/// Response from a button group.
pub struct ButtonGroupResponse {
    /// The UI response.
    pub response: egui::Response,
}

impl ButtonGroup {
    /// Create a new button group with a unique ID.
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            orientation: ButtonGroupOrientation::Horizontal,
        }
    }

    /// Set the button group orientation.
    #[must_use]
    pub const fn orientation(mut self, o: ButtonGroupOrientation) -> Self {
        self.orientation = o;
        self
    }

    /// Show the button group. Render buttons inside the closure.
    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> ButtonGroupResponse {
        let is_horizontal = self.orientation == ButtonGroupOrientation::Horizontal;

        // Create a child UI with zero item spacing so buttons are flush
        let layout = if is_horizontal {
            egui::Layout::left_to_right(egui::Align::Center)
        } else {
            egui::Layout::top_down(egui::Align::LEFT)
        };

        let inner_response = ui.with_layout(layout, |ui| {
            // Remove spacing between items so buttons are flush
            if is_horizontal {
                ui.spacing_mut().item_spacing.x = 0.0;
            } else {
                ui.spacing_mut().item_spacing.y = 0.0;
            }

            // Suppress individual button corner radius and borders
            // by setting all rounding to 0 — we'll draw our own outer border
            ui.style_mut().visuals.widgets.inactive.corner_radius = 0.into();
            ui.style_mut().visuals.widgets.hovered.corner_radius = 0.into();
            ui.style_mut().visuals.widgets.active.corner_radius = 0.into();

            content(ui);
        });

        let group_rect = inner_response.response.rect;
        let response = ui.interact(group_rect, self.id, Sense::hover());

        ButtonGroupResponse { response }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_group_builder() {
        let bg = ButtonGroup::new("test").orientation(ButtonGroupOrientation::Vertical);
        assert_eq!(bg.orientation, ButtonGroupOrientation::Vertical);
    }
}
