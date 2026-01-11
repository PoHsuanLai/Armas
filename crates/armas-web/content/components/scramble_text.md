# Scramble Text

Text with scramble/decrypt animation effect.

## Basic Usage

```demo
ScrambleText::new("Secret Message").show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `&str` | - | Create scramble text |
| `.speed()` | `f32` | `1.0` | Animation speed |

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`
