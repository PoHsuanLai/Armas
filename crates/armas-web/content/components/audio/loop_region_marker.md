# Loop Region Marker

Visual markers for loop start/end points with draggable handles. Shows highlighted loop region background in the timeline for defining playback loop regions.

## Basic Usage

```demo
use armas::components::audio::LoopRegionMarker;

let mut loop_start = 1.0;  // beats
let mut loop_end = 3.0;   // beats

let response = LoopRegionMarker::new(&mut loop_start, &mut loop_end)
    .id("basic_loop")
    .beat_width(40.0)  // Must match Timeline
    .measures(4)
    .height(50.0)
    .show(ui);

if response.loop_start_changed {
    ui.label(format!("Loop start: {}", loop_start));
}
```

## With Snap to Grid

```demo
use armas::components::audio::LoopRegionMarker;

let mut loop_start = 0.0;
let mut loop_end = 3.0;

LoopRegionMarker::new(&mut loop_start, &mut loop_end)
    .id("snap_loop")
    .beat_width(40.0)
    .measures(4)
    .height(50.0)
    .snap_to_grid(true)
    .grid_division(1.0)  // snap to whole beats
    .show(ui);
```

## Fine Grid Snapping

```demo
use armas::components::audio::LoopRegionMarker;

let mut loop_start = 0.5;
let mut loop_end = 3.5;

LoopRegionMarker::new(&mut loop_start, &mut loop_end)
    .id("fine_snap_loop")
    .beat_width(40.0)
    .measures(4)
    .height(50.0)
    .snap_to_grid(true)
    .grid_division(0.25)  // snap to 16th notes
    .show(ui);
```

## Custom Color

```demo
use armas::components::audio::LoopRegionMarker;

let mut loop_start = 0.0;
let mut loop_end = 4.0;

LoopRegionMarker::new(&mut loop_start, &mut loop_end)
    .id("custom_color_loop")
    .beat_width(40.0)
    .measures(4)
    .height(50.0)
    .color(egui::Color32::from_rgb(100, 200, 255))
    .show(ui);
```

## Without Labels

```demo
use armas::components::audio::LoopRegionMarker;

let mut loop_start = 1.0;
let mut loop_end = 3.0;

LoopRegionMarker::new(&mut loop_start, &mut loop_end)
    .id("no_labels_loop")
    .beat_width(40.0)
    .measures(4)
    .height(50.0)
    .show_labels(false)
    .show(ui);
```

## Disabled State

```demo
use armas::components::audio::LoopRegionMarker;

let mut loop_start = 0.0;
let mut loop_end = 4.0;

// Loop is disabled, no visual markers shown
LoopRegionMarker::new(&mut loop_start, &mut loop_end)
    .id("disabled_loop")
    .beat_width(40.0)
    .measures(4)
    .height(50.0)
    .enabled(false)
    .show(ui);

ui.label("Loop disabled - no markers shown");
```

## With State Persistence

```demo
use armas::components::audio::LoopRegionMarker;

let mut loop_start = 0.0;
let mut loop_end = 4.0;

LoopRegionMarker::new(&mut loop_start, &mut loop_end)
    .id("my_loop_marker")
    .beat_width(40.0)
    .measures(4)
    .height(50.0)
    .snap_to_grid(true)
    .grid_division(1.0)
    .show(ui);
```

## In Timeline Context

```demo
use armas::components::audio::LoopRegionMarker;

let mut loop_start = 2.0;
let mut loop_end = 6.0;

ui.vertical(|ui| {
    ui.label("Timeline with Loop Region");

    // Draw loop marker at the top of timeline
    let response = LoopRegionMarker::new(&mut loop_start, &mut loop_end)
        .id("timeline_context_loop")
        .beat_width(40.0)
        .measures(8)
        .height(50.0)
        .enabled(true)
        .snap_to_grid(true)
        .grid_division(0.5)
        .show(ui);

    // Show feedback
    if response.loop_start_changed || response.loop_end_changed {
        ui.label(format!("Loop: {:.1} - {:.1} beats", loop_start, loop_end));
    }

    if response.region_clicked {
        ui.label("Loop region clicked!");
    }
});
```

## Multiple Loop Markers

```demo
use armas::components::audio::LoopRegionMarker;

let mut loop_a_start = 0.0;
let mut loop_a_end = 3.0;
let mut loop_b_start = 4.0;
let mut loop_b_end = 7.0;

ui.vertical(|ui| {
    ui.label("Loop A:");
    LoopRegionMarker::new(&mut loop_a_start, &mut loop_a_end)
        .id("loop_a")
        .beat_width(40.0)
        .measures(8)
        .height(50.0)
        .color(egui::Color32::from_rgb(100, 150, 255))
        .show(ui);

    ui.add_space(8.0);

    ui.label("Loop B:");
    LoopRegionMarker::new(&mut loop_b_start, &mut loop_b_end)
        .id("loop_b")
        .beat_width(40.0)
        .measures(8)
        .height(50.0)
        .color(egui::Color32::from_rgb(255, 150, 100))
        .show(ui);
});
```

## API Reference

### Constructor

```rust
LoopRegionMarker::new(loop_start: &mut f32, loop_end: &mut f32) -> Self
```

Creates a new loop region marker with mutable references to start and end positions (in beats).

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.id()` | `impl Into<egui::Id>` | None | Unique ID for state persistence |
| `.beat_width()` | `f32` | `60.0` | Pixels per beat (must match Timeline) |
| `.measures()` | `u32` | `16` | Number of measures to display |
| `.beats_per_measure()` | `u32` | `4` | Beats per measure |
| `.enabled()` | `bool` | `true` | Enable or disable loop markers |
| `.snap_to_grid()` | `bool` | `false` | Enable snap to grid when dragging |
| `.grid_division()` | `f32` | `1.0` | Grid division for snapping (beats) |
| `.color()` | `Color32` | `theme.secondary()` | Custom color for loop region |
| `.handle_width()` | `f32` | `8.0` | Width of draggable handles |
| `.show_labels()` | `bool` | `true` | Show time labels on handles |

### Show Method

```rust
pub fn show(self, ui: &mut egui::Ui) -> LoopRegionMarkerResponse
```

Returns a `LoopRegionMarkerResponse` with interaction data.

### LoopRegionMarkerResponse

```rust
pub struct LoopRegionMarkerResponse {
    pub response: Response,
    pub loop_start_changed: bool,
    pub loop_end_changed: bool,
    pub region_clicked: bool,
}
```

- `response`: The egui Response for the component
- `loop_start_changed`: True if start handle was dragged
- `loop_end_changed`: True if end handle was dragged
- `region_clicked`: True if loop region area was clicked

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

### Loop Region
- Semi-transparent colored fill (30% opacity)
- Spans from start to end handle
- Uses secondary theme color by default
- Clickable for selection

### Handles
- Vertical bars with draggable interaction (smallest height: 50px)
- Arrow indicators (left arrow for start, right arrow for end)
- Rounded corners
- Standard border (1px)
- Glow effect on hover (3 layers)
- Color matches loop region

### Labels
- Beat position displayed above each handle
- 1 decimal precision (e.g., "4.5")
- Small proportional font (10pt)
- Can be hidden with `.show_labels(false)`

## Interaction

- **Drag handles**: Move loop start/end points
- **Snap to grid**: When enabled, handles snap to grid divisions
- **Click region**: Detects clicks on the loop region background
- **Auto-swap**: If start is dragged past end, they automatically swap
- **State persistence**: Position saved when ID is set

## Use Cases

### Basic Looping

```demo
use armas::components::audio::LoopRegionMarker;

let mut loop_start = 0.0;
let mut loop_end = 4.0;

let response = LoopRegionMarker::new(&mut loop_start, &mut loop_end)
    .id("basic_looping")
    .beat_width(40.0)
    .measures(4)
    .height(50.0)
    .enabled(true)
    .snap_to_grid(true)
    .grid_division(1.0)
    .show(ui);

if response.loop_start_changed || response.loop_end_changed {
    // Update audio engine loop points
    ui.label(format!("Update loop: {} to {}", loop_start, loop_end));
}
```

### Recording Punch-In/Out

```demo
use armas::components::audio::LoopRegionMarker;

let mut punch_in = 2.0;
let mut punch_out = 4.0;

ui.label("Record Region:");
LoopRegionMarker::new(&mut punch_in, &mut punch_out)
    .id("punch_in_out")
    .beat_width(40.0)
    .measures(4)
    .height(50.0)
    .color(egui::Color32::from_rgb(255, 100, 100))
    .snap_to_grid(true)
    .show(ui);
```

### Selection Range

```demo
use armas::components::audio::LoopRegionMarker;

let mut selection_start = 1.0;
let mut selection_end = 3.0;

ui.label("Selected Range:");
LoopRegionMarker::new(&mut selection_start, &mut selection_end)
    .id("selection_range")
    .beat_width(40.0)
    .measures(4)
    .height(50.0)
    .color(egui::Color32::from_rgb(150, 150, 150))
    .snap_to_grid(false)  // Free selection
    .show(ui);

ui.label(format!("Duration: {:.2} beats", selection_end - selection_start));
```

## Integration with Timeline

The loop region marker is designed to work seamlessly with the Timeline component:

```demo
use armas::components::audio::{LoopRegionMarker, Timeline};

let mut loop_start = 0.0;
let mut loop_end = 4.0;

ui.vertical(|ui| {
    // Loop marker at the top - using same beat_width and measures as Timeline
    LoopRegionMarker::new(&mut loop_start, &mut loop_end)
        .id("timeline_loop")
        .beat_width(40.0)  // Must match Timeline
        .measures(4)      // Must match Timeline
        .height(50.0)
        .snap_to_grid(true)
        .grid_division(1.0)
        .show(ui);

    ui.add_space(4.0);

    // Timeline content below with matching parameters
    // Timeline::new()
    //     .beat_width(40.0)
    //     .measures(4)
    //     .show(ui, &mut tracks, &mut playhead_pos, theme);
});
```

## Dependencies

- `egui = "0.33"`
- Theme colors: `secondary`, `on_surface`
- Minimum version: `armas 0.1.0`

## Related Components

- **Timeline**: Main timeline component
- **TimeRuler**: Shows beat/bar markers
- **Playhead**: Current playback position
- **SelectionRange**: Similar marker for editing selections
- **PunchMarker**: Recording region markers
