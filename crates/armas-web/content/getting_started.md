# Getting Started

## Installation

Add Armas to your `Cargo.toml`:

```toml
[dependencies]
armas = "0.1.0"
egui = "0.33"
```

## Theme

```demo
let theme = Theme::dark();
ui.label(format!("Theme colors loaded: {:?}", theme.colors.primary));
```

## Basic Button

```demo
Button::new("Click me")
    .variant(ButtonVariant::Filled)
    .show(ui);
```

## Layout

```demo
VStack::new(16.0).show(ui, |ui| {
    ui.label("Item 1");
    ui.label("Item 2");
    ui.label("Item 3");
});
```
