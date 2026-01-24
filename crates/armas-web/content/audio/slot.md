# Slot

Plugin/effect insert slot with activity meter and bypass indicator.

## Empty Slot

```demo
Slot::new()
    .show(ui);
// Shows "+" indicating empty slot
```

## With Effect

```demo
Slot::new()
    .effect("Reverb")
    .level(0.7)
    .show(ui);
```

## Bypassed Effect

```demo
Slot::new()
    .effect("EQ")
    .bypassed(true)
    .level(0.5)
    .show(ui);
```

## Different Effect Types

```demo
ui.horizontal(|ui| {
    // Reverb (blue - spatial/time-based)
    Slot::new()
        .effect("Reverb")
        .level(0.6)
        .show(ui);

    // EQ (green - corrective)
    Slot::new()
        .effect("EQ")
        .level(0.4)
        .show(ui);
});

ui.horizontal(|ui| {
    // Compressor (orange - dynamic)
    Slot::new()
        .effect("Compressor")
        .level(0.8)
        .show(ui);

    // Chorus (purple - modulation)
    Slot::new()
        .effect("Chorus")
        .level(0.5)
        .show(ui);
});

ui.horizontal(|ui| {
    // Distortion (red - aggressive)
    Slot::new()
        .effect("Distortion")
        .level(0.9)
        .show(ui);

    // Empty slot
    Slot::new()
        .show(ui);
});
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | `120x36` | Create insert slot with default size |
| `.size()` | `(f32, f32)` | - | Set custom width and height |
| `.width()` | `f32` | `120.0` | Set width |
| `.height()` | `f32` | `36.0` | Set height |
| `.effect()` | `&str` | `None` | Set effect name |
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
