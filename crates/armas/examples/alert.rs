use armas::ext::ArmasContextExt;
use armas::{alert_error, alert_info, alert_success, alert_warning, Alert, AlertVariant, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Alert Component Example",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(AlertExample::default()))
        }),
    )
}

struct AlertExample {
    show_dismissible_info: bool,
    show_dismissible_success: bool,
    show_dismissible_warning: bool,
    show_dismissible_error: bool,
}

impl Default for AlertExample {
    fn default() -> Self {
        Self {
            show_dismissible_info: true,
            show_dismissible_success: true,
            show_dismissible_warning: true,
            show_dismissible_error: true,
        }
    }
}

impl eframe::App for AlertExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Alert Component Examples");
            ui.add_space(20.0);

            // Basic alerts
            ui.label("Basic Alerts:");
            ui.add_space(10.0);

            alert_info(ui, "This is an informational message");
            ui.add_space(10.0);

            alert_success(ui, "Operation completed successfully!");
            ui.add_space(10.0);

            alert_warning(ui, "Please review before proceeding");
            ui.add_space(10.0);

            alert_error(ui, "An error occurred while processing your request");
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Alerts with titles
            ui.label("Alerts with Titles:");
            ui.add_space(10.0);

            Alert::info("Your account has been verified and is now active")
                .title("Account Verified")
                .show(ui);
            ui.add_space(10.0);

            Alert::success("Your changes have been saved to the database")
                .title("Changes Saved")
                .show(ui);
            ui.add_space(10.0);

            Alert::warning("This action cannot be undone. Please make sure you want to proceed.")
                .title("Warning")
                .show(ui);
            ui.add_space(10.0);

            Alert::error("Failed to connect to the server. Please check your internet connection.")
                .title("Connection Error")
                .show(ui);
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Dismissible alerts
            ui.label("Dismissible Alerts:");
            ui.add_space(10.0);

            if self.show_dismissible_info {
                let response = Alert::info("Click the X button to dismiss this alert")
                    .title("Dismissible Info")
                    .dismissible(true)
                    .show(ui);

                if response.dismissed {
                    self.show_dismissible_info = false;
                }
                ui.add_space(10.0);
            }

            if self.show_dismissible_success {
                let response = Alert::success("You can close this success message")
                    .title("Dismissible Success")
                    .dismissible(true)
                    .show(ui);

                if response.dismissed {
                    self.show_dismissible_success = false;
                }
                ui.add_space(10.0);
            }

            if self.show_dismissible_warning {
                let response = Alert::warning("This warning can be dismissed")
                    .title("Dismissible Warning")
                    .dismissible(true)
                    .show(ui);

                if response.dismissed {
                    self.show_dismissible_warning = false;
                }
                ui.add_space(10.0);
            }

            if self.show_dismissible_error {
                let response = Alert::error("Close this error when you're ready")
                    .title("Dismissible Error")
                    .dismissible(true)
                    .show(ui);

                if response.dismissed {
                    self.show_dismissible_error = false;
                }
                ui.add_space(10.0);
            }

            if ui.button("Reset Dismissible Alerts").clicked() {
                self.show_dismissible_info = true;
                self.show_dismissible_success = true;
                self.show_dismissible_warning = true;
                self.show_dismissible_error = true;
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Without icons
            ui.label("Alerts Without Icons:");
            ui.add_space(10.0);

            Alert::info("This alert has no icon")
                .show_icon(false)
                .show(ui);
            ui.add_space(10.0);

            Alert::success("Icon-free success message")
                .title("Success")
                .show_icon(false)
                .show(ui);
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Custom widths
            ui.label("Custom Width Alerts:");
            ui.add_space(10.0);

            Alert::warning("This alert has a fixed width of 400px")
                .title("Fixed Width")
                .width(400.0)
                .show(ui);
            ui.add_space(10.0);

            Alert::info("This alert has a fixed width of 600px")
                .title("Wider Alert")
                .width(600.0)
                .show(ui);
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Complex example
            ui.label("Complex Alert:");
            ui.add_space(10.0);

            Alert::new(
                "Your trial period will expire in 3 days. Upgrade to a premium plan to continue enjoying all features without interruption.",
                AlertVariant::Warning
            )
            .title("Trial Expiring Soon")
            .dismissible(true)
            .show(ui);
        });
    }
}
