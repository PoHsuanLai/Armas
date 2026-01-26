# Quick Start

```toml
[dependencies]
armas = "0.1.0"
egui = "0.33"
```

```rust
use armas::prelude::*;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "My App",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

struct MyApp {
    theme: Theme,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { theme: Theme::dark() }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Button::new("Click me")
                .variant(ButtonVariant::Filled)
                .show(ui, &theme);
        });
    }
}
```

## Next Steps

- [Components](/components)
- [Themes](/installation/themes)
