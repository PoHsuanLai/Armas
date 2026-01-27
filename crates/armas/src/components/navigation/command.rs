//! Command Component
//!
//! A command palette for search and quick actions. Styled to match shadcn/ui command.

use crate::animation::{Animation, EasingFunction};
use crate::components::basic::Kbd;
use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{vec2, Align2, Color32, Key, Modifiers, Pos2, Rect, Sense, Ui};

// Constants matching shadcn styling
const PANEL_WIDTH: f32 = 512.0;
const PANEL_MAX_HEIGHT: f32 = 384.0;
const CORNER_RADIUS: f32 = 6.0;
const INPUT_HEIGHT: f32 = 44.0;
const INPUT_PADDING_X: f32 = 12.0;
const INPUT_GAP: f32 = 8.0;
const INPUT_TEXT_SIZE: f32 = 14.0;
const LIST_MAX_HEIGHT: f32 = 300.0;
const LIST_PADDING: f32 = 4.0;
const GROUP_PADDING_X: f32 = 8.0;
const GROUP_PADDING_Y: f32 = 6.0;
const GROUP_HEADING_SIZE: f32 = 12.0;
const ITEM_HEIGHT: f32 = 32.0;
const ITEM_RADIUS: f32 = 2.0;
const ITEM_PADDING_X: f32 = 8.0;
const ITEM_GAP: f32 = 8.0;
const ITEM_TEXT_SIZE: f32 = 14.0;
const ICON_SIZE: f32 = 16.0;

/// Internal representation of a command item
#[derive(Clone)]
enum CommandItem {
    Action {
        id: String,
        label: String,
        icon: Option<String>,
        shortcut: Option<String>,
    },
    Group {
        heading: String,
    },
    Separator,
}

/// Builder for configuring individual commands
pub struct CommandItemBuilder<'a> {
    items: &'a mut Vec<CommandItem>,
    index: usize,
}

impl<'a> CommandItemBuilder<'a> {
    /// Set command icon
    pub fn icon(self, icon: impl Into<String>) -> Self {
        if let Some(CommandItem::Action {
            icon: ref mut i, ..
        }) = self.items.get_mut(self.index)
        {
            *i = Some(icon.into());
        }
        self
    }

    /// Set keyboard shortcut display (use format like "⌘+K" or "Ctrl+Shift+P")
    pub fn shortcut(self, shortcut: impl Into<String>) -> Self {
        if let Some(CommandItem::Action {
            shortcut: ref mut s,
            ..
        }) = self.items.get_mut(self.index)
        {
            *s = Some(shortcut.into());
        }
        self
    }
}

/// Builder for adding commands to the menu
pub struct CommandBuilder<'a> {
    items: &'a mut Vec<CommandItem>,
}

impl<'a> CommandBuilder<'a> {
    /// Add a command item
    pub fn item(&mut self, id: &str, label: &str) -> CommandItemBuilder<'_> {
        self.items.push(CommandItem::Action {
            id: id.to_string(),
            label: label.to_string(),
            icon: None,
            shortcut: None,
        });
        let index = self.items.len() - 1;
        CommandItemBuilder {
            items: self.items,
            index,
        }
    }

    /// Add a group heading
    pub fn group(&mut self, heading: &str) {
        self.items.push(CommandItem::Group {
            heading: heading.to_string(),
        });
    }

    /// Add a separator
    pub fn separator(&mut self) {
        self.items.push(CommandItem::Separator);
    }
}

/// Response from command
pub struct CommandResponse {
    /// ID of executed command, if any
    pub executed: Option<String>,
    /// Whether the command palette is currently open
    pub is_open: bool,
    /// Whether the open state changed this frame (opened or closed)
    pub changed: bool,
}

/// Command palette component
pub struct Command {
    id: Option<egui::Id>,
    placeholder: String,
    trigger_key: Key,
    trigger_modifiers: Modifiers,
    // Internal state
    is_open: bool,
    search: String,
    selected: usize,
    animation: Animation<f32>,
}

impl Command {
    /// Create a new command palette
    pub fn new() -> Self {
        Self {
            id: None,
            placeholder: "Type a command or search...".to_string(),
            trigger_key: Key::K,
            trigger_modifiers: Modifiers::COMMAND,
            is_open: false,
            search: String::new(),
            selected: 0,
            animation: Animation::new(0.0, 1.0, 0.15).easing(EasingFunction::CubicOut),
        }
    }

    /// Set a custom ID for state persistence
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set placeholder text for the search input
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the keyboard shortcut to trigger the command palette
    pub fn trigger(mut self, key: Key, modifiers: Modifiers) -> Self {
        self.trigger_key = key;
        self.trigger_modifiers = modifiers;
        self
    }

    /// Show the command palette
    pub fn show<R>(
        &mut self,
        ui: &mut Ui,
        content: impl FnOnce(&mut CommandBuilder) -> R,
    ) -> CommandResponse {
        let theme = ui.ctx().armas_theme();
        let ctx = ui.ctx().clone();
        let id = self.id.unwrap_or_else(|| ui.id().with("command"));

        // Load state
        self.load_state(&ctx, id);
        let was_open = self.is_open;

        // Collect items
        let mut items = Vec::new();
        let mut builder = CommandBuilder { items: &mut items };
        content(&mut builder);

        // Handle trigger key
        self.handle_trigger(&ctx);

        let mut executed = None;

        if self.is_open {
            // Update animation
            let dt = ctx.input(|i| i.unstable_dt);
            self.animation.update(dt);
            if self.animation.is_running() {
                ctx.request_repaint();
            }

            // Filter items
            let filtered = self.filter_items(&items);

            // Draw UI
            let mut should_close = false;
            self.draw_backdrop(ui, id, &mut should_close);
            executed = self.draw_panel(ui, id, &theme, &filtered, &mut should_close);

            // Handle keyboard
            self.handle_keyboard(&ctx, &filtered, &mut should_close, &mut executed);

            if should_close {
                self.is_open = false;
                self.search.clear();
                self.selected = 0;
                self.animation.reset();
            }
        }

        let changed = was_open != self.is_open;

        // Save state
        self.save_state(&ctx, id);

        CommandResponse {
            executed,
            is_open: self.is_open,
            changed,
        }
    }

    // ========================================================================
    // State persistence
    // ========================================================================

    fn load_state(&mut self, ctx: &egui::Context, id: egui::Id) {
        let state_id = id.with("state");
        let stored: Option<(bool, String, usize, Animation<f32>)> =
            ctx.data_mut(|d| d.get_temp(state_id));
        if let Some((is_open, search, selected, animation)) = stored {
            self.is_open = is_open;
            self.search = search;
            self.selected = selected;
            self.animation = animation;
        }
    }

    fn save_state(&self, ctx: &egui::Context, id: egui::Id) {
        let state_id = id.with("state");
        ctx.data_mut(|d| {
            d.insert_temp(
                state_id,
                (
                    self.is_open,
                    self.search.clone(),
                    self.selected,
                    self.animation.clone(),
                ),
            )
        });
    }

    // ========================================================================
    // Event handlers
    // ========================================================================

    fn handle_trigger(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if i.key_pressed(self.trigger_key) && i.modifiers.matches_exact(self.trigger_modifiers)
            {
                self.is_open = !self.is_open;
                if self.is_open {
                    self.search.clear();
                    self.selected = 0;
                    self.animation.start();
                } else {
                    self.animation.reset();
                }
            }
        });
    }

    fn handle_keyboard(
        &mut self,
        ctx: &egui::Context,
        items: &[&CommandItem],
        should_close: &mut bool,
        executed: &mut Option<String>,
    ) {
        let action_count = items
            .iter()
            .filter(|i| matches!(i, CommandItem::Action { .. }))
            .count();

        ctx.input(|i| {
            if i.key_pressed(Key::Escape) {
                *should_close = true;
            }

            if i.key_pressed(Key::ArrowDown) && self.selected < action_count.saturating_sub(1) {
                self.selected += 1;
            }

            if i.key_pressed(Key::ArrowUp) && self.selected > 0 {
                self.selected -= 1;
            }

            if i.key_pressed(Key::Enter) && action_count > 0 {
                let mut idx = 0;
                for item in items {
                    if let CommandItem::Action { id, .. } = item {
                        if idx == self.selected {
                            *executed = Some(id.clone());
                            *should_close = true;
                            break;
                        }
                        idx += 1;
                    }
                }
            }
        });
    }

    // ========================================================================
    // Filtering
    // ========================================================================

    fn filter_items<'a>(&self, items: &'a [CommandItem]) -> Vec<&'a CommandItem> {
        let query = self.search.to_lowercase();
        items
            .iter()
            .filter(|item| match item {
                CommandItem::Action { label, .. } => {
                    query.is_empty() || label.to_lowercase().contains(&query)
                }
                CommandItem::Group { .. } | CommandItem::Separator => query.is_empty(),
            })
            .collect()
    }

    // ========================================================================
    // Drawing
    // ========================================================================

    fn draw_backdrop(&self, ui: &mut Ui, id: egui::Id, should_close: &mut bool) {
        let screen = ui.ctx().viewport_rect();
        let alpha = (self.animation.value() * 128.0) as u8;

        egui::Area::new(id.with("backdrop"))
            .order(egui::Order::Foreground)
            .anchor(Align2::LEFT_TOP, vec2(0.0, 0.0))
            .show(ui.ctx(), |ui| {
                let response = ui.allocate_response(screen.size(), Sense::click());
                ui.painter()
                    .rect_filled(screen, 0.0, Color32::from_black_alpha(alpha));
                if response.clicked() {
                    *should_close = true;
                }
            });
    }

    fn draw_panel(
        &mut self,
        ui: &mut Ui,
        id: egui::Id,
        theme: &Theme,
        filtered: &[&CommandItem],
        should_close: &mut bool,
    ) -> Option<String> {
        let screen = ui.ctx().viewport_rect();
        let mut executed = None;

        egui::Area::new(id.with("panel"))
            .order(egui::Order::Foreground)
            .anchor(Align2::CENTER_TOP, vec2(0.0, screen.height() * 0.2))
            .show(ui.ctx(), |ui| {
                let panel_rect =
                    Rect::from_min_size(ui.cursor().min, vec2(PANEL_WIDTH, PANEL_MAX_HEIGHT));

                // Panel background
                ui.painter()
                    .rect_filled(panel_rect, CORNER_RADIUS, theme.popover());
                ui.painter().rect_stroke(
                    panel_rect,
                    CORNER_RADIUS,
                    egui::Stroke::new(1.0, theme.border()),
                    egui::StrokeKind::Inside,
                );

                ui.scope_builder(egui::UiBuilder::new().max_rect(panel_rect), |ui| {
                    ui.vertical(|ui| {
                        // Input - we need to handle this specially
                        self.draw_input(ui, id, theme);

                        // Separator
                        self.draw_separator(ui, theme, PANEL_WIDTH);

                        // List
                        if let Some((exec, sel)) = self.draw_list(ui, theme, filtered, should_close)
                        {
                            executed = exec;
                            self.selected = sel;
                        }
                    });
                });
            });

        executed
    }

    fn draw_input(&mut self, ui: &mut Ui, _id: egui::Id, theme: &Theme) {
        let input_rect = Rect::from_min_size(ui.cursor().min, vec2(PANEL_WIDTH, INPUT_HEIGHT));

        // Search icon
        let icon_x = input_rect.left() + INPUT_PADDING_X;
        ui.painter().text(
            Pos2::new(icon_x, input_rect.center().y),
            Align2::LEFT_CENTER,
            "🔍",
            egui::FontId::proportional(ICON_SIZE),
            theme.muted_foreground(),
        );

        // Text input - positioned after icon with gap
        let text_left = icon_x + ICON_SIZE + INPUT_GAP;
        let text_rect = Rect::from_min_max(
            Pos2::new(text_left, input_rect.top()),
            Pos2::new(input_rect.right() - INPUT_PADDING_X, input_rect.bottom()),
        );

        ui.scope_builder(egui::UiBuilder::new().max_rect(text_rect), |ui| {
            ui.centered_and_justified(|ui| {
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.search)
                        .frame(false)
                        .hint_text(&self.placeholder)
                        .font(egui::FontId::proportional(INPUT_TEXT_SIZE))
                        .vertical_align(egui::Align::Center),
                );
                response.request_focus();
            });
        });

        ui.advance_cursor_after_rect(input_rect);
    }

    fn draw_separator(&self, ui: &mut Ui, theme: &Theme, width: f32) {
        let y = ui.cursor().top();
        ui.painter().hline(
            ui.cursor().left()..=ui.cursor().left() + width,
            y,
            egui::Stroke::new(1.0, theme.border()),
        );
        ui.add_space(1.0);
    }

    fn draw_list(
        &self,
        ui: &mut Ui,
        theme: &Theme,
        items: &[&CommandItem],
        should_close: &mut bool,
    ) -> Option<(Option<String>, usize)> {
        let mut executed = None;
        let mut new_selected = self.selected;
        let mut action_index = 0;

        egui::ScrollArea::vertical()
            .max_height(LIST_MAX_HEIGHT)
            .show(ui, |ui| {
                ui.add_space(LIST_PADDING);

                ui.horizontal(|ui| {
                    ui.add_space(LIST_PADDING);
                    ui.vertical(|ui| {
                        if items.is_empty() {
                            self.draw_empty(ui, theme);
                        } else {
                            for item in items {
                                match item {
                                    CommandItem::Action {
                                        id,
                                        label,
                                        icon,
                                        shortcut,
                                    } => {
                                        let is_selected = action_index == self.selected;
                                        let result = self.draw_item(
                                            ui,
                                            theme,
                                            id,
                                            label,
                                            icon.as_deref(),
                                            shortcut.as_deref(),
                                            is_selected,
                                        );

                                        if let Some(clicked_id) = result.0 {
                                            executed = Some(clicked_id);
                                            *should_close = true;
                                        }
                                        if result.1 {
                                            new_selected = action_index;
                                        }
                                        action_index += 1;
                                    }
                                    CommandItem::Group { heading } => {
                                        self.draw_group_heading(ui, theme, heading);
                                    }
                                    CommandItem::Separator => {
                                        ui.add_space(LIST_PADDING);
                                        self.draw_separator(
                                            ui,
                                            theme,
                                            PANEL_WIDTH - LIST_PADDING * 4.0,
                                        );
                                        ui.add_space(LIST_PADDING);
                                    }
                                }
                            }
                        }
                    });
                    ui.add_space(LIST_PADDING);
                });

                ui.add_space(LIST_PADDING);
            });

        Some((executed, new_selected))
    }

    fn draw_empty(&self, ui: &mut Ui, theme: &Theme) {
        ui.add_space(24.0);
        ui.centered_and_justified(|ui| {
            ui.label(
                egui::RichText::new("No results found.")
                    .color(theme.muted_foreground())
                    .size(ITEM_TEXT_SIZE),
            );
        });
        ui.add_space(24.0);
    }

    fn draw_group_heading(&self, ui: &mut Ui, theme: &Theme, heading: &str) {
        ui.add_space(GROUP_PADDING_Y);
        ui.horizontal(|ui| {
            ui.add_space(GROUP_PADDING_X);
            ui.label(
                egui::RichText::new(heading)
                    .color(theme.muted_foreground())
                    .size(GROUP_HEADING_SIZE)
                    .strong(),
            );
        });
        ui.add_space(GROUP_PADDING_Y);
    }

    fn draw_item(
        &self,
        ui: &mut Ui,
        theme: &Theme,
        id: &str,
        label: &str,
        icon: Option<&str>,
        shortcut: Option<&str>,
        is_selected: bool,
    ) -> (Option<String>, bool) {
        let available_width = ui.available_width() - LIST_PADDING;
        let (rect, response) =
            ui.allocate_exact_size(vec2(available_width, ITEM_HEIGHT), Sense::click());

        let hovered = response.hovered();
        let clicked = response.clicked();

        // Background
        if is_selected || hovered {
            ui.painter().rect_filled(rect, ITEM_RADIUS, theme.accent());
        }

        // Text color based on state
        let text_color = if is_selected || hovered {
            theme.accent_foreground()
        } else {
            theme.popover_foreground()
        };

        let icon_color = if is_selected || hovered {
            theme.accent_foreground()
        } else {
            theme.muted_foreground()
        };

        let mut x = rect.left() + ITEM_PADDING_X;

        // Icon
        if let Some(icon_text) = icon {
            ui.painter().text(
                Pos2::new(x, rect.center().y),
                Align2::LEFT_CENTER,
                icon_text,
                egui::FontId::proportional(ICON_SIZE),
                icon_color,
            );
            x += ICON_SIZE + ITEM_GAP;
        }

        // Label
        ui.painter().text(
            Pos2::new(x, rect.center().y),
            Align2::LEFT_CENTER,
            label,
            egui::FontId::proportional(ITEM_TEXT_SIZE),
            text_color,
        );

        // Shortcut using Kbd component
        if let Some(shortcut_text) = shortcut {
            // Position the Kbd at the right side
            let kbd_rect = Rect::from_min_max(
                Pos2::new(rect.right() - 100.0, rect.top()),
                Pos2::new(rect.right() - ITEM_PADDING_X, rect.bottom()),
            );
            ui.scope_builder(egui::UiBuilder::new().max_rect(kbd_rect), |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    Kbd::new(shortcut_text).show(ui, theme);
                });
            });
        }

        let executed = if clicked { Some(id.to_string()) } else { None };
        (executed, hovered)
    }
}

impl Default for Command {
    fn default() -> Self {
        Self::new()
    }
}
