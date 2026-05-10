# Button Group

Groups buttons with connected borders.

```demo
ButtonGroup::new("bg_demo").show(ui, |ui| {
    Button::new("Bold").variant(ButtonVariant::Outline).show(ui);
    Button::new("Italic").variant(ButtonVariant::Outline).show(ui);
    Button::new("Underline").variant(ButtonVariant::Outline).show(ui);
});
```

## Vertical

Stack buttons vertically.

```demo
ButtonGroup::new("bg_vert")
    .orientation(ButtonGroupOrientation::Vertical)
    .show(ui, |ui| {
        Button::new("Top").variant(ButtonVariant::Outline).show(ui);
        Button::new("Middle").variant(ButtonVariant::Outline).show(ui);
        Button::new("Bottom").variant(ButtonVariant::Outline).show(ui);
    });
```
