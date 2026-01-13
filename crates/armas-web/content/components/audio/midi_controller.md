# MIDI Controller

Complete MIDI controller interface combining piano keyboard, mod/pitch wheels, XY pad, drum pads, and step sequencer into a unified surface.

## Basic Usage

```demo
use armas::components::audio::{MidiController, MidiControllerState};

let mut state = MidiControllerState::default();

let response = MidiController::new(&mut state)
    .show(ui);

// Handle MIDI events
if let Some(piano_response) = response.piano {
    for note in piano_response.clicked_keys {
        println!("Piano note pressed: {}", note);
    }
}

if response.mod_wheel_changed {
    println!("Modulation: {:.2}", state.mod_wheel);
}
```

## Layouts

### Full Layout (Default)

Complete controller with all sections visible.

```demo
use armas::components::audio::{MidiController, MidiControllerState, ControllerLayout};

let mut state = MidiControllerState::default();

MidiController::new(&mut state)
    .layout(ControllerLayout::Full)
    .show(ui);
```

### Compact Layout

Essential controls only - keyboard and wheels.

```demo
use armas::components::audio::{MidiController, MidiControllerState, ControllerLayout};

let mut state = MidiControllerState::default();

MidiController::new(&mut state)
    .layout(ControllerLayout::Compact)
    .show(ui);
```

### Performance Layout

Optimized for live performance - keyboard, wheels, XY pad, and drum pads.

```demo
use armas::components::audio::{MidiController, MidiControllerState, ControllerLayout};

let mut state = MidiControllerState::default();

MidiController::new(&mut state)
    .layout(ControllerLayout::Performance)
    .show(ui);
```

## Custom Sections

Control which sections are visible manually.

```demo
use armas::components::audio::{MidiController, MidiControllerState, ControllerSections};

let mut state = MidiControllerState::default();

MidiController::new(&mut state)
    .sections(ControllerSections {
        show_piano: true,
        show_wheels: true,
        show_xy_pad: false,
        show_drum_pads: true,
        show_sequencer: false,
    })
    .show(ui);
```

## Piano Configuration

Customize piano keyboard range.

```demo
use armas::components::audio::{MidiController, MidiControllerState};

let mut state = MidiControllerState::default();

MidiController::new(&mut state)
    .piano(2, 4) // 2 octaves starting from C4
    .show(ui);
```

## Drum Pad Grid

Configure drum pad layout.

```demo
use armas::components::audio::{MidiController, MidiControllerState};

let mut state = MidiControllerState::default();

MidiController::new(&mut state)
    .drum_pads(4, 4) // 4x4 grid
    .show(ui);
```

## Step Sequencer Steps

Set number of sequencer steps.

```demo
use armas::components::audio::{MidiController, MidiControllerState};

let mut state = MidiControllerState::default();

MidiController::new(&mut state)
    .sequencer_steps(32) // 32-step sequencer
    .show(ui);
```

## Visual Variants

Customize component appearance.

```demo
use armas::components::audio::{MidiController, MidiControllerState, WheelVariant, PadVariant};

let mut state = MidiControllerState::default();

MidiController::new(&mut state)
    .wheel_variant(WheelVariant::Elevated)
    .pad_variant(PadVariant::Outlined)
    .show(ui);
```

## Complete DAW Controller

```demo
use armas::components::audio::{MidiController, MidiControllerState};

let mut state = MidiControllerState::default();

let response = MidiController::new(&mut state)
    .layout(armas::components::audio::ControllerLayout::Full)
    .piano(3, 3) // 3 octaves from C3
    .drum_pads(4, 4)
    .sequencer_steps(16)
    .show(ui);

// Piano keyboard events
if let Some(piano) = response.piano {
    if !piano.clicked_keys.is_empty() {
        ui.label(format!("Piano notes: {:?}", piano.clicked_keys));
    }
}

// Wheel events
if response.mod_wheel_changed {
    ui.label(format!("Mod wheel: {:.2}", state.mod_wheel));
}

if response.pitch_wheel_changed {
    ui.label(format!("Pitch bend: {:+.2}", state.pitch_wheel));
}

// XY pad events
if response.xy_pad_changed {
    ui.label(format!("XY: ({:.2}, {:.2})", state.xy_x, state.xy_y));
}

// Drum pads
if let Some(pads) = response.drum_pads {
    if let Some((note, velocity)) = pads.pressed {
        ui.label(format!("Drum pad {} hit: {}", note, velocity));
    }
}

// Sequencer pattern
if response.sequencer_changed {
    let active_steps: Vec<usize> = state.sequencer_steps
        .iter()
        .enumerate()
        .filter_map(|(i, &active)| if active { Some(i + 1) } else { None })
        .collect();
    ui.label(format!("Active steps: {:?}", active_steps));
}
```

## API Reference

### Constructor

```rust
MidiController::new(state: &mut MidiControllerState) -> Self
```

Creates a new MIDI controller with mutable state reference.

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.layout()` | `ControllerLayout` | `Full` | Preset layout configuration |
| `.sections()` | `ControllerSections` | All visible | Custom section visibility |
| `.piano()` | `(u8, i32)` | `(3, 3)` | Piano octaves and start octave |
| `.drum_pads()` | `(usize, usize)` | `(4, 4)` | Drum pad grid (rows, cols) |
| `.sequencer_steps()` | `usize` | `16` | Number of sequencer steps |
| `.wheel_variant()` | `WheelVariant` | `Filled` | Visual variant for wheels |
| `.pad_variant()` | `PadVariant` | `Filled` | Visual variant for pads |

### Show Method

```rust
pub fn show(self, ui: &mut egui::Ui) -> MidiControllerResponse
```

Returns a `MidiControllerResponse` with all interaction data.

### MidiControllerState

```rust
pub struct MidiControllerState {
    pub mod_wheel: f32,          // 0.0 to 1.0
    pub pitch_wheel: f32,        // -1.0 to 1.0
    pub xy_x: f32,               // 0.0 to 1.0
    pub xy_y: f32,               // 0.0 to 1.0
    pub active_notes: HashMap<u8, u8>,  // note -> velocity
    pub drum_pads: HashMap<u8, PadState>,  // note -> PadState
    pub sequencer_steps: Vec<bool>,  // step pattern
}
```

Persistent state for the controller. Store this in your application state.

### MidiControllerResponse

```rust
pub struct MidiControllerResponse {
    pub response: Response,
    pub piano: Option<PianoResponse>,
    pub drum_pads: Option<MidiPadResponse>,
    pub mod_wheel_changed: bool,
    pub pitch_wheel_changed: bool,
    pub xy_pad_changed: bool,
    pub sequencer_changed: bool,
}
```

All interaction events from the controller.

### ControllerLayout

```rust
pub enum ControllerLayout {
    Full,        // All sections
    Compact,     // Piano and wheels only
    Performance, // Piano, wheels, XY pad, and drum pads
}
```

### ControllerSections

```rust
pub struct ControllerSections {
    pub show_piano: bool,
    pub show_wheels: bool,
    pub show_xy_pad: bool,
    pub show_drum_pads: bool,
    pub show_sequencer: bool,
}
```

## Section Details

### Piano Keyboard
- Multi-octave chromatic keyboard
- Velocity-sensitive (default 100)
- Visual feedback for pressed keys
- Horizontal orientation

### Mod & Pitch Wheels
- **Modulation Wheel**: Stays at position (0.0-1.0)
- **Pitch Bend Wheel**: Springs back to center (-1.0 to 1.0)
- Realistic 3D wheel rendering
- Glow effects on interaction

### XY Pad
- 2D controller for simultaneous parameter control
- Normalized values (0.0-1.0)
- Crosshair position indicator
- Default size: 150x150px

### Drum Pads
- 4x4 grid by default (configurable)
- Velocity-sensitive triggers
- GM MIDI drum map (notes 36-51)
- Color-coded by semantic scheme

### Step Sequencer
- 16 steps by default (configurable)
- Toggle on/off per step
- Step numbers displayed
- Suitable for rhythm programming

## MIDI Integration

The controller state maps directly to MIDI events:

```demo
use armas::components::audio::{MidiController, MidiControllerState};

let mut state = MidiControllerState::default();
let response = MidiController::new(&mut state).show(ui);

// Send MIDI events to your audio engine
if let Some(piano) = response.piano {
    for note in piano.clicked_keys {
        // send_midi_note_on(note, 100);
    }
    for note in piano.released_keys {
        // send_midi_note_off(note);
    }
}

if response.mod_wheel_changed {
    // send_midi_cc(1, (state.mod_wheel * 127.0) as u8); // CC 1 = Modulation
}

if response.pitch_wheel_changed {
    // let value = ((state.pitch_wheel + 1.0) * 8191.5) as i16;
    // send_midi_pitch_bend(value);
}
```

## Use Cases

### Live Performance Controller

```demo
use armas::components::audio::{MidiController, MidiControllerState, ControllerLayout};

let mut state = MidiControllerState::default();

MidiController::new(&mut state)
    .layout(ControllerLayout::Performance)
    .piano(2, 4) // 2 octaves from C4
    .drum_pads(4, 4)
    .show(ui);
```

### Production Workstation

```demo
use armas::components::audio::{MidiController, MidiControllerState, ControllerLayout};

let mut state = MidiControllerState::default();

MidiController::new(&mut state)
    .layout(ControllerLayout::Full)
    .piano(5, 2) // 5 octaves from C2
    .sequencer_steps(32)
    .show(ui);
```

### MIDI Learn Interface

```demo
use armas::components::audio::{MidiController, MidiControllerState};

let mut state = MidiControllerState::default();
let response = MidiController::new(&mut state).show(ui);

// Map any controller to parameters
if response.mod_wheel_changed {
    ui.label("Learning: Modulation Wheel → Filter Cutoff");
}

if response.xy_pad_changed {
    ui.label(format!("XY Position: X={:.2}, Y={:.2}", state.xy_x, state.xy_y));
}
```

## Dependencies

- `egui = "0.33"`
- All audio components: `Piano`, `ModWheel`, `XYPad`, `MidiPad`, `StepSequencer`
- Theme colors: Uses theme system for consistent appearance
- Minimum version: `armas 0.1.0`

## Related Components

- **Piano**: Chromatic keyboard input
- **ModWheel**: Modulation and pitch bend controllers
- **XYPad**: 2D parameter control
- **MidiPad**: Drum pad grid
- **StepSequencer**: Pattern programming
- **PianoRoll**: Timeline-based MIDI editor
