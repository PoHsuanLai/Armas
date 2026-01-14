# Timeline

Complete DAW-style timeline with tracks, markers, rulers, and playhead. Supports folder tracks, loop regions, markers, tempo/time signature changes, and more.

## Basic Usage

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Drums", egui::Color32::from_rgb(255, 100, 100))
        .region(Region::new("Kick", 0.0, 4.0)),
    Track::new("Bass", egui::Color32::from_rgb(100, 255, 100))
        .region(Region::new("Bassline", 0.0, 4.0)),
];

let mut playhead_pos = 0.0;

Timeline::new()
    .track_header_width(150.0)
    .track_height(60.0)
    .beat_width(50.0)
    .measures(4)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);
```

## With Markers

Timeline supports markers for navigation, tempo changes, loop regions, and more:

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Vocals", egui::Color32::from_rgb(255, 100, 100))
        .region(Region::new("Verse", 0.0, 2.0))
        .region(Region::new("Chorus", 3.0, 2.0)),
    Track::new("Guitar", egui::Color32::from_rgb(255, 200, 50))
        .region(Region::new("Riff", 0.0, 4.0)),
];

let mut playhead_pos = 0.0;

// Unified marker data (cue points, tempo, time signatures)
let mut markers = vec![
    MarkerData::new(0.0, "Intro"),
    MarkerData::tempo(1.0, 120.0),
    MarkerData::new(2.0, "Verse"),
    MarkerData::new(3.0, "Chorus"),
];

let mut loop_region = LoopRegionData::new(2.0, 4.0);

Timeline::new()
    .markers(&mut markers)
    .loop_region(&mut loop_region)
    .beat_width(50.0)
    .measures(4)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);
```

## All Marker Types

Timeline supports ruler markers (cue points, tempo, time signatures) and region markers (loop, selection, punch):

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Track 1", egui::Color32::from_rgb(100, 180, 255))
        .region(Region::new("Clip", 0.0, 8.0)),
];

let mut playhead_pos = 0.0;

// Unified ruler markers (cue points, tempo, time signatures)
let mut markers = vec![
    MarkerData::new(0.0, "Intro"),               // Cue point (top third)
    MarkerData::tempo(0.0, 120.0),                // Tempo (middle third)
    MarkerData::time_signature(0.0, 4, 4),        // Time sig (bottom third)
    MarkerData::new(4.0, "Verse"),
    MarkerData::tempo(4.0, 140.0),
];

// Loop region (for playback looping)
let mut loop_region = LoopRegionData::new(2.0, 6.0);

// Selection range (for editing)
let mut selection_range = SelectionRangeData::new(4.0, 8.0);

// Punch region (for recording)
let mut punch_region = PunchRegionData::new(3.0, 7.0);

Timeline::new()
    .markers(&mut markers)
    .loop_region(&mut loop_region)
    .selection_range(&mut selection_range)
    .punch_region(&mut punch_region)
    .beat_width(40.0)
    .measures(8)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);
```

## With Snap Grid

Enable snap grid for visual alignment:

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Track 1", egui::Color32::from_rgb(255, 150, 100))
        .region(Region::new("Clip", 0.0, 4.0)),
];

let mut playhead_pos = 0.0;

Timeline::new()
    .show_snap_grid(true)
    .snap_grid_subdivision(4)  // 16th notes
    .beat_width(60.0)
    .measures(4)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);
```

## Folder Tracks

Organize tracks hierarchically with folders:

```demo
let theme = ui.ctx().armas_theme();

let mut tracks = vec![
    Track::new_folder("Vocals", egui::Color32::from_rgb(255, 100, 100))
        .child(
            Track::new("Lead Vocal", egui::Color32::from_rgb(255, 120, 120))
                .region(Region::new("Verse 1", 0.0, 4.0))
        )
        .child(
            Track::new("Backing Vocal", egui::Color32::from_rgb(255, 140, 140))
                .region(Region::new("Harmonies", 4.0, 4.0))
        ),
    Track::new("Bass", egui::Color32::from_rgb(100, 150, 255))
        .region(Region::new("Bassline", 0.0, 8.0)),
];

let mut playhead_pos = 0.0;

Timeline::new()
    .track_header_width(160.0)
    .beat_width(45.0)
    .measures(8)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);
```

## Zoom Control Integration

Combine with ZoomControl for interactive zooming:

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Track 1", egui::Color32::from_rgb(100, 180, 255))
        .region(Region::new("Clip", 0.0, 4.0)),
];

let mut playhead_pos = 0.0;
let mut zoom = 1.0;

// Zoom control
ZoomControl::new(&mut zoom)
    .id("timeline_zoom")
    .min_zoom(0.5)
    .max_zoom(2.0)
    .show(ui);

ui.add_space(8.0);

// Apply zoom to beat_width
let zoomed_beat_width = 50.0 * zoom;

Timeline::new()
    .beat_width(zoomed_beat_width)
    .measures(4)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);
```

## Scrolling to Position

Use `.scroll_to_beat()` to follow playhead or jump to positions:

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Track 1", egui::Color32::from_rgb(255, 100, 100))
        .region(Region::new("Clip A", 0.0, 2.0))
        .region(Region::new("Clip B", 8.0, 2.0)),
];

// Use persisted state for playhead position
let mut playhead_pos = ui.ctx().data_mut(|d| {
    d.get_persisted(egui::Id::new("playhead_pos")).unwrap_or(10.0)
});

ui.horizontal(|ui| {
    if ui.button("Jump to Start").clicked() {
        playhead_pos = 0.0;
    }
    if ui.button("Jump to Beat 8").clicked() {
        playhead_pos = 8.0;
    }
});

ui.add_space(8.0);

Timeline::new()
    .beat_width(50.0)
    .measures(16)
    .scroll_to_beat(playhead_pos)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);

// Persist playhead position for next frame
ui.ctx().data_mut(|d| {
    d.insert_persisted(egui::Id::new("playhead_pos"), playhead_pos);
});
```

## Handling Interactions

Timeline returns detailed interaction information:

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Track 1", egui::Color32::from_rgb(255, 150, 100))
        .region(Region::new("Clip A", 0.0, 2.0)),
    Track::new("Track 2", egui::Color32::from_rgb(100, 200, 255))
        .region(Region::new("Clip B", 1.0, 3.0)),
];

let mut playhead_pos = 0.0;

let response = Timeline::new()
    .beat_width(50.0)
    .measures(4)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);

ui.add_space(8.0);
if let Some(track_idx) = response.track_clicked {
    ui.label(format!("Track clicked: {}", tracks[track_idx].name));
}
if let Some((track_idx, region_idx)) = response.region_clicked {
    ui.label(format!("Region clicked: {}", tracks[track_idx].regions[region_idx].name));
}
if let Some((track_idx, beat_pos)) = response.empty_clicked {
    ui.label(format!("Empty area clicked: Track {}, Beat {:.2}", track_idx, beat_pos));
}
if response.playhead_moved {
    ui.label(format!("Playhead moved to: {:.2}", playhead_pos));
}
```

## API Reference

### Timeline Constructor

```rust
Timeline::new() -> Self
```

Creates a new timeline with default settings.

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.id()` | `impl Into<egui::Id>` | Auto | Custom ID (important for multiple timelines) |
| `.track_header_width()` | `f32` | `200.0` | Width of track header column |
| `.track_height()` | `f32` | `80.0` | Height of each track row |
| `.beat_width()` | `f32` | `60.0` | Pixels per beat (zoom level) |
| `.measures()` | `u32` | `16` | Number of measures to display |
| `.beats_per_measure()` | `u32` | `4` | Time signature numerator |
| `.ruler_height()` | `f32` | `40.0` | Height of time ruler at top |
| `.show_playhead()` | `bool` | `true` | Show playhead indicator |
| `.playhead_color()` | `Color32` | `theme.primary()` | Playhead color |
| `.scroll_to_beat()` | `f32` | `None` | Scroll to show this beat position |
| `.markers()` | `&mut Vec<MarkerData>` | `None` | Unified markers (cue points, tempo, time sigs) |
| `.loop_region()` | `&mut LoopRegionData` | `None` | Loop region |
| `.selection_range()` | `&mut SelectionRangeData` | `None` | Selection range |
| `.punch_region()` | `&mut PunchRegionData` | `None` | Punch in/out region |
| `.show_snap_grid()` | `bool` | `false` | Show snap grid |
| `.snap_grid_subdivision()` | `u32` | `4` | Snap grid subdivision (lines per beat) |

### Show Method

```rust
pub fn show(
    self,
    ui: &mut Ui,
    tracks: &mut Vec<Track>,
    playhead_position: &mut f32,
    theme: &Theme,
) -> TimelineResponse
```

### TimelineResponse

```rust
pub struct TimelineResponse {
    pub response: Response,
    pub track_clicked: Option<usize>,
    pub region_clicked: Option<(usize, usize)>,
    pub empty_clicked: Option<(usize, f32)>,
    pub playhead_moved: bool,
    pub playhead_position: f32,
}
```

### Track Data Structures

```rust
// Create tracks
Track::new(name: impl Into<String>, color: Color32) -> Self
Track::new_folder(name: impl Into<String>, color: Color32) -> Self

// Builder methods
.region(region: Region)
.regions(regions: Vec<Region>)
.child(child: Track)  // For folders
.children(children: Vec<Track>)  // For folders
.collapsed(collapsed: bool)  // For folders
```

### Marker Data Structures

```rust
// Unified ruler markers (automatically positioned by type)
MarkerData::new(position: f32, label: impl Into<String>)  // Cue point (blue, top third)
    .color(color: Color32)

MarkerData::tempo(position: f32, bpm: f32)  // Tempo marker (teal, middle third)
    .color(color: Color32)

MarkerData::time_signature(position: f32, numerator: u32, denominator: u32)  // Time sig (purple, bottom third)
    .color(color: Color32)

// Region markers
LoopRegionData::new(start: f32, end: f32)

SelectionRangeData::new(start: f32, end: f32)

PunchRegionData::new(punch_in: f32, punch_out: f32)
```

## Visual Design

### Layout Structure

```
┌─────────────┬─────────────────────────────────────┐
│             │         Time Ruler + Markers        │
│   (empty)   │  (horizontal scroll)                │
├─────────────┼─────────────────────────────────────┤
│   Track     │                                     │
│   Header 1  │   Timeline Track 1 (regions)        │
├─────────────┼─────────────────────────────────────┤
│   Track     │                                     │
│   Header 2  │   Timeline Track 2 (regions)        │
├─────────────┼─────────────────────────────────────┤
│     ...     │          ... (scrollable)           │
└─────────────┴─────────────────────────────────────┘
```

### Marker Visual Hierarchy

Markers are rendered as overlays in the ruler area. The unified marker system positions different marker types vertically to prevent overlapping:

- **Ruler Markers** (vertical thirds):
  - Top third: Cue points (blue) - "Intro", "Chorus", etc.
  - Middle third: Tempo markers (teal) - "120 BPM", "140 BPM"
  - Bottom third: Time signatures (purple) - "4/4", "3/4", "7/8"

- **Region Markers** (vertical halves in track area):
  - Top half: Loop regions (yellow/secondary)
  - Middle third: Selection ranges (gray)
  - Bottom half: Punch regions (red)

- **Interactions**: All markers are draggable with snap-to-grid support
- **Height**: Ruler area is 40px tall by default, markers occupy their designated thirds

### Components Used Internally

- **TimeRuler**: Bar and beat numbers
- **TrackHeader**: Track name and controls (M/S/R)
- **TimelineTrack**: Audio/MIDI regions
- **Playhead**: Vertical line with time display
- **Marker**: Unified component for cue points, tempo, and time signatures
- **SnapGrid**: Visual alignment grid
- **LoopRegionMarker**, **SelectionRange**, **PunchMarker**: Region markers

## Use Cases

### Full DAW Arrangement View

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Lead Vocals", egui::Color32::from_rgb(255, 100, 100))
        .regions(vec![
            Region::new("Verse 1", 0.0, 4.0),
            Region::new("Chorus", 4.0, 3.0),
            Region::new("Verse 2", 8.0, 4.0),
        ]),
    Track::new("Guitar", egui::Color32::from_rgb(255, 200, 50))
        .region(Region::new("Strumming", 0.0, 8.0)),
    Track::new("Bass", egui::Color32::from_rgb(100, 150, 255))
        .region(Region::new("Bassline", 0.0, 8.0)),
];

let mut playhead_pos = 0.0;

let mut markers = vec![
    MarkerData::new(0.0, "Verse 1"),
    MarkerData::new(4.0, "Chorus"),
];

let mut loop_region = LoopRegionData::new(4.0, 7.0);

Timeline::new()
    .markers(&mut markers)
    .loop_region(&mut loop_region)
    .show_snap_grid(true)
    .track_header_width(140.0)
    .track_height(50.0)
    .beat_width(45.0)
    .measures(8)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);
```

## Performance

- Efficient scrolling with egui's ScrollArea
- Visibility culling for tracks and regions
- Synchronized scroll state
- Handles 50+ tracks smoothly
- Minimal allocations during rendering

## Related Components

- **Marker**: Unified ruler markers (cue points, tempo, time signatures)
- **LoopRegionMarker**: Loop regions for playback
- **SelectionRange**: Selection ranges for editing
- **PunchMarker**: Punch in/out regions for recording
- **SnapGrid**: Visual alignment grid
- **ZoomControl**: Zoom UI control
- **TimeRuler**: Bar/beat display
- **Playhead**: Position indicator
- **TrackHeader**: Track controls (M/S/R)
- **TimelineTrack**: Audio/MIDI region display
