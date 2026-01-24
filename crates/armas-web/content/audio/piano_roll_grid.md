# Piano Roll Grid

Background grid for piano roll editors with horizontal lines aligned to piano keys and vertical lines for time divisions.

## Basic Usage

```demo
let theme = ui.ctx().armas_theme();
PianoRollGrid::new()
    .show(ui, &theme);
```

## Custom Note Range

```demo
let theme = ui.ctx().armas_theme();
PianoRollGrid::new()
    .start_note(48)  // C3
    .octaves(3)
    .show(ui, &theme);
```

## Time Divisions

### Quarter Notes (Default)

```demo
let theme = ui.ctx().armas_theme();
PianoRollGrid::new()
    .measures(4)
    .division(GridDivision::Quarter)
    .show(ui, &theme);
```

### Eighth Notes

```demo
let theme = ui.ctx().armas_theme();
PianoRollGrid::new()
    .measures(4)
    .division(GridDivision::Eighth)
    .show(ui, &theme);
```

### Sixteenth Notes

```demo
let theme = ui.ctx().armas_theme();
PianoRollGrid::new()
    .measures(4)
    .division(GridDivision::Sixteenth)
    .show(ui, &theme);
```

### Half Notes

```demo
let theme = ui.ctx().armas_theme();
PianoRollGrid::new()
    .measures(4)
    .division(GridDivision::Half)
    .show(ui, &theme);
```

### Whole Notes

```demo
let theme = ui.ctx().armas_theme();
PianoRollGrid::new()
    .measures(4)
    .division(GridDivision::Whole)
    .show(ui, &theme);
```

## Custom Dimensions

Match the piano component's dimensions:

```demo
let theme = ui.ctx().armas_theme();
PianoRollGrid::new()
    .white_key_width(50.0)
    .white_key_height(140.0)
    .show(ui, &theme);
```

## Multiple Measures

```demo
let theme = ui.ctx().armas_theme();
PianoRollGrid::new()
    .measures(8)
    .division(GridDivision::Quarter)
    .show(ui, &theme);
```

## Custom Beat Width

```demo
let theme = ui.ctx().armas_theme();
PianoRollGrid::new()
    .beat_width(80.0)
    .measures(4)
    .division(GridDivision::Quarter)
    .show(ui, &theme);
```

## Without Measure Numbers

```demo
let theme = ui.ctx().armas_theme();
PianoRollGrid::new()
    .show_measure_numbers(false)
    .measures(4)
    .show(ui, &theme);
```

## Custom Opacity

```demo
let theme = ui.ctx().armas_theme();
PianoRollGrid::new()
    .line_opacity(0.3)
    .measures(4)
    .show(ui, &theme);
```

## Without Beat Emphasis

```demo
let theme = ui.ctx().armas_theme();
PianoRollGrid::new()
    .emphasize_beats(false)
    .measures(4)
    .show(ui, &theme);
```

## Combined with Piano

Complete piano roll setup (vertical piano on left, grid on right):

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
PianoRollGrid::new() -> Self
```

Creates a new piano roll grid starting at C4 (MIDI 60) with 2 octaves, 4 measures, and quarter note divisions.

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.start_note()` | `u8` | `60` (C4) | Starting MIDI note (0-127) |
| `.octaves()` | `u8` | `2` | Number of octaves to display |
| `.white_key_width()` | `f32` | `40.0` | Width of white keys (should match piano) |
| `.white_key_height()` | `f32` | `120.0` | Height of white keys (should match piano) |
| `.black_key_height_ratio()` | `f32` | `0.6` | Black key height as ratio of white keys |
| `.measures()` | `u32` | `4` | Number of measures to display |
| `.division()` | `GridDivision` | `Quarter` | Time division for vertical lines |
| `.beat_width()` | `f32` | `50.0` | Width per beat in pixels |
| `.show_measure_numbers()` | `bool` | `true` | Show measure numbers at top |
| `.line_opacity()` | `f32` | `0.15` | Grid line opacity (0.0-1.0) |
| `.emphasize_beats()` | `bool` | `true` | Emphasize beat and measure lines |

### Show Method

```rust
pub fn show(self, ui: &mut egui::Ui, theme: &Theme) -> Response
```

Returns an `egui::Response` for the grid area.

### GridDivision

```rust
pub enum GridDivision {
    Whole,      // 4 beats (whole note)
    Half,       // 2 beats (half note)
    Quarter,    // 1 beat (quarter note)
    Eighth,     // 1/2 beat (eighth note)
    Sixteenth,  // 1/4 beat (sixteenth note)
}
```

Each division determines the spacing of vertical grid lines.

## Visual Design

### Horizontal Lines
- Aligned with white piano key boundaries
- One line per white key position
- Subtle opacity for non-intrusive background
- Helps visualize note pitch levels

### Vertical Lines
- Time divisions based on `GridDivision` setting
- Three levels of emphasis:
  - **Measure lines**: Boldest (2.0px, 2.5x opacity)
  - **Beat lines**: Medium (1.5px, 1.8x opacity)
  - **Division lines**: Subtle (1.0px, 1.0x opacity)

### Measure Numbers
- Displayed at top-left of each measure
- Small font (10pt) in `on_surface_variant` color
- Optional, can be hidden

### Background
- Semi-transparent surface color (30% opacity)
- Provides subtle separation from main canvas

## Grid Alignment

When combining with Piano component, ensure dimensions match:

```rust
// Keep these values synchronized
let start = 60;
let octaves = 2;
let key_width = 40.0;
let key_height = 120.0;

ui.horizontal(|ui| {
    // Vertical piano on the left
    Piano::new()
        .start_note(start)
        .octaves(octaves)
        .white_key_width(key_width)
        .white_key_height(key_height)
        .orientation(PianoOrientation::Vertical)
        .show(ui, &theme);

    // Grid on the right
    PianoRollGrid::new()
        .start_note(start)
        .octaves(octaves)
        .white_key_width(key_width)
        .white_key_height(key_height)
        .show(ui, &theme);
});
```

## Time Signature

The grid assumes **4/4 time signature** (4 beats per measure). Each measure contains:
- 4 beats (quarter notes)
- 8 eighth notes
- 16 sixteenth notes
- 2 half notes
- 1 whole note

The `beat_width` parameter controls horizontal spacing. At default 50.0px:
- Each measure: 200px (4 beats × 50px)
- 4 measures: 800px total width

## Use Cases

### DAW Piano Roll Background

```demo
let theme = ui.ctx().armas_theme();
PianoRollGrid::new()
    .start_note(36)  // C2
    .octaves(5)      // 5 octaves
    .measures(8)     // 8 measures
    .division(GridDivision::Sixteenth)
    .beat_width(60.0)
    .show(ui, &theme);
```

### Simple Step Sequencer

```demo
let theme = ui.ctx().armas_theme();
PianoRollGrid::new()
    .octaves(1)
    .measures(4)
    .division(GridDivision::Eighth)
    .beat_width(80.0)
    .show(ui, &theme);
```

### Score Notation Guide

```demo
let theme = ui.ctx().armas_theme();
PianoRollGrid::new()
    .measures(4)
    .division(GridDivision::Quarter)
    .emphasize_beats(true)
    .line_opacity(0.2)
    .show(ui, &theme);
```

## Performance

- Efficient rendering using painter primitives
- Only visible grid lines are drawn
- No animation or continuous repaint required
- Suitable for real-time DAW applications

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `outline`, `on_surface_variant`
- Minimum version: `armas 0.1.0`
