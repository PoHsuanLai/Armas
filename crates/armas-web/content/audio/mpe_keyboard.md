# MPE Keyboard

Interactive piano keyboard with MPE (MIDI Polyphonic Expression) support and per-note visualization.

```demo
MPEKeyboard::new().show(ui, &theme);
```

## Expression

Inner circle = velocity, outer circle = pressure, X = pitch bend, Y = slide.

```demo
let mut notes = std::collections::HashMap::new();
notes.insert(60, MPENote::with_velocity(60, 0.9).pressure(0.6).pitch_bend(-0.5).slide(0.3));
notes.insert(64, MPENote::with_velocity(64, 0.7).pressure(0.8).pitch_bend(1.0).slide(0.7));
notes.insert(67, MPENote::with_velocity(67, 0.85).pressure(0.4).pitch_bend(0.0).slide(0.5));
MPEKeyboard::new().octaves(2).active_notes(notes).show(ui, &theme);
```

## Orientations

```demo
ui.vertical(|ui| {
    ui.label("Horizontal");
    MPEKeyboard::new().orientation(MPEOrientation::Horizontal).show(ui, &theme);
    ui.add_space(12.0);
    ui.label("Vertical");
    MPEKeyboard::new().orientation(MPEOrientation::Vertical).show(ui, &theme);
});
```

## Custom Range

```demo
MPEKeyboard::new().start_note(48).octaves(3).show(ui, &theme);
```

## Custom Colors

```demo
let mut notes = std::collections::HashMap::new();
notes.insert(60, MPENote::new(60).pressure(0.5));
notes.insert(64, MPENote::new(64).pressure(0.7));
MPEKeyboard::new().active_notes(notes).circle_fill_color(egui::Color32::from_rgb(255, 100, 50)).circle_outline_color(egui::Color32::from_rgb(255, 200, 100)).show(ui, &theme);
```
