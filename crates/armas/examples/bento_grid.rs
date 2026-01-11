//! Bento Grid Layout Example
//!
//! Demonstrates modern grid layout with variable-sized tiles

use armas::ext::ArmasContextExt;
use armas::{BentoGrid, BentoItem, GridSpan, Theme};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_title("Armas - Bento Grid Layout"),
        ..Default::default()
    };

    eframe::run_native(
        "Bento Grid",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(BentoGridApp::new()))
        }),
    )
}

struct BentoGridApp {
}

impl BentoGridApp {
    fn new() -> Self {
        Self {
        }
    }
}

impl eframe::App for BentoGridApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let theme = ui.ctx().armas_theme();
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.heading("Bento Grid Layout");
                    ui.add_space(10.0);
                    ui.label("Modern grid layout with variable-sized tiles");
                    ui.add_space(30.0);
                });

                ui.add_space(20.0);

                // Example 1: Dashboard Layout
                ui.label(
                    egui::RichText::new("Dashboard Layout")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                let grid = BentoGrid::new(3, 150.0)
                    .gap(12.0)
                    .default_background(egui::Color32::from_gray(30))
                    .default_border(Some(egui::Color32::from_gray(60)));

                let items = vec![
                    BentoItem::new(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            ui.label(egui::RichText::new("📊").size(40.0));
                            ui.add_space(10.0);
                            ui.label(egui::RichText::new("Analytics").size(18.0).strong());
                            ui.label(
                                egui::RichText::new("View your stats")
                                    .size(12.0)
                                    .color(egui::Color32::from_gray(160)),
                            );
                        });
                    })
                    .span(GridSpan::Single),
                    BentoItem::new(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            ui.label(egui::RichText::new("👥").size(40.0));
                            ui.add_space(10.0);
                            ui.label(egui::RichText::new("Team").size(18.0).strong());
                            ui.label(
                                egui::RichText::new("Manage members")
                                    .size(12.0)
                                    .color(egui::Color32::from_gray(160)),
                            );
                        });
                    })
                    .span(GridSpan::Single),
                    BentoItem::new(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            ui.label(egui::RichText::new("⚙️").size(40.0));
                            ui.add_space(10.0);
                            ui.label(egui::RichText::new("Settings").size(18.0).strong());
                            ui.label(
                                egui::RichText::new("Configure app")
                                    .size(12.0)
                                    .color(egui::Color32::from_gray(160)),
                            );
                        });
                    })
                    .span(GridSpan::Single),
                    BentoItem::new(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(40.0);
                            ui.label(egui::RichText::new("📈").size(50.0));
                            ui.add_space(15.0);
                            ui.label(egui::RichText::new("Revenue Overview").size(20.0).strong());
                            ui.add_space(10.0);
                            ui.label(egui::RichText::new("$42,567").size(36.0).strong());
                            ui.label(
                                egui::RichText::new("+12.5% from last month")
                                    .size(14.0)
                                    .color(egui::Color32::from_rgb(0, 200, 0)),
                            );
                        });
                    })
                    .span(GridSpan::Wide)
                    .background(egui::Color32::from_rgba_unmultiplied(59, 130, 246, 20)),
                    BentoItem::new(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(40.0);
                            ui.label(egui::RichText::new("📝").size(50.0));
                            ui.add_space(15.0);
                            ui.label(egui::RichText::new("Recent Activity").size(20.0).strong());
                            ui.add_space(20.0);
                            ui.label("• Task completed");
                            ui.label("• New member joined");
                            ui.label("• Report generated");
                        });
                    })
                    .span(GridSpan::Tall),
                ];

                grid.show(ui, &theme, items);

                ui.add_space(50.0);
                ui.separator();
                ui.add_space(30.0);

                // Example 2: Feature Grid
                ui.label(
                    egui::RichText::new("Feature Showcase")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                let feature_grid = BentoGrid::new(4, 120.0).gap(16.0);

                let features = vec![
                    BentoItem::new(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(30.0);
                            ui.label(egui::RichText::new("⚡").size(60.0));
                            ui.add_space(15.0);
                            ui.label(egui::RichText::new("Fast Performance").size(22.0).strong());
                            ui.add_space(10.0);
                            ui.label(
                                egui::RichText::new(
                                    "Lightning-fast rendering with 60fps animations",
                                )
                                .size(13.0)
                                .color(egui::Color32::from_gray(160)),
                            );
                        });
                    })
                    .span(GridSpan::Large)
                    .background(egui::Color32::from_rgba_unmultiplied(236, 72, 153, 20))
                    .padding(20.0),
                    BentoItem::new(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            ui.label(egui::RichText::new("🎨").size(40.0));
                            ui.add_space(10.0);
                            ui.label(egui::RichText::new("Beautiful UI").size(16.0).strong());
                        });
                    })
                    .span(GridSpan::Single),
                    BentoItem::new(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            ui.label(egui::RichText::new("🔒").size(40.0));
                            ui.add_space(10.0);
                            ui.label(egui::RichText::new("Secure").size(16.0).strong());
                        });
                    })
                    .span(GridSpan::Single),
                    BentoItem::new(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            ui.label(egui::RichText::new("🌐").size(40.0));
                            ui.add_space(10.0);
                            ui.label(egui::RichText::new("Cross-Platform").size(16.0).strong());
                        });
                    })
                    .span(GridSpan::Wide)
                    .background(egui::Color32::from_rgba_unmultiplied(147, 51, 234, 20)),
                    BentoItem::new(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            ui.label(egui::RichText::new("📦").size(40.0));
                            ui.add_space(10.0);
                            ui.label(egui::RichText::new("Easy to Use").size(16.0).strong());
                        });
                    })
                    .span(GridSpan::Single),
                ];

                feature_grid.show(ui, &theme, features);

                ui.add_space(50.0);
                ui.separator();
                ui.add_space(20.0);

                // Tips
                ui.horizontal(|ui| {
                    ui.add_space(40.0);
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("💡 Tips")
                                .size(16.0)
                                .strong()
                                .color(theme.on_surface()),
                        );
                        ui.add_space(10.0);
                        ui.label("• Use GridSpan::Large for hero/featured items");
                        ui.label("• GridSpan::Wide works great for charts and stats");
                        ui.label("• GridSpan::Tall is perfect for activity feeds");
                        ui.label("• Mix different spans for visual interest");
                        ui.label("• Adjust cell_size based on content density");
                    });
                });

                ui.add_space(30.0);
            });
        });
    }
}
