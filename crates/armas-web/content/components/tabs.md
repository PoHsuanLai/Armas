# Tabs

Animated tab navigation component with smooth transitions and multiple style variants.

## Basic Usage

```demo
let mut tabs = AnimatedTabs::new(vec!["Home", "Profile", "Settings"]);
if let Some(new_index) = tabs.show(ui) {
    // Tab changed to new_index
}
```

## Tab Styles

### Underline (Default)

```demo
let mut tabs = AnimatedTabs::new(vec!["Tab 1", "Tab 2", "Tab 3"])
    .style(TabStyle::Underline);
tabs.show(ui);
```

### Pill Style

```demo
let mut tabs = AnimatedTabs::new(vec!["Option A", "Option B", "Option C"])
    .style(TabStyle::Pill);
tabs.show(ui);
```

### Segment Style

```demo
let mut tabs = AnimatedTabs::new(vec!["Monthly", "Yearly"])
    .style(TabStyle::Segment);
tabs.show(ui);
```

## Pre-selected Tab

```demo
let mut tabs = AnimatedTabs::new(vec!["Home", "About", "Contact"])
    .active(1);
tabs.show(ui);
```

## Without Animation

```demo
let mut tabs = AnimatedTabs::new(vec!["Fast", "Instant", "Quick"])
    .animate(false);
tabs.show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `Vec<&str>` | - | Create tabs with labels |
| `.active()` | `usize` | `0` | Set active tab index |
| `.style()` | `TabStyle` | `Underline` | Set tab style |
| `.animate()` | `bool` | `true` | Enable/disable animation |

## Tab Styles

- `TabStyle::Underline` - Underline indicator (default)
- `TabStyle::Pill` - Pill/button style
- `TabStyle::Segment` - Connected segments

## Response

Returns `Option<usize>` - The new active tab index if changed.

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface`, `on_surface`
- Smooth indicator animation
