# Sidebar

Collapsible sidebar navigation styled to match shadcn/ui conventions with spring-based animations.

Supports both **controlled mode** (with external `SidebarState`) and **uncontrolled mode**.

## Basic Usage

```demo
Sidebar::new()
    .show(ui, |sidebar| {
        sidebar.item("H", "Home");
        sidebar.item("P", "Profile");
        sidebar.item("S", "Settings");
    });
```

## With Active Item

```demo
Sidebar::new()
    .show(ui, |sidebar| {
        sidebar.item("D", "Dashboard").active(true);
        sidebar.item("A", "Analytics");
    });
```

## With Group Labels

```demo
Sidebar::new()
    .show(ui, |sidebar| {
        sidebar.group_label("Platform");
        sidebar.item("D", "Dashboard").active(true);
        sidebar.item("A", "Analytics");

        sidebar.group_label("Settings");
        sidebar.item("P", "Profile");
        sidebar.item("N", "Notifications");
    });
```

## With Expandable Groups

```demo
Sidebar::new()
    .show(ui, |sidebar| {
        sidebar.item("H", "Home").active(true);
        sidebar.group("S", "Settings", |group| {
            group.item("P", "Profile");
            group.item("N", "Notifications");
            group.item("X", "Privacy");
        });
        sidebar.group("T", "Tools", |group| {
            group.item("A", "Analytics");
            group.item("L", "Logs");
        });
    });
```

## With Badges

```demo
Sidebar::new()
    .show(ui, |sidebar| {
        sidebar.item("H", "Home").active(true);
        sidebar.item("M", "Messages").badge("5");
        sidebar.item("N", "Notifications").badge("12");
    });
```

## Variants

### Standard (default)

```demo
Sidebar::new()
    .variant(SidebarVariant::Sidebar)
    .show(ui, |sidebar| {
        sidebar.item("H", "Home").active(true);
        sidebar.item("S", "Settings");
    });
```

### Floating

```demo
Sidebar::new()
    .variant(SidebarVariant::Floating)
    .show(ui, |sidebar| {
        sidebar.item("H", "Home").active(true);
        sidebar.item("S", "Settings");
    });
```

## Collapsible Modes

### Icon Mode (default)

```demo
Sidebar::new()
    .collapsible(CollapsibleMode::Icon)
    .show(ui, |sidebar| {
        sidebar.item("H", "Home").active(true);
        sidebar.item("S", "Settings");
    });
```

### Not Collapsible

```demo
Sidebar::new()
    .collapsible(CollapsibleMode::None)
    .show(ui, |sidebar| {
        sidebar.item("H", "Home").active(true);
        sidebar.item("S", "Settings");
    });
```

## Controlled Mode

Use `SidebarState` for external control over the sidebar state.

```rust
// Store state somewhere persistent (e.g., in your app struct)
let mut sidebar_state = SidebarState::new(true);

// In your UI code:
Sidebar::new()
    .state(&mut sidebar_state)
    .show(ui, |sidebar| {
        sidebar.item("H", "Home").active(true);
    });

// Toggle from anywhere (e.g., a button, keyboard shortcut):
if toggle_button_clicked {
    sidebar_state.toggle();
}
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create sidebar with shadcn defaults |
| `.state()` | `&mut SidebarState` | - | Use external state (controlled mode) |
| `.collapsed()` | `bool` | `false` | Start in collapsed state (uncontrolled) |
| `.collapsed_width()` | `f32` | `48.0` | Width when collapsed (3rem) |
| `.expanded_width()` | `f32` | `256.0` | Width when expanded (16rem) |
| `.collapsible()` | `CollapsibleMode` | `Icon` | Collapsible behavior |
| `.variant()` | `SidebarVariant` | `Sidebar` | Visual style variant |
| `.show_icons()` | `bool` | `true` | Show icons |
| `.show()` | closure | - | Render with closure-based API |

### SidebarState

| Method | Description |
|--------|-------------|
| `::new(open: bool)` | Create new state |
| `.toggle()` | Toggle open/closed with animation |
| `.set_open(bool)` | Set open state with animation |
| `.is_open()` | Check if currently open |
| `.width()` | Get current animated width |
| `.is_animating()` | Check if animation is running |

### CollapsibleMode

| Variant | Description |
|---------|-------------|
| `Icon` | Collapse to icon-only view (default) |
| `Offcanvas` | Slide completely off screen |
| `None` | Not collapsible |

### SidebarVariant

| Variant | Description |
|---------|-------------|
| `Sidebar` | Standard sidebar with right border (default) |
| `Floating` | Rounded corners with shadow |
| `Inset` | Similar to floating for inset layouts |

### SidebarBuilder (in closure)

| Method | Type | Description |
|--------|------|-------------|
| `.item()` | `(&str, &str)` | Add item with icon and label |
| `.group_label()` | `&str` | Add non-interactive section header |
| `.group()` | `(&str, &str, closure)` | Add expandable group |

### ItemBuilder (chainable from .item())

| Method | Type | Description |
|--------|------|-------------|
| `.active()` | `bool` | Mark as active |
| `.badge()` | `&str` | Add badge text |

## Sizing (shadcn conventions)

| Element | Size | Notes |
|---------|------|-------|
| Expanded width | 256px | 16rem |
| Collapsed width | 48px | 3rem |
| Item height | 32px | h-8 |
| Sub-item height | 28px | h-7 |
| Item gap | 4px | gap-1 |
| Icon size | 16px | size-4 |
| Corner radius | 6px | rounded-md |
