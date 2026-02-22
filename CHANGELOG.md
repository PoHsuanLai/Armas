# Changelog

## 0.2.0

### Breaking Changes

- **Flattened component module structure** — All components now live directly under `armas_basic::components::*` instead of being nested in subdirectories (`basic/`, `button/`, `cards/`, `navigation/`, `overlays/`). Import paths like `armas_basic::components::basic::Slider` become `armas_basic::components::Slider`. The prelude is unaffected.
- **Renamed `Toggle` / `Switch`** — The old `Toggle` component is now `Switch` (iOS-style toggle). `Toggle` now refers to a toolbar-style toggle button (matching shadcn/ui naming).
- **Renamed `Breadcrumbs` to `Breadcrumb`** — Singular form, matching shadcn/ui convention.
- **Removed `AlertDialog`** — Use `Dialog` with `DialogSize::Small` instead.
- **Removed `ThreeValueSlider`** — Niche component not in shadcn/ui.
- **Removed `Collapsible`**, **`DataTable`**, **`NavigationMenu`**, **`ScrollArea`**, **`AspectRatio`** — Removed for maintainability; functionality covered by egui primitives.

### Added

- **Carousel** — Horizontal/vertical content carousel with navigation arrows and dot indicators.
- **HoverCard** — Content card that appears on hover with configurable delay.
- **Menubar** — Horizontal menu bar with dropdown submenus and keyboard navigation.
- **Resizable** — Resizable split panels with drag handles.
- **ButtonGroup** — Grouped buttons with shared styling and orientation.
- **InputGroup** — Input with attached prefix/suffix addons.
- **Calendar** — Standalone date grid used by DatePicker.
- **NumberField** — Numeric input with increment/decrement buttons.
- **Checkbox** — Dedicated checkbox component (previously part of Toggle).
- **`armas_audio` prelude** — Added `armas_audio::prelude` module for convenient imports.
- **Comprehensive `armas_basic` prelude** — All public components and types are now exported from the prelude.

### Changed

- **Restructured workspace into modular crates** — `armas`, `armas-basic`, `armas-audio`, `armas-icon` are now separate publishable crates with an umbrella `armas` crate.
- **Runtime SVG loading** — Icon tessellation now happens at runtime instead of compile time, reducing binary bloat.
- **Consolidated API conventions** — Consistent builder patterns, response types, and show method signatures across all components.
- **Flattened web showcase content** — Documentation sections consolidated under `components/` matching the flat source structure.
- **Scroll blocking for menus** — Dropdown and context menus now block background scroll when open.
- **Piano/pad input** — Fixed to use `Sense::drag()` for proper press/release behavior.
- **Input box state persistence** — Fixed state persistence across frames.
- **AudioData for waveform regions** — `RegionType::Audio` now carries peak data for waveform rendering.

### Removed

- All `#[allow(dead_code)]` annotations — Unused toast builder methods removed; remaining public items no longer need the annotation.

## 0.1.2

Initial public release on crates.io.
