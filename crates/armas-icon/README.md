# armas-icon

[![Crates.io](https://img.shields.io/crates/v/armas-icon.svg)](https://crates.io/crates/armas-icon)
[![Documentation](https://docs.rs/armas-icon/badge.svg)](https://docs.rs/armas-icon)
[![License](https://img.shields.io/crates/l/armas-icon.svg)](https://github.com/PoHsuanLai/Armas)

Generic SVG-based icon system for [egui](https://github.com/emilk/egui).

## Overview

`armas-icon` provides a lightweight icon rendering system that converts SVG paths into tessellated geometry at compile time for efficient runtime rendering. This is a foundational library used by other Armas crates but can be used standalone for custom icon needs.

## Features

- **Compile-time SVG processing** - Icons are parsed and tessellated during build
- **Efficient rendering** - Pre-tessellated geometry for fast GPU rendering
- **Flexible sizing** - Scale icons to any size at runtime
- **Color customization** - Apply any color to icons dynamically

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
armas-icon = "0.1.0"
egui = "0.33"
```

## How It Works

This crate provides the `IconData` type that stores pre-tessellated triangle data from SVG files. Icons are processed at compile time using `usvg` and `lyon` to convert SVG paths into efficient triangle meshes.

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Links

- [Repository](https://github.com/PoHsuanLai/Armas)
- [Documentation](https://docs.rs/armas-icon)
- [Main Armas Library](https://crates.io/crates/armas)
