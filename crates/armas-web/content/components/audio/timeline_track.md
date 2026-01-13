# Timeline Track

Horizontal track row for DAW timelines that displays audio/MIDI/automation regions. Supports multiple region types with different visualizations, region selection, muting, and click interactions.

## Audio Track (Default)

```demo
let theme = ui.ctx().armas_theme();
let mut regions = vec![
    Region::new("Intro", 0.0, 2.0),
    Region::new("Verse", 2.0, 2.0),
];

TimelineTrack::new()
    .height(80.0)
    .beat_width(60.0)
    .measures(2)
    .track_color(egui::Color32::from_rgb(100, 150, 255))
    .show(ui, &mut regions, &theme);
```

## MIDI Track

```demo
let theme = ui.ctx().armas_theme();
let mut regions = vec![
    Region::midi("Piano", 0.0, 2.0),
    Region::midi("Chords", 2.0, 2.0),
];

TimelineTrack::new()
    .height(80.0)
    .beat_width(60.0)
    .measures(2)
    .track_color(egui::Color32::from_rgb(255, 150, 100))
    .show(ui, &mut regions, &theme);
```

## Automation Track

```demo
let theme = ui.ctx().armas_theme();
let mut regions = vec![
    Region::automation("Volume", 0.0, 2.0),
    Region::automation("Pan", 2.0, 2.0),
];

TimelineTrack::new()
    .height(80.0)
    .beat_width(60.0)
    .measures(2)
    .track_color(egui::Color32::from_rgb(150, 255, 150))
    .show(ui, &mut regions, &theme);
```

## Mixed Region Types

```demo
let theme = ui.ctx().armas_theme();
let mut regions = vec![
    Region::new("Audio", 0.0, 1.5),
    Region::midi("MIDI", 2.0, 1.5),
    Region::automation("Auto", 4.0, 1.5),
];

TimelineTrack::new()
    .height(80.0)
    .beat_width(60.0)
    .measures(2)
    .track_color(egui::Color32::from_rgb(180, 150, 255))
    .show(ui, &mut regions, &theme);
```

## With Region Selection

```demo
let theme = ui.ctx().armas_theme();
let mut regions = vec![
    Region::new("Clip 1", 0.0, 2.0),
    Region::new("Clip 2", 2.0, 2.0).selected(true),
    Region::new("Clip 3", 4.0, 2.0),
];

TimelineTrack::new()
    .height(80.0)
    .beat_width(60.0)
    .measures(2)
    .track_color(egui::Color32::from_rgb(255, 150, 100))
    .show(ui, &mut regions, &theme);
```

## With Muted Regions

```demo
let theme = ui.ctx().armas_theme();
let mut regions = vec![
    Region::new("Active", 0.0, 2.0),
    Region::new("Muted", 2.0, 2.0).muted(true),
    Region::new("Active", 4.0, 2.0),
];

TimelineTrack::new()
    .height(80.0)
    .beat_width(60.0)
    .measures(2)
    .track_color(egui::Color32::from_rgb(100, 255, 150))
    .show(ui, &mut regions, &theme);
```

## Custom Region Colors

```demo
let theme = ui.ctx().armas_theme();
let mut regions = vec![
    Region::new("Red", 0.0, 2.0)
        .color(egui::Color32::from_rgb(255, 100, 100)),
    Region::new("Green", 2.0, 2.0)
        .color(egui::Color32::from_rgb(100, 255, 100)),
    Region::new("Blue", 4.0, 2.0)
        .color(egui::Color32::from_rgb(100, 100, 255)),
];

TimelineTrack::new()
    .height(80.0)
    .beat_width(60.0)
    .measures(2)
    .show(ui, &mut regions, &theme);
```

## Different Track Heights

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    ui.label("Compact (50px)");
    let mut regions1 = vec![Region::new("Clip", 0.0, 4.0)];
    TimelineTrack::new()
        .height(50.0)
        .beat_width(60.0)
        .measures(2)
        .track_color(egui::Color32::from_rgb(255, 150, 100))
        .show(ui, &mut regions1, &theme);

    ui.add_space(8.0);

    ui.label("Standard (80px)");
    let mut regions2 = vec![Region::new("Clip", 0.0, 4.0)];
    TimelineTrack::new()
        .height(80.0)
        .beat_width(60.0)
        .measures(2)
        .track_color(egui::Color32::from_rgb(100, 200, 255))
        .show(ui, &mut regions2, &theme);

    ui.add_space(8.0);

    ui.label("Tall (120px)");
    let mut regions3 = vec![Region::new("Clip", 0.0, 4.0)];
    TimelineTrack::new()
        .height(120.0)
        .beat_width(60.0)
        .measures(2)
        .track_color(egui::Color32::from_rgb(150, 100, 255))
        .show(ui, &mut regions3, &theme);
});
```

## Different Zoom Levels

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    ui.label("Zoomed Out (40px per beat)");
    let mut regions1 = vec![
        Region::new("A", 0.0, 2.0),
        Region::new("B", 3.0, 3.0),
    ];
    TimelineTrack::new()
        .height(70.0)
        .beat_width(40.0)
        .measures(2)
        .track_color(egui::Color32::from_rgb(255, 180, 100))
        .show(ui, &mut regions1, &theme);

    ui.add_space(8.0);

    ui.label("Normal (60px per beat)");
    let mut regions2 = vec![
        Region::new("A", 0.0, 2.0),
        Region::new("B", 3.0, 3.0),
    ];
    TimelineTrack::new()
        .height(70.0)
        .beat_width(60.0)
        .measures(2)
        .track_color(egui::Color32::from_rgb(100, 180, 255))
        .show(ui, &mut regions2, &theme);

    ui.add_space(8.0);

    ui.label("Zoomed In (80px per beat)");
    let mut regions3 = vec![
        Region::new("A", 0.0, 2.0),
        Region::new("B", 3.0, 2.0),
    ];
    TimelineTrack::new()
        .height(70.0)
        .beat_width(80.0)
        .measures(2)
        .track_color(egui::Color32::from_rgb(180, 100, 255))
        .show(ui, &mut regions3, &theme);
});
```

## Custom Background Color

```demo
let theme = ui.ctx().armas_theme();
let mut regions = vec![
    Region::new("Clip 1", 0.0, 2.0),
    Region::new("Clip 2", 3.0, 2.0),
];

TimelineTrack::new()
    .height(80.0)
    .beat_width(60.0)
    .measures(2)
    .track_color(egui::Color32::from_rgb(255, 200, 100))
    .background_color(egui::Color32::from_rgb(25, 25, 30))
    .show(ui, &mut regions, &theme);
```

## Handling Interactions

```demo
let theme = ui.ctx().armas_theme();
let mut regions = vec![
    Region::new("Clip 1", 0.0, 2.0),
    Region::new("Clip 2", 3.0, 2.0),
];

let response = TimelineTrack::new()
    .height(80.0)
    .beat_width(60.0)
    .measures(2)
    .track_color(egui::Color32::from_rgb(120, 180, 255))
    .show(ui, &mut regions, &theme);

ui.add_space(8.0);
if let Some(region_idx) = response.region_clicked {
    ui.label(format!("Clicked region: {} ({})", region_idx, regions[region_idx].name));
}
if let Some(beat_pos) = response.empty_clicked {
    ui.label(format!("Clicked empty area at beat: {:.2}", beat_pos));
}
```

## Multiple Tracks

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    let tracks = vec![
        ("Drums", egui::Color32::from_rgb(255, 100, 100)),
        ("Bass", egui::Color32::from_rgb(100, 255, 100)),
        ("Guitar", egui::Color32::from_rgb(255, 200, 50)),
        ("Vocals", egui::Color32::from_rgb(100, 150, 255)),
    ];

    for (name, color) in tracks {
        ui.horizontal(|ui| {
            ui.label(name);
            ui.add_space(4.0);
            let mut regions = vec![
                Region::new("A", 0.0, 3.0),
            ];
            TimelineTrack::new()
                .height(50.0)
                .beat_width(50.0)
                .measures(2)
                .track_color(color)
                .show(ui, &mut regions, &theme);
        });
        ui.add_space(2.0);
    }
});
```

## API Reference

### Region Struct

```rust
pub struct Region {
    pub name: String,            // Region name
    pub start: f32,              // Start position in beats
    pub duration: f32,           // Duration in beats
    pub region_type: RegionType, // Audio, MIDI, or Automation
    pub color: Option<Color32>,  // Region color
    pub selected: bool,          // Whether selected
    pub muted: bool,             // Whether muted
}
```

#### Region Constructors

**Audio Regions:**
```rust
Region::new(name, start, duration) -> Self  // Audio with simulated waveform
Region::audio(name, start, duration, data: WaveformData) -> Self  // With real data
```

**MIDI Regions:**
```rust
Region::midi(name, start, duration) -> Self  // MIDI with simulated pattern
Region::midi_with_data(name, start, duration, data: MidiData) -> Self  // With real notes
```

**Automation Regions:**
```rust
Region::automation(name, start, duration) -> Self  // With simulated curve
Region::automation_with_data(name, start, duration, data: AutomationData) -> Self  // With real data
```

**Builder methods:**
- `.color(color: Color32)` - Set region color
- `.selected(selected: bool)` - Set selected state
- `.muted(muted: bool)` - Set muted state

### Region Types

#### WaveformData

For audio regions with real waveform data:

```rust
pub struct WaveformData {
    pub peaks: Vec<(f32, f32)>,  // (min, max) pairs for each sample window
}

// Create from peak data
let data = WaveformData::from_peaks(peaks);
let region = Region::audio("Vocals", 0.0, 4.0, data);
```

#### MidiData

For MIDI regions with note information:

```rust
pub struct MidiNote {
    pub note: u8,       // 0-127 (60 = middle C)
    pub start: f32,     // Beats relative to region start
    pub duration: f32,  // Duration in beats
    pub velocity: u8,   // 0-127
}

pub struct MidiData {
    pub notes: Vec<MidiNote>,
}

// Create from MIDI notes
let data = MidiData::from_notes(vec![
    MidiNote { note: 60, start: 0.0, duration: 0.5, velocity: 100 },
    MidiNote { note: 64, start: 0.5, duration: 0.5, velocity: 90 },
]);
let region = Region::midi_with_data("Piano", 0.0, 4.0, data);
```

#### AutomationData

For automation regions with control points:

```rust
pub struct AutomationPoint {
    pub time: f32,   // Beats relative to region start
    pub value: f32,  // Normalized 0.0 to 1.0
}

pub struct AutomationData {
    pub points: Vec<AutomationPoint>,
}

// Create from automation points
let data = AutomationData::from_points(vec![
    AutomationPoint { time: 0.0, value: 0.0 },
    AutomationPoint { time: 2.0, value: 1.0 },
    AutomationPoint { time: 4.0, value: 0.5 },
]);
let region = Region::automation_with_data("Volume", 0.0, 4.0, data);
```

**Integration with audio-automation crate:**

If you're using the [`audio-automation`](https://github.com/yourusername/audio-automation) crate, you can convert envelopes directly:

```rust
#[cfg(feature = "audio-automation")]
let data = AutomationData::from_envelope(&envelope, start_time, duration, 50);
```

### TimelineTrack Constructor

```rust
TimelineTrack::new() -> Self
```

Creates a new timeline track with default settings (80px height, 60px per beat, 8 measures, 4/4 time).

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.height()` | `f32` | `80.0` | Track height in pixels |
| `.beat_width()` | `f32` | `60.0` | Pixels per beat (zoom level) |
| `.measures()` | `u32` | `8` | Number of measures to display |
| `.beats_per_measure()` | `u32` | `4` | Time signature numerator |
| `.track_color()` | `Color32` | `None` | Default color for regions |
| `.background_color()` | `Color32` | `theme.surface()` | Track background color |

### Show Method

```rust
pub fn show(
    self,
    ui: &mut Ui,
    regions: &mut Vec<Region>,
    theme: &Theme,
) -> TimelineTrackResponse
```

Shows the timeline track and returns interaction response.

**Parameters:**
- `ui` - egui UI context
- `regions` - Mutable reference to regions vector
- `theme` - Armas theme for styling

**Returns:** `TimelineTrackResponse` with interaction details

### TimelineTrackResponse Struct

```rust
pub struct TimelineTrackResponse {
    pub response: Response,           // egui Response for track
    pub region_clicked: Option<usize>, // Index of clicked region
    pub empty_clicked: Option<f32>,   // Beat position of empty click
}
```

## Visual Design

### Track Background

- Default: `theme.surface()`
- Customizable with `.background_color()`
- Corner radius: `theme.spacing.corner_radius_small` (8px)
- Shows subtle beat grid lines (measure lines at 30% alpha, beat lines at 15% alpha)

### Beat Grid Lines

**Measure Lines:**
- 1.0px width
- 30% opacity of `theme.outline()`
- Mark measure boundaries

**Beat Lines:**
- 0.5px width
- 15% opacity of `theme.outline_variant()`
- Mark individual beats

### Regions

**Background:**
- 180 alpha for active regions (70% opacity)
- 150 alpha for muted regions with darkened color (60% opacity)
- Glassmorphic appearance
- Corner radius: `theme.spacing.corner_radius_small` (8px)
- 4px margin from track top/bottom

**Borders:**
- Selected: 2px `theme.primary()` stroke
- Unselected: 1px semi-transparent black stroke

**Text:**
- Region name displayed at top-left
- 12px font size
- White text for active regions
- `theme.on_surface_variant()` for muted regions
- 6px padding from edges

**Waveform Visualization:**
- Simple vertical lines spaced 4px apart
- Simulated waveform using sine wave pattern
- 100 alpha (translucent)
- Not shown for muted regions
- Gives visual sense of audio content

### Interaction States

**Normal:** Semi-transparent with subtle border
**Selected:** Bright primary-colored border
**Muted:** Darkened color, reduced opacity, no waveform
**Hover:** (handled by egui Response)

## Synchronization with TimeRuler

**Critical:** When using TimelineTrack with TimeRuler, these parameters **must match exactly**:

- `.beat_width()` - Same pixels per beat
- `.measures()` - Same number of measures
- `.beats_per_measure()` - Same time signature

This ensures the track grid aligns perfectly with the ruler markings.

## Use Cases

### DAW Arrangement View

For a complete scrollable DAW timeline with synchronized ruler, playhead, headers, and tracks, see the `DAWTimeline` component which combines all these pieces.

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    // Time ruler at top
    TimeRuler::new()
        .id("arrangement_ruler")
        .measures(2)
        .beat_width(60.0)
        .show(ui, &theme);

    ui.add_space(4.0);

    // Multiple tracks
    let mut drums_regions = vec![
        Region::new("Kick", 0.0, 4.0),
    ];
    TimelineTrack::new()
        .height(60.0)
        .beat_width(60.0)
        .measures(2)
        .track_color(egui::Color32::from_rgb(255, 100, 100))
        .show(ui, &mut drums_regions, &theme);

    ui.add_space(2.0);

    let mut bass_regions = vec![
        Region::new("Bass", 0.0, 3.0),
    ];
    TimelineTrack::new()
        .height(60.0)
        .beat_width(60.0)
        .measures(2)
        .track_color(egui::Color32::from_rgb(100, 255, 100))
        .show(ui, &mut bass_regions, &theme);
});
```

## Performance

- **Minimal rendering**: Only draws visible regions and grid lines
- **Direct painting**: No intermediate buffers or allocations
- **Efficient hit testing**: Simple rect-based click detection
- **Scales well**: Handles 100+ regions per track smoothly

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `outline`, `outline_variant`, `primary`, `on_surface_variant`
- Minimum version: `armas 0.1.0`
