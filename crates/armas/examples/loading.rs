use armas::{CircularProgress, LoadingDots, Skeleton, Spinner, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 800.0])
            .with_title("Loading Animations Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Loading Animations Demo",
        options,
        Box::new(|_cc| Ok(Box::new(LoadingApp::new()))),
    )
}

struct LoadingApp {
    theme: Theme,
    // Spinners
    spinner_default: Spinner,
    spinner_small: Spinner,
    spinner_large: Spinner,
    spinner_fast: Spinner,
    spinner_colored: Spinner,
    // Loading dots
    dots_default: LoadingDots,
    dots_small: LoadingDots,
    dots_many: LoadingDots,
    dots_colored: LoadingDots,
    // Skeletons
    skeleton_text: Skeleton,
    skeleton_title: Skeleton,
    skeleton_card: Skeleton,
    skeleton_image: Skeleton,
    // Circular progress
    circular_default: CircularProgress,
    circular_small: CircularProgress,
    circular_large: CircularProgress,
    circular_colored: CircularProgress,
}

impl LoadingApp {
    fn new() -> Self {
        let theme = Theme::dark();
        Self {
            theme: theme.clone(),
            // Spinners
            spinner_default: Spinner::new(&theme),
            spinner_small: Spinner::new(&theme).size(24.0),
            spinner_large: Spinner::new(&theme).size(64.0),
            spinner_fast: Spinner::new(&theme).speed(6.28), // 2x speed
            spinner_colored: Spinner::new(&theme)
                .color(egui::Color32::from_rgb(168, 85, 247))
                .size(48.0),
            // Loading dots
            dots_default: LoadingDots::new(&theme),
            dots_small: LoadingDots::new(&theme).dot_size(6.0).spacing(10.0),
            dots_many: LoadingDots::new(&theme).dot_count(5).spacing(16.0),
            dots_colored: LoadingDots::new(&theme)
                .color(egui::Color32::from_rgb(34, 197, 94))
                .dot_size(10.0),
            // Skeletons
            skeleton_text: Skeleton::new(300.0, 16.0),
            skeleton_title: Skeleton::new(200.0, 24.0),
            skeleton_card: Skeleton::new(350.0, 200.0).corner_radius(8.0),
            skeleton_image: Skeleton::new(150.0, 150.0)
                .corner_radius(12.0)
                .shimmer_width(0.5),
            // Circular progress
            circular_default: CircularProgress::new(&theme),
            circular_small: CircularProgress::new(&theme).size(24.0).stroke_width(2.0),
            circular_large: CircularProgress::new(&theme).size(64.0).stroke_width(4.0),
            circular_colored: CircularProgress::new(&theme)
                .color(egui::Color32::from_rgb(251, 191, 36))
                .size(48.0),
        }
    }
}

impl eframe::App for LoadingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("⏳ Loading Animations Component");
            ui.add_space(10.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Spinners section
                ui.heading("Rotating Spinners");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.set_min_width(150.0);
                            ui.set_min_height(100.0);
                            ui.vertical_centered(|ui| {
                                ui.label("Default");
                                ui.add_space(10.0);
                                self.spinner_default.show(ui, &self.theme);
                                ui.add_space(5.0);
                                ui.label("40px");
                            });
                        });
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.set_min_width(150.0);
                            ui.set_min_height(100.0);
                            ui.vertical_centered(|ui| {
                                ui.label("Small");
                                ui.add_space(10.0);
                                self.spinner_small.show(ui, &self.theme);
                                ui.add_space(5.0);
                                ui.label("24px");
                            });
                        });
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.set_min_width(150.0);
                            ui.set_min_height(100.0);
                            ui.vertical_centered(|ui| {
                                ui.label("Large");
                                ui.add_space(10.0);
                                self.spinner_large.show(ui, &self.theme);
                                ui.add_space(5.0);
                                ui.label("64px");
                            });
                        });
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.set_min_width(150.0);
                            ui.set_min_height(100.0);
                            ui.vertical_centered(|ui| {
                                ui.label("Fast");
                                ui.add_space(10.0);
                                self.spinner_fast.show(ui, &self.theme);
                                ui.add_space(5.0);
                                ui.label("2x speed");
                            });
                        });
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.set_min_width(150.0);
                            ui.set_min_height(100.0);
                            ui.vertical_centered(|ui| {
                                ui.label("Purple");
                                ui.add_space(10.0);
                                self.spinner_colored.show(ui, &self.theme);
                                ui.add_space(5.0);
                                ui.label("Custom color");
                            });
                        });
                    });
                });

                ui.add_space(20.0);

                // Loading dots section
                ui.heading("Loading Dots");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.set_min_width(200.0);
                            ui.set_min_height(80.0);
                            ui.vertical_centered(|ui| {
                                ui.label("Default (3 dots)");
                                ui.add_space(15.0);
                                self.dots_default.show(ui, &self.theme);
                            });
                        });
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.set_min_width(200.0);
                            ui.set_min_height(80.0);
                            ui.vertical_centered(|ui| {
                                ui.label("Small");
                                ui.add_space(15.0);
                                self.dots_small.show(ui, &self.theme);
                            });
                        });
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.set_min_width(200.0);
                            ui.set_min_height(80.0);
                            ui.vertical_centered(|ui| {
                                ui.label("Many (5 dots)");
                                ui.add_space(15.0);
                                self.dots_many.show(ui, &self.theme);
                            });
                        });
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.set_min_width(200.0);
                            ui.set_min_height(80.0);
                            ui.vertical_centered(|ui| {
                                ui.label("Green");
                                ui.add_space(15.0);
                                self.dots_colored.show(ui, &self.theme);
                            });
                        });
                    });
                });

                ui.add_space(20.0);

                // Circular progress section
                ui.heading("Circular Progress");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.set_min_width(150.0);
                            ui.set_min_height(100.0);
                            ui.vertical_centered(|ui| {
                                ui.label("Default");
                                ui.add_space(10.0);
                                self.circular_default.show(ui, &self.theme);
                                ui.add_space(5.0);
                                ui.label("40px");
                            });
                        });
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.set_min_width(150.0);
                            ui.set_min_height(100.0);
                            ui.vertical_centered(|ui| {
                                ui.label("Small");
                                ui.add_space(10.0);
                                self.circular_small.show(ui, &self.theme);
                                ui.add_space(5.0);
                                ui.label("24px");
                            });
                        });
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.set_min_width(150.0);
                            ui.set_min_height(100.0);
                            ui.vertical_centered(|ui| {
                                ui.label("Large");
                                ui.add_space(10.0);
                                self.circular_large.show(ui, &self.theme);
                                ui.add_space(5.0);
                                ui.label("64px");
                            });
                        });
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.set_min_width(150.0);
                            ui.set_min_height(100.0);
                            ui.vertical_centered(|ui| {
                                ui.label("Yellow");
                                ui.add_space(10.0);
                                self.circular_colored.show(ui, &self.theme);
                                ui.add_space(5.0);
                                ui.label("Custom color");
                            });
                        });
                    });
                });

                ui.add_space(20.0);

                // Skeleton loaders section
                ui.heading("Skeleton Loaders");
                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Text Line");
                        ui.add_space(10.0);
                        self.skeleton_text.show(ui, &self.theme);
                    });
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Title");
                        ui.add_space(10.0);
                        self.skeleton_title.show(ui, &self.theme);
                    });
                });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Card Placeholder");
                            ui.add_space(10.0);
                            self.skeleton_card.show(ui, &self.theme);
                        });
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Image Placeholder");
                            ui.add_space(10.0);
                            self.skeleton_image.show(ui, &self.theme);
                        });
                    });
                });

                ui.add_space(20.0);

                // Usage examples
                ui.separator();
                ui.heading("Use Cases");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.set_min_width(250.0);
                        ui.vertical(|ui| {
                            ui.label("Loading data...");
                            ui.add_space(10.0);
                            ui.horizontal(|ui| {
                                self.spinner_default.show(ui, &self.theme);
                                ui.add_space(10.0);
                                ui.label("Fetching users");
                            });
                        });
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.set_min_width(250.0);
                        ui.vertical(|ui| {
                            ui.label("Processing...");
                            ui.add_space(10.0);
                            ui.horizontal(|ui| {
                                self.dots_default.show(ui, &self.theme);
                                ui.add_space(10.0);
                                ui.label("Please wait");
                            });
                        });
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.set_min_width(250.0);
                        ui.vertical(|ui| {
                            ui.label("Content loading");
                            ui.add_space(10.0);
                            self.skeleton_text.show(ui, &self.theme);
                            ui.add_space(5.0);
                            self.skeleton_text.show(ui, &self.theme);
                            ui.add_space(5.0);
                            self.skeleton_text.show(ui, &self.theme);
                        });
                    });
                });
            });
        });
    }
}
