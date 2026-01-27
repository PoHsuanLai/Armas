//! Hero section component matching shadcn style

use armas::*;
use armas_animated::DotPattern;
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
        let is_mobile = rect.width() < 768.0;

        // Paint dot pattern as background (no layout space consumed)
        DotPattern::new()
            .spacing(24.0)
            .dot_radius(1.0)
            .fade(0.8)
            .paint(ui, rect);

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(rect.width());

            let content_width = 600.0f32.min(rect.width() - 48.0);
            let margin = (rect.width() - content_width) / 2.0;

            ui.add_space(margin.max(24.0));

            ui.vertical(|ui| {
                ui.set_max_width(content_width);

                // Push content toward vertical center
                let content_height = 200.0; // approximate: title + subtitle + buttons
                let top_space = ((rect.height() - content_height) / 2.0).max(if is_mobile {
                    60.0
                } else {
                    120.0
                });
                ui.add_space(top_space);

                // Title
                let title_size = if is_mobile { 32.0 } else { 48.0 };
                ui.label(
                    egui::RichText::new("Build your component library")
                        .size(title_size)
                        .family(egui::FontFamily::Name("InterBold".into()))
                        .color(self.theme.foreground()),
                );

                ui.add_space(16.0);

                // Subtitle
                let subtitle_size = if is_mobile { 16.0 } else { 18.0 };
                ui.label(
                    egui::RichText::new(
                        "Beautiful, accessible components for egui. \
                         Copy and paste into your apps. Open source.",
                    )
                    .size(subtitle_size)
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

                let bottom_space = if is_mobile { 60.0 } else { 120.0 };
                ui.add_space(bottom_space);
            });
        });

        response
    }
}
