# Armas

[![Crates.io](https://img.shields.io/crates/v/armas.svg)](https://crates.io/crates/armas)
[![Docs.rs](https://docs.rs/armas/badge.svg)](https://docs.rs/armas)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/PoHsuanLai/armas/actions/workflows/deploy.yml/badge.svg)](https://pohsuanlai.github.io/Armas/)

Live Demo: [https://pohsuanlai.github.io/armas/](https://pohsuanlai.github.io/Armas/)

A modern, theme-aware, [Material Design 3](https://m3.material.io/) compliant component library for [egui](https://github.com/emilk/egui).

Armas brings the polished aesthetics of modern web development (inspired by Shadcn, HeroUI, and Aceternity) to the Rust native ecosystem. It is designed for professional tools, game engines (like Bevy), and high-performance applications that require a sophisticated look without sacrificing the immediate-mode performance benefits of egui.

## Motivation

Egui is a fantastic library, but the default styling is often too utilitarian. Trying to build a modern UI using egui’s raw styling API often feels like fighting the tool rather than using it.

**Armas** provides a suite of pre-styled components, a UI kit for egui. We handle the drawing commands and style configurations so you can just drop in a button that looks professional immediately.

## Usage

Armas components are designed to be drop-in replacements or enhancements for standard egui widgets.

```rust
use armas::prelude::*;

// 1. Initialize the theme (usually in your app creation)
let theme = Theme::dark();
cc.egui_ctx.set_armas_theme(theme);

// 2. Use components in your update loop
ui.vertical(|ui| {
    Button::new("Deploy Project")
        .variant(ButtonVariant::Primary)
        .show(ui);
        
    // Standard egui widgets still work, but Armas provides styled wrappers
    // for common patterns.
});
```

## Running the Showcase

The workspace includes a comprehensive showcase application demonstrating all available components and themes.

**Run Native:**
```bash
cargo run -p armas-web
```

**Run Web (WASM):**
Ensure you have `trunk` installed (`cargo install trunk`).
```bash
cd crates/armas-web
trunk serve
```

## Attributions & Inspiration

Armas stands on the shoulders of giants. The visual design and component API structure are heavily inspired by the best-in-class web UI libraries:

*   **[Shadcn/ui](https://ui.shadcn.com/)**
*   **[HeroUI](https://www.heroui.com/)**
*   **[Aceternity UI](https://ui.aceternity.com/)**

## License

Licensed under the [MIT license](LICENSE).
