# Number Field

Numeric input with increment and decrement stepper buttons.

```demo
let mut value = 18.0;
NumberField::new().id("nf_1").show(ui, &mut value);
```

## With Min/Max

```demo
let mut value = 5.0;
NumberField::new().id("nf_minmax").min(0.0).max(10.0).show(ui, &mut value);
```

## With Step

```demo
let mut value = 50.0;
NumberField::new().id("nf_step").min(0.0).max(100.0).step(5.0).show(ui, &mut value);
```

## Decimal Step

```demo
let mut value = 0.5;
NumberField::new().id("nf_decimal").min(0.0).max(1.0).step(0.1).show(ui, &mut value);
```

## With Label

```demo
let mut value = 3.0;
NumberField::new().id("nf_label").min(1.0).max(10.0).label("Quantity").show(ui, &mut value);
```

## With Description

```demo
let mut value = 18.0;
NumberField::new().id("nf_desc").min(0.0).max(120.0).label("Age").description("Must be 18 or older").show(ui, &mut value);
```

## Custom Width

```demo
let mut value = 42.0;
NumberField::new().id("nf_width").width(120.0).show(ui, &mut value);
```

## Disabled

```demo
let mut value = 10.0;
NumberField::new().id("nf_disabled").disabled(true).show(ui, &mut value);
```
