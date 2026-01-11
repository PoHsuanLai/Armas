# Lamp Effect

Dramatic horizontal lighting effect with symmetric conic gradients, perfect for hero sections.

## Basic Usage

```demo
let mut lamp = LampEffect::new(800.0, 600.0);
lamp.show(ui);
```

## Custom Color

```demo
use egui::Color32;

let mut lamp = LampEffect::new(800.0, 600.0)
    .lamp_color(Color32::from_rgb(255, 100, 100))
    .background_color(Color32::from_rgb(10, 10, 30))
    .animation_duration(1.0);

lamp.show(ui);
```

## With Content Overlay

```demo
use armas::Theme;

let theme = Theme::dark();
let mut lamp = LampEffect::new(800.0, 600.0);

lamp.show_with_content(ui, &theme, |ui| {
    ui.heading("Hero Title");
    ui.label("Dramatic content with lamp effect");
});
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `(f32, f32)` | - | Create with width and height |
| `.lamp_color()` | `Color32` | `cyan` | Light beam color |
| `.background_color()` | `Color32` | `dark slate` | Background color |
| `.animation_duration()` | `f32` | `0.8` | Animation duration in seconds |

## Dependencies

- `egui = "0.33"`
- Theme colors: None (uses built-in colors)
