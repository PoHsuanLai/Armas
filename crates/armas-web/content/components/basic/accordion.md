# Accordion

Collapsible content panels with smooth animations.

## Basic Usage

```demo
let mut accordion = AccordionItem::new("What is Armas?").id("accordion_1");
accordion.show(ui, |ui| {
    ui.label("Armas is a component library for egui with Material Design inspired theming and aceternity-style visual effects.");
});
```

## Multiple Items

```demo
let mut item1 = AccordionItem::new("Getting Started").id("accordion_2");
item1.show(ui, |ui| {
    ui.label("Install Armas by adding it to your Cargo.toml dependencies.");
});

ui.add_space(8.0);

let mut item2 = AccordionItem::new("Features").id("accordion_3");
item2.show(ui, |ui| {
    ui.label("60+ components, animations, and effects. Material Design theming.");
});

ui.add_space(8.0);

let mut item3 = AccordionItem::new("Documentation").id("accordion_4");
item3.show(ui, |ui| {
    ui.label("Check out the docs at docs.rs for detailed API reference.");
});
```

## Initially Open

```demo
let mut accordion = AccordionItem::new("Already Open").id("accordion_5")
    .open(true);
accordion.show(ui, |ui| {
    ui.label("This accordion starts in the open state.");
});
```

## Without Icon

```demo
let mut accordion = AccordionItem::new("No Chevron Icon")
    .show_icon(false);
accordion.show(ui, |ui| {
    ui.label("This accordion has no chevron icon.");
});
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.open()` | `bool` | `false` | Sets initial open state |
| `.show_icon()` | `bool` | `true` | Show/hide chevron icon |
| `.animate()` | `bool` | `true` | Enable/disable animation |

## Animation Details

- **Trigger**: Click header to toggle
- **Duration**: ~125ms smooth expansion/collapse
- **Easing**: Exponential ease-out
- **Performance**: 60fps, GPU accelerated

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `on_surface`, `outline`
