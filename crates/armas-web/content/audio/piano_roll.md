# Piano Roll

Complete DAW-style piano roll editor with vertical piano keyboard, grid, and interactive note blocks. Drag to paint notes, click notes to remove.

## Basic Usage

```demo
let theme = ui.ctx().armas_theme();
let mut notes = vec![];

let response = PianoRoll::new()
    .notes(notes.clone())
    .show(ui, &theme);

notes = response.notes;

if response.modified {
    ui.label(format!("Total notes: {}", notes.len()));
}
```

## With Initial Notes

```demo
let theme = ui.ctx().armas_theme();
let mut notes = vec![
    Note::new(60, 0.0, 1.0),   // C4 at beat 0
    Note::new(64, 1.0, 1.0),   // E4 at beat 1
    Note::new(67, 2.0, 1.0),   // G4 at beat 2
    Note::new(72, 3.0, 1.0),   // C5 at beat 3
];

let response = PianoRoll::new()
    .notes(notes.clone())
    .show(ui, &theme);

notes = response.notes;
```

## Custom Range

```demo
let theme = ui.ctx().armas_theme();
let mut notes = vec![];

let response = PianoRoll::new()
    .start_note(48)  // C3
    .octaves(3)
    .notes(notes.clone())
    .show(ui, &theme);

notes = response.notes;
```

## More Measures

```demo
let theme = ui.ctx().armas_theme();
let mut notes = vec![];

let response = PianoRoll::new()
    .measures(8)
    .notes(notes.clone())
    .show(ui, &theme);

notes = response.notes;
```

## Different Grid Divisions

### Eighth Notes

```demo
let theme = ui.ctx().armas_theme();
let mut notes = vec![];

let response = PianoRoll::new()
    .division(GridDivision::Eighth)
    .default_note_duration(0.5)
    .notes(notes.clone())
    .show(ui, &theme);

notes = response.notes;
```

### Sixteenth Notes

```demo
let theme = ui.ctx().armas_theme();
let mut notes = vec![];

let response = PianoRoll::new()
    .division(GridDivision::Sixteenth)
    .default_note_duration(0.25)
    .notes(notes.clone())
    .show(ui, &theme);

notes = response.notes;
```

## Custom Dimensions

```demo
let theme = ui.ctx().armas_theme();
let mut notes = vec![];

let response = PianoRoll::new()
    .white_key_width(50.0)
    .white_key_height(140.0)
    .beat_width(80.0)
    .notes(notes.clone())
    .show(ui, &theme);

notes = response.notes;
```

## Note with Velocity

```demo
let theme = ui.ctx().armas_theme();
let mut notes = vec![
    Note::with_velocity(60, 0.0, 1.0, 0.3),  // Soft
    Note::with_velocity(64, 1.0, 1.0, 0.6),  // Medium
    Note::with_velocity(67, 2.0, 1.0, 1.0),  // Loud
];

let response = PianoRoll::new()
    .notes(notes.clone())
    .show(ui, &theme);

notes = response.notes;
```

## Detecting Changes

```demo
let theme = ui.ctx().armas_theme();
let mut notes = vec![];

let response = PianoRoll::new()
    .notes(notes.clone())
    .show(ui, &theme);

notes = response.notes;

if !response.added_notes.is_empty() {
    ui.label(format!("Added {} notes", response.added_notes.len()));
}

if !response.removed_notes.is_empty() {
    ui.label(format!("Removed {} notes", response.removed_notes.len()));
}
```

## Read-Only Mode

```demo
let theme = ui.ctx().armas_theme();
let notes = vec![
    Note::new(60, 0.0, 1.0),
    Note::new(64, 1.0, 1.0),
    Note::new(67, 2.0, 1.0),
];

PianoRoll::new()
    .notes(notes)
    .editable(false)
    .show(ui, &theme);
```

## Without Piano

```demo
let theme = ui.ctx().armas_theme();
let mut notes = vec![];

let response = PianoRoll::new()
    .show_piano(false)
    .notes(notes.clone())
    .show(ui, &theme);

notes = response.notes;
```

## API Reference

### Constructor

```rust
PianoRoll::new() -> Self
```

Creates a new piano roll starting at C4 (MIDI 60) with 2 octaves and 4 measures.

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.start_note()` | `u8` | `60` (C4) | Starting MIDI note (0-127) |
| `.octaves()` | `u8` | `2` | Number of octaves to display |
| `.white_key_width()` | `f32` | `40.0` | Width of white keys in pixels |
| `.white_key_height()` | `f32` | `120.0` | Height of white keys in pixels |
| `.measures()` | `u32` | `4` | Number of measures to display |
| `.division()` | `GridDivision` | `Quarter` | Grid division for snapping |
| `.beat_width()` | `f32` | `50.0` | Width per beat in pixels |
| `.default_note_duration()` | `f32` | `1.0` | Default note duration when placing (in beats) |
| `.notes()` | `Vec<Note>` | Empty | Notes to display and edit |
| `.show_grid()` | `bool` | `true` | Show grid background |
| `.show_piano()` | `bool` | `true` | Show vertical piano keyboard |
| `.note_opacity()` | `f32` | `0.85` | Opacity of note blocks (0.0-1.0) |
| `.editable()` | `bool` | `true` | Enable note editing |

### Show Method

```rust
pub fn show(self, ui: &mut egui::Ui, theme: &Theme) -> PianoRollResponse
```

Returns a `PianoRollResponse` with updated notes and modification status.

### Note Structure

```rust
pub struct Note {
    pub note: u8,          // MIDI note number (0-127)
    pub start_beat: f32,   // Start position in beats
    pub duration: f32,     // Duration in beats
    pub velocity: f32,     // Velocity (0.0-1.0)
}

// Constructors
Note::new(note: u8, start_beat: f32, duration: f32) -> Self
Note::with_velocity(note: u8, start_beat: f32, duration: f32, velocity: f32) -> Self
```

### PianoRollResponse

| Field | Type | Description |
|-------|------|-------------|
| `notes` | `Vec<Note>` | All notes after user interaction |
| `modified` | `bool` | Whether notes were modified this frame |
| `added_notes` | `Vec<Note>` | Newly added notes |
| `removed_notes` | `Vec<Note>` | Removed notes |

## Interactions

### Placing Notes
- **Drag across grid**: Paint multiple notes by dragging
- **Click and drag**: Start placing notes and continue dragging
- **Click piano key**: Places a note at beat 0 for that key
- Notes snap to the grid division automatically

### Removing Notes
- **Click existing note**: Removes the note instantly

### Visual Feedback
- **Hover over empty cell**: Shows ghost preview of note placement
- **Note blocks**: Glassmorphic style with velocity-based opacity
- **Grid snapping**: Notes automatically snap to grid divisions
- **Drag painting**: Smoothly paint notes across multiple cells

## Visual Design

### Note Blocks
- **Glassmorphic style**: Semi-transparent with gradient
- **Rounded corners**: 4px radius for modern look
- **Velocity mapping**: Opacity reflects note velocity (0.0-1.0)
- **Primary color**: Uses theme's primary color
- **Highlight**: Subtle top highlight for glass effect
- **Border**: 1px border for definition

### Grid Integration
- **Alternating rows**: Subtle background stripes for readability
- **Theme-aware**: Black lines on light theme, white on dark
- **Measure emphasis**: Thicker lines at measure boundaries
- **Beat emphasis**: Medium lines at beat boundaries

### Piano Integration
- **Vertical layout**: Piano on left, grid on right
- **Aligned rows**: Grid rows align perfectly with piano keys
- **Interactive**: Click piano keys to place notes

## Use Cases

### DAW Sequencer

```demo
let theme = ui.ctx().armas_theme();
let mut notes = vec![
    Note::new(60, 0.0, 0.5),
    Note::new(64, 0.5, 0.5),
    Note::new(67, 1.0, 0.5),
    Note::new(72, 1.5, 0.5),
];

let response = PianoRoll::new()
    .start_note(48)
    .octaves(3)
    .measures(4)
    .division(GridDivision::Sixteenth)
    .notes(notes.clone())
    .show(ui, &theme);

notes = response.notes;

if response.modified {
    // Trigger audio engine update
}
```

### Music Education Tool

```demo
let theme = ui.ctx().armas_theme();
let notes = vec![
    Note::new(60, 0.0, 1.0),  // C
    Note::new(64, 1.0, 1.0),  // E
    Note::new(67, 2.0, 1.0),  // G
];

PianoRoll::new()
    .octaves(1)
    .measures(4)
    .beat_width(100.0)
    .notes(notes)
    .editable(false)
    .show(ui, &theme);
```

### Step Sequencer

```demo
let theme = ui.ctx().armas_theme();
let mut notes = vec![];

let response = PianoRoll::new()
    .octaves(1)
    .measures(2)
    .division(GridDivision::Sixteenth)
    .default_note_duration(0.25)
    .beat_width(60.0)
    .notes(notes.clone())
    .show(ui, &theme);

notes = response.notes;
```

## Performance

- **Efficient rendering**: Only visible notes are drawn
- **Click detection**: Fast spatial lookup for note selection
- **Smooth interactions**: No lag when placing/removing notes
- **Scalable**: Handles hundreds of notes without performance issues

## Integration Example

Complete example with playback and state management:

```rust
use armas::{PianoRoll, Note, GridDivision};
use std::collections::HashMap;

struct Sequencer {
    notes: Vec<Note>,
    playing: bool,
    current_beat: f32,
}

impl Sequencer {
    fn update(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        // Piano roll editor
        let response = PianoRoll::new()
            .start_note(36)
            .octaves(4)
            .measures(8)
            .division(GridDivision::Sixteenth)
            .notes(self.notes.clone())
            .show(ui, theme);

        // Update notes if modified
        if response.modified {
            self.notes = response.notes;

            // Trigger audio engine update
            for note in &response.added_notes {
                self.schedule_note(note);
            }
        }

        // Playback controls
        ui.horizontal(|ui| {
            if ui.button(if self.playing { "Stop" } else { "Play" }).clicked() {
                self.playing = !self.playing;
            }
        });
    }

    fn schedule_note(&self, note: &Note) {
        // Send to audio engine
    }
}
```

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `background`, `outline`
- Components: `Piano`, `PianoRollGrid`
- Minimum version: `armas 0.1.0`
