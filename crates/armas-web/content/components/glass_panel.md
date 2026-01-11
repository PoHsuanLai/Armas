# Glass Panel

Glassmorphism effect panel with backdrop blur.

## Basic Usage

```demo
GlassPanel::new()
    .show(ui, &theme, |ui| {
        ui.heading("Glass Effect");
        ui.label("Content with blur backdrop");
    });
```

## With Title

```demo
GlassPanel::new()
    .title("Settings")
    .show(ui, &theme, |ui| {
        ui.label("Panel with title");
    });
```

## Custom Styling

```demo
GlassPanel::new()
    .opacity(0.9)
    .glow_intensity(0.5)
    .corner_radius(16.0)
    .show(ui, &theme, |ui| {
        ui.label("Custom styled panel");
    });
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create glass panel |
| `.title()` | `&str` | `None` | Optional panel title |
| `.opacity()` | `f32` | `0.7` | Background opacity (0.0-1.0) |
| `.blur()` | `f32` | `10.0` | Blur amount (cosmetic only) |
| `.glow_intensity()` | `f32` | `0.3` | Border glow intensity (0.0-1.0) |
| `.width()` | `f32` | `None` | Custom width (None = fill available) |
| `.corner_radius()` | `f32` | `theme.spacing.corner_radius` | Corner radius |
| `.inner_margin()` | `f32` | `theme.spacing.spacing_medium` | Inner padding |
| `.show()` | `(&mut Ui, &Theme, impl FnOnce(&mut Ui))` | - | Show panel with content |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `outline_variant`, `on_surface`, `primary`
