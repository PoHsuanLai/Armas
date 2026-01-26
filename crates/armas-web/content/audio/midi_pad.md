# MIDI Pad

Grid-based drum pad controller with velocity-sensitive visual feedback. Perfect for drum machines, samplers, and MPC-style controllers.

## Basic Usage

```demo
let pads = vec![
    PadConfig::new(36).label("Kick".to_string()),
    PadConfig::new(38).label("Snare".to_string()),
    PadConfig::new(42).label("HH".to_string()),
    PadConfig::new(46).label("Tom".to_string()),
];

let response = MidiPad::new()
    .grid(2, 2)
    .pads(pads)
    .show(ui, &theme);

if let Some((note, velocity)) = response.pressed {
    ui.label(format!("Note {}, Velocity {}", note, velocity));
}
```

## Grid Layouts

### 4x4 Grid (Default)

```demo
let pads = (0..16).map(|i| PadConfig::new(36 + i)).collect();

MidiPad::new()
    .grid(4, 4)
    .pads(pads)
    .show(ui, &theme);
```

### 2x8 Grid

```demo
let pads = (0..16).map(|i| PadConfig::new(36 + i)).collect();

MidiPad::new()
    .grid(2, 8)
    .pads(pads)
    .show(ui, &theme);
```

### Custom Size

```demo
let pads = vec![
    PadConfig::new(36).label("BD".to_string()),
    PadConfig::new(38).label("SD".to_string()),
];

MidiPad::new()
    .grid(1, 2)
    .pad_size(80.0)
    .gap(12.0)
    .pads(pads)
    .show(ui, &theme);
```

## Variants

### Filled (Default)

```demo
let pads = vec![
    PadConfig::new(36).label("Kick".to_string()),
    PadConfig::new(38).label("Snare".to_string()),
    PadConfig::new(42).label("HH".to_string()),
    PadConfig::new(46).label("Tom".to_string()),
];

MidiPad::new()
    .grid(2, 2)
    .pads(pads)
    .variant(PadVariant::Filled)
    .show(ui, &theme);
```

### Outlined

```demo
let pads = vec![
    PadConfig::new(36).label("Kick".to_string()),
    PadConfig::new(38).label("Snare".to_string()),
    PadConfig::new(42).label("HH".to_string()),
    PadConfig::new(46).label("Tom".to_string()),
];

MidiPad::new()
    .grid(2, 2)
    .pads(pads)
    .variant(PadVariant::Outlined)
    .show(ui, &theme);
```

### Elevated

```demo
let pads = vec![
    PadConfig::new(36).label("Kick".to_string()),
    PadConfig::new(38).label("Snare".to_string()),
    PadConfig::new(42).label("HH".to_string()),
    PadConfig::new(46).label("Tom".to_string()),
];

MidiPad::new()
    .grid(2, 2)
    .pads(pads)
    .variant(PadVariant::Elevated)
    .show(ui, &theme);
```

## Color Schemes

### Semantic Colors (Default)

```demo
let pads = (0..6).map(|i| PadConfig::new(36 + i as u8)).collect();

MidiPad::new()
    .grid(2, 3)
    .pads(pads)
    .color_scheme(PadColorScheme::Semantic)
    .show(ui, &theme);
```

### Monochrome

```demo
let pads = (0..6).map(|i| PadConfig::new(36 + i as u8)).collect();

MidiPad::new()
    .grid(2, 3)
    .pads(pads)
    .color_scheme(PadColorScheme::Monochrome)
    .show(ui, &theme);
```

### Custom Colors

```demo
let pads = vec![
    PadConfig::new(36).label("BD".to_string()).color(egui::Color32::from_rgb(255, 100, 100)),
    PadConfig::new(38).label("SD".to_string()).color(egui::Color32::from_rgb(100, 255, 100)),
    PadConfig::new(42).label("HH".to_string()).color(egui::Color32::from_rgb(100, 100, 255)),
    PadConfig::new(46).label("Tom".to_string()).color(egui::Color32::from_rgb(255, 255, 100)),
];

MidiPad::new()
    .grid(2, 2)
    .pads(pads)
    .color_scheme(PadColorScheme::Custom)
    .show(ui, &theme);
```

## Velocity Display

### With Velocity

```demo
let pads = vec![
    PadConfig::new(36).label("Kick".to_string()),
    PadConfig::new(38).label("Snare".to_string()),
];

let mut pad_states = std::collections::HashMap::new();
pad_states.insert(36, PadState::new(36, 127)); // Full velocity
pad_states.insert(38, PadState::new(38, 64));  // Half velocity

MidiPad::new()
    .grid(1, 2)
    .pads(pads)
    .pad_states(pad_states)
    .show_velocity(true)
    .show(ui, &theme);
```

### Without Velocity

```demo
let pads = vec![
    PadConfig::new(36).label("Kick".to_string()),
    PadConfig::new(38).label("Snare".to_string()),
];

MidiPad::new()
    .grid(1, 2)
    .pads(pads)
    .show_velocity(false)
    .show(ui, &theme);
```

## Glow Effects

```demo
let pads = vec![
    PadConfig::new(36).label("Soft".to_string()),
    PadConfig::new(38).label("Medium".to_string()),
    PadConfig::new(42).label("Hard".to_string()),
];

let mut pad_states = std::collections::HashMap::new();
pad_states.insert(36, PadState::new(36, 40));
pad_states.insert(38, PadState::new(38, 80));
pad_states.insert(42, PadState::new(42, 127));

MidiPad::new()
    .grid(1, 3)
    .pads(pads)
    .pad_states(pad_states)
    .glow_intensity(1.2)
    .show(ui, &theme);
```

## Drum Machine Example

```demo
// GM Drum Map
let drum_pads = vec![
    PadConfig::new(36).label("BD".to_string()),   // Bass Drum
    PadConfig::new(38).label("SD".to_string()),   // Snare
    PadConfig::new(42).label("CHH".to_string()),  // Closed Hi-Hat
    PadConfig::new(46).label("OHH".to_string()),  // Open Hi-Hat
    PadConfig::new(41).label("LT".to_string()),   // Low Tom
    PadConfig::new(43).label("LMT".to_string()),  // Low-Mid Tom
    PadConfig::new(45).label("MT".to_string()),   // Mid Tom
    PadConfig::new(47).label("HMT".to_string()),  // High-Mid Tom
    PadConfig::new(48).label("HT".to_string()),   // High Tom
    PadConfig::new(49).label("Crash".to_string()),
    PadConfig::new(51).label("Ride".to_string()),
    PadConfig::new(37).label("Rim".to_string()),
    PadConfig::new(39).label("Clap".to_string()),
    PadConfig::new(54).label("Tamb".to_string()),
    PadConfig::new(56).label("Cowb".to_string()),
    PadConfig::new(58).label("Vibra".to_string()),
];

let response = MidiPad::new()
    .grid(4, 4)
    .pads(drum_pads)
    .variant(PadVariant::Filled)
    .pad_size(70.0)
    .show(ui, &theme);

if let Some((note, vel)) = response.pressed {
    ui.label(format!("Trigger note {} at velocity {}", note, vel));
}
```

## Handling State

```demo
// Store this in your app state
let mut pad_states: std::collections::HashMap<u8, PadState> = std::collections::HashMap::new();

let pads = vec![
    PadConfig::new(36).label("Kick".to_string()),
    PadConfig::new(38).label("Snare".to_string()),
    PadConfig::new(42).label("HH".to_string()),
    PadConfig::new(46).label("Tom".to_string()),
];

let response = MidiPad::new()
    .grid(2, 2)
    .pads(pads)
    .pad_states(pad_states.clone())
    .show(ui, &theme);

// Update state based on user interaction
if let Some((note, velocity)) = response.pressed {
    pad_states.insert(note, PadState::new(note, velocity));
    // Trigger your audio engine here
}

if let Some(note) = response.released {
    pad_states.remove(&note);
    // Send note off to audio engine
}
```

## API Reference

### Constructor

```rust
MidiPad::new() -> Self
```

Creates a new MIDI pad grid with default 4x4 layout.

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.grid()` | `(usize, usize)` | `(4, 4)` | Grid dimensions (rows, cols) |
| `.pads()` | `Vec<PadConfig>` | Empty | Pad configurations |
| `.pad_states()` | `HashMap<u8, PadState>` | Empty | Current pad states (velocities) |
| `.variant()` | `PadVariant` | `Filled` | Visual variant |
| `.color_scheme()` | `PadColorScheme` | `Semantic` | Color scheme |
| `.pad_size()` | `f32` | `60.0` | Pad size (width & height) in pixels |
| `.gap()` | `f32` | `8.0` | Gap between pads in pixels |
| `.glow_intensity()` | `f32` | `0.8` | Glow intensity (0.0-2.0) |
| `.show_velocity()` | `bool` | `true` | Show velocity as brightness |

### Show Method

```rust
pub fn show(self, ui: &mut egui::Ui) -> MidiPadResponse
```

Returns a `MidiPadResponse` containing user interaction.

### PadConfig

```rust
PadConfig::new(note: u8) -> Self
```

| Method | Type | Description |
|--------|------|-------------|
| `.label()` | `impl Into<String>` | Set pad label text |
| `.color()` | `Color32` | Set custom color |

### PadState

```rust
PadState::new(note: u8, velocity: u8) -> Self
```

| Field | Type | Description |
|-------|------|-------------|
| `note` | `u8` | MIDI note number (0-127) |
| `velocity` | `u8` | Velocity (0-127), 0 = not pressed |

| Method | Return | Description |
|--------|--------|-------------|
| `.is_pressed()` | `bool` | Check if pad is pressed (velocity > 0) |

### PadVariant

```rust
pub enum PadVariant {
    Filled,   // Solid backgrounds
    Outlined, // Transparent with borders
    Elevated, // Shadow effect
}
```

### PadColorScheme

```rust
pub enum PadColorScheme {
    Semantic,   // Cycle through theme colors
    Monochrome, // Single primary color
    Custom,     // Use PadConfig colors
}
```

### MidiPadResponse

| Field | Type | Description |
|-------|------|-------------|
| `pressed` | `Option<(u8, u8)>` | Pad pressed (note, velocity) |
| `released` | `Option<u8>` | Pad released (note) |
| `held` | `Vec<u8>` | Pads currently held |

| Method | Return | Description |
|--------|--------|-------------|
| `.has_press()` | `bool` | Check if any pad was pressed |
| `.has_release()` | `bool` | Check if any pad was released |
| `.has_held()` | `bool` | Check if any pads are held |

## MIDI Note Reference

Common drum mappings (General MIDI standard):

| Note | Name | Note | Name |
|------|------|------|------|
| 36 | Bass Drum | 42 | Closed Hi-Hat |
| 38 | Snare | 46 | Open Hi-Hat |
| 37 | Side Stick | 49 | Crash Cymbal |
| 39 | Hand Clap | 51 | Ride Cymbal |
| 41 | Low Tom | 53 | Ride Bell |
| 43 | Low-Mid Tom | 54 | Tambourine |
| 45 | Mid Tom | 56 | Cowbell |
| 47 | High-Mid Tom | 57 | Crash 2 |
| 48 | High Tom | 58 | Vibraslap |

## Visual Design

### Material Design 3 Styling
- **Filled**: Uses `surface_variant` backgrounds with theme colors
- **Outlined**: Transparent backgrounds with colored borders
- **Elevated**: Shadow layers for depth perception

### Velocity Response
- Brightness increases with velocity (0-127)
- Multi-layer glow effect on pressed pads
- Glow intensity configurable (default 0.8)

### Color Schemes
- **Semantic**: Cycles through error, warning, success, info, primary, secondary
- **Monochrome**: All pads use primary color
- **Custom**: Each pad can have unique color

### Layout
- Square pads with rounded corners (8px radius)
- Configurable gap between pads (default 8px)
- Labels truncated to 6 characters with ellipsis
- Row-major order (left-to-right, top-to-bottom)

## Accessibility

- **Mouse**: Click to press, release to stop
- **Visual Feedback**: Hover highlighting, press glow
- **Labels**: Optional text labels on each pad
- **Velocity**: Brightness indicates hit strength

## Use Cases

### MPC-Style Controller

```demo
let pads: Vec<_> = (0..16).enumerate().map(|(i, _)| {
    PadConfig::new(36 + i as u8)
        .label(format!("P{}", i + 1))
}).collect();

MidiPad::new()
    .grid(4, 4)
    .pads(pads)
    .pad_size(65.0)
    .gap(6.0)
    .variant(PadVariant::Filled)
    .show(ui, &theme);
```

### Finger Drumming

```demo
let pads = vec![
    PadConfig::new(36).label("BD".to_string()).color(egui::Color32::from_rgb(255, 100, 100)),
    PadConfig::new(38).label("SD".to_string()).color(egui::Color32::from_rgb(100, 150, 255)),
];

MidiPad::new()
    .grid(1, 2)
    .pads(pads)
    .pad_size(120.0)
    .glow_intensity(1.5)
    .show(ui, &theme);
```

### Sample Trigger Grid

```demo
let samples = vec![
    "Kick", "Snare", "Clap", "Hat",
    "Perc1", "Perc2", "FX1", "FX2",
];

let pads: Vec<_> = samples.iter().enumerate().map(|(i, name)| {
    PadConfig::new(36 + i as u8).label(name.to_string())
}).collect();

MidiPad::new()
    .grid(2, 4)
    .pads(pads)
    .variant(PadVariant::Elevated)
    .show(ui, &theme);
```

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `secondary`, `error`, `warning`, `success`, `info`, `surface`, `surface_variant`, `outline`, `outline_variant`
- Standard library: `HashMap` for state management
- Minimum version: `armas 0.1.0`

## Related Components

- **Piano**: Chromatic keyboard for melodic input
- **PianoRoll**: Complete piano roll editor with note editing
- **Knob**: Rotary control for continuous parameters
- **Fader**: Linear fader for volume/pan controls
