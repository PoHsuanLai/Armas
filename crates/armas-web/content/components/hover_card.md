# Hover Card

Interactive card with 3D tilt effect on mouse hover.

## Basic Usage

```demo
let mut hover_card = HoverCard::new(300.0, 200.0, &theme);
hover_card.show(ui, &theme, |ui, _intensity| {
    ui.heading("Hover Me");
    ui.label("Card tilts on hover");
}, |ui, _intensity| {
    ui.heading("Hover Me");
    ui.label("Card tilts on hover");
});
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create hover card |
| `.intensity()` | `f32` | `1.0` | Tilt intensity |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`
