# Track Header

Header section for DAW timeline tracks with name, controls, and color indicator. Shows track name (editable), color bar, and M/S/R control buttons.

## Basic Usage

```demo
let theme = ui.ctx().armas_theme();
let mut name = "Audio 1".to_string();
let mut controls = TrackControls::default();

TrackHeader::new()
    .width(200.0)
    .color(egui::Color32::from_rgb(100, 150, 255))
    .show(ui, &mut name, &mut controls, &mut false, &theme);
```

## Multiple Tracks

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    let mut name1 = "Vocals".to_string();
    let mut controls1 = TrackControls::default();
    TrackHeader::new()
        .width(200.0)
        .color(egui::Color32::from_rgb(255, 100, 100))
        .show(ui, &mut name1, &mut controls1, &mut false, &theme);

    ui.add_space(4.0);

    let mut name2 = "Guitar".to_string();
    let mut controls2 = TrackControls::default();
    TrackHeader::new()
        .width(200.0)
        .color(egui::Color32::from_rgb(100, 255, 100))
        .show(ui, &mut name2, &mut controls2, &mut false, &theme);

    ui.add_space(4.0);

    let mut name3 = "Drums".to_string();
    let mut controls3 = TrackControls::default();
    TrackHeader::new()
        .width(200.0)
        .color(egui::Color32::from_rgb(100, 100, 255))
        .show(ui, &mut name3, &mut controls3, &mut false, &theme);
});
```

## Custom Width and Height

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    ui.label("Narrow (150px)");
    let mut name1 = "Track 1".to_string();
    let mut controls1 = TrackControls::default();
    TrackHeader::new()
        .width(150.0)
        .height(50.0)
        .color(egui::Color32::from_rgb(255, 150, 50))
        .show(ui, &mut name1, &mut controls1, &mut false, &theme);

    ui.add_space(8.0);

    ui.label("Wide (300px)");
    let mut name2 = "Track 2".to_string();
    let mut controls2 = TrackControls::default();
    TrackHeader::new()
        .width(300.0)
        .height(70.0)
        .color(egui::Color32::from_rgb(150, 100, 255))
        .show(ui, &mut name2, &mut controls2, &mut false, &theme);
});
```

## Compact Mode

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    ui.label("Standard Mode");
    let mut name1 = "Track 1".to_string();
    let mut controls1 = TrackControls::default();
    TrackHeader::new()
        .width(180.0)
        .color(egui::Color32::from_rgb(255, 180, 100))
        .show(ui, &mut name1, &mut controls1, &mut false, &theme);

    ui.add_space(8.0);

    ui.label("Compact Mode (smaller buttons & spacing)");
    let mut name2 = "Track 2".to_string();
    let mut controls2 = TrackControls::default();
    TrackHeader::new()
        .width(180.0)
        .compact(true)
        .color(egui::Color32::from_rgb(100, 180, 255))
        .show(ui, &mut name2, &mut controls2, &mut false, &theme);
});
```

## Without Controls

```demo
let theme = ui.ctx().armas_theme();
let mut name = "Audio Track".to_string();
let mut controls = TrackControls::default();

TrackHeader::new()
    .width(200.0)
    .show_controls(false)
    .color(egui::Color32::from_rgb(150, 255, 150))
    .show(ui, &mut name, &mut controls, &mut false, &theme);
```

## Non-Editable Name

```demo
let theme = ui.ctx().armas_theme();
let mut name = "Master Bus".to_string();
let mut controls = TrackControls::default();

TrackHeader::new()
    .width(200.0)
    .editable(false)
    .color(egui::Color32::from_rgb(255, 200, 50))
    .show(ui, &mut name, &mut controls, &mut false, &theme);
```

## Custom Card Background Color

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    ui.label("Default card color");
    let mut name1 = "Track 1".to_string();
    let mut controls1 = TrackControls::default();
    TrackHeader::new()
        .width(220.0)
        .color(egui::Color32::from_rgb(100, 150, 255))
        .show(ui, &mut name1, &mut controls1, &mut false, &theme);

    ui.add_space(8.0);

    ui.label("Custom dark card background");
    let mut name2 = "Track 2".to_string();
    let mut controls2 = TrackControls::default();
    TrackHeader::new()
        .width(220.0)
        .color(egui::Color32::from_rgb(255, 100, 100))
        .card_color(egui::Color32::from_rgb(30, 30, 35))
        .show(ui, &mut name2, &mut controls2, &mut false, &theme);

    ui.add_space(8.0);

    ui.label("Custom colored card background");
    let mut name3 = "Track 3".to_string();
    let mut controls3 = TrackControls::default();
    TrackHeader::new()
        .width(220.0)
        .color(egui::Color32::from_rgb(100, 255, 150))
        .card_color(egui::Color32::from_rgba_unmultiplied(50, 100, 80, 255))
        .show(ui, &mut name3, &mut controls3, &mut false, &theme);
});
```

## Handling Control Clicks

```demo
let theme = ui.ctx().armas_theme();
let mut name = "Audio 1".to_string();
let mut controls = TrackControls::default();

let response = TrackHeader::new()
    .width(220.0)
    .color(egui::Color32::from_rgb(100, 200, 255))
    .show(ui, &mut name, &mut controls, &mut false, &theme);

// Display state changes
ui.add_space(8.0);
if response.name_changed {
    ui.label(format!("Name changed to: {}", response.name));
}
if response.mute_clicked {
    ui.label(format!("Mute: {}", controls.muted));
}
if response.solo_clicked {
    ui.label(format!("Solo: {}", controls.soloed));
}
if response.arm_clicked {
    ui.label(format!("Armed: {}", controls.armed));
}

// Show current state
ui.add_space(4.0);
ui.label(format!("Current state - M:{} S:{} R:{}",
    controls.muted, controls.soloed, controls.armed));
```

## Different Track Colors

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    let colors = vec![
        ("Red", egui::Color32::from_rgb(255, 80, 80)),
        ("Orange", egui::Color32::from_rgb(255, 160, 50)),
        ("Yellow", egui::Color32::from_rgb(255, 220, 50)),
        ("Green", egui::Color32::from_rgb(80, 255, 120)),
        ("Cyan", egui::Color32::from_rgb(50, 220, 255)),
        ("Blue", egui::Color32::from_rgb(80, 120, 255)),
        ("Purple", egui::Color32::from_rgb(180, 80, 255)),
    ];

    for (color_name, color) in colors {
        let mut name = format!("{} Track", color_name);
        let mut controls = TrackControls::default();
        TrackHeader::new()
            .width(200.0)
            .height(50.0)
            .color(color)
            .show(ui, &mut name, &mut controls, &mut false, &theme);
        ui.add_space(2.0);
    }
});
```

## Pre-Configured States

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    ui.label("Muted Track");
    let mut name1 = "Muted Track".to_string();
    let mut controls1 = TrackControls {
        muted: true,
        soloed: false,
        armed: false,
    };
    TrackHeader::new()
        .width(200.0)
        .color(egui::Color32::from_rgb(150, 150, 150))
        .show(ui, &mut name1, &mut controls1, &mut false, &theme);

    ui.add_space(8.0);

    ui.label("Soloed Track");
    let mut name2 = "Solo Track".to_string();
    let mut controls2 = TrackControls {
        muted: false,
        soloed: true,
        armed: false,
    };
    TrackHeader::new()
        .width(200.0)
        .color(egui::Color32::from_rgb(255, 200, 50))
        .show(ui, &mut name2, &mut controls2, &mut false, &theme);

    ui.add_space(8.0);

    ui.label("Armed for Recording");
    let mut name3 = "Record Track".to_string();
    let mut controls3 = TrackControls {
        muted: false,
        soloed: false,
        armed: true,
    };
    TrackHeader::new()
        .width(200.0)
        .color(egui::Color32::from_rgb(255, 80, 80))
        .show(ui, &mut name3, &mut controls3, &mut false, &theme);
});
```

## Folder Tracks

Folder tracks can contain child tracks and show collapse/expand controls. Use `.is_folder(true)` to enable the collapse button and `.indent_level()` for nested tracks.

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    ui.label("Folder Track");
    let mut folder_name = "Vocals".to_string();
    let mut folder_controls = TrackControls::default();
    let mut folder_collapsed = false;

    let response = TrackHeader::new()
        .width(200.0)
        .color(egui::Color32::from_rgb(255, 100, 100))
        .is_folder(true)
        .show(ui, &mut folder_name, &mut folder_controls, &mut folder_collapsed, &theme);

    if response.collapse_clicked {
        ui.label(format!("Folder collapsed: {}", folder_collapsed));
    }

    // Child tracks (only show when not collapsed)
    if !folder_collapsed {
        ui.add_space(2.0);

        ui.label("Child Track 1 (indented)");
        let mut child1_name = "Lead Vocal".to_string();
        let mut child1_controls = TrackControls::default();
        let mut child1_collapsed = false;
        TrackHeader::new()
            .width(200.0)
            .color(egui::Color32::from_rgb(255, 120, 120))
            .indent_level(1)
            .show(ui, &mut child1_name, &mut child1_controls, &mut child1_collapsed, &theme);

        ui.add_space(2.0);

        ui.label("Child Track 2 (indented)");
        let mut child2_name = "Backing Vocal".to_string();
        let mut child2_controls = TrackControls::default();
        let mut child2_collapsed = false;
        TrackHeader::new()
            .width(200.0)
            .color(egui::Color32::from_rgb(255, 140, 140))
            .indent_level(1)
            .show(ui, &mut child2_name, &mut child2_controls, &mut child2_collapsed, &theme);
    }
});
```

## API Reference

### Constructor

```rust
TrackHeader::new() -> Self
```

Creates a new track header with default settings (200x60px, editable, with controls).

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.width()` | `f32` | `200.0` | Header width in pixels |
| `.height()` | `f32` | `60.0` | Header height in pixels |
| `.color()` | `Color32` | `theme.primary()` | Track color indicator |
| `.card_color()` | `Color32` | `None` | Custom card background color |
| `.editable()` | `bool` | `true` | Allow track name editing |
| `.show_controls()` | `bool` | `true` | Show M/S/R control buttons |
| `.compact()` | `bool` | `false` | Compact mode (smaller controls) |
| `.is_folder()` | `bool` | `false` | Is this a folder track (shows collapse button) |
| `.indent_level()` | `usize` | `0` | Indentation level for nested tracks |

### Show Method

```rust
pub fn show(
    self,
    ui: &mut Ui,
    name: &mut String,
    controls: &mut TrackControls,
    collapsed: &mut bool,
    theme: &Theme,
) -> TrackHeaderResponse
```

Shows the track header and returns response with interaction info.

**Parameters:**
- `ui` - egui UI context
- `name` - Mutable reference to track name string
- `controls` - Mutable reference to control state
- `collapsed` - Mutable reference to collapsed state (for folder tracks)
- `theme` - Armas theme for styling

**Returns:** `TrackHeaderResponse` with interaction details

### TrackControls Struct

```rust
pub struct TrackControls {
    pub muted: bool,    // Track is muted
    pub soloed: bool,   // Track is soloed
    pub armed: bool,    // Track is record-armed
}
```

**Default:** All controls set to `false`

### TrackHeaderResponse Struct

```rust
pub struct TrackHeaderResponse {
    pub response: Response,      // egui Response for entire header
    pub name_changed: bool,      // Track name was modified
    pub name: String,            // Current track name
    pub mute_clicked: bool,      // Mute button was clicked
    pub solo_clicked: bool,      // Solo button was clicked
    pub arm_clicked: bool,       // Record arm button was clicked
    pub controls: TrackControls, // Current control state
    pub collapse_clicked: bool,  // Collapse/expand button was clicked (folder tracks)
}
```

## Visual Design

### Card Wrapper

- Uses Material Design `Card` component with `Filled` variant
- Default background: `theme.muted()` (from Card)
- Can be customized with `.card_color()` for themed tracks
- Corner radius: `theme.spacing.corner_radius_medium` (12px)
- Provides subtle elevation and consistent padding

### Color Indicator Bar

- Left edge of card, inside the card padding
- 4px wide (3px in compact mode)
- Full height of header
- Uses track color (or `theme.primary()` if not specified)
- Provides quick visual identification of tracks

### Track Name

- Uses native egui `TextEdit` for editable names
- Theme-aware text color: `theme.foreground()`
- Borderless, inline editing experience
- Full width of header minus color bar and spacing
- Hint text: "Track Name"
- Can be displayed as non-editable label with `.editable(false)`

### Control Buttons

**Button Layout:**
- Horizontal row below track name
- M (Mute), S (Solo), R (Record Arm)
- 24x24px buttons (20x20px in compact mode)
- 4px spacing between buttons (2px compact)

**Button States:**
- Active: `ButtonVariant::Filled` (filled background)
- Inactive: `ButtonVariant::Outlined` (outlined only)
- Toggle behavior on click

### Spacing

**Standard Mode:**
- Color bar: 8px left offset
- Vertical spacing: 4px between elements
- Button spacing: 4px

**Compact Mode:**
- Color bar: 6px left offset
- Vertical spacing: 2px between elements
- Button spacing: 2px

## Use Cases

### DAW Timeline Track List

```demo
let theme = ui.ctx().armas_theme();

ui.vertical(|ui| {
    ui.heading("Tracks");
    ui.separator();

    let tracks = vec![
        ("Lead Vocals", egui::Color32::from_rgb(255, 100, 100)),
        ("Backing Vocals", egui::Color32::from_rgb(255, 150, 150)),
        ("Acoustic Guitar", egui::Color32::from_rgb(200, 150, 100)),
        ("Electric Guitar", egui::Color32::from_rgb(255, 200, 50)),
        ("Bass", egui::Color32::from_rgb(100, 150, 255)),
        ("Drums", egui::Color32::from_rgb(150, 255, 150)),
        ("Synth Pad", egui::Color32::from_rgb(200, 100, 255)),
    ];

    for (track_name, color) in tracks {
        let mut name = track_name.to_string();
        let mut controls = TrackControls::default();
        TrackHeader::new()
            .width(220.0)
            .height(55.0)
            .color(color)
            .compact(true)
            .show(ui, &mut name, &mut controls, &mut false, &theme);
        ui.add_space(2.0);
    }
});
```

### Mixer Channel Strip

```demo
let theme = ui.ctx().armas_theme();

ui.horizontal(|ui| {
    let colors = vec![
        egui::Color32::from_rgb(255, 100, 100),
        egui::Color32::from_rgb(100, 255, 100),
        egui::Color32::from_rgb(100, 100, 255),
        egui::Color32::from_rgb(255, 200, 50),
    ];

    for (i, color) in colors.into_iter().enumerate() {
        ui.vertical(|ui| {
            let mut name = format!("Ch {}", i + 1);
            let mut controls = TrackControls::default();

            TrackHeader::new()
                .width(100.0)
                .height(80.0)
                .color(color)
                .show(ui, &mut name, &mut controls, &mut false, &theme);
        });
    }
});
```

## Performance

- **Minimal allocations**: Direct painting with temporary string clone only when name changes
- **Efficient rendering**: Uses egui's standard UI building blocks (Button, Input)
- **No animations**: Static UI for consistent performance
- **Scales well**: Suitable for 100+ tracks in a DAW session

## Dependencies

- `egui = "0.33"`
- Uses Armas components: `Button`, `Input`
- Theme colors: `surface`, `outline_variant`, `primary`
- Minimum version: `armas 0.1.0`
