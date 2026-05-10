# Icon Button

Icon-only buttons with multiple style variants.

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    IconButton::from_owned(web_icons::dark()).show(ui);
    IconButton::from_owned(web_icons::light()).show(ui);
});
```

## Variants

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    IconButton::from_owned(web_icons::dark()).variant(ButtonVariant::Default).show(ui);
    IconButton::from_owned(web_icons::dark()).variant(ButtonVariant::Secondary).show(ui);
    IconButton::from_owned(web_icons::dark()).variant(ButtonVariant::Outline).show(ui);
    IconButton::from_owned(web_icons::dark()).variant(ButtonVariant::Ghost).show(ui);
});
```

## Sizes

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    IconButton::from_owned(web_icons::dark()).size(16.0).padding(4.0).show(ui);
    IconButton::from_owned(web_icons::dark()).size(24.0).padding(8.0).show(ui);
    IconButton::from_owned(web_icons::dark()).size(32.0).padding(12.0).show(ui);
});
```

## Custom Colors

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    IconButton::from_owned(web_icons::light())
        .variant(ButtonVariant::Ghost)
        .icon_color(theme.primary())
        .hover_icon_color(theme.primary_foreground())
        .show(ui);
    IconButton::from_owned(web_icons::light())
        .variant(ButtonVariant::Ghost)
        .icon_color(theme.destructive())
        .hover_icon_color(theme.destructive_foreground())
        .show(ui);
});
```
