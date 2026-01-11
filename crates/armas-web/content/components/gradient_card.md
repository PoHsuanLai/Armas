# Gradient Card

Card with animated gradient border.

## Basic Usage

```demo
let mut card = GradientCard::new();
card.show(ui, &theme, |ui| {
    ui.heading("Gradient Card");
    ui.label("Beautiful animated gradient border");
});
```

## Custom Colors

```demo
let mut card = GradientCard::new()
    .gradient_colors(vec![
        theme.primary(),
        theme.secondary(),
        theme.error(),
    ]);
card.show(ui, &theme, |ui| {
    ui.label("Custom gradient");
});
```

## Preset Gradients

```demo
// Rainbow gradient
let mut card = GradientCard::rainbow();
card.show(ui, &theme, |ui| {
    ui.label("Rainbow");
});

// Warm gradient
let mut card = GradientCard::warm();
card.show(ui, &theme, |ui| {
    ui.label("Warm");
});

// Cool gradient
let mut card = GradientCard::cool();
card.show(ui, &theme, |ui| {
    ui.label("Cool");
});

// Neon gradient
let mut card = GradientCard::neon();
card.show(ui, &theme, |ui| {
    ui.label("Neon");
});

// Gold gradient
let mut card = GradientCard::gold();
card.show(ui, &theme, |ui| {
    ui.label("Gold");
});
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create gradient card |
| `.width()` | `f32` | `None` | Card width (None = fill available) |
| `.height()` | `f32` | `None` | Card height (None = auto-size) |
| `.border_width()` | `f32` | `2.0` | Border width in pixels |
| `.corner_radius()` | `f32` | `8.0` | Corner radius |
| `.gradient_colors()` | `Vec<Color32>` | `[blue, purple, pink]` | Gradient colors (min 2) |
| `.rotation_speed()` | `f32` | `PI / 4.0` | Rotation speed (radians/sec) |
| `.animate()` | `bool` | `true` | Enable/disable animation |
| `.background_color()` | `Color32` | `theme.surface()` | Background color |
| `.glow_on_hover()` | `bool` | `true` | Enable glow effect on hover |
| `.show()` | `(&mut Ui, &Theme, impl FnOnce(&mut Ui))` | - | Show card with content |

## Preset Methods

| Method | Description |
|--------|-------------|
| `::blue_purple()` | Blue to purple gradient (default) |
| `::rainbow()` | Rainbow gradient |
| `::warm()` | Red to orange to yellow gradient |
| `::cool()` | Cyan to blue to purple gradient |
| `::neon()` | Bright neon colors |
| `::gold()` | Gold gradient |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`
