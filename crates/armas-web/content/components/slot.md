# Slot

Insert slot component (Studio One style).

## Empty Slot

```demo
Slot::new(80.0, 24.0)
    .show(ui);
// Shows "+" indicating empty slot
```

## With Effect

```demo
Slot::new(80.0, 24.0)
    .with_effect("Reverb")
    .level(0.7)
    .show(ui);
```

## Bypassed Effect

```demo
Slot::new(80.0, 24.0)
    .with_effect("EQ")
    .bypassed(true)
    .level(0.5)
    .show(ui);
```

## Different Effect Types

```demo
// Reverb (blue - spatial/time-based)
Slot::new(80.0, 24.0)
    .with_effect("Reverb")
    .show(ui);

// EQ (green - corrective)
Slot::new(80.0, 24.0)
    .with_effect("EQ")
    .show(ui);

// Compressor (orange - dynamic)
Slot::new(80.0, 24.0)
    .with_effect("Compressor")
    .show(ui);

// Chorus (purple - modulation)
Slot::new(80.0, 24.0)
    .with_effect("Chorus")
    .show(ui);

// Distortion (red - aggressive)
Slot::new(80.0, 24.0)
    .with_effect("Distortion")
    .show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(width, height)` | `(f32, f32)` | - | Create insert slot |
| `.with_effect()` | `&str` | `None` | Set effect name |
| `.bypassed()` | `bool` | `false` | Set bypass state |
| `.level()` | `f32` | `0.0` | Activity level (0.0-1.0) for mini meter |
| `.show(&mut Ui)` | - | - | Show the slot |

## Effect Color Coding

The component automatically colors effects based on their name:
- Blue: Reverb, Delay, Echo (spatial/time-based)
- Green: EQ, Filter (corrective/clean)
- Orange: Compressor, Limiter, Gate (dynamic)
- Purple: Chorus, Flanger, Phaser (modulation)
- Red: Distortion, Drive, Saturation (aggressive)
- Primary: Other effects

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `outline_variant`, `on_surface`, `on_surface_variant`, `primary`, `info`, `success`, `warning`, `secondary`, `error`
