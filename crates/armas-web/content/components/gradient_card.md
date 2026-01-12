# Gradient Card

Card with animated gradient border.

## Basic Usage

```demo
GradientCard::new().show(ui, &theme, |ui| {
    ui.heading("Gradient Card");
    ui.label("Beautiful animated gradient border");
    ui.add_space(8.0);
    ui.label("Hover to see the glow effect!");
});
```

## Custom Colors

```demo
GradientCard::new()
    .gradient_colors(vec![
        theme.primary(),
        theme.secondary(),
        theme.error(),
    ])
    .show(ui, &theme, |ui| {
        ui.heading("Custom Colors");
        ui.label("Using theme colors for the gradient");
    });
```

## Preset Gradients

```demo
GradientCard::rainbow().show(ui, &theme, |ui| {
    ui.heading("Rainbow");
    ui.label("All the colors of the rainbow");
});
```

```demo
GradientCard::warm().show(ui, &theme, |ui| {
    ui.heading("Warm Gradient");
    ui.label("Red to orange to yellow");
});
```

```demo
GradientCard::cool().show(ui, &theme, |ui| {
    ui.heading("Cool Gradient");
    ui.label("Cyan to blue to purple");
});
```

```demo
GradientCard::neon().show(ui, &theme, |ui| {
    ui.heading("Neon Gradient");
    ui.label("Bright and saturated!");
});
```

```demo
GradientCard::gold().show(ui, &theme, |ui| {
    ui.heading("Gold Gradient");
    ui.label("Shiny and premium");
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
