//! Command Menu Component
//!
//! macOS/VS Code style command palette with fuzzy search

use crate::animation::{Animation, EasingFunction};
use crate::ext::ArmasContextExt;
use crate::{Card, CardVariant, Input};
use egui::{vec2, Align2, Color32, Key, Modifiers, Sense, Ui};

// ============================================================================
// NEW CLOSURE-BASED API
// ============================================================================

/// Internal representation of a command menu item
#[derive(Clone)]
enum InternalMenuItem {
    Command {
        id: String,
        name: String,
        description: Option<String>,
        icon: Option<String>,
        shortcut: Option<String>,
    },
    Category {
        name: String,
    },
}

/// Builder for configuring individual commands
pub struct CommandBuilder {
    id: String,
    name: String,
    description: Option<String>,
    icon: Option<String>,
    shortcut: Option<String>,
}

impl CommandBuilder {
    fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            description: None,
            icon: None,
            shortcut: None,
        }
    }

    /// Set command description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set command icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set keyboard shortcut display
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }
}

/// Builder for adding commands and categories to the menu
pub struct CommandMenuBuilder<'a> {
    items: &'a mut Vec<InternalMenuItem>,
}

impl<'a> CommandMenuBuilder<'a> {
    /// Add a command to the menu
    pub fn command(&mut self, id: &str, name: &str) -> CommandBuilder {
        let builder = CommandBuilder::new(id.to_string(), name.to_string());
        self.items.push(InternalMenuItem::Command {
            id: builder.id.clone(),
            name: builder.name.clone(),
            description: builder.description.clone(),
            icon: builder.icon.clone(),
            shortcut: builder.shortcut.clone(),
        });
        builder
    }

    /// Add a category separator
    pub fn category(&mut self, name: &str) {
        self.items.push(InternalMenuItem::Category {
            name: name.to_string(),
        });
    }
}

/// Response from command menu
pub struct CommandMenuResponse {
    /// ID of executed command, if any
    pub executed_command: Option<String>,
}

/// Command menu state
pub struct CommandMenu {
    placeholder: String,
    trigger_key: Key,
    trigger_modifiers: Modifiers,
}

impl CommandMenu {
    /// Create a new command menu
    pub fn new() -> Self {
        Self {
            placeholder: "Type a command...".to_string(),
            trigger_key: Key::K,
            trigger_modifiers: Modifiers::COMMAND,
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set trigger key combination (default: Cmd+K)
    pub fn trigger(mut self, key: Key, modifiers: Modifiers) -> Self {
        self.trigger_key = key;
        self.trigger_modifiers = modifiers;
        self
    }

    /// Show the command menu with closure-based API
    ///
    /// # Example
    ///
    /// ```ignore
    /// CommandMenu::new()
    ///     .placeholder("Type a command...")
    ///     .show(ui, |menu| {
    ///         menu.command("new_file", "New File")
    ///             .description("Create a new file")
    ///             .icon("📄")
    ///             .shortcut("Cmd+N");
    ///
    ///         menu.category("Edit");
    ///         menu.command("copy", "Copy").shortcut("Cmd+C");
    ///     })
    /// ```
    pub fn show<R>(
        &mut self,
        ui: &mut Ui,
        content: impl FnOnce(&mut CommandMenuBuilder) -> R,
    ) -> CommandMenuResponse {
        let theme = ui.ctx().armas_theme();

        // Get state from memory
        let menu_id = ui.id().with("command_menu_state");
        let (mut is_open, mut search_text, mut selected_index, mut fade_animation): (
            bool,
            String,
            usize,
            Animation<f32>,
        ) = ui.ctx().data_mut(|d| {
            d.get_temp(menu_id).unwrap_or((
                false,
                String::new(),
                0,
                Animation::new(0.0, 1.0, 0.15).easing(EasingFunction::CubicOut),
            ))
        });

        let mut executed_command = None;
        let mut should_close = false;

        // Check for trigger key
        ui.input(|i| {
            if i.key_pressed(self.trigger_key) && i.modifiers.matches_exact(self.trigger_modifiers)
            {
                is_open = !is_open;
                if is_open {
                    search_text.clear();
                    selected_index = 0;
                    fade_animation.start();
                } else {
                    fade_animation.reset();
                }
            }
        });

        if !is_open {
            // Save state
            ui.ctx().data_mut(|d| {
                d.insert_temp(
                    menu_id,
                    (is_open, search_text, selected_index, fade_animation),
                );
            });
            return CommandMenuResponse { executed_command };
        }

        // Collect items from closure
        let mut items = Vec::new();
        let mut builder = CommandMenuBuilder { items: &mut items };
        content(&mut builder);

        // Filter commands based on search
        let query = search_text.to_lowercase();
        let filtered_items: Vec<(usize, &InternalMenuItem)> = items
            .iter()
            .enumerate()
            .filter(|(_, item)| match item {
                InternalMenuItem::Command {
                    name, description, ..
                } => {
                    query.is_empty()
                        || name.to_lowercase().contains(&query)
                        || description
                            .as_ref()
                            .map(|d| d.to_lowercase().contains(&query))
                            .unwrap_or(false)
                }
                InternalMenuItem::Category { .. } => query.is_empty(),
            })
            .collect();

        // Update animation
        let dt = ui.ctx().input(|i| i.unstable_dt);
        fade_animation.update(dt);

        if fade_animation.is_running() {
            ui.ctx().request_repaint();
        }

        let screen_rect = ui.ctx().viewport_rect();
        let eased = fade_animation.value();

        // Draw backdrop using Area
        let backdrop_alpha = (eased * 180.0) as u8;
        let backdrop_color = Color32::from_black_alpha(backdrop_alpha);

        egui::Area::new(ui.id().with("command_menu_backdrop"))
            .order(egui::Order::Foreground)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .show(ui.ctx(), |ui| {
                let backdrop_response = ui.allocate_response(screen_rect.size(), Sense::click());
                ui.painter().rect_filled(screen_rect, 0.0, backdrop_color);

                // Click backdrop to close
                if backdrop_response.clicked() {
                    should_close = true;
                }
            });

        // Draw modal panel using Area
        let panel_width = 600.0;
        let panel_height = 400.0;

        egui::Area::new(ui.id().with("command_menu_panel"))
            .order(egui::Order::Foreground)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .show(ui.ctx(), |ui| {
                let panel_rect = egui::Rect::from_center_size(
                    screen_rect.center(),
                    vec2(panel_width, panel_height),
                );

                ui.scope_builder(egui::UiBuilder::new().max_rect(panel_rect), |ui| {
                    // Use Card component with MD3 Elevated variant for floating command palette
                    Card::new()
                        .variant(CardVariant::Elevated) // Use Elevated for command palette
                        .stroke(theme.secondary())
                        .corner_radius(12.0)
                        .inner_margin(0.0)
                        .show(ui, &theme, |ui| {
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 0.0;

                                // Search input with padding
                                ui.add_space(theme.spacing.md);
                                ui.horizontal(|ui| {
                                    ui.add_space(theme.spacing.md);

                                    let search_response = Input::new(&self.placeholder)
                                        .left_icon("🔍")
                                        .width(panel_width - theme.spacing.md * 2.0)
                                        .show(ui, &mut search_text);

                                    // Auto-focus search input
                                    search_response.request_focus();

                                    ui.add_space(theme.spacing.md);
                                });
                                ui.add_space(theme.spacing.sm);

                                // Separator
                                ui.separator();

                                // Command list with scroll area
                                egui::ScrollArea::vertical()
                                    .max_height(panel_height - 100.0)
                                    .show(ui, |ui| {
                                        ui.spacing_mut().item_spacing.y = 2.0;

                                        if filtered_items.is_empty() {
                                            ui.add_space(theme.spacing.xl);
                                            ui.centered_and_justified(|ui| {
                                                ui.label(
                                                    egui::RichText::new("No commands found")
                                                        .color(theme.on_surface_variant())
                                                        .size(14.0),
                                                );
                                            });
                                        } else {
                                            let mut command_index = 0;
                                            for (_, item) in &filtered_items {
                                                match item {
                                                    InternalMenuItem::Category { name } => {
                                                        // Render category header
                                                        ui.add_space(theme.spacing.sm);
                                                        ui.horizontal(|ui| {
                                                            ui.add_space(theme.spacing.md);
                                                            ui.label(
                                                                egui::RichText::new(name)
                                                                    .color(
                                                                        theme.on_surface_variant(),
                                                                    )
                                                                    .size(12.0)
                                                                    .strong(),
                                                            );
                                                        });
                                                        ui.add_space(theme.spacing.xs);
                                                    }
                                                    InternalMenuItem::Command {
                                                        id,
                                                        name,
                                                        description,
                                                        icon,
                                                        shortcut,
                                                    } => {
                                                        let is_selected =
                                                            command_index == selected_index;
                                                        command_index += 1;

                                                        let item_height = if description.is_some() {
                                                            56.0
                                                        } else {
                                                            40.0
                                                        };

                                                        let (rect, response) = ui
                                                            .allocate_exact_size(
                                                                vec2(
                                                                    ui.available_width(),
                                                                    item_height,
                                                                ),
                                                                Sense::click(),
                                                            );

                                                        // Background highlighting
                                                        if is_selected || response.hovered() {
                                                            ui.painter().rect_filled(
                                                                rect,
                                                                theme.spacing.corner_radius,
                                                                theme.hover(),
                                                            );
                                                        }

                                                        // Content
                                                        let content_rect = rect.shrink2(vec2(
                                                            theme.spacing.md,
                                                            theme.spacing.sm,
                                                        ));

                                                        let mut cursor_x = content_rect.min.x;

                                                        // Icon
                                                        if let Some(icon_text) = icon {
                                                            ui.painter().text(
                                                                egui::pos2(
                                                                    cursor_x,
                                                                    content_rect.center().y,
                                                                ),
                                                                Align2::LEFT_CENTER,
                                                                icon_text,
                                                                egui::FontId::proportional(18.0),
                                                                theme.on_surface(),
                                                            );
                                                            cursor_x += 28.0;
                                                        }

                                                        // Name
                                                        let name_y = if description.is_some() {
                                                            content_rect.min.y + 4.0
                                                        } else {
                                                            content_rect.center().y
                                                        };

                                                        ui.painter().text(
                                                            egui::pos2(cursor_x, name_y),
                                                            Align2::LEFT_TOP,
                                                            name,
                                                            egui::FontId::proportional(15.0),
                                                            theme.on_surface(),
                                                        );

                                                        // Description
                                                        if let Some(desc) = description {
                                                            ui.painter().text(
                                                                egui::pos2(cursor_x, name_y + 20.0),
                                                                Align2::LEFT_TOP,
                                                                desc,
                                                                egui::FontId::proportional(12.0),
                                                                theme.on_surface_variant(),
                                                            );
                                                        }

                                                        // Shortcut
                                                        if let Some(shortcut_text) = shortcut {
                                                            ui.painter().text(
                                                                egui::pos2(
                                                                    content_rect.max.x,
                                                                    content_rect.center().y,
                                                                ),
                                                                Align2::RIGHT_CENTER,
                                                                shortcut_text,
                                                                egui::FontId::monospace(11.0),
                                                                theme.on_surface_variant(),
                                                            );
                                                        }

                                                        // Handle interaction
                                                        if response.clicked() {
                                                            executed_command = Some(id.clone());
                                                            should_close = true;
                                                        } else if response.hovered() {
                                                            selected_index = command_index - 1;
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        ui.add_space(theme.spacing.sm);
                                    });
                            });
                        });
                });
            });

        // Handle keyboard navigation
        ui.input(|i| {
            if i.key_pressed(Key::Escape) {
                should_close = true;
            }

            // Count only commands for navigation, not categories
            let command_count = filtered_items
                .iter()
                .filter(|(_, item)| matches!(item, InternalMenuItem::Command { .. }))
                .count();

            if i.key_pressed(Key::ArrowDown) && selected_index < command_count.saturating_sub(1) {
                selected_index += 1;
            }

            if i.key_pressed(Key::ArrowUp) && selected_index > 0 {
                selected_index -= 1;
            }

            if i.key_pressed(Key::Enter) && command_count > 0 {
                // Find the command at the selected index
                let mut cmd_idx = 0;
                for (_, item) in &filtered_items {
                    if let InternalMenuItem::Command { id, .. } = item {
                        if cmd_idx == selected_index {
                            executed_command = Some(id.clone());
                            should_close = true;
                            break;
                        }
                        cmd_idx += 1;
                    }
                }
            }
        });

        // Close if requested
        if should_close {
            is_open = false;
            fade_animation.reset();
        }

        // Save state
        ui.ctx().data_mut(|d| {
            d.insert_temp(
                menu_id,
                (is_open, search_text, selected_index, fade_animation),
            );
        });

        CommandMenuResponse { executed_command }
    }
}

impl Default for CommandMenu {
    fn default() -> Self {
        Self::new()
    }
}
