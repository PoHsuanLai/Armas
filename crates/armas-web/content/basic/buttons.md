# Button

Displays a button or a component that looks like a button.

```demo
Button::new("Button").show(ui);
```

## Variants

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Button::new("Default").show(ui);
    Button::new("Secondary").variant(ButtonVariant::Secondary).show(ui);
    Button::new("Outline").variant(ButtonVariant::Outline).show(ui);
    Button::new("Ghost").variant(ButtonVariant::Ghost).show(ui);
    Button::new("Link").variant(ButtonVariant::Link).show(ui);
});
```

## Sizes

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Button::new("Small").size(ButtonSize::Small).show(ui);
    Button::new("Default").show(ui);
    Button::new("Large").size(ButtonSize::Large).show(ui);
});
```

## Disabled

```demo
Button::new("Disabled").enabled(false).show(ui);
```

## Full Width

```demo
Button::new("Full Width").full_width(true).show(ui);
```

## With Icon

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Button::new("← Back").variant(ButtonVariant::Outline).show(ui);
    Button::new("Next →").show(ui);
});
```
