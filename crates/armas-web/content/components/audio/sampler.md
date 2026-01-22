# Sampler UI Components

Professional audio sampler interface components for DAW-style applications. Build single or multi-zone samplers with waveform editing, playback controls, and ADSR envelopes.

## Features

- **Waveform Display** - Interactive audio visualization with draggable markers
- **Playback Parameters** - Pitch, pan, volume, and loop controls
- **ADSR Envelope** - Attack, Decay, Sustain, Release editing with automation curves
- **Sample Zones** - Composable single-zone sampler with all controls
- **Generic Data** - Works with any sample data format
- **Theme Integration** - Full Armas theme system support
- **Pure UI** - No file I/O or audio processing

## Component Overview

The sampler system consists of four independent, composable components:

```
WaveformDisplay
├── Canvas rendering
├── Draggable markers (start/end)
├── Loop region highlighting
└── Playhead indicator

PlaybackParameters
├── Pitch slider (-24 to +24 semitones)
├── Pan slider (-1.0 to +1.0)
├── Volume slider (0.0 to 1.0)
├── Loop toggle
└── Loop length slider

EnvelopeSection
├── ADSR curve visualization
├── Attack/Decay/Sustain/Release sliders
└── AutomationEditor integration

SampleZone (Composite)
├── WaveformDisplay
├── PlaybackParameters
└── EnvelopeSection
```

## Basic Usage

### Single Zone Sampler

```rust,ignore
use armas::components::audio::{SampleZone, PlaybackParams, ADSREnvelope};

let mut params = PlaybackParams::default();
let mut envelope = ADSREnvelope::default();

let response = SampleZone::new(
    "Kick Drum".to_string(),
    sample_data,
    |sample| sample.abs(),  // amplitude function
    duration,
    sample_rate,
    &mut params,
    &mut envelope,
    theme
)
.waveform_size(egui::Vec2::new(800.0, 200.0))
.show_envelope(true)
.show(ui);

if response.playback_params_changed {
    println!("Playback params updated");
}
```

### Multi-Zone Sampler

```rust,ignore
struct MultiZoneSampler {
    zones: Vec<SamplerZoneData>,
    selected_zone: usize,
}

impl MultiZoneSampler {
    fn show(&mut self, ui: &mut egui::Ui) {
        // Zone selector
        ui.horizontal(|ui| {
            for (idx, zone) in self.zones.iter().enumerate() {
                if ui.selectable_label(idx == self.selected_zone, &zone.name).clicked() {
                    self.selected_zone = idx;
                }
            }
        });

        ui.separator();

        // Show selected zone
        let zone = &mut self.zones[self.selected_zone];
        SampleZone::new(
            zone.name.clone(),
            &zone.samples,
            |s| s.abs(),
            zone.duration,
            44100,
            &mut zone.params,
            &mut zone.envelope,
            theme
        )
        .show(ui);
    }
}
```

## API Reference

### WaveformDisplay

Generic waveform visualization component.

#### Creating a WaveformDisplay

```rust,ignore
use armas::components::audio::WaveformDisplay;

let waveform = WaveformDisplay::new(
    sample_data,           // &[T] - your sample data
    |sample| sample.abs(), // amplitude function
    10.0,                  // duration in seconds
    44100,                 // sample rate
    theme
)
.config(config)
.playhead(2.5)
.sample_bounds(0.0, 10.0)
.loop_region(2.0, 8.0)
.loop_enabled(true)
.show(ui, egui::Vec2::new(800.0, 200.0));
```

#### Builder Methods

##### `.config(config: WaveformConfig)`
Configure grid, colors, and scale settings.

```rust,ignore
let config = WaveformConfig {
    pixels_per_second: 100.0,
    height: 200.0,
    peak_color: Color32::from_rgb(100, 200, 255),
    rms_color: Color32::from_rgb(100, 200, 255).gamma_multiply(0.5),
    show_grid: true,
    grid_interval: 1.0,
};

waveform.config(config)
```

##### `.playhead(pos: f64)`
Set the playhead position in seconds.

```rust,ignore
waveform.playhead(2.5)  // Show playhead at 2.5 seconds
```

##### `.sample_bounds(start: f64, end: f64)`
Set the sample start and end markers position (in seconds).

```rust,ignore
waveform.sample_bounds(0.1, 9.9)
```

##### `.loop_region(start: f64, end: f64)`
Set the loop region boundaries (in seconds).

```rust,ignore
waveform.loop_region(2.0, 8.0)
```

##### `.loop_enabled(enabled: bool)`
Enable/disable loop region visualization.

```rust,ignore
waveform.loop_enabled(true)
```

### PlaybackParameters

Controls for pitch, pan, volume, and loop settings.

#### Creating PlaybackParameters

```rust,ignore
use armas::components::audio::PlaybackParamsUI;

let mut params = PlaybackParams {
    pitch: 0.0,
    pan: 0.0,
    volume: 0.8,
    loop_enabled: false,
    loop_length: 1.0,
};

let response = PlaybackParamsUI::new(&mut params, theme)
    .show_loop_controls(true)
    .show(ui);
```

#### Response

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

#### Builder Methods

##### `.show_loop_controls(show: bool)`
Show or hide loop-related controls.

```rust,ignore
PlaybackParamsUI::new(&mut params, theme)
    .show_loop_controls(false)  // Hide loop controls
    .show(ui)
```

### EnvelopeSection

ADSR envelope editor with automation visualization.

#### Creating EnvelopeSection

```rust,ignore
use armas::components::audio::EnvelopeSection;

let mut envelope = ADSREnvelope::default();

let response = EnvelopeSection::new(&mut envelope, theme)
    .canvas_size(egui::Vec2::new(600.0, 200.0))
    .show_labels(true)
    .show(ui);
```

#### ADSR Parameters

```rust
pub struct ADSREnvelope {
    pub attack: f32,      // 0-5 seconds
    pub decay: f32,       // 0-5 seconds
    pub sustain: f32,     // 0.0-1.0 (level)
    pub release: f32,     // 0-5 seconds
    pub automation: AutomationEnvelope<CurveType>,
}
```

#### Response

```rust
pub struct EnvelopeSectionResponse {
    pub attack_changed: bool,
    pub decay_changed: bool,
    pub sustain_changed: bool,
    pub release_changed: bool,
    pub envelope: ADSREnvelope,
}
```

#### Builder Methods

##### `.canvas_size(size: Vec2)`
Set the automation curve canvas size.

```rust,ignore
EnvelopeSection::new(&mut envelope, theme)
    .canvas_size(egui::Vec2::new(600.0, 150.0))
    .show(ui)
```

##### `.show_labels(show: bool)`
Show or hide parameter labels.

```rust,ignore
EnvelopeSection::new(&mut envelope, theme)
    .show_labels(true)
    .show(ui)
```

### SampleZone

Complete single-zone sampler combining all components.

#### Creating a SampleZone

```rust,ignore
use armas::components::audio::SampleZone;

let mut params = PlaybackParams::default();
let mut envelope = ADSREnvelope::default();

let response = SampleZone::new(
    "Piano".to_string(),
    sample_data,
    |sample| sample.abs(),
    duration,
    sample_rate,
    &mut params,
    &mut envelope,
    theme
)
.waveform_size(egui::Vec2::new(800.0, 200.0))
.show_envelope(true)
.show(ui);
```

#### Response

```rust
pub struct SampleZoneResponse {
    pub playback_params_changed: bool,
    pub envelope_changed: bool,
}
```

#### Builder Methods

##### `.waveform_config(config: WaveformConfig)`
Configure the waveform display.

```rust,ignore
let config = WaveformConfig {
    pixels_per_second: 80.0,
    ..Default::default()
};

SampleZone::new(...)
    .waveform_config(config)
    .show(ui)
```

##### `.waveform_size(size: Vec2)`
Set the waveform display dimensions.

```rust,ignore
SampleZone::new(...)
    .waveform_size(egui::Vec2::new(800.0, 250.0))
    .show(ui)
```

##### `.show_envelope(show: bool)`
Show or hide the ADSR envelope section.

```rust,ignore
SampleZone::new(...)
    .show_envelope(true)   // Show envelope
    .show(ui)
```

## Data Types

### PlaybackParams

```rust
pub struct PlaybackParams {
    pub pitch: f32,        // -24 to +24 semitones
    pub pan: f32,          // -1.0 (left) to 1.0 (right)
    pub volume: f32,       // 0.0 to 1.0
    pub loop_enabled: bool,
    pub loop_length: f64,  // 0.1 to 60.0 seconds
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

### ADSREnvelope

```rust
pub struct ADSREnvelope {
    pub attack: f32,       // 0-5 seconds
    pub decay: f32,        // 0-5 seconds
    pub sustain: f32,      // 0.0-1.0
    pub release: f32,      // 0-5 seconds
    pub automation: AutomationEnvelope<CurveType>,
}

impl Default for ADSREnvelope {
    fn default() -> Self {
        // Creates default ADSR shape
    }
}
```

### WaveformConfig

```rust
pub struct WaveformConfig {
    pub pixels_per_second: f32,   // Default: 100.0
    pub height: f32,               // Default: 150.0
    pub peak_color: Color32,       // Peak waveform color
    pub rms_color: Color32,        // RMS level color
    pub show_grid: bool,           // Show time grid
    pub grid_interval: f32,        // Grid spacing in seconds
}

impl Default for WaveformConfig { ... }
```

## Styling & Theme Integration

All sampler components use the Armas theme system for consistent styling:

- **Waveform colors** - Configurable via `WaveformConfig`
- **Grid colors** - Primary and outline variant from theme
- **Playhead** - Secondary color with glow effect
- **Markers** - Green (start) and red (end) with custom styling
- **Automation curve** - Customizable via point color
- **Text labels** - On-surface variant text
- **Backgrounds** - Surface color with rounded corners

### Custom Styling Example

```rust,ignore
let config = WaveformConfig {
    pixels_per_second: 120.0,
    height: 250.0,
    peak_color: Color32::from_rgb(100, 200, 255),
    rms_color: Color32::from_rgb(100, 200, 255).gamma_multiply(0.5),
    show_grid: true,
    grid_interval: 0.5,
};

SampleZone::new(...)
    .waveform_config(config)
    .show(ui)
```

## Complete Example

Full working example with a single sample zone:

```rust,ignore
use armas::components::audio::{SampleZone, PlaybackParams, ADSREnvelope, WaveformConfig};
use egui::{Color32, Vec2};

struct SamplerDemo {
    sample_data: Vec<f32>,
    duration: f64,
    playback_params: PlaybackParams,
    envelope: ADSREnvelope,
}

impl SamplerDemo {
    fn new(sample_data: Vec<f32>) -> Self {
        Self {
            duration: sample_data.len() as f64 / 44100.0,
            sample_data,
            playback_params: PlaybackParams::default(),
            envelope: ADSREnvelope::default(),
        }
    }

    fn show(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let config = WaveformConfig {
            pixels_per_second: 100.0,
            height: 200.0,
            peak_color: Color32::from_rgb(100, 200, 255),
            rms_color: Color32::from_rgb(100, 200, 255).gamma_multiply(0.5),
            show_grid: true,
            grid_interval: 1.0,
        };

        let response = SampleZone::new(
            "Piano Sample".to_string(),
            &self.sample_data,
            |sample| sample.abs(),
            self.duration,
            44100,
            &mut self.playback_params,
            &mut self.envelope,
            theme
        )
        .waveform_config(config)
        .waveform_size(Vec2::new(800.0, 250.0))
        .show_envelope(true)
        .show(ui);

        // Handle changes
        if response.playback_params_changed {
            println!("Pitch: {}", self.playback_params.pitch);
            println!("Pan: {}", self.playback_params.pan);
            println!("Volume: {}", self.playback_params.volume);
        }

        if response.envelope_changed {
            println!("ADSR changed");
            println!("A: {}, D: {}, S: {}, R: {}",
                self.envelope.attack,
                self.envelope.decay,
                self.envelope.sustain,
                self.envelope.release
            );
        }
    }
}
```

## Performance Considerations

- **Waveform rendering** - Samples at configurable intervals (default: 1 sample per pixel)
- **Grid rendering** - Only visible content is drawn
- **Automation curve** - Sampled at 100+ points per frame
- **Theme caching** - Retrieved once per frame

For very long samples (1000+ seconds):
- Increase `pixels_per_second` to reduce detail
- Implement zoom/pan controls
- Consider chunked rendering for zoomed-in views

## Composability

Each component is independent and can be used separately:

```rust,ignore
// Just the waveform
WaveformDisplay::new(data, fn, 10.0, 44100, theme).show(ui, size);

// Just the controls
PlaybackParamsUI::new(&mut params, theme).show(ui);

// Just the envelope
EnvelopeSection::new(&mut envelope, theme).show(ui);

// All together
SampleZone::new(...).show(ui);
```

## Next Steps

The sampler system is purely presentational. To build a complete sampler, add:

1. **Audio engine** - Sample playback and effects processing
2. **File loader** - Load sample data from disk
3. **Multi-zone manager** - Create, delete, and organize zones
4. **Persistence** - Save/load sampler configurations
5. **MIDI integration** - Trigger samples from MIDI keyboard
6. **Advanced controls** - Filters, LFO, effects, modulation

All components integrate seamlessly with your audio pipeline.
