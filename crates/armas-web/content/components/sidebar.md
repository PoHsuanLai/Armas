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

## With Expandable Sub-Items

```demo
use egui::Id;

// Load expanded states from memory
let settings_id = Id::new("sidebar_settings_expanded");
let settings_expanded = ui.data_mut(|d| d.get_temp::<bool>(settings_id).unwrap_or(true));

let tools_id = Id::new("sidebar_tools_expanded");
let tools_expanded = ui.data_mut(|d| d.get_temp::<bool>(tools_id).unwrap_or(false));

let items = vec![
    SidebarItem::new("🏠", "Home").active(true),
    SidebarItem::new("⚙️", "Settings")
        .expanded(settings_expanded)
        .with_children(vec![
            SidebarItem::new("👤", "Profile"),
            SidebarItem::new("🔔", "Notifications"),
            SidebarItem::new("🔒", "Privacy"),
        ]),
    SidebarItem::new("🔧", "Tools")
        .expanded(tools_expanded)
        .with_children(vec![
            SidebarItem::new("📊", "Analytics"),
            SidebarItem::new("📝", "Logs"),
        ]),
];

let mut sidebar = Sidebar::new(items);
let response = sidebar.show(ui);

// Save expanded states back to memory
if let Some(parent_idx) = response.clicked {
    match parent_idx {
        1 => { // Settings clicked
            let new_state = !settings_expanded;
            ui.data_mut(|d| d.insert_temp(settings_id, new_state));
        }
        2 => { // Tools clicked
            let new_state = !tools_expanded;
            ui.data_mut(|d| d.insert_temp(tools_id, new_state));
        }
        _ => {}
    }
}
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
