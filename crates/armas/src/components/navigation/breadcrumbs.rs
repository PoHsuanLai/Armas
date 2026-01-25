//! Breadcrumbs Component
//!
//! Navigation path indicator styled like shadcn/ui Breadcrumb.
//! Shows the current location in a hierarchy with clickable navigation items.
//!
//! # Example
//!
//! ```rust,no_run
//! # use egui::Ui;
//! # fn example(ui: &mut Ui) {
//! use armas::Breadcrumbs;
//!
//! Breadcrumbs::new()
//!     .show(ui, |breadcrumbs| {
//!         breadcrumbs.item("Home", None);
//!         breadcrumbs.item("Projects", None);
//!         breadcrumbs.item("Armas", None).current();
//!     });
//! # }
//! ```

use crate::ext::ArmasContextExt;
use egui::{Sense, Ui};

// shadcn Breadcrumb constants
const ITEM_GAP: f32 = 6.0; // gap-1.5
const FONT_SIZE: f32 = 14.0; // text-sm
const SEPARATOR_SIZE: f32 = 14.0; // size-3.5

/// Breadcrumbs navigation component
///
/// Shows a navigation path with clickable items, styled like shadcn/ui.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::Breadcrumbs;
///
/// Breadcrumbs::new()
///     .show(ui, |breadcrumbs| {
///         breadcrumbs.item("Home", None);
///         breadcrumbs.item("Projects", None);
///         breadcrumbs.item("Armas", None).current();
///     });
/// # }
/// ```
pub struct Breadcrumbs {
    spacing: f32,
}

impl Breadcrumbs {
    /// Create a new breadcrumbs component
    pub fn new() -> Self {
        Self { spacing: ITEM_GAP }
    }

    /// Set spacing between items (default: 6.0)
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Show the breadcrumbs with closure-based API
    pub fn show<R>(
        self,
        ui: &mut Ui,
        content: impl FnOnce(&mut BreadcrumbsBuilder) -> R,
    ) -> BreadcrumbsResponse {
        let mut clicked: Option<usize> = None;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = self.spacing;

            let mut builder = BreadcrumbsBuilder {
                ui,
                spacing: self.spacing,
                item_index: 0,
                clicked: &mut clicked,
            };

            content(&mut builder);
        });

        BreadcrumbsResponse { clicked }
    }
}

impl Default for Breadcrumbs {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for adding breadcrumb items
pub struct BreadcrumbsBuilder<'a> {
    ui: &'a mut Ui,
    spacing: f32,
    item_index: usize,
    clicked: &'a mut Option<usize>,
}

impl<'a> BreadcrumbsBuilder<'a> {
    /// Add a breadcrumb item with optional icon
    pub fn item(&mut self, label: &str, icon: Option<&str>) -> ItemBuilder<'_> {
        let theme = self.ui.ctx().armas_theme();

        // Show separator before this item (if not first)
        if self.item_index > 0 {
            // ChevronRight separator - shadcn uses lucide ChevronRight at size-3.5
            self.ui.add_space(self.spacing);

            // Draw chevron right icon
            let (rect, _) = self.ui.allocate_exact_size(
                egui::vec2(SEPARATOR_SIZE, SEPARATOR_SIZE),
                Sense::hover(),
            );

            if self.ui.is_rect_visible(rect) {
                let painter = self.ui.painter();
                let color = theme.muted_foreground();
                let stroke = egui::Stroke::new(1.5, color);

                // Draw > shape
                let center = rect.center();
                let half = SEPARATOR_SIZE * 0.2;
                painter.line_segment(
                    [
                        egui::pos2(center.x - half, center.y - half * 1.5),
                        egui::pos2(center.x + half, center.y),
                    ],
                    stroke,
                );
                painter.line_segment(
                    [
                        egui::pos2(center.x + half, center.y),
                        egui::pos2(center.x - half, center.y + half * 1.5),
                    ],
                    stroke,
                );
            }

            self.ui.add_space(self.spacing);
        }

        let item_builder = ItemBuilder {
            ui: self.ui,
            label: label.to_string(),
            icon: icon.map(|s| s.to_string()),
            is_current: false,
            item_index: self.item_index,
            clicked: self.clicked,
            rendered: false,
        };

        self.item_index += 1;
        item_builder
    }
}

/// Builder for chaining item modifiers
pub struct ItemBuilder<'a> {
    ui: &'a mut Ui,
    label: String,
    icon: Option<String>,
    is_current: bool,
    item_index: usize,
    clicked: &'a mut Option<usize>,
    rendered: bool,
}

impl<'a> ItemBuilder<'a> {
    /// Mark this item as the current/active item (non-clickable)
    pub fn current(mut self) -> Self {
        self.is_current = true;
        self
    }

    fn render(&mut self) {
        if self.rendered {
            return;
        }
        self.rendered = true;

        let theme = self.ui.ctx().armas_theme();

        // Build label with optional icon
        let display_label = if let Some(icon) = &self.icon {
            format!("{} {}", icon, self.label)
        } else {
            self.label.clone()
        };

        // Current item: text-foreground font-normal (non-clickable)
        if self.is_current {
            self.ui.label(
                egui::RichText::new(&display_label)
                    .size(FONT_SIZE)
                    .color(theme.foreground()),
            );
        } else {
            // Clickable items: text-muted-foreground, hover:text-foreground
            let response = self.ui.add(
                egui::Label::new(
                    egui::RichText::new(&display_label)
                        .size(FONT_SIZE)
                        .color(theme.muted_foreground()),
                )
                .sense(Sense::click()),
            );

            // Apply hover color
            if response.hovered() {
                // Re-render with foreground color on hover
                let rect = response.rect;
                self.ui.painter().rect_filled(
                    rect,
                    0.0,
                    egui::Color32::TRANSPARENT,
                );
                self.ui.painter().text(
                    rect.left_center(),
                    egui::Align2::LEFT_CENTER,
                    &display_label,
                    egui::FontId::proportional(FONT_SIZE),
                    theme.foreground(),
                );
            }

            if response.clicked() {
                *self.clicked = Some(self.item_index);
            }
        }
    }
}

impl<'a> Drop for ItemBuilder<'a> {
    fn drop(&mut self) {
        self.render();
    }
}

/// Response from breadcrumbs
#[derive(Debug, Clone, Copy)]
pub struct BreadcrumbsResponse {
    /// Index of clicked item (if any)
    pub clicked: Option<usize>,
}
