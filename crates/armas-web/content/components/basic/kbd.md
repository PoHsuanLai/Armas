# Kbd

Keyboard shortcut display element. Styled to match shadcn/ui kbd.

## Basic Usage

```demo
Kbd::new("K").show(ui);
```

## Key Combinations

Key combinations are automatically split on `+`:

```demo
ui.horizontal(|ui| {
    Kbd::new("Ctrl+K").show(ui);
});
```

```demo
ui.horizontal(|ui| {
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

## In Menu Items

```demo
ui.horizontal(|ui| {
    ui.label("Open File");
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        Kbd::new("Ctrl+O").show(ui);
    });
});
```

## API Reference

| Method | Type | Description |
|--------|------|-------------|
| `::new()` | `impl Into<String>` | Create with key text |
| `.show()` | `&mut Ui` | Display the kbd element |

## Styling

The component uses shadcn/ui styling:
- Background: `muted`
- Text: `muted-foreground`
- Font size: 11px
- Height: 20px
- Min width: 20px
- Border radius: 4px
