# Marker

Unified visual markers for important positions in the timeline. Displays a vertical line with a badge for marking:
- **Cue Points**: Song sections (Intro, Verse, Chorus, Bridge, Outro)
- **Tempo Changes**: BPM automation points (e.g., "120 BPM")
- **Time Signatures**: Meter changes (e.g., "3/4", "7/8")

The marker type is automatically detected based on the content string.

## Basic Usage - Cue Point

```demo
use armas_audio::Marker;

let mut position = 16.0; // beats

egui::ScrollArea::horizontal().show(ui, |ui| {
    let response = Marker::new(&mut position, "Chorus")
        .beat_width(60.0)
        .measures(16)
        .show(ui);

    if response.position_changed {
        ui.label(format!("Marker position: {:.1}", position));
    }
});
```

## Usage with Timeline (Recommended)

```demo
use armas_audio::{Timeline, MarkerData, Track};

let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Track 1", egui::Color32::from_rgb(100, 180, 255)),
];
let mut playhead_pos = 0.0;

let mut markers = vec![
    MarkerData::new(0.0, "Intro"),
    MarkerData::tempo(16.0, 120.0),
    MarkerData::time_signature(32.0, 3, 4),
    MarkerData::new(48.0, "Chorus"),
];

Timeline::new()
    .beat_width(50.0)
    .measures(16)
    .markers(&mut markers)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);
```

The Timeline automatically handles vertical positioning:
- Cue points (top third)
- Tempo markers (middle third)
- Time signature markers (bottom third)

## Multiple Markers

```demo
use armas_audio::Marker;

let mut intro_pos = 0.0;
let mut verse_pos = 8.0;
let mut chorus_pos = 16.0;
let mut bridge_pos = 32.0;

egui::ScrollArea::horizontal().show(ui, |ui| {
    ui.horizontal(|ui| {
        Marker::new(&mut intro_pos, "Intro")
            .beat_width(40.0)
            .measures(16)
            .color(egui::Color32::from_rgb(100, 200, 100))
            .id("intro")
            .show(ui);

        Marker::new(&mut verse_pos, "Verse")
            .beat_width(40.0)
            .measures(16)
            .color(egui::Color32::from_rgb(70, 140, 230))
            .id("verse")
            .show(ui);

        Marker::new(&mut chorus_pos, "Chorus")
            .beat_width(40.0)
            .measures(16)
            .color(egui::Color32::from_rgb(230, 140, 70))
            .id("chorus")
            .show(ui);

        Marker::new(&mut bridge_pos, "Bridge")
            .beat_width(40.0)
            .measures(16)
            .color(egui::Color32::from_rgb(200, 100, 200))
            .id("bridge")
            .show(ui);
    });
});
```

## With Custom Colors

```demo
use armas_audio::Marker;

let mut position = 8.0;

egui::ScrollArea::horizontal().show(ui, |ui| {
    Marker::new(&mut position, "Drop")
        .beat_width(50.0)
        .measures(16)
        .color(egui::Color32::from_rgb(255, 100, 100))
        .show(ui);
});
```

## Non-Draggable Marker

```demo
use armas_audio::Marker;

let mut position = 12.0;

egui::ScrollArea::horizontal().show(ui, |ui| {
    Marker::new(&mut position, "Fixed Point")
        .beat_width(50.0)
        .measures(16)
        .draggable(false)
        .show(ui);
});

ui.label("This marker cannot be dragged");
```

## Without Vertical Line

```demo
use armas_audio::Marker;

let mut position = 4.0;

egui::ScrollArea::horizontal().show(ui, |ui| {
    Marker::new(&mut position, "Badge Only")
        .beat_width(50.0)
        .measures(16)
        .show_line(false)
        .show(ui);
});
```

## With Custom Tooltip

```demo
use armas_audio::Marker;

let mut position = 16.0;

egui::ScrollArea::horizontal().show(ui, |ui| {
    Marker::new(&mut position, "Chorus")
        .beat_width(50.0)
        .measures(16)
        .tooltip("Main chorus section\nDuration: 8 bars\nKey: C Major")
        .show(ui);
});

ui.label("Hover over the marker to see custom tooltip");
```

## With Snap to Grid

```demo
use armas_audio::Marker;

let mut position = 7.5;

egui::ScrollArea::horizontal().show(ui, |ui| {
    Marker::new(&mut position, "Verse 2")
        .beat_width(50.0)
        .measures(16)
        .snap_to_grid(true)
        .grid_division(1.0)  // Snap to whole beats
        .show(ui);
});
```

## Fine Grid Snapping

```demo
use armas_audio::Marker;

let mut position = 8.25;

egui::ScrollArea::horizontal().show(ui, |ui| {
    Marker::new(&mut position, "Fill")
        .beat_width(50.0)
        .measures(16)
        .snap_to_grid(true)
        .grid_division(0.25)  // Snap to 16th notes
        .show(ui);
});
```

## Song Structure Example

```demo
use armas_audio::Marker;

let mut intro = 0.0;
let mut verse1 = 8.0;
let mut chorus1 = 24.0;
let mut verse2 = 40.0;
let mut chorus2 = 56.0;
let mut bridge = 72.0;
let mut chorus3 = 88.0;
let mut outro = 104.0;

ui.vertical(|ui| {
    ui.label("Song Structure:");

    egui::ScrollArea::horizontal().show(ui, |ui| {
        ui.horizontal(|ui| {
            Marker::new(&mut intro, "Intro")
                .beat_width(30.0)
                .measures(32)
                .color(egui::Color32::from_rgb(150, 150, 150))
                .id("intro_marker")
                .show(ui);

            Marker::new(&mut verse1, "Verse 1")
                .beat_width(30.0)
                .measures(32)
                .color(egui::Color32::from_rgb(70, 140, 230))
                .id("verse1_marker")
                .show(ui);

            Marker::new(&mut chorus1, "Chorus 1")
                .beat_width(30.0)
                .measures(32)
                .color(egui::Color32::from_rgb(230, 140, 70))
                .id("chorus1_marker")
                .show(ui);

            Marker::new(&mut verse2, "Verse 2")
                .beat_width(30.0)
                .measures(32)
                .color(egui::Color32::from_rgb(70, 140, 230))
                .id("verse2_marker")
                .show(ui);

            Marker::new(&mut chorus2, "Chorus 2")
                .beat_width(30.0)
                .measures(32)
                .color(egui::Color32::from_rgb(230, 140, 70))
                .id("chorus2_marker")
                .show(ui);

            Marker::new(&mut bridge, "Bridge")
                .beat_width(30.0)
                .measures(32)
                .color(egui::Color32::from_rgb(200, 100, 200))
                .id("bridge_marker")
                .show(ui);

            Marker::new(&mut chorus3, "Chorus 3")
                .beat_width(30.0)
                .measures(32)
                .color(egui::Color32::from_rgb(230, 140, 70))
                .id("chorus3_marker")
                .show(ui);

            Marker::new(&mut outro, "Outro")
                .beat_width(30.0)
                .measures(32)
                .color(egui::Color32::from_rgb(150, 150, 150))
                .id("outro_marker")
                .show(ui);
        });
    });
});
```

## Disabled State

```demo
use armas_audio::Marker;

let mut position = 16.0;

egui::ScrollArea::horizontal().show(ui, |ui| {
    Marker::new(&mut position, "Disabled")
        .beat_width(50.0)
        .measures(16)
        .enabled(false)
        .show(ui);
});

ui.label("Marker is disabled - not visible");
```

## MarkerData API (Timeline Integration)

### Constructors

```rust
// Cue point marker (blue, top third in ruler)
MarkerData::new(position: f32, label: impl Into<String>) -> Self

// Tempo marker (teal, middle third in ruler)
MarkerData::tempo(position: f32, bpm: f32) -> Self

// Time signature marker (purple, bottom third in ruler)
MarkerData::time_signature(position: f32, numerator: u32, denominator: u32) -> Self
```

### MarkerData Builder Methods

```rust
// Set custom color (overrides default)
.color(color: Color32) -> Self
```

### Example

```demo
use armas_audio::MarkerData;

let markers = vec![
    MarkerData::new(0.0, "Intro")
        .color(egui::Color32::from_rgb(150, 150, 150)),
    MarkerData::tempo(16.0, 140.0),
    MarkerData::time_signature(32.0, 3, 4),
    MarkerData::new(48.0, "Chorus")
        .color(egui::Color32::from_rgb(230, 140, 70)),
];
```

## API Reference (Direct Component Usage)

### Constructor

```rust
Marker::new(position: &mut f32, content: impl Into<String>) -> Self
```

Creates a new marker at a specific beat position with content text.

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.id()` | `impl Into<egui::Id>` | None | Unique ID for state persistence |
| `.beat_width()` | `f32` | `60.0` | Pixels per beat (must match Timeline) |
| `.measures()` | `u32` | `16` | Number of measures to display |
| `.beats_per_measure()` | `u32` | `4` | Beats per measure |
| `.enabled()` | `bool` | `true` | Enable or disable the marker |
| `.draggable()` | `bool` | `true` | Whether the marker can be dragged |
| `.snap_to_grid()` | `bool` | `true` | Enable snap to grid when dragging |
| `.grid_division()` | `f32` | `1.0` | Grid division for snapping (beats) |
| `.color()` | `Color32` | Blue `(70, 140, 230)` | Custom color for the marker |
| `.show_line()` | `bool` | `true` | Show or hide the vertical line |
| `.show_tooltip()` | `bool` | `true` | Show tooltip on hover |
| `.tooltip()` | `impl Into<String>` | Position info | Custom tooltip text |
| `.vertical_range()` | `(f32, f32)` | `(0.0, 1.0)` | Vertical position range (top%, bottom%) from 0.0-1.0 |

### Show Method

```rust
pub fn show(self, ui: &mut egui::Ui) -> MarkerResponse
```

Returns a `MarkerResponse` with interaction data.

### MarkerResponse

```rust
pub struct MarkerResponse {
    pub response: Response,
    pub position_changed: bool,
    pub clicked: bool,
    pub hovered: bool,
}
```

- `response`: The egui Response for the component
- `position_changed`: True if marker was dragged to a new position
- `clicked`: True if marker badge was clicked
- `hovered`: True if marker badge is hovered

## Visual Design

### Badge
- Compact label at top of marker
- Rounded corners with padding
- White text on colored background
- Glow effect on hover (3 layers)
- Default blue color (70, 140, 230)
- Height: 18px with 8px horizontal padding

### Vertical Line
- 2px width
- Extends from below badge to bottom of component
- Same color as badge
- Can be hidden with `.show_line(false)`
- Total component height: 80px

### Tooltip
- Shows on hover by default
- Displays position in beats and bars
- Can be customized with `.tooltip()`
- Can be disabled with `.show_tooltip(false)`

## Interaction

- **Drag badge**: Move marker to new position (if draggable)
- **Click badge**: Triggers clicked event
- **Hover badge**: Shows tooltip and glow effect
- **Snap to grid**: When enabled, marker snaps to grid divisions
- **State persistence**: Position saved when ID is set

## Common Color Schemes

### Song Sections
- **Intro/Outro**: Gray `(150, 150, 150)`
- **Verse**: Blue `(70, 140, 230)`
- **Chorus**: Orange `(230, 140, 70)`
- **Bridge**: Purple `(200, 100, 200)`
- **Pre-Chorus**: Teal `(70, 200, 200)`
- **Break/Drop**: Red `(230, 70, 70)`

### Production Markers
- **Mix Reference**: Green `(100, 200, 100)`
- **Edit Point**: Yellow `(230, 230, 70)`
- **Problem Area**: Red `(230, 70, 70)`
- **Good Take**: Green `(70, 230, 70)`

## Use Cases

### Navigation
```demo
use armas_audio::Marker;

let mut current_section = 0.0;

egui::ScrollArea::horizontal().show(ui, |ui| {
    let response = Marker::new(&mut current_section, "Current")
        .beat_width(50.0)
        .measures(16)
        .color(egui::Color32::from_rgb(100, 255, 100))
        .show(ui);

    if response.clicked {
        ui.label("Jump to this position!");
    }
});
```

### Arrangement Markers
```demo
use armas_audio::Marker;

let mut drop_point = 32.0;

ui.label("Drop Point:");
egui::ScrollArea::horizontal().show(ui, |ui| {
    Marker::new(&mut drop_point, "DROP")
        .beat_width(40.0)
        .measures(16)
        .color(egui::Color32::from_rgb(255, 50, 50))
        .snap_to_grid(true)
        .grid_division(4.0)  // Snap to bars
        .show(ui);
});
```

### Reference Points
```demo
use armas_audio::Marker;

let mut ref_point = 16.0;

ui.label("Reference Mix Point:");
egui::ScrollArea::horizontal().show(ui, |ui| {
    Marker::new(&mut ref_point, "REF")
        .beat_width(40.0)
        .measures(16)
        .color(egui::Color32::from_rgb(100, 200, 255))
        .draggable(false)  // Fixed reference
        .show(ui);
});
```

## Integration with Timeline

Markers work alongside other timeline components:

```demo
use armas_audio::{Marker, LoopRegionMarker, SelectionRange};

let mut verse_pos = 8.0;
let mut chorus_pos = 24.0;
let mut loop_start = 8.0;
let mut loop_end = 24.0;
let mut selection_start = 16.0;
let mut selection_end = 20.0;

ui.vertical(|ui| {
    let beat_width = 40.0;
    let measures = 16;

    // Markers at top
    ui.label("Song Structure:");
    egui::ScrollArea::horizontal().id_salt("markers_scroll").show(ui, |ui| {
        ui.horizontal(|ui| {
            Marker::new(&mut verse_pos, "Verse")
                .beat_width(beat_width)
                .measures(measures)
                .color(egui::Color32::from_rgb(70, 140, 230))
                .id("verse_m")
                .show(ui);

            Marker::new(&mut chorus_pos, "Chorus")
                .beat_width(beat_width)
                .measures(measures)
                .color(egui::Color32::from_rgb(230, 140, 70))
                .id("chorus_m")
                .show(ui);
        });
    });

    ui.add_space(4.0);

    // Loop region
    ui.label("Loop:");
    egui::ScrollArea::horizontal().id_salt("loop_scroll").show(ui, |ui| {
        LoopRegionMarker::new(&mut loop_start, &mut loop_end)
            .beat_width(beat_width)
            .measures(measures)
            .id("loop_m")
            .show(ui);
    });

    ui.add_space(4.0);

    // Selection
    ui.label("Selection:");
    egui::ScrollArea::horizontal().id_salt("selection_scroll").show(ui, |ui| {
        SelectionRange::new(&mut selection_start, &mut selection_end)
            .beat_width(beat_width)
            .measures(measures)
            .id("selection_m")
            .show(ui);
    });
});
```

## Dependencies

- `egui = "0.33"`
- Theme colors: Auto-generated from marker color
- Minimum version: `armas 0.1.0`

## Related Components

- **LoopRegionMarker**: Playback loop regions
- **SelectionRange**: Editing selection regions
- **PunchMarker**: Recording punch in/out regions
- **Timeline**: Main timeline component
- **TimeRuler**: Shows beat/bar markers
- **Playhead**: Current playback position
