use armas::{FeatureGrid, FeatureItem, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0])
            .with_title("Feature Grid Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Feature Grid Demo",
        options,
        Box::new(|_cc| Ok(Box::new(FeatureGridApp::new()))),
    )
}

struct FeatureGridApp {
    theme: Theme,
}

impl FeatureGridApp {
    fn new() -> Self {
        Self {
            theme: Theme::dark(),
        }
    }
}

impl eframe::App for FeatureGridApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Feature Grid Component");
            ui.add_space(20.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // 3-column grid
                let items = vec![
                    FeatureItem::new(
                        "🚀",
                        "Fast",
                        "Lightning-fast performance with zero overhead",
                    ),
                    FeatureItem::new("🎨", "Beautiful", "Gorgeous UI components out of the box"),
                    FeatureItem::new("🔧", "Flexible", "Highly customizable to fit your needs"),
                    FeatureItem::new(
                        "📦",
                        "Modular",
                        "Use only what you need, keep it lightweight",
                    ),
                    FeatureItem::new("🔒", "Secure", "Built with security best practices in mind"),
                    FeatureItem::new("📱", "Responsive", "Works great on any screen size"),
                ];

                FeatureGrid::new(items).show(ui, &self.theme);

                ui.add_space(40.0);

                // 2-column grid with custom styling
                ui.heading("Custom Styling");
                ui.add_space(20.0);

                let items = vec![
                    FeatureItem::new("⚡", "Real-time", "Instant updates and synchronization")
                        .icon_color(egui::Color32::from_rgb(251, 191, 36)),
                    FeatureItem::new("🌐", "Global", "Deploy anywhere in the world")
                        .icon_color(egui::Color32::from_rgb(59, 130, 246)),
                    FeatureItem::new("🔄", "Reliable", "99.9% uptime guaranteed")
                        .icon_color(egui::Color32::from_rgb(34, 197, 94)),
                    FeatureItem::new("💡", "Smart", "AI-powered automation")
                        .icon_color(egui::Color32::from_rgb(168, 85, 247)),
                ];

                FeatureGrid::new(items)
                    .columns(2)
                    .gap(30.0)
                    .icon_size(40.0)
                    .show(ui, &self.theme);

                ui.add_space(40.0);

                // No borders, no hover
                ui.heading("Minimal Style");
                ui.add_space(20.0);

                let items = vec![
                    FeatureItem::new("📊", "Analytics", "Deep insights into your data"),
                    FeatureItem::new("🔐", "Privacy", "Your data stays yours"),
                    FeatureItem::new("🎯", "Focused", "No distractions, just results"),
                ];

                FeatureGrid::new(items)
                    .columns(3)
                    .show_borders(false)
                    .hover_effect(false)
                    .show(ui, &self.theme);
            });
        });
    }
}
