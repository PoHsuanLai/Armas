# Tilt Card

Card with 3D tilt effect following mouse position.

## Basic Usage

```demo
let mut tilt_card = TiltCard::new(300.0, 200.0, &theme);
tilt_card.show(ui, &theme, |ui| {
    ui.heading("3D Tilt");
    ui.label("Follows your mouse");
});
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create tilt card |
| `.max_tilt()` | `f32` | `10.0` | Maximum tilt angle |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`
