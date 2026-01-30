# Mod Wheel

Rotating cylinder controller for modulation, pitch bend, and expression. Essential for expressive MIDI performance.

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
    .show(ui, &theme);
```

### Pitch Bend (Springs Back to Center)

```demo
let mut pitch = 0.0;

ModWheel::new(&mut pitch)
    .wheel_type(WheelType::PitchBend)
    .label("Pitch".to_string())
    .show(ui, &theme);

ui.label("Release to return to center");
```

### Expression Wheel

```demo
let mut expression = 0.5;

ModWheel::new(&mut expression)
    .wheel_type(WheelType::Expression)
    .label("Expr".to_string())
    .show(ui, &theme);
```

## Sizes

```demo
ui.horizontal(|ui| {
    let mut small = 0.3;
    let mut default = 0.5;
    let mut large = 0.7;

    ModWheel::new(&mut small)
        .size(WheelSize::Small)
        .label("Small".to_string())
        .show(ui, &theme);

    ui.add_space(8.0);

    ModWheel::new(&mut default)
        .size(WheelSize::Default)
        .label("Default".to_string())
        .show(ui, &theme);

    ui.add_space(8.0);

    ModWheel::new(&mut large)
        .size(WheelSize::Large)
        .label("Large".to_string())
        .show(ui, &theme);
});
```

## With Value Display

```demo
let mut value = 0.6;

ModWheel::new(&mut value)
    .wheel_type(WheelType::Modulation)
    .label("Mod".to_string())
    .show_value(true)
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
        .show(ui, &theme);

    ui.add_space(8.0);

    ModWheel::new(&mut pitch_value)
        .wheel_type(WheelType::PitchBend)
        .label("Pitch".to_string())
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
| `.size()` | `WheelSize` | `Default` | Size preset |
| `.label()` | `impl Into<String>` | None | Label below wheel |
| `.show_value()` | `bool` | `false` | Show numeric value |
| `.show_center_line()` | `bool` | Auto | Show center reference line |

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

### WheelSize

```rust
pub enum WheelSize {
    Small,   // 30x120
    Default, // 40x180
    Large,   // 50x220
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

### Cylinder Surface
- Rotating cylinder visible through a recessed slot
- Cylindrical 3D shading (darker at edges, brighter at center)
- Scrolling grip ridges simulate rotation as value changes
- Slot edge shadows add depth

### Center Line
- Automatically shown for pitch bend
- Optional for other types
- Helps find zero/center position

### Layout
- Recessed housing with rounded corners
- Cylinder surface inset within the housing
- Label positioned below wheel
- Value displayed above when enabled

## Interaction

- **Click**: Jump to position
- **Drag**: Smooth continuous control with scrolling ridges
- **Scroll**: Fine adjustment when hovered
- **Release (PitchBend)**: Springs back to center
- **Visual Feedback**: Cylinder brightens slightly on interaction

## Use Cases

### Synth Controller

```demo
ui.horizontal(|ui| {
    let mut mod_wheel = 0.5;
    let mut pitch_wheel = 0.0;

    ModWheel::new(&mut mod_wheel)
        .wheel_type(WheelType::Modulation)
        .label("Mod".to_string())
        .show(ui, &theme);

    ui.add_space(12.0);

    ModWheel::new(&mut pitch_wheel)
        .wheel_type(WheelType::PitchBend)
        .label("Pitch".to_string())
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
    .size(WheelSize::Large)
    .show(ui, &theme);
```

### Filter Sweep

```demo
let mut filter_mod = 0.3;

ModWheel::new(&mut filter_mod)
    .wheel_type(WheelType::Modulation)
    .label("Filter".to_string())
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

## Related Components

- **XYPad**: 2D controller for two parameters
- **Knob**: Rotary control for single parameter
- **Fader**: Linear slider with scale
- **Piano**: Chromatic keyboard input
