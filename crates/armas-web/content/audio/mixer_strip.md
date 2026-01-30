# Mixer Strip

DAW-style mixer channel strip with sends, routing, inserts, pan, mute/solo, meter, and fader.

## Basic Usage

```demo
use egui::Color32;

let time = ui.input(|i| i.time) as f32;
let level = ((time * 2.0).sin() * 0.5 + 0.5) * 0.8;

let mut strip = MixerStrip::new("Lead")
    .mode(MixerStripMode::Full)
    .width(80.0)
    .fader_level(0.75)
    .meter_level(level)
    .sends_color(Color32::from_gray(30))
    .routing_color(Color32::from_gray(24))
    .inserts_color(Color32::from_gray(18))
    .inserts(vec![
        Insert::new("Pro-Q 3"),
        Insert::new("LA-2A"),
        Insert::empty(),
        Insert::empty(),
    ]);

strip.show(ui, &theme);
```

## Multiple Channels

```demo
use egui::Color32;

let time = ui.input(|i| i.time) as f32;

ui.horizontal(|ui| {
    let channels = [
        ("Vocal", Color32::from_rgb(35, 25, 30), Color32::from_rgb(255, 100, 150), Color32::from_rgb(255, 80, 120)),
        ("Guitar", Color32::from_rgb(25, 30, 35), Color32::from_rgb(100, 200, 255), Color32::from_rgb(80, 180, 255)),
        ("Drums", Color32::from_rgb(30, 28, 25), Color32::from_rgb(255, 180, 100), Color32::from_rgb(255, 160, 80)),
        ("Bass", Color32::from_rgb(28, 32, 28), Color32::from_rgb(150, 255, 150), Color32::from_rgb(120, 255, 120)),
    ];

    for (i, (name, card_color, knob_color, meter_color)) in channels.iter().enumerate() {
        let phase = i as f32 * 0.5;
        let level = ((time * 2.0 + phase).sin() * 0.5 + 0.5) * 0.85;

        let mut strip = MixerStrip::new(*name)
            .width(70.0)
            .fader_level(0.7 + (i as f32 * 0.05))
            .meter_level(level)
            .card_color(*card_color)
            .knob_color(*knob_color)
            .meter_color(*meter_color);

        strip.show(ui, &theme);

        if i < channels.len() - 1 {
            ui.add_space(4.0);
        }
    }
});
```

## Display Modes

Progressively hides sections as height decreases. Set explicitly or use `Auto` (default) to adapt to available space.

```demo
use egui::Color32;

let time = ui.input(|i| i.time) as f32;

ui.horizontal(|ui| {
    let modes = [
        ("Full", MixerStripMode::Full),
        ("Standard", MixerStripMode::Standard),
        ("Compact", MixerStripMode::Compact),
        ("Minimal", MixerStripMode::Minimal),
    ];

    for (i, (name, mode)) in modes.iter().enumerate() {
        let level = ((time * 2.0 + i as f32 * 0.5).sin() * 0.5 + 0.5) * 0.8;

        let mut strip = MixerStrip::new(*name)
            .mode(*mode)
            .meter_level(level);

        strip.show(ui, &theme);

        if i < modes.len() - 1 {
            ui.add_space(4.0);
        }
    }
});
```

| Mode | Scroll Rows | Pan Label | R/I | Gain |
|------|-------------|-----------|-----|------|
| Full | 4 | yes | yes | yes |
| Standard | 3 | yes | yes | yes |
| Compact | 2 | no | no | yes |
| Minimal | 1 | no | no | no |

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(name)` | `impl Into<String>` | - | Create mixer strip |
| `.width(w)` | `f32` | `70.0` | Strip width |
| `.mode(mode)` | `MixerStripMode` | `Auto` | Display mode |
| `.scale(s)` | `f32` | `1.0` | Zoom factor (0.5-2.0) |
| `.fader_level(l)` | `f32` | `0.75` | Fader level (0.0-1.0) |
| `.pan(p)` | `f32` | `0.0` | Pan (-1.0 to 1.0) |
| `.meter_level(l)` | `f32` | `0.0` | Meter level (0.0-1.0) |
| `.card_color(c)` | `Color32` | dark | Card background |
| `.knob_color(c)` | `Color32` | primary | Pan knob glow |
| `.meter_color(c)` | `Color32` | primary | Meter color |
