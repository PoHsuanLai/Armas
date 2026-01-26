# Wobble Card

Card with wobble animation on hover.

```demo
let mut wobble_card = WobbleCard::new(300.0, 200.0);
wobble_card.show(ui, &theme, |ui| {
    ui.heading("Wobble Effect");
    ui.label("Wobbles on hover");
});
```
