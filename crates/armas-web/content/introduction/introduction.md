# Introduction

Armas is a modern, theme-aware, component library for [egui](https://github.com/emilk/egui).

Armas brings the polished aesthetics of modern web development (inspired by Shadcn, HeroUI, and Aceternity) to the Rust native ecosystem.

## Motivation

Egui is a fantastic library, but the default styling is often too utilitarian. Trying to build a modern UI using egui’s raw styling API often feels like fighting the tool rather than using it.

**Armas** provides a suite of pre-styled components, a UI kit for egui. We handle the drawing commands and style configurations so you can just drop in a button that looks professional immediately.

## Usage

Armas components are designed to be drop-in replacements or enhancements for standard egui widgets.

## Example

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 12.0;

    Button::new("Primary Action")
        .variant(ButtonVariant::Filled)
        .show(ui, &theme);

    Button::new("Secondary Action")
        .variant(ButtonVariant::Outlined)
        .show(ui, &theme);

    Badge::new("New")
        .variant(BadgeVariant::Filled)
        .show(ui, &theme);
});
```
