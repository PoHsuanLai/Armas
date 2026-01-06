use armas::{Animation, EasingFunction, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Animation Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Animation Demo",
        options,
        Box::new(|_cc| Ok(Box::new(AnimationDemoApp::new()))),
    )
}

struct AnimationDemoApp {
    theme: Theme,
    // Position animation
    pos_anim: Animation<egui::Pos2>,
    // Color animation
    color_anim: Animation<egui::Color32>,
    // Size animation
    size_anim: Animation<f32>,
    // Multiple animations with different easing
    ease_anims: Vec<(String, Animation<f32>)>,
    // Last frame time for delta calculation
    last_time: f64,
}

impl AnimationDemoApp {
    fn new() -> Self {
        let mut app = Self {
            theme: Theme::dark(),
            pos_anim: Animation::new(
                egui::Pos2::new(100.0, 100.0),
                egui::Pos2::new(700.0, 100.0),
                2.0,
            ),
            color_anim: Animation::new(
                egui::Color32::from_rgb(59, 130, 246), // Blue
                egui::Color32::from_rgb(239, 68, 68),  // Red
                2.0,
            ),
            size_anim: Animation::new(20.0, 60.0, 2.0),
            ease_anims: vec![],
            last_time: 0.0,
        };

        // Create animations with different easing functions
        let easing_functions = vec![
            ("Linear", EasingFunction::Linear),
            ("EaseIn", EasingFunction::EaseIn),
            ("EaseOut", EasingFunction::EaseOut),
            ("EaseInOut", EasingFunction::EaseInOut),
            ("CubicIn", EasingFunction::CubicIn),
            ("CubicOut", EasingFunction::CubicOut),
            ("CubicInOut", EasingFunction::CubicInOut),
            ("ElasticOut", EasingFunction::ElasticOut),
            ("BounceOut", EasingFunction::BounceOut),
        ];

        for (name, easing) in easing_functions {
            let anim = Animation::new(0.0, 500.0, 2.0).with_easing(easing);
            app.ease_anims.push((name.to_string(), anim));
        }

        app
    }

    fn restart_animations(&mut self) {
        self.pos_anim.start();
        self.color_anim.start();
        self.size_anim.start();

        for (_, anim) in &mut self.ease_anims {
            anim.start();
        }
    }
}

impl eframe::App for AnimationDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Calculate delta time
        let current_time = ctx.input(|i| i.time);
        let dt = if self.last_time == 0.0 {
            0.016 // Assume 60 FPS for first frame
        } else {
            (current_time - self.last_time) as f32
        };
        self.last_time = current_time;

        // Update animations
        self.pos_anim.update(dt);
        self.color_anim.update(dt);
        self.size_anim.update(dt);

        for (_, anim) in &mut self.ease_anims {
            anim.update(dt);
        }

        // Request repaint if any animation is running
        if self.pos_anim.is_running()
            || self.color_anim.is_running()
            || self.size_anim.is_running()
            || self.ease_anims.iter().any(|(_, a)| a.is_running())
        {
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎨 Animation System Demo");
            ui.add_space(10.0);

            // Control buttons
            ui.horizontal(|ui| {
                if ui.button("▶ Start").clicked() {
                    self.restart_animations();
                }

                if ui.button("⏸ Pause").clicked() {
                    self.pos_anim.pause();
                    self.color_anim.pause();
                    self.size_anim.pause();
                    for (_, anim) in &mut self.ease_anims {
                        anim.pause();
                    }
                }

                if ui.button("▶▶ Resume").clicked() {
                    self.pos_anim.resume();
                    self.color_anim.resume();
                    self.size_anim.resume();
                    for (_, anim) in &mut self.ease_anims {
                        anim.resume();
                    }
                }

                if ui.button("⏹ Reset").clicked() {
                    self.pos_anim.reset();
                    self.color_anim.reset();
                    self.size_anim.reset();
                    for (_, anim) in &mut self.ease_anims {
                        anim.reset();
                    }
                }
            });

            ui.add_space(20.0);

            // Combined animation demo
            ui.group(|ui| {
                ui.label("Combined Animation (Position + Color + Size)");
                ui.add_space(10.0);

                let pos = self.pos_anim.value();
                let color = self.color_anim.value();
                let size = self.size_anim.value();

                let painter = ui.painter();
                painter.circle_filled(pos, size, color);

                ui.add_space(80.0);
                ui.label(format!(
                    "Progress: {:.1}%",
                    self.pos_anim.progress() * 100.0
                ));
            });

            ui.add_space(20.0);

            // Easing function comparison
            ui.group(|ui| {
                ui.label("Easing Functions Comparison");
                ui.add_space(10.0);

                for (i, (name, anim)) in self.ease_anims.iter().enumerate() {
                    let y = 300.0 + i as f32 * 25.0;
                    let start_x = 100.0;
                    let current_x = start_x + anim.value();

                    let painter = ui.painter();

                    // Background line
                    painter.line_segment(
                        [
                            egui::Pos2::new(start_x, y),
                            egui::Pos2::new(start_x + 500.0, y),
                        ],
                        egui::Stroke::new(1.0, egui::Color32::DARK_GRAY),
                    );

                    // Moving circle
                    let color = self.theme.primary();
                    painter.circle_filled(egui::Pos2::new(current_x, y), 8.0, color);

                    // Label
                    painter.text(
                        egui::Pos2::new(start_x - 80.0, y),
                        egui::Align2::RIGHT_CENTER,
                        name,
                        egui::FontId::proportional(12.0),
                        self.theme.on_surface(),
                    );
                }

                ui.add_space(250.0);
            });
        });
    }
}
