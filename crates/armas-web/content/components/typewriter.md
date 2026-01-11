# Typewriter

Text with typewriter animation effect.

## Basic Usage

```demo
Typewriter::new("Hello World").show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `&str` | - | Create typewriter text |
| `.speed()` | `f32` | `1.0` | Typing speed |
| `.cursor()` | `bool` | `true` | Show cursor |

## Dependencies

- `egui = "0.33"`
- Theme colors: `on_surface`
