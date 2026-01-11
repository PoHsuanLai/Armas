use armas::ext::ArmasContextExt;
use armas::{Button, ButtonVariant, Theme, ToastManager, ToastPosition, ToastVariant};
use eframe::egui;
use std::time::Duration;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Toast/Notification Example",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(ToastExample::default()))
        }),
    )
}

struct ToastExample {
    toast_manager: ToastManager,
    position: ToastPosition,
}

impl Default for ToastExample {
    fn default() -> Self {
        Self {
            toast_manager: ToastManager::new(),
            position: ToastPosition::TopRight,
        }
    }
}

impl eframe::App for ToastExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Toast/Notification Examples");
            ui.add_space(20.0);

            ui.label("Click buttons to show different toast types:");
            ui.add_space(20.0);

            // Basic toast types
            ui.horizontal(|ui| {
                if Button::new("Info Toast")
                    .variant(ButtonVariant::Filled)
                    .show(ui)
                    .clicked()
                {
                    self.toast_manager.info("This is an informational message");
                }

                if Button::new("Success Toast")
                    .variant(ButtonVariant::Filled)
                    .show(ui)
                    .clicked()
                {
                    self.toast_manager
                        .success("Operation completed successfully!");
                }

                if Button::new("Warning Toast")
                    .variant(ButtonVariant::Filled)
                    .show(ui)
                    .clicked()
                {
                    self.toast_manager.warning("This is a warning message");
                }

                if Button::new("Error Toast")
                    .variant(ButtonVariant::Filled)
                    .show(ui)
                    .clicked()
                {
                    self.toast_manager.error("An error occurred!");
                }
            });
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Custom toasts
            ui.label("Custom Toasts:");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if Button::new("With Title")
                    .variant(ButtonVariant::Outlined)
                    .show(ui)
                    .clicked()
                {
                    self.toast_manager
                        .custom()
                        .title("Upload Complete")
                        .message("Your file has been uploaded successfully")
                        .variant(ToastVariant::Success)
                        .show();
                }

                if Button::new("Long Duration")
                    .variant(ButtonVariant::Outlined)
                    .show(ui)
                    .clicked()
                {
                    self.toast_manager
                        .custom()
                        .message("This toast will stay for 10 seconds")
                        .variant(ToastVariant::Info)
                        .duration(Duration::from_secs(10))
                        .show();
                }

                if Button::new("Non-dismissible")
                    .variant(ButtonVariant::Outlined)
                    .show(ui)
                    .clicked()
                {
                    self.toast_manager
                        .custom()
                        .message("You cannot close this manually")
                        .variant(ToastVariant::Warning)
                        .dismissible(false)
                        .show();
                }
            });
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Position controls
            ui.label("Toast Position:");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui
                    .selectable_label(self.position == ToastPosition::TopLeft, "Top Left")
                    .clicked()
                {
                    self.position = ToastPosition::TopLeft;
                    self.toast_manager = ToastManager::new().position(self.position);
                }
                if ui
                    .selectable_label(self.position == ToastPosition::TopCenter, "Top Center")
                    .clicked()
                {
                    self.position = ToastPosition::TopCenter;
                    self.toast_manager = ToastManager::new().position(self.position);
                }
                if ui
                    .selectable_label(self.position == ToastPosition::TopRight, "Top Right")
                    .clicked()
                {
                    self.position = ToastPosition::TopRight;
                    self.toast_manager = ToastManager::new().position(self.position);
                }
            });
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(self.position == ToastPosition::BottomLeft, "Bottom Left")
                    .clicked()
                {
                    self.position = ToastPosition::BottomLeft;
                    self.toast_manager = ToastManager::new().position(self.position);
                }
                if ui
                    .selectable_label(
                        self.position == ToastPosition::BottomCenter,
                        "Bottom Center",
                    )
                    .clicked()
                {
                    self.position = ToastPosition::BottomCenter;
                    self.toast_manager = ToastManager::new().position(self.position);
                }
                if ui
                    .selectable_label(self.position == ToastPosition::BottomRight, "Bottom Right")
                    .clicked()
                {
                    self.position = ToastPosition::BottomRight;
                    self.toast_manager = ToastManager::new().position(self.position);
                }
            });
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Spam test
            if Button::new("Spam 5 Toasts")
                .variant(ButtonVariant::Text)
                .show(ui)
                .clicked()
            {
                for i in 1..=5 {
                    self.toast_manager
                        .custom()
                        .title(format!("Toast #{}", i))
                        .message(format!("This is toast number {}", i))
                        .variant(match i % 4 {
                            0 => ToastVariant::Info,
                            1 => ToastVariant::Success,
                            2 => ToastVariant::Warning,
                            _ => ToastVariant::Error,
                        })
                        .show();
                }
            }
        });

        // Show all toasts
        self.toast_manager.show(ctx);
    }
}
