# Floating Navbar

Floating navigation bar with smooth morphing background that highlights the active item.

## Interactive Demo with Backdrop

```demo
use egui::Id;

// Store navbar visibility state
let id = Id::new("navbar_visible");
let mut visible = ui.data_mut(|d| d.get_temp::<bool>(id).unwrap_or(false));

// Button to toggle navbar
if Button::new("Toggle Navbar")
    .variant(ButtonVariant::Filled)
    .show(ui)
    .clicked()
{
    visible = !visible;
    ui.data_mut(|d| d.insert_temp(id, visible));
}

// Show navbar when visible
if visible {
    let items = vec![
        NavItem::new("Home").icon("🏠").active(true),
        NavItem::new("Search").icon("🔍"),
        NavItem::new("Profile").icon("👤"),
        NavItem::new("Settings").icon("⚙️"),
    ];

    let mut navbar = FloatingNavbar::new(items)
        .with_id("navbar_demo")
        .with_backdrop(true); // Dark background overlay

    let response = navbar.show(ui.ctx());

    // Close navbar when:
    // - Item is clicked
    // - Close button (X) is clicked
    // - Backdrop is clicked
    if response.clicked.is_some() || response.close_clicked || response.backdrop_clicked {
        visible = false;
        ui.data_mut(|d| d.insert_temp(id, visible));
    }
}
```

## Basic Usage

```demo
let items = vec![
    NavItem::new("Home").active(true),
    NavItem::new("About"),
    NavItem::new("Contact"),
];

let mut navbar = FloatingNavbar::new(items)
    .with_id("navbar_basic")
    .position(NavbarPosition::Bottom);

let response = navbar.show(ui.ctx());

if let Some(index) = response.clicked {
    // Handle navigation
}
```


## API Reference

### FloatingNavbar

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(items)` | `Vec<NavItem>` | - | Create navbar with items |
| `.with_id()` | `impl Hash` | - | Set unique ID (required for multiple instances) |
| `.position()` | `NavbarPosition` | `Top` | Set navbar position |
| `.width()` | `f32` | `800.0` | Set fixed width |
| `.with_backdrop()` | `bool` | `false` | Show darkened background overlay |
| `.show(&egui::Context)` | - | - | Show navbar and handle interactions |

### NavItem

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(label)` | `&str` | - | Create nav item |
| `.icon()` | `&str` | `None` | Set icon (emoji or text) |
| `.active()` | `bool` | `false` | Set active state |

### NavbarPosition

| Variant | Description |
|---------|-------------|
| `Top` | Position at top of screen |
| `Bottom` | Position at bottom of screen |

### NavbarResponse

| Field | Type | Description |
|-------|------|-------------|
| `response` | `Response` | The underlying egui response |
| `clicked` | `Option<usize>` | Index of clicked item, if any |
| `hovered` | `Option<usize>` | Index of hovered item, if any |
| `close_clicked` | `bool` | Whether the close button (X) was clicked |
| `backdrop_clicked` | `bool` | Whether the backdrop overlay was clicked |

## Features

- **Morphing Indicator**: Background pill smoothly animates to highlight active item
- **Floating Design**: Navbar floats above content with semi-transparent background
- **Close Button**: X button in top-right corner to dismiss navbar
- **Backdrop Dismissal**: Click backdrop overlay to close navbar
- **Auto-Sizing**: Items automatically size to fill available width
- **Icons & Labels**: Support for both icons and text labels
- **Smooth Animations**: Easing functions for natural motion

## Animation Details

The navbar uses the animation system for smooth transitions:
- **Position Animation**: Active indicator smoothly moves between items
- **Width Animation**: Indicator resizes to match item width
- **Easing**: EaseOut curve for natural deceleration
- **Duration**: 0.3s transition time

## Dependencies

- `egui = "0.33"`
- Animation system: `Animation`, `EasingFunction`
- Theme colors: `primary` (active indicator)
