# Animated Beam

Animated beam that follows a path with glowing effect.

## Basic Usage

```demo
use armas::components::PathPoint;

let path = vec![
    PathPoint::new(100.0, 100.0),
    PathPoint::new(300.0, 200.0),
    PathPoint::new(400.0, 100.0),
];

let mut beam = AnimatedBeam::new(path)
    .with_speed(0.5)
    .with_thickness(3.0);

beam.update(ui.input(|i| i.stable_dt));
beam.draw(ui);
```

## Custom Color

```demo
use armas::components::PathPoint;
use egui::Color32;

let path = vec![
    PathPoint::new(50.0, 50.0),
    PathPoint::new(250.0, 150.0),
];

let mut beam = AnimatedBeam::new(path)
    .with_color(Color32::from_rgb(255, 50, 50))
    .with_glow(0.9);

beam.update(ui.input(|i| i.stable_dt));
beam.draw(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `Vec<PathPoint>` | - | Create animated beam with path |
| `.with_speed()` | `f32` | `0.5` | Animation speed |
| `.with_color()` | `Color32` | `primary` | Beam color |
| `.with_thickness()` | `f32` | `3.0` | Beam thickness |
| `.with_glow()` | `f32` | `0.8` | Glow intensity (0.0-1.0) |
| `.with_loop_mode()` | `BeamLoopMode` | `Loop` | Loop, Once, or PingPong |
| `.with_gradient()` | `bool` | `true` | Enable gradient along beam |

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`
