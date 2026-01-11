# Feature Grid

Responsive grid layout for displaying features with icons, titles, and descriptions.

## Basic Usage

```demo
let items = vec![
    FeatureItem::new("⚡", "Fast", "Lightning speed performance"),
    FeatureItem::new("🔒", "Secure", "Enterprise-grade security"),
    FeatureItem::new("📈", "Scalable", "Grows with your needs"),
];

FeatureGrid::new(items).show(ui);
```

## Custom Columns

```demo
let items = vec![
    FeatureItem::new("✨", "Easy", "Simple to use"),
    FeatureItem::new("💪", "Powerful", "Advanced features"),
    FeatureItem::new("🚀", "Fast", "Optimized performance"),
];

FeatureGrid::new(items)
    .columns(2)
    .show(ui);
```

## Custom Icon Colors

```demo
use egui::Color32;

let items = vec![
    FeatureItem::new("💡", "Innovation", "Creative solutions")
        .icon_color(Color32::from_rgb(255, 200, 0)),

    FeatureItem::new("🎨", "Design", "Beautiful interfaces")
        .icon_color(Color32::from_rgb(200, 0, 255)),

    FeatureItem::new("⚙️", "Engineering", "Rock-solid code")
        .icon_color(Color32::from_rgb(0, 200, 255)),
];

FeatureGrid::new(items).show(ui);
```

## Custom Styling

```demo
let items = vec![
    FeatureItem::new("🔥", "Hot Features", "The latest and greatest"),
    FeatureItem::new("❄️", "Cool Features", "Stay chill with stability"),
];

FeatureGrid::new(items)
    .columns(2)
    .gap(30.0)
    .show_borders(false)
    .hover_effect(true)
    .icon_size(48.0)
    .show(ui);
```

## Auto-Responsive

```demo
// Grid automatically calculates columns based on available width
let items = vec![
    FeatureItem::new("📱", "Mobile", "Works on any device"),
    FeatureItem::new("💻", "Desktop", "Optimized for big screens"),
    FeatureItem::new("🌐", "Web", "Access anywhere"),
    FeatureItem::new("⚡", "Native", "Native performance"),
];

// Columns will be auto-calculated (1-4 based on width)
FeatureGrid::new(items).show(ui);
```

## API Reference

### FeatureGrid

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(items)` | `Vec<FeatureItem>` | - | Create grid with feature items |
| `.columns()` | `usize` | `auto` | Set number of columns (None = auto) |
| `.gap()` | `f32` | `20.0` | Gap between items |
| `.show_borders()` | `bool` | `true` | Show borders between items |
| `.hover_effect()` | `bool` | `true` | Enable hover effect |
| `.icon_size()` | `f32` | `32.0` | Icon size in pixels |
| `.show(&mut Ui)` | - | - | Show the grid |

### FeatureItem

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(icon, title, desc)` | `(&str, &str, &str)` | - | Create feature item |
| `.icon_color()` | `Color32` | `theme.primary()` | Set custom icon color |

## Design Features

- **Smart Borders**: Borders only appear between items, not on edges
- **Responsive Layout**: Auto-calculates columns based on available width
- **Hover Effects**: Optional background highlight on hover
- **Word Wrapping**: Descriptions automatically wrap to fit width

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `on_surface`, `on_surface_variant`, `outline_variant`, `hover`
