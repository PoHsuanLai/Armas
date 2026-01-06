use armas::{
    confirm_dialog, dialog, Button, ButtonVariant, ConfirmResponse, Modal, ModalSize, Theme,
};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Modal/Dialog Example",
        options,
        Box::new(|_cc| Ok(Box::new(ModalExample::default()))),
    )
}

struct ModalExample {
    theme: Theme,
    simple_modal_open: bool,
    custom_modal_open: bool,
    small_modal_open: bool,
    large_modal_open: bool,
    fullscreen_modal_open: bool,
    no_close_modal_open: bool,
    confirm_modal_open: bool,
    confirm_result: Option<ConfirmResponse>,
    custom_modal: Modal,
}

impl Default for ModalExample {
    fn default() -> Self {
        Self {
            theme: Theme::dark(),
            simple_modal_open: false,
            custom_modal_open: false,
            small_modal_open: false,
            large_modal_open: false,
            fullscreen_modal_open: false,
            no_close_modal_open: false,
            confirm_modal_open: false,
            confirm_result: None,
            custom_modal: Modal::new()
                .title("Custom Modal")
                .size(ModalSize::Custom(700.0, 450.0)),
        }
    }
}

impl eframe::App for ModalExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Modal/Dialog Examples");
            ui.add_space(20.0);

            ui.label("Click buttons to open different modal types:");
            ui.add_space(20.0);

            // Simple dialog
            ui.horizontal(|ui| {
                if Button::new("Simple Dialog")
                    .variant(ButtonVariant::Filled)
                    .show(ui, &self.theme)
                    .clicked()
                {
                    self.simple_modal_open = true;
                }
                ui.label("- Basic dialog with title");
            });
            ui.add_space(10.0);

            // Size variants
            ui.horizontal(|ui| {
                if Button::new("Small Modal")
                    .variant(ButtonVariant::Outlined)
                    .show(ui, &self.theme)
                    .clicked()
                {
                    self.small_modal_open = true;
                }
                ui.label("- 400x300 modal");
            });
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if Button::new("Large Modal")
                    .variant(ButtonVariant::Outlined)
                    .show(ui, &self.theme)
                    .clicked()
                {
                    self.large_modal_open = true;
                }
                ui.label("- 800x500 modal");
            });
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if Button::new("Full Screen")
                    .variant(ButtonVariant::Outlined)
                    .show(ui, &self.theme)
                    .clicked()
                {
                    self.fullscreen_modal_open = true;
                }
                ui.label("- Nearly full screen modal");
            });
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if Button::new("Custom Size")
                    .variant(ButtonVariant::Outlined)
                    .show(ui, &self.theme)
                    .clicked()
                {
                    self.custom_modal_open = true;
                }
                ui.label("- 700x450 custom modal");
            });
            ui.add_space(20.0);

            // Special modals
            ui.separator();
            ui.add_space(20.0);

            ui.horizontal(|ui| {
                if Button::new("Non-closable Modal")
                    .variant(ButtonVariant::Text)
                    .show(ui, &self.theme)
                    .clicked()
                {
                    self.no_close_modal_open = true;
                }
                ui.label("- Must use button to close");
            });
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if Button::new("Confirmation Dialog")
                    .variant(ButtonVariant::Filled)
                    .show(ui, &self.theme)
                    .clicked()
                {
                    self.confirm_modal_open = true;
                }
                ui.label("- Yes/No confirmation");
                if let Some(result) = self.confirm_result {
                    ui.label(format!("Last result: {:?}", result));
                }
            });
        });

        // Render modals
        dialog(
            ctx,
            &self.theme,
            &mut self.simple_modal_open,
            "Simple Dialog",
            |ui| {
                ui.label("This is a simple modal dialog.");
                ui.add_space(10.0);
                ui.label("You can close it by:");
                ui.label("• Clicking the X button");
                ui.label("• Pressing ESC");
                ui.label("• Clicking outside (backdrop)");
            },
        );

        let mut small_modal = Modal::new().title("Small Modal").size(ModalSize::Small);
        small_modal.show(ctx, &self.theme, &mut self.small_modal_open, |ui| {
            ui.label("This is a small 400x300 modal.");
            ui.add_space(10.0);
            ui.label("Perfect for quick confirmations or short forms.");
        });

        let mut large_modal = Modal::new().title("Large Modal").size(ModalSize::Large);
        large_modal.show(ctx, &self.theme, &mut self.large_modal_open, |ui| {
            ui.label("This is a large 800x500 modal.");
            ui.add_space(10.0);
            ui.label("Great for detailed content, forms, or settings.");
            ui.add_space(20.0);

            ui.heading("Example Form");
            ui.add_space(10.0);
            ui.text_edit_singleline(&mut String::from("Username"));
            ui.text_edit_singleline(&mut String::from("Email"));
            ui.text_edit_multiline(&mut String::from("Description"));
        });

        let mut fullscreen_modal = Modal::new()
            .title("Full Screen Modal")
            .size(ModalSize::FullScreen);
        fullscreen_modal.show(ctx, &self.theme, &mut self.fullscreen_modal_open, |ui| {
            ui.label("This modal takes up 95% of the screen.");
            ui.add_space(10.0);
            ui.label("Useful for complex interfaces or workflows.");
        });

        self.custom_modal
            .show(ctx, &self.theme, &mut self.custom_modal_open, |ui| {
                ui.label("This is a custom 700x450 modal.");
                ui.add_space(10.0);
                ui.label("You can set any custom width and height.");
                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    ui.label("Status:");
                    ui.label("Everything looks good!");
                });
            });

        let mut no_close_modal = Modal::new().title("Cannot Close Easily").closable(false);
        let mut should_close = false;
        no_close_modal.show(ctx, &self.theme, &mut self.no_close_modal_open, |ui| {
            ui.label("This modal cannot be closed with ESC or backdrop click.");
            ui.add_space(10.0);
            ui.label("You must use this button:");
            ui.add_space(20.0);

            if ui.button("Close Modal").clicked() {
                should_close = true;
            }
        });
        if should_close {
            self.no_close_modal_open = false;
        }

        // Confirmation dialog
        let result = confirm_dialog(
            ctx,
            &self.theme,
            &mut self.confirm_modal_open,
            "Confirm Action",
            "Are you sure you want to proceed?",
        );

        if result != ConfirmResponse::None {
            self.confirm_result = Some(result);
        }
    }
}
