use armas::ext::ArmasContextExt;
use armas::{Badge, BadgeColor, BadgeVariant, NotificationBadge, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Badge Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Badge Demo",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(BadgeApp::new()))
        }),
    )
}

struct BadgeApp {
    notification_count: usize,
}

impl BadgeApp {
    fn new() -> Self {
        Self {
            notification_count: 12,
        }
    }
}

impl eframe::App for BadgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Badge Component");
            ui.add_space(20.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Variants
                ui.heading("Variants");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    Badge::new("Filled")
                        .variant(BadgeVariant::Filled)
                        .show(ui);
                    ui.add_space(10.0);
                    Badge::new("Outlined")
                        .variant(BadgeVariant::Outlined)
                        .show(ui);
                    ui.add_space(10.0);
                    Badge::new("Soft")
                        .variant(BadgeVariant::Soft)
                        .show(ui);
                });

                ui.add_space(30.0);

                // Colors
                ui.heading("Colors");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    Badge::new("Primary")
                        .color(BadgeColor::Primary)
                        .show(ui);
                    ui.add_space(10.0);
                    Badge::new("Success")
                        .color(BadgeColor::Success)
                        .show(ui);
                    ui.add_space(10.0);
                    Badge::new("Warning")
                        .color(BadgeColor::Warning)
                        .show(ui);
                    ui.add_space(10.0);
                    Badge::new("Error")
                        .color(BadgeColor::Error)
                        .show(ui);
                    ui.add_space(10.0);
                    Badge::new("Info")
                        .color(BadgeColor::Info)
                        .show(ui);
                });

                ui.add_space(30.0);

                // With dots
                ui.heading("With Dot Indicators");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    Badge::new("Online")
                        .color(BadgeColor::Success)
                        .with_dot()
                        .show(ui);
                    ui.add_space(10.0);
                    Badge::new("Away")
                        .color(BadgeColor::Warning)
                        .with_dot()
                        .show(ui);
                    ui.add_space(10.0);
                    Badge::new("Offline")
                        .color(BadgeColor::Neutral)
                        .with_dot()
                        .show(ui);
                });

                ui.add_space(30.0);

                // Removable
                ui.heading("Removable Tags");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if Badge::new("React")
                        .color(BadgeColor::Info)
                        .removable()
                        .show(ui)
                        .removed
                    {
                        println!("Removed React");
                    }
                    ui.add_space(10.0);
                    if Badge::new("Rust")
                        .color(BadgeColor::Warning)
                        .removable()
                        .show(ui)
                        .removed
                    {
                        println!("Removed Rust");
                    }
                    ui.add_space(10.0);
                    if Badge::new("TypeScript")
                        .color(BadgeColor::Primary)
                        .removable()
                        .show(ui)
                        .removed
                    {
                        println!("Removed TypeScript");
                    }
                });

                ui.add_space(30.0);

                // Notification badges
                ui.heading("Notification Badges");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label("Messages");
                    ui.add_space(5.0);
                    NotificationBadge::new(self.notification_count).show(ui);

                    ui.add_space(20.0);

                    ui.label("Alerts");
                    ui.add_space(5.0);
                    NotificationBadge::new(3)
                        .color(egui::Color32::from_rgb(251, 191, 36))
                        .show(ui);

                    ui.add_space(20.0);

                    ui.label("Updates");
                    ui.add_space(5.0);
                    NotificationBadge::new(156)
                        .color(egui::Color32::from_rgb(59, 130, 246))
                        .show(ui);
                });

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("Increment").clicked() {
                        self.notification_count += 1;
                    }
                    if ui.button("Decrement").clicked() && self.notification_count > 0 {
                        self.notification_count -= 1;
                    }
                    if ui.button("Clear").clicked() {
                        self.notification_count = 0;
                    }
                });

                ui.add_space(30.0);

                // Sizes
                ui.heading("Sizes");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    Badge::new("Small").size(11.0).show(ui);
                    ui.add_space(10.0);
                    Badge::new("Medium").size(13.0).show(ui);
                    ui.add_space(10.0);
                    Badge::new("Large").size(15.0).show(ui);
                });
            });
        });
    }
}
