//! Hero section for landing page with animated background

use armas::*;
use eframe::egui;

/// Hero section with animated background and CTA buttons
pub struct HeroSection {
    title: String,
    tagline: String,
    show_background_effect: bool,
}

impl HeroSection {
    pub fn new(title: impl Into<String>, tagline: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            tagline: tagline.into(),
            show_background_effect: true,
        }
    }

    pub fn show(
        self,
        ui: &mut egui::Ui,
        on_get_started: impl FnOnce(),
        on_view_docs: impl FnOnce(),
    ) -> egui::Response {
        let mut get_started_clicked = false;
        let mut view_docs_clicked = false;

        // Allocate space for hero section
        let available_height = ui.available_height().max(600.0);
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(ui.available_width(), available_height),
            egui::Sense::hover(),
        );

        // Draw background effect
        if self.show_background_effect {
            // Simple gradient background (could be replaced with Aurora/Vortex)
            let painter = ui.painter();

            // Dark gradient from top to bottom
            let top_color = egui::Color32::from_rgb(15, 15, 25);

            painter.rect_filled(rect, 0.0, top_color);

            // Add subtle radial gradient in center
            let center = rect.center();
            let max_radius = rect.width().max(rect.height());

            for i in (0..20).rev() {
                let radius = (i as f32 / 20.0) * max_radius * 0.5;
                let alpha = (i as f32 / 20.0 * 30.0) as u8;
                let color = egui::Color32::from_rgba_unmultiplied(100, 80, 200, alpha);
                painter.circle_filled(center, radius, color);
            }
        }

        // Content overlay
        let _ = ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 48.0;
                ui.add_space(available_height * 0.25);

                // Title with gradient effect
                ui.vertical_centered(|ui| {
                    // Main title
                    ui.label(
                        egui::RichText::new(&self.title)
                            .size(64.0)
                            .strong()
                            .color(egui::Color32::WHITE),
                    );

                    ui.add_space(12.0);

                    // Tagline
                    ui.label(
                        egui::RichText::new(&self.tagline)
                            .size(20.0)
                            .color(egui::Color32::from_rgb(180, 180, 200)),
                    );

                    ui.add_space(16.0);

                    // Subtitle
                    ui.label(
                        egui::RichText::new(
                            "Build beautiful UIs with Material Design inspired components",
                        )
                        .size(16.0)
                        .color(egui::Color32::from_rgb(140, 140, 160)),
                    );

                    ui.add_space(32.0);

                    // CTA Buttons
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 16.0;

                        if ShimmerButton::new("Get Started")
                            .min_size(egui::vec2(140.0, 48.0))
                            .show(ui)
                            .clicked()
                        {
                            get_started_clicked = true;
                        }

                        if FigmaButton::outlined("View Docs")
                            .min_size(egui::vec2(140.0, 48.0))
                            .show(ui)
                            .clicked()
                        {
                            view_docs_clicked = true;
                        }
                    });
                });

                ui.add_space(64.0);

                // Feature highlights
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 48.0;

                        for (icon, text) in [
                            ("🎨", "70+ Components"),
                            ("✨", "Smooth Animations"),
                            ("🎯", "Material Design"),
                            ("📱", "Responsive Layout"),
                        ] {
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new(icon).size(32.0));
                                ui.add_space(8.0);
                                ui.label(
                                    egui::RichText::new(text)
                                        .size(14.0)
                                        .color(egui::Color32::from_rgb(180, 180, 200)),
                                );
                            });
                        }
                    });
                });
            });
        });

        // Trigger callbacks
        if get_started_clicked {
            on_get_started();
        }
        if view_docs_clicked {
            on_view_docs();
        }

        response
    }
}

/// Feature showcase section for home page
pub struct FeatureShowcase {
    features: Vec<FeatureShowcaseItem>,
}

#[derive(Clone)]
pub struct FeatureShowcaseItem {
    pub title: String,
    pub description: String,
    pub icon: String,
}

impl FeatureShowcase {
    pub fn new(features: Vec<FeatureShowcaseItem>) -> Self {
        Self { features }
    }

    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = ui.ctx().armas_theme();
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 32.0;
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("Features")
                        .size(32.0)
                        .strong()
                        .color(theme.on_surface()),
                );

                ui.add_space(16.0);

                // Feature grid using egui::Grid
                egui::Grid::new("feature_showcase_grid")
                    .spacing([24.0, 24.0])
                    .show(ui, |ui| {
                        let mut col_count = 0;
                        for feature in self.features {
                            if col_count >= 3 {
                                ui.end_row();
                                col_count = 0;
                            }

                            Card::new().hover_effect(true).show(ui, &theme, |ui| {
                                ui.set_min_size(egui::vec2(200.0, 160.0));

                                ui.vertical(|ui| {
                                    ui.spacing_mut().item_spacing.y = 12.0;
                                    ui.label(egui::RichText::new(&feature.icon).size(36.0));

                                    ui.strong(&feature.title);

                                    ui.label(
                                        egui::RichText::new(&feature.description)
                                            .size(13.0)
                                            .color(theme.on_surface_variant()),
                                    );
                                });
                            });

                            col_count += 1;
                        }
                    });
            });
        });

        ui.interact(ui.min_rect(), ui.id(), egui::Sense::hover())
    }
}
