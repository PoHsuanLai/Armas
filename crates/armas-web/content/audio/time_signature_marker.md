# Time Signature Marker

Visual markers for time signature (meter) changes. Shows a vertical line with a time signature badge displaying the meter (e.g., "4/4", "3/4", "7/8").

## Basic Usage

```demo
use armas_audio::TimeSignatureMarker;

let mut position = 32.0; // beats
let mut numerator = 3;
let mut denominator = 4;

egui::ScrollArea::horizontal().show(ui, |ui| {
    TimeSignatureMarker::new(&mut position, &mut numerator, &mut denominator)
        .beat_width(60.0)
        .measures(16)
        .show(ui);
});
```

## Multiple Time Signatures

```demo
use armas_audio::TimeSignatureMarker;

let mut ts1_pos = 0.0;
let mut ts1_num = 4;
let mut ts1_den = 4;
let mut ts2_pos = 32.0;
let mut ts2_num = 7;
let mut ts2_den = 8;

egui::ScrollArea::horizontal().show(ui, |ui| {
    ui.horizontal(|ui| {
        TimeSignatureMarker::new(&mut ts1_pos, &mut ts1_num, &mut ts1_den)
            .beat_width(40.0)
            .measures(16)
            .id("ts1")
            .show(ui);

        TimeSignatureMarker::new(&mut ts2_pos, &mut ts2_num, &mut ts2_den)
            .beat_width(40.0)
            .measures(16)
            .id("ts2")
            .show(ui);
    });
});
```

## Custom Color

```demo
use armas_audio::TimeSignatureMarker;

let mut position = 16.0;
let mut num = 5;
let mut den = 4;

egui::ScrollArea::horizontal().show(ui, |ui| {
    TimeSignatureMarker::new(&mut position, &mut num, &mut den)
        .beat_width(50.0)
        .measures(16)
        .color(egui::Color32::from_rgb(255, 150, 200))
        .show(ui);
});
```

## Common Time Signatures

- **4/4**: Common time (most popular)
- **3/4**: Waltz time
- **6/8**: Compound duple
- **7/8**: Progressive/odd meter
- **5/4**: Take Five time
- **12/8**: Compound quadruple

## Visual Design

- **Height**: 70px
- **Badge**: 24px tall, 28px wide
- **Display**: Stacked numerator/denominator with divider line
- **Vertical Line**: 2px width from badge to bottom
- **Default Color**: Purple (180, 100, 220)
- **Tooltip**: Shows time signature and position on hover

## API Reference

### Constructor
```rust
TimeSignatureMarker::new(position: &mut f32, numerator: &mut u32, denominator: &mut u32) -> Self
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
| `.snap_to_grid()` | `bool` | `true` | Snap to grid |
| `.grid_division()` | `f32` | `1.0` | Grid division |
| `.color()` | `Color32` | Purple (180, 100, 220) | Custom color |
| `.show_line()` | `bool` | `true` | Show vertical line |

## Related Components
- **TempoMarker**: Tempo changes
- **Marker**: Cue points and navigation
- **Timeline**: Main timeline component
