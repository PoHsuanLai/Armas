# Installation

## Requirements

- Rust 1.70+
- egui 0.33

## Setup

```toml
[dependencies]
armas = "0.1.0"
egui = "0.33"
eframe = "0.33"
```

### Web (WASM)

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve
```

### Bevy

```toml
[dependencies]
armas = "0.1.0"
bevy = "0.12"
bevy_egui = "0.23"
```

## Verify

```rust
use armas::prelude::*;
use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Armas Test",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(TestApp::default()))),
    )
}

#[derive(Default)]
struct TestApp;

impl eframe::App for TestApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Button::new("Test Button")
                .variant(ButtonVariant::Filled)
                .show(ui);
        });
    }
}
```

## Next Steps

- [Quick Start](/installation/quick_start)
- [Themes](/installation/themes)
