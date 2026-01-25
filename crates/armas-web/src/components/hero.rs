//! Hero section for the home page

use armas::prelude::*;
use armas_animated::AuroraBackground;
use eframe::egui;

pub struct Hero;

impl Hero {
    pub fn show(ui: &mut egui::Ui) {
        let theme = ui.ctx().armas_theme();
        let available_rect = ui.available_rect_before_wrap();
        let hero_height = ui.ctx().viewport_rect().height();

        // Create full viewport rect for aurora
        let full_rect = egui::Rect::from_min_size(
            available_rect.min,
            egui::vec2(available_rect.width(), hero_height),
        );

        // Draw aurora background
        ui.painter()
            .rect_filled(full_rect, 0.0, egui::Color32::BLACK);

        AuroraBackground::cyberpunk(full_rect.width(), full_rect.height())
            .id("hero_aurora")
            .show(ui);

        // Draw content on top
        ui.scope_builder(egui::UiBuilder::new().max_rect(full_rect), |ui| {
            ui.set_height(hero_height);

            egui::Frame::new()
                .fill(theme.card().gamma_multiply(0.3))
                .inner_margin(0.0)
                .show(ui, |ui| {
                    ui.set_height(hero_height);
                    ui.set_width(ui.available_width());

                    // Center everything horizontally only
                    ui.vertical(|ui| {
                        ui.add_space(hero_height * 0.35);

                        // Hero title - centered horizontally
                        ui.horizontal(|ui| {
                            ui.allocate_space(egui::vec2(
                                (ui.available_width() - 200.0) / 2.0,
                                0.0,
                            ));
                            ui.label(
                                egui::RichText::new("Armas")
                                    .size(72.0)
                                    .family(egui::FontFamily::Name("InterBold".into()))
                                    .color(egui::Color32::WHITE),
                            );
                        });

                        ui.add_space(16.0);

                        // Subtitle - centered
                        ui.horizontal(|ui| {
                            let text_width = 400.0; // Approximate width
                            ui.allocate_space(egui::vec2(
                                (ui.available_width() - text_width) / 2.0,
                                0.0,
                            ));
                            ui.label(
                                egui::RichText::new("Modern UI Components for egui")
                                    .size(24.0)
                                    .color(egui::Color32::from_gray(200)),
                            );
                        });

                        ui.add_space(32.0);

                        // Description - centered
                        ui.horizontal(|ui| {
                            let text_width = 600.0; // Approximate width
                            ui.allocate_space(egui::vec2(
                                (ui.available_width() - text_width) / 2.0,
                                0.0,
                            ));
                            ui.label(
                                egui::RichText::new(
                                    "Material Design inspired components with smooth animations",
                                )
                                .size(16.0)
                                .color(egui::Color32::from_gray(160)),
                            );
                        });

                        ui.add_space(48.0);

                        // CTA Buttons - centered
                        ui.horizontal(|ui| {
                            let buttons_width = 316.0; // 140 + 16 + 160
                            ui.allocate_space(egui::vec2(
                                (ui.available_width() - buttons_width) / 2.0,
                                0.0,
                            ));

                            if Button::new("Get Started")
                                .variant(ButtonVariant::Default)
                                .size(ButtonSize::Large)
                                .min_width(140.0)
                                .show(ui)
                                .clicked()
                            {
                                // Navigate to components page
                                ui.data_mut(|d| {
                                    d.insert_temp(
                                        egui::Id::new("current_view"),
                                        "components".to_string(),
                                    )
                                });
                            }

                            ui.add_space(16.0);

                            if Button::new("View on GitHub")
                                .variant(ButtonVariant::Outline)
                                .size(ButtonSize::Large)
                                .min_width(160.0)
                                .show(ui)
                                .clicked()
                            {
                                Self::open_github();
                            }
                        });
                    });
                });
        });
    }

    fn open_github() {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                let _ = window.open_with_url("https://github.com/PoHsuanLai/Armas");
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = open::that("https://github.com/PoHsuanLai/Armas");
        }
    }
}
