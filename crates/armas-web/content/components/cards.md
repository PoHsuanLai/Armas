# Cards

Interactive card components with hover effects and animations.

## Gradient Card

Card with animated gradient backgrounds.

```demo
let theme = ui.ctx().armas_theme();
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
let theme = ui.ctx().armas_theme();
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
let cards: Vec<FocusCard> = vec![
    FocusCard::new("Card 1", "First card"),
    FocusCard::new("Card 2", "Second card"),
    FocusCard::new("Card 3", "Third card"),
];
let mut focus_cards = FocusCards::new(cards).card_size(200.0, 150.0);
focus_cards.show(ui);
```

## Animation Details

- **Trigger**: Hover
- **Duration**: 200-300ms
- **Easing**: EaseOut
- **Performance**: 60fps, transform-based animations
