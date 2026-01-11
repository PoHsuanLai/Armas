# Vortex Background

Swirling vortex effect with rotating particles in concentric rings.

## Basic Usage

```demo
let mut vortex = VortexBackground::new(800.0, 600.0);
vortex.show(ui);
```

## Custom Configuration

```demo
use egui::Color32;

let mut vortex = VortexBackground::new(800.0, 600.0)
    .particle_count(30)
    .ring_count(8)
    .rotation_speed(0.5)
    .particle_size(3.0)
    .radius_variation(0.3)
    .colors(vec![
        Color32::from_rgb(255, 100, 200),
        Color32::from_rgb(100, 200, 255),
        Color32::from_rgb(200, 255, 100),
    ]);

vortex.show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `(f32, f32)` | - | Create with width and height |
| `.particle_count()` | `usize` | `50` | Particles per ring |
| `.ring_count()` | `usize` | `6` | Number of concentric rings |
| `.colors()` | `Vec<Color32>` | `gradient` | Particle colors |
| `.rotation_speed()` | `f32` | `0.3` | Rotation speed |
| `.radius_variation()` | `f32` | `0.2` | Wobble effect (0.0-1.0) |
| `.particle_size()` | `f32` | `2.0` | Particle size |

## Dependencies

- `egui = "0.33"`
- Theme colors: `gradient`
