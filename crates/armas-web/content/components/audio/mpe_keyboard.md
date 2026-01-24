# MPE Keyboard

Interactive piano keyboard with MPE (MIDI Polyphonic Expression) support. Uses JUCE-style floating circle visualization to show per-note pitch bend, pressure (aftertouch), and slide - perfect for expressive MIDI controllers like Roli Seaboard, Linnstrument, or Sensel Morph.

## Basic Usage

```demo
let theme = ui.ctx().armas_theme();
MPEKeyboard::new()
    .show(ui, &theme);
```

## With Active MPE Notes

Show notes with various expression parameters visualized as floating circles:

```demo
let theme = ui.ctx().armas_theme();
let mut notes = std::collections::HashMap::new();

// C4 with pressure (larger outer circle)
notes.insert(60, MPENote::new(60).pressure(0.7));
// E4 with pitch bend up (circle shifts right)
notes.insert(64, MPENote::with_velocity(64, 0.9).pitch_bend(2.0));
// G4 with slide (circle moves up)
notes.insert(67, MPENote::new(67).slide(0.8));

MPEKeyboard::new()
    .active_notes(notes)
    .show(ui, &theme);
```

## Circle Visualization Explained

The JUCE-style circle visualization encodes all 4 MPE dimensions:

- **Inner filled circle size** = velocity (how hard the key was struck)
- **Outer circle outline size** = pressure (aftertouch intensity)
- **Circle X position** = pitch bend (horizontal movement)
- **Circle Y position** = slide (vertical position on the key)

```demo
let theme = ui.ctx().armas_theme();
let mut notes = std::collections::HashMap::new();

// High velocity, high pressure (large circles)
notes.insert(60, MPENote::with_velocity(60, 0.9).pressure(0.8));
// Low velocity, low pressure (small circles)
notes.insert(64, MPENote::with_velocity(64, 0.3).pressure(0.1));
// Medium velocity, high pressure (medium inner, large outer)
notes.insert(67, MPENote::with_velocity(67, 0.5).pressure(0.9));

MPEKeyboard::new()
    .active_notes(notes)
    .show(ui, &theme);
```

## Pitch Bend Visualization

Circle shifts horizontally based on pitch bend amount:

```demo
let theme = ui.ctx().armas_theme();
let mut notes = std::collections::HashMap::new();

// Bend up (circle shifts right)
notes.insert(60, MPENote::new(60).pitch_bend(6.0));  // +6 semitones
// Bend down (circle shifts left)
notes.insert(64, MPENote::new(64).pitch_bend(-6.0)); // -6 semitones
// No bend (circle centered)
notes.insert(67, MPENote::new(67));

MPEKeyboard::new()
    .active_notes(notes)
    .show(ui, &theme);
```

## Slide Visualization

Circle moves vertically based on slide position:

```demo
let theme = ui.ctx().armas_theme();
let mut notes = std::collections::HashMap::new();

notes.insert(60, MPENote::new(60).slide(0.0));   // Bottom of key
notes.insert(62, MPENote::new(62).slide(0.25));  // Lower
notes.insert(64, MPENote::new(64).slide(0.5));   // Center (default)
notes.insert(65, MPENote::new(65).slide(0.75));  // Upper
notes.insert(67, MPENote::new(67).slide(1.0));   // Top of key

MPEKeyboard::new()
    .active_notes(notes)
    .show(ui, &theme);
```

## Combined Expression

All MPE parameters working together:

```demo
let theme = ui.ctx().armas_theme();
let mut notes = std::collections::HashMap::new();

// Expressive chord with different parameters per note
notes.insert(60, MPENote::with_velocity(60, 0.9)
    .pressure(0.6)
    .pitch_bend(-0.5)
    .slide(0.3));
notes.insert(64, MPENote::with_velocity(64, 0.7)
    .pressure(0.8)
    .pitch_bend(1.0)
    .slide(0.7));
notes.insert(67, MPENote::with_velocity(67, 0.85)
    .pressure(0.4)
    .pitch_bend(0.0)
    .slide(0.5));

MPEKeyboard::new()
    .octaves(2)
    .active_notes(notes)
    .show(ui, &theme);
```

## Custom Range

```demo
let theme = ui.ctx().armas_theme();
MPEKeyboard::new()
    .start_note(48)  // C3
    .octaves(3)
    .show(ui, &theme);
```

## Orientations

### Horizontal (Default)

```demo
let theme = ui.ctx().armas_theme();
MPEKeyboard::new()
    .orientation(MPEOrientation::Horizontal)
    .show(ui, &theme);
```

### Horizontal Facing Up

```demo
let theme = ui.ctx().armas_theme();
MPEKeyboard::new()
    .orientation(MPEOrientation::HorizontalUp)
    .show(ui, &theme);
```

### Vertical

```demo
let theme = ui.ctx().armas_theme();
MPEKeyboard::new()
    .orientation(MPEOrientation::Vertical)
    .show(ui, &theme);
```

## Custom Circle Colors

```demo
let theme = ui.ctx().armas_theme();
let mut notes = std::collections::HashMap::new();
notes.insert(60, MPENote::new(60).pressure(0.5));
notes.insert(64, MPENote::new(64).pressure(0.7));

MPEKeyboard::new()
    .active_notes(notes)
    .circle_fill_color(egui::Color32::from_rgb(255, 100, 50))
    .circle_outline_color(egui::Color32::from_rgb(255, 200, 100))
    .show(ui, &theme);
```

## Glass Styling

```demo
let theme = ui.ctx().armas_theme();
let mut notes = std::collections::HashMap::new();
notes.insert(60, MPENote::new(60).pressure(0.5));
notes.insert(64, MPENote::new(64).pressure(0.7));

MPEKeyboard::new()
    .white_key_opacity(0.8)
    .black_key_opacity(0.9)
    .active_notes(notes)
    .show(ui, &theme);
```

## API Reference

### Constructor

```rust
MPEKeyboard::new() -> Self
```

Creates a new MPE keyboard starting at middle C (MIDI note 60) with 2 octaves.

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.start_note()` | `u8` | `60` (C4) | Starting MIDI note (0-127) |
| `.octaves()` | `u8` | `2` | Number of octaves to display |
| `.white_key_width()` | `f32` | `40.0` | Width of white keys in pixels |
| `.white_key_height()` | `f32` | `120.0` | Height of white keys in pixels |
| `.white_key_opacity()` | `f32` | `0.7` | Glass opacity for white keys (0.0-1.0) |
| `.black_key_opacity()` | `f32` | `0.85` | Glass opacity for black keys (0.0-1.0) |
| `.show_labels()` | `bool` | `true` | Show note labels on keys |
| `.orientation()` | `MPEOrientation` | `Horizontal` | Keyboard orientation |
| `.active_notes()` | `HashMap<u8, MPENote>` | Empty | Map of active MPE notes |
| `.with_note()` | `MPENote` | - | Add a single active note |
| `.pitch_bend_range()` | `f32` | `48.0` | Pitch bend range in semitones for scaling |
| `.circle_fill_color()` | `Color32` | Theme primary | Fill color for velocity circles |
| `.circle_outline_color()` | `Color32` | Theme secondary | Outline color for pressure circles |
| `.scrollable()` | `f32` | - | Enable scrolling with viewport size |
| `.momentum_scrolling()` | `bool` | `true` | Enable momentum physics when scrolling |
| `.momentum_damping()` | `f64` | `5.0` | Damping factor for momentum (1.0-20.0) |

### MPENote

Per-note expression data:

```rust
MPENote::new(note: u8) -> Self
MPENote::with_velocity(note: u8, velocity: f32) -> Self

// Builder methods
.pressure(f32)    // 0.0-1.0, aftertouch
.pitch_bend(f32)  // Semitones (e.g., -48.0 to +48.0)
.slide(f32)       // 0.0-1.0, vertical position
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `note` | `u8` | Required | MIDI note number (0-127) |
| `velocity` | `f32` | `0.8` | Initial strike velocity (0.0-1.0) |
| `pressure` | `f32` | `0.0` | Aftertouch/pressure (0.0-1.0) |
| `pitch_bend` | `f32` | `0.0` | Pitch bend in semitones |
| `slide` | `f32` | `0.5` | Slide position (0.0=bottom, 1.0=top) |

### MPEOrientation

```rust
pub enum MPEOrientation {
    Horizontal,      // Keys left-to-right, facing down
    HorizontalUp,    // Keys left-to-right, facing up
    Vertical,        // Keys bottom-to-top, facing right
    VerticalLeft,    // Keys bottom-to-top, facing left
}
```

### MPEKeyboardResponse

| Field | Type | Description |
|-------|------|-------------|
| `clicked_keys` | `Vec<u8>` | MIDI notes clicked this frame |
| `released_keys` | `Vec<u8>` | MIDI notes released this frame |

## Visual Design

### JUCE-Style Circle Visualization

Each active note displays floating concentric circles that encode all MPE dimensions:

| Dimension | Visual Encoding |
|-----------|----------------|
| **Velocity** | Inner filled circle size |
| **Pressure** | Outer circle outline size |
| **Pitch Bend** | Circle horizontal position |
| **Slide** | Circle vertical position |

### Glass Effect

Same styling as the standard Piano component:
- **White keys**: Pure white with controlled opacity
- **Black keys**: Dark with higher opacity
- **Active keys**: Circle overlay shows expression
- **Hover state**: Slightly increased opacity

## MPE vs Standard MIDI

| Feature | Standard MIDI | MPE |
|---------|---------------|-----|
| Pitch Bend | Channel-wide | Per-note |
| Aftertouch | Channel-wide | Per-note |
| Slide/CC74 | Channel-wide | Per-note |
| Channels Used | 1 | 1 (global) + 1 per note |

## Use Cases

### Roli Seaboard Visualizer

```demo
let theme = ui.ctx().armas_theme();
let mut notes = std::collections::HashMap::new();

// Simulate expressive playing
notes.insert(60, MPENote::with_velocity(60, 0.8)
    .pressure(0.6)
    .pitch_bend(0.3)
    .slide(0.4));
notes.insert(64, MPENote::with_velocity(64, 0.7)
    .pressure(0.4)
    .pitch_bend(-0.2)
    .slide(0.6));

MPEKeyboard::new()
    .octaves(2)
    .active_notes(notes)
    .show(ui, &theme);
```

### MPE Synth Interface

```demo
let theme = ui.ctx().armas_theme();
let response = MPEKeyboard::new()
    .octaves(3)
    .start_note(48)
    .show(ui, &theme);

if !response.clicked_keys.is_empty() {
    ui.label(format!("Note on: {:?}", response.clicked_keys));
}
```

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `background`, `primary`, `secondary`, `border`
- Standard library: `HashMap` for note tracking
- Minimum version: `armas-audio 0.1.0`
