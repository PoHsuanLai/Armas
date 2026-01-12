# Dot Pattern

Background dot grid pattern effect.

## Basic Usage

```demo
DotPattern::new()
    .width(800.0)
    .height(400.0)
    .show(ui);
```

## Custom Spacing

```demo
use egui::Color32;

DotPattern::new()
    .width(800.0)
    .height(400.0)
    .spacing(30.0)
    .color(Color32::from_rgb(100, 150, 255))
    .show(ui);
```

## With Fade

```demo
DotPattern::new()
    .width(800.0)
    .height(400.0)
    .fade(0.7)
    .show(ui);
```

## With Glow

```demo
DotPattern::new()
    .width(800.0)
    .height(400.0)
    .glow(true)
    .show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create dot pattern |
| `.spacing()` | `f32` | `20.0` | Space between dots |
| `.size()` | `f32` | `2.0` | Dot size |
| `.color()` | `Color32` | `outline` | Dot color |
| `.opacity()` | `f32` | `0.3` | Dot opacity |

## Dependencies

- `egui = "0.33"`
- Theme colors: `outline`
