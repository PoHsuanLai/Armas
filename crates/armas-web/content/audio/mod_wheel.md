# Mod Wheel

Vertical strip controller for modulation, pitch bend, and expression. Essential for expressive MIDI performance.

## Basic Usage

```demo
let mut value = 0.0;

let response = ModWheel::new(&mut value)
    .wheel_type(WheelType::Modulation)
    .label("Mod".to_string())
    .show(ui, &theme);

if response.changed() {
    ui.label(format!("Value: {:.2}", value));
}
```

## Wheel Types

### Modulation Wheel

```demo
let mut mod_value = 0.0;

ModWheel::new(&mut mod_value)
    .wheel_type(WheelType::Modulation)
    .label("Mod".to_string())
    .height(200.0)
    .show(ui, &theme);
```

### Pitch Bend (Springs Back to Center)

```demo
let mut pitch = 0.0;

ModWheel::new(&mut pitch)
    .wheel_type(WheelType::PitchBend)
    .label("Pitch".to_string())
    .height(200.0)
    .show(ui, &theme);

ui.label("Release to return to center");
```

### Expression Wheel

```demo
let mut expression = 0.5;

ModWheel::new(&mut expression)
    .wheel_type(WheelType::Expression)
    .label("Expr".to_string())
    .height(200.0)
    .show(ui, &theme);
```

## Variants

### Filled (Default)

```demo
let mut value = 0.3;

ModWheel::new(&mut value)
    .wheel_type(WheelType::Modulation)
    .variant(WheelVariant::Filled)
    .height(180.0)
    .show(ui, &theme);
```

### Outlined

```demo
let mut value = 0.5;

ModWheel::new(&mut value)
    .wheel_type(WheelType::Modulation)
    .variant(WheelVariant::Outlined)
    .height(180.0)
    .show(ui, &theme);
```

### Elevated

```demo
let mut value = 0.7;

ModWheel::new(&mut value)
    .wheel_type(WheelType::Modulation)
    .variant(WheelVariant::Elevated)
    .height(180.0)
    .show(ui, &theme);
```

## With Value Display

```demo
let mut value = 0.6;

ModWheel::new(&mut value)
    .wheel_type(WheelType::Modulation)
    .label("Mod".to_string())
    .show_value(true)
    .height(200.0)
    .show(ui, &theme);
```

## Custom Sizing

```demo
let mut value = 0.4;

ModWheel::new(&mut value)
    .wheel_type(WheelType::Modulation)
    .width(50.0)
    .height(250.0)
    .label("Wide".to_string())
    .show(ui, &theme);
```

## Pitch Bend Example

```demo
let mut pitch_bend = 0.0;

let response = ModWheel::new(&mut pitch_bend)
    .wheel_type(WheelType::PitchBend)
    .label("Pitch".to_string())
    .show_value(true)
    .show_center_line(true)
    .height(220.0)
    .show(ui, &theme);

// Convert to semitones (+/- 2 semitones typical)
let semitones = pitch_bend * 2.0;
ui.label(format!("Pitch: {:+.1} semitones", semitones));
```

## Multiple Wheels

```demo
ui.horizontal(|ui| {
    let mut mod_value = 0.0;
    let mut pitch_value = 0.0;

    ModWheel::new(&mut mod_value)
        .wheel_type(WheelType::Modulation)
        .label("Mod".to_string())
        .height(180.0)
        .show(ui, &theme);

    ui.add_space(8.0);

    ModWheel::new(&mut pitch_value)
        .wheel_type(WheelType::PitchBend)
        .label("Pitch".to_string())
        .height(180.0)
        .show(ui, &theme);
});
```

## API Reference

### Constructor

```rust
ModWheel::new(value: &mut f32) -> Self
```

Creates a new mod wheel with mutable reference to value.

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.wheel_type()` | `WheelType` | `Modulation` | Type of wheel controller |
| `.variant()` | `WheelVariant` | `Filled` | Visual variant |
| `.width()` | `f32` | `40.0` | Width in pixels |
| `.height()` | `f32` | `200.0` | Height in pixels |
| `.label()` | `impl Into<String>` | None | Label below wheel |
| `.show_value()` | `bool` | `false` | Show numeric value |
| `.show_center_line()` | `bool` | Auto | Show center reference line |
| `.glow_intensity()` | `f32` | `0.8` | Glow intensity when dragging (0.0-1.0) |

### Show Method

```rust
pub fn show(self, ui: &mut egui::Ui) -> Response
```

Returns an egui `Response`. Check `response.changed()` to detect value changes.

### WheelType

```rust
pub enum WheelType {
    Modulation,  // Stays at position (0.0 to 1.0)
    PitchBend,   // Springs back to center (-1.0 to 1.0)
    Expression,  // Stays at position (0.0 to 1.0)
}
```

**Key Differences:**
- **Modulation/Expression**: Values from 0.0 to 1.0, stays where positioned
- **PitchBend**: Values from -1.0 to 1.0, automatically returns to 0.0 when released

### WheelVariant

```rust
pub enum WheelVariant {
    Filled,   // Solid background
    Outlined, // Transparent with border
    Elevated, // Shadow effect
}
```

## Value Range

### Modulation & Expression
- Range: **0.0 (bottom) to 1.0 (top)**
- Value persists after release

### Pitch Bend
- Range: **-1.0 (down) to 1.0 (up)**
- **Automatically returns to 0.0** when released
- Center line shown by default
- Typically mapped to ±2 semitones

## Visual Design

### Handle
- Rectangular draggable strip
- Primary theme color
- Multi-layer glow effect when dragging
- 16px height, spans width minus margins

### Center Line
- Automatically shown for pitch bend
- Optional for other types
- Helps find zero/center position

### Layout
- Vertical strip with rounded corners (8px radius)
- Label positioned below wheel
- Value displayed above when enabled

## Interaction

- **Click**: Jump to position
- **Drag**: Smooth continuous control
- **Release (PitchBend)**: Springs back to center
- **Visual Feedback**: Glow effect on interaction

## Use Cases

### Synth Controller

```demo
ui.horizontal(|ui| {
    let mut mod_wheel = 0.5;
    let mut pitch_wheel = 0.0;

    ModWheel::new(&mut mod_wheel)
        .wheel_type(WheelType::Modulation)
        .label("Mod".to_string())
        .height(200.0)
        .variant(WheelVariant::Elevated)
        .show(ui, &theme);

    ui.add_space(12.0);

    ModWheel::new(&mut pitch_wheel)
        .wheel_type(WheelType::PitchBend)
        .label("Pitch".to_string())
        .height(200.0)
        .variant(WheelVariant::Elevated)
        .show(ui, &theme);
});
```

### Expression Control

```demo
let mut dynamics = 0.7;

ModWheel::new(&mut dynamics)
    .wheel_type(WheelType::Expression)
    .label("Dynamics".to_string())
    .show_value(true)
    .height(220.0)
    .show(ui, &theme);
```

### Filter Sweep

```demo
let mut filter_mod = 0.3;

ModWheel::new(&mut filter_mod)
    .wheel_type(WheelType::Modulation)
    .label("Filter".to_string())
    .height(200.0)
    .glow_intensity(1.2)
    .show(ui, &theme);

let cutoff_hz = 100.0 + filter_mod * 5000.0;
ui.label(format!("Cutoff: {:.0} Hz", cutoff_hz));
```

## MIDI Mapping

Standard MIDI CC mappings:

| Wheel Type | MIDI CC | Range | Behavior |
|------------|---------|-------|----------|
| Modulation | CC 1 | 0-127 | Stays at position |
| Pitch Bend | Pitch Bend | -8192 to +8191 | Springs to center |
| Expression | CC 11 | 0-127 | Stays at position |

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface`, `surface_variant`, `outline`, `outline_variant`, `on_surface`, `on_surface_variant`
- Minimum version: `armas 0.1.0`

## Related Components

- **XYPad**: 2D controller for two parameters
- **Knob**: Rotary control for single parameter
- **Fader**: Linear slider with scale
- **Piano**: Chromatic keyboard input
