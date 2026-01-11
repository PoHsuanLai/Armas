# Cards

## Gradient Card

```demo
GradientCard::rainbow()
    .width(200.0)
    .height(120.0)
    .show(ui, &theme, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label("Rainbow");
        });
    });
```

## Wobble Card

```demo
let mut wobble = WobbleCard::new(200.0, 120.0);
wobble.show(ui, &theme, |ui| {
    ui.vertical_centered(|ui| {
        ui.add_space(40.0);
        ui.label("Wobble");
    });
});
```

## Focus Card

```demo
FocusCard::new("Card Title", "Card description").show(ui, &theme);
```
