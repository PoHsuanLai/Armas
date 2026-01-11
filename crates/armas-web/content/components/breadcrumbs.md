# Breadcrumbs

Navigation path indicator showing the current location in a hierarchy.

## Basic Usage

```demo
let mut breadcrumbs = Breadcrumbs::new()
    .add_item(BreadcrumbItem::new("Home"))
    .add_item(BreadcrumbItem::new("Projects"))
    .add_item(BreadcrumbItem::new("Armas").current());

breadcrumbs.show(ui);
```

## With Icons

```demo
let mut breadcrumbs = Breadcrumbs::new()
    .add_item(BreadcrumbItem::new("Home").icon("🏠"))
    .add_item(BreadcrumbItem::new("Documents").icon("📁"))
    .add_item(BreadcrumbItem::new("Reports").icon("📊").current());

breadcrumbs.show(ui);
```

## Custom Separator

```demo
let mut breadcrumbs = Breadcrumbs::new()
    .separator("/")
    .add_item(BreadcrumbItem::new("Users"))
    .add_item(BreadcrumbItem::new("John"))
    .add_item(BreadcrumbItem::new("Projects").current());

breadcrumbs.show(ui);
```

## Handling Clicks

```demo
let mut breadcrumbs = Breadcrumbs::new()
    .add_item(BreadcrumbItem::new("Home"))
    .add_item(BreadcrumbItem::new("Settings"))
    .add_item(BreadcrumbItem::new("Profile").current());

let response = breadcrumbs.show(ui);
if let Some(index) = response.clicked {
    // Navigate to clicked breadcrumb
}
```

## API Reference

### Breadcrumbs

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.add_item()` | `BreadcrumbItem` | - | Add breadcrumb item |
| `.item()` | `&str` | - | Add simple item by label |
| `.separator()` | `&str` | `"›"` | Separator between items |
| `.spacing()` | `f32` | `4.0` | Spacing between items |

### BreadcrumbItem

| Method | Type | Description |
|--------|------|-------------|
| `::new()` | `&str` | Create new item |
| `.icon()` | `&str` | Set icon/emoji |
| `.current()` | - | Mark as current/active |

### Response

| Field | Type | Description |
|-------|------|-------------|
| `clicked` | `Option<usize>` | Index of clicked breadcrumb |

## Dependencies

- `egui = "0.33"`
- Button component
- Theme colors: `primary`, `on_surface`
