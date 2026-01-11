use armas::ext::ArmasContextExt;
use armas::{AnimatedTabs, TabStyle, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_title("Animated Tabs Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Animated Tabs Demo",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(TabsApp::new()))
        }),
    )
}

struct TabsApp {
    underline_tabs: AnimatedTabs,
    pill_tabs: AnimatedTabs,
    segment_tabs: AnimatedTabs,
    theme: Theme,
}

impl TabsApp {
    fn new() -> Self {
        Self {
            underline_tabs: AnimatedTabs::new(vec!["Home", "Products", "About", "Contact"])
                .style(TabStyle::Underline),
            pill_tabs: AnimatedTabs::new(vec!["Overview", "Analytics", "Reports"])
                .style(TabStyle::Pill),
            segment_tabs: AnimatedTabs::new(vec!["Day", "Week", "Month"]).style(TabStyle::Segment),
            theme: Theme::dark(),
        }
    }
}

impl eframe::App for TabsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Animated Tabs Component");
            ui.add_space(20.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Underline style
                ui.heading("Underline Style");
                ui.add_space(10.0);

                if let Some(index) = self.underline_tabs.show(ui) {
                    println!("Selected underline tab: {}", index);
                }

                ui.add_space(20.0);
                show_tab_content(
                    ui,
                    &self.theme,
                    self.underline_tabs.active_index,
                    &[
                        "Welcome to our homepage",
                        "Browse our products",
                        "Learn about us",
                        "Get in touch",
                    ],
                );

                ui.add_space(40.0);

                // Pill style
                ui.heading("Pill Style");
                ui.add_space(10.0);

                if let Some(index) = self.pill_tabs.show(ui) {
                    println!("Selected pill tab: {}", index);
                }

                ui.add_space(20.0);
                show_tab_content(
                    ui,
                    &self.theme,
                    self.pill_tabs.active_index,
                    &["Dashboard overview", "View analytics", "Generate reports"],
                );

                ui.add_space(40.0);

                // Segment style
                ui.heading("Segment Style");
                ui.add_space(10.0);

                if let Some(index) = self.segment_tabs.show(ui) {
                    println!("Selected segment tab: {}", index);
                }

                ui.add_space(20.0);
                show_tab_content(
                    ui,
                    &self.theme,
                    self.segment_tabs.active_index,
                    &["Today's activity", "This week's summary", "Monthly report"],
                );
            });
        });
    }
}

fn show_tab_content(ui: &mut egui::Ui, theme: &Theme, active: usize, content: &[&str]) {
    let text = content.get(active).unwrap_or(&"No content");

    let rect = ui.available_rect_before_wrap();
    let content_rect = egui::Rect::from_min_size(rect.min, egui::Vec2::new(rect.width(), 100.0));

    ui.painter().rect_filled(content_rect, 8.0, theme.surface());

    ui.painter().text(
        content_rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(16.0),
        theme.on_surface(),
    );

    ui.allocate_space(egui::Vec2::new(rect.width(), 100.0));
}
