# Layout

## VStack

```demo
VStack::new(16.0).show(ui, |ui| {
    ui.label("Item 1");
    ui.label("Item 2");
    ui.label("Item 3");
});
```

## HStack

```demo
HStack::new(12.0).show(ui, |ui| {
    Button::new("Save").variant(ButtonVariant::Filled).show(ui);
    Button::new("Cancel").variant(ButtonVariant::Outlined).show(ui);
});
```

## Grid

```demo
Grid::new()
    .columns(3)
    .gap(16.0)
    .show(ui, |ui| {
        for i in 1..=6 {
            ui.label(format!("Item {}", i));
        }
    });
```

## Container

```demo
Container::new(ContainerSize::Medium)
    .padding(24.0)
    .show(ui, |ui| {
        ui.label("Constrained content");
    });
```

## Spacer

```demo
ui.label("Before spacer");
Spacer::fixed(24.0).show(ui);
ui.label("After spacer");
```

## Divider

```demo
ui.label("Content above");
Divider::horizontal().show(ui, &theme);
ui.label("Content below");
```
