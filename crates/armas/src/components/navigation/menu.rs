//! Menu Component
//!
//! Dropdown and context menus

use crate::{Badge, BadgeColor, Popover, PopoverColor, PopoverPosition, PopoverStyle};
use egui::{vec2, Id, Key, Rect, Sense};

/// Menu item
#[derive(Clone)]
pub struct MenuItem {
    /// Item label
    pub label: String,
    /// Optional icon/emoji
    pub icon: Option<String>,
    /// Optional keyboard shortcut display
    pub shortcut: Option<String>,
    /// Whether the item is disabled
    pub disabled: bool,
    /// Optional badge text
    pub badge: Option<String>,
    /// Optional badge color
    pub badge_color: Option<BadgeColor>,
    /// Whether this is a separator
    pub is_separator: bool,
}

impl MenuItem {
    /// Create a new menu item
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            shortcut: None,
            disabled: false,
            badge: None,
            badge_color: None,
            is_separator: false,
        }
    }

    /// Create a separator
    pub fn separator() -> Self {
        Self {
            label: String::new(),
            icon: None,
            shortcut: None,
            disabled: false,
            badge: None,
            badge_color: None,
            is_separator: true,
        }
    }

    /// Set an icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set a keyboard shortcut
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set a badge
    pub fn badge(mut self, text: impl Into<String>, color: BadgeColor) -> Self {
        self.badge = Some(text.into());
        self.badge_color = Some(color);
        self
    }
}

/// Menu component
///
/// Dropdown and context menus with keyboard navigation
///
/// # Example
///
/// ```rust,no_run
/// # use egui::{Context, Rect};
/// # fn example(ctx: &Context, anchor_rect: Rect) {
/// use armas::Menu;
///
/// let mut menu = Menu::new("context_menu");
/// let response = menu.show(ctx, anchor_rect, |menu| {
///     menu.item("Copy").shortcut("Cmd+C");
///     menu.separator();
///     menu.item("Delete").icon("🗑️").disabled(true);
/// });
/// # }
/// ```
#[derive(Clone)]
pub struct Menu {
    id: Id,
    popover: Popover,
    selected_index: Option<usize>,
    // Internal state management
    is_open: Option<bool>, // None = use internal state, Some = external control
}

impl Menu {
    /// Create a new menu
    pub fn new(id: impl Into<Id>) -> Self {
        let id = id.into();
        Self {
            id,
            popover: Popover::new(id.with("popover"))
                .position(PopoverPosition::Bottom)
                .width(220.0),
            selected_index: None,
            is_open: None, // Use internal state by default
        }
    }

    /// Set the menu to be open (for external control)
    pub fn open(mut self, is_open: bool) -> Self {
        self.is_open = Some(is_open);
        self
    }


    /// Set the menu position
    pub fn position(mut self, position: PopoverPosition) -> Self {
        self.popover = self.popover.position(position);
        self
    }

    /// Set the menu width
    pub fn width(mut self, width: f32) -> Self {
        self.popover = self.popover.width(width);
        self
    }

    /// Set the menu color
    pub fn color(mut self, color: PopoverColor) -> Self {
        self.popover = self.popover.color(color);
        self
    }

    /// Set the menu style
    pub fn style(mut self, style: PopoverStyle) -> Self {
        self.popover = self.popover.style(style);
        self
    }

    /// Show the menu anchored to a button/element with closure-based API
    pub fn show<R>(
        &mut self,
        ctx: &egui::Context,
        anchor_rect: Rect,
        content: impl FnOnce(&mut MenuBuilder) -> R,
    ) -> MenuResponse {
        let theme = crate::ext::ArmasContextExt::armas_theme(ctx);

        let mut response = MenuResponse {
            selected: None,
            clicked_outside: false,
        };

        // Load state from egui memory if not externally controlled
        let state_id = self.id.with("menu_state");
        let mut is_open = if let Some(external_open) = self.is_open {
            external_open
        } else {
            ctx.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false))
        };

        // Build items using closure
        let mut builder = MenuBuilder {
            items: Vec::new(),
            item_index: 0,
        };
        content(&mut builder);
        let items = builder.items;

        // Handle keyboard navigation
        if is_open {
            ctx.input(|i| {
                if i.key_pressed(Key::ArrowDown) {
                    Self::navigate_down(&mut self.selected_index, &items);
                } else if i.key_pressed(Key::ArrowUp) {
                    Self::navigate_up(&mut self.selected_index, &items);
                } else if i.key_pressed(Key::Enter) {
                    if let Some(idx) = self.selected_index {
                        if idx < items.len()
                            && !items[idx].disabled
                            && !items[idx].is_separator
                        {
                            response.selected = Some(idx);
                            is_open = false;
                            self.selected_index = None;
                        }
                    }
                } else if i.key_pressed(Key::Escape) {
                    is_open = false;
                    self.selected_index = None;
                }
            });
        } else {
            self.selected_index = None;
        }

        let mut should_close = false;

        // Set popover open state externally
        self.popover.set_open(is_open);

        let popover_response = self.popover.show(ctx, &theme, anchor_rect, |ui| {
            ui.spacing_mut().item_spacing = vec2(0.0, 2.0);

            for (idx, item) in items.iter().enumerate() {
                if item.is_separator {
                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(4.0);
                    continue;
                }

                let is_selected = self.selected_index == Some(idx);

                let (rect, item_response) = ui.allocate_exact_size(
                    vec2(ui.available_width(), 32.0),
                    if item.disabled {
                        Sense::hover()
                    } else {
                        Sense::click()
                    },
                );

                // Update hover state
                if item_response.hovered() && !item.disabled {
                    self.selected_index = Some(idx);
                }

                // Draw background
                if (is_selected || item_response.hovered()) && !item.disabled {
                    ui.painter().rect_filled(rect, 4.0, theme.surface_variant());
                }

                // Draw content
                ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        ui.add_space(12.0);

                        // Icon
                        if let Some(icon) = &item.icon {
                            ui.label(icon);
                        }

                        // Label
                        let text_color = if item.disabled {
                            theme.on_surface_variant().linear_multiply(0.5)
                        } else {
                            theme.on_surface()
                        };

                        ui.colored_label(text_color, &item.label);

                        // Badge and shortcut on the right
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(12.0);

                            // Badge
                            if let Some(badge_text) = &item.badge {
                                let color = item.badge_color.unwrap_or(BadgeColor::Neutral);
                                Badge::new(badge_text).color(color).show(ui);
                                ui.add_space(4.0);
                            }

                            // Shortcut
                            if let Some(shortcut) = &item.shortcut {
                                ui.colored_label(
                                    theme.on_surface_variant().linear_multiply(0.7),
                                    shortcut,
                                );
                            }
                        });
                    });
                });

                // Handle click
                if item_response.clicked() && !item.disabled {
                    response.selected = Some(idx);
                    should_close = true;
                    self.selected_index = None;
                }
            }
        });

        if should_close {
            is_open = false;
        }

        if popover_response.clicked_outside {
            response.clicked_outside = true;
        }

        // Persist state if not externally controlled
        if self.is_open.is_none() {
            ctx.data_mut(|d| d.insert_temp(state_id, is_open));
        }

        response
    }

    fn navigate_down(selected_index: &mut Option<usize>, items: &[MenuItemData]) {
        let start_idx = selected_index.map(|i| i + 1).unwrap_or(0);

        for i in start_idx..items.len() {
            if !items[i].is_separator && !items[i].disabled {
                *selected_index = Some(i);
                return;
            }
        }

        // Wrap around
        for i in 0..start_idx {
            if !items[i].is_separator && !items[i].disabled {
                *selected_index = Some(i);
                return;
            }
        }
    }

    fn navigate_up(selected_index: &mut Option<usize>, items: &[MenuItemData]) {
        let start_idx = selected_index.unwrap_or(items.len()) as isize - 1;

        for i in (0..=start_idx).rev() {
            let idx = i as usize;
            if idx < items.len() && !items[idx].is_separator && !items[idx].disabled
            {
                *selected_index = Some(idx);
                return;
            }
        }

        // Wrap around
        for i in (start_idx + 1..items.len() as isize).rev() {
            let idx = i as usize;
            if !items[idx].is_separator && !items[idx].disabled {
                *selected_index = Some(idx);
                return;
            }
        }
    }
}

/// Builder for adding menu items
pub struct MenuBuilder {
    items: Vec<MenuItemData>,
    item_index: usize,
}

#[derive(Clone)]
struct MenuItemData {
    label: String,
    icon: Option<String>,
    shortcut: Option<String>,
    disabled: bool,
    badge: Option<String>,
    badge_color: Option<BadgeColor>,
    is_separator: bool,
}

impl MenuBuilder {
    /// Add a menu item
    pub fn item(&mut self, label: &str) -> MenuItemBuilder<'_> {
        let item_data = MenuItemData {
            label: label.to_string(),
            icon: None,
            shortcut: None,
            disabled: false,
            badge: None,
            badge_color: None,
            is_separator: false,
        };

        self.items.push(item_data);
        let current_index = self.item_index;
        self.item_index += 1;

        MenuItemBuilder {
            items: &mut self.items,
            item_index: current_index,
        }
    }

    /// Add a separator
    pub fn separator(&mut self) {
        self.items.push(MenuItemData {
            label: String::new(),
            icon: None,
            shortcut: None,
            disabled: false,
            badge: None,
            badge_color: None,
            is_separator: true,
        });
        self.item_index += 1;
    }
}

/// Builder for chaining menu item modifiers
pub struct MenuItemBuilder<'a> {
    items: &'a mut Vec<MenuItemData>,
    item_index: usize,
}

impl<'a> MenuItemBuilder<'a> {
    /// Set an icon
    pub fn icon(self, icon: &str) -> Self {
        if let Some(item) = self.items.get_mut(self.item_index) {
            item.icon = Some(icon.to_string());
        }
        self
    }

    /// Set a keyboard shortcut
    pub fn shortcut(self, shortcut: &str) -> Self {
        if let Some(item) = self.items.get_mut(self.item_index) {
            item.shortcut = Some(shortcut.to_string());
        }
        self
    }

    /// Set disabled state
    pub fn disabled(self, disabled: bool) -> Self {
        if let Some(item) = self.items.get_mut(self.item_index) {
            item.disabled = disabled;
        }
        self
    }

    /// Set a badge
    pub fn badge(self, text: &str, color: BadgeColor) -> Self {
        if let Some(item) = self.items.get_mut(self.item_index) {
            item.badge = Some(text.to_string());
            item.badge_color = Some(color);
        }
        self
    }
}

/// Response from a menu
#[derive(Debug, Clone, Copy)]
pub struct MenuResponse {
    /// Index of selected item (if any)
    pub selected: Option<usize>,
    /// Whether the user clicked outside the menu
    pub clicked_outside: bool,
}
