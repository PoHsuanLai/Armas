# Examples

Armas comes with 20+ examples demonstrating various components and features.

## Running Examples

```bash
# List all examples
cargo run --example

# Run a specific example
cargo run --example button
cargo run --example animation
cargo run --example layout
```

## Available Examples

### Components
- `button` - Button variants and states
- `card` - Card layouts and styles
- `fader` - Audio fader controls
- `loading` - Loading indicators (Spinner, Dots, Skeleton, Progress)
- `testimonial` - Testimonial cards
- `accordion` - Collapsible content panels
- `tabs` - Tab navigation

### Effects
- `animation` - Animation system demo
- `scrolling_banner` - Infinite scrolling text
- `spotlight` - Mouse-following spotlight effect
- `retro_grid` - Animated retro grid background

### Layout
- `layout` - Layout system primitives (VStack, HStack, Grid)
- `grid_test` - Responsive grid layouts

## Example Structure

Each example is self-contained and follows this pattern:

```rust
use armas::prelude::*;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Example Name",
        native_options,
        Box::new(|cc| Ok(Box::new(MyExample::new(cc)))),
    )
}

struct MyExample {
    theme: Theme,
}

impl MyExample {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            theme: Theme::dark(),
        }
    }
}

impl eframe::App for MyExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Your component code here
        });
    }
}
```
