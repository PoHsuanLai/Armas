# MPE Keyboard

Interactive piano keyboard with MPE (MIDI Polyphonic Expression) support and per-note visualization.

```demo
let theme = ui.ctx().armas_theme();
MPEKeyboard::new().show(ui, &theme);
```

## With Active Notes

```demo
let theme = ui.ctx().armas_theme();
let mut notes = std::collections::HashMap::new();
notes.insert(60, MPENote::new(60).pressure(0.7));
notes.insert(64, MPENote::with_velocity(64, 0.9).pitch_bend(2.0));
notes.insert(67, MPENote::new(67).slide(0.8));
MPEKeyboard::new().active_notes(notes).show(ui, &theme);
```

## Expression Visualization

JUCE-style circles encode MPE dimensions: inner circle = velocity, outer circle = pressure, X position = pitch bend, Y position = slide.

```demo
let theme = ui.ctx().armas_theme();
let mut notes = std::collections::HashMap::new();
notes.insert(60, MPENote::with_velocity(60, 0.9).pressure(0.8));
notes.insert(64, MPENote::with_velocity(64, 0.3).pressure(0.1));
notes.insert(67, MPENote::with_velocity(67, 0.5).pressure(0.9));
MPEKeyboard::new().active_notes(notes).show(ui, &theme);
```

## Pitch Bend

```demo
let theme = ui.ctx().armas_theme();
let mut notes = std::collections::HashMap::new();
notes.insert(60, MPENote::new(60).pitch_bend(6.0));
notes.insert(64, MPENote::new(64).pitch_bend(-6.0));
notes.insert(67, MPENote::new(67));
MPEKeyboard::new().active_notes(notes).show(ui, &theme);
```

## Slide

```demo
let theme = ui.ctx().armas_theme();
let mut notes = std::collections::HashMap::new();
notes.insert(60, MPENote::new(60).slide(0.0));
notes.insert(62, MPENote::new(62).slide(0.25));
notes.insert(64, MPENote::new(64).slide(0.5));
notes.insert(65, MPENote::new(65).slide(0.75));
notes.insert(67, MPENote::new(67).slide(1.0));
MPEKeyboard::new().active_notes(notes).show(ui, &theme);
```

## Combined Expression

```demo
let theme = ui.ctx().armas_theme();
let mut notes = std::collections::HashMap::new();
notes.insert(60, MPENote::with_velocity(60, 0.9).pressure(0.6).pitch_bend(-0.5).slide(0.3));
notes.insert(64, MPENote::with_velocity(64, 0.7).pressure(0.8).pitch_bend(1.0).slide(0.7));
notes.insert(67, MPENote::with_velocity(67, 0.85).pressure(0.4).pitch_bend(0.0).slide(0.5));
MPEKeyboard::new().octaves(2).active_notes(notes).show(ui, &theme);
```

## Orientations

```demo
let theme = ui.ctx().armas_theme();
ui.vertical(|ui| {
    ui.label("Horizontal");
    MPEKeyboard::new().orientation(MPEOrientation::Horizontal).show(ui, &theme);
    ui.add_space(12.0);
    ui.label("Horizontal Up");
    MPEKeyboard::new().orientation(MPEOrientation::HorizontalUp).show(ui, &theme);
    ui.add_space(12.0);
    ui.label("Vertical");
    MPEKeyboard::new().orientation(MPEOrientation::Vertical).show(ui, &theme);
});
```

## Custom Range

```demo
let theme = ui.ctx().armas_theme();
MPEKeyboard::new().start_note(48).octaves(3).show(ui, &theme);
```

## Custom Colors

```demo
let theme = ui.ctx().armas_theme();
let mut notes = std::collections::HashMap::new();
notes.insert(60, MPENote::new(60).pressure(0.5));
notes.insert(64, MPENote::new(64).pressure(0.7));
MPEKeyboard::new().active_notes(notes).circle_fill_color(egui::Color32::from_rgb(255, 100, 50)).circle_outline_color(egui::Color32::from_rgb(255, 200, 100)).show(ui, &theme);
```

## Glass Styling

```demo
let theme = ui.ctx().armas_theme();
let mut notes = std::collections::HashMap::new();
notes.insert(60, MPENote::new(60).pressure(0.5));
notes.insert(64, MPENote::new(64).pressure(0.7));
MPEKeyboard::new().white_key_opacity(0.8).black_key_opacity(0.9).active_notes(notes).show(ui, &theme);
```

## Interactive Example

```demo
let theme = ui.ctx().armas_theme();
let response = MPEKeyboard::new().octaves(3).start_note(48).show(ui, &theme);
if !response.clicked_keys.is_empty() {
    ui.label(format!("Note on: {:?}", response.clicked_keys));
}
```
