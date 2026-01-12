# Tooltip

Contextual information that appears on hover.

## Basic Usage

```demo
let response = Button::new("Hover me").show(ui);
let mut tooltip = Tooltip::new("This is a tooltip");
tooltip.show(ui, &response);
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
let response = ui.label("ℹ️");
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
        ui.ctx().armas_theme().success(),
        "● Online"
    );
    let mut tooltip = Tooltip::new("All systems operational")
        .position(TooltipPosition::Top)
        .delay(0);
    tooltip.show(ui, &status);
});
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.position()` | `TooltipPosition` | `Auto` | Tooltip placement |
| `.max_width()` | `f32` | `200.0` | Maximum width before wrapping |
| `.delay()` | `u64` | `500` | Delay in milliseconds |
| `.show_arrow()` | `bool` | `true` | Show arrow pointer |

## Positions

- `TooltipPosition::Top` - Above target
- `TooltipPosition::Bottom` - Below target
- `TooltipPosition::Left` - Left of target
- `TooltipPosition::Right` - Right of target
- `TooltipPosition::Auto` - Automatically positioned (default)

## Return Value

The `show()` method returns `bool`:
- `true` if tooltip is currently visible
- `false` if not visible (not hovered or delay not elapsed)

## Best Practices

1. **Keep it concise** - Tooltips should provide quick, helpful information
2. **Use appropriate delays** - 500ms default works well, use 0ms for critical info
3. **Consider mobile** - Tooltips don't work well on touch devices
4. **Don't repeat visible text** - Add context, don't duplicate
5. **Test positioning** - Use `Auto` for responsive layouts

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `on_surface`, `outline`
