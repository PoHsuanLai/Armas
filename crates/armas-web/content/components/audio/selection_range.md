# Selection Range

Visual markers for selection start/end points with draggable handles. Shows highlighted selection region background in the timeline for editing operations (copy, cut, paste, delete).

## Basic Usage

```demo
use armas::components::audio::SelectionRange;

let mut selection_start = 1.0;  // beats
let mut selection_end = 3.0;   // beats

let response = SelectionRange::new(&mut selection_start, &mut selection_end)
    .id("basic_selection")
    .beat_width(40.0)  // Must match Timeline
    .measures(4)
    .height(60.0)
    .show(ui);

if response.selection_start_changed {
    ui.label(format!("Selection start: {}", selection_start));
}
```

## With Snap to Grid

```demo
use armas::components::audio::SelectionRange;

let mut selection_start = 0.0;
let mut selection_end = 3.0;

SelectionRange::new(&mut selection_start, &mut selection_end)
    .id("snap_selection")
    .beat_width(40.0)
    .measures(4)
    .height(60.0)
    .snap_to_grid(true)
    .grid_division(1.0)  // snap to whole beats
    .show(ui);
```

## Fine Grid Snapping

```demo
use armas::components::audio::SelectionRange;

let mut selection_start = 0.5;
let mut selection_end = 3.5;

SelectionRange::new(&mut selection_start, &mut selection_end)
    .id("fine_snap_selection")
    .beat_width(40.0)
    .measures(4)
    .height(60.0)
    .snap_to_grid(true)
    .grid_division(0.25)  // snap to 16th notes
    .show(ui);
```

## Custom Color

```demo
use armas::components::audio::SelectionRange;

let mut selection_start = 0.0;
let mut selection_end = 4.0;

SelectionRange::new(&mut selection_start, &mut selection_end)
    .id("custom_color_selection")
    .beat_width(40.0)
    .measures(4)
    .height(60.0)
    .color(egui::Color32::from_rgb(100, 200, 255))
    .show(ui);
```

## Without Labels

```demo
use armas::components::audio::SelectionRange;

let mut selection_start = 1.0;
let mut selection_end = 3.0;

SelectionRange::new(&mut selection_start, &mut selection_end)
    .id("no_labels_selection")
    .beat_width(40.0)
    .measures(4)
    .height(60.0)
    .show_labels(false)
    .show(ui);
```

## Disabled State

```demo
use armas::components::audio::SelectionRange;

let mut selection_start = 0.0;
let mut selection_end = 4.0;

// Selection is disabled, no visual markers shown
SelectionRange::new(&mut selection_start, &mut selection_end)
    .id("disabled_selection")
    .beat_width(40.0)
    .measures(4)
    .height(60.0)
    .enabled(false)
    .show(ui);

ui.label("Selection disabled - no markers shown");
```

## With State Persistence

```demo
use armas::components::audio::SelectionRange;

let mut selection_start = 0.0;
let mut selection_end = 4.0;

SelectionRange::new(&mut selection_start, &mut selection_end)
    .id("my_selection")
    .beat_width(40.0)
    .measures(4)
    .height(60.0)
    .snap_to_grid(true)
    .grid_division(1.0)
    .show(ui);
```

## In Timeline Context

```demo
use armas::components::audio::SelectionRange;

let mut selection_start = 2.0;
let mut selection_end = 6.0;

ui.vertical(|ui| {
    ui.label("Timeline with Selection Range");

    // Draw selection marker at the top of timeline
    let response = SelectionRange::new(&mut selection_start, &mut selection_end)
        .id("timeline_context_selection")
        .beat_width(40.0)
        .measures(8)
        .height(60.0)
        .enabled(true)
        .snap_to_grid(true)
        .grid_division(0.5)
        .show(ui);

    // Show feedback
    if response.selection_start_changed || response.selection_end_changed {
        ui.label(format!("Selection: {:.1} - {:.1} beats", selection_start, selection_end));
    }

    if response.region_clicked {
        ui.label("Selection region clicked!");
    }
});
```

## Multiple Selections

```demo
use armas::components::audio::SelectionRange;

let mut sel_a_start = 0.0;
let mut sel_a_end = 3.0;
let mut sel_b_start = 4.0;
let mut sel_b_end = 7.0;

ui.vertical(|ui| {
    ui.label("Selection A:");
    SelectionRange::new(&mut sel_a_start, &mut sel_a_end)
        .id("sel_a")
        .beat_width(40.0)
        .measures(8)
        .height(60.0)
        .color(egui::Color32::from_rgb(100, 150, 255))
        .show(ui);

    ui.add_space(8.0);

    ui.label("Selection B:");
    SelectionRange::new(&mut sel_b_start, &mut sel_b_end)
        .id("sel_b")
        .beat_width(40.0)
        .measures(8)
        .height(60.0)
        .color(egui::Color32::from_rgb(255, 150, 100))
        .show(ui);
});
```

## API Reference

### Constructor

```rust
SelectionRange::new(selection_start: &mut f32, selection_end: &mut f32) -> Self
```

Creates a new selection range marker with mutable references to start and end positions (in beats).

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.id()` | `impl Into<egui::Id>` | None | Unique ID for state persistence |
| `.beat_width()` | `f32` | `60.0` | Pixels per beat (must match Timeline) |
| `.measures()` | `u32` | `16` | Number of measures to display |
| `.beats_per_measure()` | `u32` | `4` | Beats per measure |
| `.enabled()` | `bool` | `true` | Enable or disable selection markers |
| `.snap_to_grid()` | `bool` | `false` | Enable snap to grid when dragging |
| `.grid_division()` | `f32` | `1.0` | Grid division for snapping (beats) |
| `.color()` | `Color32` | Gray `(150, 150, 150)` | Custom color for selection region |
| `.handle_width()` | `f32` | `8.0` | Width of draggable handles |
| `.show_labels()` | `bool` | `true` | Show time labels on handles |

### Show Method

```rust
pub fn show(self, ui: &mut egui::Ui) -> SelectionRangeResponse
```

Returns a `SelectionRangeResponse` with interaction data.

### SelectionRangeResponse

```rust
pub struct SelectionRangeResponse {
    pub response: Response,
    pub selection_start_changed: bool,
    pub selection_end_changed: bool,
    pub region_clicked: bool,
}
```

- `response`: The egui Response for the component
- `selection_start_changed`: True if start handle was dragged
- `selection_end_changed`: True if end handle was dragged
- `region_clicked`: True if selection region area was clicked

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

### Selection Region
- Semi-transparent colored fill (40% opacity - slightly more opaque than loop)
- Border outline for clearer visibility
- Spans from start to end handle
- Uses neutral gray by default (different from loop's secondary color)
- Clickable for selection

### Handles
- Vertical bars with draggable interaction (medium height: 60px)
- Bracket indicators ([ and ]) to distinguish from loop arrows
- Rounded corners
- Standard border (1px)
- Glow effect on hover (3 layers)
- Color matches selection region

### Labels
- Beat position displayed above each handle
- 1 decimal precision (e.g., "4.5")
- Small proportional font (10pt)
- Can be hidden with `.show_labels(false)`

## Interaction

- **Drag handles**: Move selection start/end points
- **Snap to grid**: When enabled, handles snap to grid divisions
- **Click region**: Detects clicks on the selection region background
- **Auto-swap**: If start is dragged past end, they automatically swap
- **State persistence**: Position saved when ID is set

## Differences from Loop Region Marker

| Feature | SelectionRange | LoopRegionMarker |
|---------|----------------|------------------|
| **Purpose** | Editing operations (copy/cut/paste) | Playback looping |
| **Height** | 60px (medium) | 50px (shortest) |
| **Default Color** | Neutral gray (150, 150, 150) | Theme secondary color |
| **Opacity** | 40% (more visible) | 30% (more subtle) |
| **Border** | Yes (1px outline) | No |
| **Handle Indicator** | Brackets `[ ]` | Arrows `< >` |
| **Handle Border** | Standard 1px | Standard 1px |
| **Visual Priority** | Higher (editing focus) | Lower (background feature) |

## Use Cases

### Copy/Cut Region

```demo
use armas::components::audio::SelectionRange;

let mut selection_start = 1.0;
let mut selection_end = 3.0;

let response = SelectionRange::new(&mut selection_start, &mut selection_end)
    .id("copy_cut_region")
    .beat_width(40.0)
    .measures(4)
    .height(60.0)
    .snap_to_grid(true)
    .show(ui);

ui.horizontal(|ui| {
    if ui.button("Copy").clicked() && response.region_clicked {
        ui.label(format!("Copied: {:.1} - {:.1}", selection_start, selection_end));
    }
    if ui.button("Cut").clicked() && response.region_clicked {
        ui.label(format!("Cut: {:.1} - {:.1}", selection_start, selection_end));
    }
});
```

### Delete Range

```demo
use armas::components::audio::SelectionRange;

let mut selection_start = 2.0;
let mut selection_end = 4.0;

ui.label("Delete Region:");
SelectionRange::new(&mut selection_start, &mut selection_end)
    .id("delete_range")
    .beat_width(40.0)
    .measures(4)
    .height(60.0)
    .color(egui::Color32::from_rgb(255, 100, 100))  // Red for delete
    .snap_to_grid(true)
    .show(ui);

if ui.button("Delete Selection").clicked() {
    ui.label(format!("Deleting: {:.1} - {:.1}", selection_start, selection_end));
}
```

### Time Range Selection

```demo
use armas::components::audio::SelectionRange;

let mut selection_start = 0.0;
let mut selection_end = 4.0;

ui.label("Select Time Range:");
SelectionRange::new(&mut selection_start, &mut selection_end)
    .id("time_range_selection")
    .beat_width(40.0)
    .measures(4)
    .height(60.0)
    .snap_to_grid(false)  // Free selection
    .show(ui);

let duration = selection_end - selection_start;
ui.label(format!("Duration: {:.2} beats ({:.2} bars)", duration, duration / 4.0));
```

## Integration with Timeline

The selection range is designed to work alongside the Timeline and LoopRegionMarker:

```demo
use armas::components::audio::{SelectionRange, LoopRegionMarker, Timeline};

let mut loop_start = 0.0;
let mut loop_end = 8.0;
let mut selection_start = 2.0;
let mut selection_end = 4.0;

ui.vertical(|ui| {
    // Both markers use the same parameters as Timeline
    let beat_width = 40.0;
    let measures = 8;

    // Loop marker (background feature)
    LoopRegionMarker::new(&mut loop_start, &mut loop_end)
        .id("timeline_loop")
        .beat_width(beat_width)
        .measures(measures)
        .height(50.0)
        .show(ui);

    ui.add_space(2.0);

    // Selection marker (foreground editing feature)
    SelectionRange::new(&mut selection_start, &mut selection_end)
        .id("timeline_selection")
        .beat_width(beat_width)
        .measures(measures)
        .height(60.0)
        .snap_to_grid(true)
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
- **Timeline**: Main timeline component
- **TimeRuler**: Shows beat/bar markers
- **Playhead**: Current playback position
- **TimelineTrack**: Track component for regions
