# armas-icon

[![Crates.io](https://img.shields.io/crates/v/armas-icon.svg)](https://crates.io/crates/armas-icon)
[![Documentation](https://docs.rs/armas-icon/badge.svg)](https://docs.rs/armas-icon)
[![License](https://img.shields.io/crates/l/armas-icon.svg)](https://github.com/PoHsuanLai/Armas)

SVG icon rendering system for [egui](https://github.com/emilk/egui).

## Overview

Converts SVG paths into tessellated geometry at compile time for efficient runtime rendering. Used internally by other Armas crates but can be used standalone for custom icon needs.

## Installation

```toml
[dependencies]
armas-icon = "0.1.0"
egui = "0.33"
```

## How it works

Icons are parsed from SVG files at compile time using `usvg` and tessellated into triangle meshes with `lyon`. The resulting geometry is stored in `IconData` structs for runtime rendering.

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Links

- [Repository](https://github.com/PoHsuanLai/Armas)
- [Documentation](https://docs.rs/armas-icon)
