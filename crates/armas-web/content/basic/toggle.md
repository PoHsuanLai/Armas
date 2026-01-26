# Toggle

Animated toggle switches and checkboxes with spring physics.

```demo
let mut checked = false;
Toggle::new().id("toggle_1").show(ui, &mut checked, &theme);
```

## Variants

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    let mut switch = true;
    Toggle::new().id("switch").show(ui, &mut switch, &theme);
    let mut checkbox = true;
    Toggle::new().id("checkbox").variant(ToggleVariant::Checkbox).show(ui, &mut checkbox, &theme);
});
```

## Sizes

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    let mut small = true;
    Toggle::new().id("small").size(ToggleSize::Small).show(ui, &mut small, &theme);
    let mut medium = true;
    Toggle::new().id("medium").show(ui, &mut medium, &theme);
    let mut large = true;
    Toggle::new().id("large").size(ToggleSize::Large).show(ui, &mut large, &theme);
});
```

## With Label

```demo
let mut checked = false;
Toggle::new().id("labeled").label("Enable notifications").show(ui, &mut checked, &theme);
```

## With Description

```demo
let mut checked = true;
Toggle::new().id("described").label("Auto-save").description("Automatically save changes").show(ui, &mut checked, &theme);
```

## Disabled

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    let mut off = false;
    Toggle::new().id("disabled_off").disabled(true).show(ui, &mut off, &theme);
    let mut on = true;
    Toggle::new().id("disabled_on").disabled(true).show(ui, &mut on, &theme);
});
```
