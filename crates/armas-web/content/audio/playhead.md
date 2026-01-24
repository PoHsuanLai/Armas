# Playhead

Vertical line showing current playback position in DAW timeline. Draggable for scrubbing through the timeline.

## Basic Usage

The Playhead renders as an overlay within a specified rect. You need to render your timeline content first, get its rect, then render the playhead on top.

```demo
let theme = ui.ctx().armas_theme();
let mut position = 4.5; // Beat position
let beat_width = 60.0;
let timeline_height = 200.0;

// Allocate space for timeline
let timeline_width = 8.0 * 4.0 * beat_width; // 8 measures, 4 beats each
let (rect, _) = ui.allocate_exact_size(
    egui::vec2(timeline_width, timeline_height),
    egui::Sense::hover()
);

// Draw timeline background
ui.painter().rect_filled(rect, 4.0, theme.muted());

// Render playhead overlay
Playhead::new().beat_width(beat_width).height(timeline_height)
    .show_in_rect(ui, rect, &mut position, &theme);

ui.label(format!("Position: {:.2} beats", position));
```

## With TimeRuler (Scrollable)

The playhead works inside scrollable areas by rendering it as an overlay on top of your timeline content.

```demo
let theme = ui.ctx().armas_theme();
let mut position = 2.0;
let beat_width = 80.0;
let measures = 8;

// Time ruler with scroll
TimeRuler::new()
    .id("playhead_ruler_scroll")
    .measures(measures)
    .beat_width(beat_width)
    .show(ui, &theme);

// Timeline content inside scrollable area
egui::ScrollArea::horizontal()
    .id_salt("playhead_scroll_area")
    .show(ui, |ui| {
        let timeline_width = measures as f32 * 4.0 * beat_width;
        let height = 200.0;

        // Allocate and draw timeline content
        let (rect, _) = ui.allocate_exact_size(
            egui::vec2(timeline_width, height),
            egui::Sense::hover()
        );
        ui.painter().rect_filled(rect, 4.0, theme.muted());

        // Render playhead overlay
        Playhead::new().beat_width(beat_width).height(height)
            .show_in_rect(ui, rect, &mut position, &theme);
    });

ui.label(format!("Playback at beat {:.2}", position));
```

## Custom Colors

```demo
let theme = ui.ctx().armas_theme();
let mut pos1 = 1.0;
let mut pos2 = 3.0;
let beat_width = 60.0;
let timeline_height = 150.0;

ui.horizontal(|ui| {
    ui.vertical(|ui| {
        ui.label("Red Playhead");
        let (rect, _) = ui.allocate_exact_size(
            egui::vec2(beat_width * 8.0, timeline_height),
            egui::Sense::hover()
        );
        ui.painter().rect_filled(rect, 4.0, theme.muted());
        Playhead::new().beat_width(beat_width).height(timeline_height)
            .id("red_playhead")
            .color(egui::Color32::from_rgb(255, 80, 80))
            .show_in_rect(ui, rect, &mut pos1, &theme);
    });

    ui.vertical(|ui| {
        ui.label("Green Playhead");
        let (rect, _) = ui.allocate_exact_size(
            egui::vec2(beat_width * 8.0, timeline_height),
            egui::Sense::hover()
        );
        ui.painter().rect_filled(rect, 4.0, theme.muted());
        Playhead::new().beat_width(beat_width).height(timeline_height)
            .id("green_playhead")
            .color(egui::Color32::from_rgb(80, 255, 120))
            .show_in_rect(ui, rect, &mut pos2, &theme);
    });
});
```

## Different Styles

```demo
let theme = ui.ctx().armas_theme();
let mut position = 2.5;
let beat_width = 60.0;
let timeline_width = beat_width * 8.0;

ui.vertical(|ui| {
    ui.label("Thin Line");
    let (rect, _) = ui.allocate_exact_size(egui::vec2(timeline_width, 100.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 4.0, theme.muted());
    Playhead::new().beat_width(beat_width).height(100.0)
        .id("thin_line")
        .line_width(1.0)
        .show_in_rect(ui, rect, &mut position, &theme);

    ui.add_space(10.0);

    ui.label("Thick Line");
    let (rect, _) = ui.allocate_exact_size(egui::vec2(timeline_width, 100.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 4.0, theme.muted());
    Playhead::new().beat_width(beat_width).height(100.0)
        .id("thick_line")
        .line_width(4.0)
        .show_in_rect(ui, rect, &mut position, &theme);

    ui.add_space(10.0);

    ui.label("No Handle");
    let (rect, _) = ui.allocate_exact_size(egui::vec2(timeline_width, 100.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 4.0, theme.muted());
    Playhead::new().beat_width(beat_width).height(100.0)
        .id("no_handle")
        .show_handle(false)
        .show_in_rect(ui, rect, &mut position, &theme);
});
```

## Handle Sizes

```demo
let theme = ui.ctx().armas_theme();
let mut pos1 = 1.0;
let mut pos2 = 3.0;
let mut pos3 = 5.0;
let beat_width = 60.0;
let timeline_width = beat_width * 8.0;
let timeline_height = 120.0;

ui.horizontal(|ui| {
    ui.vertical(|ui| {
        ui.label("Small");
        let (rect, _) = ui.allocate_exact_size(egui::vec2(timeline_width, timeline_height), egui::Sense::hover());
        ui.painter().rect_filled(rect, 4.0, theme.muted());
        Playhead::new().beat_width(beat_width).height(timeline_height)
            .id("small_handle")
            .handle_size(4.0)
            .show_in_rect(ui, rect, &mut pos1, &theme);
    });

    ui.vertical(|ui| {
        ui.label("Medium");
        let (rect, _) = ui.allocate_exact_size(egui::vec2(timeline_width, timeline_height), egui::Sense::hover());
        ui.painter().rect_filled(rect, 4.0, theme.muted());
        Playhead::new().beat_width(beat_width).height(timeline_height)
            .id("medium_handle")
            .handle_size(6.0)
            .show_in_rect(ui, rect, &mut pos2, &theme);
    });

    ui.vertical(|ui| {
        ui.label("Large");
        let (rect, _) = ui.allocate_exact_size(egui::vec2(timeline_width, timeline_height), egui::Sense::hover());
        ui.painter().rect_filled(rect, 4.0, theme.muted());
        Playhead::new().beat_width(beat_width).height(timeline_height)
            .id("large_handle")
            .handle_size(9.0)
            .show_in_rect(ui, rect, &mut pos3, &theme);
    });
});
```

## Glow Effects

```demo
let theme = ui.ctx().armas_theme();
let mut pos1 = 1.0;
let mut pos2 = 3.0;
let mut pos3 = 5.0;
let beat_width = 60.0;
let timeline_width = beat_width * 8.0;
let timeline_height = 120.0;

ui.horizontal(|ui| {
    ui.vertical(|ui| {
        ui.label("No Glow");
        let (rect, _) = ui.allocate_exact_size(egui::vec2(timeline_width, timeline_height), egui::Sense::hover());
        ui.painter().rect_filled(rect, 4.0, theme.muted());
        Playhead::new().beat_width(beat_width).height(timeline_height)
            .id("no_glow")
            .show_glow(false)
            .show_in_rect(ui, rect, &mut pos1, &theme);
    });

    ui.vertical(|ui| {
        ui.label("Subtle Glow");
        let (rect, _) = ui.allocate_exact_size(egui::vec2(timeline_width, timeline_height), egui::Sense::hover());
        ui.painter().rect_filled(rect, 4.0, theme.muted());
        Playhead::new().beat_width(beat_width).height(timeline_height)
            .id("subtle_glow")
            .glow_intensity(0.3)
            .show_in_rect(ui, rect, &mut pos2, &theme);
    });

    ui.vertical(|ui| {
        ui.label("Strong Glow");
        let (rect, _) = ui.allocate_exact_size(egui::vec2(timeline_width, timeline_height), egui::Sense::hover());
        ui.painter().rect_filled(rect, 4.0, theme.muted());
        Playhead::new().beat_width(beat_width).height(timeline_height)
            .id("strong_glow")
            .glow_intensity(0.8)
            .show_in_rect(ui, rect, &mut pos3, &theme);
    });
});
```

## Detecting Position Changes

```demo
let theme = ui.ctx().armas_theme();
let mut position = 3.5;
let beat_width = 60.0;
let timeline_width = beat_width * 8.0;
let timeline_height = 150.0;

let (rect, _) = ui.allocate_exact_size(egui::vec2(timeline_width, timeline_height), egui::Sense::hover());
ui.painter().rect_filled(rect, 4.0, theme.muted());

let response = Playhead::new().beat_width(beat_width).height(timeline_height)
    .show_in_rect(ui, rect, &mut position, &theme);

if response.changed() {
    ui.label(format!("Scrubbing to: {:.2} beats", position));
} else {
    ui.label("Drag the playhead to scrub");
}
```

## Full Timeline Example

```demo
let theme = ui.ctx().armas_theme();
let mut playback_position = 6.0;
let beat_width = 50.0;
let measures = 16;

ui.vertical(|ui| {
    // Timeline ruler
    TimeRuler::new()
        .id("full_timeline")
        .measures(measures)
        .beat_width(beat_width)
        .show(ui, &theme);

    // Timeline content area with playhead overlay
    let timeline_width = measures as f32 * 4.0 * beat_width;
    let timeline_height = 300.0;

    let (rect, _) = ui.allocate_exact_size(egui::vec2(timeline_width, timeline_height), egui::Sense::hover());
    ui.painter().rect_filled(rect, 4.0, theme.muted());

    // Track placeholders
    for i in 0..3 {
        let track_y = rect.min.y + (i as f32 * timeline_height / 3.0);
        ui.painter().text(
            egui::pos2(rect.min.x + 10.0, track_y + 20.0),
            egui::Align2::LEFT_TOP,
            format!("Track {}", i + 1),
            egui::FontId::proportional(14.0),
            theme.foreground()
        );
    }

    // Playhead overlay on top
    Playhead::new().beat_width(beat_width).height(timeline_height)
        .show_in_rect(ui, rect, &mut playback_position, &theme);

    ui.label(format!("Playing at beat {:.1}", playback_position));
});
```

## API Reference

### Constructor

```rust
Playhead::new() -> Self
```

Creates a new playhead indicator with default settings (60px beat width, 400px height).

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.beat_width()` | `f32` | `60.0` | Pixels per beat (must match TimeRuler) |
| `.height()` | `f32` | `400.0` | Height of the vertical line |
| `.id()` | `impl Into<egui::Id>` | Auto-generated | Custom ID (important for multiple playheads) |
| `.color()` | `Color32` | `theme.destructive()` | Playhead line color |
| `.line_width()` | `f32` | `2.0` | Width of the vertical line |
| `.show_handle()` | `bool` | `true` | Show draggable handle at top |
| `.handle_size()` | `f32` | `6.0` | Handle radius in pixels |
| `.show_glow()` | `bool` | `true` | Enable subtle glow effect on entire playhead |
| `.glow_intensity()` | `f32` | `0.3` | Glow intensity (0.0-1.0) |

### Show Method

```rust
pub fn show_in_rect(self, ui: &mut Ui, timeline_rect: Rect, position: &mut f32, theme: &Theme) -> Response
```

Renders the playhead as an overlay within the specified rect.

**Parameters:**
- `timeline_rect`: The rect of the timeline area where the playhead should be rendered
- `position`: Current beat position (will be modified by dragging)
- `theme`: Theme for styling

**Returns:**
- `Response` with `.changed()` indicating if position was modified

**Usage Pattern:**
1. Allocate space and render your timeline content
2. Get the rect from the allocation
3. Call `show_in_rect()` with that rect to render the playhead overlay

## Visual Design

### Line Style
- **Color**: `theme.destructive()` (red) by default for high visibility
- **Width**: 2.0px for clear visibility without being obtrusive
- **Full height**: Spans entire timeline height

### Handle Design
- **Rounded triangle** (teardrop shape) at top pointing down
- **Smooth curves**: No sharp edges for elegant appearance
- **Shadow**: Subtle drop shadow for depth
- **Hover effect**: Brightens on hover
- **3D highlight**: Inner highlight for dimensionality
- **Border**: Subtle border for contrast

### Interaction
- **Wide hit area**: Handle has 4x interaction width for easy grabbing
- **Smooth dragging**: Direct pixel-to-beat conversion
- **Position clamping**: Prevents negative positions
- **Visual feedback**: Handle brightens when hovered or dragged

## Positioning System

The playhead uses the same beat-based coordinate system as TimeRuler:

```rust
x_position = beat_position * beat_width
```

**Example:**
- Beat position: 4.5
- Beat width: 60px
- X position: 270px

**Critical**: The `beat_width` parameter **must match** the TimeRuler's `beat_width` for proper alignment.

## Use Cases

### DAW Playback

```demo
let theme = ui.ctx().armas_theme();
let mut playhead_pos = 0.0;
let beat_width = 60.0;
let measures = 8;

ui.vertical(|ui| {
    ui.label("Audio Timeline");

    TimeRuler::new()
        .id("daw_playback")
        .measures(measures)
        .beat_width(beat_width)
        .show(ui, &theme);

    let timeline_width = measures as f32 * 4.0 * beat_width;
    let timeline_height = 250.0;

    let (rect, _) = ui.allocate_exact_size(egui::vec2(timeline_width, timeline_height), egui::Sense::hover());
    ui.painter().rect_filled(rect, 4.0, theme.muted());

    Playhead::new().beat_width(beat_width).height(timeline_height)
        .show_in_rect(ui, rect, &mut playhead_pos, &theme);

    ui.horizontal(|ui| {
        if ui.button("|<").clicked() {
            playhead_pos = 0.0;
        }
        if ui.button(">").clicked() {
            // Start playback
        }
        if ui.button("||").clicked() {
            // Pause playback
        }
        ui.label(format!("Beat: {:.2}", playhead_pos));
    });
});
```

### Loop Region Markers

Using multiple playheads requires unique IDs for each one.

```demo
let theme = ui.ctx().armas_theme();
let mut loop_start = 4.0;
let mut loop_end = 12.0;
let beat_width = 50.0;
let measures = 16;

ui.vertical(|ui| {
    TimeRuler::new()
        .id("loop_markers")
        .measures(measures)
        .beat_width(beat_width)
        .show(ui, &theme);

    let timeline_width = measures as f32 * 4.0 * beat_width;
    let timeline_height = 200.0;

    let (rect, _) = ui.allocate_exact_size(egui::vec2(timeline_width, timeline_height), egui::Sense::hover());
    ui.painter().rect_filled(rect, 4.0, theme.muted());

    // Loop start marker (green)
    Playhead::new().beat_width(beat_width).height(timeline_height)
        .id("loop_start")
        .color(egui::Color32::from_rgb(100, 255, 100))
        .show_in_rect(ui, rect, &mut loop_start, &theme);

    // Loop end marker (red)
    Playhead::new().beat_width(beat_width).height(timeline_height)
        .id("loop_end")
        .color(egui::Color32::from_rgb(255, 100, 100))
        .show_in_rect(ui, rect, &mut loop_end, &theme);

    ui.label(format!("Loop: {:.1} - {:.1} beats", loop_start, loop_end));
});
```

### MIDI Editor Cursor

```demo
let theme = ui.ctx().armas_theme();
let mut cursor_pos = 5.5;
let beat_width = 70.0;
let measures = 8;

ui.vertical(|ui| {
    ui.label("MIDI Editor");

    TimeRuler::new()
        .id("midi_cursor")
        .measures(measures)
        .beat_width(beat_width)
        .division(GridDivision::Sixteenth)
        .show(ui, &theme);

    let timeline_width = measures as f32 * 4.0 * beat_width;
    let timeline_height = 180.0;

    let (rect, _) = ui.allocate_exact_size(egui::vec2(timeline_width, timeline_height), egui::Sense::hover());
    ui.painter().rect_filled(rect, 4.0, theme.muted());

    Playhead::new().beat_width(beat_width).height(timeline_height)
        .color(theme.primary())
        .line_width(1.5)
        .handle_size(5.0)
        .show_in_rect(ui, rect, &mut cursor_pos, &theme);

    ui.label(format!("Insert position: {:.2}", cursor_pos));
});
```

## Performance

- **Minimal rendering**: Only draws when visible
- **Efficient dragging**: Direct position calculation without intermediate state
- **No allocations**: All rendering uses immediate mode painting
- **Responsive**: Wide interaction area for easy grabbing

## Synchronization

When using Playhead with TimeRuler:

1. **Match beat_width**: Use the same `beat_width` value
2. **Layering**: Render playhead after (on top of) the timeline
3. **Height**: Set playhead height to span all tracks
4. **Position updates**: Update position during playback or from transport controls

## Dependencies

- `egui = "0.33"`
- Theme colors: `error()` (default), `surface()` (handle border)
- Minimum version: `armas 0.1.0`
