# Feature Grid

Responsive grid layout for displaying features with icons, titles, and descriptions.

## Basic Usage

```demo
FeatureGrid::new().show(ui, |grid| {
    grid.feature("⚡", "Fast", "Lightning speed performance");
    grid.feature("🔒", "Secure", "Enterprise-grade security");
    grid.feature("📈", "Scalable", "Grows with your needs");
});
```

## Custom Columns

```demo
FeatureGrid::new()
    .columns(2)
    .show(ui, |grid| {
        grid.feature("✨", "Easy", "Simple to use");
        grid.feature("💪", "Powerful", "Advanced features");
        grid.feature("🚀", "Fast", "Optimized performance");
    });
```

## Custom Icon Colors

```demo
use egui::Color32;

FeatureGrid::new().show(ui, |grid| {
    grid.feature_with_color(
        "💡",
        "Innovation",
        "Creative solutions",
        Color32::from_rgb(255, 200, 0)
    );
    grid.feature_with_color(
        "🎨",
        "Design",
        "Beautiful interfaces",
        Color32::from_rgb(200, 0, 255)
    );
    grid.feature_with_color(
        "⚙️",
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
        grid.feature("🔥", "Hot Features", "The latest and greatest");
        grid.feature("❄️", "Cool Features", "Stay chill with stability");
    });
```

## Auto-Responsive

```demo
// Grid automatically calculates columns based on available width
// Columns will be auto-calculated (1-4 based on width)
FeatureGrid::new().show(ui, |grid| {
    grid.feature("📱", "Mobile", "Works on any device");
    grid.feature("💻", "Desktop", "Optimized for big screens");
    grid.feature("🌐", "Web", "Access anywhere");
    grid.feature("⚡", "Native", "Native performance");
});
```

## API Reference

### FeatureGrid

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create new feature grid |
| `.columns()` | `usize` | `auto` | Set number of columns (None = auto) |
| `.gap()` | `f32` | `20.0` | Gap between items |
| `.show_borders()` | `bool` | `true` | Show borders between items |
| `.hover_effect()` | `bool` | `true` | Enable hover effect |
| `.icon_size()` | `f32` | `32.0` | Icon size in pixels |
| `.show(&mut Ui, closure)` | - | - | Show the grid with builder closure |

### GridBuilder (inside closure)

| Method | Type | Description |
|--------|------|-------------|
| `.feature(icon, title, desc)` | `(&str, &str, &str)` | Add feature item |
| `.feature_with_color(icon, title, desc, color)` | `(&str, &str, &str, Color32)` | Add feature with custom icon color |

## Design Features

- **Smart Borders**: Borders only appear between items, not on edges
- **Responsive Layout**: Auto-calculates columns based on available width
- **Hover Effects**: Optional background highlight on hover
- **Word Wrapping**: Descriptions automatically wrap to fit width

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `on_surface`, `on_surface_variant`, `outline_variant`, `hover`
