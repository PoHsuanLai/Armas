use armas::ext::ArmasContextExt;
use armas::{Theme, Timeline, TimelineItem};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 800.0])
            .with_title("Timeline Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Timeline Demo",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(TimelineApp::new()))
        }),
    )
}

struct TimelineApp {
}

impl TimelineApp {
    fn new() -> Self {
        Self {
        }
    }
}

impl eframe::App for TimelineApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Timeline Component");
            ui.add_space(20.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Project timeline
                ui.heading("Project Progress");
                ui.add_space(10.0);

                let items = vec![
                    TimelineItem::new(
                        "Project Started",
                        "Initial planning and requirements gathering completed",
                    )
                    .time("3 days ago")
                    .icon("🚀")
                    .icon_color(egui::Color32::from_rgb(59, 130, 246)),
                    TimelineItem::new(
                        "Design Phase",
                        "UI/UX mockups created and reviewed by stakeholders",
                    )
                    .time("2 days ago")
                    .icon("🎨")
                    .icon_color(egui::Color32::from_rgb(168, 85, 247)),
                    TimelineItem::new(
                        "Development Started",
                        "Backend API implementation in progress. Core endpoints completed.",
                    )
                    .time("1 day ago")
                    .icon("💻")
                    .icon_color(egui::Color32::from_rgb(34, 197, 94))
                    .highlighted(true),
                    TimelineItem::new("Testing", "Unit tests and integration tests to be written")
                        .time("Upcoming")
                        .icon("🧪")
                        .icon_color(egui::Color32::from_rgb(251, 191, 36)),
                    TimelineItem::new("Deployment", "Deploy to production environment")
                        .time("Next week")
                        .icon("🎯")
                        .icon_color(egui::Color32::from_rgb(239, 68, 68)),
                ];

                Timeline::new(items)
                    .dot_size(12.0)
                    .item_gap(32.0)
                    .show(ui);

                ui.add_space(40.0);

                // Simple timeline without icons
                ui.heading("Activity Log");
                ui.add_space(10.0);

                let activities = vec![
                    TimelineItem::new("User Login", "john@example.com logged in from 192.168.1.1")
                        .time("10:34 AM"),
                    TimelineItem::new("File Upload", "Uploaded document.pdf (2.4 MB)")
                        .time("10:36 AM")
                        .highlighted(true),
                    TimelineItem::new("Settings Changed", "Updated notification preferences")
                        .time("10:42 AM"),
                    TimelineItem::new("Comment Posted", "Added comment on issue #1234")
                        .time("11:15 AM"),
                ];

                Timeline::new(activities)
                    .dot_size(8.0)
                    .line_width(2.0)
                    .item_gap(24.0)
                    .show(ui);
            });
        });
    }
}
