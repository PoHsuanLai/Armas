# Icon Button

Icon-only buttons with Material Design variants.

```demo
use armas::components::button::{IconButton, ButtonVariant};
use armas_audio::TransportIcon;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    IconButton::new(TransportIcon::Play.data()).show(ui);
    IconButton::new(TransportIcon::Pause.data()).show(ui);
    IconButton::new(TransportIcon::Stop.data()).show(ui);
});
```

## Variants

```demo
use armas::components::button::{IconButton, ButtonVariant};
use armas_audio::TransportIcon;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    IconButton::new(TransportIcon::Play.data()).variant(ButtonVariant::Filled).show(ui);
    IconButton::new(TransportIcon::Play.data()).variant(ButtonVariant::FilledTonal).show(ui);
    IconButton::new(TransportIcon::Play.data()).variant(ButtonVariant::Elevated).show(ui);
    IconButton::new(TransportIcon::Play.data()).variant(ButtonVariant::Outlined).show(ui);
    IconButton::new(TransportIcon::Play.data()).variant(ButtonVariant::Text).show(ui);
});
```

## Sizes

```demo
use armas::components::button::{IconButton, ButtonVariant};
use armas_audio::TransportIcon;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    IconButton::new(TransportIcon::Play.data()).size(16.0).padding(4.0).show(ui);
    IconButton::new(TransportIcon::Play.data()).size(24.0).padding(8.0).show(ui);
    IconButton::new(TransportIcon::Play.data()).size(32.0).padding(12.0).show(ui);
});
```

## Transport Controls

```demo
use armas::components::button::{IconButton, ButtonVariant};
use armas_audio::TransportIcon;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    IconButton::new(TransportIcon::Play.data()).variant(ButtonVariant::Filled).show(ui);
    IconButton::new(TransportIcon::Pause.data()).variant(ButtonVariant::Filled).show(ui);
    IconButton::new(TransportIcon::Stop.data()).variant(ButtonVariant::Filled).show(ui);
    IconButton::new(TransportIcon::Record.data()).variant(ButtonVariant::Filled).show(ui);
    IconButton::new(TransportIcon::Rewind.data()).variant(ButtonVariant::Filled).show(ui);
    IconButton::new(TransportIcon::Forward.data()).variant(ButtonVariant::Filled).show(ui);
});
```
