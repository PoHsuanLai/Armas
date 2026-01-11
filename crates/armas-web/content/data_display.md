# Data Display

## Badge

```demo
Badge::new("New").show(ui);
ui.add_space(8.0);
Badge::new("5").show(ui);
ui.add_space(8.0);
Badge::new("Pro").show(ui);
```

## Avatar

```demo
Avatar::new("JD").show(ui);
ui.add_space(8.0);
Avatar::new("AB").show(ui);
ui.add_space(8.0);
Avatar::new("XY").show(ui);
```

## Tooltip

```demo
let response = ui.button("Hover me");
Tooltip::new("This is a tooltip").show_if_hovered(ui, response);
```
