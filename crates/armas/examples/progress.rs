use armas::{CircularProgressBar, LinearProgress, RingProgress, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 700.0])
            .with_title("Progress Indicators Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Progress Indicators Demo",
        options,
        Box::new(|_cc| Ok(Box::new(ProgressApp::new()))),
    )
}

struct ProgressApp {
    theme: Theme,
    progress: f32,
}

impl ProgressApp {
    fn new() -> Self {
        Self {
            theme: Theme::dark(),
            progress: 0.65,
        }
    }
}

impl eframe::App for ProgressApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Progress Indicators");
            ui.add_space(20.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Linear Progress
                ui.heading("Linear Progress");
                ui.add_space(10.0);

                ui.label("Determinate:");
                LinearProgress::new(self.progress).show(ui, &self.theme);

                ui.add_space(10.0);

                ui.label("With label:");
                LinearProgress::new(self.progress)
                    .show_label()
                    .show(ui, &self.theme);

                ui.add_space(10.0);

                ui.label("Custom color:");
                LinearProgress::new(self.progress)
                    .color(egui::Color32::from_rgb(34, 197, 94))
                    .height(6.0)
                    .show(ui, &self.theme);

                ui.add_space(10.0);

                ui.label("Indeterminate (loading):");
                LinearProgress::indeterminate().show(ui, &self.theme);

                ui.add_space(30.0);

                // Circular Progress
                ui.heading("Circular Progress");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Basic");
                        CircularProgressBar::new(self.progress)
                            .size(80.0)
                            .show(ui, &self.theme);
                    });

                    ui.add_space(30.0);

                    ui.vertical(|ui| {
                        ui.label("With percentage");
                        CircularProgressBar::new(self.progress)
                            .size(80.0)
                            .show_percentage(true)
                            .show(ui, &self.theme);
                    });

                    ui.add_space(30.0);

                    ui.vertical(|ui| {
                        ui.label("Custom color");
                        CircularProgressBar::new(self.progress)
                            .size(80.0)
                            .color(egui::Color32::from_rgb(168, 85, 247))
                            .show_percentage(true)
                            .show(ui, &self.theme);
                    });

                    ui.add_space(30.0);

                    ui.vertical(|ui| {
                        ui.label("Indeterminate");
                        CircularProgressBar::indeterminate()
                            .size(60.0)
                            .show(ui, &self.theme);
                    });
                });

                ui.add_space(30.0);

                // Ring Progress
                ui.heading("Ring Progress");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    RingProgress::new(0.75)
                        .size(120.0)
                        .label("Storage")
                        .show(ui, &self.theme);

                    ui.add_space(30.0);

                    RingProgress::new(0.42)
                        .size(120.0)
                        .label("Memory")
                        .color(egui::Color32::from_rgb(251, 191, 36))
                        .show(ui, &self.theme);

                    ui.add_space(30.0);

                    RingProgress::new(0.88)
                        .size(120.0)
                        .label("CPU")
                        .color(egui::Color32::from_rgb(239, 68, 68))
                        .show(ui, &self.theme);
                });

                ui.add_space(30.0);

                // Controls
                ui.heading("Controls");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("Reset").clicked() {
                        self.progress = 0.0;
                    }
                    if ui.button("25%").clicked() {
                        self.progress = 0.25;
                    }
                    if ui.button("50%").clicked() {
                        self.progress = 0.5;
                    }
                    if ui.button("75%").clicked() {
                        self.progress = 0.75;
                    }
                    if ui.button("100%").clicked() {
                        self.progress = 1.0;
                    }
                });

                ui.add_space(10.0);

                ui.add(egui::Slider::new(&mut self.progress, 0.0..=1.0).text("Progress"));
            });
        });
    }
}
