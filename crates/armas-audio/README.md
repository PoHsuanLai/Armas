# armas-audio

[![Crates.io](https://img.shields.io/crates/v/armas-audio.svg)](https://crates.io/crates/armas-audio)
[![Documentation](https://docs.rs/armas-audio/badge.svg)](https://docs.rs/armas-audio)
[![License](https://img.shields.io/crates/l/armas-audio.svg)](https://github.com/PoHsuanLai/Armas)

Audio UI components for [egui](https://github.com/emilk/egui) - specialized widgets for audio production interfaces.

## Overview

`armas-audio` provides a comprehensive set of UI components specifically designed for DAW (Digital Audio Workstation) applications, music production tools, and audio software. Built on top of the Armas component library with glassmorphic styling and professional audio workflows in mind.

## Features

- **🎹 MIDI Controllers** - Piano keyboards, MPE keyboards, drum pads, XY pads
- **🎚️ Mix Controls** - Faders, knobs, meters, mixer strips
- **📊 Timeline Components** - Piano roll, automation editor, waveform display
- **🎵 Sequencers** - Drum sequencer, step sequencer
- **🎛️ Professional Styling** - Glassmorphic design, smooth animations
- **🎯 MPE Support** - Full MIDI Polyphonic Expression visualization

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
armas-audio = "0.1.0"
armas = "0.1.0"
egui = "0.33"
```

### Examples

**Fader:**
```rust
use armas_audio::Fader;

let mut level = 0.8;
Fader::new(&mut level)
    .height(200.0)
    .show(ui, &theme);
```

**Piano Keyboard:**
```rust
use armas_audio::Piano;
use std::collections::HashSet;

let pressed_keys = HashSet::from([60, 64, 67]); // C major chord
let response = Piano::new()
    .octaves(2)
    .pressed_keys(pressed_keys)
    .show(ui, &theme);

for note in response.clicked_keys {
    println!("Note pressed: {}", note);
}
```

**Knob:**
```rust
use armas_audio::Knob;

let mut value = 0.5;
let response = Knob::new()
    .label("Cutoff")
    .range(0.0..=1.0)
    .show(ui, &mut value, &theme);

if response.changed {
    println!("Knob value: {}", value);
}
```

**Audio Meter:**
```rust
use armas_audio::AudioMeter;

let peak_level = 0.8;
let rms_level = 0.6;

AudioMeter::new(peak_level)
    .rms(rms_level)
    .height(150.0)
    .show(ui, &theme);
```

**MPE Keyboard with Expression:**
```rust
use armas_audio::{MPEKeyboard, MPENote};
use std::collections::HashMap;

let mut notes = HashMap::new();
notes.insert(60, MPENote::new(60)
    .velocity(0.8)
    .pressure(0.5)
    .pitch_bend(0.2));

let response = MPEKeyboard::new()
    .active_notes(notes)
    .show(ui, &theme);
```

**Transport Controls:**
```rust
use armas_audio::Transport;

let mut transport = Transport::new();
let response = transport.show(ui, &theme);

if response.play_clicked {
    println!("Play pressed");
}
if response.stop_clicked {
    println!("Stop pressed");
}
```

## Available Components

### MIDI Input
- **Piano** - Glassmorphic piano keyboard with multiple octaves
- **MPEKeyboard** - MIDI Polyphonic Expression keyboard with per-note visualization
- **XYPad** - 2D touch controller for parameter control
- **ModWheel** - Modulation wheel controller
- **MIDIPad** - Drum pad / MIDI trigger pad

### Mix Controls
- **Fader** - Vertical/horizontal fader with precise control
- **Knob** - Rotary knob with velocity-based dragging
- **AudioMeter** - Peak and RMS level meter with clip detection
- **MixerStrip** - Complete channel strip (fader, pan, meters, controls)

### Timeline & Sequencing
- **Timeline** - Multi-track timeline with drag & drop
- **TimelineTrack** - Individual track in a timeline
- **TimelineMarker** - Cue points, tempo, time signature markers
- **TimelineRegion** - Regions/clips on the timeline
- **PianoRoll** - MIDI note editor
- **PianoRollGrid** - Grid for piano roll
- **WaveformDisplay** - Audio waveform visualization with markers
- **Playhead** - Playback position indicator
- **TimeRuler** - Time/measure ruler

### Sequencers
- **DrumSequencer** - Grid-based drum pattern sequencer
- **StepSequencer** - Step sequencer for melodic patterns

### Automation
- **AutomationEditor** - Automation curve editor with multiple curve types
- **AutomationCanvas** - Canvas for drawing automation
- **PointHandle** - Draggable automation point with velocity mode

### Utilities
- **Transport** - Play/stop/record/loop transport controls
- **ZoomControl** - Horizontal and vertical zoom controls
- **SnapGrid** - Grid snapping configuration
- **TrackHeader** - Track name and controls

## Styling

All components follow a consistent design language:
- **Glassmorphic** - Semi-transparent elements with blur
- **Smooth Animations** - Easing and momentum scrolling
- **Professional Look** - Inspired by modern DAWs
- **Theme Integration** - Uses Armas theme system

## MPE Support

Full support for MIDI Polyphonic Expression with visual feedback:
- Per-note velocity (inner circle size)
- Per-note pressure (outer circle size)
- Per-note pitch bend (circle X position)
- Per-note slide (circle Y position)

## Documentation

- [Full API Documentation](https://docs.rs/armas-audio)
- [Main Armas Library](https://crates.io/crates/armas)

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Links

- [Repository](https://github.com/PoHsuanLai/Armas)
- [Issue Tracker](https://github.com/PoHsuanLai/Armas/issues)
