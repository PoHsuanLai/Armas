# Tooltip

Contextual information that appears on hover.

```demo
let response = Button::new("Hover me").show(ui, &theme);
Tooltip::new("This is a tooltip").show(ui, &response);
```

## Positions

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    let top = Button::new("Top").show(ui, &theme);
    Tooltip::new("Appears above").position(TooltipPosition::Top).show(ui, &top);
    let bottom = Button::new("Bottom").show(ui, &theme);
    Tooltip::new("Appears below").position(TooltipPosition::Bottom).show(ui, &bottom);
    let left = Button::new("Left").show(ui, &theme);
    Tooltip::new("Appears left").position(TooltipPosition::Left).show(ui, &left);
    let right = Button::new("Right").show(ui, &theme);
    Tooltip::new("Appears right").position(TooltipPosition::Right).show(ui, &right);
});
```

## With Delay

```demo
let response = Button::new("Delayed").show(ui, &theme);
Tooltip::new("Shows after 500ms").delay(500).show(ui, &response);
```
