# Toggle

Animated toggle switches and checkboxes with spring-based physics animations.

## Basic Usage

```demo
let mut checked = false;
Toggle::new()
    .id("toggle_1")
    .show(ui, &mut checked);
```

## Variants

### Switch

```demo
let mut checked = false;
Toggle::new()
    .id("toggle_2")
    .variant(ToggleVariant::Switch)
    .show(ui, &mut checked);
```

### Checkbox

```demo
let mut checked = false;
Toggle::new()
    .id("toggle_3")
    .variant(ToggleVariant::Checkbox)
    .show(ui, &mut checked);
```

## Sizes

### Small

```demo
let mut checked = true;
Toggle::new()
    .id("toggle_4")
    .size(ToggleSize::Small)
    .show(ui, &mut checked);
```

### Medium

```demo
let mut checked = true;
Toggle::new()
    .id("toggle_5")
    .size(ToggleSize::Medium)
    .show(ui, &mut checked);
```

### Large

```demo
let mut checked = true;
Toggle::new()
    .id("toggle_6")
    .size(ToggleSize::Large)
    .show(ui, &mut checked);
```

## With Label

```demo
let mut checked = false;
Toggle::new()
    .id("toggle_7")
    .label("Enable notifications")
    .show(ui, &mut checked);
```

## With Description

```demo
let mut checked = true;
Toggle::new()
    .id("toggle_8")
    .label("Auto-save")
    .description("Automatically save changes")
    .show(ui, &mut checked);
```

## Disabled State

```demo
let mut checked = false;
Toggle::new()
    .id("toggle_9")
    .label("Disabled toggle")
    .disabled(true)
    .show(ui, &mut checked);
```

```demo
let mut checked = true;
Toggle::new()
    .id("toggle_10")
    .label("Disabled (checked)")
    .disabled(true)
    .show(ui, &mut checked);
```

## Checkbox Variant

### Small Checkbox

```demo
let mut checked = true;
Toggle::new()
    .id("toggle_11")
    .variant(ToggleVariant::Checkbox)
    .size(ToggleSize::Small)
    .label("Remember me")
    .show(ui, &mut checked);
```

### Medium Checkbox

```demo
let mut checked = false;
Toggle::new()
    .id("toggle_12")
    .variant(ToggleVariant::Checkbox)
    .size(ToggleSize::Medium)
    .label("I agree to the terms")
    .show(ui, &mut checked);
```

### Large Checkbox

```demo
let mut checked = true;
Toggle::new()
    .id("toggle_13")
    .variant(ToggleVariant::Checkbox)
    .size(ToggleSize::Large)
    .label("Enable feature")
    .show(ui, &mut checked);
```

## Handling Changes

```demo
let mut checked = false;
let response = Toggle::new()
    .id("toggle_14")
    .label("Setting")
    .show(ui, &mut checked);

if response.changed {
    // Handle state change
}
```

## Toggle Group

```demo
use armas::components::ToggleGroupState;

let state_id = ui.id().with("toggle_group");
let mut state = ui.data_mut(|d| {
    d.get_temp::<ToggleGroupState>(state_id).unwrap_or_else(|| {
        let mut s = ToggleGroupState::default();
        s.set_checked("notifications", true);
        s.set_checked("sound", false);
        s.set_checked("vibration", true);
        s
    })
});

let response = ToggleGroup::new(&mut state)
    .label("Settings")
    .show(ui, |group| {
        group.toggle("notifications", "Notifications");
        group.toggle("sound", "Sound");
        group.toggle("vibration", "Vibration");
    });

for (id, state) in response.changed {
    // Handle individual toggle changes
}

ui.data_mut(|d| d.insert_temp(state_id, state));
```

## API Reference

### Toggle

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.id()` | `egui::Id` | `None` | ID for state persistence |
| `.variant()` | `ToggleVariant` | `Switch` | Toggle or checkbox |
| `.size()` | `ToggleSize` | `Medium` | Size preset |
| `.label()` | `&str` | `None` | Label text |
| `.description()` | `&str` | `None` | Description text |
| `.disabled()` | `bool` | `false` | Disabled state |

### ToggleGroup

| Method | Type | Description |
|--------|------|-------------|
| `::new()` | `&mut ToggleGroupState` | Create group with state |
| `.label()` | `&str` | Group label |
| `.show()` | closure | Render with closure-based API |

### ToggleGroupBuilder (in closure)

| Method | Type | Description |
|--------|------|-------------|
| `.toggle()` | `(&str, &str)` | Add toggle with ID and label |

### ToggleResponse

| Field | Type | Description |
|-------|------|-------------|
| `response` | `Response` | Underlying egui response |
| `changed` | `bool` | Whether state changed |

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface`, `on_surface`
- SpringAnimation for smooth physics-based animations
