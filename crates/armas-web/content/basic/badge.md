# Badge

Small status indicator for labels, counts, and categories.

```demo
Badge::new("New").show(ui, &theme);
```

## Variants

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Badge::new("Soft").show(ui, &theme);
    Badge::new("Filled").variant(BadgeVariant::Filled).show(ui, &theme);
    Badge::new("Outlined").variant(BadgeVariant::Outlined).show(ui, &theme);
});
```

## Destructive

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Badge::new("Error").destructive().show(ui, &theme);
    Badge::new("Error").destructive().variant(BadgeVariant::Filled).show(ui, &theme);
    Badge::new("Error").destructive().variant(BadgeVariant::Outlined).show(ui, &theme);
});
```

## With Dot

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Badge::new("New").dot().show(ui, &theme);
    Badge::new("5 Notifications").dot().destructive().show(ui, &theme);
});
```

## Removable

```demo
let response = Badge::new("Removable").removable().show(ui, &theme);
if response.removed {
    // Handle removal
}
```

## Custom Color

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Badge::new("Custom").color(Color32::from_rgb(100, 200, 150)).show(ui, &theme);
    Badge::new("Active").variant(BadgeVariant::Filled).color(theme.chart_2()).show(ui, &theme);
});
```
