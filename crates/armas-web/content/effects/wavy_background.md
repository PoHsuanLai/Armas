# Wavy Background

Animated wavy lines background that creates a dynamic, flowing pattern effect.

## Basic Usage

```demo
let mut wavy = WavyBackground::new(ui.available_width(), 200.0);
wavy.show(ui);
```

## Tall Waves

```demo
let mut wavy = WavyBackground::new(ui.available_width(), 300.0);
wavy.show(ui);
```

## Short Waves

```demo
let mut wavy = WavyBackground::new(ui.available_width(), 150.0);
wavy.show(ui);
```

## Full Width Waves

```demo
let mut wavy = WavyBackground::new(ui.available_width(), 250.0);
wavy.show(ui);
```

## API Reference

| Constructor | Parameters | Description |
|-------------|-----------|-------------|
| `::new()` | `(width: f32, height: f32)` | Create wavy background with dimensions |

| Method | Type | Description |
|--------|------|-------------|
| `.show()` | `&mut Ui` | Show wavy background |

## Features

- **Animated**: Continuous wave motion
- **Smooth**: Sinusoidal wave patterns
- **Auto-play**: Starts animating on render
- **Performance**: Optimized for smooth 60fps
- **Responsive**: Adapts to container width

## Animation Details

- **Pattern**: Multiple sine wave layers
- **Duration**: Continuous loop
- **Easing**: Smooth sinusoidal
- **Speed**: Configurable wave speed
- **Direction**: Horizontal flow

## Use Cases

- Section dividers
- Hero backgrounds
- Card backgrounds
- Loading screens
- Feature sections

## Dependencies

- `egui = "0.33"`
- Requires continuous repainting
- Uses mathematical wave functions
