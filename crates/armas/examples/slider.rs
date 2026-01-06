use armas::{Slider, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([700.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Slider Component Example",
        options,
        Box::new(|_cc| Ok(Box::new(SliderExample::default()))),
    )
}

struct SliderExample {
    theme: Theme,
    volume: f32,
    brightness: f32,
    temperature: f32,
    speed: f32,
    zoom: f32,
    opacity: f32,
}

impl Default for SliderExample {
    fn default() -> Self {
        Self {
            theme: Theme::dark(),
            volume: 75.0,
            brightness: 50.0,
            temperature: 20.0,
            speed: 1.0,
            zoom: 100.0,
            opacity: 1.0,
        }
    }
}

impl eframe::App for SliderExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Slider Component Examples");
            ui.add_space(20.0);

            // Basic sliders with labels
            ui.label("Basic Sliders:");
            ui.add_space(10.0);

            let response = Slider::new(self.volume, 0.0, 100.0)
                .label("Volume")
                .suffix("%")
                .width(300.0)
                .show(ui, &self.theme);

            if response.changed {
                self.volume = response.value;
            }
            ui.add_space(10.0);

            let response = Slider::new(self.brightness, 0.0, 100.0)
                .label("Brightness")
                .suffix("%")
                .width(300.0)
                .show(ui, &self.theme);

            if response.changed {
                self.brightness = response.value;
            }
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Different value ranges
            ui.label("Different Value Ranges:");
            ui.add_space(10.0);

            let response = Slider::new(self.temperature, -10.0, 40.0)
                .label("Temperature")
                .suffix("°C")
                .width(400.0)
                .show(ui, &self.theme);

            if response.changed {
                self.temperature = response.value;
            }
            ui.add_space(10.0);

            let response = Slider::new(self.speed, 0.0, 10.0)
                .label("Playback Speed")
                .suffix("x")
                .width(400.0)
                .show(ui, &self.theme);

            if response.changed {
                self.speed = response.value;
            }
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // With step values
            ui.label("Slider with Step (snaps to 10):");
            ui.add_space(10.0);

            let response = Slider::new(self.zoom, 50.0, 200.0)
                .label("Zoom Level")
                .suffix("%")
                .step(10.0)
                .width(400.0)
                .show(ui, &self.theme);

            if response.changed {
                self.zoom = response.value;
            }
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Different widths
            ui.label("Different Widths:");
            ui.add_space(10.0);

            Slider::new(50.0, 0.0, 100.0)
                .label("Short Slider")
                .width(200.0)
                .show(ui, &self.theme);

            ui.add_space(10.0);

            Slider::new(50.0, 0.0, 100.0)
                .label("Medium Slider")
                .width(400.0)
                .show(ui, &self.theme);

            ui.add_space(10.0);

            Slider::new(50.0, 0.0, 100.0)
                .label("Long Slider")
                .width(600.0)
                .show(ui, &self.theme);

            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Without value label
            ui.label("Slider Without Value Display:");
            ui.add_space(10.0);

            let response = Slider::new(self.opacity, 0.0, 1.0)
                .label("Opacity")
                .show_value(false)
                .width(300.0)
                .show(ui, &self.theme);

            if response.changed {
                self.opacity = response.value;
            }
            ui.label(format!("Current opacity: {:.2}", self.opacity));

            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Without label
            ui.label("Slider Without Label:");
            ui.add_space(10.0);

            Slider::new(50.0, 0.0, 100.0)
                .width(300.0)
                .show(ui, &self.theme);
        });
    }
}
