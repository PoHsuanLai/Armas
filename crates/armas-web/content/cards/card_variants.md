# Cards

Interactive card components with hover effects and animations.

## Gradient Card

Card with animated gradient backgrounds.

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

Card with 3D wobble effect on hover.

```demo
let mut wobble = WobbleCard::new(200.0, 120.0);
wobble.show(ui, &theme, |ui| {
    ui.vertical_centered(|ui| {
        ui.add_space(40.0);
        ui.label("Wobble");
    });
});
```

## Focus Cards

Cards that blur others on hover focus.

```demo
FocusCards::new()
    .card_size(200.0, 150.0)
    .show(ui, &theme, |cards| {
        cards.card(|ui, theme, _opacity| {
            Card::new().show(ui, theme, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Card 1");
                });
            });
        });

        cards.card(|ui, theme, _opacity| {
            Card::new().show(ui, theme, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Card 2");
                });
            });
        });

        cards.card(|ui, theme, _opacity| {
            Card::new().show(ui, theme, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Card 3");
                });
            });
        });
    });
```

## Animation Details

- **Trigger**: Hover
- **Duration**: 200-300ms
- **Easing**: EaseOut
- **Performance**: 60fps, transform-based animations
