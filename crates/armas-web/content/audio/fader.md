# Fader

Vertical fader component inspired by DAW mixer faders.

## Basic Usage (Minimal Fader)

```demo
use egui::Id;

let id = Id::new("fader_basic");
let mut value = ui.data_mut(|d| d.get_temp::<f32>(id).unwrap_or(0.5));

let (response, new_value) = Fader::new(value).show(ui);
value = new_value;
ui.data_mut(|d| d.insert_temp(id, value));

if response.changed() {
    // Fader value changed
}
```

## Custom Size

```demo
use egui::Id;

let id = Id::new("fader_custom_size");
let mut value = ui.data_mut(|d| d.get_temp::<f32>(id).unwrap_or(0.75));

let (response, new_value) = Fader::new(value)
    .size(40.0, 300.0)
    .show(ui);

value = new_value;
ui.data_mut(|d| d.insert_temp(id, value));
```

## Complete Fader Strip (With Housing)

```demo
use egui::Id;

let id = Id::new("fader_strip");
let mut value = ui.data_mut(|d| d.get_temp::<f32>(id).unwrap_or(0.6));

// FaderStrip includes the grey gradient housing box
let (response, new_value) = FaderStrip::new(value).show(ui);
value = new_value;
ui.data_mut(|d| d.insert_temp(id, value));
```

## Custom Fader Strip Size

```demo
use egui::Id;

let id = Id::new("fader_strip_custom");
let mut value = ui.data_mut(|d| d.get_temp::<f32>(id).unwrap_or(0.8));

let (response, new_value) = FaderStrip::new(value)
    .size(50.0, 320.0)
    .show(ui);

value = new_value;
ui.data_mut(|d| d.insert_temp(id, value));
```

## Multiple Faders (Mixer)

```demo
use egui::Id;

ui.horizontal(|ui| {
    for i in 0..4 {
        ui.vertical(|ui| {
            let id = Id::new(format!("mixer_fader_{}", i));
            let mut value = ui.data_mut(|d| d.get_temp::<f32>(id).unwrap_or(0.5));

            let (_, new_val) = FaderStrip::new(value).show(ui);
            value = new_val;
            ui.data_mut(|d| d.insert_temp(id, value));

            ui.label(format!("Ch {}", i + 1));
        });
    }
});
```

## API Reference

### Fader (Minimal)

The core interactive slider without housing - use this when placing the fader in your own container.

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(value)` | `f32` | - | Create fader with value (0.0-1.0) |
| `.size(width, height)` | `(f32, f32)` | `(30.0, 240.0)` | Set custom size |
| `.show(&mut Ui)` | - | - | Show fader, returns (Response, f32) |

### FaderStrip (Complete)

The complete fader with grey gradient housing box - batteries included, ready to use.

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(value)` | `f32` | - | Create fader strip with value (0.0-1.0) |
| `.size(width, height)` | `(f32, f32)` | `(39.0, 254.0)` | Set custom size |
| `.show(&mut Ui)` | - | - | Show fader strip, returns (Response, f32) |

## Design Details

The fader is designed to look like a professional audio mixer fader:
- **Track**: Black inset plate with tick marks
- **Channel**: Dark vertical slot where the thumb moves
- **Thumb**: Detailed 3D-looking slider cap with ridges and finger groove
- **Housing** (FaderStrip only): Grey gradient outer box

The value ranges from 0.0 (bottom) to 1.0 (top), following traditional fader conventions.

## Dependencies

- `egui = "0.33"`
- Theme colors: Uses hardcoded realistic fader colors for authenticity
