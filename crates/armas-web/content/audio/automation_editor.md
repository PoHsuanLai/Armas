# AutomationEditor

Professional parameter automation curve editor for DAW-style applications. Provides timeline-based editing with multiple curve interpolation types, real-time visualization, and full coordinate system support for time and value ranges.

## Features

- **Curve type selection** - Linear, Exponential, Logarithmic, S-Curve, Stepped, Elastic, Bounce, Back, and Circular easing functions
- **Point-based editing** - Click to select, drag to move automation points with grid snapping
- **Real-time visualization** - Animated curve rendering with filled area under the curve
- **Value range display** - Y-axis labels with customizable marker count for quick reference
- **Grid canvas** - Time-based grid with beat subdivisions and configurable beat width
- **Playhead indicator** - Real-time playback position with glow effect
- **Theme-aware styling** - Full integration with Armas theme system
- **State management** - Click and drag detection with selection highlighting
- **Professional aesthetic** - Glassmorphic design matching Armas components

## Basic Usage

Create an automation envelope, configure the editor, and display it with automatic point editing. Click points to select them, drag to move them along the time and value axes with automatic grid snapping.

The response tracks changes, point selections, edits, additions, and deletions.

## API Reference

### Creating an AutomationEditor

To create an AutomationEditor, pass a mutable reference to an `AutomationEnvelope`:

```rust,ignore
let editor = AutomationEditor::new(&mut envelope);
```

### Builder Methods

#### `.canvas_size(size: Vec2)`
Set the dimensions of the automation editor canvas (default: 600x300).

```rust,ignore
let mut envelope = AutomationEnvelope::new(CurveType::Linear);

AutomationEditor::new(&mut envelope)
    .canvas_size(egui::Vec2::new(800.0, 400.0))
    .show(ui);
```

#### `.canvas_config(config: CanvasConfig)`
Configure the canvas grid, scaling, and snapping behavior.

```rust,ignore
use armas_audio::CanvasConfig;

let config = CanvasConfig {
    pixels_per_beat: 80.0,
    pixels_per_value: 50.0,
    min_value: 0.0,
    max_value: 100.0,
    grid_subdivisions: 4,
    snap_enabled: true,
    snap_interval: 0.125,
};

AutomationEditor::new(&mut envelope)
    .canvas_config(config)
    .show(ui);
```

#### `.playhead(pos: f64)`
Set the current playback position in beats for the playhead indicator (default: None).

```rust,ignore
let mut envelope = AutomationEnvelope::new(CurveType::Linear);

AutomationEditor::new(&mut envelope)
    .playhead(2.5)
    .show(ui);
```

#### `.point_color(color: Color32)`
Set the color for automation points and curve line (default: blue).

```rust,ignore
let mut envelope = AutomationEnvelope::new(CurveType::Linear);

AutomationEditor::new(&mut envelope)
    .point_color(egui::Color32::from_rgb(255, 150, 0))
    .show(ui);
```

#### `.show_values(show: bool)`
Enable/disable value label display on point hover (default: true).

```rust,ignore
let mut envelope = AutomationEnvelope::new(CurveType::Linear);

AutomationEditor::new(&mut envelope)
    .show_values(false)
    .show(ui);
```

#### `.selected_point(idx: Option<usize>)`
Set which automation point is currently selected for visual highlighting (default: None).

```rust,ignore
let mut envelope = AutomationEnvelope::new(CurveType::Linear);

AutomationEditor::new(&mut envelope)
    .selected_point(Some(0))
    .show(ui);
```

### Response

The `.show()` method returns an `AutomationEditorResponse` containing:

```rust
pub struct AutomationEditorResponse {
    /// Whether any changes were made to the automation curve
    pub changed: bool,

    /// Index of currently selected point
    pub selected_point: Option<usize>,

    /// (index, point) if a point was edited (moved)
    pub point_edited: Option<(usize, AutomationPoint)>,

    /// Index if a point was added
    pub point_added: Option<usize>,

    /// Index if a point was deleted
    pub point_deleted: Option<usize>,
}
```

## Canvas Configuration

The `CanvasConfig` struct controls how the automation grid is displayed:

```rust
pub struct CanvasConfig {
    /// Horizontal pixels per beat (controls time scale)
    pub pixels_per_beat: f32,

    /// Vertical pixels per unit value (controls value scale)
    pub pixels_per_value: f32,

    /// Minimum value on Y axis
    pub min_value: f32,

    /// Maximum value on Y axis
    pub max_value: f32,

    /// Number of beat subdivisions to show grid lines for
    pub grid_subdivisions: usize,

    /// Enable/disable grid snapping for point positions
    pub snap_enabled: bool,

    /// Snap interval in beats (e.g., 0.25 = snap to sixteenth notes)
    pub snap_interval: f64,
}
```

## Interaction

### Point Selection
- **Click** on a point to select it (visual highlight shows selected state)
- Selected points display their value on hover

### Point Editing
- **Drag** a selected point to move it in time and value
- Points snap to the grid when snapping is enabled
- Value changes are clamped to the configured min/max range
- Time changes are clamped to non-negative values

### Curve Type Selection
- Use the curve type selector toolbar to switch between interpolation methods
- Available types: Linear, Exponential, Logarithmic, S-Curve, Stepped, Elastic, Bounce, Back, Circular

## Subcomponents

The AutomationEditor is composed of several specialized subcomponents:

### AutomationCanvas
Renders the main canvas with grid, background, and playhead.

```demo
let theme = ui.ctx().armas_theme();
let config = CanvasConfig::default();

let canvas_response = AutomationCanvas::new(&theme)
    .config(config)
    .playhead(4.5)
    .show(ui, egui::Vec2::new(600.0, 300.0));
```

### PointHandle
Interactive circular handle for editing individual automation points.

```demo
let theme = ui.ctx().armas_theme();
let point_pos = egui::Pos2::new(100.0, 150.0);

let response = PointHandle::new(point_pos, egui::Color32::BLUE)
    .selected(false)
    .show_value(true)
    .value_text("0.75".to_string())
    .show(ui, &theme);

if let Some(delta) = response.drag_delta {
    // Point was dragged by delta amount
}
```

### CurveTypeSelector
Toolbar for selecting automation curve interpolation types.

```rust,ignore
let selector_response = CurveTypeSelector::new(audio_automation::CurveType::Linear)
    .show(ui);

if selector_response.changed {
    // Curve type was changed
}
```

### ValueRangeDisplay
Y-axis value labels and grid markers.

```demo
ValueRangeDisplay::new(0.0, 100.0)
    .markers(5)
    .show(ui, 300.0);
```

## Styling & Theme Integration

The AutomationEditor uses the Armas theme system for consistent styling:

- **Grid lines** - Primary color (at beat divisions), outline variant (subdivisions)
- **Playhead** - Secondary color with subtle glow effect
- **Points** - Customizable via `.point_color()`
- **Labels** - On-surface variant text
- **Background** - Surface color

Example of customizing point color to match your design:

```rust,ignore
let mut envelope = AutomationEnvelope::new(CurveType::Linear);

AutomationEditor::new(&mut envelope)
    .canvas_size(egui::Vec2::new(600.0, 300.0))
    .point_color(egui::Color32::from_rgb(100, 150, 255))
    .show(ui);
```

## Complete Example

Full example showing a real-world automation editor setup:

```rust,ignore
use audio_automation::{AutomationEnvelope, AutomationPoint, CurveType};
use armas_audio::{AutomationEditor, CanvasConfig};
use egui::Color32;

struct AutomationEditorDemo {
    envelope: AutomationEnvelope<f32>,
    playhead_position: f64,
    selected_point: Option<usize>,
}

impl AutomationEditorDemo {
    fn new() -> Self {
        let mut envelope = AutomationEnvelope::new(CurveType::Linear);
        envelope.add_point(AutomationPoint::new(0.0, 0.0));
        envelope.add_point(AutomationPoint::new(2.0, 0.5));
        envelope.add_point(AutomationPoint::new(4.0, 1.0));

        Self {
            envelope,
            playhead_position: 0.0,
            selected_point: None,
        }
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        let config = CanvasConfig {
            pixels_per_beat: 60.0,
            pixels_per_value: 40.0,
            min_value: 0.0,
            max_value: 1.0,
            grid_subdivisions: 4,
            snap_enabled: true,
            snap_interval: 0.25,
        };

        let response = AutomationEditor::new(&mut self.envelope)
            .canvas_size(egui::Vec2::new(600.0, 300.0))
            .canvas_config(config)
            .playhead(self.playhead_position)
            .selected_point(self.selected_point)
            .point_color(Color32::from_rgb(100, 150, 255))
            .show_values(true)
            .show(ui);

        // Handle responses
        if response.changed {
            if let Some(idx) = response.selected_point {
                self.selected_point = Some(idx);
            }

            if let Some((idx, point)) = response.point_edited {
                println!("Point {} edited: time={:.2}, value={:.2}", idx, point.time, point.value);
            }
        }

        // Simulate playhead movement
        self.playhead_position += 0.01;
    }
}
```

## Performance Considerations

- The curve rendering samples the automation envelope at regular intervals (100+ samples per frame by default)
- Grid rendering uses conditional drawing to avoid rendering invisible content
- Point handles are rendered individually with hover detection
- Theme colors are cached per frame via `ui.ctx().armas_theme()`

For very long automation curves (1000+ points), consider:
- Increasing the sample step interval
- Using LOD (level-of-detail) rendering for zoomed-out views
- Caching rendered curve data between frames when possible
