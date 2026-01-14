# Snap Grid

Visual grid lines for timeline alignment. Draws vertical lines at beat/subdivision intervals for visual reference and precise positioning.

## Basic Usage

```demo
use armas::components::audio::SnapGrid;

ui.vertical(|ui| {
    SnapGrid::new()
        .beat_width(60.0)
        .measures(16)
        .subdivision(4)  // 16th notes
        .show(ui);
});
```

## Different Subdivisions

```demo
use armas::components::audio::SnapGrid;

ui.label("16th Notes:");
SnapGrid::new()
    .beat_width(50.0)
    .measures(8)
    .subdivision(4)
    .show(ui);

ui.add_space(16.0);

ui.label("8th Notes:");
SnapGrid::new()
    .beat_width(50.0)
    .measures(8)
    .subdivision(2)
    .show(ui);
```

## Custom Opacity

```demo
use armas::components::audio::SnapGrid;

SnapGrid::new()
    .beat_width(60.0)
    .measures(16)
    .subdivision(4)
    .measure_opacity(0.7)   // Stronger measure lines
    .beat_opacity(0.4)      // Medium beat lines
    .subdivision_opacity(0.1) // Subtle subdivision lines
    .show(ui);
```

## Beats Only (No Subdivisions)

```demo
use armas::components::audio::SnapGrid;

SnapGrid::new()
    .beat_width(60.0)
    .measures(16)
    .show_subdivisions(false)
    .show(ui);
```

## Custom Colors

```demo
use armas::components::audio::SnapGrid;

SnapGrid::new()
    .beat_width(60.0)
    .measures(16)
    .subdivision(4)
    .measure_color(egui::Color32::from_rgb(100, 200, 255))
    .beat_color(egui::Color32::from_rgb(150, 150, 150))
    .subdivision_color(egui::Color32::from_rgb(80, 80, 80))
    .show(ui);
```

## Visual Design

### Line Types
- **Measure Lines**: Strongest (1.5px, 50% opacity default)
- **Beat Lines**: Medium (1.0px, 30% opacity default)
- **Subdivision Lines**: Weakest (0.5px, 15% opacity default)

### Common Subdivision Values
| Value | Musical Division | Notes |
|-------|------------------|-------|
| `1` | Whole beats only | Quarter notes |
| `2` | 8th notes | Half beat |
| `4` | 16th notes | Quarter beat |
| `8` | 32nd notes | Eighth beat |

## API Reference

### Constructor
```rust
SnapGrid::new() -> Self
```

### Builder Methods
| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.beat_width()` | `f32` | `60.0` | Pixels per beat |
| `.measures()` | `u32` | `16` | Number of measures |
| `.beats_per_measure()` | `u32` | `4` | Beats per measure |
| `.subdivision()` | `u32` | `4` | Lines per beat |
| `.show_beats()` | `bool` | `true` | Show beat lines |
| `.show_measures()` | `bool` | `true` | Show measure lines |
| `.show_subdivisions()` | `bool` | `true` | Show subdivision lines |
| `.beat_color()` | `Color32` | Theme outline | Custom beat color |
| `.measure_color()` | `Color32` | Theme outline | Custom measure color |
| `.subdivision_color()` | `Color32` | Theme outline | Custom subdivision color |
| `.beat_opacity()` | `f32` | `0.3` | Beat line opacity (0.0-1.0) |
| `.measure_opacity()` | `f32` | `0.5` | Measure line opacity (0.0-1.0) |
| `.subdivision_opacity()` | `f32` | `0.15` | Subdivision opacity (0.0-1.0) |

## Use Cases

### Background Grid for Timeline
```demo
use armas::components::audio::{SnapGrid, Marker};

let mut marker_pos = 16.0;

ui.vertical(|ui| {
    // Grid in background
    egui::ScrollArea::horizontal().show(ui, |ui| {
        SnapGrid::new()
            .beat_width(60.0)
            .measures(16)
            .subdivision(4)
            .show(ui);
    });

    // Markers on top
    egui::ScrollArea::horizontal().show(ui, |ui| {
        Marker::new(&mut marker_pos, "Chorus")
            .beat_width(60.0)
            .measures(16)
            .show(ui);
    });
});
```

## Related Components
- **Timeline**: Main timeline component
- **TimeRuler**: Beat/bar number display
- **All Markers**: Work with grid for alignment
