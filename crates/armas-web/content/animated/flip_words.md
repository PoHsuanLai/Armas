# Flip Words

Animated text that cycles through a list of words with smooth flip transitions.

```demo
let words = vec!["Fast", "Powerful", "Beautiful"];
let mut flip = FlipWords::new(words);
flip.show(ui, &theme);
```

## Custom Duration

```demo
let words = vec!["Quick", "Smooth", "Elegant"];
let mut flip = FlipWords::new(words).duration(3.0);
flip.show(ui, &theme);
```

## Styles

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 16.0;
    ui.label("Vertical");
    let mut flip = FlipWords::new(vec!["One", "Two", "Three"]).style(FlipStyle::Vertical);
    flip.show(ui, &theme);
    ui.label("Horizontal");
    let mut flip = FlipWords::new(vec!["Left", "Right"]).style(FlipStyle::Horizontal);
    flip.show(ui, &theme);
    ui.label("Fade");
    let mut flip = FlipWords::new(vec!["Fade", "In", "Out"]).style(FlipStyle::Fade);
    flip.show(ui, &theme);
});
```

## Custom Styling

```demo
use egui::Color32;
let words = vec!["Large", "Bold", "Colorful"];
let mut flip = FlipWords::new(words).font_size(48.0).color(Color32::WHITE).highlight(Color32::from_rgb(100, 200, 255));
flip.show(ui, &theme);
```

## Complete Example

```demo
use egui::Color32;
let words = vec!["Lightning Fast", "Beautifully Designed", "Highly Customizable", "Production Ready"];
let mut flip = FlipWords::new(words).duration(2.5).style(FlipStyle::Vertical).font_size(32.0).color(Color32::from_gray(220)).highlight(theme.primary());
flip.show(ui, &theme);
```
