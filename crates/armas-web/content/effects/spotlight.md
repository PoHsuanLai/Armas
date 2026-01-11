# Spotlight

Interactive spotlight effect that follows the mouse cursor, creating a dramatic lighting effect.

## Basic Usage

```demo
let theme = ui.ctx().armas_theme();
let mut spotlight = Spotlight::new(&theme);
spotlight.show(ui, &theme, |ui| {
    ui.vertical_centered(|ui| {
        ui.add_space(80.0);
        ui.label("Spotlight Effect");
    });
});
```

## Custom Radius

```demo
let theme = ui.ctx().armas_theme();
let mut spotlight = Spotlight::new(&theme).radius(200.0);
spotlight.show(ui, &theme, |ui| {
    ui.vertical_centered(|ui| {
        ui.add_space(80.0);
        ui.label("Large Spotlight");
    });
});
```

## With Smoothing

```demo
let theme = ui.ctx().armas_theme();
let mut spotlight = Spotlight::new(&theme)
    .radius(150.0)
    .smoothing(0.15);
spotlight.show(ui, &theme, |ui| {
    ui.vertical_centered(|ui| {
        ui.add_space(80.0);
        ui.label("Smooth Spotlight");
    });
});
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.radius()` | `f32` | `100.0` | Spotlight radius in pixels |
| `.smoothing()` | `f32` | `0.1` | Mouse movement smoothing (0-1) |

## Features

- **Interactive**: Follows mouse cursor position
- **Smooth**: Configurable smoothing for fluid movement
- **Performance**: 60fps, GPU accelerated where possible
- **Customizable**: Adjustable radius and smoothing

## Dependencies

- `egui = "0.33"`
- Theme colors: Uses theme background and primary colors
