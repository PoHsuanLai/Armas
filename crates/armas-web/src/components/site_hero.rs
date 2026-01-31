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

/// Paint a dot pattern background with radial fade from center
fn paint_dot_pattern(ui: &egui::Ui, rect: egui::Rect, theme: &Theme) {
    let color = theme.border();
    let spacing = 24.0f32;
    let dot_radius = 1.0f32;
    let fade_distance = 0.8f32;
    let center = rect.center();
    let painter = ui.painter();

    let start_x = rect.left() + (rect.width() % spacing) / 2.0;
    let start_y = rect.top() + (rect.height() % spacing) / 2.0;
    let num_cols = (rect.width() / spacing).ceil() as usize;
    let num_rows = (rect.height() / spacing).ceil() as usize;

    for row in 0..num_rows {
        let y = (row as f32).mul_add(spacing, start_y);
        if y > rect.bottom() {
            break;
        }
        for col in 0..num_cols {
            let x = (col as f32).mul_add(spacing, start_x);
            if x > rect.right() {
                break;
            }
            let dot_pos = egui::pos2(x, y);
            let distance_x = (dot_pos.x - center.x).abs() / (rect.width() / 2.0);
            let distance_y = (dot_pos.y - center.y).abs() / (rect.height() / 2.0);
            let distance = distance_x.hypot(distance_y);
            let fade_factor = 1.0 - (distance / fade_distance).min(1.0);
            let alpha = (f32::from(color.a()) * fade_factor) as u8;
            let dot_color =
                egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);
            painter.circle_filled(dot_pos, dot_radius, dot_color);
        }
    }
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
        paint_dot_pattern(ui, rect, self.theme);

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
