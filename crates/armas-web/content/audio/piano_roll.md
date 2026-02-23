# Piano Roll

DAW-style piano roll editor with vertical keyboard, grid, and interactive note blocks.

```demo
let mut notes = vec![
    Note::new(60, 0.0, 1.0),
    Note::new(64, 1.0, 1.0),
    Note::new(67, 2.0, 1.0),
    Note::new(72, 3.0, 1.0),
];
let response = PianoRoll::new()
    .notes(notes.clone())
    .show(ui, &theme);
notes = response.notes;
```

## Grid Divisions

```demo
let mut notes = vec![];
let response = PianoRoll::new()
    .division(GridDivision::Sixteenth)
    .default_note_duration(0.25)
    .notes(notes.clone())
    .show(ui, &theme);
notes = response.notes;
```

## With Velocity

```demo
let mut notes = vec![
    Note::with_velocity(60, 0.0, 1.0, 38),
    Note::with_velocity(64, 1.0, 1.0, 76),
    Note::with_velocity(67, 2.0, 1.0, 127),
];
let response = PianoRoll::new()
    .notes(notes.clone())
    .show(ui, &theme);
notes = response.notes;
```

## Read-Only

```demo
let notes = vec![
    Note::new(60, 0.0, 1.0),
    Note::new(64, 1.0, 1.0),
    Note::new(67, 2.0, 1.0),
];
PianoRoll::new()
    .notes(notes)
    .editable(false)
    .show(ui, &theme);
```
