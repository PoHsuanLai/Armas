# Waveform Display

Interactive audio waveform visualization component for DAW-style applications. Displays peak and RMS levels with customizable grid, playhead, and draggable markers.

## Basic Usage

```demo
use egui::Id;

// Generate sample data
let sample_data: Vec<f32> = (0..44100)
    .map(|i| (2.0 * std::f32::consts::PI * i as f32 / 44100.0 * 440.0).sin() * 0.8)
    .collect();

let id = Id::new("waveform_basic");
let waveform_size = egui::Vec2::new(800.0, 200.0);

WaveformDisplay::new(&sample_data, |s| s.abs(), 1.0, 44100, &theme)
    .show(ui, waveform_size);
```

## With Playhead

```demo
use egui::Id;

// Generate sample data
let sample_data: Vec<f32> = (0..44100)
    .map(|i| (2.0 * std::f32::consts::PI * i as f32 / 44100.0 * 440.0).sin() * 0.8)
    .collect();

let id = Id::new("waveform_playhead");
let mut playhead_pos: f64 = ui.data_mut(|d| d.get_temp(Id::new("playhead_pos")).unwrap_or(0.3));

WaveformDisplay::new(&sample_data, |s| s.abs(), 1.0, 44100, &theme)
    .playhead(playhead_pos)
    .show(ui, egui::Vec2::new(800.0, 200.0));

ui.data_mut(|d| d.insert_temp(Id::new("playhead_pos"), playhead_pos));
```

## With Loop Region

```demo
use egui::Id;

// Generate sample data
let sample_data: Vec<f32> = (0..44100)
    .map(|i| (2.0 * std::f32::consts::PI * i as f32 / 44100.0 * 440.0).sin() * 0.8)
    .collect();

WaveformDisplay::new(&sample_data, |s| s.abs(), 1.0, 44100, &theme)
    .loop_region(0.2, 0.8)
    .loop_enabled(true)
    .show(ui, egui::Vec2::new(800.0, 200.0));
```

## Custom Configuration

```demo
use egui::Id;

// Generate sample data
let sample_data: Vec<f32> = (0..88200)
    .map(|i| (2.0 * std::f32::consts::PI * i as f32 / 44100.0 * 220.0).sin() * 0.7)
    .collect();

let config = WaveformConfig {
    pixels_per_second: 80.0,
    height: 200.0,
    peak_color: egui::Color32::from_rgb(100, 200, 255),
    rms_color: egui::Color32::from_rgb(100, 200, 255).gamma_multiply(0.5),
    show_grid: true,
    grid_interval: 0.5,
    color_by_frequency: false,
    show_spectrogram: false,
    zoom_level: 1.0,
};

WaveformDisplay::new(&sample_data, |s| s.abs(), 2.0, 44100, &theme)
    .config(config)
    .show(ui, egui::Vec2::new(800.0, 250.0));
```

## With Sample Bounds

```demo
use egui::Id;

// Generate sample data
let sample_data: Vec<f32> = (0..44100)
    .map(|i| (2.0 * std::f32::consts::PI * i as f32 / 44100.0 * 440.0).sin() * 0.8)
    .collect();

WaveformDisplay::new(&sample_data, |s| s.abs(), 1.0, 44100, &theme)
    .sample_bounds(0.1, 0.9)
    .show(ui, egui::Vec2::new(800.0, 200.0));
```

## API Reference

### Creating a WaveformDisplay

```rust
pub struct WaveformDisplay<'a, T> {
    sample_data: &'a [T],
    amplitude_fn: fn(&T) -> f32,
    duration: f64,
    sample_rate: u32,
    theme: &'a Theme,
}

impl<'a, T> WaveformDisplay<'a, T> {
    pub fn new(
        sample_data: &'a [T],
        amplitude_fn: fn(&T) -> f32,
        duration: f64,
        sample_rate: u32,
        theme: &'a Theme,
    ) -> Self
}
```

### Builder Methods

| Method | Parameters | Description |
|--------|-----------|-------------|
| `.config()` | `WaveformConfig` | Configure grid, colors, and display settings |
| `.playhead()` | `f64` (seconds) | Set playhead position |
| `.sample_bounds()` | `(f64, f64)` | Set start and end marker positions |
| `.loop_region()` | `(f64, f64)` | Set loop region boundaries |
| `.loop_enabled()` | `bool` | Enable/disable loop region display |
| `.show()` | `(&mut Ui, Vec2)` | Render the waveform |

### WaveformConfig

```rust
pub struct WaveformConfig {
    pub pixels_per_second: f32,   // Horizontal zoom (default: 100.0)
    pub height: f32,               // Display height (default: 150.0)
    pub peak_color: Color32,       // Peak waveform color
    pub rms_color: Color32,        // RMS level color
    pub show_grid: bool,           // Show time grid (default: true)
    pub grid_interval: f32,        // Grid spacing in seconds (default: 1.0)
    pub color_by_frequency: bool,  // Warm/cool colors by frequency (default: false)
    pub show_spectrogram: bool,    // Enable spectrogram mode (default: false)
    pub zoom_level: f32,           // Zoom multiplier 1.0 = normal (default: 1.0)
}
```

## Features

- **Generic Sample Data** - Works with any type (f32, i16, etc.)
- **Peak & RMS Display** - Shows both peak and average levels with vertical bars
- **Interactive Playhead** - Draggable playhead indicator
- **Sample Markers** - Draggable start/end markers
- **Loop Region** - Visual loop region highlighting
- **Configurable Grid** - Adjustable time grid
- **Theme Integration** - Full theme system support
- **Performance Optimized** - Efficient rendering for long samples

## Performance Tips

- **Long samples**: Increase `pixels_per_second` to reduce detail
- **Real-time updates**: Playhead changes are cheap, avoid constant redraws
- **Memory**: Uses peak data structure internally, no audio processing

## Styling

Colors are configurable via `WaveformConfig`:

- **Peak color** - Shows peak amplitude (-6dB to 0dB range)
- **RMS color** - Shows average level (quieter, darker)
- **Grid color** - Comes from theme outline colors
- **Background** - Uses theme surface color

## Complete Example

```rust
use armas::components::audio::WaveformDisplay;
use egui::{Color32, Vec2};

fn show_waveform(ui: &mut egui::Ui, theme: &Theme) {
    let sample_data: Vec<f32> = vec![/* your sample data */];

    let config = WaveformConfig {
        pixels_per_second: 100.0,
        height: 200.0,
        peak_color: Color32::from_rgb(100, 200, 255),
        rms_color: Color32::from_rgb(100, 200, 255).gamma_multiply(0.5),
        show_grid: true,
        grid_interval: 1.0,
        color_by_frequency: false,
        show_spectrogram: false,
        zoom_level: 1.0,
    };

    WaveformDisplay::new(
        &sample_data,
        |sample| sample.abs(),
        2.0,  // duration in seconds
        44100, // sample rate
        theme
    )
    .config(config)
    .playhead(0.5)
    .loop_region(0.2, 1.8)
    .loop_enabled(true)
    .show(ui, Vec2::new(800.0, 200.0));
}
```
