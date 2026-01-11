# Floating Dock

macOS-style floating dock with icon magnification on hover.

## Basic Usage

```demo
let items = vec![
    DockItem::new("Home", "🏠"),
    DockItem::new("Files", "📁"),
    DockItem::new("Settings", "⚙️"),
];

let mut dock = FloatingDock::new(items, &theme);
let response = dock.show(ui);

if let Some(index) = response.clicked_item {
    println!("Clicked item {}", index);
}
```

## With Item IDs

```demo
let items = vec![
    DockItem::new("Home", "🏠").with_id("home"),
    DockItem::new("Mail", "📧").with_id("mail"),
    DockItem::new("Files", "📁").with_id("files"),
    DockItem::new("Music", "🎵").with_id("music"),
];

// Store items separately to access later
let items_clone = items.clone();
let mut dock = FloatingDock::new(items, &theme);
let response = dock.show(ui);

if let Some(index) = response.clicked_item {
    let item_id = &items_clone[index].id;
    match item_id.as_deref() {
        Some("home") => { /* go home */ },
        Some("mail") => { /* open mail */ },
        _ => {}
    }
}
```

## Custom Magnification

```demo
let items = vec![
    DockItem::new("App 1", "📱"),
    DockItem::new("App 2", "💻"),
    DockItem::new("App 3", "🖥️"),
];

let mut dock = FloatingDock::new(items, &theme)
    .with_magnification(2.0)  // 2x zoom on hover
    .with_item_size(64.0)
    .with_spacing(12.0);

dock.show(ui);
```

## Different Positions

```demo
let items = vec![
    DockItem::new("Left", "⬅️"),
    DockItem::new("Right", "➡️"),
];

// Bottom position (default)
let mut dock = FloatingDock::new(items.clone(), &theme)
    .with_position(DockPosition::Bottom);
dock.show(ui);

// Top position
let mut dock = FloatingDock::new(items.clone(), &theme)
    .with_position(DockPosition::Top);
dock.show(ui);

// Left position
let mut dock = FloatingDock::new(items.clone(), &theme)
    .with_position(DockPosition::Left);
dock.show(ui);

// Right position
let mut dock = FloatingDock::new(items.clone(), &theme)
    .with_position(DockPosition::Right);
dock.show(ui);
```

## Custom Background

```demo
use egui::Color32;

let items = vec![
    DockItem::new("Custom", "✨"),
];

let mut dock = FloatingDock::new(items, &theme)
    .with_background_color(Color32::from_rgba_unmultiplied(20, 20, 30, 220));

dock.show(ui);
```

## API Reference

### FloatingDock

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(items, theme)` | `(Vec<DockItem>, &Theme)` | - | Create dock with items |
| `.with_magnification()` | `f32` | `1.5` | Magnification factor on hover |
| `.with_item_size()` | `f32` | `48.0` | Base icon size in pixels |
| `.with_spacing()` | `f32` | `8.0` | Spacing between items |
| `.with_background_color()` | `Color32` | `theme.surface()` | Set background color |
| `.with_position()` | `DockPosition` | `Bottom` | Set dock position |
| `.show(&mut Ui)` | - | - | Show dock and handle interactions |

### DockItem

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(label, icon)` | `(&str, &str)` | - | Create dock item |
| `.with_id()` | `&str` | `None` | Set callback ID for handling clicks |

### DockPosition

| Variant | Description |
|---------|-------------|
| `Bottom` | Dock at bottom center of screen |
| `Top` | Dock at top center of screen |
| `Left` | Dock at left center of screen |
| `Right` | Dock at right center of screen |

### DockResponse

| Field | Type | Description |
|-------|------|-------------|
| `response` | `Response` | The underlying egui response |
| `clicked_item` | `Option<usize>` | Index of clicked item, if any |

## Features

- **Magnification Effect**: Icons smoothly scale up when the mouse is near them
- **Smooth Transitions**: Cosine interpolation for natural magnification curve
- **Hover Labels**: Item labels appear above icons on hover
- **Multiple Positions**: Position dock on any edge of the screen
- **Customizable**: Control size, spacing, magnification, and colors

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `surface_variant`, `hover`
