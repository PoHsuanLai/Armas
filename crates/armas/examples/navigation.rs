//! Navigation Components Showcase
//!
//! Demonstrates navigation components including FloatingNavbar, Sidebar, Breadcrumbs, FloatingDock, and CommandMenu

use armas::ext::ArmasContextExt;
use armas::{
    BreadcrumbItem, Breadcrumbs, Command, CommandMenu, DockItem, DockPosition, FloatingDock,
    FloatingNavbar, NavItem, NavbarPosition, Sidebar, SidebarItem, Theme,
};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_title("Armas - Navigation Components"),
        ..Default::default()
    };

    eframe::run_native(
        "Navigation",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(NavigationApp::new()))
        }),
    )
}

struct NavigationApp {
    floating_navbar: FloatingNavbar,
    sidebar: Sidebar,
    dock: FloatingDock,
    command_menu: CommandMenu,
    last_action: String,
    current_page: String,
}

impl NavigationApp {
    fn new() -> Self {
        let theme = Theme::dark();

        // Floating Navbar items
        let navbar_items = vec![
            NavItem::new("Home").icon("🏠").active(true),
            NavItem::new("Products").icon("📦"),
            NavItem::new("About").icon("ℹ️"),
            NavItem::new("Contact").icon("📧"),
        ];

        // Sidebar items
        let sidebar_items = vec![
            SidebarItem::new("🏠", "Dashboard").active(true),
            SidebarItem::new("📊", "Analytics"),
            SidebarItem::new("📧", "Messages").badge("3"),
            SidebarItem::new("👥", "Team"),
            SidebarItem::new("⚙️", "Settings"),
        ];

        // Dock items
        let dock_items = vec![
            DockItem::new("Home", "🏠").with_id("home"),
            DockItem::new("Search", "🔍").with_id("search"),
            DockItem::new("Mail", "📧").with_id("mail"),
            DockItem::new("Calendar", "📅").with_id("calendar"),
            DockItem::new("Photos", "🖼️").with_id("photos"),
            DockItem::new("Music", "🎵").with_id("music"),
            DockItem::new("Settings", "⚙️").with_id("settings"),
        ];

        // Commands
        let commands = vec![
            Command::new("new_file", "New File")
                .with_icon("📄")
                .with_description("Create a new file")
                .with_shortcut("⌘N")
                .with_category("File"),
            Command::new("open_file", "Open File")
                .with_icon("📂")
                .with_description("Open an existing file")
                .with_shortcut("⌘O")
                .with_category("File"),
            Command::new("save_file", "Save File")
                .with_icon("💾")
                .with_description("Save the current file")
                .with_shortcut("⌘S")
                .with_category("File"),
            Command::new("search", "Search Everywhere")
                .with_icon("🔍")
                .with_description("Search across all files")
                .with_shortcut("⌘⇧F")
                .with_category("Search"),
            Command::new("goto_line", "Go to Line")
                .with_icon("➡️")
                .with_description("Jump to a specific line number")
                .with_shortcut("⌘G")
                .with_category("Navigation"),
            Command::new("toggle_theme", "Toggle Theme")
                .with_icon("🎨")
                .with_description("Switch between light and dark theme")
                .with_shortcut("⌘T")
                .with_category("View"),
        ];

        Self {
            floating_navbar: FloatingNavbar::new(navbar_items).position(NavbarPosition::Top),
            sidebar: Sidebar::new(sidebar_items),
            dock: FloatingDock::new(dock_items, &theme)
                .with_magnification(1.8)
                .with_position(DockPosition::Bottom),
            command_menu: CommandMenu::new(commands),
            last_action: "Welcome! Try the navigation components".to_string(),
            current_page: "Dashboard".to_string(),
        }
    }
}

impl eframe::App for NavigationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let theme = ctx.armas_theme();
        // Show floating navbar (it creates its own Area internally)
        let navbar_response = self.floating_navbar.show(ctx);

        if let Some(idx) = navbar_response.clicked {
            let labels = ["Home", "Products", "About", "Contact"];
            self.last_action = format!("Navbar: clicked {}", labels[idx]);
            self.current_page = labels[idx].to_string();
        }

        // Sidebar on the left
        egui::SidePanel::left("sidebar_panel")
            .resizable(false)
            .show(ctx, |ui| {
                let sidebar_response = self.sidebar.show(ui);

                if let Some(idx) = sidebar_response.clicked {
                    let labels = ["Dashboard", "Analytics", "Messages", "Team", "Settings"];
                    self.last_action = format!("Sidebar: clicked {}", labels[idx]);
                    self.current_page = labels[idx].to_string();
                }
            });

        // Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            ui.add_space(80.0); // Space for floating navbar

            ui.vertical_centered(|ui| {
                ui.heading("Navigation Components Showcase");
                ui.add_space(10.0);
                ui.label(
                    "Floating Navbar • Sidebar • Breadcrumbs • Floating Dock • Command Menu (⌘K)",
                );
            });

            ui.add_space(20.0);

            // Breadcrumbs (rebuilt each frame since show() consumes self)
            let breadcrumb_response = Breadcrumbs::new()
                .add_item(BreadcrumbItem::new("Home").icon("🏠"))
                .add_item(BreadcrumbItem::new("Products"))
                .add_item(BreadcrumbItem::new("Electronics").current())
                .show(ui);

            if let Some(idx) = breadcrumb_response.clicked {
                self.last_action = format!("Breadcrumb: clicked item {}", idx);
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Current page and last action
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.label("Current Page:");
                ui.label(
                    egui::RichText::new(&self.current_page)
                        .color(egui::Color32::from_rgb(100, 200, 255))
                        .size(18.0)
                        .strong(),
                );
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.label("Last action:");
                ui.label(
                    egui::RichText::new(&self.last_action)
                        .color(egui::Color32::from_rgb(150, 150, 150))
                        .size(14.0),
                );
            });

            ui.add_space(30.0);

            // Component descriptions
            ui.horizontal(|ui| {
                ui.add_space(20.0);

                ui.vertical(|ui| {
                    ui.heading("Components");
                    ui.add_space(10.0);

                    ui.label("🔝 Floating Navbar (Top)");
                    ui.label("   • Morphing pill animation");
                    ui.label("   • Follows active item");
                    ui.add_space(5.0);

                    ui.label("📂 Sidebar (Left)");
                    ui.label("   • Expand/collapse animation");
                    ui.label("   • Click toggle button");
                    ui.add_space(5.0);

                    ui.label("🍞 Breadcrumbs");
                    ui.label("   • Clickable navigation path");
                    ui.label("   • Separator animations");
                    ui.add_space(5.0);

                    ui.label("⬇️ Floating Dock (Bottom)");
                    ui.label("   • macOS-style magnification");
                    ui.label("   • Smooth animations");
                    ui.add_space(5.0);

                    ui.label("⌨️ Command Menu (⌘K)");
                    ui.label("   • Fuzzy search");
                    ui.label("   • Keyboard navigation");
                });

                ui.add_space(60.0);

                ui.vertical(|ui| {
                    ui.heading("Code Examples");
                    ui.add_space(10.0);

                    ui.label("Floating Navbar:");
                    ui.code(
                        "let items = vec![
  NavItem::new(\"Home\").icon(\"🏠\"),
  NavItem::new(\"About\").icon(\"ℹ️\"),
];
let mut navbar = FloatingNavbar::new(items)
  .position(NavbarPosition::Top);
navbar.show(ui);",
                    );

                    ui.add_space(10.0);

                    ui.label("Sidebar:");
                    ui.code(
                        "let items = vec![
  SidebarItem::new(\"🏠\", \"Home\"),
  SidebarItem::new(\"📧\", \"Mail\").badge(\"3\"),
];
let mut sidebar = Sidebar::new(items);
sidebar.show(ui);",
                    );
                });
            });

            ui.add_space(100.0); // Space for floating dock
        });

        // Show dock
        egui::Area::new(egui::Id::new("dock_area"))
            .fixed_pos(egui::pos2(0.0, 0.0))
            .show(ctx, |ui| {
                let dock_response = self.dock.show(ui);
                if let Some(clicked_idx) = dock_response.clicked_item {
                    self.last_action = format!("Dock: clicked item {}", clicked_idx);
                }
            });

        // Show command menu
        egui::Area::new(egui::Id::new("command_menu_area"))
            .fixed_pos(egui::pos2(0.0, 0.0))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let cmd_response = self.command_menu.show(ui);
                if let Some(cmd_id) = cmd_response.executed_command {
                    self.last_action = format!("Command executed: {}", cmd_id);
                }
            });
    }
}
