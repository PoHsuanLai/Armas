# Tooltip

Contextual information that appears on hover, styled like shadcn/ui.

## Basic Usage

```demo
let response = Button::new("Hover me").show(ui);
Tooltip::new("This is a tooltip").show(ui, &response);
```

## Helper Function

For simple cases, use the `tooltip` helper:

```demo
let response = Button::new("Simple tooltip").show(ui);
tooltip(ui, &response, "Quick tooltip text");
```

## Positions

### Top (Default)

```demo
let response = Button::new("Top tooltip").show(ui);
Tooltip::new("Appears above")
    .position(TooltipPosition::Top)
    .show(ui, &response);
```

### Bottom

```demo
let response = Button::new("Bottom tooltip").show(ui);
Tooltip::new("Appears below")
    .position(TooltipPosition::Bottom)
    .show(ui, &response);
```

### Left

```demo
let response = Button::new("Left tooltip").show(ui);
Tooltip::new("Appears to the left")
    .position(TooltipPosition::Left)
    .show(ui, &response);
```

### Right

```demo
let response = Button::new("Right tooltip").show(ui);
Tooltip::new("Appears to the right")
    .position(TooltipPosition::Right)
    .show(ui, &response);
```

## Custom Delay

### No Delay (Default)

```demo
let response = Button::new("Instant tooltip").show(ui);
Tooltip::new("Shows immediately").show(ui, &response);
```

### With Delay

```demo
let response = Button::new("Delayed tooltip").show(ui);
Tooltip::new("Shows after 500ms")
    .delay(500)
    .show(ui, &response);
```

## Max Width

### Narrow

```demo
let response = Button::new("Narrow tooltip").show(ui);
Tooltip::new("This is a longer tooltip text that will wrap to multiple lines")
    .max_width(150.0)
    .show(ui, &response);
```

### Wide

```demo
let response = Button::new("Wide tooltip").show(ui);
Tooltip::new("This is a longer tooltip text that stays on one line when possible")
    .max_width(400.0)
    .show(ui, &response);
```

## Without Arrow

```demo
let response = Button::new("No arrow").show(ui);
Tooltip::new("Tooltip without arrow pointer")
    .arrow(false)
    .show(ui, &response);
```

## Combined Examples

### Icon with Tooltip

```demo
let response = ui.label("(i)");
Tooltip::new("Click for more information")
    .position(TooltipPosition::Right)
    .show(ui, &response);
```

### Button with Help

```demo
ui.horizontal(|ui| {
    Button::new("Submit").show(ui);
    let help = ui.label("?");
    Tooltip::new("Click Submit to process the form")
        .max_width(200.0)
        .show(ui, &help);
});
```

### Multi-line Tooltip

```demo
let response = Button::new("Details").show(ui);
Tooltip::new("Line 1: First detail\nLine 2: Second detail\nLine 3: Third detail")
    .max_width(250.0)
    .show(ui, &response);
```

### With Custom Configuration

```demo
let response = Button::new("Custom").show(ui);
tooltip_with(ui, &response, "Customized tooltip", |t| {
    t.position(TooltipPosition::Bottom)
     .delay(300)
     .max_width(200.0)
});
```

## API Reference

### TooltipPosition Enum

```rust
pub enum TooltipPosition {
    Top,     // Above target (default)
    Bottom,  // Below target
    Left,    // Left of target
    Right,   // Right of target
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
| `.position()` | `TooltipPosition` | Auto-detect | Tooltip placement |
| `.max_width()` | `f32` | `300.0` | Maximum width before wrapping |
| `.delay()` | `u64` | `0` | Delay in milliseconds |
| `.arrow()` | `bool` | `true` | Show arrow pointer |

#### Show Method

```rust
pub fn show(self, ui: &mut Ui, target_response: &Response) -> bool
```

Returns `true` if tooltip is currently visible.

### Helper Functions

```rust
// Simple tooltip
tooltip(ui, &response, "Text");

// Tooltip with configuration
tooltip_with(ui, &response, "Text", |t| t.delay(500));
```

## shadcn/ui Styling

The Tooltip follows shadcn/ui conventions:

- **Background**: `bg-foreground` (inverted - dark in light mode, light in dark mode)
- **Text**: `text-background` (inverted for contrast)
- **Padding**: `px-3 py-1.5` (12px horizontal, 6px vertical)
- **Font size**: `text-xs` (12px)
- **Border radius**: `rounded-md` (6px)
- **Delay**: `0ms` by default (instant)
