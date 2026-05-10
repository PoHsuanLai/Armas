# Slider

Horizontal slider for value selection.

```demo
let mut value = 50.0;
Slider::new(0.0, 100.0).id("slider_1").show(ui, &mut value);
```

## With Label

```demo
let mut value = 75.0;
Slider::new(0.0, 100.0).id("slider_2").label("Volume").show(ui, &mut value);
```

## With Suffix

```demo
let mut value = 50.0;
Slider::new(0.0, 100.0).id("slider_3").label("Opacity").suffix("%").show(ui, &mut value);
```

## With Step

```demo
let mut value = 5.0;
Slider::new(0.0, 10.0).id("slider_5").label("Rating").step(1.0).show(ui, &mut value);
```
