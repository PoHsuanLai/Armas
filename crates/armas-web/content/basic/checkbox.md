# Checkbox

A checkbox control with animated checkmark.

```demo
let mut checked = false;
Checkbox::new().id("cb_1").show(ui, &mut checked);
```

## With Label

```demo
let mut checked = false;
Checkbox::new().id("cb_label").label("Accept terms and conditions").show(ui, &mut checked);
```

## With Description

```demo
let mut checked = true;
Checkbox::new().id("cb_desc").label("Marketing emails").description("Receive emails about new products and features").show(ui, &mut checked);
```

## Disabled

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 12.0;
    let mut off = false;
    Checkbox::new().id("cb_dis_off").disabled(true).show(ui, &mut off);
    let mut on = true;
    Checkbox::new().id("cb_dis_on").disabled(true).show(ui, &mut on);
});
```
