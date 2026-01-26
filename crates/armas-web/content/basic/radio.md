# Radio

Radio buttons for single selection from a group.

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
    .show(ui, is_selected, &theme);

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
        .show(ui, true, &theme);

    Radio::new()
        .size(RadioSize::Medium)
        .label("Medium")
        .show(ui, true, &theme);

    Radio::new()
        .size(RadioSize::Large)
        .label("Large")
        .show(ui, true, &theme);
});
```

