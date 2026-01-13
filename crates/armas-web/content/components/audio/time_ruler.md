# Time Ruler

Horizontal ruler showing measures, beats, and subdivisions for DAW timelines. Designed to align perfectly with PianoRollGrid's vertical grid lines.

## Basic Usage

TimeRuler is scrollable by default for long timelines.

```demo
let theme = ui.ctx().armas_theme();

TimeRuler::new()
    .id("basic_ruler")
    .measures(16)
    .beat_width(60.0)
    .show(ui, &theme);
```

## With Piano Roll Grid

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    // Time ruler above
    TimeRuler::new()
        .id("piano_ruler")
        .measures(4)
        .beat_width(80.0)
        .division(GridDivision::Sixteenth)
        .show(ui, &theme);

    // Piano roll grid below (must match parameters!)
    PianoRollGrid::new()
        .measures(4)
        .beat_width(80.0)
        .division(GridDivision::Sixteenth)
        .show(ui, &theme);
});
```

## Different Zoom Levels

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    ui.label("Zoomed Out (40px per beat)");
    TimeRuler::new()
        .id("zoom_out")
        .measures(8)
        .beat_width(40.0)
        .show(ui, &theme);

    ui.add_space(10.0);

    ui.label("Normal (60px per beat)");
    TimeRuler::new()
        .id("zoom_normal")
        .measures(8)
        .beat_width(60.0)
        .show(ui, &theme);

    ui.add_space(10.0);

    ui.label("Zoomed In (100px per beat)");
    TimeRuler::new()
        .id("zoom_in")
        .measures(8)
        .beat_width(100.0)
        .show(ui, &theme);
});
```

## Grid Divisions

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    ui.label("Quarter Notes");
    TimeRuler::new()
        .id("quarter")
        .measures(4)
        .division(GridDivision::Quarter)
        .show(ui, &theme);

    ui.add_space(5.0);

    ui.label("Eighth Notes");
    TimeRuler::new()
        .id("eighth")
        .measures(4)
        .division(GridDivision::Eighth)
        .show(ui, &theme);

    ui.add_space(5.0);

    ui.label("Sixteenth Notes");
    TimeRuler::new()
        .id("sixteenth")
        .measures(4)
        .division(GridDivision::Sixteenth)
        .show(ui, &theme);
});
```

## Time Signatures

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    ui.label("4/4 Time");
    TimeRuler::new()
        .id("4_4_time")
        .measures(4)
        .beats_per_measure(4)
        .show(ui, &theme);

    ui.add_space(5.0);

    ui.label("3/4 Time");
    TimeRuler::new()
        .id("3_4_time")
        .measures(4)
        .beats_per_measure(3)
        .show(ui, &theme);

    ui.add_space(5.0);

    ui.label("6/8 Time");
    TimeRuler::new()
        .id("6_8_time")
        .measures(4)
        .beats_per_measure(6)
        .show(ui, &theme);
});
```

## Without Beat Numbers

```demo
let theme = ui.ctx().armas_theme();

TimeRuler::new()
    .id("no_beat_numbers")
    .measures(8)
    .show_beat_numbers(false)
    .show(ui, &theme);
```

## Without Subdivisions

```demo
let theme = ui.ctx().armas_theme();

TimeRuler::new()
    .id("no_subdivisions")
    .measures(8)
    .show_subdivisions(false)
    .show(ui, &theme);
```

## Minutes:Seconds Display

```demo
let theme = ui.ctx().armas_theme();

TimeRuler::new()
    .id("time_display")
    .measures(8)
    .time_mode(TimeDisplayMode::MinutesSeconds)
    .tempo(120.0)
    .show(ui, &theme);
```

## Custom Height

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    ui.label("Compact (24px)");
    TimeRuler::new()
        .id("compact_height")
        .measures(4)
        .height(24.0)
        .show(ui, &theme);

    ui.add_space(5.0);

    ui.label("Standard (36px)");
    TimeRuler::new()
        .id("standard_height")
        .measures(4)
        .height(36.0)
        .show(ui, &theme);

    ui.add_space(5.0);

    ui.label("Tall (48px)");
    TimeRuler::new()
        .id("tall_height")
        .measures(4)
        .height(48.0)
        .show(ui, &theme);
});
```

## Without Scrolling

Use `.show_no_scroll()` when you want to manage scrolling externally or synchronize with other components.

```demo
let theme = ui.ctx().armas_theme();

TimeRuler::new()
    .id("no_scroll_ruler")
    .measures(4)
    .beat_width(80.0)
    .show_no_scroll(ui, &theme);
```

## API Reference

### Constructor

```rust
TimeRuler::new() -> Self
```

Creates a new time ruler with default settings (8 measures, 60px per beat, 4/4 time).

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.id()` | `impl Into<egui::Id>` | Auto | Custom ID for ScrollArea (prevents conflicts) |
| `.measures()` | `u32` | `8` | Number of measures to display |
| `.beat_width()` | `f32` | `60.0` | Pixels per beat (zoom level) |
| `.beats_per_measure()` | `u32` | `4` | Time signature numerator |
| `.division()` | `GridDivision` | `Sixteenth` | Grid subdivision density |
| `.height()` | `f32` | `36.0` | Ruler height in pixels |
| `.show_beat_numbers()` | `bool` | `true` | Show beat numbers within measures |
| `.show_subdivisions()` | `bool` | `true` | Show subdivision tick marks |
| `.time_mode()` | `TimeDisplayMode` | `BarsBeatsSixteenths` | Time display format |
| `.tempo()` | `f32` | `120.0` | Tempo in BPM (for minutes:seconds) |

### Show Methods

```rust
pub fn show(self, ui: &mut egui::Ui, theme: &Theme) -> Response
```

Shows the time ruler wrapped in a horizontal ScrollArea (default). Returns an `egui::Response` for hover detection.

```rust
pub fn show_no_scroll(self, ui: &mut egui::Ui, theme: &Theme) -> Response
```

Shows the time ruler without ScrollArea wrapper. Use this when you want to manage scrolling externally or synchronize scroll position with other components.

### GridDivision Enum

```rust
pub enum GridDivision {
    Whole,      // 4 beats
    Half,       // 2 beats
    Quarter,    // 1 beat
    Eighth,     // 1/2 beat
    Sixteenth,  // 1/4 beat
}
```

### TimeDisplayMode Enum

```rust
pub enum TimeDisplayMode {
    BarsBeatsSixteenths,  // "1", "2", "3" format
    MinutesSeconds,       // "0:00", "0:15" format
}
```

## Visual Design

### Grid Line Hierarchy

The ruler uses three levels of visual emphasis:

**Measure Lines (Strongest):**
- 2.0px stroke width
- Full height (100%)
- `theme.outline()` color
- Marks measure boundaries

**Beat Lines (Medium):**
- 1.5px stroke width
- 60% height
- `theme.outline()` color
- Marks each beat within measures

**Subdivision Lines (Subtle):**
- 0.5px stroke width
- 30% height
- `theme.outline_variant()` color
- Marks subdivisions based on `.division()`

### Typography

- **Measure numbers**: 11px, `theme.on_surface()` (prominent)
- **Beat numbers**: 9px, `theme.on_surface_variant()` (subtle)
- **Alignment**: Left-aligned with small offset

### Background

- Color: `theme.surface()`
- Corner radius: `theme.spacing.corner_radius_small` (8px)
- Bottom border: 1px `theme.outline_variant()`

## Synchronization with PianoRollGrid

**Critical**: When using TimeRuler with PianoRollGrid, the following parameters **must match exactly**:

- `.measures()` - Same number of measures
- `.beat_width()` - Same pixels per beat
- `.beats_per_measure()` - Same time signature
- `.division()` - Same grid subdivision

This ensures vertical grid lines align perfectly with ruler markings.

## Use Cases

### DAW Timeline

```demo
let theme = ui.ctx().armas_theme();
let measures = 16;
let beat_width = 60.0;

ui.vertical(|ui| {
    // Timeline ruler
    TimeRuler::new()
        .id("daw_timeline")
        .measures(measures)
        .beat_width(beat_width)
        .show(ui, &theme);

    // Track lanes would go below...
    for i in 0..3 {
        ui.label(format!("Track {}", i + 1));
        ui.separator();
    }
});
```

### MIDI Editor

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    TimeRuler::new()
        .id("midi_editor")
        .measures(8)
        .beat_width(80.0)
        .division(GridDivision::Sixteenth)
        .show(ui, &theme);

    ui.label("MIDI notes would appear here...");
});
```

### Audio Arrangement

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    TimeRuler::new()
        .id("audio_arrangement")
        .measures(32)
        .beat_width(50.0)
        .time_mode(TimeDisplayMode::MinutesSeconds)
        .tempo(140.0)
        .show(ui, &theme);

    ui.label("Audio regions would appear below...");
});
```

## Performance

- **Minimal rendering**: Only draws visible lines
- **No allocations**: Direct painting with no intermediate buffers
- **Efficient text**: Measure numbers rendered once per measure
- **Scales well**: Handles 100+ measures smoothly

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `outline`, `outline_variant`, `on_surface`, `on_surface_variant`
- Re-uses `GridDivision` from `PianoRollGrid`
- Minimum version: `armas 0.1.0`
