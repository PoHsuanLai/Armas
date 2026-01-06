//! Menu Component
//!
//! Dropdown and context menus

use crate::layout::HStack;
use crate::{Badge, BadgeColor, Popover, PopoverPosition, Theme};
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
pub struct Menu {
    id: Id,
    items: Vec<MenuItem>,
    popover: Popover,
    selected_index: Option<usize>,
}

impl Menu {
    /// Create a new menu
    pub fn new(id: impl Into<Id>) -> Self {
        let id = id.into();
        Self {
            id,
            items: Vec::new(),
            popover: Popover::new(id.with("popover"))
                .position(PopoverPosition::Bottom)
                .width(220.0),
            selected_index: None,
        }
    }

    /// Add an item to the menu
    pub fn add_item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// Add a simple item
    pub fn item(self, label: impl Into<String>) -> Self {
        self.add_item(MenuItem::new(label))
    }

    /// Add a separator
    pub fn separator(self) -> Self {
        self.add_item(MenuItem::separator())
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

    /// Show the menu anchored to a button/element
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        theme: &Theme,
        anchor_rect: Rect,
        is_open: &mut bool,
    ) -> MenuResponse {
        let mut response = MenuResponse {
            selected: None,
            clicked_outside: false,
        };

        // Handle keyboard navigation
        if *is_open {
            ctx.input(|i| {
                if i.key_pressed(Key::ArrowDown) {
                    self.navigate_down();
                } else if i.key_pressed(Key::ArrowUp) {
                    self.navigate_up();
                } else if i.key_pressed(Key::Enter) {
                    if let Some(idx) = self.selected_index {
                        if idx < self.items.len()
                            && !self.items[idx].disabled
                            && !self.items[idx].is_separator
                        {
                            response.selected = Some(idx);
                            *is_open = false;
                            self.selected_index = None;
                        }
                    }
                } else if i.key_pressed(Key::Escape) {
                    *is_open = false;
                    self.selected_index = None;
                }
            });
        } else {
            self.selected_index = None;
        }

        let mut should_close = false;
        let popover_response = self.popover.show(ctx, theme, anchor_rect, is_open, |ui| {
            ui.spacing_mut().item_spacing = vec2(0.0, 2.0);

            for (idx, item) in self.items.iter().enumerate() {
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
                ui.allocate_ui_at_rect(rect, |ui| {
                    HStack::new(8.0).show(ui, |ui| {
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
                                Badge::new(badge_text).color(color).show(ui, theme);
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
            *is_open = false;
        }

        if popover_response.clicked_outside {
            response.clicked_outside = true;
        }

        response
    }

    fn navigate_down(&mut self) {
        let start_idx = self.selected_index.map(|i| i + 1).unwrap_or(0);

        for i in start_idx..self.items.len() {
            if !self.items[i].is_separator && !self.items[i].disabled {
                self.selected_index = Some(i);
                return;
            }
        }

        // Wrap around
        for i in 0..start_idx {
            if !self.items[i].is_separator && !self.items[i].disabled {
                self.selected_index = Some(i);
                return;
            }
        }
    }

    fn navigate_up(&mut self) {
        let start_idx = self.selected_index.unwrap_or(self.items.len()) as isize - 1;

        for i in (0..=start_idx).rev() {
            let idx = i as usize;
            if idx < self.items.len() && !self.items[idx].is_separator && !self.items[idx].disabled
            {
                self.selected_index = Some(idx);
                return;
            }
        }

        // Wrap around
        for i in (start_idx + 1..self.items.len() as isize).rev() {
            let idx = i as usize;
            if !self.items[idx].is_separator && !self.items[idx].disabled {
                self.selected_index = Some(idx);
                return;
            }
        }
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
