//! Component demo card with glowing border and glass effect

use armas::*;
use eframe::egui;

/// Card for displaying component demos with premium aesthetic
pub struct ComponentDemoCard {
    min_height: f32,
    show_glow: bool,
    dark_background: bool,
}

impl ComponentDemoCard {
    pub fn new() -> Self {
        Self {
            min_height: 400.0,
            show_glow: true,
            dark_background: true,
        }
    }

    pub fn min_height(mut self, height: f32) -> Self {
        self.min_height = height;
        self
    }

    pub fn glow(mut self, show: bool) -> Self {
        self.show_glow = show;
        self
    }

    pub fn dark_background(mut self, dark: bool) -> Self {
        self.dark_background = dark;
        self
    }

    pub fn show<R>(
        self,
        ui: &mut egui::Ui,
        content: impl FnOnce(&mut egui::Ui) -> R,
    ) -> R {
        // For now, skip the glowing border since it complicates the return type
        // TODO: Fix GlowingBorder to return inner result
        self.show_inner(ui, content)
    }

    fn show_inner<R>(
        &self,
        ui: &mut egui::Ui,
        content: impl FnOnce(&mut egui::Ui) -> R,
    ) -> R {
        let theme = ui.ctx().armas_theme();
        let panel_result = GlassPanel::new()
            .blur(10.0)
            .opacity(0.05)
            .corner_radius(16.0)
            .show(ui, &theme, |ui| {
                ui.set_min_height(self.min_height);

                // Optional dark background overlay
                if self.dark_background {
                    let rect = ui.available_rect_before_wrap();
                    ui.painter().rect_filled(
                        rect,
                        16.0,
                        egui::Color32::from_rgba_unmultiplied(10, 10, 10, 240),
                    );
                }

                ui.vertical_centered(|ui| {
                    ui.add_space(self.min_height * 0.3);
                    content(ui)
                })
                .inner
            });

        panel_result.inner
    }
}

impl Default for ComponentDemoCard {
    fn default() -> Self {
        Self::new()
    }
}
