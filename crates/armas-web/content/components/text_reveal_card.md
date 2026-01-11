# Text Reveal Card

Card with text reveal effect on hover.

## Basic Usage

```demo
let mut text_reveal = TextRevealCard::new(
    300.0,
    200.0,
    "Hover to reveal".to_string(),
    "Hidden message!".to_string()
);
text_reveal.show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `(&str, &str)` | - | Create with visible and hidden text |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `primary`
