# XY Pad

2D touch controller for simultaneous parameter control.

```demo
let mut x = 0.5;
let mut y = 0.5;
let response = XYPad::new(&mut x, &mut y).size(200.0).show(ui, &theme);
```

## With Labels

```demo
let mut x = 0.5;
let mut y = 0.5;
XYPad::new(&mut x, &mut y).size(200.0).x_label("Cutoff".to_string()).y_label("Resonance".to_string()).show(ui, &theme);
```

## Variants

```demo
let mut x1 = 0.5;
let mut y1 = 0.5;
let mut x2 = 0.5;
let mut y2 = 0.5;
let mut x3 = 0.5;
let mut y3 = 0.5;
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 12.0;
    XYPad::new(&mut x1, &mut y1).size(150.0).variant(XYPadVariant::Filled).show(ui, &theme);
    XYPad::new(&mut x2, &mut y2).size(150.0).variant(XYPadVariant::Outlined).show(ui, &theme);
    XYPad::new(&mut x3, &mut y3).size(150.0).variant(XYPadVariant::Elevated).show(ui, &theme);
});
```
