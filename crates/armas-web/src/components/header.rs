//! Top header/navbar component for the showcase

use crate::{layout, showcase_sizes};
use armas::*;
use eframe::egui;

// Include logo at compile time
const LOGO_BYTES: &[u8] = include_bytes!("../../assets/images/logo.png");

pub struct Header;

impl Header {
    pub fn show(ui: &mut egui::Ui, search_text: &mut String, sidebar_open: &mut bool) -> bool {
        let theme = ui.ctx().armas_theme();
        let mut hamburger_clicked = false;

        GlassPanel::new()
            .blur(10.0)
            .opacity(0.05)
            .inner_margin(0.0)
            .show(ui, &theme, |ui| {
                let total_height = showcase_sizes::HEADER_HEIGHT;
                ui.set_height(total_height);

                let full_rect = ui.available_rect_before_wrap();
                let screen_width = full_rect.width();

                // Determine layout based on screen width
                let is_mobile = screen_width < layout::MOBILE_BREAKPOINT;
                let is_tablet = (layout::MOBILE_BREAKPOINT..layout::DESKTOP_BREAKPOINT).contains(&screen_width);

                // Left side: Logo (always visible)
                let left_rect =
                    egui::Rect::from_min_size(full_rect.min, egui::vec2(200.0, total_height));

                let _ = ui.scope_builder(egui::UiBuilder::new().max_rect(left_rect), |ui| {
                    ui.set_height(total_height);
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(24.0);

                        // Load and display logo image
                        let logo_image = Self::load_logo_image();
                        let logo_texture = ui.ctx().load_texture(
                            "armas_logo",
                            logo_image.clone(),
                            egui::TextureOptions::LINEAR,
                        );

                        // Calculate size maintaining aspect ratio with max height of 22px
                        let aspect_ratio = logo_image.width() as f32 / logo_image.height() as f32;
                        let logo_height = 22.0;
                        let logo_width = logo_height * aspect_ratio;
                        let logo_size = egui::vec2(logo_width, logo_height);

                        // Make logo clickable
                        let logo_response = ui.add(
                            egui::Image::new(&logo_texture)
                                .max_size(logo_size)
                                .sense(egui::Sense::click())
                        );

                        if logo_response.clicked() {
                            Self::open_website();
                        }

                        if logo_response.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }

                        // Make text label clickable too
                        let text_response = ui.add(egui::Label::new(
                            egui::RichText::new("Armas")
                                .size(20.0)
                                .family(egui::FontFamily::Name("InterBold".into()))
                                .color(egui::Color32::WHITE)
                        ).sense(egui::Sense::click()));

                        if text_response.clicked() {
                            Self::open_website();
                        }

                        if text_response.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                    });
                });

                // Right side: Responsive content
                let right_side_width = if is_mobile {
                    150.0 // Mobile: Smaller search + hamburger
                } else if is_tablet {
                    400.0 // Tablet: Full header with smaller search
                } else {
                    500.0 // Desktop: Full header with larger search
                };

                let right_rect = egui::Rect::from_min_size(
                    egui::pos2(full_rect.max.x - right_side_width, full_rect.min.y),
                    egui::vec2(right_side_width, total_height),
                );

                let _ = ui.scope_builder(egui::UiBuilder::new().max_rect(right_rect), |ui| {
                    ui.set_height(total_height);
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        // Mobile: Small search + hamburger
                        if is_mobile {
                            Input::new("Search...")
                                .left_icon("🔍")
                                .width(100.0)
                                .variant(InputVariant::Filled)
                                .show(ui, search_text);

                            ui.add_space(8.0);

                            if Button::new("☰")
                                .variant(ButtonVariant::Text)
                                .text_color(egui::Color32::from_gray(160))
                                .hover_text_color(egui::Color32::WHITE)
                                .show(ui)
                                .clicked()
                            {
                                *sidebar_open = !*sidebar_open;
                                hamburger_clicked = true;
                            }

                            ui.add_space(32.0); // Match content panel's right padding
                        }
                        // Tablet & Desktop: Full header (Components + GitHub + Search)
                        else {
                            if Button::new("Components")
                                .variant(ButtonVariant::Text)
                                .text_color(egui::Color32::from_gray(160))
                                .hover_text_color(egui::Color32::WHITE)
                                .show(ui)
                                .clicked()
                            {
                                // Navigate to components page
                                ui.data_mut(|d| d.insert_temp(egui::Id::new("current_view"), "components".to_string()));
                            }

                            ui.add_space(16.0);

                            if Button::new("GitHub")
                                .variant(ButtonVariant::Text)
                                .text_color(egui::Color32::from_gray(160))
                                .hover_text_color(egui::Color32::WHITE)
                                .show(ui)
                                .clicked()
                            {
                                Self::open_github();
                            }

                            ui.add_space(16.0);

                            // Tablet: Smaller search box, Desktop: Larger search box
                            let search_width = if is_tablet {
                                showcase_sizes::SEARCH_WIDTH_MIN
                            } else {
                                showcase_sizes::SEARCH_WIDTH_MAX
                            };

                            Input::new("Search...")
                                .left_icon("🔍")
                                .width(search_width)
                                .variant(InputVariant::Filled)
                                .show(ui, search_text);

                            ui.add_space(32.0); // Match content panel's right padding
                        }
                    });
                });
            });

        ui.separator();
        hamburger_clicked
    }

    fn load_logo_image() -> egui::ColorImage {
        let image = image::load_from_memory(LOGO_BYTES)
            .expect("Failed to load logo image")
            .to_rgba8();
        let size = [image.width() as usize, image.height() as usize];
        let pixels = image.as_flat_samples();
        egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice())
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

    fn open_website() {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                let _ = window.open_with_url("https://pohsuanlai.github.io/Armas/");
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = open::that("https://pohsuanlai.github.io/Armas/");
        }
    }
}
