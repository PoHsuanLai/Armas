# Badge

Small status indicator for labels, counts, and categories.

```demo
Badge::new("New").show(ui);
```

## Variants

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Badge::new("Default").show(ui);
    Badge::new("Secondary").variant(BadgeVariant::Secondary).show(ui);
    Badge::new("Destructive").variant(BadgeVariant::Destructive).show(ui);
    Badge::new("Outline").variant(BadgeVariant::Outline).show(ui);
});
```

## Destructive

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Badge::new("Error").destructive().show(ui);
    Badge::new("Error").destructive().variant(BadgeVariant::Default).show(ui);
    Badge::new("Error").destructive().variant(BadgeVariant::Outline).show(ui);
});
```

## With Dot

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Badge::new("New").dot().show(ui);
    Badge::new("5 Notifications").dot().destructive().show(ui);
});
```

## Removable

```demo
let response = Badge::new("Removable").removable().show(ui);
if response.removed {
    // Handle removal
}
```

## Custom Color

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Badge::new("Custom").color(Color32::from_rgb(100, 200, 150)).show(ui);
    Badge::new("Active").variant(BadgeVariant::Default).color(theme.chart_2()).show(ui);
});
```
