# Fader

Vertical fader control for DAW mixer channels.

```demo
use egui::Id;
let id = Id::new("fader_basic");
let mut value = ui.data_mut(|d| d.get_temp::<f32>(id).unwrap_or(0.5));
let (response, new_value) = Fader::new(value).show(ui);
value = new_value;
ui.data_mut(|d| d.insert_temp(id, value));
```

## With Housing

```demo
use egui::Id;
let id = Id::new("fader_strip");
let mut value = ui.data_mut(|d| d.get_temp::<f32>(id).unwrap_or(0.6));
let (response, new_value) = FaderStrip::new(value).show(ui);
value = new_value;
ui.data_mut(|d| d.insert_temp(id, value));
```

## Mixer Channels

```demo
use egui::Id;
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    for i in 0..4 {
        ui.vertical(|ui| {
            let id = Id::new(format!("mixer_{}", i));
            let mut value = ui.data_mut(|d| d.get_temp::<f32>(id).unwrap_or(0.5));
            let (_, new_val) = FaderStrip::new(value).show(ui);
            value = new_val;
            ui.data_mut(|d| d.insert_temp(id, value));
            ui.label(format!("Ch {}", i + 1));
        });
    }
});
```
