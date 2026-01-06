//! Section Header Component
//!
//! Collapsible section header with arrow indicator.
//! Matches Studio One's "Sends ▼" style headers.

use crate::theme::Theme;
use egui;

/// Collapsible section header component
pub struct SectionHeader<'a> {
    pub label: &'a str,
    pub collapsed: bool,
}

impl<'a> SectionHeader<'a> {
    pub fn new(label: &'a str, collapsed: bool) -> Self {
        Self { label, collapsed }
    }

    pub fn show(self, ui: &mut egui::Ui, theme: &Theme) -> egui::Response {
        let font_size = ui.spacing().interact_size.y * 0.45;
        let arrow = if self.collapsed { "▶" } else { "▼" };
        let text = format!("{} {}", self.label, arrow);

        let response = ui.add(
            egui::Label::new(
                egui::RichText::new(text)
                    .size(font_size * 0.9)
                    .color(theme.primary()),
            )
            .sense(egui::Sense::click()),
        );

        response
    }
}
