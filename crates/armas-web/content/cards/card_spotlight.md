# Card Spotlight

A card with an animated spotlight effect that follows the mouse cursor.

## Basic Usage

```demo
use egui::Color32;

CardSpotlight::new()
    .width(400.0)
    .height(300.0)
    .show(ui, &theme, |ui| {
        ui.heading("Authentication steps");
        ui.add_space(8.0);
        ui.label("Follow these steps to secure your account:");
        ui.add_space(8.0);

        ui.label("✓ Enter your email address");
        ui.label("✓ Create a strong password");
        ui.label("✓ Set up two-factor authentication");
        ui.label("✓ Verify your identity");

        ui.add_space(12.0);
        ui.label("Ensuring your account is properly secured helps protect your personal information and data.");
    });
```

## Custom Spotlight Color

```demo
use egui::Color32;

CardSpotlight::new()
    .width(400.0)
    .height(300.0)
    .spotlight_color(Color32::from_rgba_unmultiplied(255, 100, 150, 100))
    .show(ui, &theme, |ui| {
        ui.heading("Custom Pink Spotlight");
        ui.add_space(8.0);
        ui.label("Hover over the card to see the pink spotlight effect.");
    });
```

## Larger Spotlight

```demo
use egui::Color32;

CardSpotlight::new()
    .width(400.0)
    .height(300.0)
    .spotlight_radius(250.0)
    .spotlight_color(Color32::from_rgba_unmultiplied(100, 255, 150, 80))
    .show(ui, &theme, |ui| {
        ui.heading("Large Spotlight");
        ui.add_space(8.0);
        ui.label("This card has a larger spotlight radius for a more dramatic effect.");
    });
```

## With Rich Content

```demo
use egui::Color32;

CardSpotlight::new()
    .width(450.0)
    .height(350.0)
    .show(ui, &theme, |ui| {
        ui.heading("Premium Features");
        ui.add_space(12.0);

        ui.label("✓ Unlimited projects");
        ui.label("✓ Advanced analytics");
        ui.label("✓ Priority support");
        ui.label("✓ Custom integrations");
        ui.label("✓ Team collaboration");

        ui.add_space(16.0);

        if ui.button("Upgrade Now").clicked() {
            // Handle upgrade
        }
    });
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create new card spotlight |
| `.width()` | `f32` | `available_width()` | Fixed width |
| `.height()` | `f32` | `400.0` | Fixed height |
| `.spotlight_color()` | `Color32` | Blue (80 alpha) | Spotlight color |
| `.spotlight_radius()` | `f32` | `150.0` | Spotlight radius in pixels |
| `.spotlight_intensity()` | `f32` | `0.3` | Intensity (0.0 - 1.0) |
| `.show()` | `&mut Ui, &Theme, impl FnOnce(&mut Ui)` | - | Display card with content |
