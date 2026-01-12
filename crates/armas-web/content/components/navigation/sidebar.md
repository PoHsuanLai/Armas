# Sidebar

Collapsible sidebar navigation with smooth animations and customizable width.

## Basic Usage

```demo
Sidebar::new()
    .show(ui, |sidebar| {
        sidebar.item("🏠", "Home");
        sidebar.item("👤", "Profile");
        sidebar.item("⚙️", "Settings");
    });
```

## With Active Item

```demo
Sidebar::new()
    .show(ui, |sidebar| {
        sidebar.item("📊", "Dashboard").active(true);
        sidebar.item("📈", "Analytics");
    });
```

## With Expandable Sub-Items

```demo
Sidebar::new()
    .show(ui, |sidebar| {
        sidebar.item("🏠", "Home").active(true);
        sidebar.group("⚙️", "Settings", |group| {
            group.item("👤", "Profile");
            group.item("🔔", "Notifications");
            group.item("🔒", "Privacy");
        });
        sidebar.group("🔧", "Tools", |group| {
            group.item("📊", "Analytics");
            group.item("📝", "Logs");
        });
    });
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create sidebar |
| `.width()` | `f32` | `250.0` | Sidebar width |
| `.collapsible()` | `bool` | `false` | Enable collapsing |
| `.show()` | closure | - | Render with closure-based API |

### SidebarBuilder (in closure)

| Method | Type | Description |
|--------|------|-------------|
| `.item()` | `(&str, &str)` | Add item with icon and label |
| `.group()` | `(&str, &str, closure)` | Add expandable group |

### ItemBuilder (chainable from .item())

| Method | Type | Description |
|--------|------|-------------|
| `.active()` | `bool` | Mark as active |

### GroupBuilder (in group closure)

| Method | Type | Description |
|--------|------|-------------|
| `.item()` | `(&str, &str)` | Add child item with icon and label |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `primary`, `on_surface`
