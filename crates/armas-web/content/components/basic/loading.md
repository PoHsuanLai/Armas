# Loading

Loading indicators and progress components.

## Spinner

Circular loading spinner.

```demo
Spinner::new().size(40.0).show(ui);
```

## Loading Dots

Animated dot sequence for loading states.

```demo
LoadingDots::new().show(ui);
```

## Skeleton

Placeholder loading animation for content.

```demo
Skeleton::new(300.0, 20.0).show(ui);
ui.add_space(8.0);
Skeleton::new(250.0, 20.0).show(ui);
ui.add_space(8.0);
Skeleton::new(200.0, 20.0).show(ui);
```

## Circular Progress

Progress indicator with percentage.

```demo
let mut progress = CircularProgress::new();
progress.show(ui);
```

## API Reference

### Spinner

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.size()` | `f32` | `32.0` | Spinner diameter |
| `.color()` | `Color32` | `primary` | Spinner color |

### Skeleton

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.width()` | `f32` | Required | Skeleton width |
| `.height()` | `f32` | Required | Skeleton height |
| `.rounding()` | `f32` | `4.0` | Corner rounding |

## Animation Details

- **Trigger**: Auto-play on render
- **Duration**: 1-2s continuous loop
- **Easing**: Linear for spinners, EaseInOut for pulses
- **Performance**: 60fps, optimized animations

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface_variant`
