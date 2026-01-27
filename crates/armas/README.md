# Armas

[![Crates.io](https://img.shields.io/crates/v/armas.svg)](https://crates.io/crates/armas)
[![Documentation](https://docs.rs/armas/badge.svg)](https://docs.rs/armas)
[![License](https://img.shields.io/crates/l/armas.svg)](https://github.com/PoHsuanLai/Armas)

A comprehensive component library for [egui](https://github.com/emilk/egui) with Material Design inspired theming and reusable UI components.

## Overview

Armas provides a rich set of UI components following shadcn/ui design patterns, built on top of egui. It includes a powerful theming system with serializable color palettes and spacing configurations, plus over 50 pre-built components for building modern applications.

## Features

- **🎨 Powerful Theme System** - Serializable themes with color palettes and spacing
- **🧩 50+ Components** - Buttons, inputs, dialogs, menus, cards, layouts, and more
- **🎯 shadcn/ui Inspired** - Clean, modern design patterns
- **📦 Zero Runtime Dependencies** - Pure Rust, compiles to WASM
- **⚡ Animation System** - Easing functions, interpolation, momentum scrolling
- **🎭 Variants & Sizes** - Flexible component styling with builder patterns

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
armas = "0.1.0"
egui = "0.33"
```

### Basic Example

```rust
use armas::prelude::*;
use egui::Context;

fn ui(ctx: &Context) {
    let theme = ctx.armas_theme();

    egui::CentralPanel::default().show(ctx, |ui| {
        // Button with variants
        Button::new("Click me")
            .variant(ButtonVariant::Default)
            .size(ButtonSize::Default)
            .show(ui);

        // Input field
        let mut text = String::new();
        Input::new(&mut text)
            .placeholder("Enter text...")
            .show(ui);

        // Card layout
        Card::new()
            .title("Example Card")
            .description("This is a card component")
            .show(ui, |ui| {
                ui.label("Card content goes here");
            });
    });
}
```

## Component Categories

### Basic Components
- **Button** - Multiple variants (default, outline, ghost, etc.)
- **Input** - Text input with validation
- **Select** - Dropdown selection
- **Toggle** - Switch/checkbox
- **Slider** - Range sliders
- **Progress** - Progress bars and loading indicators
- **Badge** - Labels and tags
- **Alert** - Notification messages

### Layout Components
- **Card** - Container with header/footer
- **Separator** - Dividers
- **Accordion** - Collapsible sections
- **Tabs** - Tabbed navigation
- **BentoGrid** - Responsive grid layout
- **Sidebar** - Side navigation panels
- **Table** - Data tables

### Navigation
- **Menu** - Dropdown menus
- **Breadcrumbs** - Navigation trails
- **Pagination** - Page navigation
- **TreeView** - Hierarchical navigation
- **CommandPalette** - Quick command access

### Overlay Components
- **Dialog** - Modal dialogs
- **Sheet** - Slide-out panels
- **Drawer** - Side drawers
- **Tooltip** - Hover information

## Theme System

Armas includes a comprehensive theming system:

```rust
use armas::theme::{Theme, ThemeColors};

// Load theme from JSON
let theme = Theme::load_from_json("theme.json").unwrap();

// Access theme colors
let primary = theme.primary();
let background = theme.background();

// Use with egui context
ctx.set_armas_theme(theme);
```

## Documentation

- [Full Documentation](https://docs.rs/armas)
- [Component Gallery](https://armas-ui.vercel.app) (coming soon)
- [Examples](https://github.com/PoHsuanLai/Armas/tree/master/examples)

## Related Crates

- [`armas-audio`](https://crates.io/crates/armas-audio) - Audio/DAW UI components
- [`armas-animated`](https://crates.io/crates/armas-animated) - Animated text and effects
- [`armas-icon`](https://crates.io/crates/armas-icon) - Icon rendering system

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Links

- [Repository](https://github.com/PoHsuanLai/Armas)
- [Issue Tracker](https://github.com/PoHsuanLai/Armas/issues)
