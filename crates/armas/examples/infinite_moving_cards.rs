//! Infinite Moving Cards Example
//!
//! Demonstrates continuous scrolling card carousel

use armas::ext::ArmasContextExt;
use armas::{InfiniteMovingCards, MovingCard, ScrollDirection, ScrollSpeed, Theme};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_title("Armas - Infinite Moving Cards"),
        ..Default::default()
    };

    eframe::run_native(
        "Infinite Moving Cards",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(InfiniteMovingCardsApp::new()))
        }),
    )
}

struct InfiniteMovingCardsApp {
    carousel1: InfiniteMovingCards,
    carousel2: InfiniteMovingCards,
    carousel3: InfiniteMovingCards,
}

impl InfiniteMovingCardsApp {
    fn new() -> Self {
        // Testimonials carousel
        let testimonials = vec![
            MovingCard::new(
                "It was the best of times, it was the worst of times",
                "From A Tale of Two Cities",
            )
            .author("Charles Dickens")
            .background_color(egui::Color32::from_rgb(30, 58, 138)),
            MovingCard::new("To be, or not to be, that is the question", "From Hamlet")
                .author("William Shakespeare")
                .background_color(egui::Color32::from_rgb(88, 28, 135)),
            MovingCard::new(
                "All that we see or seem is but a dream within a dream",
                "From A Dream Within a Dream",
            )
            .author("Edgar Allan Poe")
            .background_color(egui::Color32::from_rgb(127, 29, 29)),
            MovingCard::new(
                "It is a truth universally acknowledged",
                "From Pride and Prejudice",
            )
            .author("Jane Austen")
            .background_color(egui::Color32::from_rgb(20, 83, 45)),
            MovingCard::new("Call me Ishmael", "From Moby-Dick")
                .author("Herman Melville")
                .background_color(egui::Color32::from_rgb(120, 53, 15)),
        ];

        // Feature cards
        let features = vec![
            MovingCard::new("Fast Performance", "Lightning-fast rendering with egui")
                .background_color(egui::Color32::from_rgb(15, 23, 42)),
            MovingCard::new("Smooth Animations", "Buttery smooth 60fps animations")
                .background_color(egui::Color32::from_rgb(17, 24, 39)),
            MovingCard::new("Customizable", "Full control over colors and behavior")
                .background_color(egui::Color32::from_rgb(24, 24, 27)),
            MovingCard::new("Responsive", "Adapts to any screen size")
                .background_color(egui::Color32::from_rgb(23, 23, 23)),
        ];

        // Fast scrolling cards
        let fast_cards = vec![
            MovingCard::new("Quick", "Rapid deployment")
                .background_color(egui::Color32::from_rgb(59, 7, 100)),
            MovingCard::new("Agile", "Flexible development")
                .background_color(egui::Color32::from_rgb(71, 85, 105)),
            MovingCard::new("Efficient", "Optimized performance")
                .background_color(egui::Color32::from_rgb(55, 48, 163)),
        ];

        Self {
            carousel1: InfiniteMovingCards::new(testimonials)
                .speed(ScrollSpeed::Slow)
                .pause_on_hover(true),
            carousel2: InfiniteMovingCards::new(features)
                .speed(ScrollSpeed::Normal)
                .direction(ScrollDirection::Right)
                .pause_on_hover(true),
            carousel3: InfiniteMovingCards::new(fast_cards)
                .speed(ScrollSpeed::Fast)
                .card_size(280.0, 150.0)
                .pause_on_hover(false),
        }
    }
}

impl eframe::App for InfiniteMovingCardsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let theme = ui.ctx().armas_theme();
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.heading("Infinite Moving Cards");
                    ui.add_space(10.0);
                    ui.label("Continuous scrolling card carousels with pause on hover");
                    ui.add_space(40.0);
                });

                // Example 1: Slow testimonials scrolling left
                ui.label(
                    egui::RichText::new("Classic Testimonials (Slow, Left)")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                self.carousel1.show(ui);

                ui.add_space(40.0);

                // Example 2: Normal speed features scrolling right
                ui.label(
                    egui::RichText::new("Feature Highlights (Normal, Right)")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                self.carousel2.show(ui);

                ui.add_space(40.0);

                // Example 3: Fast scrolling, no pause
                ui.label(
                    egui::RichText::new("Quick Facts (Fast, No Pause)")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                self.carousel3.show(ui);

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
                        ui.label("• Hover over cards to pause scrolling (if enabled)");
                        ui.label("• direction() controls scroll direction (Left/Right)");
                        ui.label("• speed() sets animation speed (Slow/Normal/Fast)");
                        ui.label("• pause_on_hover() enables/disables pause behavior");
                        ui.label("• card_size() customizes card dimensions");
                        ui.label("• Cards automatically duplicate for seamless infinite loop");
                        ui.label("• Perfect for testimonials, features, and content showcases");
                    });
                });

                ui.add_space(30.0);
            });
        });
    }
}
