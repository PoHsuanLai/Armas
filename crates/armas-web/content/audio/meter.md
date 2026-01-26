# Audio Meter

Audio level meter with peak hold and gradients.

```demo
use std::f32::consts::PI;
let time = ui.input(|i| i.time) as f32;
let level = ((time * 2.0).sin() * 0.5 + 0.5) * 0.8;
AudioMeter::new(level).height(200.0).show(ui);
```

## Styles

```demo
use std::f32::consts::PI;
let theme = ui.ctx().armas_theme();
let time = ui.input(|i| i.time) as f32;
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 12.0;
    let level1 = ((time * 1.5).sin() * 0.5 + 0.5) * 0.9;
    AudioMeter::new(level1).height(180.0).vu_colors(&theme).show(ui);
    let level2 = ((time * 2.5).sin() * 0.5 + 0.5) * 0.85;
    AudioMeter::new(level2).height(180.0).style(MeterStyle::Segmented(20)).vu_colors(&theme).show(ui);
    let level3 = ((time * 2.2).sin() * 0.5 + 0.5) * 0.7;
    AudioMeter::new(level3).height(180.0).monochrome(theme.primary()).show(ui);
});
```

## With Scale

```demo
use std::f32::consts::PI;
let time = ui.input(|i| i.time) as f32;
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 12.0;
    let level1 = ((time * 1.8).sin() * 0.5 + 0.5) * 0.75;
    AudioMeter::new(level1).height(200.0).width(35.0).show_scale().show(ui);
    let level2 = ((time * 2.1).sin() * 0.5 + 0.5) * 0.8;
    AudioMeter::new(level2).height(200.0).width(35.0).scale_left().show(ui);
});
```

## Mixer Strip

```demo
use std::f32::consts::PI;
let theme = ui.ctx().armas_theme();
let time = ui.input(|i| i.time) as f32;
ui.horizontal(|ui| {
    for i in 0..4 {
        ui.vertical(|ui| {
            let phase = i as f32 * 0.5;
            let level = ((time * 2.0 + phase).sin() * 0.5 + 0.5) * 0.85;
            AudioMeter::new(level).height(180.0).width(25.0).vu_colors(&theme).show(ui);
            ui.label(format!("Ch {}", i + 1));
        });
        ui.add_space(4.0);
    }
});
```
