//! Variant card for displaying component variants in a grid

#![allow(dead_code)]

use armas::*;
use eframe::egui;

/// Card for displaying a single component variant
pub struct VariantCard {
    title: String,
    code_preview: Option<String>,
    min_height: f32,
}

impl VariantCard {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            code_preview: None,
            min_height: 180.0,
        }
    }

    pub fn code_preview(mut self, code: impl Into<String>) -> Self {
        self.code_preview = Some(code.into());
        self
    }

    pub fn min_height(mut self, height: f32) -> Self {
        self.min_height = height;
        self
    }

    pub fn show<R>(
        self,
        ui: &mut egui::Ui,
        content: impl FnOnce(&mut egui::Ui) -> R,
    ) -> (egui::Response, R) {
        let theme = ui.ctx().armas_theme();
        let card_result = GlassPanel::new()
            .blur(10.0)
            .opacity(0.05)
            .corner_radius(12.0)
            .show(ui, &theme, |ui| {
                ui.set_min_height(self.min_height);

                // Manual vertical layout with spacing to preserve inner R
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 16.0;

                    // Demo area (centered)
                    let result = ui
                        .vertical_centered(|ui| {
                            ui.add_space(20.0);
                            let result = content(ui);
                            ui.add_space(20.0);
                            result
                        })
                        .inner;

                    ui.separator();

                    // Info section
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 4.0;
                        ui.strong(&self.title);

                        if let Some(code) = &self.code_preview {
                            ui.label(
                                egui::RichText::new(code)
                                    .monospace()
                                    .size(11.0)
                                    .color(theme.muted_foreground()),
                            );
                        }
                    });

                    result
                })
                .inner
            });

        (card_result.response, card_result.inner)
    }
}

/// Simple grid layout helper
pub fn variant_grid<R>(
    ui: &mut egui::Ui,
    columns: usize,
    gap: f32,
    content: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    let _ = columns; // Reserved for future grid layout implementation
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = gap;
        content(ui)
    })
    .inner
}

/// Helper to create a grid row
pub fn grid_row(ui: &mut egui::Ui, gap: f32, content: impl FnOnce(&mut egui::Ui)) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        content(ui);
    });
}
