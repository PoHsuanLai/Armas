use armas::ext::ArmasContextExt;
use armas::{BreadcrumbItem, Breadcrumbs, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Breadcrumbs Component Example",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(BreadcrumbsExample::default()))
        }),
    )
}

struct BreadcrumbsExample {
    current_path: Vec<String>,
    status_message: String,
}

impl Default for BreadcrumbsExample {
    fn default() -> Self {
        Self {
            current_path: vec![
                "Home".to_string(),
                "Projects".to_string(),
                "Armas".to_string(),
                "Components".to_string(),
            ],
            status_message: String::from("Navigate using breadcrumbs"),
        }
    }
}

impl eframe::App for BreadcrumbsExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Breadcrumbs Component Examples");
            ui.add_space(20.0);

            ui.label(&self.status_message);
            ui.add_space(20.0);

            // Basic breadcrumbs
            ui.label("Basic Breadcrumbs:");
            ui.add_space(10.0);

            let mut breadcrumbs = Breadcrumbs::new();
            for (idx, path) in self.current_path.iter().enumerate() {
                let is_current = idx == self.current_path.len() - 1;
                let item = if is_current {
                    BreadcrumbItem::new(path).current()
                } else {
                    BreadcrumbItem::new(path)
                };
                breadcrumbs = breadcrumbs.add_item(item);
            }

            let response = breadcrumbs.show(ui);
            if let Some(clicked) = response.clicked {
                // Navigate to clicked breadcrumb
                self.current_path.truncate(clicked + 1);
                self.status_message = format!("Navigated to: {}", self.current_path.join(" / "));
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Breadcrumbs with home icon
            ui.label("With Home Icon:");
            ui.add_space(10.0);

            let mut breadcrumbs = Breadcrumbs::new().show_home_icon(true);
            for (idx, path) in self.current_path.iter().enumerate() {
                let is_current = idx == self.current_path.len() - 1;
                let item = if is_current {
                    BreadcrumbItem::new(path).current()
                } else {
                    BreadcrumbItem::new(path)
                };
                breadcrumbs = breadcrumbs.add_item(item);
            }

            let response = breadcrumbs.show(ui);
            if let Some(clicked) = response.clicked {
                if clicked == 0 {
                    // Home clicked
                    self.current_path = vec!["Home".to_string()];
                    self.status_message = "Navigated to: Home".to_string();
                } else {
                    self.current_path.truncate(clicked);
                    self.status_message =
                        format!("Navigated to: {}", self.current_path.join(" / "));
                }
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Breadcrumbs with custom separator
            ui.label("Custom Separator (/):");
            ui.add_space(10.0);

            let mut breadcrumbs = Breadcrumbs::new().separator("/");
            for (idx, path) in self.current_path.iter().enumerate() {
                let is_current = idx == self.current_path.len() - 1;
                let item = if is_current {
                    BreadcrumbItem::new(path).current()
                } else {
                    BreadcrumbItem::new(path)
                };
                breadcrumbs = breadcrumbs.add_item(item);
            }

            let response = breadcrumbs.show(ui);
            if let Some(clicked) = response.clicked {
                self.current_path.truncate(clicked + 1);
                self.status_message = format!("Navigated to: {}", self.current_path.join(" / "));
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Breadcrumbs with icons
            ui.label("With Icons:");
            ui.add_space(10.0);

            let icons = vec!["🏠", "📁", "📦", "🧩"];
            let mut breadcrumbs = Breadcrumbs::new();
            for (idx, path) in self.current_path.iter().enumerate() {
                let is_current = idx == self.current_path.len() - 1;
                let icon = if idx < icons.len() {
                    icons[idx]
                } else {
                    "📄"
                };
                let item = if is_current {
                    BreadcrumbItem::new(path).icon(icon).current()
                } else {
                    BreadcrumbItem::new(path).icon(icon)
                };
                breadcrumbs = breadcrumbs.add_item(item);
            }

            let response = breadcrumbs.show(ui);
            if let Some(clicked) = response.clicked {
                self.current_path.truncate(clicked + 1);
                self.status_message = format!("Navigated to: {}", self.current_path.join(" / "));
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Navigation buttons to modify path
            ui.label("Simulate navigation:");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("Add 'Examples'").clicked() {
                    self.current_path.push("Examples".to_string());
                    self.status_message = "Added level: Examples".to_string();
                }

                if ui.button("Add 'Breadcrumbs.rs'").clicked() {
                    self.current_path.push("Breadcrumbs.rs".to_string());
                    self.status_message = "Added level: Breadcrumbs.rs".to_string();
                }

                if ui.button("Go back").clicked() && self.current_path.len() > 1 {
                    let removed = self.current_path.pop().unwrap();
                    self.status_message = format!("Removed level: {}", removed);
                }

                if ui.button("Reset to Home").clicked() {
                    self.current_path = vec!["Home".to_string()];
                    self.status_message = "Reset to: Home".to_string();
                }
            });
        });
    }
}
