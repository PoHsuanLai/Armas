# Card Stack

Stacked cards with 3D perspective and interactive animations.

## Basic Usage

```demo
let mut card_stack = CardStack::new(300.0, 200.0);
card_stack.show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `Vec<T>` | - | Create card stack |
| `.spacing()` | `f32` | `20.0` | Space between cards |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`
