//! Command Menu Component
//!
//! macOS/VS Code style command palette with fuzzy search

use crate::animation::{Animation, EasingFunction};
use crate::ext::ArmasContextExt;
use crate::{Card, Input};
use egui::{vec2, Align2, Color32, Key, Modifiers, Sense, Ui};

/// A command that can be executed
#[derive(Clone, Debug)]
pub struct Command {
    /// Display name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Optional icon (emoji or single character)
    pub icon: Option<String>,
    /// Command ID for handling execution
    pub id: String,
    /// Optional keyboard shortcut display
    pub shortcut: Option<String>,
    /// Optional category for grouping
    pub category: Option<String>,
}

impl Command {
    /// Create a new command
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        let id = id.into();
        Self {
            name: name.into(),
            description: None,
            icon: None,
            id,
            shortcut: None,
            category: None,
        }
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set keyboard shortcut
    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Set category
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }
}

/// Command menu state
#[derive(Clone)]
pub struct CommandMenu {
    commands: Vec<Command>,
    is_open: bool,
    search_text: String,
    selected_index: usize,
    filtered_commands: Vec<usize>,
    trigger_key: Key,
    trigger_modifiers: Modifiers,
    fade_animation: Animation<f32>,
}

impl CommandMenu {
    /// Create a new command menu
    pub fn new(commands: Vec<Command>) -> Self {
        let filtered_commands: Vec<usize> = (0..commands.len()).collect();

        Self {
            commands,
            is_open: false,
            search_text: String::new(),
            selected_index: 0,
            filtered_commands,
            trigger_key: Key::K,
            trigger_modifiers: Modifiers::COMMAND,
            fade_animation: Animation::new(0.0, 1.0, 0.15).with_easing(EasingFunction::CubicOut),
        }
    }

    /// Set trigger key combination (default: Cmd+K)
    pub fn with_trigger(mut self, key: Key, modifiers: Modifiers) -> Self {
        self.trigger_key = key;
        self.trigger_modifiers = modifiers;
        self
    }

    /// Open the command menu
    pub fn open(&mut self) {
        self.is_open = true;
        self.search_text.clear();
        self.selected_index = 0;
        self.update_filtered_commands();
        self.fade_animation.start();
    }

    /// Close the command menu
    pub fn close(&mut self) {
        self.is_open = false;
        self.search_text.clear();
        self.selected_index = 0;
        self.fade_animation.reset();
    }

    /// Toggle the command menu
    pub fn toggle(&mut self) {
        if self.is_open {
            self.close();
        } else {
            self.open();
        }
    }

    /// Check if menu is open
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Update filtered commands based on search
    fn update_filtered_commands(&mut self) {
        if self.search_text.is_empty() {
            self.filtered_commands = (0..self.commands.len()).collect();
        } else {
            let query = self.search_text.to_lowercase();
            self.filtered_commands = self
                .commands
                .iter()
                .enumerate()
                .filter(|(_, cmd)| {
                    cmd.name.to_lowercase().contains(&query)
                        || cmd
                            .description
                            .as_ref()
                            .map(|d| d.to_lowercase().contains(&query))
                            .unwrap_or(false)
                        || cmd
                            .category
                            .as_ref()
                            .map(|c| c.to_lowercase().contains(&query))
                            .unwrap_or(false)
                })
                .map(|(idx, _)| idx)
                .collect();
        }

        // Reset selection if out of bounds
        if self.selected_index >= self.filtered_commands.len() {
            self.selected_index = 0;
        }
    }

    /// Show the command menu
    pub fn show(&mut self, ui: &mut Ui) -> CommandMenuResponse {
        let theme = ui.ctx().armas_theme();
        let mut executed_command = None;
        let mut should_close = false;

        // Check for trigger key
        ui.input(|i| {
            if i.key_pressed(self.trigger_key) && i.modifiers.matches_exact(self.trigger_modifiers)
            {
                self.toggle();
            }
        });

        if !self.is_open {
            return CommandMenuResponse { executed_command };
        }

        // Update animation
        let dt = ui.ctx().input(|i| i.unstable_dt);
        self.fade_animation.update(dt);

        if self.fade_animation.is_running() {
            ui.ctx().request_repaint();
        }

        let screen_rect = ui.ctx().viewport_rect();
        let eased = self.fade_animation.value();

        // Draw backdrop using Area
        let backdrop_alpha = (eased * 180.0) as u8;
        let backdrop_color = Color32::from_rgba_unmultiplied(0, 0, 0, backdrop_alpha);

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
                let panel_rect =
                    egui::Rect::from_center_size(screen_rect.center(), vec2(panel_width, panel_height));

                ui.scope_builder(egui::UiBuilder::new().max_rect(panel_rect), |ui| {
                    // Use Card component for consistent styling with secondary border
                    Card::new()
                        .stroke(theme.secondary())
                        .corner_radius(12.0)
                        .inner_margin(0.0)
                        .elevation(3)
                        .show(ui, &theme, |ui| {
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 0.0;

                                // Search input with padding
                                ui.add_space(theme.spacing.md);
                                ui.horizontal(|ui| {
                                    ui.add_space(theme.spacing.md);

                                    let search_response = Input::new("Type to search commands...")
                                        .with_left_icon("🔍")
                                        .with_width(panel_width - theme.spacing.md * 2.0)
                                        .show(ui, &mut self.search_text);

                                    if search_response.changed() {
                                        self.update_filtered_commands();
                                    }

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

                                        if self.filtered_commands.is_empty() {
                                            ui.add_space(theme.spacing.xl);
                                            ui.centered_and_justified(|ui| {
                                                ui.label(
                                                    egui::RichText::new("No commands found")
                                                        .color(theme.on_surface_variant())
                                                        .size(14.0),
                                                );
                                            });
                                        } else {
                                            for (display_idx, &cmd_idx) in
                                                self.filtered_commands.iter().enumerate()
                                            {
                                                let command = &self.commands[cmd_idx];
                                                let is_selected = display_idx == self.selected_index;

                                                let item_height = if command.description.is_some() {
                                                    56.0
                                                } else {
                                                    40.0
                                                };

                                                let (rect, response) = ui.allocate_exact_size(
                                                    vec2(ui.available_width(), item_height),
                                                    Sense::click(),
                                                );

                                                // Background highlighting - use subtle hover style
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
                                                if let Some(icon) = &command.icon {
                                                    ui.painter().text(
                                                        egui::pos2(cursor_x, content_rect.center().y),
                                                        Align2::LEFT_CENTER,
                                                        icon,
                                                        egui::FontId::proportional(18.0),
                                                        theme.on_surface(),
                                                    );
                                                    cursor_x += 28.0;
                                                }

                                                // Name
                                                let name_y = if command.description.is_some() {
                                                    content_rect.min.y + 4.0
                                                } else {
                                                    content_rect.center().y
                                                };

                                                ui.painter().text(
                                                    egui::pos2(cursor_x, name_y),
                                                    Align2::LEFT_TOP,
                                                    &command.name,
                                                    egui::FontId::proportional(15.0),
                                                    theme.on_surface(),
                                                );

                                                // Description
                                                if let Some(desc) = &command.description {
                                                    ui.painter().text(
                                                        egui::pos2(cursor_x, name_y + 20.0),
                                                        Align2::LEFT_TOP,
                                                        desc,
                                                        egui::FontId::proportional(12.0),
                                                        theme.on_surface_variant(),
                                                    );
                                                }

                                                // Shortcut
                                                if let Some(shortcut) = &command.shortcut {
                                                    ui.painter().text(
                                                        egui::pos2(
                                                            content_rect.max.x,
                                                            content_rect.center().y,
                                                        ),
                                                        Align2::RIGHT_CENTER,
                                                        shortcut,
                                                        egui::FontId::monospace(11.0),
                                                        theme.on_surface_variant(),
                                                    );
                                                }

                                                // Handle interaction
                                                if response.clicked() {
                                                    executed_command = Some(command.id.clone());
                                                    should_close = true;
                                                } else if response.hovered() {
                                                    self.selected_index = display_idx;
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

            if i.key_pressed(Key::ArrowDown)
                && self.selected_index < self.filtered_commands.len().saturating_sub(1)
            {
                self.selected_index += 1;
            }

            if i.key_pressed(Key::ArrowUp) && self.selected_index > 0 {
                self.selected_index -= 1;
            }

            if i.key_pressed(Key::Enter) && !self.filtered_commands.is_empty() {
                let cmd_idx = self.filtered_commands[self.selected_index];
                executed_command = Some(self.commands[cmd_idx].id.clone());
                should_close = true;
            }
        });

        // Close if requested
        if should_close {
            self.close();
        }

        CommandMenuResponse { executed_command }
    }
}

/// Response from command menu
pub struct CommandMenuResponse {
    /// ID of executed command, if any
    pub executed_command: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_creation() {
        let cmd = Command::new("test", "Test Command")
            .with_description("A test")
            .with_icon("✨")
            .with_shortcut("Ctrl+T");

        assert_eq!(cmd.id, "test");
        assert_eq!(cmd.name, "Test Command");
        assert_eq!(cmd.icon, Some("✨".to_string()));
    }

    #[test]
    fn test_menu_creation() {
        let commands = vec![
            Command::new("cmd1", "First"),
            Command::new("cmd2", "Second"),
        ];

        let menu = CommandMenu::new(commands);
        assert_eq!(menu.filtered_commands.len(), 2);
        assert!(!menu.is_open);
    }

    #[test]
    fn test_menu_toggle() {
        let commands = vec![Command::new("test", "Test")];
        let mut menu = CommandMenu::new(commands);

        assert!(!menu.is_open());
        menu.open();
        assert!(menu.is_open());
        menu.close();
        assert!(!menu.is_open());
    }
}
