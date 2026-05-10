# Switch

Animated toggle switch with spring physics.

```demo
let mut checked = false;
Switch::new().id("switch_1").show(ui, &mut checked);
```

## Sizes

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    let mut small = true;
    Switch::new().id("sw_small").size(SwitchSize::Small).show(ui, &mut small);
    let mut medium = true;
    Switch::new().id("sw_medium").show(ui, &mut medium);
    let mut large = true;
    Switch::new().id("sw_large").size(SwitchSize::Large).show(ui, &mut large);
});
```

## With Label

```demo
let mut checked = false;
Switch::new().id("sw_labeled").label("Enable notifications").show(ui, &mut checked);
```

## With Description

```demo
let mut checked = true;
Switch::new().id("sw_described").label("Auto-save").description("Automatically save changes").show(ui, &mut checked);
```

## Disabled

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    let mut off = false;
    Switch::new().id("sw_disabled_off").disabled(true).show(ui, &mut off);
    let mut on = true;
    Switch::new().id("sw_disabled_on").disabled(true).show(ui, &mut on);
});
```
