# Card

Material Design 3 card component for displaying grouped content with three distinct variants.

## Basic Usage

```demo
let theme = ui.ctx().armas_theme();
Card::new()
    .title("Card Title")
    .show(ui, &theme, |ui| {
        ui.label("Card content goes here");
    });
```

## Material Design 3 Variants

### Filled (Default)

Filled cards provide subtle separation with a filled background. Best for standard content grouping.

```demo
let theme = ui.ctx().armas_theme();
Card::new()
    .variant(CardVariant::Filled)
    .title("Filled Card")
    .width(300.0)
    .show(ui, &theme, |ui| {
        ui.label("Uses surface_variant background");
        ui.label("No border, subtle separation");
        ui.add_space(8.0);
        ui.label("Best for: Standard content cards");
    });
```

### Outlined

Outlined cards have a clear boundary with a stroke. Best for forms and sections needing clear separation.

```demo
let theme = ui.ctx().armas_theme();
Card::new()
    .variant(CardVariant::Outlined)
    .title("Outlined Card")
    .width(300.0)
    .show(ui, &theme, |ui| {
        ui.label("Uses surface background");
        ui.label("1px outline_variant border");
        ui.add_space(8.0);
        ui.label("Best for: Forms, clearly defined sections");
    });
```

### Elevated

Elevated cards provide visual separation with shadow effect. Best for floating or important content.

```demo
let theme = ui.ctx().armas_theme();
Card::new()
    .variant(CardVariant::Elevated)
    .title("Elevated Card")
    .width(300.0)
    .show(ui, &theme, |ui| {
        ui.label("Uses surface background");
        ui.label("Shadow effect (simulated border)");
        ui.add_space(8.0);
        ui.label("Best for: Floating content, modals");
    });
```

## Comparison

View all three variants side by side:

```demo
let theme = ui.ctx().armas_theme();
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 16.0;

    Card::new()
        .variant(CardVariant::Filled)
        .title("Filled")
        .width(180.0)
        .show(ui, &theme, |ui| {
            ui.label("Subtle");
            ui.label("separation");
        });

    Card::new()
        .variant(CardVariant::Outlined)
        .title("Outlined")
        .width(180.0)
        .show(ui, &theme, |ui| {
            ui.label("Clear");
            ui.label("boundary");
        });

    Card::new()
        .variant(CardVariant::Elevated)
        .title("Elevated")
        .width(180.0)
        .show(ui, &theme, |ui| {
            ui.label("Visual");
            ui.label("separation");
        });
});
```

## Clickable Cards

Cards can be made interactive with hover effects:

```demo
let theme = ui.ctx().armas_theme();
let response = Card::new()
    .variant(CardVariant::Outlined)
    .title("Click Me")
    .clickable(true)
    .width(300.0)
    .show(ui, &theme, |ui| {
        ui.label("This card is clickable!");
        ui.label("Try hovering and clicking");
    });

if response.clicked() {
    ui.label("Card was clicked!");
}
```

## Custom Styling

Override default colors and properties:

```demo
let theme = ui.ctx().armas_theme();
Card::new()
    .variant(CardVariant::Filled)
    .title("Custom Card")
    .fill(egui::Color32::from_rgb(60, 40, 80))
    .stroke(theme.primary())
    .corner_radius(16.0)
    .inner_margin(24.0)
    .width(300.0)
    .show(ui, &theme, |ui| {
        ui.label("Custom background color");
        ui.label("Custom border color");
        ui.label("Custom corner radius");
    });
```

## API Reference

### Constructor

```rust
Card::new() -> Self
```

Creates a new card with default Filled variant.

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.variant()` | `CardVariant` | `Filled` | Sets the MD3 variant (Filled, Outlined, Elevated) |
| `.title()` | `&str` | `None` | Sets the card title |
| `.clickable()` | `bool` | `false` | Makes the card interactive with hover effects |
| `.width()` | `f32` | Fill available | Sets fixed width |
| `.inner_margin()` | `f32` | `theme.spacing.md` | Sets internal padding |
| `.fill()` | `Color32` | Variant default | Overrides background color |
| `.stroke()` | `Color32` | Variant default | Overrides border color |
| `.corner_radius()` | `f32` | `theme.corner_radius` | Sets corner rounding |

### Show Method

```rust
pub fn show<R>(
    self,
    ui: &mut egui::Ui,
    theme: &Theme,
    content: impl FnOnce(&mut egui::Ui) -> R,
) -> CardResponse<R>
```

Returns `CardResponse` containing the `egui::Response` and inner content result.

## Composition Examples

### With Layout Components

```demo
let theme = ui.ctx().armas_theme();
Card::new()
    .variant(CardVariant::Outlined)
    .title("Profile")
    .width(300.0)
    .show(ui, &theme, |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 12.0;
            Avatar::new("JD")
                .size(AvatarSize::Large)
                .show(ui);
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("John Doe").strong());
                ui.label("Software Engineer");
            });
        });
    });
```

## Accessibility

- **Keyboard**: Tab to focus (when clickable), Enter/Space to activate
- **Screen Reader**: Announces card title and clickable state
- **Focus**: Visible focus indicator on keyboard navigation
- **Contrast**: All variants meet WCAG AA standards with theme colors

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `surface_variant`, `outline`, `outline_variant`, `on_surface`
- Minimum version: `armas 0.2.0`
