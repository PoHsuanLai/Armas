# Playback Parameters

Sample playback control component with pitch, pan, volume, and loop settings. Features smooth sliders with real-time feedback and optional loop controls.

## Basic Usage

```demo
use egui::Id;

let id = Id::new("playback_params_basic");
let mut params: PlaybackParams = ui.data_mut(|d|
    d.get_temp(id).unwrap_or_default()
);

let response = PlaybackParamsUI::new(&mut params, &theme)
    .show_loop_controls(true)
    .show(ui);

ui.data_mut(|d| d.insert_temp(id, params));

if response.pitch_changed {
    // Update pitch
}
```

## Without Loop Controls

```demo
use egui::Id;

let id = Id::new("playback_params_no_loop");
let mut params: PlaybackParams = ui.data_mut(|d|
    d.get_temp(id).unwrap_or_default()
);

let response = PlaybackParamsUI::new(&mut params, &theme)
    .show_loop_controls(false)
    .show(ui);

ui.data_mut(|d| d.insert_temp(id, params));
```

## With Pitch Shifting Example

```demo
use egui::Id;

let id = Id::new("playback_params_pitch");
let mut params: PlaybackParams = ui.data_mut(|d|
    d.get_temp(id).unwrap_or_default()
);

let response = PlaybackParamsUI::new(&mut params, &theme)
    .show_loop_controls(true)
    .show(ui);

if response.pitch_changed {
    ui.label(format!("Pitch: {:.1} semitones", params.pitch));
}

ui.data_mut(|d| d.insert_temp(id, params));
```

## Multiple Tracks (Mixer Style)

```demo
use egui::Id;

ui.label("Track Mixing");
ui.separator();

ui.horizontal(|ui| {
    for track_idx in 0..3 {
        ui.vertical(|ui| {
            let id = Id::new(format!("mixer_track_{}", track_idx));
            let mut params: PlaybackParams = ui.data_mut(|d|
                d.get_temp(id).unwrap_or_default()
            );

            ui.label(format!("Track {}", track_idx + 1));

            let response = PlaybackParamsUI::new(&mut params, &theme)
                .show_loop_controls(false)
                .show(ui);

            ui.data_mut(|d| d.insert_temp(id, params));
        });
    }
});
```

## API Reference

### PlaybackParams Data Structure

```rust
pub struct PlaybackParams {
    pub pitch: f32,          // -24 to +24 semitones
    pub pan: f32,            // -1.0 (left) to 1.0 (right)
    pub volume: f32,         // 0.0 to 1.0
    pub loop_enabled: bool,
    pub loop_length: f64,    // 0.1 to 60.0 seconds
}

impl Default for PlaybackParams {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            pan: 0.0,
            volume: 0.8,
            loop_enabled: false,
            loop_length: 1.0,
        }
    }
}
```

### PlaybackParamsUI

```rust
pub struct PlaybackParamsUI<'a> {
    params: &'a mut PlaybackParams,
    theme: &'a Theme,
    show_loop_controls: bool,
}

impl<'a> PlaybackParamsUI<'a> {
    pub fn new(params: &'a mut PlaybackParams, theme: &'a Theme) -> Self

    pub fn show_loop_controls(mut self, show: bool) -> Self

    pub fn show(self, ui: &mut Ui) -> PlaybackParamsResponse
}
```

### Response Structure

```rust
pub struct PlaybackParamsResponse {
    pub pitch_changed: bool,
    pub pan_changed: bool,
    pub volume_changed: bool,
    pub loop_enabled_changed: bool,
    pub loop_length_changed: bool,
    pub params: PlaybackParams,
}
```

### Builder Methods

| Method | Parameters | Description |
|--------|-----------|-------------|
| `.show_loop_controls()` | `bool` | Show/hide loop-related controls |
| `.show()` | `&mut Ui` | Render the component and return response |

## Features

- **Pitch Control** - ±24 semitones (octave range)
- **Pan Stereo** - Full stereo range with center detent
- **Volume Control** - Linear 0-100% range
- **Loop Toggle** - Enable/disable sample looping
- **Loop Length** - Adjustable loop duration
- **Joyful Design** - Rounded corners and Material Design 3 styling
- **Real-time Feedback** - Immediate visual feedback on all controls

## Parameter Ranges

| Parameter | Min | Max | Default | Unit |
|-----------|-----|-----|---------|------|
| Pitch | -24 | 24 | 0 | semitones |
| Pan | -1.0 | 1.0 | 0 | normalized (-L to +R) |
| Volume | 0.0 | 1.0 | 0.8 | normalized (0-100%) |
| Loop Length | 0.1 | 60.0 | 1.0 | seconds |

## Styling

The component uses Material Design 3 principles with:

- **Rounded corners** - 12px radius for modern feel
- **Card layout** - Filled card variant with subtle background
- **Semantic colors** - Uses theme on_surface and on_surface_variant
- **Consistent spacing** - Theme-based margins and padding
- **Visual hierarchy** - Section titles with subtle colors

## Usage Patterns

### Pattern 1: Single Zone Sampler

```rust
let mut params = PlaybackParams::default();

let response = PlaybackParamsUI::new(&mut params, theme)
    .show_loop_controls(true)
    .show(ui);

if response.pitch_changed {
    update_sample_pitch(params.pitch);
}
```

### Pattern 2: Multi-Track Mixer

```rust
for (idx, track) in self.tracks.iter_mut().enumerate() {
    let response = PlaybackParamsUI::new(&mut track.params, theme)
        .show_loop_controls(false)
        .show(ui);

    if response.volume_changed {
        update_track_volume(idx, track.params.volume);
    }
}
```

### Pattern 3: Preset System

```rust
let preset_values = vec![
    PlaybackParams { pitch: -12.0, ..Default::default() }, // one octave down
    PlaybackParams { pitch: 0.0, ..Default::default() },   // original
    PlaybackParams { pitch: 12.0, ..Default::default() },  // one octave up
];

if ui.button("Apply Preset 1").clicked() {
    *params = preset_values[0];
}
```

## Complete Example

```rust
use armas_audio::{PlaybackParams, PlaybackParamsUI};

struct SamplerState {
    params: PlaybackParams,
}

impl SamplerState {
    fn show(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let response = PlaybackParamsUI::new(&mut self.params, theme)
            .show_loop_controls(true)
            .show(ui);

        // Handle changes
        if response.pitch_changed {
            println!("Pitch: {} semitones", self.params.pitch);
        }

        if response.pan_changed {
            println!("Pan: {}", self.params.pan);
        }

        if response.volume_changed {
            println!("Volume: {:.1}%", self.params.volume * 100.0);
        }

        if response.loop_enabled_changed {
            println!("Loop: {}", self.params.loop_enabled);
        }

        if response.loop_length_changed {
            println!("Loop length: {:.2}s", self.params.loop_length);
        }
    }
}
```

## Performance

- **Lightweight** - No heavy calculations, purely UI
- **Responsive** - Immediate feedback on slider interactions
- **Memory efficient** - Minimal state management
- **Theme cached** - Retrieved once per frame
