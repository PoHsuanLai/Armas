# Moving Border

Button with animated gradient border that travels around the edges.

## Basic Usage

```demo
let mut button = MovingBorder::new("Click Me");
if button.show(ui).clicked() {
    // Handle click
}
```

## Custom Configuration

```demo
use egui::Color32;

let mut button = MovingBorder::new("Subscribe")
    .width(200.0)
    .height(50.0)
    .border_width(3.0)
    .border_colors(vec![
        Color32::from_rgb(255, 0, 255),
        Color32::from_rgb(0, 255, 255),
        Color32::from_rgb(255, 255, 0),
    ])
    .animation_speed(1.5);

button.show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `impl Into<String>` | - | Create with button text |
| `.width()` | `f32` | `auto` | Button width |
| `.height()` | `f32` | `40.0` | Button height |
| `.border_width()` | `f32` | `2.0` | Border thickness |
| `.border_colors()` | `Vec<Color32>` | `gradient` | Gradient colors |
| `.background()` | `Color32` | `surface` | Background color |
| `.text_color()` | `Color32` | `on_surface` | Text color |
| `.corner_radius()` | `f32` | `8.0` | Corner radius |
| `.animation_speed()` | `f32` | `1.0` | Animation speed |

## Dependencies

- `egui = "0.33"`
- Theme colors: `gradient`, `surface`, `on_surface`
