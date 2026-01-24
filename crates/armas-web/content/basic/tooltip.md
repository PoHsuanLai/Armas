# Tooltip

Contextual information that appears on hover with multiple style and color variants.

## Basic Usage

```demo
let response = Button::new("Hover me").show(ui);
let mut tooltip = Tooltip::new("This is a tooltip");
tooltip.show(ui, &response);
```

## Style Variants

### Default Style

```demo
let response = Button::new("Default Tooltip").show(ui);
let mut tooltip = Tooltip::new("Simple default tooltip")
    .style(TooltipStyle::Default);
tooltip.show(ui, &response);
```

### Rich Style

```demo
let response = Button::new("Rich Tooltip").show(ui);
let mut tooltip = Tooltip::new("Elevated rich tooltip with larger padding")
    .style(TooltipStyle::Rich);
tooltip.show(ui, &response);
```

## Color Variants

### Surface (Default)

```demo
let response = Button::new("Surface").show(ui);
let mut tooltip = Tooltip::new("Default surface color")
    .color(TooltipColor::Surface);
tooltip.show(ui, &response);
```

### Primary

```demo
let response = Button::new("Primary").show(ui);
let mut tooltip = Tooltip::new("Primary color tooltip")
    .color(TooltipColor::Primary);
tooltip.show(ui, &response);
```

### Success

```demo
let response = Button::new("Success").show(ui);
let mut tooltip = Tooltip::new("Operation completed successfully")
    .color(TooltipColor::Success);
tooltip.show(ui, &response);
```

### Warning

```demo
let response = Button::new("Warning").show(ui);
let mut tooltip = Tooltip::new("Proceed with caution")
    .color(TooltipColor::Warning);
tooltip.show(ui, &response);
```

### Error

```demo
let response = Button::new("Error").show(ui);
let mut tooltip = Tooltip::new("An error occurred")
    .color(TooltipColor::Error);
tooltip.show(ui, &response);
```

### Info

```demo
let response = Button::new("Info").show(ui);
let mut tooltip = Tooltip::new("Additional information available")
    .color(TooltipColor::Info);
tooltip.show(ui, &response);
```

## All Color Variants

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    let r1 = Button::new("Surface").show(ui);
    let mut t1 = Tooltip::new("Surface").color(TooltipColor::Surface);
    t1.show(ui, &r1);

    let r2 = Button::new("Primary").show(ui);
    let mut t2 = Tooltip::new("Primary").color(TooltipColor::Primary);
    t2.show(ui, &r2);

    let r3 = Button::new("Success").show(ui);
    let mut t3 = Tooltip::new("Success").color(TooltipColor::Success);
    t3.show(ui, &r3);

    let r4 = Button::new("Warning").show(ui);
    let mut t4 = Tooltip::new("Warning").color(TooltipColor::Warning);
    t4.show(ui, &r4);

    let r5 = Button::new("Error").show(ui);
    let mut t5 = Tooltip::new("Error").color(TooltipColor::Error);
    t5.show(ui, &r5);

    let r6 = Button::new("Info").show(ui);
    let mut t6 = Tooltip::new("Info").color(TooltipColor::Info);
    t6.show(ui, &r6);
});
```

## Positions

### Top

```demo
let response = Button::new("Top tooltip").show(ui);
let mut tooltip = Tooltip::new("Appears above")
    .position(TooltipPosition::Top);
tooltip.show(ui, &response);
```

### Bottom

```demo
let response = Button::new("Bottom tooltip").show(ui);
let mut tooltip = Tooltip::new("Appears below")
    .position(TooltipPosition::Bottom);
tooltip.show(ui, &response);
```

### Left

```demo
let response = Button::new("Left tooltip").show(ui);
let mut tooltip = Tooltip::new("Appears to the left")
    .position(TooltipPosition::Left);
tooltip.show(ui, &response);
```

### Right

```demo
let response = Button::new("Right tooltip").show(ui);
let mut tooltip = Tooltip::new("Appears to the right")
    .position(TooltipPosition::Right);
tooltip.show(ui, &response);
```

### Auto (Default)

```demo
let response = Button::new("Auto tooltip").show(ui);
let mut tooltip = Tooltip::new("Automatically positioned based on available space")
    .position(TooltipPosition::Auto);
tooltip.show(ui, &response);
```

## Custom Delay

### No Delay

```demo
let response = Button::new("Instant tooltip").show(ui);
let mut tooltip = Tooltip::new("Shows immediately")
    .delay(0);
tooltip.show(ui, &response);
```

### Long Delay

```demo
let response = Button::new("Delayed tooltip").show(ui);
let mut tooltip = Tooltip::new("Shows after 1 second")
    .delay(1000);
tooltip.show(ui, &response);
```

## Max Width

### Narrow

```demo
let response = Button::new("Narrow tooltip").show(ui);
let mut tooltip = Tooltip::new("This is a longer tooltip text that will wrap to multiple lines")
    .max_width(150.0);
tooltip.show(ui, &response);
```

### Wide

```demo
let response = Button::new("Wide tooltip").show(ui);
let mut tooltip = Tooltip::new("This is a longer tooltip text that will wrap to multiple lines")
    .max_width(300.0);
tooltip.show(ui, &response);
```

## Without Arrow

```demo
let response = Button::new("No arrow").show(ui);
let mut tooltip = Tooltip::new("Tooltip without arrow pointer")
    .show_arrow(false);
tooltip.show(ui, &response);
```

## Combined Examples

### Icon with Tooltip

```demo
let response = ui.label("(i)");
let mut tooltip = Tooltip::new("Click for more information")
    .position(TooltipPosition::Right)
    .delay(300);
tooltip.show(ui, &response);
```

### Button with Help

```demo
ui.horizontal(|ui| {
    Button::new("Submit").show(ui);
    let help = ui.label("?");
    let mut tooltip = Tooltip::new("Click Submit to process the form")
        .max_width(200.0);
    tooltip.show(ui, &help);
});
```

### Disabled Element

```demo
let response = ui.add_enabled(false, egui::Button::new("Disabled"));
let mut tooltip = Tooltip::new("This action is currently unavailable")
    .delay(200);
tooltip.show(ui, &response);
```

### Multi-line Tooltip

```demo
let response = Button::new("Details").show(ui);
let mut tooltip = Tooltip::new("Line 1: First detail\nLine 2: Second detail\nLine 3: Third detail")
    .max_width(250.0);
tooltip.show(ui, &response);
```

### Status Indicator

```demo
ui.horizontal(|ui| {
    ui.label("Server Status:");
    let status = ui.colored_label(
        ui.ctx().armas_theme().chart_2(),
        "● Online"
    );
    let mut tooltip = Tooltip::new("All systems operational")
        .position(TooltipPosition::Top)
        .delay(0);
    tooltip.show(ui, &status);
});
```

## Combined Variants

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 8.0;

    let r1 = Button::new("Rich + Primary").show(ui);
    let mut t1 = Tooltip::new("Rich style with primary color")
        .style(TooltipStyle::Rich)
        .color(TooltipColor::Primary);
    t1.show(ui, &r1);

    let r2 = Button::new("Rich + Success").show(ui);
    let mut t2 = Tooltip::new("Rich style with success color")
        .style(TooltipStyle::Rich)
        .color(TooltipColor::Success);
    t2.show(ui, &r2);

    let r3 = Button::new("Rich + Error").show(ui);
    let mut t3 = Tooltip::new("Rich style with error color")
        .style(TooltipStyle::Rich)
        .color(TooltipColor::Error);
    t3.show(ui, &r3);
});
```

## API Reference

### TooltipStyle Enum

```rust
pub enum TooltipStyle {
    Default,  // Simple, minimal tooltip
    Rich,     // Elevated appearance with larger padding
}
```

### TooltipColor Enum

```rust
pub enum TooltipColor {
    Surface,  // Default surface variant
    Primary,  // Primary color
    Success,  // Success/positive
    Warning,  // Warning/caution
    Error,    // Error/danger
    Info,     // Information
}
```

### TooltipPosition Enum

```rust
pub enum TooltipPosition {
    Top,     // Above target
    Bottom,  // Below target
    Left,    // Left of target
    Right,   // Right of target
    Auto,    // Automatically positioned (default)
}
```

### Tooltip

#### Constructor

```rust
Tooltip::new(text: impl Into<String>) -> Self
```

#### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.style()` | `TooltipStyle` | `Default` | Set visual style |
| `.color()` | `TooltipColor` | `Surface` | Set color variant |
| `.position()` | `TooltipPosition` | `Auto` | Tooltip placement |
| `.max_width()` | `f32` | `200.0` | Maximum width before wrapping |
| `.delay()` | `u64` | `500` | Delay in milliseconds |
| `.show_arrow()` | `bool` | `true` | Show arrow pointer |

#### Show Method

```rust
pub fn show(&mut self, ui: &mut Ui, target_response: &Response) -> bool
```

Returns `true` if tooltip is currently visible.

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface_variant`, `primary`, `success`, `warning`, `error`, `info`, `on_surface`, `outline`
- Minimum version: `armas 0.2.0`
