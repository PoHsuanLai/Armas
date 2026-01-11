# Sidebar

Collapsible sidebar navigation with smooth animations and customizable width.

## Basic Usage

```demo
let items = vec![
    SidebarItem::new("🏠", "Home"),
    SidebarItem::new("👤", "Profile"),
    SidebarItem::new("⚙️", "Settings"),
];
let mut sidebar = Sidebar::new(items);
sidebar.show(ui);
```

## With Active Item

```demo
let items = vec![
    SidebarItem::new("📊", "Dashboard").active(true),
    SidebarItem::new("📈", "Analytics"),
];
let mut sidebar = Sidebar::new(items);
sidebar.show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create sidebar |
| `.add_item()` | `SidebarItem` | - | Add sidebar item |
| `.width()` | `f32` | `250.0` | Sidebar width |
| `.collapsible()` | `bool` | `false` | Enable collapsing |

### SidebarItem

| Method | Type | Description |
|--------|------|-------------|
| `::new()` | `&str` | Create item with label |
| `.icon()` | `&str` | Set icon/emoji |
| `.active()` | - | Mark as active |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `primary`, `on_surface`
