# Button

Displays a button or a component that looks like a button.

```demo
Button::new("Button").show(ui, &theme);
```

## Variants

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Button::new("Default").show(ui, &theme);
    Button::new("Secondary").variant(ButtonVariant::Secondary).show(ui, &theme);
    Button::new("Outline").variant(ButtonVariant::Outline).show(ui, &theme);
    Button::new("Ghost").variant(ButtonVariant::Ghost).show(ui, &theme);
    Button::new("Link").variant(ButtonVariant::Link).show(ui, &theme);
});
```

## Sizes

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Button::new("Small").size(ButtonSize::Small).show(ui, &theme);
    Button::new("Default").show(ui, &theme);
    Button::new("Large").size(ButtonSize::Large).show(ui, &theme);
});
```

## Disabled

```demo
Button::new("Disabled").enabled(false).show(ui, &theme);
```

## Full Width

```demo
Button::new("Full Width").full_width(true).show(ui, &theme);
```

## With Icon

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Button::new("← Back").variant(ButtonVariant::Outline).show(ui, &theme);
    Button::new("Next →").show(ui, &theme);
});
```
