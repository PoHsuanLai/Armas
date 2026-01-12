# Installation

## Prerequisites

Armas requires:
- **Rust** 1.70 or later
- **egui** 0.33

## Add to Your Project

Add Armas to your `Cargo.toml`:

```toml
[dependencies]
armas = "0.1.0"
egui = "0.33"
eframe = "0.33"  # For native/web apps
```

## Platform-Specific Setup

### Native Applications

For native desktop applications:

```toml
[dependencies]
armas = "0.1.0"
egui = "0.33"
eframe = "0.33"
```

### Web Applications (WASM)

For web applications, add the WASM target:

```bash
rustup target add wasm32-unknown-unknown
```

And use a bundler like Trunk:

```bash
cargo install trunk
trunk serve
```

### Bevy Integration

For Bevy game engine integration:

```toml
[dependencies]
armas = "0.1.0"
bevy = "0.12"
bevy_egui = "0.23"
```

## Verify Installation

Create a simple test to verify everything works:

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
            ui.heading("Armas is working!");

            Button::new("Test Button")
                .variant(ButtonVariant::Filled)
                .show(ui);
        });
    }
}
```

Run with:

```bash
cargo run
```

If you see a window with a working button, you're all set! 🎉

## Next Steps

- Follow the [Quick Start](/installation/quick_start) guide
- Learn about [Themes](/installation/themes)
- Explore [Examples](/installation/examples)
