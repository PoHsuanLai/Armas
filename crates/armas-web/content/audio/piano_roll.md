# Piano Roll

DAW-style piano roll editor with vertical keyboard, grid, and interactive note blocks.

```demo
let state_id = ui.id().with("piano_roll_basic");
let mut notes: Vec<Note> = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(|| vec![
        Note::new(60, 0.0, 1.0),
        Note::new(64, 1.0, 1.0),
        Note::new(67, 2.0, 1.0),
        Note::new(72, 3.0, 1.0),
    ])
});
let response = PianoRoll::new()
    .id(state_id)
    .notes(notes.clone())
    .show(ui, &theme);
notes = response.notes;
ui.ctx().data_mut(|d| d.insert_temp(state_id, notes));
```

## Grid Divisions

```demo
let state_id = ui.id().with("piano_roll_divisions");
let mut notes: Vec<Note> = ui.ctx().data_mut(|d| d.get_temp(state_id).unwrap_or_default());
let response = PianoRoll::new()
    .id(state_id)
    .division(GridDivision::Sixteenth)
    .default_note_duration(0.25)
    .notes(notes.clone())
    .show(ui, &theme);
notes = response.notes;
ui.ctx().data_mut(|d| d.insert_temp(state_id, notes));
```

## With Velocity

```demo
let state_id = ui.id().with("piano_roll_velocity");
let mut notes: Vec<Note> = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(|| vec![
        Note::with_velocity(60, 0.0, 1.0, 38),
        Note::with_velocity(64, 1.0, 1.0, 76),
        Note::with_velocity(67, 2.0, 1.0, 127),
    ])
});
let response = PianoRoll::new()
    .id(state_id)
    .notes(notes.clone())
    .show(ui, &theme);
notes = response.notes;
ui.ctx().data_mut(|d| d.insert_temp(state_id, notes));
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
