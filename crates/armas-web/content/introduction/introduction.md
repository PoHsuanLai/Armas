# Introduction

Armas is a component library for [egui](https://github.com/emilk/egui) with shadcn-inspired theming.

## Features

- **60+ Components** — Inputs, layout, feedback, display, and background effects
- **Theming** — Consistent color system with built-in themes (Dark, Light, Nord, Dracula, Studio)
- **Animations** — Spring physics, easing functions, staggered sequences
- **Builder API** — Clean, chainable component configuration

## Example

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 12.0;

    Button::new("Primary Action")
        .variant(ButtonVariant::Filled)
        .show(ui);

    Button::new("Secondary Action")
        .variant(ButtonVariant::Outlined)
        .show(ui);

    Badge::new("New")
        .variant(BadgeVariant::Filled)
        .show(ui);
});
```

## Next Steps

- [Installation](/installation/quick_start)
- [Philosophy](/introduction/philosophy)
- [Components](/components)
