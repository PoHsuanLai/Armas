# Calendar

A standalone month calendar for date selection.

```demo
let mut selected = None;
let mut calendar = Calendar::new("cal_demo");
calendar.show(ui, &mut selected);
```

## With Footer

Adds Today and Clear action buttons below the grid.

```demo
let mut selected = None;
let mut calendar = Calendar::new("cal_footer").show_footer(true);
calendar.show(ui, &mut selected);
```

## Hide Outside Days

Hides leading/trailing days from adjacent months.

```demo
let mut selected = None;
let mut calendar = Calendar::new("cal_no_outside").show_outside_days(false);
calendar.show(ui, &mut selected);
```
