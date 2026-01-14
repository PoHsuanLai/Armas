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
    .id(ui.id().with("basic_timeline"))
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
    .id(ui.id().with("markers_timeline"))
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
    .id(ui.id().with("all_markers_timeline"))
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
    .id(ui.id().with("snap_grid_timeline"))
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
    .id(ui.id().with("folder_tracks_timeline"))
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
    .id(ui.id().with("zoom_control_timeline"))
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
    .id(ui.id().with("scroll_to_beat_timeline"))
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

// Use persisted state for interaction feedback
let last_interaction_id = ui.id().with("last_interaction");
let mut last_interaction: String = ui.ctx().data_mut(|d| {
    d.get_persisted(last_interaction_id).unwrap_or_default()
});

let response = Timeline::new()
    .id(ui.id().with("event_handling_timeline"))
    .beat_width(50.0)
    .measures(4)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);

// Capture interactions and persist for display
if let Some(track_idx) = response.track_clicked {
    last_interaction = format!("Track clicked: {}", tracks[track_idx].name);
}
if let Some(track_idx) = response.track_mute_clicked {
    last_interaction = format!("Mute toggled on: {}", tracks[track_idx].name);
}
if let Some(track_idx) = response.track_solo_clicked {
    last_interaction = format!("Solo toggled on: {}", tracks[track_idx].name);
}
if let Some(track_idx) = response.track_arm_clicked {
    last_interaction = format!("Record arm toggled on: {}", tracks[track_idx].name);
}
if let Some((track_idx, region_idx)) = response.region_clicked {
    last_interaction = format!("Region clicked: {}", tracks[track_idx].regions[region_idx].name);
}
if let Some((track_idx, beat_pos)) = response.empty_clicked {
    last_interaction = format!("Empty area clicked: Track {}, Beat {:.2}", track_idx, beat_pos);
}
if response.playhead_moved {
    last_interaction = format!("Playhead moved to: {:.2}", playhead_pos);
}

// Persist feedback state
ui.ctx().data_mut(|d| {
    d.insert_persisted(last_interaction_id, last_interaction.clone());
});

// Display persisted feedback
ui.add_space(8.0);
if !last_interaction.is_empty() {
    ui.label(format!("Last interaction: {}", last_interaction));
} else {
    ui.colored_label(egui::Color32::GRAY, "Last interaction: None yet");
}
```

## Region Selection

Timeline supports interactive selection ranges for common DAW features:

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Track 1", egui::Color32::from_rgb(100, 180, 255))
        .region(Region::new("Clip", 0.0, 8.0)),
];

let mut playhead_pos = 2.0;

// Selection range markers for timeline editing
let mut selection_range = SelectionRangeData { start: 1.0, end: 3.0 };
let mut loop_region = LoopRegionData { start: 4.0, end: 6.0 };
let mut punch_region = PunchRegionData { punch_in: 0.5, punch_out: 7.5 };

Timeline::new()
    .id(ui.id().with("region_selection_timeline"))
    .beat_width(50.0)
    .measures(4)
    .show_snap_grid(true)
    // Add region markers to timeline
    .selection_range(&mut selection_range)
    .loop_region(&mut loop_region)
    .punch_region(&mut punch_region)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);

// Display current range values
ui.add_space(8.0);
ui.horizontal(|ui| {
    ui.label("Selection:");
    ui.label(format!("{:.1} - {:.1}", selection_range.start, selection_range.end));
});
ui.horizontal(|ui| {
    ui.label("Loop:");
    ui.label(format!("{:.1} - {:.1}", loop_region.start, loop_region.end));
});
ui.horizontal(|ui| {
    ui.label("Punch:");
    ui.label(format!("{:.1} - {:.1}", punch_region.punch_in, punch_region.punch_out));
});
```

## Track Selection

Tracks can be selected by clicking on the track header. Selected tracks are indicated by a colored border:

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Track 1", egui::Color32::from_rgb(100, 180, 255))
        .region(Region::new("Clip", 0.0, 4.0)),
    Track::new("Track 2", egui::Color32::from_rgb(255, 150, 100))
        .region(Region::new("Clip", 0.0, 4.0)),
    Track::new("Track 3", egui::Color32::from_rgb(100, 255, 150))
        .region(Region::new("Clip", 0.0, 4.0)),
];

let mut playhead_pos = 0.0;

Timeline::new()
    .id(ui.id().with("track_selection_timeline"))
    .beat_width(50.0)
    .measures(4)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);

// Display selected tracks
ui.add_space(8.0);
let selected_tracks: Vec<_> = tracks.iter()
    .enumerate()
    .filter(|(_, track)| track.selected)
    .map(|(idx, track)| format!("{} (Track {})", track.name, idx + 1))
    .collect();

if selected_tracks.is_empty() {
    ui.colored_label(egui::Color32::GRAY, "No tracks selected - click a track header to select");
} else {
    ui.label(format!("Selected tracks: {}", selected_tracks.join(", ")));
}
```

## Advanced Features

### Auto-Follow Playhead

Enable automatic scrolling to keep the playhead visible during playback:

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Track 1", egui::Color32::from_rgb(100, 180, 255))
        .region(Region::new("Clip", 0.0, 16.0)),
];

let mut playhead_pos = 0.0;

// Simulate playback progression
playhead_pos += 0.1;

Timeline::new()
    .id(ui.id().with("auto_follow_timeline"))
    .auto_follow_playhead(true)         // Enable auto-follow
    .auto_follow_margin(0.25)           // Keep playhead at 25% from left
    .beat_width(50.0)
    .measures(16)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);

ui.label(format!("Playhead position: {:.1} beats", playhead_pos));
```

### Zoom Control Integration

Set zoom limits and allow interactive zooming:

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Track 1", egui::Color32::from_rgb(255, 150, 100))
        .region(Region::new("Clip", 0.0, 8.0)),
];

let mut playhead_pos = 0.0;
let mut zoom = 1.0;

// Zoom control UI
ui.horizontal(|ui| {
    ui.label("Zoom:");
    ui.add(egui::Slider::new(&mut zoom, 0.5..=2.0).text("x"));
});

ui.add_space(8.0);

let zoomed_beat_width = 50.0 * zoom;

Timeline::new()
    .id(ui.id().with("zoom_timeline"))
    .beat_width(zoomed_beat_width)
    .min_zoom(0.5)      // Prevent zooming beyond 0.5x
    .max_zoom(2.0)      // Prevent zooming beyond 2.0x
    .measures(8)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);
```

### Empty State Message

Show helpful message when no tracks exist:

```demo
let theme = ui.ctx().armas_theme();
let mut tracks: Vec<Track> = vec![];  // Empty track list

let mut playhead_pos = 0.0;

Timeline::new()
    .id(ui.id().with("empty_timeline"))
    .empty_message("No tracks yet. Click '+' to add a track.")
    .beat_width(50.0)
    .measures(4)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);
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
| `.min_zoom()` | `f32` | `0.5` | Minimum zoom level (beat_width multiplier) |
| `.max_zoom()` | `f32` | `2.0` | Maximum zoom level (beat_width multiplier) |
| `.auto_follow_playhead()` | `bool` | `false` | Auto-scroll to keep playhead visible |
| `.auto_follow_margin()` | `f32` | `0.25` | Margin percent for auto-follow (0.0-1.0) |
| `.visible_render_margin()` | `f32` | `2.0` | Render margin outside viewport (in beats) |
| `.empty_message()` | `impl Into<String>` | `None` | Message to show when no tracks exist |

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
    pub response: Response,                                  // Base egui Response
    pub track_clicked: Option<usize>,                       // Which track was clicked
    pub track_mute_clicked: Option<usize>,                  // Mute button clicked on track
    pub track_solo_clicked: Option<usize>,                  // Solo button clicked on track
    pub track_arm_clicked: Option<usize>,                   // Record arm button clicked on track
    pub track_collapse_clicked: Option<usize>,              // Collapse/expand button clicked (folder tracks)
    pub region_clicked: Option<(usize, usize)>,             // (track_idx, region_idx) if clicked
    pub empty_clicked: Option<(usize, f32)>,                // (track_idx, beat_pos) if clicked
    pub playhead_moved: bool,                               // Playhead was dragged
    pub playhead_clicked: bool,                             // Playhead was clicked (not just moved)
    pub playhead_position: f32,                             // Current playhead position in beats
    pub marker_moved: Option<usize>,                        // Which marker was moved (if any)
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

// Region markers - interactive ranges on the timeline
LoopRegionData::new(start: f32, end: f32)           // Yellow loop bracket
SelectionRangeData::new(start: f32, end: f32)       // Gray selection bracket
PunchRegionData::new(punch_in: f32, punch_out: f32) // Red punch in/out bracket
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
    .id(ui.id().with("complete_example_timeline"))
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
