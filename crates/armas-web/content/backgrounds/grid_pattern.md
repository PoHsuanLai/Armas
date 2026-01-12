# Grid Pattern

Infinite grid background with optional perspective and fade effects.

## Basic Usage

```demo
GridPattern::new()
    .width(800.0)
    .height(400.0)
    .spacing(50.0)
    .show(ui);
```

## With Perspective

```demo
GridPattern::new()
    .width(800.0)
    .height(400.0)
    .spacing(40.0)
    .perspective(true)
    .show(ui);
```

## With Dots at Intersections

```demo
use egui::Color32;

GridPattern::new()
    .width(800.0)
    .height(400.0)
    .spacing(50.0)
    .dots(Color32::from_rgb(150, 200, 255), 3.0)
    .show(ui);
```

## Custom Configuration

```demo
use egui::Color32;

GridPattern::new()
    .width(800.0)
    .height(400.0)
    .spacing(40.0)
    .color(Color32::from_rgb(100, 150, 255))
    .dots(Color32::from_rgb(150, 200, 255), 3.0)
    .perspective(true)
    .fade(0.5)
    .thickness(1.5)
    .show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `(f32, f32, f32, &Theme)` | - | Create with width, height, spacing, theme |
| `.color()` | `Color32` | `outline_variant` | Grid line color |
| `.dots()` | `(Color32, f32)` | `None` | Enable dots at intersections |
| `.fade()` | `f32` | `0.3` | Fade distance (0.0-1.0) |
| `.perspective()` | `bool` | `false` | Enable 3D perspective |
| `.thickness()` | `f32` | `1.0` | Line thickness |

## Dependencies

- `egui = "0.33"`
- Theme colors: `outline_variant`
