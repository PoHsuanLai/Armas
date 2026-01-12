# Tilt Card

Card with 3D tilt effect following mouse position.

## Basic Usage

```demo
// Get or create tilt card from memory
let card_id = ui.id().with("basic_tilt");
let mut tilt_card = ui.data_mut(|d| {
    d.get_temp::<TiltCard>(card_id).unwrap_or_else(|| {
        TiltCard::new(300.0, 200.0, &theme)
    })
});

tilt_card.show(ui, &theme, |ui| {
    ui.heading("3D Tilt");
    ui.label("Follows your mouse");
});

// Store back to memory
ui.data_mut(|d| d.insert_temp(card_id, tilt_card));
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create tilt card |
| `.max_tilt()` | `f32` | `10.0` | Maximum tilt angle |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`
