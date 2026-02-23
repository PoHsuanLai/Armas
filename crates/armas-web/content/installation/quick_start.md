# Quick Start

```toml
[dependencies]
armas = "0.2"
egui = "0.33"
eframe = "0.33"
```

```rust
use armas::prelude::*;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "My App",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(MyApp))
        }),
    )
}

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Button::new("Click me")
                .variant(ButtonVariant::Default)
                .show(ui);
        });
    }
}
```

## Next Steps

- [Components](/components)
- [Themes](/installation/themes)
