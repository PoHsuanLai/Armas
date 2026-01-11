//! Breadcrumbs Component
//!
//! Navigation path indicator showing the current location in a hierarchy

use crate::ext::ArmasContextExt;
use crate::layout::HStack;
use crate::{Button, ButtonVariant, Theme};
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
/// use armas::{Breadcrumbs, BreadcrumbItem};
///
/// let mut breadcrumbs = Breadcrumbs::new()
///     .separator("/")
///     .add_item(BreadcrumbItem::new("Home").icon("🏠"))
///     .add_item(BreadcrumbItem::new("Projects"))
///     .add_item(BreadcrumbItem::new("Armas").current());
///
/// let response = breadcrumbs.show(ui);
/// if let Some(index) = response.clicked {
///     println!("Clicked breadcrumb at index: {}", index);
/// }
/// ```
pub struct Breadcrumbs {
    items: Vec<BreadcrumbItem>,
    separator: String,
    spacing: f32,
    show_home_icon: bool,
}

impl Breadcrumbs {
    /// Create a new breadcrumbs component
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            separator: "›".to_string(),
            spacing: 4.0,
            show_home_icon: false,
        }
    }

    /// Add a breadcrumb item
    pub fn add_item(mut self, item: BreadcrumbItem) -> Self {
        self.items.push(item);
        self
    }

    /// Add a simple item by label
    pub fn item(self, label: impl Into<String>) -> Self {
        self.add_item(BreadcrumbItem::new(label))
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

    /// Show the breadcrumbs
    pub fn show(self, ui: &mut Ui) -> BreadcrumbsResponse {
        let theme = ui.ctx().armas_theme();
        let mut clicked = None;

        HStack::new(self.spacing).show(ui, |ui| {
            // Optional home icon
            if self.show_home_icon {
                if Button::new("🏠")
                    .variant(ButtonVariant::Text)
                    .show(ui)
                    .clicked()
                {
                    clicked = Some(0);
                }

                if !self.items.is_empty() {
                    ui.add_space(self.spacing);
                    ui.colored_label(
                        theme.on_surface_variant().linear_multiply(0.6),
                        &self.separator,
                    );
                    ui.add_space(self.spacing);
                }
            }

            // Breadcrumb items
            for (idx, item) in self.items.iter().enumerate() {
                // Don't show separator before first item (unless we showed home icon)
                if idx > 0 {
                    ui.add_space(self.spacing);
                    ui.colored_label(
                        theme.on_surface_variant().linear_multiply(0.6),
                        &self.separator,
                    );
                    ui.add_space(self.spacing);
                }

                // Build label with optional icon
                let label = if let Some(icon) = &item.icon {
                    format!("{} {}", icon, item.label)
                } else {
                    item.label.clone()
                };

                // Current item is not clickable
                if item.is_current {
                    ui.colored_label(theme.on_surface(), &label);
                } else {
                    // Clickable items use text button
                    if Button::new(&label)
                        .variant(ButtonVariant::Text)
                        .show(ui)
                        .clicked()
                    {
                        clicked = Some(if self.show_home_icon { idx + 1 } else { idx });
                    }
                }
            }
        });

        BreadcrumbsResponse { clicked }
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
