# Kbd

Keyboard shortcut display element.

```demo
Kbd::new("K").show(ui);
```

## Key Combinations

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Kbd::new("Ctrl+K").show(ui);
    Kbd::new("Cmd+Shift+P").show(ui);
});
```

## Common Shortcuts

```demo
ui.vertical(|ui| {
    ui.horizontal(|ui| {
        ui.label("Copy:");
        Kbd::new("Ctrl+C").show(ui);
    });
    ui.horizontal(|ui| {
        ui.label("Paste:");
        Kbd::new("Ctrl+V").show(ui);
    });
    ui.horizontal(|ui| {
        ui.label("Save:");
        Kbd::new("Ctrl+S").show(ui);
    });
});
```
