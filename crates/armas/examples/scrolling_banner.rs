use armas::{ScrollDirection, ScrollingBanner, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_title("Scrolling Banner Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Scrolling Banner Demo",
        options,
        Box::new(|_cc| Ok(Box::new(ScrollingBannerApp::new()))),
    )
}

struct ScrollingBannerApp {
    theme: Theme,
    banner_left: ScrollingBanner,
    banner_right: ScrollingBanner,
    banner_up: ScrollingBanner,
    banner_down: ScrollingBanner,
    banner_fast: ScrollingBanner,
    banner_slow: ScrollingBanner,
    banner_logos: ScrollingBanner,
}

impl ScrollingBannerApp {
    fn new() -> Self {
        Self {
            theme: Theme::dark(),
            banner_left: ScrollingBanner::new()
                .direction(ScrollDirection::Left)
                .speed(50.0),
            banner_right: ScrollingBanner::new()
                .direction(ScrollDirection::Right)
                .speed(50.0),
            banner_up: ScrollingBanner::new()
                .direction(ScrollDirection::Up)
                .speed(30.0),
            banner_down: ScrollingBanner::new()
                .direction(ScrollDirection::Down)
                .speed(30.0),
            banner_fast: ScrollingBanner::new()
                .direction(ScrollDirection::Left)
                .speed(150.0),
            banner_slow: ScrollingBanner::new()
                .direction(ScrollDirection::Left)
                .speed(20.0),
            banner_logos: ScrollingBanner::new()
                .direction(ScrollDirection::Left)
                .speed(40.0)
                .gap(48.0),
        }
    }
}

impl eframe::App for ScrollingBannerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Scrolling Banner Component");
            ui.add_space(10.0);

            // Horizontal scrolling - Left
            ui.group(|ui| {
                ui.label("Scrolling Left (50px/s)");
                ui.add_space(5.0);

                ui.set_height(50.0);
                self.banner_left.show(ui, &self.theme, |ui, _index| {
                    ui.horizontal(|ui| {
                        ui.label("Item 1");
                        ui.add_space(20.0);
                        ui.label("Item 2");
                        ui.add_space(20.0);
                        ui.label("Item 3");
                        ui.add_space(20.0);
                        ui.label("Item 4");
                        ui.add_space(20.0);
                        ui.label("Item 5");
                    });
                });
            });

            ui.add_space(10.0);

            // Horizontal scrolling - Right
            ui.group(|ui| {
                ui.label("→ Scrolling Right (50px/s)");
                ui.add_space(5.0);

                ui.set_height(50.0);
                self.banner_right.show(ui, &self.theme, |ui, _index| {
                    ui.horizontal(|ui| {
                        ui.colored_label(self.theme.primary(), "React");
                        ui.add_space(20.0);
                        ui.colored_label(self.theme.secondary(), "Vue");
                        ui.add_space(20.0);
                        ui.colored_label(self.theme.success(), "Svelte");
                        ui.add_space(20.0);
                        ui.colored_label(self.theme.warning(), "Angular");
                        ui.add_space(20.0);
                        ui.colored_label(self.theme.error(), "Solid");
                    });
                });
            });

            ui.add_space(10.0);

            // Speed comparison
            ui.group(|ui| {
                ui.label("Fast (150px/s)");
                ui.add_space(5.0);

                ui.set_height(50.0);
                self.banner_fast.show(ui, &self.theme, |ui, _index| {
                    ui.horizontal(|ui| {
                        for i in 1..=10 {
                            ui.label(format!("#{}", i));
                            ui.add_space(15.0);
                        }
                    });
                });
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.label("Slow (20px/s)");
                ui.add_space(5.0);

                ui.set_height(50.0);
                self.banner_slow.show(ui, &self.theme, |ui, _index| {
                    ui.horizontal(|ui| {
                        ui.label("Slow");
                        ui.add_space(30.0);
                        ui.label("and");
                        ui.add_space(30.0);
                        ui.label("steady");
                        ui.add_space(30.0);
                        ui.label("wins");
                        ui.add_space(30.0);
                        ui.label("the");
                        ui.add_space(30.0);
                        ui.label("race");
                    });
                });
            });

            ui.add_space(10.0);

            // Logo-style banner
            ui.group(|ui| {
                ui.label("Logo Carousel");
                ui.add_space(5.0);

                ui.set_height(50.0);
                self.banner_logos.show(ui, &self.theme, |ui, _index| {
                    ui.horizontal(|ui| {
                        // Simulate logo cards
                        for name in ["Rust", "egui", "TypeScript", "React", "Go"].iter() {
                            ui.label(*name);
                            ui.add_space(20.0);
                        }
                    });
                });
            });

            ui.add_space(10.0);

            // Vertical scrolling example
            ui.horizontal(|ui| {
                // Scrolling up
                ui.group(|ui| {
                    ui.label("Scrolling Up");
                    ui.add_space(5.0);

                    ui.set_width(150.0);
                    ui.set_height(200.0);
                    self.banner_up.show(ui, &self.theme, |ui, _index| {
                        ui.vertical(|ui| {
                            for i in 1..=5 {
                                ui.label(format!("Line {}", i));
                                ui.add_space(10.0);
                            }
                        });
                    });
                });

                ui.add_space(20.0);

                // Scrolling down
                ui.group(|ui| {
                    ui.label("Scrolling Down");
                    ui.add_space(5.0);

                    ui.set_width(150.0);
                    ui.set_height(200.0);
                    self.banner_down.show(ui, &self.theme, |ui, _index| {
                        ui.vertical(|ui| {
                            for i in 1..=5 {
                                ui.colored_label(self.theme.primary(), format!("Item {}", i));
                                ui.add_space(10.0);
                            }
                        });
                    });
                });
            });

            ui.add_space(20.0);

            // Controls
            ui.separator();
            ui.label("💡 Tip: Hover over any banner to pause it!");
        });
    }
}
