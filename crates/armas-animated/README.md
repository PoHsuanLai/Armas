# armas-animated

[![Crates.io](https://img.shields.io/crates/v/armas-animated.svg)](https://crates.io/crates/armas-animated)
[![Documentation](https://docs.rs/armas-animated/badge.svg)](https://docs.rs/armas-animated)
[![License](https://img.shields.io/crates/l/armas-animated.svg)](https://github.com/PoHsuanLai/Armas)

Animated text and visual effect components for [egui](https://github.com/emilk/egui).

## Overview

`armas-animated` provides a collection of animated UI components and decorative backgrounds for egui applications. Perfect for creating engaging landing pages, dashboards, and interactive interfaces.

## Features

- **Text Animations** - Typewriter, flip words, scramble, scrolling banner
- **Background Effects** - Aurora, meteor shower, sparkles, spotlight
- **Decorative Patterns** - Dot grid, retro grid, gradient overlays
- **Smooth Animations** - Built-in easing and timing controls
- **Customizable** - Full control over colors, speeds, and behaviors

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
armas-animated = "0.1.0"
armas = "0.1.0"
egui = "0.33"
```

### Examples

**Typewriter Effect:**
```rust
use armas_animated::Typewriter;

let mut typewriter = Typewriter::new("Hello, World!")
    .id("my_typewriter")
    .speed(0.05);

typewriter.show(ui);
```

**Flip Words Animation:**
```rust
use armas_animated::{FlipWords, FlipStyle};

let words = vec!["beautiful", "modern", "elegant"];
let mut flip = FlipWords::new(words)
    .id("flip")
    .style(FlipStyle::default())
    .duration(2.0);

flip.show(ui);
```

**Aurora Background:**
```rust
use armas_animated::AuroraBackground;

AuroraBackground::new()
    .colors(vec![
        Color32::from_rgb(100, 200, 255),
        Color32::from_rgb(150, 100, 255),
    ])
    .show(ui, rect, &theme);
```

**Meteor Shower:**
```rust
use armas_animated::MeteorShower;

let mut meteor = MeteorShower::new()
    .id("meteors")
    .count(20)
    .speed(0.5);

meteor.show(ui, rect, &theme);
```

## Available Components

### Text Animations
- **Typewriter** - Classic typewriter text reveal
- **WordTypewriter** - Word-by-word typewriter effect
- **FlipWords** - Animated word cycling with flip transitions
- **ScrambleText** - Text scramble/decode animation
- **ScrollingBanner** - Infinite scrolling text banner

### Background Effects
- **AuroraBackground** - Animated aurora borealis effect
- **MeteorShower** - Falling meteor animation
- **Sparkles** - Animated sparkle particles
- **Spotlight** - Mouse-tracking spotlight effect

### Decorative Patterns
- **DotPattern** - Static dot grid pattern
- **GridPattern** - Static grid pattern
- **RetroGrid** - Retro-style perspective grid

### Visual Effects
- **GradientText** - Text with animated gradient colors
- **MovingBorder** - Button with animated gradient border
- **GlowingBorder** - Animated glowing border effect

## Animation Control

Most animated components support:
- **Custom IDs** - For state persistence across frames
- **Speed/Duration** - Control animation timing
- **Loop Control** - Enable/disable looping
- **Color Customization** - Set custom colors and gradients

## Documentation

- [Full API Documentation](https://docs.rs/armas-animated)
- [Main Armas Library](https://crates.io/crates/armas)

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Links

- [Repository](https://github.com/PoHsuanLai/Armas)
- [Issue Tracker](https://github.com/PoHsuanLai/Armas/issues)
