# Installation

## Requirements

- Rust 1.70+
- egui 0.33

## Setup

```toml
[dependencies]
armas = "0.2"
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
armas = "0.2"
bevy = "0.12"
bevy_egui = "0.23"
```

## Verify

```rust
use armas::prelude::*;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Armas Test",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(TestApp))
        }),
    )
}

struct TestApp;

impl eframe::App for TestApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Button::new("Test Button")
                .variant(ButtonVariant::Default)
                .show(ui);
        });
    }
}
```

## Next Steps

- [Quick Start](/installation/quick_start)
- [Themes](/installation/themes)
