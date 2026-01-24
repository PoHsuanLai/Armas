# Section Header

Collapsible section header with directional arrow indicator.

## Interactive Example

```demo
use egui::Id;

let id = Id::new("section_header_demo");
let mut collapsed = ui.data_mut(|d| d.get_temp::<bool>(id).unwrap_or(false));

if SectionHeader::new("Settings", collapsed)
    .show(ui)
    .clicked()
{
    collapsed = !collapsed;
    ui.data_mut(|d| d.insert_temp(id, collapsed));
}

if !collapsed {
    ui.indent("settings_content", |ui| {
        ui.label("Setting 1: Enabled");
        ui.label("Setting 2: Disabled");
        ui.label("Setting 3: Auto");
    });
}
```

## Expanded State

```demo
SectionHeader::new("Audio Effects", false)
    .show(ui);
ui.indent("audio_effects", |ui| {
    ui.label("• Reverb");
    ui.label("• Delay");
    ui.label("• Compressor");
});
```

## Collapsed State

```demo
SectionHeader::new("Advanced Options", true)
    .show(ui);
// Content hidden when collapsed
```

## Multiple Sections

```demo
use egui::Id;

for (i, section) in ["General", "Audio", "MIDI", "Performance"].iter().enumerate() {
    let id = Id::new(format!("section_{}", i));
    let mut collapsed = ui.data_mut(|d| d.get_temp::<bool>(id).unwrap_or(true));

    if SectionHeader::new(section, collapsed)
        .show(ui)
        .clicked()
    {
        collapsed = !collapsed;
        ui.data_mut(|d| d.insert_temp(id, collapsed));
    }

    if !collapsed {
        ui.indent(format!("content_{}", i), |ui| {
            ui.label(format!("{} settings here...", section));
        });
    }
}
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(label, collapsed)` | `(&str, bool)` | - | Create section header |
| `.show(&mut Ui)` | - | - | Show the header (returns clickable Response) |

## Note

The arrow indicator automatically changes based on the collapsed state:
- **▼** when expanded (section visible)
- **▶** when collapsed (section hidden)

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`
