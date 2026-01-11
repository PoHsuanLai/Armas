use armas::ext::ArmasContextExt;
use armas::{Button, ButtonVariant, Drawer, DrawerPosition, DrawerSize, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Drawer Component Example",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(DrawerExample::default()))
        }),
    )
}

struct DrawerExample {
    left_drawer_open: bool,
    right_drawer_open: bool,
    top_drawer_open: bool,
    bottom_drawer_open: bool,
    settings_drawer_open: bool,
    nav_drawer_open: bool,
    left_drawer: Drawer,
    right_drawer: Drawer,
    top_drawer: Drawer,
    bottom_drawer: Drawer,
    settings_drawer: Drawer,
    nav_drawer: Drawer,
}

impl Default for DrawerExample {
    fn default() -> Self {
        Self {
            left_drawer_open: false,
            right_drawer_open: false,
            top_drawer_open: false,
            bottom_drawer_open: false,
            settings_drawer_open: false,
            nav_drawer_open: false,
            left_drawer: Drawer::new("left")
                .position(DrawerPosition::Left)
                .size(DrawerSize::Medium)
                .title("Left Drawer"),
            right_drawer: Drawer::new("right")
                .position(DrawerPosition::Right)
                .size(DrawerSize::Medium)
                .title("Right Drawer"),
            top_drawer: Drawer::new("top")
                .position(DrawerPosition::Top)
                .size(DrawerSize::Small)
                .title("Top Drawer"),
            bottom_drawer: Drawer::new("bottom")
                .position(DrawerPosition::Bottom)
                .size(DrawerSize::Small)
                .title("Bottom Drawer"),
            settings_drawer: Drawer::new("settings")
                .position(DrawerPosition::Right)
                .size(DrawerSize::Large)
                .title("Settings"),
            nav_drawer: Drawer::new("nav")
                .position(DrawerPosition::Left)
                .size(DrawerSize::Small)
                .title("Navigation"),
        }
    }
}

impl eframe::App for DrawerExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Drawer Component Examples");
            ui.add_space(20.0);

            ui.label("Click buttons to open drawers from different positions:");
            ui.add_space(20.0);

            // Position examples
            ui.label("Different Positions:");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if Button::new("Open Left")
                    .variant(ButtonVariant::Outlined)
                    .show(ui)
                    .clicked()
                {
                    self.left_drawer_open = true;
                }

                if Button::new("Open Right")
                    .variant(ButtonVariant::Outlined)
                    .show(ui)
                    .clicked()
                {
                    self.right_drawer_open = true;
                }

                if Button::new("Open Top")
                    .variant(ButtonVariant::Outlined)
                    .show(ui)
                    .clicked()
                {
                    self.top_drawer_open = true;
                }

                if Button::new("Open Bottom")
                    .variant(ButtonVariant::Outlined)
                    .show(ui)
                    .clicked()
                {
                    self.bottom_drawer_open = true;
                }
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Practical examples
            ui.label("Practical Examples:");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if Button::new("⚙ Settings")
                    .variant(ButtonVariant::Filled)
                    .show(ui)
                    .clicked()
                {
                    self.settings_drawer_open = true;
                }

                if Button::new("☰ Navigation")
                    .variant(ButtonVariant::Filled)
                    .show(ui)
                    .clicked()
                {
                    self.nav_drawer_open = true;
                }
            });

            ui.add_space(20.0);

            ui.label("💡 Tips:");
            ui.label("  • Click backdrop or ✕ button to close");
            ui.label("  • Press ESC to close active drawer");
            ui.label("  • Drawers animate smoothly when opening/closing");
        });

        // Show drawers
        self.left_drawer
            .show(ctx, &self.theme, &mut self.left_drawer_open, |ui| {
                ui.label("This is a left drawer!");
                ui.add_space(10.0);
                ui.label("It slides in from the left side of the screen.");
                ui.add_space(20.0);
                ui.separator();
                ui.add_space(20.0);
                ui.label("You can put any content here:");
                ui.add_space(10.0);
                for i in 1..=10 {
                    ui.label(format!("Menu item {}", i));
                }
            });

        self.right_drawer
            .show(ctx, &self.theme, &mut self.right_drawer_open, |ui| {
                ui.label("This is a right drawer!");
                ui.add_space(10.0);
                ui.label("It slides in from the right side.");
                ui.add_space(20.0);
                ui.separator();
                ui.add_space(20.0);

                ui.label("Common use cases:");
                ui.label("  • Shopping cart");
                ui.label("  • Notifications");
                ui.label("  • User profile");
                ui.label("  • Quick settings");
            });

        self.top_drawer
            .show(ctx, &self.theme, &mut self.top_drawer_open, |ui| {
                ui.label("This is a top drawer!");
                ui.add_space(10.0);
                ui.label("It slides down from the top.");
                ui.add_space(10.0);
                ui.label("Useful for notifications or alerts.");
            });

        self.bottom_drawer
            .show(ctx, &self.theme, &mut self.bottom_drawer_open, |ui| {
                ui.label("This is a bottom drawer!");
                ui.add_space(10.0);
                ui.label("It slides up from the bottom.");
                ui.add_space(10.0);
                ui.label("Great for mobile-style bottom sheets.");
            });

        let theme = &self.theme;
        let mut close_settings = false;

        self.settings_drawer
            .show(ctx, theme, &mut self.settings_drawer_open, |ui| {
                ui.heading("Application Settings");
                ui.add_space(20.0);

                ui.label("Appearance");
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.label("Theme:");
                    ui.radio_value(&mut 0, 0, "Dark");
                    ui.radio_value(&mut 0, 1, "Light");
                });
                ui.add_space(20.0);

                ui.label("Notifications");
                ui.add_space(10.0);
                ui.checkbox(&mut true, "Enable notifications");
                ui.checkbox(&mut false, "Sound effects");
                ui.checkbox(&mut true, "Desktop notifications");
                ui.add_space(20.0);

                ui.label("Advanced");
                ui.add_space(10.0);
                ui.checkbox(&mut false, "Developer mode");
                ui.checkbox(&mut true, "Auto-save");
                ui.add_space(20.0);

                ui.separator();
                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if Button::new("Save")
                        .variant(ButtonVariant::Filled)
                        .show(ui)
                        .clicked()
                    {
                        close_settings = true;
                    }

                    if Button::new("Cancel")
                        .variant(ButtonVariant::Outlined)
                        .show(ui)
                        .clicked()
                    {
                        close_settings = true;
                    }
                });
            });

        if close_settings {
            self.settings_drawer_open = false;
        }

        let theme = &self.theme;
        let mut close_nav = false;

        self.nav_drawer
            .show(ctx, theme, &mut self.nav_drawer_open, |ui| {
                ui.heading("Menu");
                ui.add_space(20.0);

                let nav_items = vec![
                    ("🏠", "Home"),
                    ("📊", "Dashboard"),
                    ("📁", "Projects"),
                    ("👥", "Team"),
                    ("📧", "Messages"),
                    ("⚙", "Settings"),
                    ("❓", "Help"),
                ];

                for (icon, label) in nav_items {
                    if Button::new(format!("{} {}", icon, label))
                        .variant(ButtonVariant::Text)
                        .show(ui)
                        .clicked()
                    {
                        close_nav = true;
                    }
                    ui.add_space(4.0);
                }

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(20.0);

                if Button::new("🚪 Logout")
                    .variant(ButtonVariant::Outlined)
                    .show(ui)
                    .clicked()
                {
                    close_nav = true;
                }
            });

        if close_nav {
            self.nav_drawer_open = false;
        }
    }
}
