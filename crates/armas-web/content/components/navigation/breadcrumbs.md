# Breadcrumbs

Navigation path indicator showing the current location in a hierarchy.

## Basic Usage

```demo
let response = Breadcrumbs::new()
    .show(ui, |breadcrumbs| {
        breadcrumbs.item("Home", None);
        breadcrumbs.item("Projects", None);
        breadcrumbs.item("Armas", None).current();
    });
```

## With Icons

```demo
Breadcrumbs::new()
    .show(ui, |breadcrumbs| {
        breadcrumbs.item("Home", Some("🏠"));
        breadcrumbs.item("Documents", Some("📁"));
        breadcrumbs.item("Reports", Some("📊")).current();
    });
```

## Custom Separator

```demo
Breadcrumbs::new()
    .separator("/")
    .show(ui, |breadcrumbs| {
        breadcrumbs.item("Users", None);
        breadcrumbs.item("John", None);
        breadcrumbs.item("Projects", None).current();
    });
```

## Handling Clicks

```demo
let response = Breadcrumbs::new()
    .show(ui, |breadcrumbs| {
        breadcrumbs.item("Home", None);
        breadcrumbs.item("Settings", None);
        breadcrumbs.item("Profile", None).current();
    });

if let Some(index) = response.clicked {
    // Navigate to clicked breadcrumb
}
```

## API Reference

### Breadcrumbs

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create new breadcrumbs |
| `.separator()` | `&str` | `"›"` | Separator between items |
| `.spacing()` | `f32` | `4.0` | Spacing between items |
| `.show_home_icon()` | `bool` | `false` | Show home icon before first item |
| `.show()` | closure | - | Render with closure-based API |

### BreadcrumbsBuilder (in closure)

| Method | Type | Description |
|--------|------|-------------|
| `.item()` | `(&str, Option<&str>)` | Add item with label and optional icon |

### ItemBuilder (chainable from .item())

| Method | Type | Description |
|--------|------|-------------|
| `.current()` | - | Mark as current/active item |

### Response

| Field | Type | Description |
|-------|------|-------------|
| `clicked` | `Option<usize>` | Index of clicked breadcrumb |

## Dependencies

- `egui = "0.33"`
- Button component
- Theme colors: `primary`, `on_surface`
