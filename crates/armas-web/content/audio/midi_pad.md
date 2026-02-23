# MIDI Pad

Grid-based drum pad controller with velocity-sensitive visual feedback.

```demo
let pads = vec![
    PadConfig::new(36).label("Kick".to_string()),
    PadConfig::new(38).label("Snare".to_string()),
    PadConfig::new(42).label("HH".to_string()),
    PadConfig::new(46).label("Tom".to_string()),
];

MidiPad::new()
    .grid(2, 2)
    .pads(pads)
    .show(ui, &theme);
```

## 4x4 Grid

```demo
let pads = (0..16).map(|i| PadConfig::new(36 + i)).collect();

MidiPad::new()
    .grid(4, 4)
    .pads(pads)
    .show(ui, &theme);
```

## Variants

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 12.0;
    let pads = || vec![
        PadConfig::new(36).label("BD".to_string()),
        PadConfig::new(38).label("SD".to_string()),
        PadConfig::new(42).label("HH".to_string()),
        PadConfig::new(46).label("Tom".to_string()),
    ];

    MidiPad::new().grid(2, 2).pads(pads()).variant(PadVariant::Filled).show(ui, &theme);
    MidiPad::new().grid(2, 2).pads(pads()).variant(PadVariant::Outlined).show(ui, &theme);
    MidiPad::new().grid(2, 2).pads(pads()).variant(PadVariant::Elevated).show(ui, &theme);
});
```

## Color Schemes

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 12.0;
    let pads = || (0..6).map(|i| PadConfig::new(36 + i as u8)).collect();

    MidiPad::new().grid(2, 3).pads(pads()).color_scheme(PadColorScheme::Semantic).show(ui, &theme);
    MidiPad::new().grid(2, 3).pads(pads()).color_scheme(PadColorScheme::Monochrome).show(ui, &theme);
});
```

## Custom Colors

```demo
let pads = vec![
    PadConfig::new(36).label("BD".to_string()).color(egui::Color32::from_rgb(255, 100, 100)),
    PadConfig::new(38).label("SD".to_string()).color(egui::Color32::from_rgb(100, 255, 100)),
    PadConfig::new(42).label("HH".to_string()).color(egui::Color32::from_rgb(100, 100, 255)),
    PadConfig::new(46).label("Tom".to_string()).color(egui::Color32::from_rgb(255, 255, 100)),
];

MidiPad::new()
    .grid(2, 2)
    .pads(pads)
    .color_scheme(PadColorScheme::Custom)
    .show(ui, &theme);
```
