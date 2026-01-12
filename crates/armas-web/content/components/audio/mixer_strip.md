# Mixer Strip

Complete DAW-style mixer channel strip with sends, routing, inserts, pan, mute/solo, meter, and fader.

## Basic Usage

```demo
use egui::Color32;
use std::f32::consts::PI;

let time = ui.input(|i| i.time) as f32;
let level = ((time * 2.0).sin() * 0.5 + 0.5) * 0.8;

let mut strip = MixerStrip::new("Vocal")
    .width(80.0)
    .fader_level(0.75)
    .pan(0.0)
    .meter_level(level);

strip.show(ui);
```

## Multiple Channels

```demo
use egui::Color32;
use std::f32::consts::PI;

let time = ui.input(|i| i.time) as f32;

ui.horizontal(|ui| {
    let channels = [
        ("Vocal", Color32::from_rgb(35, 25, 30)),
        ("Guitar", Color32::from_rgb(25, 30, 35)),
        ("Drums", Color32::from_rgb(30, 28, 25)),
        ("Bass", Color32::from_rgb(28, 32, 28)),
    ];

    for (i, (name, color)) in channels.iter().enumerate() {
        let phase = i as f32 * 0.5;
        let level = ((time * 2.0 + phase).sin() * 0.5 + 0.5) * 0.85;

        let mut strip = MixerStrip::new(*name)
            .width(70.0)
            .fader_level(0.7 + (i as f32 * 0.05))
            .meter_level(level)
            .card_color(*color);

        strip.show(ui);

        if i < channels.len() - 1 {
            ui.add_space(4.0);
        }
    }
});
```

## With Custom Settings

```demo
use egui::Color32;
use std::f32::consts::PI;

let time = ui.input(|i| i.time) as f32;
let level = ((time * 1.5).sin() * 0.5 + 0.5) * 0.9;

let mut strip = MixerStrip::new("Drum Bus")
    .width(70.0)
    .fader_level(0.8)
    .pan(-0.3)
    .meter_level(level);

strip.show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(name)` | `impl Into<String>` | - | Create mixer strip with name |
| `.width(width)` | `f32` | `70.0` | Set strip width |
| `.fader_level(level)` | `f32` | `0.75` | Set fader level (0.0-1.0) |
| `.pan(pan)` | `f32` | `0.0` | Set pan value (-1.0 to 1.0) |
| `.meter_level(level)` | `f32` | `0.0` | Set meter level (0.0-1.0) |
| `.card_color(color)` | `Color32` | `RGB(28,28,30)` | Set card background color |
| `.show(&mut Ui)` | - | - | Show mixer strip, returns Response |

## Features

- **Sends Section**: Manages send effects (Reverb, Delay)
- **Routing**: Input and output routing buttons
- **Insert Slots**: 4 plugin insert slots
- **Pan Control**: Rotary pan knob with center indicator
- **Mute/Solo**: Toggle buttons with outlined style
- **Gain Display**: Real-time dB value display
- **Meter**: Professional audio level meter
- **Fader**: Smooth volume fader control
- **Card Container**: Dark card background for professional DAW aesthetic

## Components Used

The mixer strip internally uses:
- `Card` - Container with dark background
- `Badge` - Send effect labels
- `Button` - All interactive controls (sends, routing, M/S)
- `GlowingDivider` - Visual separator
- `Knob` - Pan control
- `Slot` - Plugin insert slots
- `AudioMeter` - Level metering
- `Fader` - Volume fader
- `Modal` - Send configuration dialog

## State Management

The mixer strip maintains its own state including:
- Fader level (converted to dB)
- Pan position (-1.0 to 1.0)
- Mute/Solo state
- Insert plugin names
- Input/Output routing

## Layout

The strip layout from top to bottom:
1. Sends button (opens modal)
2. Send badges (vertical list)
3. Glowing divider
4. Input routing button
5. Output routing button
6. 4x Insert slots
7. Pan knob
8. Mute/Solo buttons
9. dB value display
10. Meter + Fader (side by side)
11. Channel name

## Dependencies

- `egui = "0.33"`
- Internal components: Card, Badge, Button, GlowingDivider, Knob, Slot, AudioMeter, Fader, Modal
