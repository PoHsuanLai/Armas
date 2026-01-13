# Transport Control

Professional DAW transport bar with playback controls, time display, tempo, and loop/metronome controls.

## Basic Usage

**Important**: Store the `TransportControl` instance in your app state to preserve playback state, time, and settings across frames.

For proper state persistence, store the Transport in your app struct:

```rust
struct MyApp {
    transport: TransportControl,
}

impl MyApp {
    fn new() -> Self {
        Self {
            transport: TransportControl::new()
                .tempo(120.0)
                .time_signature(4, 4),
        }
    }
}

// In your UI code:
let (response, transport) = self.transport.show(ui, theme);
self.transport = transport;
if response.play_clicked {
    // Start playback
}
if response.stop_clicked {
    // Stop playback
}
```

## Live Demo

```demo
// Note: In the demo, state persists via egui's memory system.
// In a real app, store TransportControl in your app struct.
let transport_id = ui.id().with("transport_demo");
let mut transport: TransportControl = ui.ctx().data_mut(|d| {
    d.get_persisted(transport_id).unwrap_or_else(|| {
        TransportControl::new()
            .tempo(120.0)
            .time_signature(4, 4)
    })
});

let (response, new_transport) = transport.show(ui, &theme);

// Save state
ui.ctx().data_mut(|d| {
    d.insert_persisted(transport_id, new_transport);
});

// Display response info
if response.play_clicked {
    ui.label("▶ Play clicked");
}
if response.pause_clicked {
    ui.label("⏸ Pause clicked");
}
if response.stop_clicked {
    ui.label("⏹ Stop clicked");
}
```

## With Custom Settings

```demo
let transport_id = ui.id().with("transport_custom");
let mut transport: TransportControl = ui.ctx().data_mut(|d| {
    d.get_persisted(transport_id).unwrap_or_else(|| {
        TransportControl::new()
            .tempo(140.0)
            .time_signature(3, 4)
            .loop_enabled(true)
            .metronome_enabled(true)
    })
});

let (response, new_transport) = transport.show(ui, &theme);

ui.ctx().data_mut(|d| {
    d.insert_persisted(transport_id, new_transport);
});
```

## API Reference

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create new transport control |
| `.state(state)` | `TransportState` | `Stopped` | Set playback state |
| `.current_time(time)` | `f64` | `0.0` | Set current time in seconds |
| `.tempo(bpm)` | `f32` | `120.0` | Set tempo in BPM |
| `.time_signature(num, denom)` | `(u8, u8)` | `(4, 4)` | Set time signature |
| `.loop_enabled(enabled)` | `bool` | `false` | Set loop enabled state |
| `.metronome_enabled(enabled)` | `bool` | `false` | Set metronome enabled state |
| `.width(width)` | `f32` | Fill available | Set transport width |
| `.show(&mut Ui, &Theme)` | - | - | Show transport, returns `(TransportResponse, Self)` |

### Getter Methods

| Method | Return Type | Description |
|--------|-------------|-------------|
| `.get_state()` | `TransportState` | Get current playback state |
| `.get_current_time()` | `f64` | Get current time in seconds |
| `.get_tempo()` | `f32` | Get tempo in BPM |
| `.get_time_signature()` | `(u8, u8)` | Get time signature (numerator, denominator) |
| `.is_loop_enabled()` | `bool` | Check if loop is enabled |
| `.is_metronome_enabled()` | `bool` | Check if metronome is enabled |

### TransportState

```rust
pub enum TransportState {
    Stopped,   // Transport is stopped
    Playing,   // Transport is playing
    Paused,    // Transport is paused
    Recording, // Transport is recording
}
```

### TransportResponse

The response from `show()` contains:

- `response: Response` - Standard egui response
- `play_clicked: bool` - Play button clicked (state changed to Playing)
- `pause_clicked: bool` - Pause button clicked while playing
- `stop_clicked: bool` - Stop button clicked (resets time to 0)
- `record_clicked: bool` - Record button clicked (toggles recording)
- `rewind_clicked: bool` - Rewind button clicked (sets time to 0)
- `forward_clicked: bool` - Fast forward button clicked
- `loop_toggled: bool` - Loop button toggled
- `metronome_toggled: bool` - Metronome button toggled
- `tempo_changed: bool` - Tempo value was edited
- `new_tempo: Option<f32>` - New tempo value (if changed this frame)

## Features

### Navigation Controls (Left Section)

- **⏮ Rewind**: Jump to start (sets time to 0.0)
- **▶/⏸ Play/Pause**: Toggle between play and pause states
- **⏹ Stop**: Stop playback and reset time to 0
- **⏩ Fast Forward**: Custom forward behavior (click detected)

The play button dynamically shows ▶ when stopped/paused and ⏸ when playing/recording.

### Time Display

Shows current playback position in **MM:SS.mmm** format:
- Minutes (00-99)
- Seconds (00-59)
- Milliseconds (000-999)
- Monospace font for alignment
- Fixed width container (100px)

### Tempo Control

Editable BPM (Beats Per Minute):
- Displays current tempo with 1 decimal place
- Inline text field for direct editing
- Valid range: 0.0 - 999.0 BPM
- Changes trigger `tempo_changed` flag

### Time Signature Display

Shows current time signature (e.g., "4/4", "3/4", "6/8"):
- Read-only display
- Fixed width container (50px)
- Set via builder method

### Toggle Controls (Right Section)

- **🔄 Loop**: Enable/disable loop playback
- **🎵 Metronome**: Enable/disable metronome click
- **🔴 Record**: Toggle recording mode

Toggle buttons show as:
- **Filled** variant when active/enabled
- **Text/Outlined** variant when inactive

## Usage Patterns

### Basic Playback Control

```rust
let (response, new_transport) = transport.show(ui, &theme);
transport = new_transport;

if response.play_clicked {
    audio_engine.start();
}
if response.pause_clicked {
    audio_engine.pause();
}
if response.stop_clicked {
    audio_engine.stop();
    audio_engine.seek_to(0.0);
}
```

### Sync Transport Time

```rust
// Update transport with playback position
transport = transport.current_time(audio_engine.get_position());

let (response, new_transport) = transport.show(ui, &theme);
transport = new_transport;

// Handle rewind/navigation
if response.rewind_clicked {
    audio_engine.seek_to(0.0);
}
```

### Tempo and Metronome

```rust
let (response, new_transport) = transport.show(ui, &theme);

if response.tempo_changed {
    if let Some(new_bpm) = response.new_tempo {
        audio_engine.set_tempo(new_bpm);
    }
}

if response.metronome_toggled {
    audio_engine.set_metronome(new_transport.is_metronome_enabled());
}

transport = new_transport;
```

### Recording

```rust
let (response, new_transport) = transport.show(ui, &theme);

if response.record_clicked {
    match new_transport.get_state() {
        TransportState::Recording => {
            audio_engine.start_recording();
        }
        _ => {
            audio_engine.stop_recording();
        }
    }
}

transport = new_transport;
```

## Visual Design

The transport follows professional DAW aesthetics:

- **Grouped controls**: Related buttons grouped with `ui.group()`
- **Material Design buttons**: Using Armas button variants
- **Consistent sizing**: 32x32px for most buttons, 40x32px for play
- **Visual hierarchy**: Filled primary action (play), text secondary actions
- **Spacing**: Theme-consistent spacing between groups
- **Monospace time**: Clear, aligned time display
- **Active state indication**: Filled buttons show active toggles

## Layout Structure

```
[ ⏮ ▶ ⏹ ⏩ ] | [ 00:00.000 ] | [ BPM: 120.0 ] | [ 4/4 ] | [ 🔄 🎵 🔴 ]
   Navigation      Time          Tempo          Sig      Toggles
```

All sections are horizontally arranged with consistent spacing.

## State Management

The Transport Control maintains internal state:
- Current playback state (Stopped/Playing/Paused/Recording)
- Current time position (f64 seconds)
- Tempo (f32 BPM)
- Time signature (numerator, denominator)
- Loop enabled flag
- Metronome enabled flag

State changes are communicated through the `TransportResponse`, allowing your audio engine to react to user interactions.

## Integration with Audio Engine

Typical integration pattern:

```rust
struct DAWApp {
    transport: TransportControl,
    audio_engine: AudioEngine,
}

impl DAWApp {
    fn update(&mut self, ui: &mut Ui, theme: &Theme) {
        // Sync transport with engine state
        let transport = self.transport.clone()
            .current_time(self.audio_engine.get_position())
            .state(self.audio_engine.get_state());

        let (response, new_transport) = transport.show(ui, &theme);
        self.transport = new_transport;

        // Handle all transport events
        if response.play_clicked {
            self.audio_engine.play();
        }
        if response.stop_clicked {
            self.audio_engine.stop();
        }
        if response.tempo_changed {
            if let Some(bpm) = response.new_tempo {
                self.audio_engine.set_tempo(bpm);
            }
        }
        // ... handle other events
    }
}
```

## Dependencies

- `egui = "0.33"`
- Internal components: Button with variants (Filled, Text, Outlined)
- Theme system for consistent styling
