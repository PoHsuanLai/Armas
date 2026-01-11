# Progress

Progress indicators for showing task completion and loading states.

## Linear Progress

```demo
LinearProgress::new(0.7).show(ui);
```

## Circular Progress Bar

```demo
CircularProgressBar::new(0.5).show(ui);
```

## Ring Progress

```demo
RingProgress::new(0.8).show(ui);
```

## API Reference

| Component | Method | Type | Description |
|-----------|--------|------|-------------|
| `LinearProgress` | `::new()` | `f32` | Linear progress bar (0.0-1.0) |
| `CircularProgressBar` | `::new()` | `f32` | Circular progress (0.0-1.0) |
| `RingProgress` | `::new()` | `f32` | Ring-shaped progress (0.0-1.0) |
| `.with_label()` | `&str` | `None` | Progress label |

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface`
