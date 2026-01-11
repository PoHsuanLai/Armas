# Retro Grid

Cyberpunk-style perspective grid with animated lines and horizon glow.

## Basic Usage

```demo
let mut grid = RetroGrid::new(800.0, 600.0);
grid.show(ui);
```

## Custom Configuration

```demo
use egui::Color32;

let mut grid = RetroGrid::new(800.0, 600.0)
    .grid_color(Color32::from_rgb(0, 255, 255))
    .horizon_color(Color32::from_rgb(255, 0, 255))
    .cell_size(60.0)
    .perspective_depth(0.7)
    .animation_speed(30.0);

grid.show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `(f32, f32)` | - | Create with width and height |
| `.grid_color()` | `Color32` | `cyan` | Grid line color |
| `.horizon_color()` | `Color32` | `magenta` | Horizon glow color |
| `.cell_size()` | `f32` | `50.0` | Grid cell size |
| `.perspective_depth()` | `f32` | `0.6` | Perspective amount (0.0-1.0) |
| `.animate()` | `bool` | `true` | Enable animation |
| `.animation_speed()` | `f32` | `20.0` | Animation speed |

## Dependencies

- `egui = "0.33"`
- Theme colors: None (uses built-in cyan/magenta)
