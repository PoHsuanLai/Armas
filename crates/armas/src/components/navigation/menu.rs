//! Menu Component (shadcn/ui style)
//!
//! Dropdown and context menus with keyboard navigation, checkbox items,
//! radio groups, submenus, and destructive variants.
//!
//! Styled to match shadcn/ui dropdown-menu:
//! - Content: bg-popover text-popover-foreground border rounded-md p-1 shadow-md
//! - Item: px-2 py-1.5 text-sm rounded-sm gap-2
//! - Item hover: focus:bg-accent focus:text-accent-foreground
//! - Destructive: text-destructive focus:bg-destructive/10
//! - Disabled: opacity-50
//! - Shortcut: text-muted-foreground text-xs ml-auto tracking-widest
//! - Separator: bg-border h-px -mx-1 my-1
//! - Label: px-2 py-1.5 text-sm font-medium

use crate::components::basic::Kbd;
use crate::{Popover, PopoverPosition, PopoverStyle};
use egui::{vec2, Color32, Id, Key, Rect, Sense, Ui};

// ============================================================================
// Constants (matching shadcn Tailwind values)
// ============================================================================

// Content: min-w-[8rem] = 128px
const CONTENT_MIN_WIDTH: f32 = 128.0;

// Item: px-2 = 8px, py-1.5 = 6px, text-sm = 14px, gap-2 = 8px, rounded-sm = 2px
const ITEM_PADDING_X: f32 = 8.0;
const ITEM_HEIGHT: f32 = 26.0; // py-1.5 (6px) + text-sm (14px) + py-1.5 (6px) = 26px
const ITEM_GAP: f32 = 8.0;
const ITEM_RADIUS: f32 = 2.0;
const ITEM_TEXT_SIZE: f32 = 14.0;
const ITEM_ICON_SIZE: f32 = 16.0; // size-4 = 16px

// Inset: pl-8 = 32px
const ITEM_INSET_LEFT: f32 = 32.0;

// Checkbox/Radio indicator: left-2 = 8px, size-3.5 = 14px
const INDICATOR_LEFT: f32 = 8.0;
const INDICATOR_SIZE: f32 = 14.0;

// Separator: -mx-1 my-1 h-px
const SEPARATOR_MARGIN_X: f32 = -4.0;
const SEPARATOR_MARGIN_Y: f32 = 4.0;

// Submenu: chevron size
const CHEVRON_SIZE: f32 = 16.0;

// ============================================================================
// Menu Item Types
// ============================================================================

#[derive(Clone)]
enum MenuItemKind {
    Item { destructive: bool },
    Separator,
    Checkbox { checked: bool },
    Radio { group: String, value: String, selected: bool },
    Submenu { items: Vec<MenuItemData> },
}

#[derive(Clone)]
struct MenuItemData {
    label: String,
    icon: Option<String>,
    shortcut: Option<String>,
    disabled: bool,
    inset: bool,
    kind: MenuItemKind,
}

impl MenuItemData {
    fn is_selectable(&self) -> bool {
        !self.disabled && !matches!(self.kind, MenuItemKind::Separator)
    }
}

// ============================================================================
// Menu Builder
// ============================================================================

/// Builder for constructing menu contents
pub struct MenuBuilder {
    items: Vec<MenuItemData>,
}

impl MenuBuilder {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add a regular menu item
    pub fn item(&mut self, label: impl Into<String>) -> MenuItemBuilder<'_> {
        self.items.push(MenuItemData {
            label: label.into(),
            icon: None,
            shortcut: None,
            disabled: false,
            inset: false,
            kind: MenuItemKind::Item { destructive: false },
        });
        MenuItemBuilder { items: &mut self.items }
    }

    /// Add a separator line
    pub fn separator(&mut self) {
        self.items.push(MenuItemData {
            label: String::new(),
            icon: None,
            shortcut: None,
            disabled: false,
            inset: false,
            kind: MenuItemKind::Separator,
        });
    }

    /// Add a checkbox item
    pub fn checkbox(&mut self, label: impl Into<String>, checked: bool) -> MenuItemBuilder<'_> {
        self.items.push(MenuItemData {
            label: label.into(),
            icon: None,
            shortcut: None,
            disabled: false,
            inset: false,
            kind: MenuItemKind::Checkbox { checked },
        });
        MenuItemBuilder { items: &mut self.items }
    }

    /// Add a radio item
    pub fn radio(
        &mut self,
        label: impl Into<String>,
        group: impl Into<String>,
        value: impl Into<String>,
        selected: bool,
    ) -> MenuItemBuilder<'_> {
        self.items.push(MenuItemData {
            label: label.into(),
            icon: None,
            shortcut: None,
            disabled: false,
            inset: false,
            kind: MenuItemKind::Radio {
                group: group.into(),
                value: value.into(),
                selected,
            },
        });
        MenuItemBuilder { items: &mut self.items }
    }

    /// Add a submenu
    pub fn submenu(
        &mut self,
        label: impl Into<String>,
        content: impl FnOnce(&mut MenuBuilder),
    ) -> MenuItemBuilder<'_> {
        let mut sub_builder = MenuBuilder::new();
        content(&mut sub_builder);

        self.items.push(MenuItemData {
            label: label.into(),
            icon: None,
            shortcut: None,
            disabled: false,
            inset: false,
            kind: MenuItemKind::Submenu { items: sub_builder.items },
        });
        MenuItemBuilder { items: &mut self.items }
    }
}

// ============================================================================
// Menu Item Builder
// ============================================================================

/// Builder for chaining menu item modifiers
pub struct MenuItemBuilder<'a> {
    items: &'a mut Vec<MenuItemData>,
}

impl MenuItemBuilder<'_> {
    fn current(&mut self) -> Option<&mut MenuItemData> {
        self.items.last_mut()
    }

    /// Set an icon (emoji or symbol)
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        if let Some(item) = self.current() {
            item.icon = Some(icon.into());
        }
        self
    }

    /// Set a keyboard shortcut display string
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        if let Some(item) = self.current() {
            item.shortcut = Some(shortcut.into());
        }
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        if let Some(item) = self.current() {
            item.disabled = disabled;
        }
        self
    }

    /// Set inset (extra left padding for alignment with icon items)
    pub fn inset(mut self) -> Self {
        if let Some(item) = self.current() {
            item.inset = true;
        }
        self
    }

    /// Make this a destructive item (red text, for delete actions)
    pub fn destructive(mut self) -> Self {
        if let Some(item) = self.current() {
            if let MenuItemKind::Item { destructive } = &mut item.kind {
                *destructive = true;
            }
        }
        self
    }
}

// ============================================================================
// Menu Response
// ============================================================================

/// Response from showing a menu
#[derive(Debug, Clone, Default)]
pub struct MenuResponse {
    /// Index of selected/clicked item (if any)
    pub selected: Option<usize>,
    /// Whether the user clicked outside the menu
    pub clicked_outside: bool,
    /// Checkbox that was toggled: (index, new_checked_state)
    pub checkbox_toggled: Option<(usize, bool)>,
    /// Radio item that was selected: (group_name, value)
    pub radio_selected: Option<(String, String)>,
    /// Whether the menu is currently open
    pub is_open: bool,
}

impl MenuResponse {
    /// Check if a specific item index was selected
    pub fn is_selected(&self, index: usize) -> bool {
        self.selected == Some(index)
    }
}

// ============================================================================
// Menu Component
// ============================================================================

/// Menu component for dropdown and context menus
#[derive(Clone)]
pub struct Menu {
    id: Id,
    popover: Popover,
    is_open: Option<bool>,
    width: f32,
}

impl Menu {
    /// Create a new menu
    pub fn new(id: impl Into<Id>) -> Self {
        let id = id.into();
        Self {
            id,
            popover: Popover::new(id.with("popover"))
                .position(PopoverPosition::Bottom)
                .style(PopoverStyle::Default) // shadcn: rounded-md border shadow-md
                .padding(4.0) // p-1 = 4px (shadcn)
                .show_arrow(false),
            is_open: None,
            width: 200.0,
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
        self.width = width.max(CONTENT_MIN_WIDTH);
        self
    }

    /// Show the menu anchored to a rect
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        anchor_rect: Rect,
        content: impl FnOnce(&mut MenuBuilder),
    ) -> MenuResponse {
        let theme = crate::ext::ArmasContextExt::armas_theme(ctx);

        // Build items using closure
        let mut builder = MenuBuilder::new();
        content(&mut builder);
        let items = builder.items;

        // Load state
        let (mut is_open, mut selected_index, mut open_submenu) = self.load_state(ctx);

        // Override with external control if set
        if let Some(external_open) = self.is_open {
            is_open = external_open;
        }

        // Handle keyboard navigation
        if is_open {
            self.handle_keyboard(ctx, &items, &mut is_open, &mut selected_index);
        } else {
            selected_index = None;
            open_submenu = None;
        }

        // Initialize response
        let mut response = MenuResponse {
            selected: None,
            clicked_outside: false,
            checkbox_toggled: None,
            radio_selected: None,
            is_open,
        };

        // Set popover open state and width
        self.popover.set_open(is_open);
        self.popover = self.popover.clone().width(self.width);

        let menu_id = self.id;
        let menu_width = self.width;
        let popover_response = self.popover.show(ctx, &theme, anchor_rect, |ui| {
            ui.spacing_mut().item_spacing = vec2(0.0, 1.0);

            render_items(
                ui,
                &theme,
                menu_id,
                menu_width,
                &items,
                &mut selected_index,
                &mut open_submenu,
                &mut response,
            );
        });

        if popover_response.clicked_outside {
            response.clicked_outside = true;
            is_open = false;
        }

        // Update response with final open state
        response.is_open = is_open;

        // Save state
        self.save_state(ctx, is_open, selected_index, open_submenu);

        response
    }

    // ========================================================================
    // State Management
    // ========================================================================

    fn load_state(&self, ctx: &egui::Context) -> (bool, Option<usize>, Option<usize>) {
        let state_id = self.id.with("menu_state");
        let selected_id = self.id.with("selected_index");
        let submenu_id = self.id.with("open_submenu");

        let is_open = ctx.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
        let selected_index = ctx.data_mut(|d| d.get_temp(selected_id));
        let open_submenu = ctx.data_mut(|d| d.get_temp(submenu_id));

        (is_open, selected_index, open_submenu)
    }

    fn save_state(
        &self,
        ctx: &egui::Context,
        is_open: bool,
        selected_index: Option<usize>,
        open_submenu: Option<usize>,
    ) {
        ctx.data_mut(|d| {
            if self.is_open.is_none() {
                d.insert_temp(self.id.with("menu_state"), is_open);
            }
            d.insert_temp(self.id.with("selected_index"), selected_index);
            d.insert_temp(self.id.with("open_submenu"), open_submenu);
        });
    }

    // ========================================================================
    // Keyboard Navigation
    // ========================================================================

    fn handle_keyboard(
        &self,
        ctx: &egui::Context,
        items: &[MenuItemData],
        is_open: &mut bool,
        selected_index: &mut Option<usize>,
    ) {
        ctx.input(|i| {
            if i.key_pressed(Key::ArrowDown) {
                navigate_down(selected_index, items);
            } else if i.key_pressed(Key::ArrowUp) {
                navigate_up(selected_index, items);
            } else if i.key_pressed(Key::Enter) || i.key_pressed(Key::Space) {
                if let Some(idx) = *selected_index {
                    if idx < items.len() && items[idx].is_selectable() {
                        // Keep menu open for checkbox/radio, close for regular items
                        if !matches!(
                            items[idx].kind,
                            MenuItemKind::Checkbox { .. }
                                | MenuItemKind::Radio { .. }
                                | MenuItemKind::Submenu { .. }
                        ) {
                            *is_open = false;
                        }
                        *selected_index = None;
                    }
                }
            } else if i.key_pressed(Key::Escape) {
                *is_open = false;
                *selected_index = None;
            }
        });
    }

}

// ============================================================================
// Rendering Functions (free functions to avoid borrow issues)
// ============================================================================

fn render_items(
    ui: &mut Ui,
    theme: &crate::Theme,
    menu_id: Id,
    menu_width: f32,
    items: &[MenuItemData],
    selected_index: &mut Option<usize>,
    open_submenu: &mut Option<usize>,
    response: &mut MenuResponse,
) {
    for (idx, item) in items.iter().enumerate() {
        match &item.kind {
            MenuItemKind::Separator => {
                render_separator(ui, theme);
            }
            MenuItemKind::Item { destructive } => {
                if let Some(result) = render_item(
                    ui,
                    theme,
                    idx,
                    item,
                    *destructive,
                    selected_index,
                    ItemVariant::Normal,
                ) {
                    response.selected = Some(result);
                }
            }
            MenuItemKind::Checkbox { checked } => {
                if render_item(
                    ui,
                    theme,
                    idx,
                    item,
                    false,
                    selected_index,
                    ItemVariant::Checkbox(*checked),
                ).is_some() {
                    response.selected = Some(idx);
                    response.checkbox_toggled = Some((idx, !checked));
                }
            }
            MenuItemKind::Radio { group, value, selected } => {
                if render_item(
                    ui,
                    theme,
                    idx,
                    item,
                    false,
                    selected_index,
                    ItemVariant::Radio(*selected),
                ).is_some() {
                    response.selected = Some(idx);
                    response.radio_selected = Some((group.clone(), value.clone()));
                }
            }
            MenuItemKind::Submenu { items: sub_items } => {
                render_submenu(
                    ui,
                    theme,
                    menu_id,
                    menu_width,
                    idx,
                    item,
                    sub_items,
                    selected_index,
                    open_submenu,
                    response,
                );
            }
        }
    }
}

fn render_separator(ui: &mut Ui, theme: &crate::Theme) {
    ui.add_space(SEPARATOR_MARGIN_Y);
    let rect = ui.allocate_space(vec2(ui.available_width(), 1.0)).1;
    // Extend separator to edges (-mx-1)
    let extended_rect = Rect::from_min_max(
        rect.min + vec2(SEPARATOR_MARGIN_X, 0.0),
        rect.max - vec2(SEPARATOR_MARGIN_X, 0.0),
    );
    ui.painter().rect_filled(extended_rect, 0.0, theme.border());
    ui.add_space(SEPARATOR_MARGIN_Y);
}

fn render_item(
    ui: &mut Ui,
    theme: &crate::Theme,
    idx: usize,
    item: &MenuItemData,
    destructive: bool,
    selected_index: &mut Option<usize>,
    variant: ItemVariant,
) -> Option<usize> {
    let is_selected = *selected_index == Some(idx);
    let has_indicator = matches!(variant, ItemVariant::Checkbox(_) | ItemVariant::Radio(_));

    let (rect, item_response) = ui.allocate_exact_size(
        vec2(ui.available_width(), ITEM_HEIGHT),
        if item.disabled { Sense::hover() } else { Sense::click() },
    );

    // Update hover state
    if item_response.hovered() && !item.disabled {
        *selected_index = Some(idx);
    }

    // Render background
    render_item_background(ui, theme, rect, is_selected || item_response.hovered(), destructive, item.disabled);

    // Render content
    render_item_content(
        ui,
        theme,
        rect,
        item,
        destructive,
        is_selected || item_response.hovered(),
        has_indicator,
        variant,
    );

    if item_response.clicked() && !item.disabled {
        Some(idx)
    } else {
        None
    }
}

fn render_item_background(
    ui: &mut Ui,
    theme: &crate::Theme,
    rect: Rect,
    highlighted: bool,
    destructive: bool,
    disabled: bool,
) {
    if highlighted && !disabled {
        let bg_color = if destructive {
            // destructive/10 = 10% opacity
            Color32::from_rgba_unmultiplied(
                theme.colors.destructive[0],
                theme.colors.destructive[1],
                theme.colors.destructive[2],
                25,
            )
        } else {
            theme.accent()
        };
        ui.painter().rect_filled(rect, ITEM_RADIUS, bg_color);
    }
}

fn render_item_content(
    ui: &mut Ui,
    theme: &crate::Theme,
    rect: Rect,
    item: &MenuItemData,
    destructive: bool,
    highlighted: bool,
    has_indicator: bool,
    variant: ItemVariant,
) {
    let (text_color, icon_color) = get_item_colors(theme, destructive, highlighted, item.disabled);

    let mut x = rect.left();

    // Left padding
    x += if has_indicator || item.inset {
        ITEM_INSET_LEFT
    } else {
        ITEM_PADDING_X
    };

    // Checkbox/Radio indicator
    if let Some(checked) = variant.is_checked() {
        if checked {
            render_indicator(ui, theme, rect, matches!(variant, ItemVariant::Checkbox(_)));
        }
    }

    // Icon
    if let Some(icon) = &item.icon {
        ui.painter().text(
            egui::pos2(x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            icon,
            egui::FontId::proportional(ITEM_ICON_SIZE),
            icon_color,
        );
        x += ITEM_ICON_SIZE + ITEM_GAP;
    }

    // Label
    ui.painter().text(
        egui::pos2(x, rect.center().y),
        egui::Align2::LEFT_CENTER,
        &item.label,
        egui::FontId::proportional(ITEM_TEXT_SIZE),
        text_color,
    );

    // Shortcut (right-aligned)
    if let Some(shortcut) = &item.shortcut {
        let shortcut_rect = Rect::from_min_max(
            egui::pos2(rect.right() - 80.0, rect.top()),
            egui::pos2(rect.right() - ITEM_PADDING_X, rect.bottom()),
        );
        ui.scope_builder(egui::UiBuilder::new().max_rect(shortcut_rect), |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                Kbd::new(shortcut).show(ui);
            });
        });
    }
}

fn render_indicator(ui: &mut Ui, theme: &crate::Theme, rect: Rect, is_checkbox: bool) {
    let indicator_pos = rect.left_center() + vec2(INDICATOR_LEFT + INDICATOR_SIZE / 2.0, 0.0);

    if is_checkbox {
        // Checkmark
        ui.painter().text(
            indicator_pos,
            egui::Align2::CENTER_CENTER,
            "✓",
            egui::FontId::proportional(INDICATOR_SIZE),
            theme.foreground(),
        );
    } else {
        // Radio dot (filled circle)
        ui.painter().circle_filled(indicator_pos, 3.0, theme.foreground());
    }
}

fn render_submenu(
    ui: &mut Ui,
    theme: &crate::Theme,
    menu_id: Id,
    menu_width: f32,
    idx: usize,
    item: &MenuItemData,
    sub_items: &[MenuItemData],
    selected_index: &mut Option<usize>,
    open_submenu: &mut Option<usize>,
    response: &mut MenuResponse,
) {
    let is_selected = *selected_index == Some(idx);

    let (rect, item_response) = ui.allocate_exact_size(
        vec2(ui.available_width(), ITEM_HEIGHT),
        if item.disabled { Sense::hover() } else { Sense::click() },
    );

    // Check if mouse is currently hovering over the submenu content area
    // We store the submenu rect in memory to check against
    let submenu_rect_id = menu_id.with(("submenu_rect", idx));
    let submenu_rect: Option<Rect> = ui.ctx().data_mut(|d| d.get_temp(submenu_rect_id));

    let mouse_pos = ui.ctx().input(|i| i.pointer.hover_pos());

    // Check if mouse is in submenu
    let mouse_in_submenu = mouse_pos
        .zip(submenu_rect)
        .map(|(pos, r)| r.contains(pos))
        .unwrap_or(false);

    // Check if mouse is in the "bridge" zone between trigger and submenu
    // This is the gap area to the right of the trigger, at the same vertical level
    let mouse_in_bridge = mouse_pos
        .zip(submenu_rect)
        .map(|(pos, sub_rect)| {
            // Bridge zone: from right edge of trigger to left edge of submenu,
            // vertically spanning from trigger top to submenu bottom (or vice versa)
            let bridge_left = rect.right();
            let bridge_right = sub_rect.left();
            let bridge_top = rect.top().min(sub_rect.top());
            let bridge_bottom = rect.bottom().max(sub_rect.bottom());

            pos.x >= bridge_left && pos.x <= bridge_right &&
            pos.y >= bridge_top && pos.y <= bridge_bottom
        })
        .unwrap_or(false);

    // Keep submenu open if:
    // 1. Hovering over the trigger item, OR
    // 2. Mouse is inside the submenu content area, OR
    // 3. Mouse is in the bridge zone between trigger and submenu
    let should_keep_open = item_response.hovered() || mouse_in_submenu || mouse_in_bridge;

    if item_response.hovered() && !item.disabled {
        *selected_index = Some(idx);
        *open_submenu = Some(idx);
    } else if *open_submenu == Some(idx) && !should_keep_open {
        // Close submenu if mouse left trigger, submenu, and bridge zone
        *open_submenu = None;
    }

    // Render background
    let highlighted = is_selected || item_response.hovered() || *open_submenu == Some(idx);
    render_item_background(ui, theme, rect, highlighted, false, item.disabled);

    // Render content (label + chevron)
    let (text_color, icon_color) = get_item_colors(theme, false, highlighted, item.disabled);

    let mut x = rect.left() + ITEM_PADDING_X;

    // Icon
    if let Some(icon) = &item.icon {
        ui.painter().text(
            egui::pos2(x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            icon,
            egui::FontId::proportional(ITEM_ICON_SIZE),
            icon_color,
        );
        x += ITEM_ICON_SIZE + ITEM_GAP;
    }

    // Label
    ui.painter().text(
        egui::pos2(x, rect.center().y),
        egui::Align2::LEFT_CENTER,
        &item.label,
        egui::FontId::proportional(ITEM_TEXT_SIZE),
        text_color,
    );

    // Chevron (right arrow)
    ui.painter().text(
        egui::pos2(rect.right() - ITEM_PADDING_X - CHEVRON_SIZE / 2.0, rect.center().y),
        egui::Align2::CENTER_CENTER,
        "›",
        egui::FontId::proportional(CHEVRON_SIZE),
        icon_color,
    );

    // Show submenu if open
    if *open_submenu == Some(idx) && !item.disabled {
        // Position submenu to the right of the item
        let submenu_anchor = Rect::from_min_size(
            egui::pos2(rect.right(), rect.top()),
            vec2(0.0, rect.height()),
        );

        let submenu_id = menu_id.with(("submenu", idx));
        let mut submenu = Menu::new(submenu_id)
            .position(PopoverPosition::Right)
            .width(menu_width)
            .open(true);

        let sub_response = submenu.show(ui.ctx(), submenu_anchor, |builder| {
            // Copy items into the builder
            for sub_item in sub_items {
                add_item_to_builder(builder, sub_item, menu_id, menu_width);
            }
        });

        // Store the submenu's actual rendered rect for hover detection next frame
        // We need to get the rect from egui's Area system
        let area_id = submenu_id.with("popover");
        if let Some(area_state) = ui.ctx().memory(|m| m.area_rect(area_id)) {
            // Expand the rect slightly to create a buffer zone for mouse movement
            let expanded_rect = area_state.expand(4.0);
            ui.ctx().data_mut(|d| d.insert_temp(submenu_rect_id, expanded_rect));
        }

        // Propagate submenu responses
        if sub_response.selected.is_some() {
            response.selected = sub_response.selected;
        }
        if sub_response.checkbox_toggled.is_some() {
            response.checkbox_toggled = sub_response.checkbox_toggled;
        }
        if sub_response.radio_selected.is_some() {
            response.radio_selected = sub_response.radio_selected;
        }
    } else {
        // Clear the stored rect when submenu is closed
        ui.ctx().data_mut(|d| d.remove::<Rect>(submenu_rect_id));
    }
}

fn add_item_to_builder(builder: &mut MenuBuilder, item: &MenuItemData, menu_id: Id, menu_width: f32) {
    match &item.kind {
        MenuItemKind::Separator => builder.separator(),
        MenuItemKind::Item { destructive } => {
            let mut b = builder.item(&item.label);
            if let Some(icon) = &item.icon {
                b = b.icon(icon);
            }
            if let Some(shortcut) = &item.shortcut {
                b = b.shortcut(shortcut);
            }
            if item.disabled {
                b = b.disabled(true);
            }
            if item.inset {
                b = b.inset();
            }
            if *destructive {
                b.destructive();
            }
        }
        MenuItemKind::Checkbox { checked } => {
            let mut b = builder.checkbox(&item.label, *checked);
            if let Some(icon) = &item.icon {
                b = b.icon(icon);
            }
            if item.disabled {
                b.disabled(true);
            }
        }
        MenuItemKind::Radio { group, value, selected } => {
            let mut b = builder.radio(&item.label, group, value, *selected);
            if let Some(icon) = &item.icon {
                b = b.icon(icon);
            }
            if item.disabled {
                b.disabled(true);
            }
        }
        MenuItemKind::Submenu { items } => {
            let items_clone = items.clone();
            let mut b = builder.submenu(&item.label, |sub| {
                for sub_item in &items_clone {
                    add_item_to_builder(sub, sub_item, menu_id, menu_width);
                }
            });
            if let Some(icon) = &item.icon {
                b = b.icon(icon);
            }
            if item.disabled {
                b.disabled(true);
            }
        }
    }
}

fn get_item_colors(
    theme: &crate::Theme,
    destructive: bool,
    highlighted: bool,
    disabled: bool,
) -> (Color32, Color32) {
    let text_color = if disabled {
        theme.foreground().linear_multiply(0.5)
    } else if destructive {
        theme.destructive()
    } else if highlighted {
        theme.accent_foreground()
    } else {
        theme.foreground()
    };

    let icon_color = if disabled {
        theme.muted_foreground().linear_multiply(0.5)
    } else if destructive {
        theme.destructive()
    } else {
        theme.muted_foreground()
    };

    (text_color, icon_color)
}

// ============================================================================
// Item Variant Helper
// ============================================================================

#[derive(Clone, Copy)]
enum ItemVariant {
    Normal,
    Checkbox(bool),
    Radio(bool),
}

impl ItemVariant {
    fn is_checked(&self) -> Option<bool> {
        match self {
            ItemVariant::Normal => None,
            ItemVariant::Checkbox(checked) | ItemVariant::Radio(checked) => Some(*checked),
        }
    }
}

// ============================================================================
// Navigation Helpers
// ============================================================================

fn navigate_down(selected_index: &mut Option<usize>, items: &[MenuItemData]) {
    let start_idx = selected_index.map(|i| i + 1).unwrap_or(0);

    for (i, item) in items.iter().enumerate().skip(start_idx) {
        if item.is_selectable() {
            *selected_index = Some(i);
            return;
        }
    }

    // Wrap around
    for (i, item) in items.iter().enumerate().take(start_idx) {
        if item.is_selectable() {
            *selected_index = Some(i);
            return;
        }
    }
}

fn navigate_up(selected_index: &mut Option<usize>, items: &[MenuItemData]) {
    let start_idx = selected_index.unwrap_or(items.len()) as isize - 1;

    for i in (0..=start_idx).rev() {
        let idx = i as usize;
        if idx < items.len() && items[idx].is_selectable() {
            *selected_index = Some(idx);
            return;
        }
    }

    // Wrap around
    for i in (start_idx + 1..items.len() as isize).rev() {
        let idx = i as usize;
        if items[idx].is_selectable() {
            *selected_index = Some(idx);
            return;
        }
    }
}

// ============================================================================
// Convenience: MenuItem struct for pre-built items
// ============================================================================

/// Pre-built menu item (alternative to builder pattern)
#[derive(Clone)]
pub struct MenuItem {
    pub label: String,
    pub icon: Option<String>,
    pub shortcut: Option<String>,
    pub disabled: bool,
    pub destructive: bool,
}

impl MenuItem {
    /// Create a new menu item
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            shortcut: None,
            disabled: false,
            destructive: false,
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

    /// Make this a destructive item
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
}
