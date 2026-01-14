# Zoom Control

Interactive zoom control for timeline scaling. Provides buttons and slider for adjusting timeline zoom level with precise control.

## Basic Usage

```demo
use armas::components::audio::ZoomControl;

let mut zoom_level = 1.0;

ZoomControl::new(&mut zoom_level)
    .id("basic_zoom")
    .min_zoom(0.1)
    .max_zoom(10.0)
    .show(ui);

ui.label(format!("Current zoom: {:.1}x", zoom_level));
```

## Buttons Only

```demo
use armas::components::audio::ZoomControl;

let mut zoom = 1.0;

ZoomControl::new(&mut zoom)
    .id("buttons_only")
    .show_slider(false)
    .show(ui);
```

## Slider Only

```demo
use armas::components::audio::ZoomControl;

let mut zoom = 1.0;

ZoomControl::new(&mut zoom)
    .id("slider_only")
    .show_buttons(false)
    .show(ui);
```

## Custom Range and Step

```demo
use armas::components::audio::ZoomControl;

let mut zoom = 1.0;

ZoomControl::new(&mut zoom)
    .id("custom_range")
    .min_zoom(0.5)
    .max_zoom(5.0)
    .button_step(0.5)  // Larger steps
    .show(ui);
```

## Compact Mode

```demo
use armas::components::audio::ZoomControl;

let mut zoom = 1.0;

ZoomControl::new(&mut zoom)
    .id("compact")
    .show_label(false)
    .slider_width(100.0)
    .show(ui);
```

## Visual Design

### Components
- **Label**: "Zoom:" text (optional)
- **Zoom Out Button**: "−" decreases zoom
- **Slider**: Logarithmic scale for smooth zooming
- **Zoom In Button**: "+" increases zoom
- **Zoom Display**: Shows current zoom level (e.g., "1.5x")
- **1:1 Button**: Reset to 100% zoom
- **Fit Button**: Zoom to fit content

### Default Settings
- Min Zoom: 0.1x (10%)
- Max Zoom: 10.0x (1000%)
- Button Step: 0.2x
- Slider Width: 150px
- Logarithmic scaling for natural feel

## API Reference

### Constructor
```rust
ZoomControl::new(zoom_level: &mut f32) -> Self
```

### Builder Methods
| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.id()` | `egui::Id` | `None` | Set ID for state persistence (required for demos) |
| `.min_zoom()` | `f32` | `0.1` | Minimum zoom level |
| `.max_zoom()` | `f32` | `10.0` | Maximum zoom level |
| `.show_slider()` | `bool` | `true` | Show slider control |
| `.show_buttons()` | `bool` | `true` | Show +/- buttons |
| `.show_label()` | `bool` | `true` | Show "Zoom:" label |
| `.button_step()` | `f32` | `0.2` | Zoom step for buttons |
| `.slider_width()` | `f32` | `150.0` | Slider width in pixels |

### Response
```rust
pub struct ZoomControlResponse {
    pub response: Response,
    pub changed: bool,
    pub zoomed_in: bool,
    pub zoomed_out: bool,
    pub reset: bool,
}
```

## Use Cases

### Timeline Zoom
```demo
use armas::components::audio::{ZoomControl, Marker};

let mut zoom = 1.0;
let mut marker_pos = 16.0;

ZoomControl::new(&mut zoom)
    .id("timeline_zoom_demo")
    .show(ui);

ui.add_space(8.0);

// Apply zoom to beat_width
let beat_width = 60.0 * zoom;

egui::ScrollArea::horizontal().show(ui, |ui| {
    Marker::new(&mut marker_pos, "Chorus")
        .id("zoom_marker")
        .beat_width(beat_width)
        .measures(16)
        .show(ui);
});
```

### Adaptive Zoom
```demo
use armas::components::audio::ZoomControl;

let mut zoom = 1.0;

let response = ZoomControl::new(&mut zoom)
    .id("adaptive_zoom")
    .min_zoom(0.25)
    .max_zoom(4.0)
    .show(ui);

if response.changed {
    ui.label(format!("Zoom changed to {:.1}x", zoom));
}

if response.reset {
    ui.label("Reset to 1:1");
}
```

## Common Zoom Presets

| Zoom Level | Description | Use Case |
|-----------|-------------|----------|
| 0.1x | 10% | Overview of entire project |
| 0.5x | 50% | Large section view |
| 1.0x | 100% | Default view |
| 2.0x | 200% | Detailed editing |
| 5.0x | 500% | Precise sample editing |
| 10.0x | 1000% | Maximum zoom for waveform |

## Related Components
- **Timeline**: Main timeline component (applies zoom to beat_width)
- **SnapGrid**: Grid adapts to zoom level
- **All Markers**: Scale with zoom level
