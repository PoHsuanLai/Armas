//! Card component examples
//!
//! Demonstrates Material Design card component with different elevations,
//! clickable cards, and various content types.

use armas::ext::ArmasContextExt;
use armas::{components::Card, Theme};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Card Examples"),
        ..Default::default()
    };

    eframe::run_native(
        "Card Examples",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(CardExample::default()))
        }),
    )
}

struct CardExample {
    current_theme: usize,
    selected_card: Option<usize>,
}

const THEMES: &[(&str, fn() -> Theme)] = &[
    ("Dark", Theme::dark),
    ("Light", Theme::light),
    ("Nord", Theme::nord),
    ("Dracula", Theme::dracula),
];

impl Default for CardExample {
    fn default() -> Self {
        Self {
            current_theme: 0,
            selected_card: None,
        }
    }
}

impl eframe::App for CardExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update theme in context
        let theme = THEMES[self.current_theme].1();
        ctx.set_armas_theme(theme.clone());

        // Apply theme colors to egui
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = theme.background();
        style.visuals.panel_fill = theme.background();
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Card Component Examples");
            ui.add_space(20.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Theme selector
                ui.horizontal(|ui| {
                    ui.label("Theme:");
                    for (i, (name, _)) in THEMES.iter().enumerate() {
                        if ui.selectable_label(self.current_theme == i, *name).clicked() {
                            self.current_theme = i;
                        }
                    }
                });

                ui.add_space(20.0);

                // Example 1: Basic cards with different elevations
                ui.heading("Elevation Levels");
                ui.label("Cards with different elevation levels (0-5)");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    for elevation in 0..=2 {
                        Card::new()
                            .title(&format!("Elevation {}", elevation))
                            .elevation(elevation)
                            .width(150.0)
                            .show(ui, &theme, |ui| {
                                ui.label(format!("Level: {}", elevation));
                                ui.separator();
                                ui.label("Border thickness increases with elevation");
                            });
                        ui.add_space(10.0);
                    }
                });

                ui.horizontal(|ui| {
                    for elevation in 3..=5 {
                        Card::new()
                            .title(&format!("Elevation {}", elevation))
                            .elevation(elevation)
                            .width(150.0)
                            .show(ui, &theme, |ui| {
                                ui.label(format!("Level: {}", elevation));
                                ui.separator();
                                ui.label("Higher elevations = thicker borders");
                            });
                        ui.add_space(10.0);
                    }
                });

                ui.add_space(30.0);

                // Example 2: Clickable cards
                ui.heading("Clickable Cards");
                ui.label("Click on cards to select them");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    for i in 0..3 {
                        let is_selected = self.selected_card == Some(i);

                        let response = Card::new()
                            .title(&format!("Card {}", i + 1))
                            .elevation(if is_selected { 4 } else { 1 })
                            .clickable(true)
                            .width(180.0)
                            .show(ui, &theme, |ui| {
                                ui.label("Click to select");
                                ui.add_space(5.0);
                                if is_selected {
                                    ui.label(
                                        egui::RichText::new("✓ Selected")
                                            .color(theme.success()),
                                    );
                                } else {
                                    ui.label("Not selected");
                                }
                            });

                        if response.clicked() {
                            self.selected_card = Some(i);
                        }

                        ui.add_space(10.0);
                    }
                });

                ui.add_space(30.0);

                // Example 3: Content cards
                ui.heading("Content Cards");
                ui.label("Cards with different types of content");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    // User profile card
                    Card::new()
                        .title("User Profile")
                        .elevation(2)
                        .width(200.0)
                        .show(ui, &theme, |ui| {
                            ui.label(egui::RichText::new("John Doe").size(18.0).strong());
                            ui.label("Software Engineer");
                            ui.separator();
                            ui.label("📧 john@example.com");
                            ui.label("📍 San Francisco, CA");
                        });

                    ui.add_space(10.0);

                    // Stats card
                    Card::new()
                        .title("Statistics")
                        .elevation(2)
                        .width(200.0)
                        .show(ui, &theme, |ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(
                                        egui::RichText::new("1,234")
                                            .size(24.0)
                                            .color(theme.primary()),
                                    );
                                    ui.label("Users");
                                });
                                ui.separator();
                                ui.vertical(|ui| {
                                    ui.label(
                                        egui::RichText::new("5,678")
                                            .size(24.0)
                                            .color(theme.success()),
                                    );
                                    ui.label("Active");
                                });
                            });
                        });

                    ui.add_space(10.0);

                    // Alert card
                    Card::new().title("Alert").elevation(3).width(200.0).show(
                        ui,
                        &theme,
                        |ui| {
                            ui.label(
                                egui::RichText::new("⚠ Warning")
                                    .color(theme.warning())
                                    .size(16.0),
                            );
                            ui.separator();
                            ui.label("System update available. Please restart to apply changes.");
                        },
                    );
                });

                ui.add_space(30.0);

                // Example 4: Full-width card
                ui.heading("Full-Width Card");
                ui.add_space(10.0);

                Card::new()
                    .title("Dashboard Overview")
                    .elevation(1)
                    .show(ui, &theme, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label("Total Revenue");
                                ui.label(
                                    egui::RichText::new("$12,345")
                                        .size(20.0)
                                        .color(theme.success()),
                                );
                            });
                            ui.separator();
                            ui.vertical(|ui| {
                                ui.label("New Customers");
                                ui.label(
                                    egui::RichText::new("89")
                                        .size(20.0)
                                        .color(theme.primary()),
                                );
                            });
                            ui.separator();
                            ui.vertical(|ui| {
                                ui.label("Pending Orders");
                                ui.label(
                                    egui::RichText::new("23")
                                        .size(20.0)
                                        .color(theme.warning()),
                                );
                            });
                        });
                    });

                ui.add_space(30.0);

                // Example 5: Card without title
                ui.heading("Titleless Cards");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    Card::new()
                        .elevation(1)
                        .width(120.0)
                        .show(ui, &theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(egui::RichText::new("🎨").size(32.0));
                                ui.label("Design");
                            });
                        });

                    ui.add_space(10.0);

                    Card::new()
                        .elevation(1)
                        .width(120.0)
                        .show(ui, &theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(egui::RichText::new("💻").size(32.0));
                                ui.label("Development");
                            });
                        });

                    ui.add_space(10.0);

                    Card::new()
                        .elevation(1)
                        .width(120.0)
                        .show(ui, &theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(egui::RichText::new("🚀").size(32.0));
                                ui.label("Deploy");
                            });
                        });
                });
            });
        });
    }
}
