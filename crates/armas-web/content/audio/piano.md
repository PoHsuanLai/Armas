# Piano

Interactive piano keyboard with glassmorphic styling. Perfect for DAW piano rolls and music applications.

## Basic Usage

```demo
let theme = ui.ctx().armas_theme();
Piano::new()
    .show(ui, &theme);
```

## Custom Range

```demo
let theme = ui.ctx().armas_theme();
Piano::new()
    .start_note(48)  // C3
    .octaves(3)
    .show(ui, &theme);
```

## Custom Sizing

```demo
let theme = ui.ctx().armas_theme();
Piano::new()
    .white_key_width(50.0)
    .white_key_height(140.0)
    .show(ui, &theme);
```

## Orientations

### Horizontal (Default - Facing Down)

```demo
let theme = ui.ctx().armas_theme();
Piano::new()
    .orientation(PianoOrientation::Horizontal)
    .show(ui, &theme);
```

### Horizontal Facing Up

```demo
let theme = ui.ctx().armas_theme();
Piano::new()
    .orientation(PianoOrientation::HorizontalUp)
    .show(ui, &theme);
```

### Vertical (Facing Right)

```demo
let theme = ui.ctx().armas_theme();
Piano::new()
    .orientation(PianoOrientation::Vertical)
    .show(ui, &theme);
```

### Vertical Facing Left

```demo
let theme = ui.ctx().armas_theme();
Piano::new()
    .orientation(PianoOrientation::VerticalLeft)
    .show(ui, &theme);
```

## Glass Styling

```demo
let theme = ui.ctx().armas_theme();
Piano::new()
    .white_key_opacity(0.8)
    .black_key_opacity(0.9)
    .glow_intensity(1.2)
    .show(ui, &theme);
```

## With Active Keys

```demo
let theme = ui.ctx().armas_theme();
let mut pressed_keys = std::collections::HashSet::new();
pressed_keys.insert(60); // C4
pressed_keys.insert(64); // E4
pressed_keys.insert(67); // G4

Piano::new()
    .pressed_keys(pressed_keys)
    .show(ui, &theme);
```

## Handling Interactions

```demo
let theme = ui.ctx().armas_theme();
let response = Piano::new().show(ui, &theme);

if !response.clicked_keys.is_empty() {
    ui.label(format!("Clicked {} keys", response.clicked_keys.len()));
}

if !response.released_keys.is_empty() {
    ui.label(format!("Released {} keys", response.released_keys.len()));
}
```

## With Piano Roll Grid

Combine with PianoRollGrid for complete DAW piano roll (vertical piano on left, grid on right):

```demo
let theme = ui.ctx().armas_theme();

ui.horizontal(|ui| {
    // Vertical piano on the left
    Piano::new()
        .start_note(60)
        .octaves(2)
        .white_key_width(40.0)
        .white_key_height(120.0)
        .orientation(PianoOrientation::Vertical)
        .show(ui, &theme);

    // Grid extends to the right
    PianoRollGrid::new()
        .start_note(60)
        .octaves(2)
        .white_key_width(40.0)
        .white_key_height(120.0)
        .measures(4)
        .division(GridDivision::Quarter)
        .show(ui, &theme);
});
```

## API Reference

### Constructor

```rust
Piano::new() -> Self
```

Creates a new piano starting at middle C (MIDI note 60) with 2 octaves.

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.start_note()` | `u8` | `60` (C4) | Starting MIDI note (0-127) |
| `.octaves()` | `u8` | `2` | Number of octaves to display |
| `.white_key_width()` | `f32` | `40.0` | Width of white keys in pixels |
| `.white_key_height()` | `f32` | `120.0` | Height of white keys in pixels |
| `.white_key_opacity()` | `f32` | `0.7` | Glass opacity for white keys (0.0-1.0) |
| `.black_key_opacity()` | `f32` | `0.85` | Glass opacity for black keys (0.0-1.0) |
| `.glow_intensity()` | `f32` | `0.8` | Glow intensity for pressed keys (0.0-2.0) |
| `.show_labels()` | `bool` | `true` | Show note labels on keys |
| `.orientation()` | `PianoOrientation` | `Horizontal` | Piano orientation |
| `.pressed_keys()` | `HashSet<u8>` | Empty | Set of currently pressed MIDI notes |

### Show Method

```rust
pub fn show(self, ui: &mut egui::Ui, theme: &Theme) -> PianoResponse
```

Returns a `PianoResponse` containing clicked and released keys.

### PianoOrientation

```rust
pub enum PianoOrientation {
    Horizontal,      // Keys left-to-right, facing down
    HorizontalUp,    // Keys left-to-right, facing up
    Vertical,        // Keys bottom-to-top, facing right
    VerticalLeft,    // Keys bottom-to-top, facing left
}
```

### PianoResponse

| Field | Type | Description |
|-------|------|-------------|
| `clicked_keys` | `Vec<u8>` | MIDI notes clicked this frame |
| `released_keys` | `Vec<u8>` | MIDI notes released this frame |

### PianoKey Helper

```rust
PianoKey::new(note: u8, is_black: bool) -> Self
PianoKey::note_name(&self) -> String  // Returns "C4", "C#4", etc.
PianoKey::is_black_key(note: u8) -> bool
```

## MIDI Note Reference

The piano uses standard MIDI note numbering:
- **Middle C (C4)** = 60
- **A4 (440 Hz)** = 69
- **Note formula**: `note = octave * 12 + semitone`
- **Octave formula**: `octave = (note / 12) - 1`

**Black keys** (sharps/flats): C#, D#, F#, G#, A# (modulo 12: 1, 3, 6, 8, 10)

## Visual Design

### Glass Effect
- **White keys**: Pure white (255,255,255) with controlled opacity
- **Black keys**: Dark (20,20,20) with higher opacity
- **Pressed keys**: Reduced opacity with subtle multi-layer glow
- **Hover state**: Slightly increased opacity
- **Labels**: Dark gray text (60,60,60) independent of theme

### Layout
- Black keys are 60% width and height of white keys
- Black keys positioned centered between white keys
- Rounded corners on the direction keys face
- Shimmer effect on glass surface

## Accessibility

- **Mouse**: Click to press keys, release to stop
- **Visual Feedback**: Glow effect on pressed keys, hover highlighting
- **Labels**: Optional note name labels (C4, D#4, etc.) on each key
- **Performance**: Optimized rendering with continuous repaint during interaction

## Use Cases

### DAW Piano Roll

```demo
let theme = ui.ctx().armas_theme();
let mut active_notes = std::collections::HashSet::new();
// Simulate some active notes
active_notes.insert(60);
active_notes.insert(64);

let response = Piano::new()
    .start_note(36)  // C2
    .octaves(3)      // 3 octaves
    .pressed_keys(active_notes.clone())
    .show(ui, &theme);

if !response.clicked_keys.is_empty() {
    ui.label("Notes triggered!");
}
```

### MIDI Visualizer

```demo
let theme = ui.ctx().armas_theme();
Piano::new()
    .glow_intensity(1.5)  // Brighter for visualization
    .show_labels(false)   // Cleaner look
    .show(ui, &theme);
```

### Music Education

```demo
let theme = ui.ctx().armas_theme();
Piano::new()
    .octaves(1)
    .white_key_width(60.0)  // Larger for learning
    .white_key_height(160.0)
    .show_labels(true)      // Show note names
    .show(ui, &theme);
```

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `background`, `primary`, `outline`, `outline_variant`
- Standard library: `HashSet` for key tracking
- Minimum version: `armas 0.1.0`
