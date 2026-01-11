use armas::ext::ArmasContextExt;
use armas::{InputState, InputVariant, Textarea, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 900.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Textarea Component Example",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(TextareaExample::default()))
        }),
    )
}

struct TextareaExample {
    basic_text: String,
    outlined_text: String,
    filled_text: String,
    with_label: String,
    with_helper: String,
    limited_text: String,
    success_text: String,
    error_text: String,
    warning_text: String,
    fixed_rows: String,
}

impl Default for TextareaExample {
    fn default() -> Self {
        Self {
            basic_text: String::new(),
            outlined_text: String::new(),
            filled_text: String::new(),
            with_label: String::from("Some initial text..."),
            with_helper: String::new(),
            limited_text: String::new(),
            success_text: String::from("This is valid!"),
            error_text: String::from("Error message here"),
            warning_text: String::from("Warning: check this"),
            fixed_rows: String::from(
                "This textarea\nhas a fixed\nnumber of rows\nand cannot be resized.",
            ),
        }
    }
}

impl eframe::App for TextareaExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Textarea Component Examples");
                ui.add_space(20.0);

                // Basic variants
                ui.label("Variants:");
                ui.add_space(10.0);

                Textarea::new("Enter default textarea...")
                    .variant(InputVariant::Default)
                    .show(ui,&mut self.basic_text);
                ui.add_space(10.0);

                Textarea::new("Enter outlined textarea...")
                    .variant(InputVariant::Outlined)
                    .show(ui,&mut self.outlined_text);
                ui.add_space(10.0);

                Textarea::new("Enter filled textarea...")
                    .variant(InputVariant::Filled)
                    .show(ui,&mut self.filled_text);
                ui.add_space(20.0);

                ui.separator();
                ui.add_space(20.0);

                // With label
                ui.label("With Label:");
                ui.add_space(10.0);

                Textarea::new("Enter your description...")
                    .label("Description")
                    .variant(InputVariant::Outlined)
                    .show(ui,&mut self.with_label);
                ui.add_space(20.0);

                ui.separator();
                ui.add_space(20.0);

                // With helper text
                ui.label("With Helper Text:");
                ui.add_space(10.0);

                Textarea::new("Enter your comment...")
                    .label("Comment")
                    .helper_text("This will be visible to other users")
                    .variant(InputVariant::Outlined)
                    .show(ui,&mut self.with_helper);
                ui.add_space(20.0);

                ui.separator();
                ui.add_space(20.0);

                // With character limit
                ui.label("With Character Limit (200 chars):");
                ui.add_space(10.0);

                Textarea::new("Type something...")
                    .label("Limited Text")
                    .helper_text("Maximum 200 characters")
                    .max_chars(200)
                    .variant(InputVariant::Outlined)
                    .show(ui,&mut self.limited_text);
                ui.add_space(20.0);

                ui.separator();
                ui.add_space(20.0);

                // Different states
                ui.label("Different States:");
                ui.add_space(10.0);

                Textarea::new("Success state...")
                    .label("Success")
                    .state(InputState::Success)
                    .helper_text("Looks good!")
                    .variant(InputVariant::Outlined)
                    .show(ui,&mut self.success_text);
                ui.add_space(10.0);

                Textarea::new("Error state...")
                    .label("Error")
                    .state(InputState::Error)
                    .helper_text("This field has an error")
                    .variant(InputVariant::Outlined)
                    .show(ui,&mut self.error_text);
                ui.add_space(10.0);

                Textarea::new("Warning state...")
                    .label("Warning")
                    .state(InputState::Warning)
                    .helper_text("Please review this field")
                    .variant(InputVariant::Outlined)
                    .show(ui,&mut self.warning_text);
                ui.add_space(20.0);

                ui.separator();
                ui.add_space(20.0);

                // Different row counts
                ui.label("Different Row Counts:");
                ui.add_space(10.0);

                let mut temp1 = String::new();
                Textarea::new("Small (2 rows)...")
                    .label("Small")
                    .rows(2)
                    .variant(InputVariant::Outlined)
                    .show(ui,&mut temp1);
                ui.add_space(10.0);

                let mut temp2 = String::new();
                Textarea::new("Large (8 rows)...")
                    .label("Large")
                    .rows(8)
                    .variant(InputVariant::Outlined)
                    .show(ui,&mut temp2);
                ui.add_space(20.0);

                ui.separator();
                ui.add_space(20.0);

                // Non-resizable
                ui.label("Non-resizable (Fixed Rows):");
                ui.add_space(10.0);

                Textarea::new("Cannot resize...")
                    .label("Fixed Size")
                    .rows(4)
                    .resizable(false)
                    .variant(InputVariant::Outlined)
                    .show(ui,&mut self.fixed_rows);
                ui.add_space(20.0);

                ui.separator();
                ui.add_space(20.0);

                // Custom width
                ui.label("Custom Width:");
                ui.add_space(10.0);

                let mut temp3 = String::new();
                Textarea::new("Fixed width textarea...")
                    .label("400px Wide")
                    .width(400.0)
                    .variant(InputVariant::Outlined)
                    .show(ui,&mut temp3);
            });
        });
    }
}
