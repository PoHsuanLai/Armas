# Aurora

Dynamic aurora background with flowing gradients that create a mesmerizing northern lights effect.

## Cyberpunk Theme (Slow)

```demo
AuroraBackground::cyberpunk(ui.available_width(), 300.0)
    .id("aurora_slow")
    .speed(0.2)
    .show(ui);
```

## Borealis Theme (Medium)

```demo
AuroraBackground::borealis(ui.available_width(), 300.0)
    .id("aurora_medium")
    .speed(0.5)
    .time_offset(10.0)
    .show(ui);
```

## Sunset Theme (Normal)

```demo
AuroraBackground::sunset(ui.available_width(), 300.0)
    .id("aurora_normal")
    .speed(1.0)
    .time_offset(20.0)
    .show(ui);
```

## Fast Animation

```demo
AuroraBackground::cyberpunk(ui.available_width(), 250.0)
    .id("aurora_fast")
    .speed(3.0)
    .time_offset(30.0)
    .show(ui);
```

## API Reference

| Constructor | Parameters | Description |
|-------------|-----------|-------------|
| `::new()` | `(width: f32, height: f32)` | Create empty aurora (no blobs) |
| `::cyberpunk()` | `(width: f32, height: f32)` | Create aurora with cyberpunk colors (cyan, magenta, blue) |
| `::borealis()` | `(width: f32, height: f32)` | Create aurora with borealis colors (green, teal, purple) |
| `::sunset()` | `(width: f32, height: f32)` | Create aurora with sunset colors (orange, pink, yellow) |

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.speed()` | `f32` | `1.0` | Set animation speed multiplier |
| `.add_blob()` | `(pos, radius, colors, speed)` | - | Add custom colored blob |
| `.show()` | `&mut Ui` | - | Render the aurora background |

## Themes

Aurora comes with three built-in color themes:

- **Cyberpunk**: Cyan, magenta, and blue blobs with a futuristic neon feel
- **Borealis**: Green, teal, and purple like the northern lights
- **Sunset**: Orange, pink, and yellow for warm, vibrant backgrounds

## Features

- **Animated blobs**: Floating gradient spheres with organic motion
- **Multiple layers**: Up to 3 blobs per theme for depth
- **Smooth movement**: Sinusoidal motion patterns
- **Configurable speed**: Adjust animation speed multiplier
- **Custom blobs**: Add your own colored blobs
- **Performance**: Optimized 60fps animation

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
