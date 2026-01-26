# Loading

Loading indicators and animated placeholders.

```demo
Spinner::new().size(40.0).show(ui);
```

## Loading Dots

```demo
LoadingDots::new().show(ui);
```

## Skeleton

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 8.0;
    Skeleton::new(300.0, 20.0).show(ui);
    Skeleton::new(250.0, 20.0).show(ui);
    Skeleton::new(200.0, 20.0).show(ui);
});
```
