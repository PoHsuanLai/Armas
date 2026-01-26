# Step Sequencer

Grid of toggle buttons for rhythm programming and pattern creation. Perfect for drum machines and pattern-based sequencers.

## Basic Usage

```demo
let mut steps = vec![false; 16];
steps[0] = true;  // First step
steps[4] = true;  // Fifth step
steps[8] = true;  // Ninth step

let response = StepSequencer::new(&mut steps)
    .steps(16)
    .show(ui, &theme);

if response.changed() {
    ui.label("Pattern changed!");
}
```

## With Playback Position

```demo
let mut steps = vec![true, false, true, false, true, false, true, false];
let current_step = 2; // Simulated playback position

StepSequencer::new(&mut steps)
    .steps(8)
    .current_step(Some(current_step))
    .show(ui, &theme);
```

## With Step Numbers

```demo
let mut steps = vec![false; 16];
steps[0] = true;
steps[5] = true;
steps[10] = true;

StepSequencer::new(&mut steps)
    .steps(16)
    .show_step_numbers(true)
    .show(ui, &theme);
```

## Custom Step Size

```demo
let mut steps = vec![true, false, true, false];

StepSequencer::new(&mut steps)
    .steps(4)
    .step_size(60.0, 60.0)
    .gap(8.0)
    .show(ui, &theme);
```

## Custom Accent Color

```demo
let mut steps = vec![true, false, true, true, false, true, false, false];

StepSequencer::new(&mut steps)
    .steps(8)
    .accent_color(egui::Color32::from_rgb(255, 100, 100))
    .show(ui, &theme);
```

## 16-Step Drum Pattern

```demo
let mut kick_pattern = vec![
    true, false, false, false,
    true, false, false, false,
    true, false, false, false,
    true, false, false, false
];

ui.label("Kick:");
StepSequencer::new(&mut kick_pattern)
    .steps(16)
    .accent_color(egui::Color32::from_rgb(255, 100, 100))
    .show(ui, &theme);

let mut snare_pattern = vec![
    false, false, false, false,
    true, false, false, false,
    false, false, false, false,
    true, false, false, false
];

ui.label("Snare:");
StepSequencer::new(&mut snare_pattern)
    .steps(16)
    .accent_color(egui::Color32::from_rgb(100, 150, 255))
    .show(ui, &theme);
```

## With Live Playback

```demo
let mut steps = vec![true, false, true, false, true, false, true, true];
let current_step = 3; // Simulated playback

StepSequencer::new(&mut steps)
    .steps(8)
    .current_step(Some(current_step))
    .glow_intensity(1.2)
    .show(ui, &theme);

ui.label(format!("Playing step: {}", current_step + 1));
```

## Multiple Patterns

```demo
ui.vertical(|ui| {
    let mut kick = vec![true, false, false, false, true, false, false, false];
    let mut snare = vec![false, false, true, false, false, false, true, false];
    let mut hat = vec![true, true, true, true, true, true, true, true];

    ui.label("Kick:");
    StepSequencer::new(&mut kick)
        .steps(8)
        .step_size(35.0, 35.0)
        .accent_color(egui::Color32::from_rgb(255, 80, 80))
        .show(ui, &theme);

    ui.add_space(4.0);
    ui.label("Snare:");
    StepSequencer::new(&mut snare)
        .steps(8)
        .step_size(35.0, 35.0)
        .accent_color(egui::Color32::from_rgb(100, 150, 255))
        .show(ui, &theme);

    ui.add_space(4.0);
    ui.label("Hi-Hat:");
    StepSequencer::new(&mut hat)
        .steps(8)
        .step_size(35.0, 35.0)
        .accent_color(egui::Color32::from_rgb(255, 220, 100))
        .show(ui, &theme);
});
```

## API Reference

### Constructor

```rust
StepSequencer::new(steps: &mut Vec<bool>) -> Self
```

Creates a new step sequencer with mutable reference to step pattern.

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.steps()` | `usize` | `16` | Number of steps to display |
| `.current_step()` | `Option<usize>` | None | Current playback step (0-indexed) |
| `.step_size()` | `(f32, f32)` | `(40.0, 40.0)` | Step width and height in pixels |
| `.gap()` | `f32` | `4.0` | Gap between steps in pixels |
| `.accent_color()` | `Color32` | `theme.primary()` | Color for active steps |
| `.show_step_numbers()` | `bool` | `false` | Show step numbers (1-indexed) |
| `.glow_intensity()` | `f32` | `0.8` | Glow intensity for current step (0.0-1.0) |
| `.velocities()` | `&Vec<f32>` | None | Optional velocity data for visualization |
| `.measure_accent()` | `usize` | None | Show accents every N steps for rhythm structure |

### Show Method

```rust
pub fn show(self, ui: &mut egui::Ui) -> Response
```

Returns an egui `Response`. The steps vector is automatically resized to match the number of steps.

## Step Pattern

- **Vec<bool>**: Each element represents one step (true = on, false = off)
- **Auto-resize**: Vector automatically resized to match `.steps()` count
- **0-indexed**: First step is index 0
- **Toggle interaction**: Click any step to toggle on/off

## Visual Design

### Active Steps
- Filled with accent color (default: theme primary)
- Border visible on all variants
- Brighter on hover

### Inactive Steps
- Filled with surface_variant color
- Subtle border
- Brightens on hover

### Current Step Indicator
- Colored bar at bottom of step
- Multi-layer glow effect
- Uses theme secondary color
- Only shown when `current_step` is set

### Step Numbers
- Optional 1-indexed numbering (1, 2, 3...)
- Contrasting text color based on step state
- Small proportional font (9pt)

## Interaction

- **Click**: Toggle step on/off
- **Visual Feedback**: Hover highlighting, glow on current step
- **No drag**: Each step toggles independently (not a slider)

## Use Cases

### Basic Drum Machine

```demo
let mut pattern = vec![
    true, false, false, false,
    true, false, true, false,
    false, false, true, false,
    true, false, true, false
];

StepSequencer::new(&mut pattern)
    .steps(16)
    .show_step_numbers(true)
    .show(ui, &theme);
```

### 808-Style Patterns

```demo
ui.vertical(|ui| {
    let mut bd = vec![true, false, false, false, false, false, true, false];
    let mut sd = vec![false, false, true, false, false, false, true, false];
    let mut ch = vec![false, true, false, true, false, true, false, true];
    let mut oh = vec![false, false, false, false, true, false, false, false];

    ui.label("BD:");
    StepSequencer::new(&mut bd).steps(8).show(ui, &theme);

    ui.label("SD:");
    StepSequencer::new(&mut sd).steps(8).show(ui, &theme);

    ui.label("CH:");
    StepSequencer::new(&mut ch).steps(8).show(ui, &theme);

    ui.label("OH:");
    StepSequencer::new(&mut oh).steps(8).show(ui, &theme);
});
```

### Melody Sequencer

```demo
// Each step could trigger different notes
let mut melody = vec![
    true, false, true, false,
    false, true, false, true,
    true, false, false, true,
    false, true, true, false
];

StepSequencer::new(&mut melody)
    .steps(16)
    .accent_color(egui::Color32::from_rgb(150, 200, 255))
    .show(ui, &theme);

ui.label(format!("Active steps: {}", melody.iter().filter(|&&s| s).count()));
```

## Pattern Analysis

```demo
let mut steps = vec![true, false, true, true, false, true, false, false];

StepSequencer::new(&mut steps)
    .steps(8)
    .show(ui, &theme);

let active_count = steps.iter().filter(|&&s| s).count();
let density = active_count as f32 / steps.len() as f32;

ui.label(format!("Active: {}/{}", active_count, steps.len()));
ui.label(format!("Density: {:.1}%", density * 100.0));
```

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `secondary`, `surface`, `surface_variant`, `outline`, `outline_variant`, `on_surface`, `on_surface_variant`
- Minimum version: `armas 0.1.0`

## Related Components

- **MidiPad**: Grid of momentary trigger pads
- **Piano**: Chromatic keyboard input
- **PianoRoll**: Timeline-based note editor
- **Transport**: Playback controls
