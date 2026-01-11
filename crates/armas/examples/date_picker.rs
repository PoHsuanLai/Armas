use armas::ext::ArmasContextExt;
use armas::fonts;
use armas::{Date, DatePicker, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "DatePicker Component Example",
        options,
        Box::new(|cc| {
            // Load Inter font using embedded font files
            let inter_regular = include_bytes!("../fonts/Inter-Regular.otf");
            let inter_medium = include_bytes!("../fonts/Inter-Medium.otf");
            let inter_semibold = include_bytes!("../fonts/Inter-SemiBold.otf");
            let inter_bold = include_bytes!("../fonts/Inter-Bold.otf");

            fonts::load_font_family(
                &cc.egui_ctx,
                "Inter",
                inter_regular,
                Some(inter_medium),
                Some(inter_semibold),
                Some(inter_bold),
            );

            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(DatePickerExample::default()))
        }),
    )
}

struct DatePickerExample {
    birthday: Option<Date>,
    appointment: Option<Date>,
    deadline: Option<Date>,
    start_date: Option<Date>,
    birthday_picker: DatePicker,
    appointment_picker: DatePicker,
    deadline_picker: DatePicker,
    start_date_picker: DatePicker,
}

impl Default for DatePickerExample {
    fn default() -> Self {
        Self {
            birthday: None,
            appointment: None,
            deadline: None,
            start_date: Some(Date::today()),
            birthday_picker: DatePicker::new("birthday")
                .label("Birthday")
                .placeholder("Select your birthday..."),
            appointment_picker: DatePicker::new("appointment")
                .label("Appointment Date")
                .placeholder("Choose a date..."),
            deadline_picker: DatePicker::new("deadline")
                .label("Project Deadline")
                .placeholder("Set deadline..."),
            start_date_picker: DatePicker::new("start_date")
                .label("Start Date")
                .placeholder("Select start date..."),
        }
    }
}

impl eframe::App for DatePickerExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let theme = ctx.armas_theme();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("DatePicker Component Examples");
            ui.add_space(20.0);

            // Basic date picker
            ui.label("Basic Date Picker:");
            ui.add_space(10.0);

            let response = self
                .birthday_picker
                .show(ui, &mut self.birthday);
            if response.changed {
                if let Some(date) = &self.birthday {
                    println!("Birthday changed to: {}", date.format());
                }
            }

            if let Some(date) = &self.birthday {
                ui.add_space(5.0);
                ui.colored_label(
                    theme.success(),
                    format!(
                        "Selected: {} ({}, {})",
                        date.format(),
                        date.month_name(),
                        date.year
                    ),
                );
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Another date picker
            ui.label("Appointment Scheduler:");
            ui.add_space(10.0);

            self.appointment_picker
                .show(ui, &mut self.appointment);

            if let Some(date) = &self.appointment {
                ui.add_space(5.0);
                ui.label(format!("Your appointment is on: {}", date.format()));
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Deadline picker
            ui.label("Project Deadline:");
            ui.add_space(10.0);

            self.deadline_picker
                .show(ui, &mut self.deadline);

            if let Some(date) = &self.deadline {
                ui.add_space(5.0);
                let today = Date::today();
                let days_diff = days_between(&today, date);

                if days_diff > 0 {
                    ui.colored_label(
                        theme.warning(),
                        format!("⚠ Deadline in {} days", days_diff),
                    );
                } else if days_diff == 0 {
                    ui.colored_label(theme.error(), "⚠ Deadline is today!");
                } else {
                    ui.colored_label(
                        theme.error(),
                        format!("⚠ Overdue by {} days", -days_diff),
                    );
                }
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Pre-filled date picker
            ui.label("Start Date (Pre-filled with today):");
            ui.add_space(10.0);

            self.start_date_picker
                .show(ui, &mut self.start_date);

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Info
            ui.label("💡 Features:");
            ui.label("  • Click input field to open calendar");
            ui.label("  • Type date in YYYY-MM-DD format");
            ui.label("  • Navigate months with arrow buttons");
            ui.label("  • Click 'Today' for current date");
            ui.label("  • Click 'Clear' to reset selection");
            ui.label("  • Today's date is outlined");
            ui.label("  • Selected date is highlighted");
        });
    }
}

// Helper function to calculate days between dates (approximate)
fn days_between(from: &Date, to: &Date) -> i32 {
    let from_days = date_to_days(from);
    let to_days = date_to_days(to);
    to_days - from_days
}

fn date_to_days(date: &Date) -> i32 {
    let mut days = 0;

    // Add years
    for y in 1..date.year {
        days += if Date::is_leap_year(y) { 366 } else { 365 };
    }

    // Add months
    for m in 1..date.month {
        days += Date::days_in_month(date.year, m) as i32;
    }

    // Add days
    days += date.day as i32;

    days
}
