use armas::{Button, ButtonVariant, Popover, PopoverPosition, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Popover Component Example",
        options,
        Box::new(|_cc| Ok(Box::new(PopoverExample::default()))),
    )
}

struct PopoverExample {
    theme: Theme,
    basic_open: bool,
    top_open: bool,
    bottom_open: bool,
    left_open: bool,
    right_open: bool,
    auto_open: bool,
    no_arrow_open: bool,
    wide_open: bool,
    basic_popover: Popover,
    top_popover: Popover,
    bottom_popover: Popover,
    left_popover: Popover,
    right_popover: Popover,
    auto_popover: Popover,
    no_arrow_popover: Popover,
    wide_popover: Popover,
}

impl Default for PopoverExample {
    fn default() -> Self {
        Self {
            theme: Theme::dark(),
            basic_open: false,
            top_open: false,
            bottom_open: false,
            left_open: false,
            right_open: false,
            auto_open: false,
            no_arrow_open: false,
            wide_open: false,
            basic_popover: Popover::new("basic"),
            top_popover: Popover::new("top").position(PopoverPosition::Top),
            bottom_popover: Popover::new("bottom").position(PopoverPosition::Bottom),
            left_popover: Popover::new("left").position(PopoverPosition::Left),
            right_popover: Popover::new("right").position(PopoverPosition::Right),
            auto_popover: Popover::new("auto").position(PopoverPosition::Auto),
            no_arrow_popover: Popover::new("no_arrow").show_arrow(false),
            wide_popover: Popover::new("wide").width(400.0),
        }
    }
}

impl eframe::App for PopoverExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Popover Component Examples");
            ui.add_space(20.0);

            ui.label("Click buttons to toggle popovers:");
            ui.add_space(20.0);

            // Basic popover
            ui.horizontal(|ui| {
                let button_response = Button::new("Basic Popover")
                    .variant(ButtonVariant::Filled)
                    .show(ui, &self.theme);

                if button_response.clicked() {
                    self.basic_open = !self.basic_open;
                }

                self.basic_popover.show(
                    ctx,
                    &self.theme,
                    button_response.rect,
                    &mut self.basic_open,
                    |ui| {
                        ui.label("This is a basic popover!");
                        ui.add_space(8.0);
                        ui.label("It contains some content.");
                        ui.add_space(8.0);
                        if ui.button("Close").clicked() {
                            self.basic_open = false;
                        }
                    },
                );
            });
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Position variants
            ui.label("Popover Positions:");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                let button = Button::new("Top")
                    .variant(ButtonVariant::Outlined)
                    .show(ui, &self.theme);
                if button.clicked() {
                    self.top_open = !self.top_open;
                }
                self.top_popover
                    .show(ctx, &self.theme, button.rect, &mut self.top_open, |ui| {
                        ui.label("Popover appears on top");
                    });

                let button = Button::new("Bottom")
                    .variant(ButtonVariant::Outlined)
                    .show(ui, &self.theme);
                if button.clicked() {
                    self.bottom_open = !self.bottom_open;
                }
                self.bottom_popover.show(
                    ctx,
                    &self.theme,
                    button.rect,
                    &mut self.bottom_open,
                    |ui| {
                        ui.label("Popover appears on bottom");
                    },
                );

                let button = Button::new("Left")
                    .variant(ButtonVariant::Outlined)
                    .show(ui, &self.theme);
                if button.clicked() {
                    self.left_open = !self.left_open;
                }
                self.left_popover
                    .show(ctx, &self.theme, button.rect, &mut self.left_open, |ui| {
                        ui.label("Popover appears on left");
                    });

                let button = Button::new("Right")
                    .variant(ButtonVariant::Outlined)
                    .show(ui, &self.theme);
                if button.clicked() {
                    self.right_open = !self.right_open;
                }
                self.right_popover.show(
                    ctx,
                    &self.theme,
                    button.rect,
                    &mut self.right_open,
                    |ui| {
                        ui.label("Popover appears on right");
                    },
                );
            });
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Auto positioning
            ui.label("Auto Position (adapts to space):");
            ui.add_space(10.0);

            let button = Button::new("Auto Position")
                .variant(ButtonVariant::Outlined)
                .show(ui, &self.theme);
            if button.clicked() {
                self.auto_open = !self.auto_open;
            }
            self.auto_popover
                .show(ctx, &self.theme, button.rect, &mut self.auto_open, |ui| {
                    ui.label("This popover automatically");
                    ui.label("chooses the best position");
                    ui.label("based on available space!");
                });
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Without arrow
            ui.label("Without Arrow:");
            ui.add_space(10.0);

            let button = Button::new("No Arrow")
                .variant(ButtonVariant::Text)
                .show(ui, &self.theme);
            if button.clicked() {
                self.no_arrow_open = !self.no_arrow_open;
            }
            self.no_arrow_popover.show(
                ctx,
                &self.theme,
                button.rect,
                &mut self.no_arrow_open,
                |ui| {
                    ui.label("This popover has no arrow pointer");
                },
            );
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Wide popover
            ui.label("Custom Width:");
            ui.add_space(10.0);

            let button = Button::new("Wide Popover")
                .variant(ButtonVariant::Filled)
                .show(ui, &self.theme);
            if button.clicked() {
                self.wide_open = !self.wide_open;
            }
            self.wide_popover
                .show(ctx, &self.theme, button.rect, &mut self.wide_open, |ui| {
                    ui.label("This is a wider popover with a custom width of 400px.");
                    ui.add_space(10.0);
                    ui.label("It can contain more content comfortably.");
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.wide_open = false;
                        }
                        if ui.button("Confirm").clicked() {
                            self.wide_open = false;
                        }
                    });
                });
        });
    }
}
