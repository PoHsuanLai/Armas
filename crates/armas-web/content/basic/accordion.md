# Accordion

Collapsible content sections with smooth animations.

```demo
Accordion::new("basic", vec!["What is Armas?"]).show(ui, |ui, _idx| {
    ui.label("Armas is a component library for egui with shadcn-inspired styling.");
});
```

## Multiple Items

```demo
Accordion::new("multi", vec!["Getting Started", "Features", "Documentation"]).show(ui, |ui, idx| {
    match idx {
        0 => { ui.label("Install Armas by adding it to your Cargo.toml."); }
        1 => { ui.label("60+ components with shadcn styling."); }
        _ => { ui.label("Check out docs.rs for API reference."); }
    }
});
```

## Allow Multiple Open

```demo
Accordion::new("multiple", vec!["Section A", "Section B", "Section C"]).allow_multiple(true).show(ui, |ui, idx| {
    ui.label(format!("Content for section {}", idx + 1));
});
```
