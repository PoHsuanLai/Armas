# XY Pad

2D touch controller for simultaneous control of two parameters. Perfect for filter controls, spatial effects, and expressive performance.

## Basic Usage

```demo
let mut x = 0.5;
let mut y = 0.5;

let response = XYPad::new(&mut x, &mut y)
    .size(200.0)
    .show(ui);

if response.changed() {
    ui.label(format!("X: {:.2}, Y: {:.2}", x, y));
}
```

## With Labels

```demo
let mut x = 0.5;
let mut y = 0.5;

XYPad::new(&mut x, &mut y)
    .size(200.0)
    .x_label("Cutoff".to_string())
    .y_label("Resonance".to_string())
    .show(ui);
```

## With Values Display

```demo
let mut x = 0.3;
let mut y = 0.7;

XYPad::new(&mut x, &mut y)
    .size(200.0)
    .show_values(true)
    .show(ui);
```

## Variants

### Filled (Default)

```demo
let mut x = 0.5;
let mut y = 0.5;

XYPad::new(&mut x, &mut y)
    .size(180.0)
    .variant(XYPadVariant::Filled)
    .show(ui);
```

### Outlined

```demo
let mut x = 0.5;
let mut y = 0.5;

XYPad::new(&mut x, &mut y)
    .size(180.0)
    .variant(XYPadVariant::Outlined)
    .show(ui);
```

### Elevated

```demo
let mut x = 0.5;
let mut y = 0.5;

XYPad::new(&mut x, &mut y)
    .size(180.0)
    .variant(XYPadVariant::Elevated)
    .show(ui);
```

## Without Crosshair

```demo
let mut x = 0.6;
let mut y = 0.4;

XYPad::new(&mut x, &mut y)
    .size(200.0)
    .show_crosshair(false)
    .show(ui);
```

## Custom Size and Handle

```demo
let mut x = 0.5;
let mut y = 0.5;

XYPad::new(&mut x, &mut y)
    .size(250.0)
    .handle_size(20.0)
    .glow_intensity(1.2)
    .show(ui);
```

## Filter Control Example

```demo
let mut cutoff = 0.5;
let mut resonance = 0.3;

XYPad::new(&mut cutoff, &mut resonance)
    .size(200.0)
    .x_label("Cutoff".to_string())
    .y_label("Resonance".to_string())
    .show_values(true)
    .show(ui);

ui.label(format!("Filter: {:.0}Hz, Q: {:.2}",
    20.0 + cutoff * 20000.0,
    0.5 + resonance * 10.0
));
```

## Spatial Effect Example

```demo
let mut pan = 0.5;
let mut depth = 0.5;

XYPad::new(&mut pan, &mut depth)
    .size(200.0)
    .x_label("Pan".to_string())
    .y_label("Depth".to_string())
    .show_crosshair(true)
    .show(ui);
```

## API Reference

### Constructor

```rust
XYPad::new(x: &mut f32, y: &mut f32) -> Self
```

Creates a new XY pad with mutable references to X and Y values (0.0-1.0).

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.size()` | `f32` | `200.0` | Pad size (width and height) in pixels |
| `.variant()` | `XYPadVariant` | `Filled` | Visual variant |
| `.x_label()` | `impl Into<String>` | None | Label for X axis |
| `.y_label()` | `impl Into<String>` | None | Label for Y axis |
| `.show_crosshair()` | `bool` | `true` | Show crosshair lines at handle position |
| `.show_values()` | `bool` | `false` | Show numeric X/Y values |
| `.handle_size()` | `f32` | `16.0` | Size of draggable handle |
| `.glow_intensity()` | `f32` | `0.8` | Glow intensity when dragging (0.0-1.0) |

### Show Method

```rust
pub fn show(self, ui: &mut egui::Ui) -> Response
```

Returns an egui `Response`. Check `response.changed()` to detect value changes.

### XYPadVariant

```rust
pub enum XYPadVariant {
    Filled,   // Solid background
    Outlined, // Transparent with border
    Elevated, // Shadow effect
}
```

## Value Range

- Both X and Y values are normalized: **0.0 to 1.0**
- X increases from left (0.0) to right (1.0)
- Y increases from bottom (0.0) to top (1.0)
- Values are automatically clamped to valid range

## Visual Design

### Handle
- Circular draggable control point
- Primary theme color
- Multi-layer glow effect when dragging
- Configurable size (default 16px diameter)

### Crosshair
- Optional horizontal and vertical lines through handle
- Helps visualize exact position
- Uses `on_surface_variant` color at low opacity

### Layout
- Square pad with rounded corners (12px radius)
- Labels positioned outside pad area
- Values displayed at top when enabled

## Interaction

- **Click**: Set position immediately
- **Drag**: Smooth continuous control
- **Visual Feedback**: Glow effect on interaction
- **Clamping**: Values automatically clamped to 0.0-1.0

## Use Cases

### Filter Sweeps

```demo
let mut cutoff = 0.5;
let mut resonance = 0.2;

XYPad::new(&mut cutoff, &mut resonance)
    .size(200.0)
    .x_label("Cutoff".to_string())
    .y_label("Q".to_string())
    .variant(XYPadVariant::Elevated)
    .show(ui);
```

### Reverb Control

```demo
let mut size = 0.6;
let mut mix = 0.3;

XYPad::new(&mut size, &mut mix)
    .size(180.0)
    .x_label("Size".to_string())
    .y_label("Mix".to_string())
    .show(ui);
```

### Delay Feedback

```demo
let mut time = 0.4;
let mut feedback = 0.5;

XYPad::new(&mut time, &mut feedback)
    .size(200.0)
    .x_label("Time".to_string())
    .y_label("Feedback".to_string())
    .show_values(true)
    .show(ui);
```

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface`, `surface_variant`, `outline`, `outline_variant`, `on_surface`, `on_surface_variant`
- Minimum version: `armas 0.1.0`

## Related Components

- **ModWheel**: 1D vertical controller for single parameter
- **Knob**: Rotary control for single parameter
- **Fader**: Linear slider for single parameter
- **MidiPad**: Grid of trigger pads
