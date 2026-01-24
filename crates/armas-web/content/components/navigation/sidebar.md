# Sidebar

Collapsible sidebar navigation.

## Basic Usage

```demo
Sidebar::new()
    .show(ui, |sidebar| {
        sidebar.item("H", "Home");
        sidebar.item("P", "Profile");
        sidebar.item("S", "Settings");
    });
```

## With Active Item

```demo
Sidebar::new()
    .show(ui, |sidebar| {
        sidebar.item("D", "Dashboard").active(true);
        sidebar.item("A", "Analytics");
    });
```

## With Expandable Groups

```demo
Sidebar::new()
    .show(ui, |sidebar| {
        sidebar.item("H", "Home").active(true);
        sidebar.group("S", "Settings", |group| {
            group.item("P", "Profile");
            group.item("N", "Notifications");
            group.item("X", "Privacy");
        });
        sidebar.group("T", "Tools", |group| {
            group.item("A", "Analytics");
            group.item("L", "Logs");
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
