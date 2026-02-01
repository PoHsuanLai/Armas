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
use crate::icon::{render_icon, WindowIcon};
use crate::{Popover, PopoverPosition, PopoverStyle};
use egui::{vec2, Color32, Id, Key, Rect, Sense, Ui};
use std::collections::HashSet;

// ============================================================================
// Submenu State (persisted in egui temp storage)
// ============================================================================

/// Tracks which submenus are currently open for a menu
#[derive(Clone, Default)]
struct SubmenuState {
    /// Set of open submenu indices
    open: HashSet<usize>,
}

impl SubmenuState {
    fn load(ctx: &egui::Context, menu_id: Id) -> Self {
        ctx.data_mut(|d| {
            d.get_temp(menu_id.with("submenu_state"))
                .unwrap_or_default()
        })
    }

    fn save(&self, ctx: &egui::Context, menu_id: Id) {
        ctx.data_mut(|d| d.insert_temp(menu_id.with("submenu_state"), self.clone()));
    }

    fn is_open(&self, idx: usize) -> bool {
        self.open.contains(&idx)
    }

    fn open_submenu(&mut self, idx: usize) {
        // Close all others and open this one
        self.open.clear();
        self.open.insert(idx);
    }

    fn close_all(&mut self) {
        self.open.clear();
    }
}

/// Context for rendering menu items (groups common parameters)
struct MenuRenderContext<'a> {
    ui: &'a mut Ui,
    theme: &'a crate::Theme,
    menu_id: Id,
    menu_width: f32,
}

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
    Item {
        destructive: bool,
    },
    Separator,
    Checkbox {
        checked: bool,
    },
    Radio {
        group: String,
        value: String,
        selected: bool,
    },
    Submenu {
        items: Vec<MenuItemData>,
    },
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
    const fn is_selectable(&self) -> bool {
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
    const fn new() -> Self {
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
        MenuItemBuilder {
            items: &mut self.items,
        }
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
        MenuItemBuilder {
            items: &mut self.items,
        }
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
        MenuItemBuilder {
            items: &mut self.items,
        }
    }

    /// Add a submenu
    pub fn submenu(
        &mut self,
        label: impl Into<String>,
        content: impl FnOnce(&mut Self),
    ) -> MenuItemBuilder<'_> {
        let mut sub_builder = Self::new();
        content(&mut sub_builder);

        self.items.push(MenuItemData {
            label: label.into(),
            icon: None,
            shortcut: None,
            disabled: false,
            inset: false,
            kind: MenuItemKind::Submenu {
                items: sub_builder.items,
            },
        });
        MenuItemBuilder {
            items: &mut self.items,
        }
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
    #[must_use]
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        if let Some(item) = self.current() {
            item.icon = Some(icon.into());
        }
        self
    }

    /// Set a keyboard shortcut display string
    #[must_use]
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        if let Some(item) = self.current() {
            item.shortcut = Some(shortcut.into());
        }
        self
    }

    /// Set disabled state
    #[must_use] 
    pub fn disabled(mut self, disabled: bool) -> Self {
        if let Some(item) = self.current() {
            item.disabled = disabled;
        }
        self
    }

    /// Set inset (extra left padding for alignment with icon items)
    #[must_use] 
    pub fn inset(mut self) -> Self {
        if let Some(item) = self.current() {
            item.inset = true;
        }
        self
    }

    /// Make this a destructive item (red text, for delete actions)
    #[must_use] 
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
    /// Checkbox that was toggled: (index, `new_checked_state`)
    pub checkbox_toggled: Option<(usize, bool)>,
    /// Radio item that was selected: (`group_name`, value)
    pub radio_selected: Option<(String, String)>,
    /// Whether the menu is currently open
    pub is_open: bool,
}

impl MenuResponse {
    /// Check if a specific item index was selected
    #[must_use] 
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
                .padding(4.0), // p-1 = 4px (shadcn)
            is_open: None,
            width: 200.0,
        }
    }

    /// Set the menu to be open (for external control)
    #[must_use] 
    pub const fn open(mut self, is_open: bool) -> Self {
        self.is_open = Some(is_open);
        self
    }

    /// Set the menu position
    #[must_use] 
    pub const fn position(mut self, position: PopoverPosition) -> Self {
        self.popover = self.popover.position(position);
        self
    }

    /// Set the menu width
    #[must_use] 
    pub const fn width(mut self, width: f32) -> Self {
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
        let (mut is_open, mut selected_index) = self.load_state(ctx);
        let mut submenu_state = SubmenuState::load(ctx, self.id);

        // Override with external control if set
        if let Some(external_open) = self.is_open {
            is_open = external_open;
        }

        // Handle keyboard navigation (only when open)
        if is_open {
            self.handle_keyboard(ctx, &items, &mut is_open, &mut selected_index);
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

            let mut ctx = MenuRenderContext {
                ui,
                theme: &theme,
                menu_id,
                menu_width,
            };
            render_items(
                &mut ctx,
                &items,
                &mut selected_index,
                &mut submenu_state,
                &mut response,
            );
        });

        if popover_response.clicked_outside {
            response.clicked_outside = true;
            is_open = false;
            submenu_state.close_all();
        }

        // Close submenus when an item is selected
        if response.selected.is_some() {
            submenu_state.close_all();
        }

        // Update response with final open state
        response.is_open = is_open;

        // Save state
        self.save_state(ctx, is_open, selected_index);
        submenu_state.save(ctx, self.id);

        response
    }

    // ========================================================================
    // State Management
    // ========================================================================

    fn load_state(&self, ctx: &egui::Context) -> (bool, Option<usize>) {
        let state_id = self.id.with("menu_state");
        let selected_id = self.id.with("selected_index");

        let is_open = ctx.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
        let selected_index = ctx.data_mut(|d| d.get_temp(selected_id));

        (is_open, selected_index)
    }

    fn save_state(&self, ctx: &egui::Context, is_open: bool, selected_index: Option<usize>) {
        ctx.data_mut(|d| {
            if self.is_open.is_none() {
                d.insert_temp(self.id.with("menu_state"), is_open);
            }
            d.insert_temp(self.id.with("selected_index"), selected_index);
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
    ctx: &mut MenuRenderContext,
    items: &[MenuItemData],
    selected_index: &mut Option<usize>,
    submenu_state: &mut SubmenuState,
    response: &mut MenuResponse,
) {
    for (idx, item) in items.iter().enumerate() {
        match &item.kind {
            MenuItemKind::Separator => {
                render_separator(ctx.ui, ctx.theme);
            }
            MenuItemKind::Item { destructive } => {
                let (result, _) = render_item_with_hover(
                    ctx.ui,
                    ctx.theme,
                    idx,
                    item,
                    *destructive,
                    selected_index,
                    ItemVariant::Normal,
                );
                if let Some(r) = result {
                    response.selected = Some(r);
                }
            }
            MenuItemKind::Checkbox { checked } => {
                let (result, _) = render_item_with_hover(
                    ctx.ui,
                    ctx.theme,
                    idx,
                    item,
                    false,
                    selected_index,
                    ItemVariant::Checkbox(*checked),
                );
                if result.is_some() {
                    response.selected = Some(idx);
                    response.checkbox_toggled = Some((idx, !checked));
                }
            }
            MenuItemKind::Radio {
                group,
                value,
                selected,
            } => {
                let (result, _) = render_item_with_hover(
                    ctx.ui,
                    ctx.theme,
                    idx,
                    item,
                    false,
                    selected_index,
                    ItemVariant::Radio(*selected),
                );
                if result.is_some() {
                    response.selected = Some(idx);
                    response.radio_selected = Some((group.clone(), value.clone()));
                }
            }
            MenuItemKind::Submenu { items: sub_items } => {
                let submenu_params = SubmenuParams {
                    idx,
                    item,
                    sub_items,
                };
                let render_params = RenderSubmenuParams {
                    menu_id: ctx.menu_id,
                    menu_width: ctx.menu_width,
                    submenu_params,
                    selected_index,
                    submenu_state,
                    response,
                };
                render_submenu(ctx.ui, ctx.theme, render_params);
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

/// Renders a menu item and returns (`clicked_index`, `is_hovered`)
fn render_item_with_hover(
    ui: &mut Ui,
    theme: &crate::Theme,
    idx: usize,
    item: &MenuItemData,
    destructive: bool,
    selected_index: &mut Option<usize>,
    variant: ItemVariant,
) -> (Option<usize>, bool) {
    let is_selected = *selected_index == Some(idx);
    let has_indicator = matches!(variant, ItemVariant::Checkbox(_) | ItemVariant::Radio(_));

    let (rect, item_response) = ui.allocate_exact_size(
        vec2(ui.available_width(), ITEM_HEIGHT),
        if item.disabled {
            Sense::hover()
        } else {
            Sense::click()
        },
    );

    let is_hovered = item_response.hovered() && !item.disabled;

    // Update hover state
    if is_hovered {
        *selected_index = Some(idx);
    }

    // Render background
    render_item_background(
        ui,
        theme,
        rect,
        is_selected || item_response.hovered(),
        destructive,
        item.disabled,
    );

    // Render content
    let params = ItemContentParams {
        rect,
        item,
        destructive,
        highlighted: is_selected || item_response.hovered(),
        has_indicator,
        variant,
    };
    render_item_content(ui, theme, &params);

    let clicked = if item_response.clicked() && !item.disabled {
        Some(idx)
    } else {
        None
    };

    (clicked, is_hovered)
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

/// Parameters for rendering item content
struct ItemContentParams<'a> {
    rect: Rect,
    item: &'a MenuItemData,
    destructive: bool,
    highlighted: bool,
    has_indicator: bool,
    variant: ItemVariant,
}

fn render_item_content(ui: &mut Ui, theme: &crate::Theme, params: &ItemContentParams) {
    let (text_color, icon_color) = get_item_colors(
        theme,
        params.destructive,
        params.highlighted,
        params.item.disabled,
    );

    let mut x = params.rect.left();

    // Left padding
    x += if params.has_indicator || params.item.inset {
        ITEM_INSET_LEFT
    } else {
        ITEM_PADDING_X
    };

    // Checkbox/Radio indicator
    if let Some(checked) = params.variant.is_checked() {
        if checked {
            render_indicator(
                ui,
                theme,
                params.rect,
                matches!(params.variant, ItemVariant::Checkbox(_)),
            );
        }
    }

    // Icon
    if let Some(icon) = &params.item.icon {
        ui.painter().text(
            egui::pos2(x, params.rect.center().y),
            egui::Align2::LEFT_CENTER,
            icon,
            egui::FontId::proportional(ITEM_ICON_SIZE),
            icon_color,
        );
        x += ITEM_ICON_SIZE + ITEM_GAP;
    }

    // Label
    ui.painter().text(
        egui::pos2(x, params.rect.center().y),
        egui::Align2::LEFT_CENTER,
        &params.item.label,
        egui::FontId::proportional(ITEM_TEXT_SIZE),
        text_color,
    );

    // Shortcut (right-aligned)
    if let Some(shortcut) = &params.item.shortcut {
        let shortcut_rect = Rect::from_min_max(
            egui::pos2(params.rect.right() - 80.0, params.rect.top()),
            egui::pos2(params.rect.right() - ITEM_PADDING_X, params.rect.bottom()),
        );
        ui.scope_builder(egui::UiBuilder::new().max_rect(shortcut_rect), |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                Kbd::new(shortcut).show(ui, theme);
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
        ui.painter()
            .circle_filled(indicator_pos, 3.0, theme.foreground());
    }
}

/// Parameters for rendering a submenu
struct SubmenuParams<'a> {
    idx: usize,
    item: &'a MenuItemData,
    sub_items: &'a [MenuItemData],
}

/// Parameters for `render_submenu` function
struct RenderSubmenuParams<'a> {
    menu_id: Id,
    menu_width: f32,
    submenu_params: SubmenuParams<'a>,
    selected_index: &'a mut Option<usize>,
    submenu_state: &'a mut SubmenuState,
    response: &'a mut MenuResponse,
}

#[allow(clippy::needless_pass_by_value)]
fn render_submenu(ui: &mut Ui, theme: &crate::Theme, params: RenderSubmenuParams) {
    let is_selected = *params.selected_index == Some(params.submenu_params.idx);
    let is_submenu_open = params.submenu_state.is_open(params.submenu_params.idx);

    let (rect, item_response) = ui.allocate_exact_size(
        vec2(ui.available_width(), ITEM_HEIGHT),
        if params.submenu_params.item.disabled {
            Sense::hover()
        } else {
            Sense::click()
        },
    );

    // Open submenu when hovering the trigger
    if item_response.hovered() && !params.submenu_params.item.disabled {
        *params.selected_index = Some(params.submenu_params.idx);
        params.submenu_state.open_submenu(params.submenu_params.idx);
    }

    // Render background
    let highlighted = is_selected || item_response.hovered() || is_submenu_open;
    render_item_background(
        ui,
        theme,
        rect,
        highlighted,
        false,
        params.submenu_params.item.disabled,
    );

    // Render content (label + chevron)
    let (text_color, icon_color) = get_item_colors(
        theme,
        false,
        highlighted,
        params.submenu_params.item.disabled,
    );

    let mut x = rect.left() + ITEM_PADDING_X;

    // Icon
    if let Some(icon) = &params.submenu_params.item.icon {
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
        &params.submenu_params.item.label,
        egui::FontId::proportional(ITEM_TEXT_SIZE),
        text_color,
    );

    // Chevron (right arrow)
    let chevron_rect = Rect::from_center_size(
        egui::pos2(
            rect.right() - ITEM_PADDING_X - CHEVRON_SIZE / 2.0,
            rect.center().y,
        ),
        vec2(CHEVRON_SIZE, CHEVRON_SIZE),
    );
    render_icon(
        ui.painter(),
        chevron_rect,
        WindowIcon::ChevronRight.data(),
        icon_color,
    );

    // Always render the submenu so it can animate closed
    // Position submenu to the right of the item
    let submenu_anchor = Rect::from_min_size(
        egui::pos2(rect.right(), rect.top()),
        vec2(0.0, rect.height()),
    );

    let submenu_id = params.menu_id.with(("submenu", params.submenu_params.idx));
    let submenu_should_be_open = params.submenu_state.is_open(params.submenu_params.idx)
        && !params.submenu_params.item.disabled;

    let mut submenu = Menu::new(submenu_id)
        .position(PopoverPosition::Right)
        .width(params.menu_width)
        .open(submenu_should_be_open);

    let sub_response = submenu.show(ui.ctx(), submenu_anchor, |builder| {
        for sub_item in params.submenu_params.sub_items {
            add_item_to_builder(builder, sub_item);
        }
    });

    // Propagate submenu responses
    if sub_response.selected.is_some() {
        params.response.selected = sub_response.selected;
    }
    if sub_response.checkbox_toggled.is_some() {
        params.response.checkbox_toggled = sub_response.checkbox_toggled;
    }
    if sub_response.radio_selected.is_some() {
        params.response.radio_selected = sub_response.radio_selected;
    }
}

fn add_item_to_builder(
    builder: &mut MenuBuilder,
    item: &MenuItemData,
) {
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
                let _ = b.destructive();
            }
        }
        MenuItemKind::Checkbox { checked } => {
            let mut b = builder.checkbox(&item.label, *checked);
            if let Some(icon) = &item.icon {
                b = b.icon(icon);
            }
            if item.disabled {
                let _ = b.disabled(true);
            }
        }
        MenuItemKind::Radio {
            group,
            value,
            selected,
        } => {
            let mut b = builder.radio(&item.label, group, value, *selected);
            if let Some(icon) = &item.icon {
                b = b.icon(icon);
            }
            if item.disabled {
                let _ = b.disabled(true);
            }
        }
        MenuItemKind::Submenu { items } => {
            let items_clone = items.clone();
            let mut b = builder.submenu(&item.label, |sub| {
                for sub_item in &items_clone {
                    add_item_to_builder(sub, sub_item);
                }
            });
            if let Some(icon) = &item.icon {
                b = b.icon(icon);
            }
            if item.disabled {
                let _ = b.disabled(true);
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
    const fn is_checked(self) -> Option<bool> {
        match self {
            Self::Normal => None,
            Self::Checkbox(checked) | Self::Radio(checked) => Some(checked),
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

#[allow(clippy::cast_possible_wrap)]
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
    /// Menu item label
    pub label: String,
    /// Optional icon
    pub icon: Option<String>,
    /// Optional keyboard shortcut text
    pub shortcut: Option<String>,
    /// Whether the item is disabled
    pub disabled: bool,
    /// Whether the item is destructive (e.g., delete action)
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
    #[must_use]
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set a keyboard shortcut
    #[must_use]
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Set disabled state
    #[must_use] 
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Make this a destructive item
    #[must_use] 
    pub const fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
}
