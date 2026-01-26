# Drum Sequencer

Multi-row drum pattern programmer with step sequencing and velocity control.

```demo
let mut rows = vec![
    DrumRow::new("Kick", 16).with_color(egui::Color32::from_rgb(255, 100, 80)),
    DrumRow::new("Snare", 16).with_color(egui::Color32::from_rgb(100, 200, 255)),
    DrumRow::new("HiHat", 16).with_color(egui::Color32::from_rgb(255, 200, 100)),
    DrumRow::new("Tom", 16).with_color(egui::Color32::from_rgb(200, 100, 255)),
];
let response = DrumSequencer::new(&mut rows).steps(16).id("main_drum_sequencer").show(ui, &theme);
if response.changed {
    ui.label("Pattern changed!");
}
```

## Variants

```demo
let mut rows1 = vec![
    DrumRow::new("Kick", 16).with_color(egui::Color32::from_rgb(255, 100, 80)),
    DrumRow::new("Snare", 16).with_color(egui::Color32::from_rgb(100, 200, 255)),
];
let mut rows2 = vec![
    DrumRow::new("Kick", 16).with_color(egui::Color32::from_rgb(255, 100, 80)),
    DrumRow::new("Snare", 16).with_color(egui::Color32::from_rgb(100, 200, 255)),
];
let mut rows3 = vec![
    DrumRow::new("Kick", 16).with_color(egui::Color32::from_rgb(255, 100, 80)),
    DrumRow::new("Snare", 16).with_color(egui::Color32::from_rgb(100, 200, 255)),
];
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 16.0;
    ui.label("Filled");
    DrumSequencer::new(&mut rows1).variant(DrumSequencerVariant::Filled).id("drum_filled").show(ui, &theme);
    ui.label("Outlined");
    DrumSequencer::new(&mut rows2).variant(DrumSequencerVariant::Outlined).id("drum_outlined").show(ui, &theme);
    ui.label("Elevated");
    DrumSequencer::new(&mut rows3).variant(DrumSequencerVariant::Elevated).id("drum_elevated").show(ui, &theme);
});
```

## Color Schemes

```demo
let mut rows_semantic = vec![
    DrumRow::new("Kick", 16).with_color(egui::Color32::from_rgb(255, 100, 80)),
    DrumRow::new("Snare", 16).with_color(egui::Color32::from_rgb(100, 200, 255)),
    DrumRow::new("HiHat", 16).with_color(egui::Color32::from_rgb(255, 200, 100)),
];
let mut rows_mono = vec![
    DrumRow::new("Kick", 16).with_color(egui::Color32::from_rgb(255, 100, 80)),
    DrumRow::new("Snare", 16).with_color(egui::Color32::from_rgb(100, 200, 255)),
    DrumRow::new("HiHat", 16).with_color(egui::Color32::from_rgb(255, 200, 100)),
];
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 16.0;
    ui.label("Semantic");
    DrumSequencer::new(&mut rows_semantic).color_scheme(DrumSequencerColorScheme::Semantic).id("drum_semantic").show(ui, &theme);
    ui.label("Monochrome");
    DrumSequencer::new(&mut rows_mono).color_scheme(DrumSequencerColorScheme::Monochrome).id("drum_mono").show(ui, &theme);
});
```

## With Velocity

```demo
let mut rows = vec![DrumRow::new("Kick", 16)];
rows[0].steps[0].active = true;
rows[0].steps[0].velocity = 1.0;
rows[0].steps[4].active = true;
rows[0].steps[4].velocity = 0.7;
rows[0].steps[8].active = true;
rows[0].steps[8].velocity = 0.4;
DrumSequencer::new(&mut rows).show_velocity(true).glow_intensity(1.0).id("drum_velocity").show(ui, &theme);
```

## Row Controls

```demo
let mut rows = vec![
    DrumRow::new("Kick", 16),
    DrumRow::new("Snare", 16),
    DrumRow::new("HiHat", 16),
];
rows[0].muted = true;
rows[1].soloed = true;
rows[2].visible = false;
DrumSequencer::new(&mut rows).steps(16).id("drum_controls").show(ui, &theme);
```
