use armas::ext::ArmasContextExt;
use armas::{GradientCard, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_title("Gradient Card Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Gradient Card Demo",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(GradientCardApp::new()))
        }),
    )
}

struct GradientCardApp {
}

impl GradientCardApp {
    fn new() -> Self {
        Self {
        }
    }
}

impl eframe::App for GradientCardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let theme = ctx.armas_theme();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Gradient Card Component");
            ui.add_space(20.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Row 1
                ui.horizontal(|ui| {
                    GradientCard::new()
                        .width(280.0)
                        .height(200.0)
                        .show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(70.0);
                                ui.heading("Default");
                                ui.label("Blue-Purple");
                            });
                        });

                    ui.add_space(20.0);

                    GradientCard::rainbow().width(280.0).height(200.0).show(
                        ui,
                        &self.theme,
                        |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(70.0);
                                ui.heading("Rainbow");
                                ui.label("Full Spectrum");
                            });
                        },
                    );

                    ui.add_space(20.0);

                    GradientCard::warm()
                        .width(280.0)
                        .height(200.0)
                        .show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(70.0);
                                ui.heading("Warm");
                                ui.label("Red-Orange-Yellow");
                            });
                        });
                });

                ui.add_space(20.0);

                // Row 2
                ui.horizontal(|ui| {
                    GradientCard::cool()
                        .width(280.0)
                        .height(200.0)
                        .show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(70.0);
                                ui.heading("Cool");
                                ui.label("Cyan-Blue-Purple");
                            });
                        });

                    ui.add_space(20.0);

                    GradientCard::neon()
                        .width(280.0)
                        .height(200.0)
                        .show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(70.0);
                                ui.heading("Neon");
                                ui.label("Bright & Bold");
                            });
                        });

                    ui.add_space(20.0);

                    GradientCard::gold()
                        .width(280.0)
                        .height(200.0)
                        .show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(70.0);
                                ui.heading("Gold");
                                ui.label("Premium");
                            });
                        });
                });

                ui.add_space(20.0);

                // Row 3 - Animation speeds
                ui.horizontal(|ui| {
                    GradientCard::new()
                        .width(280.0)
                        .height(200.0)
                        .rotation_speed(std::f32::consts::PI / 8.0)
                        .show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(70.0);
                                ui.heading("Slow");
                                ui.label("π/8 rad/s");
                            });
                        });

                    ui.add_space(20.0);

                    GradientCard::new()
                        .width(280.0)
                        .height(200.0)
                        .rotation_speed(std::f32::consts::PI)
                        .show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(70.0);
                                ui.heading("Fast");
                                ui.label("π rad/s");
                            });
                        });

                    ui.add_space(20.0);

                    GradientCard::new()
                        .width(280.0)
                        .height(200.0)
                        .animate(false)
                        .show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(70.0);
                                ui.heading("Static");
                                ui.label("No Animation");
                            });
                        });
                });

                ui.add_space(20.0);

                // Row 4 - Customization
                ui.horizontal(|ui| {
                    GradientCard::new()
                        .width(280.0)
                        .height(200.0)
                        .border_width(4.0)
                        .corner_radius(12.0)
                        .show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(70.0);
                                ui.heading("Thick Border");
                                ui.label("4px border");
                            });
                        });

                    ui.add_space(20.0);

                    GradientCard::new()
                        .width(280.0)
                        .height(200.0)
                        .gradient_colors(vec![
                            egui::Color32::from_rgb(16, 185, 129),
                            egui::Color32::from_rgb(5, 150, 105),
                            egui::Color32::from_rgb(4, 120, 87),
                        ])
                        .show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(70.0);
                                ui.heading("Custom");
                                ui.label("Emerald Green");
                            });
                        });
                });
            });
        });
    }
}
