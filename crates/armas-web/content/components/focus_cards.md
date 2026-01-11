# Focus Cards

Grid of cards that highlight on hover with blur effect.

## Basic Usage

```demo
let cards = vec![
    FocusCard::new("Feature 1", "Description 1"),
    FocusCard::new("Feature 2", "Description 2"),
    FocusCard::new("Feature 3", "Description 3"),
];
let mut focus_cards = FocusCards::new(cards);
let _response = focus_cards.show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `Vec<(T, T)>` | - | Create focus cards |
| `.columns()` | `usize` | `3` | Number of columns |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `primary`
