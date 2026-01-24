# DrumSequencer

Professional multi-row drum sequencer for DAW-style pattern programming. Each row represents a drum sound with independent step patterns and velocity control.

## Features

- **Multi-row layout** - Multiple drum sounds displayed in a grid
- **Row controls** - Name, color, visibility, mute, and solo toggles for each row
- **Step programming** - Click steps to toggle on/off with velocity support
- **Playhead indicator** - Visual line showing current playback position across all rows
- **Professional styling** - Glassmorphic design with glow effects matching Armas aesthetic
- **Velocity visualization** - Small bars inside active steps showing velocity intensity
- **Customizable layout** - Full control over step size, row height, spacing, and visual effects

## Basic Usage

Click steps to toggle them on/off. The glow effect shows active steps. State is automatically persisted using the `.id()` method.

```demo
// Create rows with desired colors
let mut rows = vec![
    DrumRow::new("Kick", 16).with_color(egui::Color32::from_rgb(255, 100, 80)),
    DrumRow::new("Snare", 16).with_color(egui::Color32::from_rgb(100, 200, 255)),
    DrumRow::new("HiHat", 16).with_color(egui::Color32::from_rgb(255, 200, 100)),
    DrumRow::new("Tom", 16).with_color(egui::Color32::from_rgb(200, 100, 255)),
];

// Use .id() for automatic state persistence across frame recreations
let response = DrumSequencer::new(&mut rows)
    .steps(16)
    .id("main_drum_sequencer")  // Persists state automatically
    .show(ui);

if response.changed {
    ui.label("Pattern changed!");
}
```

## API Reference

### Creating a DrumSequencer

```demo
let mut rows = vec![
    DrumRow::new("Kick", 16),
    DrumRow::new("Snare", 16),
];

let sequencer = DrumSequencer::new(&mut rows);
```

### Builder Methods

#### `.id(id: impl Into<egui::Id>)`
Set a unique identifier for automatic state persistence. Without an ID, the drum sequencer must store state externally.

```demo
let mut rows = vec![DrumRow::new("Kick", 16)];

DrumSequencer::new(&mut rows)
    .id("my_sequencer")  // State will persist across frames
    .show(ui);
```

#### `.steps(num_steps: usize)`
Set the number of steps per row (default: 16).

```demo
let mut rows = vec![DrumRow::new("Kick", 16)];

DrumSequencer::new(&mut rows)
    .steps(32)
    .show(ui);
```

#### `.current_step(step: Option<usize>)`
Set the current playback step for visual feedback (default: None).

```demo
let mut rows = vec![DrumRow::new("Kick", 16)];

DrumSequencer::new(&mut rows)
    .current_step(Some(5))
    .show(ui);
```

#### `.step_size(width: f32, height: f32)`
Set individual step dimensions (default: 40x32).

```demo
let mut rows = vec![DrumRow::new("Kick", 16)];

DrumSequencer::new(&mut rows)
    .step_size(50.0, 40.0)
    .show(ui);
```

#### `.row_label_width(width: f32)`
Set the width of row label panel (default: 80).

```demo
let mut rows = vec![DrumRow::new("Kick", 16)];

DrumSequencer::new(&mut rows)
    .row_label_width(100.0)
    .show(ui);
```

#### `.row_height(height: f32)`
Set the height of each row (default: 48).

```demo
let mut rows = vec![DrumRow::new("Kick", 16)];

DrumSequencer::new(&mut rows)
    .row_height(60.0)
    .show(ui);
```

#### `.gap(gap: f32)`
Set spacing between steps and rows (default: 4).

```demo
let mut rows = vec![DrumRow::new("Kick", 16)];

DrumSequencer::new(&mut rows)
    .gap(8.0)
    .show(ui);
```

#### `.glow_intensity(intensity: f32)`
Control the intensity of glow effects (0.0-1.0, default: 0.8).

```demo
let mut rows = vec![DrumRow::new("Kick", 16)];

DrumSequencer::new(&mut rows)
    .glow_intensity(0.5)
    .show(ui);
```

#### `.variant(variant: DrumSequencerVariant)`
Set the visual style of steps (default: Filled). Options: `Filled`, `Outlined`, `Elevated`.

```demo
let mut rows = vec![DrumRow::new("Kick", 16)];

DrumSequencer::new(&mut rows)
    .variant(DrumSequencerVariant::Outlined)
    .show(ui);
```

#### `.color_scheme(scheme: DrumSequencerColorScheme)`
Set the color scheme for steps (default: Semantic). Options: `Semantic`, `Monochrome`.

```demo
let mut rows = vec![DrumRow::new("Kick", 16)];

DrumSequencer::new(&mut rows)
    .color_scheme(DrumSequencerColorScheme::Monochrome)
    .show(ui);
```

#### `.show_velocity(show: bool)`
Display velocity as brightness intensity in active steps (default: true).

```demo
let mut rows = vec![DrumRow::new("Kick", 16)];
rows[0].steps[0].active = true;
rows[0].steps[0].velocity = 0.5;

DrumSequencer::new(&mut rows)
    .show_velocity(true)
    .show(ui);
```

### Showing the Sequencer

```demo
let mut rows = vec![
    DrumRow::new("Kick", 16),
    DrumRow::new("Snare", 16),
];

let response = DrumSequencer::new(&mut rows)
    .steps(16)
    .current_step(Some(3))
    .show(ui);
```

Returns a `DrumSequencerResponse` with:
- `response: Response` - The UI response for further interactions
- `step_toggled: HashMap<(usize, usize), bool>` - Map of (row_idx, step_idx) → active state
- `current_step: Option<usize>` - Current playback step
- `changed: bool` - Whether any step was modified this frame

## Working with Rows

### Creating a Row

```demo
let row = DrumRow::new("Kick", 16);
ui.label("Row created with 16 steps");
```

### Setting Row Color

```demo
let row = DrumRow::new("Snare", 16)
    .with_color(egui::Color32::from_rgb(100, 200, 255));
ui.label("Row with custom color");
```

### Row State

Each row has these fields:
- `name: String` - Display name
- `color: Color32` - Visual color
- `steps: Vec<DrumStep>` - Step data
- `visible: bool` - Whether row is displayed
- `muted: bool` - Whether row is muted
- `soloed: bool` - Whether row is soloed

```demo
let mut rows = vec![
    DrumRow::new("Kick", 16),
    DrumRow::new("Snare", 16),
    DrumRow::new("HiHat", 16),
];

rows[0].muted = true;
rows[1].soloed = true;
rows[2].visible = false;

DrumSequencer::new(&mut rows).steps(16).show(ui);
```

### Working with Steps

Each step contains:
- `active: bool` - Whether the step is on
- `velocity: f32` - MIDI velocity (0.0-1.0)

```demo
let mut rows = vec![DrumRow::new("Kick", 16)];

// Setup some steps
rows[0].steps[0].active = true;
rows[0].steps[0].velocity = 1.0;
rows[0].steps[4].active = true;
rows[0].steps[4].velocity = 0.8;

DrumSequencer::new(&mut rows).steps(16).show(ui);
```

## Styling Notes

The DrumSequencer uses the Armas theme for consistent styling with multiple visual variants:

### Variants

- **Filled** - Solid backgrounds with velocity-based brightness adjustment
- **Outlined** - Transparent backgrounds with colored borders and velocity-based alpha
- **Elevated** - Shadow layers for depth perception with 3D elevated appearance

### Color Schemes

- **Semantic** - Cycles through theme colors (primary, secondary, tertiary) for visual variety
- **Monochrome** - Uses only the primary theme color for consistent minimalist appearance

### General Features

- **Active steps** are colored based on row assignment and variant selection
- **Inactive steps** use the surface variant color
- **Current step** has a secondary color indicator bar at the bottom with multi-layer glow
- **Velocity visualization** - When enabled, active step brightness reflects MIDI velocity (0.0-1.0)
- **Muted rows** appear dimmed with reduced opacity
- **Soloed rows** have a subtle primary color tint
- **Glow effects** scale with the `glow_intensity` parameter and respond to current playback position

## Style Examples

### Filled Variant (Default)

```demo
let mut rows = vec![
    DrumRow::new("Kick", 16).with_color(egui::Color32::from_rgb(255, 100, 80)),
    DrumRow::new("Snare", 16).with_color(egui::Color32::from_rgb(100, 200, 255)),
];

DrumSequencer::new(&mut rows)
    .variant(DrumSequencerVariant::Filled)
    .color_scheme(DrumSequencerColorScheme::Semantic)
    .show(ui);
```

### Outlined Variant

```demo
let mut rows = vec![
    DrumRow::new("Kick", 16).with_color(egui::Color32::from_rgb(255, 100, 80)),
    DrumRow::new("Snare", 16).with_color(egui::Color32::from_rgb(100, 200, 255)),
];

DrumSequencer::new(&mut rows)
    .variant(DrumSequencerVariant::Outlined)
    .color_scheme(DrumSequencerColorScheme::Semantic)
    .show(ui);
```

### Elevated Variant

```demo
let mut rows = vec![
    DrumRow::new("Kick", 16).with_color(egui::Color32::from_rgb(255, 100, 80)),
    DrumRow::new("Snare", 16).with_color(egui::Color32::from_rgb(100, 200, 255)),
];

DrumSequencer::new(&mut rows)
    .variant(DrumSequencerVariant::Elevated)
    .color_scheme(DrumSequencerColorScheme::Semantic)
    .show(ui);
```

### Monochrome Color Scheme

```demo
let mut rows = vec![
    DrumRow::new("Kick", 16).with_color(egui::Color32::from_rgb(255, 100, 80)),
    DrumRow::new("Snare", 16).with_color(egui::Color32::from_rgb(100, 200, 255)),
    DrumRow::new("HiHat", 16).with_color(egui::Color32::from_rgb(255, 200, 100)),
];

DrumSequencer::new(&mut rows)
    .variant(DrumSequencerVariant::Filled)
    .color_scheme(DrumSequencerColorScheme::Monochrome)
    .show(ui);
```

### With Velocity Visualization

```demo
let mut rows = vec![DrumRow::new("Kick", 16)];

// Set up steps with varying velocities
rows[0].steps[0].active = true;
rows[0].steps[0].velocity = 1.0;  // Full brightness
rows[0].steps[4].active = true;
rows[0].steps[4].velocity = 0.7;  // Medium brightness
rows[0].steps[8].active = true;
rows[0].steps[8].velocity = 0.4;  // Low brightness

DrumSequencer::new(&mut rows)
    .show_velocity(true)
    .glow_intensity(1.0)
    .show(ui);
```

## Performance Considerations

- The sequencer efficiently renders only visible rows and steps
- Large numbers of steps (64+) may impact performance depending on row count
- Use `.gap(0.0)` to reduce spacing if rendering many rows
- Velocity visualization has minimal performance impact

## Related Components

- `StepSequencer` - Single-row step sequencer for simpler use cases
- `Piano` - MIDI keyboard for note input
- `Transport` - Playback controls (play, pause, stop)
- `Meter` - Audio level visualization
