//! Context Menu Component (shadcn/ui style)
//!
//! Right-click context menus that reuse [`DropdownMenu`] internals.
//! Opens on secondary click (right-click) and anchors to the cursor position.
//!
//! ```rust,no_run
//! # use egui::Ui;
//! # fn example(ui: &mut Ui) {
//! use armas_basic::prelude::*;
//!
//! let response = ui.allocate_response(egui::vec2(200.0, 100.0), egui::Sense::click());
//! let mut ctx_menu = ContextMenu::new("my_context_menu");
//! ctx_menu.show(ui.ctx(), &response, |menu| {
//!     menu.item("Cut").shortcut("⌘X");
//!     menu.item("Copy").shortcut("⌘C");
//!     menu.item("Paste").shortcut("⌘V");
//!     menu.separator();
//!     menu.item("Delete").destructive();
//! });
//! # }
//! ```

use super::dropdown_menu::{DropdownMenu, DropdownMenuResponse, MenuBuilder};
use egui::{Id, Rect, Response};

/// Context menu response
pub struct ContextMenuResponse {
    /// The underlying dropdown menu response
    pub inner: DropdownMenuResponse,
}

impl std::ops::Deref for ContextMenuResponse {
    type Target = DropdownMenuResponse;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Context menu triggered by right-click on a region.
///
/// Wraps [`DropdownMenu`] with right-click trigger and cursor-position anchoring.
pub struct ContextMenu {
    id: Id,
    width: f32,
}

impl ContextMenu {
    /// Create a new context menu with the given ID.
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            width: 200.0,
        }
    }

    /// Set the menu width.
    #[must_use]
    pub const fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Show the context menu. Opens when `trigger` is right-clicked.
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        trigger: &Response,
        content: impl FnOnce(&mut MenuBuilder),
    ) -> ContextMenuResponse {
        let state_id = self.id.with("ctx_menu_state");
        let anchor_id = self.id.with("ctx_menu_anchor");

        // Load persisted state
        let mut is_open = ctx.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
        let mut anchor_rect =
            ctx.data_mut(|d| d.get_temp::<Rect>(anchor_id).unwrap_or(Rect::NOTHING));

        // Open on right-click (secondary click)
        if trigger.secondary_clicked() {
            is_open = true;
            // Anchor is a zero-size rect at the pointer position
            if let Some(pos) = ctx.input(|i| i.pointer.interact_pos()) {
                anchor_rect = Rect::from_min_size(pos, egui::vec2(0.0, 0.0));
            }
        }

        // Delegate to DropdownMenu
        let mut menu = DropdownMenu::new(self.id.with("dropdown"))
            .open(is_open)
            .width(self.width);

        let response = menu.show(ctx, anchor_rect, content);

        // Close on selection or click outside
        if response.clicked_outside || response.selected.is_some() {
            is_open = false;
        }

        // Save state
        ctx.data_mut(|d| {
            d.insert_temp(state_id, is_open);
            d.insert_temp(anchor_id, anchor_rect);
        });

        ContextMenuResponse { inner: response }
    }
}
