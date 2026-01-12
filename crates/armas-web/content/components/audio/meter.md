# Audio Meter

Professional DAW-style audio level meter with smooth animations, peak hold, and customizable gradients.

## Basic Usage

```demo
use egui::Id;
use std::f32::consts::PI;

let id = Id::new("meter_basic");
let time = ui.input(|i| i.time) as f32;
let level = ((time * 2.0).sin() * 0.5 + 0.5) * 0.8; // Simulated audio

AudioMeter::new(level)
    .height(200.0)
    .show(ui);
```

## Traditional VU Colors

```demo
use egui::Id;
use std::f32::consts::PI;

let theme = ui.ctx().armas_theme();
let time = ui.input(|i| i.time) as f32;
let level = ((time * 1.5).sin() * 0.5 + 0.5) * 0.9;

AudioMeter::new(level)
    .height(200.0)
    .vu_colors(&theme)
    .show(ui);
```

## Segmented LED Style

```demo
use std::f32::consts::PI;

let theme = ui.ctx().armas_theme();
let time = ui.input(|i| i.time) as f32;
let level = ((time * 2.5).sin() * 0.5 + 0.5) * 0.85;

AudioMeter::new(level)
    .height(200.0)
    .style(MeterStyle::Segmented(20))
    .vu_colors(&theme)
    .show(ui);
```

## With Scale (Right Side)

```demo
use std::f32::consts::PI;

let time = ui.input(|i| i.time) as f32;
let level = ((time * 1.8).sin() * 0.5 + 0.5) * 0.75;

AudioMeter::new(level)
    .height(200.0)
    .width(35.0)
    .show_scale()
    .show(ui);
```

## With Scale (Left Side)

```demo
use std::f32::consts::PI;

let time = ui.input(|i| i.time) as f32;
let level = ((time * 2.1).sin() * 0.5 + 0.5) * 0.8;

AudioMeter::new(level)
    .height(200.0)
    .width(35.0)
    .scale_left()
    .show(ui);
```

## Monochrome Style

```demo
use std::f32::consts::PI;

let theme = ui.ctx().armas_theme();
let time = ui.input(|i| i.time) as f32;
let level = ((time * 2.2).sin() * 0.5 + 0.5) * 0.7;

AudioMeter::new(level)
    .height(200.0)
    .monochrome(theme.primary())
    .show(ui);
```

## Custom Colors

```demo
use std::f32::consts::PI;
use egui::Color32;

let time = ui.input(|i| i.time) as f32;
let level = ((time * 3.0).sin() * 0.5 + 0.5) * 0.9;

AudioMeter::new(level)
    .height(200.0)
    .color_range(
        Color32::from_rgb(0, 100, 255),
        Color32::from_rgb(255, 0, 255)
    )
    .show(ui);
```

## Mixer Channel Strip

```demo
use std::f32::consts::PI;

let theme = ui.ctx().armas_theme();
let time = ui.input(|i| i.time) as f32;

ui.horizontal(|ui| {
    for i in 0..4 {
        ui.vertical(|ui| {
            let phase = i as f32 * 0.5;
            let level = ((time * 2.0 + phase).sin() * 0.5 + 0.5) * 0.85;

            AudioMeter::new(level)
                .height(180.0)
                .width(25.0)
                .vu_colors(&theme)
                .show(ui);

            ui.label(format!("Ch {}", i + 1));
        });
        ui.add_space(4.0);
    }
});
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(level)` | `f32` | - | Create meter with level (0.0-1.0) |
| `.width(width)` | `f32` | `30.0` | Set meter width |
| `.height(height)` | `f32` | `200.0` | Set meter height |
| `.style(style)` | `MeterStyle` | `Smooth` | Set visual style |
| `.gradient(gradient)` | `Gradient` | - | Use custom gradient |
| `.color_range(min, max)` | `(Color32, Color32)` | green→red | Set color interpolation |
| `.vu_colors(theme)` | `&Theme` | - | Use traditional VU colors |
| `.monochrome(color)` | `Color32` | - | Single color with opacity |
| `.peak_color(color)` | `Color32` | `theme.primary()` | Set peak indicator color |
| `.scale_position(pos)` | `ScalePosition` | `None` | Set scale position (Left/Right/None) |
| `.show_scale()` | - | - | Show scale on right (convenience) |
| `.scale_left()` | - | - | Show scale on left |
| `.scale_right()` | - | - | Show scale on right |
| `.corner_radius(radius)` | `f32` | `16.0` | Set background corner radius |
| `.background_opacity(opacity)` | `f32` | `0.8` | Set background opacity (0-1) |
| `.glassmorphic(enabled)` | `bool` | `true` | Enable glassmorphic background |
| `.show(&mut Ui)` | - | - | Show meter, returns Response |

### MeterStyle

- `MeterStyle::Smooth` - Smooth gradient fill
- `MeterStyle::Segmented(count)` - LED segments with specified count

### ScalePosition

- `ScalePosition::None` - No scale markings
- `ScalePosition::Left` - Scale on the left side
- `ScalePosition::Right` - Scale on the right side

## Features

- **Smooth Animation**: Spring-based physics for natural level tracking
- **Peak Hold**: Automatic peak detection with timed fade
- **Glassmorphic**: Modern semi-transparent background with borders
- **Flexible Colors**: Simple two-color range or full custom gradients
- **Presets**: VU meter, monochrome, and custom color schemes
- **Scale Markings**: Optional dB scale (0, -6, -12, -18, -24, -∞)

## Dependencies

- `egui = "0.33"`
- Spring animation for smooth level changes
- Peak hold with 1.5s hold time + 1.0s fade
