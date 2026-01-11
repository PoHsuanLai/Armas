# Gradient Text

Text with animated gradient colors.

## Basic Usage

```demo
use egui::Color32;

let colors = vec![
    Color32::from_rgb(255, 0, 255),
    Color32::from_rgb(0, 255, 255),
];
GradientText::new("Gradient Text", colors).show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `&str` | - | Create gradient text |
| `.colors()` | `Vec<Color32>` | Default | Custom colors |

## Dependencies

- `egui = "0.33"`
- Gradient animation
