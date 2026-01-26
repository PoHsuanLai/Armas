//! Hero section component matching shadcn style

use armas::*;
use eframe::egui;

pub struct SiteHero<'a> {
    theme: &'a Theme,
}

pub struct SiteHeroResponse {
    pub get_started_clicked: bool,
    pub components_clicked: bool,
}

impl<'a> SiteHero<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }

    pub fn show(self, ui: &mut egui::Ui) -> SiteHeroResponse {
        let mut response = SiteHeroResponse {
            get_started_clicked: false,
            components_clicked: false,
        };

        let rect = ui.available_rect_before_wrap();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(rect.width());

            let content_width = 600.0f32.min(rect.width() - 48.0);
            let margin = (rect.width() - content_width) / 2.0;

            ui.add_space(margin.max(24.0));

            ui.vertical(|ui| {
                ui.set_max_width(content_width);

                ui.add_space(120.0);

                // Title
                ui.label(
                    egui::RichText::new("Build your component library")
                        .size(48.0)
                        .family(egui::FontFamily::Name("InterBold".into()))
                        .color(self.theme.foreground()),
                );

                ui.add_space(16.0);

                // Subtitle
                ui.label(
                    egui::RichText::new(
                        "Beautiful, accessible components for egui. \
                         Copy and paste into your apps. Open source.",
                    )
                    .size(18.0)
                    .color(self.theme.muted_foreground()),
                );

                ui.add_space(32.0);

                // Buttons
                ui.horizontal(|ui| {
                    if Button::new("Get Started")
                        .variant(ButtonVariant::Default)
                        .show(ui, self.theme)
                        .clicked()
                    {
                        response.get_started_clicked = true;
                    }

                    ui.add_space(8.0);

                    if Button::new("Components")
                        .variant(ButtonVariant::Outline)
                        .show(ui, self.theme)
                        .clicked()
                    {
                        response.components_clicked = true;
                    }
                });

                ui.add_space(120.0);
            });
        });

        response
    }
}
