# Icon Button

Material Design 3 icon buttons with various styling variants.

## Features

- **Material Design 3 Variants**: Filled, Filled Tonal, Elevated, Outlined, and Text
- **Interactive States**: Hover and pressed states with visual feedback
- **Customizable**: Adjustable icon size and padding
- **Theme Integration**: Automatically uses theme colors

## Variants

### Text (Default)

Subtle button with minimal styling, shows background on hover.

```demo
use armas::components::button::{IconButton, ButtonVariant};
use armas_audio::TransportIcon;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    if IconButton::new(TransportIcon::Play.data())
        .variant(ButtonVariant::Text)
        .show(ui)
        .clicked()
    {
        ui.label("Play clicked");
    }

    if IconButton::new(TransportIcon::Pause.data())
        .variant(ButtonVariant::Text)
        .show(ui)
        .clicked()
    {
        ui.label("Pause clicked");
    }

    if IconButton::new(TransportIcon::Stop.data())
        .variant(ButtonVariant::Text)
        .show(ui)
        .clicked()
    {
        ui.label("Stop clicked");
    }
});
```

### Filled

Prominent button with primary color background.

```demo
use armas::components::button::{IconButton, ButtonVariant};
use armas_audio::TransportIcon;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    IconButton::new(TransportIcon::Play.data())
        .variant(ButtonVariant::Filled)
        .show(ui);

    IconButton::new(TransportIcon::Pause.data())
        .variant(ButtonVariant::Filled)
        .show(ui);

    IconButton::new(TransportIcon::Stop.data())
        .variant(ButtonVariant::Filled)
        .show(ui);
});
```

### Filled Tonal

Softer than filled, uses secondary container color.

```demo
use armas::components::button::{IconButton, ButtonVariant};
use armas_audio::TransportIcon;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    IconButton::new(TransportIcon::Play.data())
        .variant(ButtonVariant::FilledTonal)
        .show(ui);

    IconButton::new(TransportIcon::Pause.data())
        .variant(ButtonVariant::FilledTonal)
        .show(ui);

    IconButton::new(TransportIcon::Stop.data())
        .variant(ButtonVariant::FilledTonal)
        .show(ui);
});
```

### Elevated

Subtle background with shadow for elevation.

```demo
use armas::components::button::{IconButton, ButtonVariant};
use armas_audio::TransportIcon;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    IconButton::new(TransportIcon::Play.data())
        .variant(ButtonVariant::Elevated)
        .show(ui);

    IconButton::new(TransportIcon::Pause.data())
        .variant(ButtonVariant::Elevated)
        .show(ui);

    IconButton::new(TransportIcon::Stop.data())
        .variant(ButtonVariant::Elevated)
        .show(ui);
});
```

### Outlined

Border with no background (except on hover).

```demo
use armas::components::button::{IconButton, ButtonVariant};
use armas_audio::TransportIcon;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    IconButton::new(TransportIcon::Play.data())
        .variant(ButtonVariant::Outlined)
        .show(ui);

    IconButton::new(TransportIcon::Pause.data())
        .variant(ButtonVariant::Outlined)
        .show(ui);

    IconButton::new(TransportIcon::Stop.data())
        .variant(ButtonVariant::Outlined)
        .show(ui);
});
```

## Sizes

Customize icon size and padding.

```demo
use armas::components::button::{IconButton, ButtonVariant};
use armas_audio::TransportIcon;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 4.0;
        IconButton::new(TransportIcon::Play.data())
            .variant(ButtonVariant::Filled)
            .size(16.0)
            .padding(4.0)
            .show(ui);
        ui.label("Small");
    });

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 4.0;
        IconButton::new(TransportIcon::Play.data())
            .variant(ButtonVariant::Filled)
            .size(24.0)
            .padding(8.0)
            .show(ui);
        ui.label("Medium");
    });

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 4.0;
        IconButton::new(TransportIcon::Play.data())
            .variant(ButtonVariant::Filled)
            .size(32.0)
            .padding(12.0)
            .show(ui);
        ui.label("Large");
    });
});
```

## All Transport Icons

```demo
use armas::components::button::{IconButton, ButtonVariant};
use armas_audio::TransportIcon;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    IconButton::new(TransportIcon::Play.data())
        .variant(ButtonVariant::Filled)
        .show(ui);

    IconButton::new(TransportIcon::Pause.data())
        .variant(ButtonVariant::Filled)
        .show(ui);

    IconButton::new(TransportIcon::Stop.data())
        .variant(ButtonVariant::Filled)
        .show(ui);

    IconButton::new(TransportIcon::Record.data())
        .variant(ButtonVariant::Filled)
        .show(ui);
});

ui.add_space(8.0);

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    IconButton::new(TransportIcon::Rewind.data())
        .variant(ButtonVariant::Filled)
        .show(ui);

    IconButton::new(TransportIcon::Forward.data())
        .variant(ButtonVariant::Filled)
        .show(ui);

    IconButton::new(TransportIcon::Loop.data())
        .variant(ButtonVariant::Filled)
        .show(ui);

    IconButton::new(TransportIcon::Metronome.data())
        .variant(ButtonVariant::Filled)
        .show(ui);
});
```

## Real-World Example: Transport Controls

```demo
use armas::components::button::{IconButton, ButtonVariant};
use armas_audio::TransportIcon;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 4.0;

    // Navigation
    IconButton::new(TransportIcon::Rewind.data())
        .variant(ButtonVariant::Text)
        .size(24.0)
        .padding(4.0)
        .show(ui);

    IconButton::new(TransportIcon::Play.data())
        .variant(ButtonVariant::Text)
        .size(24.0)
        .padding(4.0)
        .show(ui);

    IconButton::new(TransportIcon::Stop.data())
        .variant(ButtonVariant::Text)
        .size(24.0)
        .padding(4.0)
        .show(ui);

    IconButton::new(TransportIcon::Forward.data())
        .variant(ButtonVariant::Text)
        .size(24.0)
        .padding(4.0)
        .show(ui);

    ui.add_space(16.0);

    // Toggles
    IconButton::new(TransportIcon::Loop.data())
        .variant(ButtonVariant::Text)
        .size(24.0)
        .padding(4.0)
        .show(ui);

    IconButton::new(TransportIcon::Metronome.data())
        .variant(ButtonVariant::Text)
        .size(24.0)
        .padding(4.0)
        .show(ui);

    IconButton::new(TransportIcon::Record.data())
        .variant(ButtonVariant::Text)
        .size(24.0)
        .padding(4.0)
        .show(ui);
});
```

## API Reference

```rust
IconButton::new(icon_data: &IconData)
    .variant(variant: ButtonVariant)     // Default: Filled
    .size(size: f32)                     // Default: 24.0
    .padding(padding: f32)               // Default: 8.0
    .enabled(enabled: bool)              // Default: true
    .icon_color(color: Color32)          // Optional custom icon color
    .hover_icon_color(color: Color32)    // Optional custom hover color
    .show(ui: &mut Ui) -> Response
```

### ButtonVariant Enum

```rust
pub enum ButtonVariant {
    Filled,       // Primary color background
    FilledTonal,  // Secondary container background
    Elevated,     // Surface background with shadow
    Outlined,     // Border with no background
    Text,         // Minimal styling
}
```

## Design Guidelines

- **Text Variant**: Use for low-emphasis actions or when multiple icon buttons appear together
- **Filled Variant**: Use for primary actions that need strong emphasis
- **Filled Tonal Variant**: Use for secondary actions with medium emphasis
- **Elevated Variant**: Use to add depth while maintaining subtlety
- **Outlined Variant**: Use for medium-emphasis actions with clear boundaries

### Sizing Recommendations

- **16-20px**: Compact UI elements like toolbars
- **24px**: Standard size for most icon buttons (default)
- **32-40px**: Larger touch targets or prominent actions
- **Padding**: Typically 4-12px depending on icon size
