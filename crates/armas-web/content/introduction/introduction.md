# Introduction

Armas is a theme-aware component library for [egui](https://github.com/emilk/egui), inspired by [shadcn/ui](https://ui.shadcn.com).

It provides styled, ready-to-use components so you can build polished interfaces without manually configuring egui's drawing commands and style API.

## Usage

Add Armas to your project and use components with the builder pattern:

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 12.0;

    Button::new("Primary Action")
        .variant(ButtonVariant::Default)
        .show(ui);

    Button::new("Secondary Action")
        .variant(ButtonVariant::Outline)
        .show(ui);

    Badge::new("New")
        .variant(BadgeVariant::Default)
        .show(ui);
});
```
