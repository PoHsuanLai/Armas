//! Focus Cards Example
//!
//! Demonstrates interactive card grid with focus/blur effect

use armas::{FocusCard, FocusCards, Theme};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_title("Armas - Focus Cards"),
        ..Default::default()
    };

    eframe::run_native(
        "Focus Cards",
        options,
        Box::new(|_cc| Ok(Box::new(FocusCardsApp::new()))),
    )
}

struct FocusCardsApp {
    theme: Theme,
    focus_cards: FocusCards,
}

impl FocusCardsApp {
    fn new() -> Self {
        // Create sample cards
        let cards = vec![
            FocusCard::new("Discover", "Explore new possibilities")
                .background_color(egui::Color32::from_rgb(30, 58, 138)), // blue-900
            FocusCard::new("Create", "Build something amazing")
                .background_color(egui::Color32::from_rgb(88, 28, 135)), // purple-900
            FocusCard::new("Innovate", "Push the boundaries")
                .background_color(egui::Color32::from_rgb(127, 29, 29)), // red-900
            FocusCard::new("Collaborate", "Work together seamlessly")
                .background_color(egui::Color32::from_rgb(20, 83, 45)), // green-900
            FocusCard::new("Scale", "Grow without limits")
                .background_color(egui::Color32::from_rgb(120, 53, 15)), // orange-900
            FocusCard::new("Ship", "Deploy with confidence")
                .background_color(egui::Color32::from_rgb(12, 74, 110)), // cyan-900
        ];

        Self {
            theme: Theme::dark(),
            focus_cards: FocusCards::new(cards)
                .card_size(350.0, 450.0)
                .spacing(25.0)
                .columns(3),
        }
    }
}

impl eframe::App for FocusCardsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.heading("Focus Cards");
                    ui.add_space(10.0);
                    ui.label("Hover over a card to focus it while blurring others");
                    ui.add_space(40.0);
                });

                // Show the focus cards grid
                ui.horizontal(|ui| {
                    ui.add_space(50.0);
                    ui.vertical(|ui| {
                        let response = self.focus_cards.show(ui, &self.theme);

                        if let Some(clicked_idx) = response.clicked {
                            println!("Card {} was clicked!", clicked_idx);
                        }
                    });
                });

                ui.add_space(50.0);
                ui.separator();
                ui.add_space(20.0);

                // Tips
                ui.horizontal(|ui| {
                    ui.add_space(60.0);
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Tips")
                                .size(16.0)
                                .strong()
                                .color(self.theme.on_surface()),
                        );
                        ui.add_space(10.0);
                        ui.label("• Hover over any card to focus it");
                        ui.label("• Focused card stays sharp while others blur");
                        ui.label("• Uses Animation<f32> for smooth blur transitions");
                        ui.label("• Click on cards to interact with them");
                        ui.label("• card_size() customizes individual card dimensions");
                        ui.label("• spacing() controls the gap between cards");
                        ui.label("• columns() sets the grid layout");
                        ui.label("• Perfect for portfolios, galleries, and feature showcases");
                    });
                });

                ui.add_space(30.0);
            });
        });
    }
}
