# Armas

[![Crates.io](https://img.shields.io/crates/v/armas.svg)](https://crates.io/crates/armas)
[![Documentation](https://docs.rs/armas/badge.svg)](https://docs.rs/armas)
[![License](https://img.shields.io/crates/l/armas.svg)](https://github.com/PoHsuanLai/Armas)

UI component library for [egui](https://github.com/emilk/egui) following shadcn/ui design patterns.

## Overview

Armas is a collection of 50+ reusable UI components for egui. It includes a theme system with serializable color palettes and spacing, along with components like buttons, inputs, dialogs, menus, cards, and layouts.

The design is inspired by shadcn/ui's approach to component libraries - focused on composability and consistency rather than heavy abstraction.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
armas = "0.1.0"
egui = "0.33"
```

## Example

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

## Components

Includes the following component categories:

**Basic**: Button, Input, Select, Toggle, Slider, Progress, Badge, Alert
**Layout**: Card, Separator, Accordion, Tabs, BentoGrid, Sidebar, Table
**Navigation**: Menu, Breadcrumbs, Pagination, TreeView, CommandPalette
**Overlay**: Dialog, Sheet, Drawer, Tooltip

Most components support variants (default, outline, ghost, destructive) and sizes (sm, default, lg) through builder methods.

## Theming

The theme system stores color palettes and spacing values in serializable structs. Themes can be loaded from JSON or constructed programmatically:

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

Full API documentation is available at [docs.rs/armas](https://docs.rs/armas)

## Related Crates

- [`armas`](https://crates.io/crates/armas) - Umbrella crate with feature-gated re-exports
- [`armas-audio`](https://crates.io/crates/armas-audio) - Audio/DAW UI components
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
