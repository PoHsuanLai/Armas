# Toggle

A pressable button with on/off state.

```demo
let mut pressed = false;
Toggle::new("Bold").id("tgl_1").show(ui, &mut pressed);
```

## Variants

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    let mut default = false;
    Toggle::new("Bold").id("tgl_default").show(ui, &mut default);
    let mut outline = false;
    Toggle::new("Italic").id("tgl_outline").variant(ToggleVariant::Outline).show(ui, &mut outline);
});
```

## Sizes

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    let mut sm = false;
    Toggle::new("Sm").id("tgl_sm").size(ToggleSize::Sm).show(ui, &mut sm);
    let mut def = true;
    Toggle::new("Default").id("tgl_def").show(ui, &mut def);
    let mut lg = false;
    Toggle::new("Lg").id("tgl_lg").size(ToggleSize::Lg).show(ui, &mut lg);
});
```

## With Outline

```demo
let mut pressed = true;
Toggle::new("Bold").id("tgl_outline_on").variant(ToggleVariant::Outline).show(ui, &mut pressed);
```

## Disabled

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    let mut off = false;
    Toggle::new("Off").id("tgl_dis_off").disabled(true).show(ui, &mut off);
    let mut on = true;
    Toggle::new("On").id("tgl_dis_on").disabled(true).show(ui, &mut on);
});
```
