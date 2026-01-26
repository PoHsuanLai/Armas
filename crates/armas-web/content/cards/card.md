# Card

Material Design 3 card component for displaying grouped content.

```demo
let theme = ui.ctx().armas_theme();
Card::new().title("Card Title").show(ui, &theme, |ui| {
    ui.label("Card content goes here");
});
```

## Variants

```demo
let theme = ui.ctx().armas_theme();
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 16.0;
    Card::new().title("Filled").width(180.0).show(ui, &theme, |ui| {
        ui.label("Subtle separation");
    });
    Card::new().variant(CardVariant::Outlined).title("Outlined").width(180.0).show(ui, &theme, |ui| {
        ui.label("Clear boundary");
    });
    Card::new().variant(CardVariant::Elevated).title("Elevated").width(180.0).show(ui, &theme, |ui| {
        ui.label("Visual separation");
    });
});
```

## Clickable

```demo
let theme = ui.ctx().armas_theme();
let response = Card::new().variant(CardVariant::Outlined).title("Click Me").clickable(true).width(300.0).show(ui, &theme, |ui| {
    ui.label("This card is clickable!");
    ui.label("Try hovering and clicking");
});
if response.clicked() {
    ui.label("Card was clicked!");
}
```

## Custom Styling

```demo
let theme = ui.ctx().armas_theme();
Card::new().title("Custom Card").fill(egui::Color32::from_rgb(60, 40, 80)).stroke(theme.primary()).corner_radius(16.0).inner_margin(24.0).width(300.0).show(ui, &theme, |ui| {
    ui.label("Custom background color");
    ui.label("Custom border color");
    ui.label("Custom corner radius");
});
```

## With Layout Components

```demo
let theme = ui.ctx().armas_theme();
Card::new().variant(CardVariant::Outlined).title("Profile").width(300.0).show(ui, &theme, |ui| {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 12.0;
        Avatar::new("JD").size_preset(AvatarSize::Large).show(ui);
        ui.vertical(|ui| {
            ui.label(egui::RichText::new("John Doe").strong());
            ui.label("Software Engineer");
        });
    });
});
```
