use armas::{Button, ButtonVariant, Step, Stepper, StepperOrientation, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Stepper Component Example",
        options,
        Box::new(|_cc| Ok(Box::new(StepperExample::default()))),
    )
}

struct StepperExample {
    theme: Theme,
    current_step: usize,
    vertical_step: usize,
    clickable_step: usize,
}

impl Default for StepperExample {
    fn default() -> Self {
        Self {
            theme: Theme::dark(),
            current_step: 1,
            vertical_step: 0,
            clickable_step: 2,
        }
    }
}

impl eframe::App for StepperExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Stepper Component Examples");
            ui.add_space(20.0);

            // Basic horizontal stepper
            ui.label("Basic Horizontal Stepper:");
            ui.add_space(10.0);

            let stepper = Stepper::new()
                .add_step(Step::new("Account"))
                .add_step(Step::new("Profile"))
                .add_step(Step::new("Complete"));

            stepper.show(ui, &self.theme, self.current_step);

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("◀ Previous").clicked() && self.current_step > 0 {
                    self.current_step -= 1;
                }
                if ui.button("Next ▶").clicked() && self.current_step < 2 {
                    self.current_step += 1;
                }
                if ui.button("Reset").clicked() {
                    self.current_step = 0;
                }
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Stepper with descriptions
            ui.label("With Descriptions:");
            ui.add_space(10.0);

            let stepper = Stepper::new()
                .add_step(Step::new("Select Plan").description("Choose your subscription"))
                .add_step(Step::new("Payment").description("Enter payment details"))
                .add_step(Step::new("Confirm").description("Review and confirm"));

            stepper.show(ui, &self.theme, self.current_step);

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Vertical stepper
            ui.label("Vertical Stepper:");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    let stepper = Stepper::new()
                        .orientation(StepperOrientation::Vertical)
                        .add_step(Step::new("Personal Info").description("Name and contact"))
                        .add_step(Step::new("Address").description("Shipping address"))
                        .add_step(Step::new("Payment").description("Payment method"))
                        .add_step(Step::new("Review").description("Confirm order"));

                    stepper.show(ui, &self.theme, self.vertical_step);
                });

                ui.add_space(40.0);

                ui.vertical(|ui| {
                    ui.label("Current Step Content:");
                    ui.add_space(10.0);

                    match self.vertical_step {
                        0 => {
                            ui.heading("Personal Information");
                            ui.add_space(10.0);
                            ui.label("Enter your name and contact details...");
                        }
                        1 => {
                            ui.heading("Shipping Address");
                            ui.add_space(10.0);
                            ui.label("Enter your delivery address...");
                        }
                        2 => {
                            ui.heading("Payment Method");
                            ui.add_space(10.0);
                            ui.label("Choose how you'd like to pay...");
                        }
                        3 => {
                            ui.heading("Review Order");
                            ui.add_space(10.0);
                            ui.label("Please review your order before submitting...");
                        }
                        _ => {}
                    }

                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        if Button::new("Back")
                            .variant(ButtonVariant::Outlined)
                            .show(ui, &self.theme)
                            .clicked()
                            && self.vertical_step > 0
                        {
                            self.vertical_step -= 1;
                        }

                        if self.vertical_step < 3 {
                            if Button::new("Continue")
                                .variant(ButtonVariant::Filled)
                                .show(ui, &self.theme)
                                .clicked()
                            {
                                self.vertical_step += 1;
                            }
                        } else {
                            if Button::new("Submit Order")
                                .variant(ButtonVariant::Filled)
                                .show(ui, &self.theme)
                                .clicked()
                            {
                                self.vertical_step = 0;
                            }
                        }
                    });
                });
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Clickable stepper
            ui.label("Clickable Stepper (click on steps):");
            ui.add_space(10.0);

            let stepper = Stepper::new()
                .clickable(true)
                .add_step(Step::new("Setup"))
                .add_step(Step::new("Configure"))
                .add_step(Step::new("Deploy"))
                .add_step(Step::new("Monitor"));

            let response = stepper.show(ui, &self.theme, self.clickable_step);

            if let Some(clicked) = response.clicked_step {
                self.clickable_step = clicked;
            }

            ui.add_space(10.0);
            ui.label(format!("Current step: {}", self.clickable_step + 1));
        });
    }
}
