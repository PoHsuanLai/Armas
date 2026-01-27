//! Site header component matching shadcn style

use armas::*;
use eframe::egui;

pub struct SiteHeader<'a> {
    theme: &'a Theme,
    is_dark: bool,
}

pub struct SiteHeaderResponse {
    pub logo_clicked: bool,
    pub docs_clicked: bool,
    pub components_clicked: bool,
    pub github_clicked: bool,
    pub crates_io_clicked: bool,
    pub theme_toggle_clicked: bool,
    pub hamburger_clicked: bool,
}

impl<'a> SiteHeader<'a> {
    pub fn new(theme: &'a Theme, is_dark: bool) -> Self {
        Self { theme, is_dark }
    }

    pub fn show(self, ui: &mut egui::Ui) -> SiteHeaderResponse {
        let width = ui.available_width();
        let is_mobile = width < 768.0;

        let mut response = SiteHeaderResponse {
            logo_clicked: false,
            docs_clicked: false,
            components_clicked: false,
            github_clicked: false,
            crates_io_clicked: false,
            theme_toggle_clicked: false,
            hamburger_clicked: false,
        };

        egui::Frame::new()
            .fill(self.theme.background())
            .inner_margin(egui::Margin::symmetric(24, 0))
            .show(ui, |ui| {
                ui.set_height(48.0);
                ui.set_width(width);

                ui.horizontal_centered(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;

                    // Logo text
                    let logo_response = ui.add(
                        egui::Label::new(
                            egui::RichText::new("Armas")
                                .size(16.0)
                                .family(egui::FontFamily::Name("InterBold".into()))
                                .color(self.theme.foreground()),
                        )
                        .sense(egui::Sense::click()),
                    );

                    if logo_response.clicked() {
                        response.logo_clicked = true;
                    }
                    if logo_response.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }

                    ui.add_space(8.0);

                    // Desktop nav
                    if !is_mobile {
                        if Button::new("Docs")
                            .variant(ButtonVariant::Ghost)
                            .size(ButtonSize::Small)
                            .show(ui, self.theme)
                            .clicked()
                        {
                            response.docs_clicked = true;
                        }

                        if Button::new("Components")
                            .variant(ButtonVariant::Ghost)
                            .size(ButtonSize::Small)
                            .show(ui, self.theme)
                            .clicked()
                        {
                            response.components_clicked = true;
                        }
                    }

                    // Spacer
                    let right_width = if is_mobile { 80.0 } else { 200.0 };
                    ui.add_space(ui.available_width() - right_width);

                    // Right side
                    if is_mobile {
                        if Button::new("☰")
                            .variant(ButtonVariant::Ghost)
                            .size(ButtonSize::Small)
                            .show(ui, self.theme)
                            .clicked()
                        {
                            response.hamburger_clicked = true;
                        }
                    } else {
                        Self::render_separator(ui, self.theme);

                        if Button::new("GitHub")
                            .variant(ButtonVariant::Ghost)
                            .size(ButtonSize::Small)
                            .show(ui, self.theme)
                            .clicked()
                        {
                            response.github_clicked = true;
                        }

                        if Button::new("Crates.io")
                            .variant(ButtonVariant::Ghost)
                            .size(ButtonSize::Small)
                            .show(ui, self.theme)
                            .clicked()
                        {
                            response.crates_io_clicked = true;
                        }

                        Self::render_separator(ui, self.theme);

                        let icon = if self.is_dark { "☀" } else { "☾" };
                        if Button::new(icon)
                            .variant(ButtonVariant::Ghost)
                            .size(ButtonSize::Small)
                            .show(ui, self.theme)
                            .clicked()
                        {
                            response.theme_toggle_clicked = true;
                        }
                    }
                });
            });

        response
    }

    fn render_separator(ui: &mut egui::Ui, theme: &Theme) {
        ui.add_space(8.0);
        let rect = ui.allocate_space(egui::vec2(1.0, 16.0)).1;
        ui.painter().rect_filled(rect, 0.0, theme.border());
        ui.add_space(8.0);
    }
}
