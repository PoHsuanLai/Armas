# Envelope Section (ADSR)

ADSR envelope editor with automation curve visualization. Features attack, decay, sustain, and release controls with interactive curve editing and real-time feedback.

## Basic Usage

```demo
use egui::Id;

let id = Id::new("envelope_basic");
let mut envelope: ADSREnvelope = ui.data_mut(|d|
    d.get_temp(id).unwrap_or_default()
);

let response = EnvelopeSection::new(&mut envelope, &theme)
    .show_labels(true)
    .show(ui);

ui.data_mut(|d| d.insert_temp(id, envelope));

if response.attack_changed {
    // Attack time updated
}
```

## Custom Canvas Size

```demo
use egui::Id;

let id = Id::new("envelope_custom_size");
let mut envelope: ADSREnvelope = ui.data_mut(|d|
    d.get_temp(id).unwrap_or_default()
);

let response = EnvelopeSection::new(&mut envelope, &theme)
    .canvas_size(egui::Vec2::new(600.0, 250.0))
    .show_labels(true)
    .show(ui);

ui.data_mut(|d| d.insert_temp(id, envelope));
```

## Without Labels

```demo
use egui::Id;

let id = Id::new("envelope_no_labels");
let mut envelope: ADSREnvelope = ui.data_mut(|d|
    d.get_temp(id).unwrap_or_default()
);

let response = EnvelopeSection::new(&mut envelope, &theme)
    .canvas_size(egui::Vec2::new(600.0, 180.0))
    .show_labels(false)
    .show(ui);

ui.data_mut(|d| d.insert_temp(id, envelope));
```

## With Multiple Presets

```demo
use egui::Id;

let id = Id::new("envelope_presets");
let mut envelope: ADSREnvelope = ui.data_mut(|d|
    d.get_temp(id).unwrap_or_default()
);

// Preset buttons
ui.horizontal(|ui| {
    if ui.button("Percussion").clicked() {
        envelope = ADSREnvelope {
            attack: 0.01,
            decay: 0.3,
            sustain: 0.0,
            release: 0.2,
            ..Default::default()
        };
    }
    if ui.button("Pad").clicked() {
        envelope = ADSREnvelope {
            attack: 1.0,
            decay: 2.0,
            sustain: 0.8,
            release: 1.5,
            ..Default::default()
        };
    }
    if ui.button("Piano").clicked() {
        envelope = ADSREnvelope {
            attack: 0.05,
            decay: 0.5,
            sustain: 0.0,
            release: 1.0,
            ..Default::default()
        };
    }
});

let response = EnvelopeSection::new(&mut envelope, &theme)
    .show_labels(true)
    .show(ui);

ui.data_mut(|d| d.insert_temp(id, envelope));
```

## Parameter Display

```demo
use egui::Id;

let id = Id::new("envelope_display");
let mut envelope: ADSREnvelope = ui.data_mut(|d|
    d.get_temp(id).unwrap_or_default()
);

let response = EnvelopeSection::new(&mut envelope, &theme)
    .show_labels(true)
    .show(ui);

// Show current values
ui.separator();
ui.label("Current ADSR:");
ui.horizontal(|ui| {
    ui.label(format!("A: {:.2}s", envelope.attack));
    ui.label(format!("D: {:.2}s", envelope.decay));
    ui.label(format!("S: {:.1}%", envelope.sustain * 100.0));
    ui.label(format!("R: {:.2}s", envelope.release));
});

ui.data_mut(|d| d.insert_temp(id, envelope));
```

## API Reference

### ADSREnvelope Data Structure

```rust
pub struct ADSREnvelope {
    pub attack: f32,          // 0-5 seconds
    pub decay: f32,           // 0-5 seconds
    pub sustain: f32,         // 0.0-1.0 (normalized level)
    pub release: f32,         // 0-5 seconds
    pub automation: AutomationEnvelope<CurveType>,
}

impl Default for ADSREnvelope {
    fn default() -> Self {
        Self {
            attack: 0.1,
            decay: 0.2,
            sustain: 0.7,
            release: 0.2,
            // ... automation setup
        }
    }
}
```

### EnvelopeSection

```rust
pub struct EnvelopeSection<'a> {
    envelope: &'a mut ADSREnvelope,
    theme: &'a Theme,
    canvas_size: Vec2,
    show_labels: bool,
}

impl<'a> EnvelopeSection<'a> {
    pub fn new(envelope: &'a mut ADSREnvelope, theme: &'a Theme) -> Self

    pub fn canvas_size(mut self, size: Vec2) -> Self

    pub fn show_labels(mut self, show: bool) -> Self

    pub fn show(mut self, ui: &mut Ui) -> EnvelopeSectionResponse
}
```

### Response Structure

```rust
pub struct EnvelopeSectionResponse {
    pub attack_changed: bool,
    pub decay_changed: bool,
    pub sustain_changed: bool,
    pub release_changed: bool,
    pub envelope: ADSREnvelope,
}
```

### Builder Methods

| Method | Parameters | Description |
|--------|-----------|-------------|
| `.canvas_size()` | `Vec2` | Set automation curve canvas dimensions |
| `.show_labels()` | `bool` | Show/hide parameter labels |
| `.show()` | `&mut Ui` | Render the component |

## Parameter Ranges

| Parameter | Min | Max | Default | Unit |
|-----------|-----|-----|---------|------|
| Attack | 0.0 | 5.0 | 0.1 | seconds |
| Decay | 0.0 | 5.0 | 0.2 | seconds |
| Sustain | 0.0 | 1.0 | 0.7 | normalized (0-100%) |
| Release | 0.0 | 5.0 | 0.2 | seconds |

## Features

- **Visual Envelope Curve** - Shows ADSR shape in real-time
- **Interactive Controls** - Sliders for each parameter
- **Automation Integration** - Uses audio-automation crate for curves
- **Customizable Display** - Adjustable canvas size
- **Optional Labels** - Show/hide parameter names
- **Material Design 3** - Rounded corners and card-based layout
- **Real-time Updates** - Curve updates as parameters change

## Envelope Curve Types

The automation system supports multiple curve types:

- **Linear** - Straight line transitions (default)
- **Exponential** - Curved transitions for natural feel
- **Bezier** - Smooth curves with control points
- **Step** - Instant value changes

## Common Presets

### Percussion (Short, Snappy)
```rust
ADSREnvelope {
    attack: 0.01,    // Very fast attack
    decay: 0.3,      // Quick decay
    sustain: 0.0,    // No sustain
    release: 0.1,    // Quick release
    ..Default::default()
}
```

### Pad (Long, Sustained)
```rust
ADSREnvelope {
    attack: 1.0,     // Long fade-in
    decay: 2.0,      // Long decay
    sustain: 0.8,    // High sustain level
    release: 2.0,    // Long release tail
    ..Default::default()
}
```

### Piano (Transient)
```rust
ADSREnvelope {
    attack: 0.05,    // Fast attack
    decay: 0.5,      // Medium decay
    sustain: 0.0,    // No sustain (fades to silence)
    release: 1.0,    // Medium release
    ..Default::default()
}
```

### Pluck (Bright Decay)
```rust
ADSREnvelope {
    attack: 0.01,    // Immediate
    decay: 0.4,      // Fast bright decay
    sustain: 0.0,    // No sustain
    release: 0.3,    // Short tail
    ..Default::default()
}
```

## Styling

The component uses Material Design 3 with:

- **Card layout** - Separate cards for curve and parameters
- **Rounded corners** - 12px radius for modern feel
- **Primary color** - Curve rendered in theme primary color
- **Semantic labels** - Uses theme on_surface colors
- **Consistent spacing** - Theme-based margins

## Complete Example

```rust
use armas::components::audio::{ADSREnvelope, EnvelopeSection};
use egui::Vec2;

struct SamplerState {
    envelope: ADSREnvelope,
}

impl SamplerState {
    fn show(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let response = EnvelopeSection::new(&mut self.envelope, &theme)
            .canvas_size(Vec2::new(600.0, 200.0))
            .show_labels(true)
            .show(ui);

        // Track changes
        if response.attack_changed {
            println!("Attack: {:.2}s", self.envelope.attack);
        }
        if response.decay_changed {
            println!("Decay: {:.2}s", self.envelope.decay);
        }
        if response.sustain_changed {
            println!("Sustain: {:.1}%", self.envelope.sustain * 100.0);
        }
        if response.release_changed {
            println!("Release: {:.2}s", self.envelope.release);
        }
    }
}
```

## Integration with Sampler

The envelope is typically used as part of a complete sampler:

```rust
// In your sampler state
struct Sampler {
    envelope: ADSREnvelope,
    // ... other fields
}

// Show in UI
fn show(&mut self, ui: &mut Ui, theme: &Theme) {
    EnvelopeSection::new(&mut self.envelope, &theme)
        .canvas_size(Vec2::new(800.0, 200.0))
        .show(ui);
}
```

## Performance Considerations

- **Curve sampling** - 100+ points per frame for smooth visualization
- **Slider interactions** - Lightweight, no expensive calculations
- **Theme caching** - Retrieved once per frame
- **Memory** - Minimal state, only ADSR values + automation envelope
