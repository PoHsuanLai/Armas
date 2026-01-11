use armas::ext::ArmasContextExt;
use armas::{tooltip, tooltip_with, Button, ButtonVariant, Theme, Tooltip, TooltipPosition};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Tooltip Example",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(TooltipExample::default()))
        }),
    )
}

struct TooltipExample {
}

impl Default for TooltipExample {
    fn default() -> Self {
        Self {
        }
    }
}

impl eframe::App for TooltipExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let theme = ctx.armas_theme();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tooltip Examples");
            ui.add_space(20.0);

            // Simple tooltip
            ui.label("Simple Tooltip:");
            let response = ui.button("Hover me!");
            tooltip(ui, &theme, &response, "This is a simple tooltip");
            ui.add_space(20.0);

            // Tooltip with longer text
            ui.label("Wrapped Text:");
            let response = ui.button("Long tooltip");
            tooltip(
                ui,
                &theme,
                &response,
                "This is a much longer tooltip that will wrap to multiple lines when it exceeds the maximum width.",
            );
            ui.add_space(20.0);

            // Tooltip with custom delay
            ui.label("Custom Delay (1 second):");
            let response = ui.button("Wait for it...");
            tooltip_with(ui, &theme, &response, "This tooltip has a 1 second delay", |t| {
                t.delay(1000)
            });
            ui.add_space(20.0);

            // Tooltip with custom position
            ui.horizontal(|ui| {
                ui.label("Positions:");

                let response = ui.button("Top");
                tooltip_with(ui, &theme, &response, "Tooltip on top", |t| {
                    t.position(TooltipPosition::Top)
                });

                let response = ui.button("Bottom");
                tooltip_with(ui, &theme, &response, "Tooltip on bottom", |t| {
                    t.position(TooltipPosition::Bottom)
                });

                let response = ui.button("Left");
                tooltip_with(ui, &theme, &response, "Tooltip on left", |t| {
                    t.position(TooltipPosition::Left)
                });

                let response = ui.button("Right");
                tooltip_with(ui, &theme, &response, "Tooltip on right", |t| {
                    t.position(TooltipPosition::Right)
                });
            });
            ui.add_space(20.0);

            // Auto positioning (default)
            ui.label("Auto Position (adapts to screen edges):");
            ui.horizontal(|ui| {
                let response = ui.button("Auto 1");
                tooltip(ui, &theme, &response, "This tooltip will automatically position itself to stay on screen");

                let response = ui.button("Auto 2");
                tooltip(ui, &theme, &response, "Position adapts based on available space");
            });
            ui.add_space(20.0);

            // No arrow
            ui.label("Without Arrow:");
            let response = ui.button("No arrow");
            tooltip_with(ui, &theme, &response, "This tooltip has no arrow", |t| {
                t.show_arrow(false)
            });
            ui.add_space(20.0);

            // With Armas Button component
            ui.label("With Armas Button:");
            ui.horizontal(|ui| {
                let response = Button::new("Primary")
                    .variant(ButtonVariant::Filled)
                    .show(ui);
                tooltip(ui, &theme, &response, "Primary action button");

                let response = Button::new("Secondary")
                    .variant(ButtonVariant::Outlined)
                    .show(ui);
                tooltip(ui, &theme, &response, "Secondary action button");

                let response = Button::new("Text")
                    .variant(ButtonVariant::Text)
                    .show(ui);
                tooltip(ui, &theme, &response, "Text-only button");
            });
            ui.add_space(20.0);

            // Edge cases
            ui.label("Try hovering near screen edges:");
            ui.horizontal(|ui| {
                let response = ui.button("Edge Test 1");
                tooltip(ui, &theme, &response, "This tooltip should stay on screen even when near edges");

                let response = ui.button("Edge Test 2");
                tooltip(ui, &theme, &response, "Auto-positioning ensures visibility");
            });
        });
    }
}
