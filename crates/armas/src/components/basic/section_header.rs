//! Section Header Component
//!
//! Collapsible section header with directional arrow indicator.
//!
//! A clickable header component that displays a label with an arrow
//! indicating whether the section is expanded or collapsed. Perfect for
//! accordion-style interfaces or collapsible panels.

use crate::ext::ArmasContextExt;
use egui;

/// Collapsible section header component
///
/// Displays a clickable section header with an arrow indicator.
/// The arrow points right when collapsed, down when expanded.
///
/// # Example
///
/// ```rust,no_run
/// use armas::components::SectionHeader;
///
/// fn ui(ui: &mut egui::Ui, is_collapsed: &mut bool) {
///     if SectionHeader::new("Settings", *is_collapsed).show(ui).clicked() {
///         *is_collapsed = !*is_collapsed;
///     }
/// }
/// ```
pub struct SectionHeader<'a> {
    pub label: &'a str,
    pub collapsed: bool,
}

impl<'a> SectionHeader<'a> {
    /// Create a new section header
    pub fn new(label: &'a str, collapsed: bool) -> Self {
        Self { label, collapsed }
    }

    /// Show the section header
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = ui.ctx().armas_theme();
        let arrow = if self.collapsed { "▶" } else { "▼" };

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;

            // Arrow on the left
            ui.label(egui::RichText::new(arrow).size(14.0).color(theme.primary()));

            // Label
            ui.add(
                egui::Label::new(
                    egui::RichText::new(self.label)
                        .size(16.0)
                        .color(theme.on_surface()),
                )
                .sense(egui::Sense::click()),
            )
        })
        .inner
    }
}
