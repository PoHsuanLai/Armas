# Grid Pattern

Infinite grid background with optional perspective and fade effects.

## Basic Usage

```demo
use armas::Theme;

let theme = Theme::dark();
let grid = GridPattern::new(800.0, 600.0, 50.0, &theme);
grid.show(ui);
```

## Custom Configuration

```demo
use armas::Theme;
use egui::Color32;

let theme = Theme::dark();
let grid = GridPattern::new(800.0, 600.0, 40.0, &theme)
    .with_color(Color32::from_rgb(100, 150, 255))
    .with_dots(Color32::from_rgb(150, 200, 255), 3.0)
    .with_perspective(true)
    .with_fade(0.5)
    .with_thickness(1.5);

grid.show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `(f32, f32, f32, &Theme)` | - | Create with width, height, spacing, theme |
| `.with_color()` | `Color32` | `outline_variant` | Grid line color |
| `.with_dots()` | `(Color32, f32)` | `None` | Enable dots at intersections |
| `.with_fade()` | `f32` | `0.3` | Fade distance (0.0-1.0) |
| `.with_perspective()` | `bool` | `false` | Enable 3D perspective |
| `.with_thickness()` | `f32` | `1.0` | Line thickness |

## Dependencies

- `egui = "0.33"`
- Theme colors: `outline_variant`
