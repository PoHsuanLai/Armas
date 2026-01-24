# Floating Navbar

Floating navigation bar with morphing background highlight.

## Basic Usage

```demo
let response = FloatingNavbar::new()
    .id("navbar_basic")
    .position(NavbarPosition::Bottom)
    .show(ui.ctx(), |navbar| {
        navbar.item("Home", None).active(true);
        navbar.item("About", None);
        navbar.item("Contact", None);
    });

if let Some(index) = response.clicked {
    // Handle navigation
}
```

## With Backdrop

```demo
use egui::Id;

let id = Id::new("navbar_visible");
let mut visible = ui.data_mut(|d| d.get_temp::<bool>(id).unwrap_or(false));

if Button::new("Toggle Navbar")
    .variant(ButtonVariant::Filled)
    .show(ui)
    .clicked()
{
    visible = !visible;
    ui.data_mut(|d| d.insert_temp(id, visible));
}

if visible {
    let response = FloatingNavbar::new()
        .id("navbar_demo")
        .backdrop(true)
        .show(ui.ctx(), |navbar| {
            navbar.item("Home", None).active(true);
            navbar.item("Search", None);
            navbar.item("Profile", None);
            navbar.item("Settings", None);
        });

    if response.clicked.is_some() || response.close_clicked || response.backdrop_clicked {
        visible = false;
        ui.data_mut(|d| d.insert_temp(id, visible));
    }
}
```

## API Reference

### FloatingNavbar

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create navbar |
| `.id()` | `impl Hash` | - | Set unique ID |
| `.position()` | `NavbarPosition` | `Top` | Navbar position |
| `.width()` | `f32` | `800.0` | Fixed width |
| `.backdrop()` | `bool` | `false` | Show background overlay |
| `.show()` | closure | - | Render with closure-based API |

### ItemBuilder (chainable from .item())

| Method | Type | Description |
|--------|------|-------------|
| `.active()` | `bool` | Mark item as active |

### NavbarPosition

| Variant | Description |
|---------|-------------|
| `Top` | Position at top |
| `Bottom` | Position at bottom |

### NavbarResponse

| Field | Type | Description |
|-------|------|-------------|
| `clicked` | `Option<usize>` | Index of clicked item |
| `hovered` | `Option<usize>` | Index of hovered item |
| `close_clicked` | `bool` | Close button clicked |
| `backdrop_clicked` | `bool` | Backdrop clicked |
