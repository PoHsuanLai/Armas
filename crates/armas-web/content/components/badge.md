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

### Primary

```demo
Badge::new("Primary")
    .color(BadgeColor::Primary)
    .show(ui);
```

### Success

```demo
Badge::new("Success")
    .color(BadgeColor::Success)
    .show(ui);
```

### Warning

```demo
Badge::new("Warning")
    .color(BadgeColor::Warning)
    .show(ui);
```

### Error

```demo
Badge::new("Error")
    .color(BadgeColor::Error)
    .show(ui);
```

### Info

```demo
Badge::new("Info")
    .color(BadgeColor::Info)
    .show(ui);
```

### Neutral

```demo
Badge::new("Neutral")
    .color(BadgeColor::Neutral)
    .show(ui);
```

## With Dot Indicator

```demo
Badge::new("New")
    .with_dot()
    .show(ui);
ui.add_space(8.0);
Badge::new("5 Notifications")
    .with_dot()
    .color(BadgeColor::Error)
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
        .color(BadgeColor::Success)
        .show(ui);
    ui.add_space(8.0);
    Badge::new("Pending")
        .variant(BadgeVariant::Soft)
        .color(BadgeColor::Warning)
        .show(ui);
    ui.add_space(8.0);
    Badge::new("Inactive")
        .variant(BadgeVariant::Outlined)
        .color(BadgeColor::Neutral)
        .show(ui);
});
```

### Notification Counts

```demo
ui.horizontal(|ui| {
    ui.label("Messages");
    Badge::new("12")
        .variant(BadgeVariant::Filled)
        .color(BadgeColor::Error)
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
| `.color()` | `BadgeColor` | `Primary` | Color theme |
| `.with_dot()` | - | `false` | Show dot indicator |
| `.size()` | `f32` | `13.0` | Font size |
| `.removable()` | - | `false` | Show remove button |

## Variants

- `BadgeVariant::Filled` - Solid background
- `BadgeVariant::Outlined` - Border only
- `BadgeVariant::Soft` - Subtle background (default)

## Colors

- `BadgeColor::Primary` - Primary theme color
- `BadgeColor::Success` - Green (positive)
- `BadgeColor::Warning` - Yellow (caution)
- `BadgeColor::Error` - Red (danger)
- `BadgeColor::Info` - Blue (informational)
- `BadgeColor::Neutral` - Gray (default)

## Response

| Field | Type | Description |
|-------|------|-------------|
| `removed` | `bool` | Whether remove button was clicked |
| `response` | `Response` | Underlying egui response |

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `success`, `warning`, `error`, `info`, `outline`
