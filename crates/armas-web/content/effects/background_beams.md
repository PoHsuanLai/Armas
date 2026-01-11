# Background Beams

Animated diagonal background beams effect with subtle animation.

## Basic Usage

```demo
use egui::Vec2;

let mut beams = BackgroundBeams::new(800.0, 600.0);
beams.show(ui);
```

## Custom Configuration

```demo
use egui::{Color32, Vec2};

let mut beams = BackgroundBeams::new(800.0, 600.0)
    .beam_count(12)
    .beam_width(120.0)
    .beam_angle(60.0)
    .opacity(0.2)
    .colors(vec![
        Color32::from_rgb(100, 150, 255),
        Color32::from_rgb(150, 100, 255),
    ]);

beams.show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `(f32, f32)` | - | Create with width and height |
| `.beam_count()` | `usize` | `8` | Number of beams |
| `.beam_width()` | `f32` | `100.0` | Width of each beam |
| `.beam_angle()` | `f32` | `45.0` | Beam angle in degrees |
| `.colors()` | `Vec<Color32>` | `gradient` | Beam colors |
| `.opacity()` | `f32` | `0.15` | Overall opacity (0.0-1.0) |
| `.animate()` | `bool` | `true` | Enable animation |
| `.blur()` | `bool` | `true` | Enable blur effect |

## Dependencies

- `egui = "0.33"`
- Theme colors: `gradient`
