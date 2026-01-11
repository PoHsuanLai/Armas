# Dot Pattern

Background dot grid pattern effect.

## Basic Usage

```demo
let mut dot_pattern = DotPattern::new(800.0, 600.0, &theme);
dot_pattern.show(ui);
```

## Custom Spacing

```demo
let mut dot_pattern = DotPattern::new(800.0, 600.0, &theme)
    .spacing(30.0)
    .color(theme.primary());
dot_pattern.show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create dot pattern |
| `.spacing()` | `f32` | `20.0` | Space between dots |
| `.size()` | `f32` | `2.0` | Dot size |
| `.color()` | `Color32` | `outline` | Dot color |
| `.opacity()` | `f32` | `0.3` | Dot opacity |

## Dependencies

- `egui = "0.33"`
- Theme colors: `outline`
