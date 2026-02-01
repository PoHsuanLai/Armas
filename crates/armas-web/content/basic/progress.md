# Progress

Progress indicator for showing task completion.

```demo
Progress::new(65.0).show(ui, &theme);
```

## Multiple Values

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 8.0;
    Progress::new(25.0).show(ui, &theme);
    Progress::new(50.0).show(ui, &theme);
    Progress::new(75.0).show(ui, &theme);
    Progress::new(100.0).show(ui, &theme);
});
```

## Circular

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 16.0;
    CircularProgressBar::new(25.0).show(ui, &theme);
    CircularProgressBar::new(50.0).show(ui, &theme);
    CircularProgressBar::new(75.0).show_percentage(true).show(ui, &theme);
});
```

## Indeterminate

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 16.0;
    CircularProgressBar::indeterminate().size(32.0).show(ui, &theme);
    CircularProgressBar::indeterminate().size(48.0).show(ui, &theme);
    CircularProgressBar::indeterminate().size(64.0).stroke_width(6.0).show(ui, &theme);
});
```

