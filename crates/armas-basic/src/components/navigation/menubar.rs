//! Menubar Component (shadcn/ui style)
//!
//! A horizontal menu bar (File, Edit, View, Help) composing [`DropdownMenu`] components.
//! Supports hover-to-switch between open menus.
//!
//! ```rust,no_run
//! # use egui::Ui;
//! # fn example(ui: &mut Ui) {
//! use armas_basic::prelude::*;
//!
//! Menubar::new("app_menu").show(ui, |bar| {
//!     bar.menu("File", |menu| {
//!         menu.item("New");
//!         menu.item("Open");
//!         menu.separator();
//!         menu.item("Exit");
//!     });
//!     bar.menu("Edit", |menu| {
//!         menu.item("Undo").shortcut("⌘Z");
//!         menu.item("Redo").shortcut("⇧⌘Z");
//!     });
//! });
//! # }
//! ```

use super::dropdown_menu::{DropdownMenu, MenuBuilder};
use egui::{Id, Rect, Sense, Stroke, Ui};

// Constants matching shadcn menubar
const TRIGGER_PADDING_X: f32 = 12.0; // px-3
const TRIGGER_PADDING_Y: f32 = 6.0; // py-1.5
const TRIGGER_TEXT_SIZE: f32 = 14.0; // text-sm
const TRIGGER_RADIUS: f32 = 4.0; // rounded-sm
const BAR_HEIGHT: f32 = 40.0; // h-10
const BAR_RADIUS: f32 = 6.0; // rounded-md
const BAR_PADDING: f32 = 4.0; // p-1

/// A menu entry definition (label + content builder)
struct MenuEntry {
    label: String,
    items: Vec<super::dropdown_menu::MenuItemData>,
}

/// Builder for adding menus to a menubar.
pub struct MenubarBuilder {
    entries: Vec<MenuEntry>,
}

impl MenubarBuilder {
    const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Add a menu with a trigger label and dropdown content.
    pub fn menu(&mut self, label: impl Into<String>, content: impl FnOnce(&mut MenuBuilder)) {
        let mut builder = MenuBuilder::new();
        content(&mut builder);
        self.entries.push(MenuEntry {
            label: label.into(),
            items: builder.into_items(),
        });
    }
}

/// Menubar — a horizontal menu bar composing [`DropdownMenu`] components.
pub struct Menubar {
    id: Id,
}

/// Response from a menubar.
pub struct MenubarResponse {
    /// The UI response for the bar itself.
    pub response: egui::Response,
}

impl Menubar {
    /// Create a new menubar with a unique ID.
    pub fn new(id: impl Into<Id>) -> Self {
        Self { id: id.into() }
    }

    /// Show the menubar.
    pub fn show(self, ui: &mut Ui, menus: impl FnOnce(&mut MenubarBuilder)) -> MenubarResponse {
        let theme = crate::ext::ArmasContextExt::armas_theme(ui.ctx());

        // Build menu entries
        let mut builder = MenubarBuilder::new();
        menus(&mut builder);
        let entries = builder.entries;

        // Load persisted state
        let state_id = self.id.with("menubar_active");
        let mut active_menu: Option<usize> =
            ui.ctx().data_mut(|d| d.get_temp(state_id).unwrap_or(None));

        // Draw the bar background
        let (bar_rect, bar_response) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), BAR_HEIGHT), Sense::hover());

        // Bar background: border rounded-md bg-background
        ui.painter()
            .rect_filled(bar_rect, BAR_RADIUS, theme.background());
        ui.painter().rect_stroke(
            bar_rect,
            BAR_RADIUS,
            Stroke::new(1.0, theme.border()),
            egui::epaint::StrokeKind::Inside,
        );

        // Draw trigger buttons and collect their rects
        let mut trigger_rects: Vec<Rect> = Vec::new();
        let mut x = bar_rect.left() + BAR_PADDING;
        let trigger_y =
            bar_rect.top() + (BAR_HEIGHT - TRIGGER_PADDING_Y * 2.0 - TRIGGER_TEXT_SIZE) / 2.0;

        for (i, entry) in entries.iter().enumerate() {
            let galley = ui.painter().layout_no_wrap(
                entry.label.clone(),
                egui::FontId::proportional(TRIGGER_TEXT_SIZE),
                theme.foreground(),
            );
            let text_width = galley.size().x;
            let trigger_width = text_width + TRIGGER_PADDING_X * 2.0;
            let trigger_height = TRIGGER_TEXT_SIZE + TRIGGER_PADDING_Y * 2.0;

            let trigger_rect = Rect::from_min_size(
                egui::pos2(x, trigger_y),
                egui::vec2(trigger_width, trigger_height),
            );

            let trigger_response = ui.interact(trigger_rect, self.id.with(i), Sense::click());
            let is_active = active_menu == Some(i);
            let is_hovered = trigger_response.hovered();

            // Background: active or hovered
            if is_active || is_hovered {
                ui.painter()
                    .rect_filled(trigger_rect, TRIGGER_RADIUS, theme.accent());
            }

            // Text color
            let text_color = if is_active || is_hovered {
                theme.accent_foreground()
            } else {
                theme.foreground()
            };

            ui.painter().text(
                trigger_rect.center(),
                egui::Align2::CENTER_CENTER,
                &entry.label,
                egui::FontId::proportional(TRIGGER_TEXT_SIZE),
                text_color,
            );

            // Click to toggle
            if trigger_response.clicked() {
                if is_active {
                    active_menu = None;
                } else {
                    active_menu = Some(i);
                }
            }

            // Hover-to-switch: when a menu is already open and user hovers another trigger
            if active_menu.is_some() && !is_active && is_hovered {
                active_menu = Some(i);
            }

            trigger_rects.push(trigger_rect);
            x += trigger_width + 2.0; // small gap between triggers
        }

        // Show the active dropdown menu
        if let Some(active_idx) = active_menu {
            if active_idx < entries.len() {
                let anchor = trigger_rects[active_idx];
                let mut dropdown =
                    DropdownMenu::new(self.id.with(("dropdown", active_idx))).open(true);

                // Rebuild MenuBuilder items for the dropdown
                let entry_items = &entries[active_idx].items;
                let response = dropdown.show(ui.ctx(), anchor, |menu| {
                    replay_items(menu, entry_items);
                });

                // Close on selection or click outside
                if response.selected.is_some() || response.clicked_outside {
                    active_menu = None;
                }
            }
        }

        // Save state
        ui.ctx().data_mut(|d| d.insert_temp(state_id, active_menu));

        MenubarResponse {
            response: bar_response,
        }
    }
}

/// Replay pre-built menu items into a `MenuBuilder`.
fn replay_items(menu: &mut MenuBuilder, items: &[super::dropdown_menu::MenuItemData]) {
    for item_data in items {
        menu.push_item(item_data.clone());
    }
}
