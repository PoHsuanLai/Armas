# armas-animated

[![Crates.io](https://img.shields.io/crates/v/armas-animated.svg)](https://crates.io/crates/armas-animated)
[![Documentation](https://docs.rs/armas-animated/badge.svg)](https://docs.rs/armas-animated)
[![License](https://img.shields.io/crates/l/armas-animated.svg)](https://github.com/PoHsuanLai/Armas)

Animated text and background effects for [egui](https://github.com/emilk/egui).

## Overview

Collection of animated text components (typewriter, flip words, scramble) and background effects (aurora, meteor shower, sparkles, spotlight) for egui applications.

## Installation

```toml
[dependencies]
armas-animated = "0.1.0"
armas = "0.1.0"
egui = "0.33"
```

## Examples

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

## Components

**Text Animations**: Typewriter, WordTypewriter, FlipWords, ScrambleText, ScrollingBanner
**Background Effects**: AuroraBackground, MeteorShower, Sparkles, Spotlight
**Decorative Patterns**: DotPattern, GridPattern, RetroGrid
**Visual Effects**: GradientText, MovingBorder, GlowingBorder

Most components support custom IDs for state persistence, speed/duration controls, and color customization.

## Documentation

API documentation: [docs.rs/armas-animated](https://docs.rs/armas-animated)

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Links

- [Repository](https://github.com/PoHsuanLai/Armas)
- [Issue Tracker](https://github.com/PoHsuanLai/Armas/issues)
