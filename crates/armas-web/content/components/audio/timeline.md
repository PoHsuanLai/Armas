# Timeline

Complete scrollable timeline view combining time ruler, track headers, and timeline tracks. Perfect for DAW arrangement views with multiple tracks and regions.

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

## Multiple Tracks with Regions

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Vocals", egui::Color32::from_rgb(255, 100, 100))
        .region(Region::new("Verse", 0.0, 2.0))
        .region(Region::new("Chorus", 3.0, 2.0)),
    Track::new("Guitar", egui::Color32::from_rgb(255, 200, 50))
        .region(Region::new("Riff A", 0.0, 3.0))
        .region(Region::new("Riff B", 4.0, 2.0)),
    Track::new("Synth", egui::Color32::from_rgb(100, 150, 255))
        .region(Region::new("Pad", 0.0, 4.0)),
];

let mut playhead_pos = 0.0;

Timeline::new()
    .track_header_width(120.0)
    .track_height(50.0)
    .beat_width(40.0)
    .measures(4)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);
```

## Custom Track Heights

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Drums", egui::Color32::from_rgb(255, 100, 100))
        .region(Region::new("Beat", 0.0, 4.0)),
    Track::new("Bass", egui::Color32::from_rgb(100, 255, 100))
        .region(Region::new("Groove", 0.0, 4.0)),
];

let mut playhead_pos = 0.0;

ui.vertical(|ui| {
    ui.label("Compact (40px tracks)");
    Timeline::new()
        .id("compact_timeline")
        .track_header_width(120.0)
        .track_height(40.0)
        .beat_width(50.0)
        .measures(4)
        .show(ui, &mut tracks.clone(), &mut playhead_pos, &theme);

    ui.add_space(12.0);

    ui.label("Standard (70px tracks)");
    Timeline::new()
        .id("standard_timeline")
        .track_header_width(120.0)
        .track_height(70.0)
        .beat_width(50.0)
        .measures(4)
        .show(ui, &mut tracks.clone(), &mut playhead_pos, &theme);
});
```

## Different Zoom Levels

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Track 1", egui::Color32::from_rgb(100, 180, 255))
        .region(Region::new("Clip", 0.0, 3.0)),
];

let mut playhead_pos = 0.0;

ui.vertical(|ui| {
    ui.label("Zoomed Out (30px per beat)");
    Timeline::new()
        .id("zoomed_out")
        .track_header_width(120.0)
        .track_height(50.0)
        .beat_width(30.0)
        .measures(4)
        .show(ui, &mut tracks.clone(), &mut playhead_pos, &theme);

    ui.add_space(8.0);

    ui.label("Zoomed In (70px per beat)");
    Timeline::new()
        .id("zoomed_in")
        .track_header_width(120.0)
        .track_height(50.0)
        .beat_width(70.0)
        .measures(4)
        .show(ui, &mut tracks.clone(), &mut playhead_pos, &theme);
});
```

## Handling Interactions

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Track 1", egui::Color32::from_rgb(255, 150, 100))
        .region(Region::new("Clip A", 0.0, 2.0))
        .region(Region::new("Clip B", 3.0, 2.0)),
    Track::new("Track 2", egui::Color32::from_rgb(100, 200, 255))
        .region(Region::new("Clip C", 1.0, 3.0)),
];

let mut playhead_pos = 0.0;

let response = Timeline::new()
    .track_header_width(120.0)
    .track_height(60.0)
    .beat_width(50.0)
    .measures(4)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);

ui.add_space(8.0);
if let Some(track_idx) = response.track_clicked {
    ui.label(format!("Track clicked: {} ({})", track_idx, tracks[track_idx].name));
}
if let Some((track_idx, region_idx)) = response.region_clicked {
    ui.label(format!("Region clicked: Track {}, Region {} ({})",
        track_idx, region_idx, tracks[track_idx].regions[region_idx].name));
}
if let Some((track_idx, beat_pos)) = response.empty_clicked {
    ui.label(format!("Empty area clicked: Track {}, Beat {:.2}", track_idx, beat_pos));
}
```

## Scrolling to Playhead Position

Use `.scroll_to_beat()` to programmatically scroll the timeline to show a specific beat position. This is useful for following playhead during playback or jumping to a specific location.

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Track 1", egui::Color32::from_rgb(255, 100, 100))
        .region(Region::new("Clip A", 0.0, 2.0))
        .region(Region::new("Clip B", 8.0, 2.0)),
    Track::new("Track 2", egui::Color32::from_rgb(100, 255, 100))
        .region(Region::new("Clip C", 4.0, 3.0))
        .region(Region::new("Clip D", 12.0, 2.0)),
];

let mut playhead_pos = 10.0; // Start at beat 10

Timeline::new()
    .track_header_width(120.0)
    .track_height(60.0)
    .beat_width(50.0)
    .measures(16)
    .scroll_to_beat(playhead_pos) // Scroll to show playhead
    .show(ui, &mut tracks, &mut playhead_pos, &theme);

ui.add_space(8.0);
ui.horizontal(|ui| {
    if ui.button("Jump to Start").clicked() {
        playhead_pos = 0.0;
    }
    if ui.button("Jump to Beat 8").clicked() {
        playhead_pos = 8.0;
    }
    if ui.button("Jump to Beat 16").clicked() {
        playhead_pos = 16.0;
    }
});
```

## Folder Tracks

Create hierarchical track structures with collapsible folders to organize your arrangement. Folder tracks can contain child tracks that are shown/hidden with collapse/expand controls.

```demo
let theme = ui.ctx().armas_theme();

let mut tracks = vec![
    Track::new_folder("Vocals", egui::Color32::from_rgb(255, 100, 100))
        .child(
            Track::new("Lead Vocal", egui::Color32::from_rgb(255, 120, 120))
                .region(Region::new("Verse 1", 0.0, 4.0))
                .region(Region::new("Chorus", 4.0, 4.0))
        )
        .child(
            Track::new("Backing Vocal", egui::Color32::from_rgb(255, 140, 140))
                .region(Region::new("Harmonies", 4.0, 4.0))
        ),
    Track::new_folder("Guitars", egui::Color32::from_rgb(255, 200, 50))
        .child(
            Track::new("Rhythm Guitar", egui::Color32::from_rgb(255, 210, 70))
                .region(Region::new("Strumming", 0.0, 8.0))
        )
        .child(
            Track::new("Lead Guitar", egui::Color32::from_rgb(255, 220, 90))
                .region(Region::new("Solo", 4.0, 4.0))
        ),
    Track::new("Bass", egui::Color32::from_rgb(100, 150, 255))
        .region(Region::new("Bassline", 0.0, 8.0)),
    Track::new("Drums", egui::Color32::from_rgb(150, 255, 150))
        .region(Region::new("Beat", 0.0, 8.0)),
];

let mut playhead_pos = 0.0;

Timeline::new()
    .track_header_width(160.0)
    .track_height(50.0)
    .beat_width(45.0)
    .measures(8)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);
```

### Nested Folders

You can nest folders within folders for complex project organization:

```demo
let theme = ui.ctx().armas_theme();

let mut tracks = vec![
    Track::new_folder("Instruments", egui::Color32::from_rgb(150, 150, 255))
        .child(
            Track::new_folder("Strings", egui::Color32::from_rgb(160, 160, 255))
                .child(
                    Track::new("Violin", egui::Color32::from_rgb(170, 170, 255))
                        .region(Region::new("Melody", 0.0, 4.0))
                )
                .child(
                    Track::new("Cello", egui::Color32::from_rgb(180, 180, 255))
                        .region(Region::new("Bass", 0.0, 4.0))
                )
        )
        .child(
            Track::new("Piano", egui::Color32::from_rgb(190, 190, 255))
                .region(Region::new("Chords", 0.0, 4.0))
        ),
    Track::new("Drums", egui::Color32::from_rgb(255, 150, 150))
        .region(Region::new("Beat", 0.0, 4.0)),
];

let mut playhead_pos = 0.0;

Timeline::new()
    .track_header_width(160.0)
    .track_height(45.0)
    .beat_width(50.0)
    .measures(4)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);
```

### Collapsed State

Folders can start collapsed to save screen space:

```demo
let theme = ui.ctx().armas_theme();

let mut tracks = vec![
    Track::new_folder("Group 1", egui::Color32::from_rgb(255, 100, 100))
        .collapsed(true) // Start collapsed
        .child(Track::new("Track 1A", egui::Color32::from_rgb(255, 120, 120)))
        .child(Track::new("Track 1B", egui::Color32::from_rgb(255, 140, 140))),
    Track::new_folder("Group 2", egui::Color32::from_rgb(100, 255, 100))
        .child(Track::new("Track 2A", egui::Color32::from_rgb(120, 255, 120)))
        .child(Track::new("Track 2B", egui::Color32::from_rgb(140, 255, 140))),
    Track::new("Solo Track", egui::Color32::from_rgb(100, 150, 255))
        .region(Region::new("Clip", 0.0, 4.0)),
];

let mut playhead_pos = 0.0;

Timeline::new()
    .track_header_width(140.0)
    .track_height(45.0)
    .beat_width(50.0)
    .measures(4)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);
```

## API Reference

### Track Struct

```rust
pub struct Track {
    pub name: String,          // Track name
    pub controls: TrackControls, // M/S/R controls
    pub color: Color32,        // Track color
    pub regions: Vec<Region>,  // Regions on track
    pub is_folder: bool,       // Is this a folder track?
    pub collapsed: bool,       // Is folder collapsed?
    pub children: Vec<Track>,  // Child tracks (for folders)
}
```

#### Track Methods

```rust
Track::new(name: impl Into<String>, color: Color32) -> Self
```

Creates a new regular track with the specified name and color.

```rust
Track::new_folder(name: impl Into<String>, color: Color32) -> Self
```

Creates a new folder track that can contain child tracks.

**Builder methods:**
- `.region(region: Region)` - Add a single region
- `.regions(regions: Vec<Region>)` - Set all regions
- `.child(child: Track)` - Add a child track (for folders)
- `.children(children: Vec<Track>)` - Set all child tracks (for folders)
- `.collapsed(collapsed: bool)` - Set initial collapsed state

### Timeline Constructor

```rust
Timeline::new() -> Self
```

Creates a new timeline with default settings (200px headers, 80px tracks, 60px beats, 16 measures, 4/4 time).

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.id()` | `impl Into<egui::Id>` | Auto-generated | Custom ID (important for multiple timelines) |
| `.track_header_width()` | `f32` | `200.0` | Width of track header column |
| `.track_height()` | `f32` | `80.0` | Height of each track row |
| `.beat_width()` | `f32` | `60.0` | Pixels per beat (zoom level) |
| `.measures()` | `u32` | `16` | Number of measures to display |
| `.beats_per_measure()` | `u32` | `4` | Time signature numerator |
| `.ruler_height()` | `f32` | `40.0` | Height of time ruler at top |
| `.show_playhead()` | `bool` | `true` | Show playhead indicator |
| `.playhead_color()` | `Color32` | `theme.primary()` | Playhead color |
| `.scroll_to_beat()` | `f32` | `None` | Scroll timeline to show this beat position |

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

Shows the complete timeline view with all tracks.

**Parameters:**
- `ui` - egui UI context
- `tracks` - Mutable reference to tracks vector
- `playhead_position` - Mutable reference to playhead position in beats
- `theme` - Armas theme for styling

**Returns:** `TimelineResponse` with interaction details

### TimelineResponse Struct

```rust
pub struct TimelineResponse {
    pub response: Response,                    // egui Response
    pub track_clicked: Option<usize>,          // Track index clicked
    pub region_clicked: Option<(usize, usize)>, // (track_idx, region_idx)
    pub empty_clicked: Option<(usize, f32)>,   // (track_idx, beat_position)
    pub playhead_moved: bool,                  // Playhead was dragged
    pub playhead_position: f32,                // Current playhead position
}
```

## Visual Design

### Layout Structure

```
┌─────────────┬─────────────────────────────────────┐
│             │         Time Ruler                  │
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

### Components Used

- **TimeRuler**: Horizontal ruler showing bars and beats
- **TrackHeader**: Left column showing track name and controls
- **TimelineTrack**: Timeline row displaying audio/MIDI/automation regions
- **ScrollArea**: Synchronized scrolling for horizontal (time) and vertical (tracks)

### Synchronization

All components use the same timing parameters:
- `.beat_width()` - Pixels per beat (must match across all components)
- `.measures()` - Number of measures
- `.beats_per_measure()` - Time signature

This ensures perfect alignment between ruler, tracks, and grid lines.

## Use Cases

### DAW Arrangement View

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Lead Vocals", egui::Color32::from_rgb(255, 100, 100))
        .regions(vec![
            Region::new("Verse 1", 0.0, 4.0),
            Region::new("Chorus 1", 4.0, 3.0),
            Region::new("Verse 2", 8.0, 4.0),
        ]),
    Track::new("Backing Vocals", egui::Color32::from_rgb(255, 150, 150))
        .region(Region::new("Harmonies", 4.0, 3.0)),
    Track::new("Acoustic Guitar", egui::Color32::from_rgb(200, 150, 100))
        .region(Region::new("Strumming", 0.0, 8.0)),
    Track::new("Electric Guitar", egui::Color32::from_rgb(255, 200, 50))
        .regions(vec![
            Region::new("Lead A", 4.0, 3.0),
            Region::new("Lead B", 8.0, 2.0),
        ]),
    Track::new("Bass", egui::Color32::from_rgb(100, 150, 255))
        .region(Region::new("Bassline", 0.0, 8.0)),
    Track::new("Drums", egui::Color32::from_rgb(150, 255, 150))
        .region(Region::new("Beat", 0.0, 8.0)),
];

let mut playhead_pos = 0.0;

Timeline::new()
    .track_header_width(140.0)
    .track_height(50.0)
    .beat_width(45.0)
    .measures(8)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);
```

### Compact Mixer Timeline View

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Ch 1", egui::Color32::from_rgb(255, 100, 100))
        .region(Region::new("Take 1", 0.0, 4.0)),
    Track::new("Ch 2", egui::Color32::from_rgb(100, 255, 100))
        .region(Region::new("Take 2", 0.0, 4.0)),
    Track::new("Ch 3", egui::Color32::from_rgb(100, 100, 255))
        .region(Region::new("Take 3", 0.0, 4.0)),
    Track::new("Ch 4", egui::Color32::from_rgb(255, 200, 50))
        .region(Region::new("Take 4", 0.0, 4.0)),
];

let mut playhead_pos = 0.0;

Timeline::new()
    .track_header_width(80.0)
    .track_height(35.0)
    .beat_width(40.0)
    .measures(4)
    .ruler_height(30.0)
    .show(ui, &mut tracks, &mut playhead_pos, &theme);
```

## Performance

- **Efficient scrolling**: Uses egui's optimized ScrollArea
- **Synchronized rendering**: Single scroll state for all tracks
- **Minimal allocations**: Direct painting with no intermediate buffers
- **Scalable**: Handles 50+ tracks smoothly
- **Visibility culling**: Only renders visible regions and tracks

## Integration Notes

### With TrackControls

Track headers include M/S/R controls. Access them through the `Track.controls` field:

```rust
let mut track = Track::new("Audio 1", Color32::from_rgb(255, 100, 100));

// Read control state
if track.controls.muted {
    println!("Track is muted");
}

// Modify controls
track.controls.soloed = true;
```

### With Region Data

Regions support audio, MIDI, and automation data. See the `TimelineTrack` documentation for details on:
- `WaveformData` for audio regions
- `MidiData` for MIDI regions
- `AutomationData` for automation regions

### With Playhead

Currently, the playhead position is managed externally. Future versions may include integrated playhead dragging and transport controls.

## Dependencies

- `egui = "0.33"`
- Uses Armas components: `TimeRuler`, `TrackHeader`, `TimelineTrack`
- Theme colors: `surface`, `outline`, `primary`
- Minimum version: `armas 0.1.0`
