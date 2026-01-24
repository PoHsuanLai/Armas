# Punch Marker

Visual markers for recording punch-in/punch-out points with draggable handles. Shows highlighted punch region background in the timeline for defining auto punch recording regions.

## Basic Usage

```demo
use armas_audio::PunchMarker;

let mut punch_in = 1.0;   // beats
let mut punch_out = 3.0; // beats

let response = PunchMarker::new(&mut punch_in, &mut punch_out)
    .id("basic_punch")
    .beat_width(40.0)  // Must match Timeline
    .measures(4)
    .height(70.0)
    .show(ui);

if response.punch_in_changed {
    ui.label(format!("Punch in: {}", punch_in));
}
```

## With Snap to Grid

```demo
use armas_audio::PunchMarker;

let mut punch_in = 0.0;
let mut punch_out = 3.0;

PunchMarker::new(&mut punch_in, &mut punch_out)
    .id("snap_punch")
    .beat_width(40.0)
    .measures(4)
    .height(70.0)
    .snap_to_grid(true)
    .grid_division(1.0)  // snap to whole beats
    .show(ui);
```

## Fine Grid Snapping

```demo
use armas_audio::PunchMarker;

let mut punch_in = 0.5;
let mut punch_out = 3.5;

PunchMarker::new(&mut punch_in, &mut punch_out)
    .id("fine_snap_punch")
    .beat_width(40.0)
    .measures(4)
    .height(70.0)
    .snap_to_grid(true)
    .grid_division(0.25)  // snap to 16th notes
    .show(ui);
```

## Custom Color

```demo
use armas_audio::PunchMarker;

let mut punch_in = 0.0;
let mut punch_out = 4.0;

PunchMarker::new(&mut punch_in, &mut punch_out)
    .id("custom_color_punch")
    .beat_width(40.0)
    .measures(4)
    .height(70.0)
    .color(egui::Color32::from_rgb(255, 140, 0))  // Orange
    .show(ui);
```

## Without Labels

```demo
use armas_audio::PunchMarker;

let mut punch_in = 1.0;
let mut punch_out = 3.0;

PunchMarker::new(&mut punch_in, &mut punch_out)
    .id("no_labels_punch")
    .beat_width(40.0)
    .measures(4)
    .height(70.0)
    .show_labels(false)
    .show(ui);
```

## Disabled State

```demo
use armas_audio::PunchMarker;

let mut punch_in = 0.0;
let mut punch_out = 4.0;

// Punch is disabled, no visual markers shown
PunchMarker::new(&mut punch_in, &mut punch_out)
    .id("disabled_punch")
    .beat_width(40.0)
    .measures(4)
    .height(70.0)
    .enabled(false)
    .show(ui);

ui.label("Punch recording disabled - no markers shown");
```

## With State Persistence

```demo
use armas_audio::PunchMarker;

let mut punch_in = 0.0;
let mut punch_out = 4.0;

PunchMarker::new(&mut punch_in, &mut punch_out)
    .id("my_punch_marker")
    .beat_width(40.0)
    .measures(4)
    .height(70.0)
    .snap_to_grid(true)
    .grid_division(1.0)
    .show(ui);
```

## In Timeline Context

```demo
use armas_audio::PunchMarker;

let mut punch_in = 2.0;
let mut punch_out = 6.0;

ui.vertical(|ui| {
    ui.label("Timeline with Punch Recording Region");

    // Draw punch marker at the top of timeline
    let response = PunchMarker::new(&mut punch_in, &mut punch_out)
        .id("timeline_context_punch")
        .beat_width(40.0)
        .measures(8)
        .height(70.0)
        .enabled(true)
        .snap_to_grid(true)
        .grid_division(0.5)
        .show(ui);

    // Show feedback
    if response.punch_in_changed || response.punch_out_changed {
        ui.label(format!("Punch: {:.1} - {:.1} beats", punch_in, punch_out));
    }

    if response.region_clicked {
        ui.label("Punch region clicked!");
    }
});
```

## Multiple Punch Regions

```demo
use armas_audio::PunchMarker;

let mut punch_a_in = 0.0;
let mut punch_a_out = 3.0;
let mut punch_b_in = 4.0;
let mut punch_b_out = 7.0;

ui.vertical(|ui| {
    ui.label("Punch Region A:");
    PunchMarker::new(&mut punch_a_in, &mut punch_a_out)
        .id("punch_a")
        .beat_width(40.0)
        .measures(8)
        .height(70.0)
        .color(egui::Color32::from_rgb(220, 50, 50))
        .show(ui);

    ui.add_space(8.0);

    ui.label("Punch Region B:");
    PunchMarker::new(&mut punch_b_in, &mut punch_b_out)
        .id("punch_b")
        .beat_width(40.0)
        .measures(8)
        .height(70.0)
        .color(egui::Color32::from_rgb(255, 140, 0))
        .show(ui);
});
```

## API Reference

### Constructor

```rust
PunchMarker::new(punch_in: &mut f32, punch_out: &mut f32) -> Self
```

Creates a new punch marker with mutable references to punch-in and punch-out positions (in beats).

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.id()` | `impl Into<egui::Id>` | None | Unique ID for state persistence |
| `.beat_width()` | `f32` | `60.0` | Pixels per beat (must match Timeline) |
| `.measures()` | `u32` | `16` | Number of measures to display |
| `.beats_per_measure()` | `u32` | `4` | Beats per measure |
| `.enabled()` | `bool` | `true` | Enable or disable punch markers |
| `.snap_to_grid()` | `bool` | `false` | Enable snap to grid when dragging |
| `.grid_division()` | `f32` | `1.0` | Grid division for snapping (beats) |
| `.color()` | `Color32` | Recording red `(220, 50, 50)` | Custom color for punch region |
| `.handle_width()` | `f32` | `8.0` | Width of draggable handles |
| `.show_labels()` | `bool` | `true` | Show time labels on handles |

### Show Method

```rust
pub fn show(self, ui: &mut egui::Ui) -> PunchMarkerResponse
```

Returns a `PunchMarkerResponse` with interaction data.

### PunchMarkerResponse

```rust
pub struct PunchMarkerResponse {
    pub response: Response,
    pub punch_in_changed: bool,
    pub punch_out_changed: bool,
    pub region_clicked: bool,
}
```

- `response`: The egui Response for the component
- `punch_in_changed`: True if punch-in handle was dragged
- `punch_out_changed`: True if punch-out handle was dragged
- `region_clicked`: True if punch region area was clicked

## Grid Division Values

Common grid division values for snapping:

| Division | Musical Value | Description |
|----------|---------------|-------------|
| `1.0` | Whole beat | Quarter notes in 4/4 |
| `0.5` | Half beat | 8th notes |
| `0.25` | Quarter beat | 16th notes |
| `0.125` | Eighth beat | 32nd notes |
| `2.0` | Two beats | Half note |
| `4.0` | Four beats | Whole note (bar in 4/4) |

## Visual Design

### Punch Region
- Semi-transparent recording red fill (35% opacity)
- Dashed border to distinguish from loop/selection regions
- Spans from punch-in to punch-out handle
- Uses recording red color by default (220, 50, 50)
- Clickable for selection

### Handles
- Vertical bars with draggable interaction (taller than loop/selection handles)
- **Record button icon** (white filled circle) at center
- Small "IN" and "OUT" text labels below the icon
- Thicker border (2px) in dark red
- Rounded corners
- Stronger glow effect on hover (4 layers)
- Color matches punch region (recording red)

### Labels
- Beat position displayed above each handle
- 1 decimal precision (e.g., "8.5")
- Small proportional font (10pt)
- Can be hidden with `.show_labels(false)`

## Interaction

- **Drag handles**: Move punch-in/punch-out points
- **Snap to grid**: When enabled, handles snap to grid divisions
- **Click region**: Detects clicks on the punch region background
- **Auto-swap**: If punch-in is dragged past punch-out, they automatically swap
- **State persistence**: Position saved when ID is set

## Differences from Loop and Selection Markers

| Feature | PunchMarker | LoopRegionMarker | SelectionRange |
|---------|-------------|------------------|----------------|
| **Purpose** | Recording punch in/out | Playback looping | Editing operations |
| **Height** | 70px (tallest) | 50px (shortest) | 60px (medium) |
| **Default Color** | Recording red (220, 50, 50) | Theme secondary | Neutral gray |
| **Border Style** | Dashed (4px dash, 3px gap) | None | Solid 1px |
| **Handle Indicator** | Record button icon (●) + "IN"/"OUT" text | Arrows `< >` | Brackets `[ ]` |
| **Handle Border** | Thick 2px dark red | Standard 1px | Standard 1px |
| **Opacity** | 35% (most visible) | 30% | 40% |
| **Glow Layers** | 4 layers (strongest) | 3 layers | 3 layers |
| **Visual Priority** | Highest (recording focus) | Lowest (background) | High (editing focus) |

## Use Cases

### Auto Punch Recording

```demo
use armas_audio::PunchMarker;

let mut punch_in = 1.0;
let mut punch_out = 3.0;
let mut recording = false;

let response = PunchMarker::new(&mut punch_in, &mut punch_out)
    .id("auto_punch_recording")
    .beat_width(40.0)
    .measures(4)
    .height(70.0)
    .enabled(true)
    .snap_to_grid(true)
    .grid_division(1.0)
    .show(ui);

if response.punch_in_changed || response.punch_out_changed {
    // Update audio engine punch points
    ui.label(format!("Punch region: {} to {}", punch_in, punch_out));
}

ui.horizontal(|ui| {
    if ui.button(if recording { "Stop" } else { "Record" }).clicked() {
        recording = !recording;
    }
    ui.label(format!("Recording: {}", if recording { "ON" } else { "OFF" }));
});
```

### Cycle Recording

```demo
use armas_audio::PunchMarker;

let mut punch_in = 0.0;
let mut punch_out = 4.0;
let mut cycle_count = 0;

ui.label("Cycle Recording Region:");
PunchMarker::new(&mut punch_in, &mut punch_out)
    .id("cycle_recording")
    .beat_width(40.0)
    .measures(4)
    .height(70.0)
    .color(egui::Color32::from_rgb(220, 50, 50))
    .snap_to_grid(true)
    .show(ui);

ui.label(format!("Cycles recorded: {}", cycle_count));
if ui.button("Record Cycle").clicked() {
    cycle_count += 1;
}
```

### Comping Region

```demo
use armas_audio::PunchMarker;

let mut comp_in = 2.0;
let mut comp_out = 4.0;

ui.label("Comping Region:");
PunchMarker::new(&mut comp_in, &mut comp_out)
    .id("comping_region")
    .beat_width(40.0)
    .measures(4)
    .height(70.0)
    .color(egui::Color32::from_rgb(180, 80, 200))  // Purple for comping
    .snap_to_grid(true)
    .show(ui);

let duration = comp_out - comp_in;
ui.label(format!("Comp duration: {:.2} beats ({:.2} bars)", duration, duration / 4.0));
```

## Integration with Timeline

The punch marker is designed to work alongside Timeline, LoopRegionMarker, and SelectionRange:

```demo
use armas_audio::{PunchMarker, LoopRegionMarker, SelectionRange};

let mut loop_start = 0.0;
let mut loop_end = 8.0;
let mut punch_in = 2.0;
let mut punch_out = 4.0;
let mut selection_start = 3.0;
let mut selection_end = 5.0;

ui.vertical(|ui| {
    // All markers use the same parameters as Timeline
    let beat_width = 40.0;
    let measures = 8;

    ui.label("Loop Region:");
    LoopRegionMarker::new(&mut loop_start, &mut loop_end)
        .id("timeline_loop")
        .beat_width(beat_width)
        .measures(measures)
        .height(50.0)
        .show(ui);

    ui.add_space(2.0);

    ui.label("Punch Region:");
    PunchMarker::new(&mut punch_in, &mut punch_out)
        .id("timeline_punch")
        .beat_width(beat_width)
        .measures(measures)
        .height(70.0)
        .snap_to_grid(true)
        .show(ui);

    ui.add_space(2.0);

    ui.label("Selection:");
    SelectionRange::new(&mut selection_start, &mut selection_end)
        .id("timeline_selection")
        .beat_width(beat_width)
        .measures(measures)
        .height(60.0)
        .show(ui);

    ui.add_space(4.0);

    // Timeline content below with matching parameters
    // Timeline::new()
    //     .beat_width(beat_width)
    //     .measures(measures)
    //     .show(ui, &mut tracks, &mut playhead_pos, theme);
});
```

## Dependencies

- `egui = "0.33"`
- Theme colors: `on_surface`
- Minimum version: `armas 0.1.0`

## Related Components

- **LoopRegionMarker**: Playback loop region markers
- **SelectionRange**: Editing selection markers
- **Timeline**: Main timeline component
- **TimeRuler**: Shows beat/bar markers
- **Playhead**: Current playback position
- **TimelineTrack**: Track component for regions
