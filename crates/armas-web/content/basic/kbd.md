# Kbd

Keyboard shortcut display element.

```demo
Kbd::new("K").show(ui, &theme);
```

## Key Combinations

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Kbd::new("Ctrl+K").show(ui, &theme);
    Kbd::new("Cmd+Shift+P").show(ui, &theme);
});
```

## Common Shortcuts

```demo
ui.vertical(|ui| {
    ui.horizontal(|ui| {
        ui.label("Copy:");
        Kbd::new("Ctrl+C").show(ui, &theme);
    });
    ui.horizontal(|ui| {
        ui.label("Paste:");
        Kbd::new("Ctrl+V").show(ui, &theme);
    });
    ui.horizontal(|ui| {
        ui.label("Save:");
        Kbd::new("Ctrl+S").show(ui, &theme);
    });
});
```
