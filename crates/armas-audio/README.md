# armas-audio

[![Crates.io](https://img.shields.io/crates/v/armas-audio.svg)](https://crates.io/crates/armas-audio)
[![Documentation](https://docs.rs/armas-audio/badge.svg)](https://docs.rs/armas-audio)
[![License](https://img.shields.io/crates/l/armas-audio.svg)](https://github.com/PoHsuanLai/Armas)

Audio-specific UI components for [egui](https://github.com/emilk/egui).

## Overview

UI components for building DAW (Digital Audio Workstation) interfaces, music production tools, and audio software. Includes MIDI controllers, mixer controls, timeline components, and sequencers. Built on the Armas component library.

## Installation

```toml
[dependencies]
armas-audio = "0.1.0"
armas = "0.1.0"
egui = "0.33"
```

## Examples

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

## Components

**MIDI Input**: Piano, MPEKeyboard, XYPad, ModWheel, MIDIPad
**Mix Controls**: Fader, Knob, AudioMeter, MixerStrip
**Timeline**: Timeline, TimelineTrack, TimelineMarker, TimelineRegion, PianoRoll, WaveformDisplay, Playhead, TimeRuler
**Sequencers**: DrumSequencer, StepSequencer
**Automation**: AutomationEditor, AutomationCanvas, PointHandle
**Utilities**: Transport, ZoomControl, SnapGrid, TrackHeader

## MPE Support

MPEKeyboard supports MIDI Polyphonic Expression with visual feedback for per-note velocity, pressure, pitch bend, and slide.

## Documentation

API documentation: [docs.rs/armas-audio](https://docs.rs/armas-audio)

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Links

- [Repository](https://github.com/PoHsuanLai/Armas)
- [Issue Tracker](https://github.com/PoHsuanLai/Armas/issues)
