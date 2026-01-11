//! Button component demo
//!
//! Run with: cargo run --example button_demo

use armas::ext::ArmasContextExt;
use armas::{Button, ButtonVariant, Theme};
use eframe::egui::{self, Vec2};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_title("Button Demo - egui-alig"),
        ..Default::default()
    };

    eframe::run_native(
        "Button Demo",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(ButtonDemoApp::default()))
        }),
    )
}

struct ButtonDemoApp {
    current_theme: usize,
    click_count: usize,
    buttons_enabled: bool,
}

impl Default for ButtonDemoApp {
    fn default() -> Self {
        Self {
            current_theme: 0, // Start with Dark theme
            click_count: 0,
            buttons_enabled: true,
        }
    }
}

const THEMES: &[(&str, fn() -> Theme)] = &[
    ("Dark (M3)", Theme::dark),
    ("Light (M3)", Theme::light),
    ("Nord", Theme::nord),
    ("Dracula", Theme::dracula),
    ("Studio", Theme::studio),
];

impl eframe::App for ButtonDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set theme in context
        ctx.set_armas_theme(THEMES[self.current_theme].1());

        // Set visuals based on theme (Light vs Dark)
        ctx.set_visuals(if self.current_theme == 1 {
            egui::Visuals::light()
        } else {
            egui::Visuals::dark()
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("egui-alig Button Component Demo");
                ui.add_space(20.0);

                // Theme selector
                ui.horizontal(|ui| {
                    ui.label("Theme:");
                    for (i, (name, _)) in THEMES.iter().enumerate() {
                        if ui
                            .selectable_label(self.current_theme == i, *name)
                            .clicked()
                        {
                            self.current_theme = i;
                        }
                    }
                });
                ui.add_space(10.0);

                // Enable/disable toggle
                ui.checkbox(&mut self.buttons_enabled, "Enable buttons");
                ui.add_space(20.0);

                // Filled buttons (highest emphasis)
                ui.heading("Filled Buttons (Highest Emphasis)");
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if Button::new("Save")
                        .variant(ButtonVariant::Filled)
                        .enabled(self.buttons_enabled)
                        .show(ui)
                        .clicked()
                    {
                        self.click_count += 1;
                    }

                    if Button::new("Submit")
                        .variant(ButtonVariant::Filled)
                        .enabled(self.buttons_enabled)
                        .show(ui)
                        .clicked()
                    {
                        self.click_count += 1;
                    }
                });
                ui.add_space(20.0);

                // Filled Tonal buttons (medium-high emphasis)
                ui.heading("Filled Tonal Buttons (Medium-High Emphasis)");
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if Button::new("Next")
                        .variant(ButtonVariant::FilledTonal)
                        .enabled(self.buttons_enabled)
                        .show(ui)
                        .clicked()
                    {
                        self.click_count += 1;
                    }

                    if Button::new("Continue")
                        .variant(ButtonVariant::FilledTonal)
                        .enabled(self.buttons_enabled)
                        .show(ui)
                        .clicked()
                    {
                        self.click_count += 1;
                    }
                });
                ui.add_space(20.0);

                // Elevated buttons (tonal + shadow)
                ui.heading("Elevated Buttons (Tonal + Shadow)");
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if Button::new("Elevated")
                        .variant(ButtonVariant::Elevated)
                        .enabled(self.buttons_enabled)
                        .show(ui)
                        .clicked()
                    {
                        self.click_count += 1;
                    }

                    if Button::new("With Shadow")
                        .variant(ButtonVariant::Elevated)
                        .enabled(self.buttons_enabled)
                        .show(ui)
                        .clicked()
                    {
                        self.click_count += 1;
                    }
                });
                ui.add_space(20.0);

                // Outlined buttons (medium emphasis)
                ui.heading("Outlined Buttons (Medium Emphasis)");
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if Button::new("Outlined")
                        .variant(ButtonVariant::Outlined)
                        .enabled(self.buttons_enabled)
                        .show(ui)
                        .clicked()
                    {
                        self.click_count += 1;
                    }

                    if Button::new("Secondary Action")
                        .variant(ButtonVariant::Outlined)
                        .min_size(egui::vec2(140.0, 36.0))
                        .enabled(self.buttons_enabled)
                        .show(ui)
                        .clicked()
                    {
                        self.click_count += 1;
                    }
                });
                ui.add_space(20.0);

                // Text buttons (lowest emphasis)
                ui.heading("Text Buttons (Lowest Emphasis)");
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if Button::new("Cancel")
                        .variant(ButtonVariant::Text)
                        .enabled(self.buttons_enabled)
                        .show(ui)
                        .clicked()
                    {
                        self.click_count += 1;
                    }

                    if Button::new("Learn More")
                        .variant(ButtonVariant::Text)
                        .enabled(self.buttons_enabled)
                        .show(ui)
                        .clicked()
                    {
                        self.click_count += 1;
                    }
                });
                ui.add_space(20.0);

                // Disabled state
                ui.heading("Disabled State");
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    Button::new("Filled")
                        .variant(ButtonVariant::Filled)
                        .enabled(false)
                        .show(ui);

                    Button::new("Tonal")
                        .variant(ButtonVariant::FilledTonal)
                        .enabled(false)
                        .show(ui);

                    Button::new("Elevated")
                        .variant(ButtonVariant::Elevated)
                        .enabled(false)
                        .show(ui);

                    Button::new("Outlined")
                        .variant(ButtonVariant::Outlined)
                        .enabled(false)
                        .show(ui);

                    Button::new("Text")
                        .variant(ButtonVariant::Text)
                        .enabled(false)
                        .show(ui);

                    Button::new("Speaker")
                        .variant(ButtonVariant::Speaker)
                        .enabled(false)
                        .show(ui);
                });

                ui.add_space(30.0);
                ui.separator();
                ui.add_space(20.0);

                // Speaker-style buttons (modern audio hardware aesthetic)
                ui.heading("Speaker-Style Buttons (Modern Audio Hardware)");
                ui.label("Sleek, minimal plastic aesthetic - perfect for audio controls");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if Button::new("▶ Play")
                        .variant(ButtonVariant::Speaker)
                        .min_size(Vec2::new(100.0, 40.0))
                        .enabled(self.buttons_enabled)
                        .show(ui)
                        .clicked()
                    {
                        self.click_count += 1;
                    }

                    if Button::new("⏸ Pause")
                        .variant(ButtonVariant::Speaker)
                        .min_size(Vec2::new(100.0, 40.0))
                        .enabled(self.buttons_enabled)
                        .show(ui)
                        .clicked()
                    {
                        self.click_count += 1;
                    }

                    if Button::new("⏹ Stop")
                        .variant(ButtonVariant::Speaker)
                        .min_size(Vec2::new(100.0, 40.0))
                        .enabled(self.buttons_enabled)
                        .show(ui)
                        .clicked()
                    {
                        self.click_count += 1;
                    }
                });

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if Button::new("🔇 Mute")
                        .variant(ButtonVariant::Speaker)
                        .min_size(Vec2::new(90.0, 36.0))
                        .enabled(self.buttons_enabled)
                        .show(ui)
                        .clicked()
                    {
                        self.click_count += 1;
                    }

                    if Button::new("Solo")
                        .variant(ButtonVariant::Speaker)
                        .min_size(Vec2::new(90.0, 36.0))
                        .enabled(self.buttons_enabled)
                        .show(ui)
                        .clicked()
                    {
                        self.click_count += 1;
                    }

                    if Button::new("● Rec")
                        .variant(ButtonVariant::Speaker)
                        .min_size(Vec2::new(90.0, 36.0))
                        .enabled(self.buttons_enabled)
                        .show(ui)
                        .clicked()
                    {
                        self.click_count += 1;
                    }
                });

                ui.add_space(30.0);
                ui.separator();
                ui.add_space(10.0);

                // Click counter
                ui.label(format!("Total clicks: {}", self.click_count));
                if ui.button("Reset counter").clicked() {
                    self.click_count = 0;
                }
            });
        });
    }
}
