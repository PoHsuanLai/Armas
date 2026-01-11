//! Text Reveal Card Example
//!
//! Demonstrates text reveal effect on mouse hover

use armas::ext::ArmasContextExt;
use armas::{TextRevealCard, Theme};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_title("Armas - Text Reveal Card"),
        ..Default::default()
    };

    eframe::run_native(
        "Text Reveal Card",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(TextRevealCardApp::new()))
        }),
    )
}

struct TextRevealCardApp {
    card1: TextRevealCard,
    card2: TextRevealCard,
    card3: TextRevealCard,
}

impl TextRevealCardApp {
    fn new() -> Self {
        Self {
            card1: TextRevealCard::new(
                550.0,
                250.0,
                "Sometimes, you just need to see it.".to_string(),
                "Hover to reveal the magic ✨".to_string(),
            ),
            card2: TextRevealCard::new(
                550.0,
                250.0,
                "The best way to predict the future".to_string(),
                "is to invent it. — Alan Kay".to_string(),
            )
            .reveal_color(egui::Color32::from_rgb(168, 85, 247)),
            card3: TextRevealCard::new(
                550.0,
                250.0,
                "Hidden in plain sight".to_string(),
                "Now you see me! 👀".to_string(),
            )
            .reveal_color(egui::Color32::from_rgb(34, 197, 94))
            .background_color(egui::Color32::from_rgb(20, 20, 30)),
        }
    }
}

impl eframe::App for TextRevealCardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.heading("Text Reveal Card");
                    ui.add_space(10.0);
                    ui.label("Reveal hidden text by moving your mouse across the card");
                    ui.add_space(30.0);
                });

                // Example 1: Default cyan reveal
                ui.label(
                    egui::RichText::new("Classic Reveal")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(25.0);
                    ui.vertical(|ui| {
                        self.card1.show(ui);
                    });
                });

                ui.add_space(30.0);

                // Example 2: Purple reveal
                ui.label(
                    egui::RichText::new("Purple Gradient")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(25.0);
                    ui.vertical(|ui| {
                        self.card2.show(ui);
                    });
                });

                ui.add_space(30.0);

                // Example 3: Green reveal
                ui.label(
                    egui::RichText::new("Custom Colors")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(25.0);
                    ui.vertical(|ui| {
                        self.card3.show(ui);
                    });
                });

                ui.add_space(50.0);
                ui.separator();
                ui.add_space(20.0);

                // Tips
                ui.horizontal(|ui| {
                    ui.add_space(40.0);
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Tips")
                                .size(16.0)
                                .strong()
                                .color(theme.on_surface()),
                        );
                        ui.add_space(10.0);
                        ui.label("• Move your mouse left to right across the card");
                        ui.label("• The reveal follows your cursor position");
                        ui.label("• Smoothly animates back when you leave the card");
                        ui.label("• reveal_color() customizes the revealed text color");
                        ui.label("• Uses Animation<f32> for smooth transitions");
                        ui.label("• Clip-path effect reveals text progressively");
                        ui.label("• Perfect for interactive content reveals and CTAs");
                    });
                });

                ui.add_space(30.0);
            });
        });
    }
}
