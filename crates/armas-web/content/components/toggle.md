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
    .with_label("Enable notifications")
    .show(ui, &mut checked);
```

## With Description

```demo
let mut checked = true;
Toggle::new()
    .id("toggle_8")
    .with_label("Auto-save")
    .with_description("Automatically save changes")
    .show(ui, &mut checked);
```

## Disabled State

```demo
let mut checked = false;
Toggle::new()
    .id("toggle_9")
    .with_label("Disabled toggle")
    .disabled(true)
    .show(ui, &mut checked);
```

```demo
let mut checked = true;
Toggle::new()
    .id("toggle_10")
    .with_label("Disabled (checked)")
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
    .with_label("Remember me")
    .show(ui, &mut checked);
```

### Medium Checkbox

```demo
let mut checked = false;
Toggle::new()
    .id("toggle_12")
    .variant(ToggleVariant::Checkbox)
    .size(ToggleSize::Medium)
    .with_label("I agree to the terms")
    .show(ui, &mut checked);
```

### Large Checkbox

```demo
let mut checked = true;
Toggle::new()
    .id("toggle_13")
    .variant(ToggleVariant::Checkbox)
    .size(ToggleSize::Large)
    .with_label("Enable feature")
    .show(ui, &mut checked);
```

## Handling Changes

```demo
let mut checked = false;
let response = Toggle::new()
    .id("toggle_14")
    .with_label("Setting")
    .show(ui, &mut checked);

if response.changed {
    // Handle state change
}
```

## Toggle Group

```demo
let mut group = ToggleGroup::new()
    .with_label("Settings")
    .add_toggle(
        "notifications",
        Toggle::new().with_label("Notifications"),
        true
    )
    .add_toggle(
        "sound",
        Toggle::new().with_label("Sound"),
        false
    )
    .add_toggle(
        "vibration",
        Toggle::new().with_label("Vibration"),
        true
    );

let response = group.show(ui);
for (id, state) in response.changed {
    // Handle individual toggle changes
}
```

## API Reference

### Toggle

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.id()` | `egui::Id` | `None` | ID for state persistence |
| `.variant()` | `ToggleVariant` | `Switch` | Toggle or checkbox |
| `.size()` | `ToggleSize` | `Medium` | Size preset |
| `.with_label()` | `&str` | `None` | Label text |
| `.with_description()` | `&str` | `None` | Description text |
| `.disabled()` | `bool` | `false` | Disabled state |

### ToggleGroup

| Method | Type | Description |
|--------|------|-------------|
| `.with_label()` | `&str` | Group label |
| `.add_toggle()` | `(id, Toggle, bool)` | Add toggle with ID and default state |

### ToggleResponse

| Field | Type | Description |
|-------|------|-------------|
| `response` | `Response` | Underlying egui response |
| `changed` | `bool` | Whether state changed |

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface`, `on_surface`
- SpringAnimation for smooth physics-based animations
