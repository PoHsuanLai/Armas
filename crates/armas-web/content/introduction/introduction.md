# Introduction

Welcome to **Armas** - a modern component library for [egui](https://github.com/emilk/egui) with Material Design inspired theming and sophisticated visual effects.

## What is Armas?

Armas provides 60+ pre-built UI components that bring modern web aesthetics to egui applications. Instead of spending hours tweaking egui's raw styling API, you can drop in components that look professional and contemporary right out of the box.

## Key Features

### 🎨 **Material Design Theming**
- Consistent color system across all components
- Built-in themes (Dark, Light, Nord, Dracula, Studio)
- Serializable theme files for customization

### ⚡ **60+ Components**
- **Inputs**: Button, Input, Select, Toggle, Checkbox, Radio
- **Layout**: VStack, HStack, Grid, Container, ScrollView
- **Feedback**: Alert, Toast, Modal, Drawer, Tooltip
- **Display**: Card, Badge, Avatar, Timeline, Testimonial
- **Effects**: Aurora, Vortex, Meteors, Spotlight

### 🎭 **Smooth Animations**
- Spring physics animations
- Easing functions (Linear, EaseIn, EaseOut, Cubic, etc.)
- Staggered and sequenced animations
- 60fps performance

### 🎯 **Developer Experience**
- Builder pattern API for clean, readable code
- Comprehensive examples (20+ examples)
- Copy-paste philosophy - self-contained components
- Minimal dependencies (egui, serde, serde_json)

## Quick Example

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
        .color(BadgeColor::Primary)
        .show(ui);
});
```

## Who is Armas For?

Armas is perfect for:
- 🎮 **Game developers** using egui with Bevy or other engines
- 🖥️ **Native app developers** who want modern UI without web overhead
- 🛠️ **Tool builders** creating professional-looking dev tools
- 🎨 **Designers** who want beautiful components without custom drawing

## Next Steps

- Read about our [Philosophy](/introduction/philosophy)
- Learn [Why Egui?](/introduction/why_egui)
- Get started with [Installation](/installation/quick_start)
- Explore [Components](/components)
