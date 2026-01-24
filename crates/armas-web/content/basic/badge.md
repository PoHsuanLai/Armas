# Badge

Small status indicator for labels, counts, and categories.

## Basic Usage

```demo
Badge::new("New").show(ui);
ui.add_space(8.0);
Badge::new("5").show(ui);
ui.add_space(8.0);
Badge::new("Pro").show(ui);
```

## Variants

### Filled

```demo
Badge::new("Filled")
    .variant(BadgeVariant::Filled)
    .show(ui);
```

### Outlined

```demo
Badge::new("Outlined")
    .variant(BadgeVariant::Outlined)
    .show(ui);
```

### Soft (Default)

```demo
Badge::new("Soft")
    .variant(BadgeVariant::Soft)
    .show(ui);
```

## Colors

### Default (Primary)

```demo
Badge::new("Primary")
    .show(ui);
```

### Destructive

```demo
Badge::new("Error")
    .destructive()
    .show(ui);
```

### Custom Color

```demo
Badge::new("Custom")
    .color(Color32::from_rgb(100, 200, 150))
    .show(ui);
```

## With Dot Indicator

```demo
Badge::new("New")
    .dot()
    .show(ui);
ui.add_space(8.0);
Badge::new("5 Notifications")
    .dot()
    .destructive()
    .show(ui);
```

## Custom Size

```demo
Badge::new("Small")
    .size(10.0)
    .show(ui);
ui.add_space(8.0);
Badge::new("Medium")
    .size(13.0)
    .show(ui);
ui.add_space(8.0);
Badge::new("Large")
    .size(16.0)
    .show(ui);
```

## Removable

```demo
let response = Badge::new("Removable")
    .removable()
    .show(ui);

if response.removed {
    // Handle removal
}
```

## Combined Examples

### Status Badges

```demo
ui.horizontal(|ui| {
    Badge::new("Active")
        .variant(BadgeVariant::Filled)
        .color(theme.chart_2())
        .show(ui);
    ui.add_space(8.0);
    Badge::new("Error")
        .variant(BadgeVariant::Soft)
        .destructive()
        .show(ui);
    ui.add_space(8.0);
    Badge::new("Inactive")
        .variant(BadgeVariant::Outlined)
        .show(ui);
});
```

### Notification Counts

```demo
ui.horizontal(|ui| {
    ui.label("Messages");
    Badge::new("12")
        .variant(BadgeVariant::Filled)
        .destructive()
        .show(ui);
});
```

### Tags

```demo
ui.horizontal(|ui| {
    Badge::new("React").show(ui);
    ui.add_space(4.0);
    Badge::new("TypeScript").show(ui);
    ui.add_space(4.0);
    Badge::new("Rust").show(ui);
    ui.add_space(4.0);
    Badge::new("egui").show(ui);
});
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.variant()` | `BadgeVariant` | `Soft` | Visual style variant |
| `.color()` | `Color32` | `primary` | Custom color |
| `.destructive()` | - | - | Make destructive (red) |
| `.dot()` | - | `false` | Show dot indicator |
| `.size()` | `f32` | `13.0` | Font size |
| `.removable()` | - | `false` | Show remove button |

## Variants

- `BadgeVariant::Filled` - Solid background
- `BadgeVariant::Outlined` - Border only
- `BadgeVariant::Soft` - Subtle background (default)

## Response

| Field | Type | Description |
|-------|------|-------------|
| `removed` | `bool` | Whether remove button was clicked |
| `response` | `Response` | Underlying egui response |
