# Glowing Border

Animated glowing border effect around content.

## Basic Usage

```demo
let mut border = GlowingBorder::new();
border.show(ui, &theme, |ui| {
    ui.label("Content with glow");
});
```

## Custom Color

```demo
use egui::Color32;

let mut border = GlowingBorder::new()
    .glow_color(Color32::from_rgb(239, 68, 68))
    .glow_intensity(1.5);
border.show(ui, &theme, |ui| {
    ui.heading("Custom Glow");
});
```

## Without Pulsing

```demo
let mut border = GlowingBorder::new()
    .pulse(false)
    .glow_color(theme.secondary());
border.show(ui, &theme, |ui| {
    ui.label("Static glow");
});
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create glowing border |
| `.width()` | `f32` | `None` | Container width (auto if not set) |
| `.height()` | `f32` | `None` | Container height (auto if not set) |
| `.glow_color()` | `Color32` | `Color32::from_rgb(59, 130, 246)` | Glow color |
| `.background()` | `Color32` | `Color32::from_gray(20)` | Background color |
| `.corner_radius()` | `f32` | `12.0` | Corner radius |
| `.border_width()` | `f32` | `2.0` | Border width |
| `.glow_intensity()` | `f32` | `1.0` | Glow intensity (0.0 to 2.0) |
| `.pulse_speed()` | `f32` | `1.0` | Pulse speed |
| `.pulse()` | `bool` | `true` | Enable/disable pulsing |
| `.show()` | `(&mut Ui, &Theme, impl FnOnce(&mut Ui))` | - | Show border with content |

## Dependencies

- `egui = "0.33"`
