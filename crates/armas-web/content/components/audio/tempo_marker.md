# Tempo Marker

Visual markers for tempo changes with BPM display. Shows a vertical line with a BPM badge and small triangle flag for marking tempo automation points.

## Basic Usage

```demo
use armas::components::audio::TempoMarker;

let mut position = 16.0; // beats
let mut bpm = 120.0;

egui::ScrollArea::horizontal().show(ui, |ui| {
    TempoMarker::new(&mut position, &mut bpm)
        .beat_width(60.0)
        .measures(16)
        .show(ui);
});
```

## Multiple Tempo Changes

```demo
use armas::components::audio::TempoMarker;

let mut tempo1_pos = 0.0;
let mut tempo1_bpm = 120.0;
let mut tempo2_pos = 32.0;
let mut tempo2_bpm = 140.0;

egui::ScrollArea::horizontal().show(ui, |ui| {
    ui.horizontal(|ui| {
        TempoMarker::new(&mut tempo1_pos, &mut tempo1_bpm)
            .beat_width(40.0)
            .measures(16)
            .id("tempo1")
            .show(ui);

        TempoMarker::new(&mut tempo2_pos, &mut tempo2_bpm)
            .beat_width(40.0)
            .measures(16)
            .id("tempo2")
            .show(ui);
    });
});
```

## Custom Color

```demo
use armas::components::audio::TempoMarker;

let mut position = 8.0;
let mut bpm = 180.0;

egui::ScrollArea::horizontal().show(ui, |ui| {
    TempoMarker::new(&mut position, &mut bpm)
        .beat_width(50.0)
        .measures(16)
        .color(egui::Color32::from_rgb(255, 200, 50))
        .show(ui);
});
```

## Non-Draggable

```demo
use armas::components::audio::TempoMarker;

let mut position = 0.0;
let mut bpm = 128.0;

egui::ScrollArea::horizontal().show(ui, |ui| {
    TempoMarker::new(&mut position, &mut bpm)
        .beat_width(50.0)
        .measures(16)
        .draggable(false)
        .show(ui);
});
```

## Visual Design

- **Height**: 70px
- **Badge**: 20px tall with BPM text (e.g., "120 BPM")
- **Triangle Flag**: Small indicator below badge
- **Vertical Line**: 2px width from flag to bottom
- **Default Color**: Teal/green (50, 200, 150)
- **Tooltip**: Shows tempo and position on hover

## API Reference

### Constructor
```rust
TempoMarker::new(position: &mut f32, bpm: &mut f32) -> Self
```

### Builder Methods
| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.id()` | `impl Into<egui::Id>` | None | Unique ID for state persistence |
| `.beat_width()` | `f32` | `60.0` | Pixels per beat |
| `.measures()` | `u32` | `16` | Number of measures |
| `.beats_per_measure()` | `u32` | `4` | Beats per measure |
| `.enabled()` | `bool` | `true` | Enable or disable |
| `.draggable()` | `bool` | `true` | Position draggable |
| `.editable_bpm()` | `bool` | `true` | BPM editable (future) |
| `.snap_to_grid()` | `bool` | `true` | Snap to grid |
| `.grid_division()` | `f32` | `1.0` | Grid division |
| `.color()` | `Color32` | Teal (50, 200, 150) | Custom color |
| `.show_line()` | `bool` | `true` | Show vertical line |

## Related Components
- **Marker**: Cue points and navigation
- **TimeSignatureMarker**: Meter changes
- **Timeline**: Main timeline component
