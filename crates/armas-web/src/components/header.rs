//! Top header/navbar component for the showcase

use crate::showcase_sizes;
use armas::*;
use eframe::egui;

pub struct Header;

impl Header {
    pub fn show(ui: &mut egui::Ui, search_text: &mut String) {
        let theme = ui.ctx().armas_theme();
        GlassPanel::new()
            .blur(10.0)
            .opacity(0.05)
            .inner_margin(8.0)
            .show(ui, &theme, |ui| {
                let total_height = showcase_sizes::HEADER_HEIGHT - 16.0;
                ui.set_height(total_height);

                let full_rect = ui.available_rect_before_wrap();

                // Calculate estimated right side width
                let right_side_width = 500.0; // Components + GitHub + Search + spacing

                // Left side: Logo
                let left_rect =
                    egui::Rect::from_min_size(full_rect.min, egui::vec2(200.0, total_height));

                // Right side: Components / GitHub / Search
                let right_rect = egui::Rect::from_min_size(
                    egui::pos2(full_rect.max.x - right_side_width, full_rect.min.y),
                    egui::vec2(right_side_width, total_height),
                );

                // Render left side
                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(left_rect), |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(16.0);
                        ui.label(
                            egui::RichText::new("Armas")
                                .size(20.0)
                                .family(egui::FontFamily::Name("InterBold".into()))
                                .color(egui::Color32::WHITE)
                        );
                    });
                });

                // Render right side
                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(right_rect), |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new("Components")
                                .size(14.0)
                                .color(egui::Color32::from_gray(160)),
                        );

                        ui.add_space(16.0);

                        if Button::new("GitHub")
                            .variant(ButtonVariant::Text)
                            .text_color(egui::Color32::from_gray(160))
                            .hover_text_color(egui::Color32::WHITE)
                            .show(ui)
                            .clicked()
                        {
                            #[cfg(target_arch = "wasm32")]
                            {
                                if let Some(window) = web_sys::window() {
                                    let _ = window
                                        .open_with_url("https://github.com/yourusername/armas");
                                }
                            }
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                let _ = open::that("https://github.com/yourusername/armas");
                            }
                        }

                        ui.add_space(16.0);

                        Input::new("Search...")
                            .with_left_icon("🔍")
                            .with_width(showcase_sizes::SEARCH_WIDTH_MAX)
                            .variant(InputVariant::Filled)
                            .show(ui, search_text);

                        ui.add_space(16.0);
                    });
                });
            });

        Divider::horizontal().show(ui);
    }
}
