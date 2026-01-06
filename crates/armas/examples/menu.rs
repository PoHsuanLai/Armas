use armas::{BadgeColor, Button, ButtonVariant, Menu, MenuItem, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Menu Component Example",
        options,
        Box::new(|_cc| Ok(Box::new(MenuExample::default()))),
    )
}

struct MenuExample {
    theme: Theme,
    basic_menu_open: bool,
    file_menu_open: bool,
    edit_menu_open: bool,
    actions_menu_open: bool,
    status_message: String,
}

impl Default for MenuExample {
    fn default() -> Self {
        Self {
            theme: Theme::dark(),
            basic_menu_open: false,
            file_menu_open: false,
            edit_menu_open: false,
            actions_menu_open: false,
            status_message: String::from("No action selected"),
        }
    }
}

impl eframe::App for MenuExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Menu Component Examples");
            ui.add_space(20.0);

            ui.label(&self.status_message);
            ui.add_space(20.0);

            // Basic menu
            ui.label("Basic Menu:");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                let button = Button::new("Open Menu")
                    .variant(ButtonVariant::Filled)
                    .show(ui, &self.theme);

                if button.clicked() {
                    self.basic_menu_open = !self.basic_menu_open;
                }

                let mut menu = Menu::new("basic_menu")
                    .item("Option 1")
                    .item("Option 2")
                    .item("Option 3")
                    .separator()
                    .item("Help");

                let response = menu.show(ctx, &self.theme, button.rect, &mut self.basic_menu_open);

                if let Some(idx) = response.selected {
                    let labels = vec!["Option 1", "Option 2", "Option 3", "Help"];
                    if idx < labels.len() {
                        self.status_message = format!("Selected: {}", labels[idx]);
                    }
                }
            });
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // File menu
            ui.label("File Menu (with icons and shortcuts):");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                let button = Button::new("File")
                    .variant(ButtonVariant::Outlined)
                    .show(ui, &self.theme);

                if button.clicked() {
                    self.file_menu_open = !self.file_menu_open;
                }

                let mut menu = Menu::new("file_menu")
                    .add_item(MenuItem::new("New File").icon("📄").shortcut("Ctrl+N"))
                    .add_item(MenuItem::new("Open...").icon("📂").shortcut("Ctrl+O"))
                    .add_item(MenuItem::new("Save").icon("💾").shortcut("Ctrl+S"))
                    .add_item(
                        MenuItem::new("Save As...")
                            .icon("💾")
                            .shortcut("Ctrl+Shift+S"),
                    )
                    .separator()
                    .add_item(MenuItem::new("Close").icon("✕").shortcut("Ctrl+W"));

                let response = menu.show(ctx, &self.theme, button.rect, &mut self.file_menu_open);

                if let Some(idx) = response.selected {
                    let actions = vec!["New File", "Open", "Save", "Save As", "Close"];
                    if idx < actions.len() {
                        self.status_message = format!("File > {}", actions[idx]);
                    }
                }
            });
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Edit menu with disabled items
            ui.label("Edit Menu (with disabled items):");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                let button = Button::new("Edit")
                    .variant(ButtonVariant::Outlined)
                    .show(ui, &self.theme);

                if button.clicked() {
                    self.edit_menu_open = !self.edit_menu_open;
                }

                let mut menu = Menu::new("edit_menu")
                    .add_item(
                        MenuItem::new("Undo")
                            .icon("↶")
                            .shortcut("Ctrl+Z")
                            .disabled(true),
                    )
                    .add_item(
                        MenuItem::new("Redo")
                            .icon("↷")
                            .shortcut("Ctrl+Y")
                            .disabled(true),
                    )
                    .separator()
                    .add_item(MenuItem::new("Cut").icon("✂").shortcut("Ctrl+X"))
                    .add_item(MenuItem::new("Copy").icon("📋").shortcut("Ctrl+C"))
                    .add_item(MenuItem::new("Paste").icon("📄").shortcut("Ctrl+V"))
                    .separator()
                    .add_item(MenuItem::new("Select All").shortcut("Ctrl+A"));

                let response = menu.show(ctx, &self.theme, button.rect, &mut self.edit_menu_open);

                if let Some(idx) = response.selected {
                    let actions = vec!["Cut", "Copy", "Paste", "Select All"];
                    // Map index accounting for disabled items and separators
                    if idx == 3 {
                        self.status_message = format!("Edit > Cut");
                    } else if idx == 4 {
                        self.status_message = format!("Edit > Copy");
                    } else if idx == 5 {
                        self.status_message = format!("Edit > Paste");
                    } else if idx == 7 {
                        self.status_message = format!("Edit > Select All");
                    }
                }
            });
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Actions menu with badges
            ui.label("Actions Menu (with badges):");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                let button = Button::new("Actions")
                    .variant(ButtonVariant::Filled)
                    .show(ui, &self.theme);

                if button.clicked() {
                    self.actions_menu_open = !self.actions_menu_open;
                }

                let mut menu = Menu::new("actions_menu")
                    .add_item(
                        MenuItem::new("Messages")
                            .icon("✉")
                            .badge("5", BadgeColor::Primary),
                    )
                    .add_item(
                        MenuItem::new("Notifications")
                            .icon("🔔")
                            .badge("12", BadgeColor::Error),
                    )
                    .add_item(
                        MenuItem::new("Updates")
                            .icon("⬆")
                            .badge("New", BadgeColor::Success),
                    )
                    .separator()
                    .add_item(MenuItem::new("Settings").icon("⚙"))
                    .add_item(MenuItem::new("Logout").icon("🚪"));

                let response =
                    menu.show(ctx, &self.theme, button.rect, &mut self.actions_menu_open);

                if let Some(idx) = response.selected {
                    let actions =
                        vec!["Messages", "Notifications", "Updates", "Settings", "Logout"];
                    if idx == 0 {
                        self.status_message = format!("Opened Messages (5 new)");
                    } else if idx == 1 {
                        self.status_message = format!("Opened Notifications (12 new)");
                    } else if idx == 2 {
                        self.status_message = format!("Opened Updates");
                    } else if idx == 4 {
                        self.status_message = format!("Opened Settings");
                    } else if idx == 5 {
                        self.status_message = format!("Logged out");
                    }
                }
            });
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            ui.label("Tip: Use arrow keys to navigate and Enter to select when menu is open!");
        });
    }
}
