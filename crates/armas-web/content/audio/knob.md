# Knob

Rotary knob control with glazed ceramic appearance.

```demo
let theme = ui.ctx().armas_theme();
let mut value = 0.5;
Knob::new(value).show(ui, &mut value, &theme);
```

## With Label

```demo
let theme = ui.ctx().armas_theme();
let mut gain = 0.75;
Knob::new(gain).label("Gain").show(ui, &mut gain, &theme);
```

## Sizes

```demo
let theme = ui.ctx().armas_theme();
let mut vol = 0.6;
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 12.0;
    Knob::new(vol).diameter(40.0).label("Small").show(ui, &mut vol, &theme);
    Knob::new(vol).diameter(60.0).label("Medium").show(ui, &mut vol, &theme);
    Knob::new(vol).diameter(80.0).label("Large").show(ui, &mut vol, &theme);
});
```

## Custom Colors

```demo
let theme = ui.ctx().armas_theme();
let mut bass = 0.4;
let mut mid = 0.5;
let mut treble = 0.6;
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 12.0;
    Knob::new(bass).label("Bass").glow_color(egui::Color32::from_rgb(255, 100, 100)).show(ui, &mut bass, &theme);
    Knob::new(mid).label("Mid").glow_color(egui::Color32::from_rgb(100, 255, 100)).show(ui, &mut mid, &theme);
    Knob::new(treble).label("Treble").glow_color(egui::Color32::from_rgb(100, 200, 255)).show(ui, &mut treble, &theme);
});
```
