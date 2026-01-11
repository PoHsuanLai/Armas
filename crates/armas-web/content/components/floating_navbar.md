# Floating Navbar

Floating navigation bar with smooth morphing background that highlights the active item.

## Basic Usage

```demo
let items = vec![
    NavItem::new("Home").active(true),
    NavItem::new("About"),
    NavItem::new("Contact"),
];

let mut navbar = FloatingNavbar::new(items);
let response = navbar.show(ui.ctx());

if let Some(index) = response.clicked {
    println!("Clicked item {}", index);
}
```

## With Icons

```demo
let items = vec![
    NavItem::new("Home").icon("🏠").active(true),
    NavItem::new("Search").icon("🔍"),
    NavItem::new("Profile").icon("👤"),
    NavItem::new("Settings").icon("⚙️"),
];

let mut navbar = FloatingNavbar::new(items);
navbar.show(ui.ctx());
```

## Custom Position

```demo
let items = vec![
    NavItem::new("Top Nav").active(true),
    NavItem::new("Item 2"),
];

// Position at top (default)
let mut navbar = FloatingNavbar::new(items.clone())
    .position(NavbarPosition::Top);
navbar.show(ui.ctx());

// Position at bottom
let mut navbar = FloatingNavbar::new(items.clone())
    .position(NavbarPosition::Bottom);
navbar.show(ui.ctx());
```

## Custom Width

```demo
let items = vec![
    NavItem::new("Wide").active(true),
    NavItem::new("Navbar"),
];

let mut navbar = FloatingNavbar::new(items)
    .width(1000.0);

navbar.show(ui.ctx());
```

## Handle Interactions

```demo
let items = vec![
    NavItem::new("Dashboard"),
    NavItem::new("Analytics"),
    NavItem::new("Reports"),
];

let mut navbar = FloatingNavbar::new(items);
let response = navbar.show(ui.ctx());

// Handle clicks
if let Some(clicked_index) = response.clicked {
    match clicked_index {
        0 => { /* navigate to dashboard */ },
        1 => { /* navigate to analytics */ },
        2 => { /* navigate to reports */ },
        _ => {}
    }
}

// Handle hover
if let Some(hovered_index) = response.hovered {
    println!("Hovering item {}", hovered_index);
}
```

## API Reference

### FloatingNavbar

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(items)` | `Vec<NavItem>` | - | Create navbar with items |
| `.position()` | `NavbarPosition` | `Top` | Set navbar position |
| `.width()` | `f32` | `800.0` | Set fixed width |
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

## Features

- **Morphing Indicator**: Background pill smoothly animates to highlight active item
- **Floating Design**: Navbar floats above content with semi-transparent background
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
