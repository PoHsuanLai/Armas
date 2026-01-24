# Feature Grid

Responsive grid layout for displaying features with icons, titles, and descriptions.

## Basic Usage

```demo
FeatureGrid::new().show(ui, |grid| {
    grid.feature("*", "Fast", "Lightning speed performance");
    grid.feature("*", "Secure", "Enterprise-grade security");
    grid.feature("*", "Scalable", "Grows with your needs");
});
```

## Custom Columns

```demo
FeatureGrid::new()
    .columns(2)
    .show(ui, |grid| {
        grid.feature("*", "Easy", "Simple to use");
        grid.feature("*", "Powerful", "Advanced features");
        grid.feature("*", "Fast", "Optimized performance");
    });
```

## Custom Icon Colors

```demo
use egui::Color32;

FeatureGrid::new().show(ui, |grid| {
    grid.feature_with_color(
        "*",
        "Innovation",
        "Creative solutions",
        Color32::from_rgb(255, 200, 0)
    );
    grid.feature_with_color(
        "*",
        "Design",
        "Beautiful interfaces",
        Color32::from_rgb(200, 0, 255)
    );
    grid.feature_with_color(
        "*",
        "Engineering",
        "Rock-solid code",
        Color32::from_rgb(0, 200, 255)
    );
});
```

## Custom Styling

```demo
FeatureGrid::new()
    .columns(2)
    .gap(30.0)
    .show_borders(false)
    .hover_effect(true)
    .icon_size(48.0)
    .show(ui, |grid| {
        grid.feature("*", "Hot Features", "The latest and greatest");
        grid.feature("*", "Cool Features", "Stay chill with stability");
    });
```

## API Reference

### FeatureGrid

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create new feature grid |
| `.columns()` | `usize` | `auto` | Number of columns |
| `.gap()` | `f32` | `20.0` | Gap between items |
| `.show_borders()` | `bool` | `true` | Show borders between items |
| `.hover_effect()` | `bool` | `true` | Enable hover effect |
| `.icon_size()` | `f32` | `32.0` | Icon size in pixels |

### GridBuilder (inside closure)

| Method | Type | Description |
|--------|------|-------------|
| `.feature(icon, title, desc)` | `(&str, &str, &str)` | Add feature item |
| `.feature_with_color(...)` | `(&str, &str, &str, Color32)` | Add with custom icon color |
