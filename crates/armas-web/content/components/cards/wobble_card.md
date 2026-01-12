# Wobble Card

Card with wobble animation on hover.

## Basic Usage

```demo
let mut wobble_card = WobbleCard::new(300.0, 200.0);
wobble_card.show(ui, &theme, |ui| {
    ui.heading("Wobble Effect");
    ui.label("Wobbles on hover");
});
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create wobble card |
| `.intensity()` | `f32` | `1.0` | Wobble intensity |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`
