# Chip

Material Design 3 compact elements for actions, filters, selections, and suggestions.

## Chip Types

### Assist Chip

Assist chips for common actions.

```demo
if Chip::new("Add to cart")
    .chip_type(ChipType::Assist)
    .show(ui).clicked() {
    // Handle action
}
```

### Filter Chip

Filter chips for refinement with selection state.

```demo
let mut selected = true;

if Chip::new("Size: M")
    .chip_type(ChipType::Filter)
    .selected(selected)
    .show(ui).clicked() {
    selected = !selected;
}
```

### Input Chip

Input chips for user-entered content with remove button.

```demo
if Chip::new("user@email.com")
    .chip_type(ChipType::Input)
    .removable(true)
    .show(ui).remove_clicked() {
    // Handle remove
}
```

### Suggestion Chip

Suggestion chips for recommendations.

```demo
if Chip::new("Trending")
    .chip_type(ChipType::Suggestion)
    .show(ui).clicked() {
    // Handle suggestion selection
}
```

## All Chip Types

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    Chip::new("Assist")
        .chip_type(ChipType::Assist)
        .show(ui);

    Chip::new("Filter")
        .chip_type(ChipType::Filter)
        .show(ui);

    Chip::new("Input")
        .chip_type(ChipType::Input)
        .show(ui);

    Chip::new("Suggestion")
        .chip_type(ChipType::Suggestion)
        .show(ui);
});
```

## Sizes

### Small

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    Chip::new("Small Chip")
        .size(ChipSize::Small)
        .show(ui);

    Chip::new("Filter")
        .chip_type(ChipType::Filter)
        .size(ChipSize::Small)
        .selected(true)
        .show(ui);
});
```

### Medium (Default)

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    Chip::new("Medium Chip")
        .size(ChipSize::Medium)
        .show(ui);

    Chip::new("Filter")
        .chip_type(ChipType::Filter)
        .size(ChipSize::Medium)
        .selected(true)
        .show(ui);
});
```

### Large

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    Chip::new("Large Chip")
        .size(ChipSize::Large)
        .show(ui);

    Chip::new("Filter")
        .chip_type(ChipType::Filter)
        .size(ChipSize::Large)
        .selected(true)
        .show(ui);
});
```

## Selected State

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    Chip::new("Unselected")
        .chip_type(ChipType::Filter)
        .selected(false)
        .show(ui);

    Chip::new("Selected")
        .chip_type(ChipType::Filter)
        .selected(true)
        .show(ui);
});
```

## Disabled State

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    Chip::new("Disabled")
        .disabled(true)
        .show(ui);

    Chip::new("Disabled Selected")
        .chip_type(ChipType::Filter)
        .selected(true)
        .disabled(true)
        .show(ui);
});
```

## Removable Chips

```demo
ui.horizontal_wrapped(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    ui.spacing_mut().item_spacing.y = 8.0;

    Chip::new("Design")
        .chip_type(ChipType::Input)
        .removable(true)
        .show(ui);

    Chip::new("Development")
        .chip_type(ChipType::Input)
        .removable(true)
        .show(ui);

    Chip::new("Marketing")
        .chip_type(ChipType::Input)
        .removable(true)
        .show(ui);
});
```

## Assist Chip Examples

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    Chip::new("Share")
        .chip_type(ChipType::Assist)
        .show(ui);

    Chip::new("Add")
        .chip_type(ChipType::Assist)
        .show(ui);

    Chip::new("Edit")
        .chip_type(ChipType::Assist)
        .show(ui);

    Chip::new("Delete")
        .chip_type(ChipType::Assist)
        .show(ui);
});
```

## Filter Chip Examples

```demo
let mut filters = vec![
    ("All", false),
    ("Active", true),
    ("Completed", false),
    ("Archived", false),
];

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    for (label, selected) in &mut filters {
        if Chip::new(*label)
            .chip_type(ChipType::Filter)
            .selected(*selected)
            .show(ui).clicked() {
            *selected = !*selected;
        }
    }
});
```

## Suggestion Chip Examples

```demo
ui.horizontal_wrapped(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    ui.spacing_mut().item_spacing.y = 8.0;

    Chip::new("Popular")
        .chip_type(ChipType::Suggestion)
        .show(ui);

    Chip::new("Trending")
        .chip_type(ChipType::Suggestion)
        .show(ui);

    Chip::new("New")
        .chip_type(ChipType::Suggestion)
        .show(ui);

    Chip::new("Featured")
        .chip_type(ChipType::Suggestion)
        .show(ui);
});
```

## API Reference

### ChipType Enum

```rust
pub enum ChipType {
    Assist,      // Common actions
    Filter,      // Refinement with selection
    Input,       // User-entered content
    Suggestion,  // Recommendations
}
```

### ChipSize Enum

```rust
pub enum ChipSize {
    Small,   // 24px height
    Medium,  // 32px height (default)
    Large,   // 40px height
}
```

### Chip

#### Constructor

```rust
Chip::new(label: impl Into<String>) -> Self
```

#### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.chip_type()` | `ChipType` | `Assist` | Set chip type |
| `.size()` | `ChipSize` | `Medium` | Set chip size |
| `.icon()` | `String` | `None` | Set icon (emoji or unicode) |
| `.selected()` | `bool` | `false` | Set selected state (for Filter chips) |
| `.removable()` | `bool` | `false` | Enable remove button (for Input chips) |
| `.disabled()` | `bool` | `false` | Set disabled state |

#### Show Method

```rust
pub fn show(self, ui: &mut Ui) -> ChipResponse
```

### ChipResponse

```rust
pub struct ChipResponse {
    pub response: Response,
    pub clicked: bool,
    pub remove_clicked: bool,
}
```

| Method | Returns | Description |
|--------|---------|-------------|
| `.clicked()` | `bool` | Check if chip was clicked |
| `.remove_clicked()` | `bool` | Check if remove button was clicked |

## Composition Examples

### Tags Input

```demo
let mut tags = vec!["Design", "UI", "Components"];

ui.horizontal_wrapped(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    ui.spacing_mut().item_spacing.y = 8.0;

    for tag in &tags {
        Chip::new(*tag)
            .chip_type(ChipType::Input)
            .removable(true)
            .show(ui);
    }
});
```

### Category Filters

```demo
ui.vertical(|ui| {
    ui.label("Filter by category:");
    ui.add_space(8.0);

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        ui.spacing_mut().item_spacing.y = 8.0;

        Chip::new("Electronics")
            .chip_type(ChipType::Filter)
            .selected(true)
            .show(ui);

        Chip::new("Books")
            .chip_type(ChipType::Filter)
            .show(ui);

        Chip::new("Clothing")
            .chip_type(ChipType::Filter)
            .show(ui);

        Chip::new("Home")
            .chip_type(ChipType::Filter)
            .show(ui);
    });
});
```

### Quick Actions

```demo
ui.vertical(|ui| {
    ui.label("Quick actions:");
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;

        Chip::new("Copy")
            .chip_type(ChipType::Assist)
            .show(ui);

        Chip::new("Paste")
            .chip_type(ChipType::Assist)
            .show(ui);

        Chip::new("Share")
            .chip_type(ChipType::Assist)
            .show(ui);
    });
});
```

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface`, `surface_variant`, `outline`, `outline_variant`, `on_surface`, `on_surface_variant`
- Minimum version: `armas 0.2.0`
