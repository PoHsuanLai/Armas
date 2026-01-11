# Aurora

Dynamic aurora background with flowing gradients that create a mesmerizing northern lights effect.

## Basic Usage

```demo
let mut aurora = AuroraBackground::new(ui.available_width(), 300.0);
aurora.show(ui);
```

## Custom Height

```demo
let mut aurora = AuroraBackground::new(ui.available_width(), 400.0);
aurora.show(ui);
```

## Compact Aurora

```demo
let mut aurora = AuroraBackground::new(ui.available_width(), 200.0);
aurora.show(ui);
```

## Full Width Aurora

```demo
let mut aurora = AuroraBackground::new(ui.available_width(), 250.0);
aurora.show(ui);
```

## API Reference

| Constructor | Parameters | Description |
|-------------|-----------|-------------|
| `::new()` | `(width: f32, height: f32)` | Create aurora with dimensions |

| Method | Type | Description |
|--------|------|-------------|
| `.show()` | `&mut Ui` | Show aurora as background |

## Features

- **Animated**: Continuous flowing gradient animation
- **Colorful**: Multi-color gradient effects
- **Auto-play**: Starts animating on render
- **Performance**: Optimized for 60fps
- **Responsive**: Adapts to container size

## Animation Details

- **Duration**: Continuous loop
- **Easing**: Smooth sinusoidal
- **Colors**: Multiple gradient layers
- **Speed**: Configurable animation speed

## Use Cases

- Hero sections
- Page backgrounds
- Card backgrounds
- Landing pages
- Feature highlights

## Dependencies

- `egui = "0.33"`
- Requires continuous repainting
- Uses shader-based gradients where available
