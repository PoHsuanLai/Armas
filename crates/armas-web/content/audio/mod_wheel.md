# Mod Wheel

Rotating cylinder controller for modulation, pitch bend, and expression.

```demo
let mut value = 0.0;
ModWheel::new(&mut value)
    .wheel_type(WheelType::Modulation)
    .label("Mod".to_string())
    .show(ui, &theme);
```

## Wheel Types

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 12.0;
    let mut mod_value = 0.0;
    let mut pitch = 0.0;
    let mut expr = 0.5;

    ModWheel::new(&mut mod_value)
        .wheel_type(WheelType::Modulation)
        .label("Mod".to_string())
        .show_value(true)
        .show(ui, &theme);

    ModWheel::new(&mut pitch)
        .wheel_type(WheelType::PitchBend)
        .label("Pitch".to_string())
        .show_value(true)
        .show_center_line(true)
        .show(ui, &theme);

    ModWheel::new(&mut expr)
        .wheel_type(WheelType::Expression)
        .label("Expr".to_string())
        .show_value(true)
        .show(ui, &theme);
});
```

## Sizes

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    let mut small = 0.3;
    let mut default = 0.5;
    let mut large = 0.7;

    ModWheel::new(&mut small).size(WheelSize::Small).label("Small".to_string()).show(ui, &theme);
    ModWheel::new(&mut default).size(WheelSize::Default).label("Default".to_string()).show(ui, &theme);
    ModWheel::new(&mut large).size(WheelSize::Large).label("Large".to_string()).show(ui, &theme);
});
```
