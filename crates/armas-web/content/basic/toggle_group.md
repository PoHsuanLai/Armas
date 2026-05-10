# Toggle Group

A group of pressable toggle buttons for single or multiple selection.

```demo
let mut selected = vec![true, false, false];
ToggleGroup::new(ToggleGroupType::Single).id("tg_1").variant(ToggleGroupVariant::Outline).show(ui, &["Bold", "Italic", "Underline"], &mut selected);
```

## Variants

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 8.0;
    let mut default = vec![true, false, false];
    ToggleGroup::new(ToggleGroupType::Single).id("tg_var_default").show(ui, &["All", "Missed", "Archived"], &mut default);
    let mut outline = vec![true, false, false];
    ToggleGroup::new(ToggleGroupType::Single).id("tg_var_outline").variant(ToggleGroupVariant::Outline).show(ui, &["All", "Missed", "Archived"], &mut outline);
});
```

## Sizes

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 8.0;
    let mut sm = vec![true, false, false, false];
    ToggleGroup::new(ToggleGroupType::Single).id("tg_sm").variant(ToggleGroupVariant::Outline).size(ToggleGroupSize::Sm).show(ui, &["Top", "Bottom", "Left", "Right"], &mut sm);
    let mut default = vec![true, false, false, false];
    ToggleGroup::new(ToggleGroupType::Single).id("tg_default").variant(ToggleGroupVariant::Outline).show(ui, &["Top", "Bottom", "Left", "Right"], &mut default);
    let mut lg = vec![true, false, false, false];
    ToggleGroup::new(ToggleGroupType::Single).id("tg_lg").variant(ToggleGroupVariant::Outline).size(ToggleGroupSize::Lg).show(ui, &["Top", "Bottom", "Left", "Right"], &mut lg);
});
```

## Multiple Selection

```demo
let mut selected = vec![true, true, false];
ToggleGroup::new(ToggleGroupType::Multiple).id("tg_multi").variant(ToggleGroupVariant::Outline).show(ui, &["Bold", "Italic", "Underline"], &mut selected);
```

## With Spacing

```demo
let mut selected = vec![true, false, false, false];
ToggleGroup::new(ToggleGroupType::Single).id("tg_spacing").variant(ToggleGroupVariant::Outline).size(ToggleGroupSize::Sm).spacing(4.0).show(ui, &["Top", "Bottom", "Left", "Right"], &mut selected);
```

## Vertical

```demo
let mut selected = vec![true, true, false];
ToggleGroup::new(ToggleGroupType::Multiple).id("tg_vert").spacing(4.0).vertical(true).show(ui, &["Bold", "Italic", "Underline"], &mut selected);
```

## Disabled

```demo
let mut selected = vec![true, false, false];
ToggleGroup::new(ToggleGroupType::Multiple).id("tg_disabled").variant(ToggleGroupVariant::Outline).disabled(true).show(ui, &["Bold", "Italic", "Underline"], &mut selected);
```
