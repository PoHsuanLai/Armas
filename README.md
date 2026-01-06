# Armas

**Armas** - A component library for egui

A reusable component library for [egui](https://github.com/emilk/egui) with Material Design inspired theming. Provides a clean, professional design language with serializable themes.

## Features

- 🎨 **Material Design Color Palette** - Primary, secondary, surface, outline, semantic colors
- 💾 **Serializable Themes** - Save/load themes as JSON
- 🧩 **Reusable Components** - Slots, routing buttons, section headers
- 🎯 **Builder Pattern API** - Clean, declarative UI code
- ✨ **Professional Design** - Glassmorphism-inspired styling

## Animated Components

### ScrollingBanner
Infinite scrolling marquee with pause-on-hover and fade effects.

```rust
use armas::{ScrollingBanner, ScrollDirection};

let mut banner = ScrollingBanner::new()
    .direction(ScrollDirection::Left)
    .speed(50.0)
    .pause_on_hover(true);

banner.show(ui, &theme, |ui, index| {
    ui.label(format!("Item {}", index));
});
```

### Spotlight
Mouse-tracking radial gradient effect.

```rust
use armas::Spotlight;

let mut spotlight = Spotlight::new()
    .radius(200.0)
    .smoothing(0.15);

spotlight.show(ui, &theme, |ui| {
    ui.heading("Highlighted Content");
});
```

### Loading Animations
Multiple loading indicator styles.

```rust
use armas::{Spinner, LoadingDots, Skeleton, CircularProgress};

// Rotating spinner
let mut spinner = Spinner::new().size(40.0);
spinner.show(ui, &theme);

// Blinking dots
let mut dots = LoadingDots::new().dot_count(3);
dots.show(ui, &theme);

// Skeleton placeholder
let mut skeleton = Skeleton::new(300.0, 20.0);
skeleton.show(ui, &theme);

// Circular progress
let mut progress = CircularProgress::new().size(40.0);
progress.show(ui, &theme);
```

### GradientCard
Card with animated gradient border.

```rust
use armas::GradientCard;

let mut card = GradientCard::rainbow()
    .width(300.0)
    .height(200.0);

card.show(ui, &theme, |ui| {
    ui.heading("Premium Content");
});
```

## Basic Components

### Slot
Colored box component with level indicator.

```rust
use armas::Slot;

let slot = Slot::new(60.0, 30.0)
    .with_effect("Reverb")
    .level(0.7);
slot.show(ui, &theme);
```

### Button, Card, Fader
See examples for more components.

## Theme System

armas includes a Material Design inspired theme system:

```rust
use armas::Theme;

// Use built-in themes
let theme = Theme::ocean();  // Default dark theme with ocean blue accent
let theme = Theme::nord();   // Nordic palette

// Access colors using Material Design naming
let color = theme.primary();           // Primary brand color
let bg = theme.surface();             // Surface color for cards/panels
let text = theme.on_surface();        // Text color on surface
let border = theme.outline();         // Border color
let hover_bg = theme.hover();         // Hover state color

// Semantic colors
let error_color = theme.error();
let success_color = theme.success();

// Save/load themes
theme.save_to_file(Path::new("my_theme.json"))?;
let theme = Theme::load_from_file(Path::new("my_theme.json"))?;
```

### Theme Colors (Material Design)

The theme follows Material Design color system:

**Brand Colors**
- `primary` - Primary brand color
- `secondary` - Secondary brand color

**Surfaces**
- `background` - Deepest background
- `surface` - For cards, panels, elevated elements
- `surface_variant` - Alternative surface color

**Text**
- `on_background` - Text on background
- `on_surface` - Text on surface
- `on_surface_variant` - Dimmed/secondary text

**Borders & Outlines**
- `outline` - Default borders
- `outline_variant` - Subtle borders

**Interactive States**
- `hover` - Hover state background
- `focus` - Focus indicator color

**Semantic Colors** (Material Design standard)
- `error` - Error states
- `warning` - Warning states
- `success` - Success states
- `info` - Informational states

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
armas = "0.1"
```

## Example

```rust
use armas::{Theme, components::{Slot, SectionHeader}};

fn ui(ui: &mut egui::Ui) {
    let theme = Theme::ocean();

    // Section header
    let header = SectionHeader::new("Items", false);
    if header.show(ui, &theme).clicked() {
        // Toggle section
    }

    // Slot component
    let slot = Slot::new(60.0, 30.0)
        .with_content("Item 1")
        .level(0.6);

    let response = slot.show(ui, &theme);
    if response.clicked() {
        // Handle interaction
    }
}
```

## Design Philosophy

armas is designed with these principles:

1. **Material Design Inspired** - Follows Material Design color system and naming
2. **Compact & Professional** - Clean, modern aesthetic
3. **Builder Pattern** - Declarative, readable code
4. **Theme-Aware** - Consistent styling across all components
5. **Minimal Dependencies** - Just egui, serde, and serde_json

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- Built with [egui](https://github.com/emilk/egui)
- Inspired by [Material Design](https://m3.material.io/) color system
- Theme serialization support
