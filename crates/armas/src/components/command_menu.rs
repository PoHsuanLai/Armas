//! Command Menu Component
//!
//! macOS/VS Code style command palette with fuzzy search

use crate::ext::ArmasContextExt;
use egui::{
    vec2, Align, Color32, CornerRadius, Key, Layout, Modifiers, Sense, Stroke, StrokeKind,
    TextEdit, Ui,
};

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
pub struct CommandMenu {
    commands: Vec<Command>,
    is_open: bool,
    search_text: String,
    selected_index: usize,
    filtered_commands: Vec<usize>,
    trigger_key: Key,
    trigger_modifiers: Modifiers,
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
    }

    /// Close the command menu
    pub fn close(&mut self) {
        self.is_open = false;
        self.search_text.clear();
        self.selected_index = 0;
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

        // Overlay background
        let screen_rect = ui.ctx().viewport_rect();

        ui.painter().rect_filled(
            screen_rect,
            CornerRadius::ZERO,
            Color32::from_rgba_unmultiplied(0, 0, 0, 180), // Keep black overlay for modal
        );

        // Command menu panel
        let panel_width = 600.0;
        let panel_max_height = 400.0;

        let panel_pos = screen_rect.center() - vec2(panel_width / 2.0, panel_max_height / 2.0);
        let panel_rect = egui::Rect::from_min_size(panel_pos, vec2(panel_width, panel_max_height));

        // Background - use theme colors
        ui.painter().rect_filled(
            panel_rect,
            CornerRadius::same(theme.spacing.corner_radius as u8),
            theme.surface(),
        );
        ui.painter().rect_stroke(
            panel_rect,
            CornerRadius::same(theme.spacing.corner_radius as u8),
            Stroke::new(1.0, theme.outline()),
            StrokeKind::Outside,
        );

        // Create a UI region for the panel
        let mut panel_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(panel_rect)
                .layout(Layout::top_down(Align::LEFT)),
        );

        panel_ui.style_mut().spacing.item_spacing = vec2(0.0, 0.0);

        // Search input
        let search_response = panel_ui.add_sized(
            vec2(panel_width - 20.0, 40.0),
            TextEdit::singleline(&mut self.search_text)
                .hint_text("🔍 Type to search commands...")
                .font(egui::TextStyle::Body)
                .frame(false),
        );

        // Request focus on search input when opened
        if search_response.changed() {
            self.update_filtered_commands();
        }

        panel_ui.add_space(theme.spacing.xs);
        panel_ui.separator();
        panel_ui.add_space(theme.spacing.xs);

        // Handle keyboard navigation
        ui.input(|i| {
            if i.key_pressed(Key::Escape) {
                should_close = true;
            }

            if i.key_pressed(Key::ArrowDown) {
                if self.selected_index < self.filtered_commands.len().saturating_sub(1) {
                    self.selected_index += 1;
                }
            }

            if i.key_pressed(Key::ArrowUp) {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }

            if i.key_pressed(Key::Enter) && !self.filtered_commands.is_empty() {
                let cmd_idx = self.filtered_commands[self.selected_index];
                executed_command = Some(self.commands[cmd_idx].id.clone());
                should_close = true;
            }
        });

        // Command list
        let max_visible = 8;
        let visible_count = self.filtered_commands.len().min(max_visible);

        if self.filtered_commands.is_empty() {
            panel_ui.add_space(theme.spacing.lg);
            panel_ui.centered_and_justified(|ui| {
                ui.label(
                    egui::RichText::new("No commands found")
                        .color(theme.on_surface_variant())
                        .size(14.0),
                );
            });
        } else {
            for (display_idx, &cmd_idx) in self
                .filtered_commands
                .iter()
                .enumerate()
                .take(visible_count)
            {
                let command = &self.commands[cmd_idx];
                let is_selected = display_idx == self.selected_index;

                let item_height = 50.0;
                let (rect, response) = panel_ui
                    .allocate_exact_size(vec2(panel_width - 20.0, item_height), Sense::click());

                // Background - use theme colors
                if is_selected {
                    let primary = theme.primary();
                    ui.painter().rect_filled(
                        rect,
                        CornerRadius::same(theme.spacing.xs as u8),
                        Color32::from_rgba_unmultiplied(primary.r(), primary.g(), primary.b(), 60),
                    );
                } else if response.hovered() {
                    ui.painter().rect_filled(
                        rect,
                        CornerRadius::same(theme.spacing.xs as u8),
                        theme.hover(),
                    );
                }

                // Icon
                let mut cursor_x = rect.min.x + 10.0;
                if let Some(icon) = &command.icon {
                    ui.painter().text(
                        egui::pos2(cursor_x, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        icon,
                        egui::FontId::proportional(20.0),
                        theme.on_surface(),
                    );
                    cursor_x += 30.0;
                }

                // Name and description
                let text_y = if command.description.is_some() {
                    rect.min.y + 12.0
                } else {
                    rect.center().y
                };

                ui.painter().text(
                    egui::pos2(cursor_x, text_y),
                    egui::Align2::LEFT_TOP,
                    &command.name,
                    egui::FontId::proportional(16.0),
                    theme.on_surface(),
                );

                if let Some(desc) = &command.description {
                    ui.painter().text(
                        egui::pos2(cursor_x, text_y + 20.0),
                        egui::Align2::LEFT_TOP,
                        desc,
                        egui::FontId::proportional(12.0),
                        theme.on_surface_variant(),
                    );
                }

                // Shortcut
                if let Some(shortcut) = &command.shortcut {
                    ui.painter().text(
                        egui::pos2(rect.max.x - 10.0, rect.center().y),
                        egui::Align2::RIGHT_CENTER,
                        shortcut,
                        egui::FontId::monospace(12.0),
                        theme.on_surface_variant(),
                    );
                }

                // Handle click
                if response.clicked() {
                    executed_command = Some(command.id.clone());
                    should_close = true;
                } else if response.hovered() {
                    self.selected_index = display_idx;
                }
            }
        }

        // Request focus for keyboard input
        search_response.request_focus();
        ui.ctx().request_repaint();

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
