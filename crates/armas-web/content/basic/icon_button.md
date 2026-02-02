# Icon Button

Icon-only buttons with Material Design variants.

```demo
use armas::components::button::{IconButton, ButtonVariant};
use armas_audio::icons;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    IconButton::from_owned(icons::play()).show(ui, &theme);
    IconButton::from_owned(icons::pause()).show(ui, &theme);
    IconButton::from_owned(icons::stop()).show(ui, &theme);
});
```

## Variants

```demo
use armas::components::button::{IconButton, ButtonVariant};
use armas_audio::icons;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    IconButton::from_owned(icons::play()).variant(ButtonVariant::Filled).show(ui, &theme);
    IconButton::from_owned(icons::play()).variant(ButtonVariant::FilledTonal).show(ui, &theme);
    IconButton::from_owned(icons::play()).variant(ButtonVariant::Elevated).show(ui, &theme);
    IconButton::from_owned(icons::play()).variant(ButtonVariant::Outlined).show(ui, &theme);
    IconButton::from_owned(icons::play()).variant(ButtonVariant::Text).show(ui, &theme);
});
```

## Sizes

```demo
use armas::components::button::{IconButton, ButtonVariant};
use armas_audio::icons;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    IconButton::from_owned(icons::play()).size(16.0).padding(4.0).show(ui, &theme);
    IconButton::from_owned(icons::play()).size(24.0).padding(8.0).show(ui, &theme);
    IconButton::from_owned(icons::play()).size(32.0).padding(12.0).show(ui, &theme);
});
```

## Transport Controls

```demo
use armas::components::button::{IconButton, ButtonVariant};
use armas_audio::icons;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    IconButton::from_owned(icons::play()).variant(ButtonVariant::Filled).show(ui, &theme);
    IconButton::from_owned(icons::pause()).variant(ButtonVariant::Filled).show(ui, &theme);
    IconButton::from_owned(icons::stop()).variant(ButtonVariant::Filled).show(ui, &theme);
    IconButton::from_owned(icons::record()).variant(ButtonVariant::Filled).show(ui, &theme);
    IconButton::from_owned(icons::rewind()).variant(ButtonVariant::Filled).show(ui, &theme);
    IconButton::from_owned(icons::forward()).variant(ButtonVariant::Filled).show(ui, &theme);
});
```
