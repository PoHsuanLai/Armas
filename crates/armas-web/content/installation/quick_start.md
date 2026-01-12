# Quick Start

## Installation

Add Armas to your `Cargo.toml`:

```toml
[dependencies]
armas = "0.1.0"
egui = "0.33"
```

## Your First Component

Here's a minimal example to get you started:

```rust
use armas::prelude::*;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My Armas App",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

struct MyApp {
    theme: Theme,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            theme: Theme::dark(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Button::new("Click me")
                .variant(ButtonVariant::Filled)
                .show(ui);
        });
    }
}
```

## Next Steps

- Explore the [Components](/components) section
- Learn about [Themes](/installation/themes)
- Check out [Layout](/layout) primitives
