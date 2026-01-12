//! Breadcrumbs Component
//!
//! Navigation path indicator showing the current location in a hierarchy

use crate::ext::ArmasContextExt;
use crate::{Button, ButtonVariant};
use egui::Ui;

/// A single breadcrumb item
#[derive(Clone)]
pub struct BreadcrumbItem {
    /// Display label
    pub label: String,
    /// Optional icon/emoji
    pub icon: Option<String>,
    /// Whether this is the current/active item
    pub is_current: bool,
}

impl BreadcrumbItem {
    /// Create a new breadcrumb item
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            is_current: false,
        }
    }

    /// Set an icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Mark as the current item
    pub fn current(mut self) -> Self {
        self.is_current = true;
        self
    }
}

/// Breadcrumbs navigation component
///
/// Shows a navigation path with clickable items
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::Breadcrumbs;
///
/// Breadcrumbs::new()
///     .separator("/")
///     .show_home_icon(true)
///     .show(ui, |breadcrumbs| {
///         breadcrumbs.item("Home", Some("🏠"));
///         breadcrumbs.item("Projects", None);
///         breadcrumbs.item("Armas", None).current();
///     });
/// # }
/// ```
pub struct Breadcrumbs {
    separator: String,
    spacing: f32,
    show_home_icon: bool,
}

impl Breadcrumbs {
    /// Create a new breadcrumbs component
    pub fn new() -> Self {
        Self {
            separator: "›".to_string(),
            spacing: 4.0,
            show_home_icon: false,
        }
    }

    /// Set the separator between items
    pub fn separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }

    /// Set spacing between items
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Show a home icon before the first item
    pub fn show_home_icon(mut self, show: bool) -> Self {
        self.show_home_icon = show;
        self
    }

    /// Show the breadcrumbs with closure-based API
    pub fn show<R>(
        self,
        ui: &mut Ui,
        content: impl FnOnce(&mut BreadcrumbsBuilder) -> R,
    ) -> BreadcrumbsResponse {
        let theme = ui.ctx().armas_theme();

        let mut clicked: Option<usize> = None;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = self.spacing;

            // Optional home icon
            if self.show_home_icon {
                if Button::new("🏠")
                    .variant(ButtonVariant::Text)
                    .show(ui)
                    .clicked()
                {
                    clicked = Some(0);
                }

                ui.add_space(self.spacing);
                ui.colored_label(
                    theme.on_surface_variant().linear_multiply(0.6),
                    &self.separator,
                );
                ui.add_space(self.spacing);
            }

            let mut builder = BreadcrumbsBuilder {
                ui,
                separator: &self.separator,
                spacing: self.spacing,
                show_home_icon: self.show_home_icon,
                item_index: 0,
                clicked: &mut clicked,
            };

            content(&mut builder);
        });

        BreadcrumbsResponse { clicked }
    }

}

/// Builder for adding breadcrumb items
pub struct BreadcrumbsBuilder<'a> {
    ui: &'a mut Ui,
    separator: &'a str,
    spacing: f32,
    show_home_icon: bool,
    item_index: usize,
    clicked: &'a mut Option<usize>,
}

impl<'a> BreadcrumbsBuilder<'a> {
    /// Add a breadcrumb item
    pub fn item(&mut self, label: &str, icon: Option<&str>) -> ItemBuilder<'_> {
        let theme = self.ui.ctx().armas_theme();

        // Show separator before this item (if not first)
        if self.item_index > 0 || self.show_home_icon {
            self.ui.add_space(self.spacing);
            self.ui.colored_label(
                theme.on_surface_variant().linear_multiply(0.6),
                self.separator,
            );
            self.ui.add_space(self.spacing);
        }

        let mut item_builder = ItemBuilder {
            ui: self.ui,
            label: label.to_string(),
            icon: icon.map(|s| s.to_string()),
            is_current: false,
            item_index: self.item_index,
            show_home_icon: self.show_home_icon,
            clicked: &mut self.clicked,
        };

        item_builder.render();
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
    show_home_icon: bool,
    clicked: &'a mut Option<usize>,
}

impl<'a> ItemBuilder<'a> {
    /// Mark this item as the current/active item
    pub fn current(mut self) -> Self {
        self.is_current = true;
        // Re-render with updated state
        self.render();
        self
    }

    fn render(&mut self) {
        let theme = self.ui.ctx().armas_theme();

        // Build label with optional icon
        let label = if let Some(icon) = &self.icon {
            format!("{} {}", icon, self.label)
        } else {
            self.label.clone()
        };

        // Current item is not clickable
        if self.is_current {
            self.ui.colored_label(theme.on_surface(), &label);
        } else {
            // Clickable items use text button
            if Button::new(&label)
                .variant(ButtonVariant::Text)
                .show(self.ui)
                .clicked()
            {
                *self.clicked = Some(if self.show_home_icon {
                    self.item_index + 1
                } else {
                    self.item_index
                });
            }
        }
    }
}

impl Default for Breadcrumbs {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from breadcrumbs
#[derive(Debug, Clone, Copy)]
pub struct BreadcrumbsResponse {
    /// Index of clicked item (if any)
    pub clicked: Option<usize>,
}
