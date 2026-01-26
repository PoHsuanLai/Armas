# Separator

Divider line for separating content sections.

```demo
ui.label("Above");
Separator::new().show(ui, &theme);
ui.label("Below");
```

## Vertical

```demo
ui.horizontal(|ui| {
    ui.label("Left");
    Separator::new().vertical().length(20.0).show(ui, &theme);
    ui.label("Right");
});
```

## In Lists

```demo
ui.vertical(|ui| {
    ui.label("Item 1");
    Separator::new().show(ui, &theme);
    ui.label("Item 2");
    Separator::new().show(ui, &theme);
    ui.label("Item 3");
});
```
