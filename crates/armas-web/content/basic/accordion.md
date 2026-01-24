# Accordion

Collapsible content sections. Styled to match shadcn/ui accordion.

## Basic Usage

```demo
Accordion::new("demo_accordion", vec!["What is Armas?"])
    .show(ui, |ui, _idx| {
        ui.label("Armas is a component library for egui with shadcn-inspired styling.");
    });
```

## Multiple Items

```demo
Accordion::new("demo_multi", vec!["Getting Started", "Features", "Documentation"])
    .show(ui, |ui, idx| {
        match idx {
            0 => { ui.label("Install Armas by adding it to your Cargo.toml."); }
            1 => { ui.label("60+ components with shadcn styling."); }
            _ => { ui.label("Check out docs.rs for API reference."); }
        }
    });
```

## Allow Multiple Open

```demo
Accordion::new("demo_multiple", vec!["Section A", "Section B", "Section C"])
    .allow_multiple(true)
    .show(ui, |ui, idx| {
        ui.label(format!("Content for section {}", idx + 1));
    });
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `id, titles` | - | Create with ID and title list |
| `.allow_multiple()` | `bool` | `false` | Allow multiple sections open |
| `.show()` | `ui, fn` | - | Display with content callback |

## Styling

- Trigger: `hover:underline`, chevron on right
- Chevron: Rotates 180deg when open
- Border: Bottom border on each item
- Animation: Smooth expand/collapse
