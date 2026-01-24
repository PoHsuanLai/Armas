# Radio

Material Design 3 radio buttons for single selection from a group of options.

## Basic Usage

```demo
let theme = ui.ctx().armas_theme();
let state_id = ui.id().with("radio_basic");
let mut selected = ui.ctx().data_mut(|d| {
    d.get_temp::<Option<String>>(state_id).unwrap_or(Some("option1".to_string()))
});

RadioGroup::new(&mut selected)
    .show(ui, |group| {
        group.option("option1", "First Option");
        group.option("option2", "Second Option");
        group.option("option3", "Third Option");
    });

ui.ctx().data_mut(|d| d.insert_temp(state_id, selected));
```

## With Label

```demo
let theme = ui.ctx().armas_theme();
let state_id = ui.id().with("radio_label");
let mut selected = ui.ctx().data_mut(|d| {
    d.get_temp::<Option<String>>(state_id).unwrap_or(Some("small".to_string()))
});

RadioGroup::new(&mut selected)
    .label("Choose Size")
    .show(ui, |group| {
        group.option("small", "Small");
        group.option("medium", "Medium");
        group.option("large", "Large");
    });

ui.ctx().data_mut(|d| d.insert_temp(state_id, selected));
```

## With Descriptions

```demo
let theme = ui.ctx().armas_theme();
let state_id = ui.id().with("radio_desc");
let mut selected = ui.ctx().data_mut(|d| {
    d.get_temp::<Option<String>>(state_id).unwrap_or(Some("standard".to_string()))
});

RadioGroup::new(&mut selected)
    .label("Select Plan")
    .show(ui, |group| {
        group.option("standard", "Standard")
            .description("Basic features for individuals");
        group.option("pro", "Professional")
            .description("Advanced features for teams");
        group.option("enterprise", "Enterprise")
            .description("Full features with premium support");
    });

ui.ctx().data_mut(|d| d.insert_temp(state_id, selected));
```

## Disabled State

```demo
let theme = ui.ctx().armas_theme();
let state_id = ui.id().with("radio_disabled");
let mut selected = ui.ctx().data_mut(|d| {
    d.get_temp::<Option<String>>(state_id).unwrap_or(Some("option1".to_string()))
});

RadioGroup::new(&mut selected)
    .label("Disabled Group")
    .disabled(true)
    .show(ui, |group| {
        group.option("option1", "Option 1");
        group.option("option2", "Option 2");
        group.option("option3", "Option 3");
    });

ui.ctx().data_mut(|d| d.insert_temp(state_id, selected));
```

## Individual Radio Button

```demo
let theme = ui.ctx().armas_theme();
let state_id = ui.id().with("radio_individual");
let mut is_selected = ui.ctx().data_mut(|d| {
    d.get_temp::<bool>(state_id).unwrap_or(true)
});

let response = Radio::new()
    .label("Standalone Radio")
    .show(ui, is_selected);

if response.response.clicked() {
    is_selected = !is_selected;
}

ui.ctx().data_mut(|d| d.insert_temp(state_id, is_selected));
```

## Handling Selection Changes

```demo
let theme = ui.ctx().armas_theme();
let state_id = ui.id().with("radio_changes");
let mut selected = ui.ctx().data_mut(|d| {
    d.get_temp::<Option<String>>(state_id).unwrap_or(Some("option2".to_string()))
});

let response = RadioGroup::new(&mut selected)
    .label("Track Changes")
    .show(ui, |group| {
        group.option("option1", "Option 1");
        group.option("option2", "Option 2");
        group.option("option3", "Option 3");
    });

if response.changed {
    ui.label(format!("Selected: {:?}", response.selected));
}

ui.ctx().data_mut(|d| d.insert_temp(state_id, selected));
```

## Sizes

```demo
let theme = ui.ctx().armas_theme();

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 24.0;

    Radio::new()
        .size(RadioSize::Small)
        .label("Small")
        .show(ui, true);

    Radio::new()
        .size(RadioSize::Medium)
        .label("Medium")
        .show(ui, true);

    Radio::new()
        .size(RadioSize::Large)
        .label("Large")
        .show(ui, true);
});
```

## API Reference

### RadioGroup

#### Constructor

```rust
RadioGroup::new(selected_value: &mut Option<String>) -> Self
```

#### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.label()` | `String` | `None` | Sets the group label |
| `.disabled()` | `bool` | `false` | Disables all radio buttons in the group |

#### Show Method

```rust
pub fn show<R>(
    self,
    ui: &mut egui::Ui,
    content: impl FnOnce(&mut RadioGroupBuilder) -> R,
) -> RadioGroupResponse
```

### Radio (Individual)

#### Constructor

```rust
Radio::new() -> Self
```

#### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.size()` | `RadioSize` | `Medium` | Sets the radio size |
| `.label()` | `String` | `None` | Sets the label text |
| `.description()` | `String` | `None` | Sets the description text |
| `.disabled()` | `bool` | `false` | Disables interaction |

#### Show Method

```rust
pub fn show(self, ui: &mut egui::Ui, selected: bool) -> RadioResponse
```

## Composition Examples

### Radio Group with Layout

```demo
let theme = ui.ctx().armas_theme();
let state_id = ui.id().with("radio_layout");
let mut selected = ui.ctx().data_mut(|d| {
    d.get_temp::<Option<String>>(state_id).unwrap_or(Some("left".to_string()))
});

Card::new()
    .variant(CardVariant::Outlined)
    .title("Alignment Settings")
    .show(ui, &theme, |ui| {
        RadioGroup::new(&mut selected)
            .label("Text Alignment")
            .show(ui, |group| {
                group.option("left", "Left Aligned");
                group.option("center", "Center Aligned");
                group.option("right", "Right Aligned");
            });
    });

ui.ctx().data_mut(|d| d.insert_temp(state_id, selected));
```

### Multiple Radio Groups

```demo
let theme = ui.ctx().armas_theme();
let color_id = ui.id().with("radio_color");
let size_id = ui.id().with("radio_size");

let mut color = ui.ctx().data_mut(|d| {
    d.get_temp::<Option<String>>(color_id).unwrap_or(Some("blue".to_string()))
});
let mut size = ui.ctx().data_mut(|d| {
    d.get_temp::<Option<String>>(size_id).unwrap_or(Some("medium".to_string()))
});

ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = theme.spacing.lg;

    RadioGroup::new(&mut color)
        .label("Color")
        .show(ui, |group| {
            group.option("red", "Red");
            group.option("blue", "Blue");
            group.option("green", "Green");
        });

    RadioGroup::new(&mut size)
        .label("Size")
        .show(ui, |group| {
            group.option("small", "Small");
            group.option("medium", "Medium");
            group.option("large", "Large");
        });
});

ui.ctx().data_mut(|d| {
    d.insert_temp(color_id, color);
    d.insert_temp(size_id, size);
});
```

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `outline`, `outline_variant`, `on_surface`
- Minimum version: `armas 0.2.0`
