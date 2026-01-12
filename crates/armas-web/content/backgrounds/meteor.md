# Meteor Shower

Continuous meteor shower with shooting stars across the screen.

## Basic Usage

```demo
use armas::Theme;

let theme = Theme::dark();
MeteorShower::new(ui.available_width(), 600.0, &theme)
    .id("meteor_basic")
    .show(ui);
```

## Custom Configuration

```demo
use armas::Theme;
use egui::Color32;
use std::f32::consts::PI;

let theme = Theme::dark();
MeteorShower::new(ui.available_width(), 600.0, &theme)
    .id("meteor_custom")
    .spawn_rate(2.0)  // 2 meteors per second
    .angle(PI / 4.0)  // 45 degrees
    .color(Color32::from_rgb(255, 200, 100))
    .speed_range(0.6, 1.5)
    .show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(width, height, &Theme)` | `(f32, f32, &Theme)` | - | Create meteor shower |
| `.spawn_rate()` | `f32` | `2.0` | Meteors spawned per second |
| `.angle()` | `f32` | `PI/4` | Meteor angle in radians |
| `.color()` | `Color32` | `primary` | Meteor color |
| `.speed_range()` | `(f32, f32)` | `(0.8, 1.2)` | Min and max speed |

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`
