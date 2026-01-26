# Feature Grid

Responsive grid layout for displaying features with icons and descriptions.

```demo
FeatureGrid::new().show(ui, |grid| {
    grid.feature("*", "Fast", "Lightning speed performance");
    grid.feature("*", "Secure", "Enterprise-grade security");
    grid.feature("*", "Scalable", "Grows with your needs");
});
```

## Custom Columns

```demo
FeatureGrid::new().columns(2).show(ui, |grid| {
    grid.feature("*", "Easy", "Simple to use");
    grid.feature("*", "Powerful", "Advanced features");
    grid.feature("*", "Fast", "Optimized performance");
});
```

## Custom Colors

```demo
use egui::Color32;
FeatureGrid::new().show(ui, |grid| {
    grid.feature_with_color("*", "Innovation", "Creative solutions", Color32::from_rgb(255, 200, 0));
    grid.feature_with_color("*", "Design", "Beautiful interfaces", Color32::from_rgb(200, 0, 255));
    grid.feature_with_color("*", "Engineering", "Rock-solid code", Color32::from_rgb(0, 200, 255));
});
```

## Custom Styling

```demo
FeatureGrid::new().columns(2).gap(30.0).show_borders(false).hover_effect(true).icon_size(48.0).show(ui, |grid| {
    grid.feature("*", "Hot Features", "The latest and greatest");
    grid.feature("*", "Cool Features", "Stay chill with stability");
});
```
