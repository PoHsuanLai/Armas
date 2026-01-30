# MIDI Controller

Complete MIDI controller interface combining piano keyboard, mod/pitch wheels, XY pad, drum pads, and step sequencer.

```demo
use armas_audio::{MidiController, MidiControllerState};

let mut state = MidiControllerState::default();
MidiController::new(&mut state).id("demo_basic").show(ui, &theme);
```

## Wheel Size

```demo
use armas_audio::{MidiController, MidiControllerState, WheelSize};

let mut state = MidiControllerState::default();
MidiController::new(&mut state).wheel_size(WheelSize::Large).id("demo_wheel_size").show(ui, &theme);
```
