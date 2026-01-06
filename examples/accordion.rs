use armas::{Accordion, AccordionItem, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Accordion Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Accordion Demo",
        options,
        Box::new(|_cc| Ok(Box::new(AccordionApp::new()))),
    )
}

struct AccordionApp {
    theme: Theme,
    items: Vec<AccordionItem>,
}

impl AccordionApp {
    fn new() -> Self {
        Self {
            theme: Theme::dark(),
            items: vec![
                AccordionItem::new("Getting Started").open(true),
                AccordionItem::new("Installation"),
                AccordionItem::new("Configuration"),
                AccordionItem::new("Advanced Topics"),
            ],
        }
    }
}

impl eframe::App for AccordionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Accordion Component");
            ui.add_space(20.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                Accordion::new().allow_multiple(false).show(
                    ui,
                    &self.theme,
                    &mut self.items,
                    |ui, index| match index {
                        0 => {
                            ui.label("Welcome to the component library!");
                            ui.add_space(8.0);
                            ui.label("This accordion demonstrates collapsible sections.");
                            ui.add_space(8.0);
                            ui.label("Click on any header to expand or collapse its content.");
                        }
                        1 => {
                            ui.label("Add armas to your Cargo.toml:");
                            ui.add_space(8.0);
                            ui.code("[dependencies]\narmas = \"0.1\"");
                            ui.add_space(8.0);
                            ui.label("Then import the components you need.");
                        }
                        2 => {
                            ui.label("Configure your theme:");
                            ui.add_space(8.0);
                            ui.code("let theme = Theme::dark();");
                            ui.add_space(8.0);
                            ui.label("Or create a custom theme with your own colors.");
                        }
                        3 => {
                            ui.label("Advanced features include:");
                            ui.add_space(8.0);
                            ui.label("• Animation system with spring physics");
                            ui.label("• Gradient cards with mesh rendering");
                            ui.label("• Spotlight effects with mouse tracking");
                            ui.label("• Responsive layouts and auto-sizing");
                        }
                        _ => {}
                    },
                );
            });
        });
    }
}
