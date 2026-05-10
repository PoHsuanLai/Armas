# Changelog

## 0.2.1

### Added

- **`ModWheelResponse`** — `ModWheel` now returns a proper `ModWheelResponse` struct with `response` and `changed` fields.
- **`TimeRuler` component** — Restored as a standalone audio component with its own `show()` method, accessible via the `armas_audio` prelude.
- **`NumberField::bg_color()`** — New builder method to override the background color of a number field.
- **`Accordion::show_ui()` / `DropdownMenu::show_ui()`** — Closure-based content API extended to Accordion and DropdownMenu, enabling custom widget content inside these components.

### Changed

- **`Kbd` appearance** — Modifier keys now render as Unicode symbols (⌘ ⌥ ⇧ ⌃), the `+` separator is removed, and chips are smaller.
- **`DropdownMenu` item typography** — Menu items now use `sm` typography, matching the rest of the UI.
- **`Dialog`** — Added a `fixed_size` window path; cleaned up height handling.
- **`Select`** — Fills available width; trigger height and available-width calculation fixed.
- **`Slider`** — Fills available width by default.
- **`Switch`** — Snaps correctly without an explicit `.id()`.
- **`Input`** — Fixed vertical centering of text content.
- **`Checkbox`** — Fixed residual dot artifact on uncheck; spring animation is snappier and no longer requires an `.id()` to avoid a tiny ghost checkmark.
- **`DropdownMenu` submenus** — Submenus now close immediately when the pointer leaves both the trigger item and the submenu popover. Also fixed flat index collision between submenu items and parent menu items.
- **`Switch` thumb color** — Corrected thumb color rendering.
- **`IconButton` defaults** — Correct default size and padding applied.
- **`ToggleGroup` layout** — Fixed sizing so the group fills its container correctly.
- **Drum sequencer** — Visual style aligned with piano roll; grid cells are now click-to-place.
- **`show_content` renamed to `show_ui`** — Breaking rename for consistency; affects Button, Toggle, ToggleGroup, Tabs, Badge, Menubar, Accordion, DropdownMenu.
- **Content directory restructured** — `content/components/` renamed to `content/basic/` in the web showcase; URL routing updated accordingly.
- **`build.rs` cache invalidation** — The web build now correctly re-runs when any file in the content directory changes.

### Removed

- **Bundled transport SVG icons** — Removed from `armas-audio`; use `armas-icon` instead.
- **Audio components removed from `armas-audio`** — `AutomationEditor`, `MidiController`, `Playhead`, `SnapGrid`, `StepSequencer` removed (they were already removed from the public API in 0.2.0; source files are now deleted).
- **`armas-audio` and `armas-basic` duplicate license files** — Redundant per-crate `LICENSE-MIT` / `LICENSE-APACHE` files removed; workspace-level licenses apply.

## 0.2.0

### Breaking Changes

- **Flattened component module structure** — All components now live directly under `armas_basic::components::*` instead of being nested in subdirectories (`basic/`, `button/`, `cards/`, `navigation/`, `overlays/`). Import paths like `armas_basic::components::basic::Slider` become `armas_basic::components::Slider`. The prelude is unaffected.
- **Renamed `Toggle` / `Switch`** — The old `Toggle` component is now `Switch` (iOS-style toggle). `Toggle` now refers to a toolbar-style toggle button (matching shadcn/ui naming).
- **Renamed `Breadcrumbs` to `Breadcrumb`** — Singular form, matching shadcn/ui convention.
- **Removed `AlertDialog`** — Use `Dialog` with `DialogSize::Small` instead.
- **Removed `ThreeValueSlider`** — Niche component not in shadcn/ui.
- **Removed `Collapsible`**, **`DataTable`**, **`NavigationMenu`**, **`ScrollArea`**, **`AspectRatio`** — Removed for maintainability; functionality covered by egui primitives.
- **Removed DAW-specific audio components** — `Timeline`, `TimelineMarker`, `TimelineRegion`, `AutomationEditor`, `StepSequencer`, `MidiController`, and their internal utilities (`Playhead`, `SnapGrid`, `TimeRuler`, `TimelineTrack`, `TrackHeader`). These are application-level components better suited to individual projects. The library now focuses on reusable audio building blocks: knobs, faders, meters, keyboards, pads, XY pad, mod wheel, piano roll, drum sequencer, and mixer strip.

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
- **Closure-based content API** — `Button`, `Toggle`, `ToggleGroup`, `Tabs`, `Badge`, and `Menubar` now support custom content via closures. Use `show_content(ui, |ui, ctx| { ... })` to render icons, icon+text combos, or any custom widget inside these components. A shared `ContentContext` struct provides state-dependent color, font size, and active state so custom content matches the component's visual state automatically.
- **`Typography` type scale** — New `Theme.typography` field with a modular font size scale (`xxs` through `xxl`). All component font sizes now reference the theme instead of using hardcoded constants, making the entire library respond to custom type scales.

### Changed

- **Restructured workspace into modular crates** — `armas`, `armas-basic`, `armas-audio`, `armas-icon` are now separate publishable crates with an umbrella `armas` crate.
- **Runtime SVG loading** — Icon tessellation now happens at runtime instead of compile time, reducing binary bloat.
- **Consolidated API conventions** — Consistent builder patterns, response types, and show method signatures across all components.
- **Flattened web showcase content** — Documentation sections consolidated under `components/` matching the flat source structure.
- **Scroll blocking for menus** — Dropdown and context menus now block background scroll when open.
- **Piano/pad input** — Fixed to use `Sense::drag()` for proper press/release behavior.
- **Input box state persistence** — Fixed state persistence across frames.
- **AudioData for waveform regions** — `RegionType::Audio` now carries peak data for waveform rendering.
- **Font sizes use theme typography** — All ~25 component files migrated from hardcoded `const FONT_SIZE` values to `theme.typography.*` fields. Components affected: Accordion, Badge, Button, Calendar, Checkbox, Command, DatePicker, Dialog, Drawer, DropdownMenu, Input, InputGroup, Kbd, Menubar, NumberField, Pagination, Radio, Select, Switch, Tabs, Textarea, Toggle, Tooltip.

### Removed

- All `#[allow(dead_code)]` annotations — Unused toast builder methods removed; remaining public items no longer need the annotation.

## 0.1.2

Initial public release on crates.io.
