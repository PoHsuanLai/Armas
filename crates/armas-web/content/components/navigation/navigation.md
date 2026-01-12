# Navigation

Components for navigating through your application.

## Breadcrumbs

Show current navigation path.

```demo
Breadcrumbs::new()
    .separator("›")
    .show(ui, |breadcrumbs| {
        breadcrumbs.item("Home", None);
        breadcrumbs.item("Projects", None);
        breadcrumbs.item("Current", None).current();
    });
```

## Tabs

Animated tab navigation with smooth indicator.

```demo
let mut tabs = AnimatedTabs::new(vec![
    "Overview".to_string(),
    "Details".to_string(),
    "Settings".to_string(),
]);
tabs.show(ui);
```

## Pagination

Navigate through pages of content.

```demo
Pagination::new(1, 10).show(ui);
```

## API Reference

### Breadcrumbs

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.items()` | `Vec<&str>` | Required | Path segments |
| `.separator()` | `&str` | `"/"` | Separator character |

### AnimatedTabs

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.style()` | `TabStyle` | `Underline` | Tab visual style |
| `.active()` | `usize` | `0` | Active tab index |
| `.animate()` | `bool` | `true` | Enable animations |

### Pagination

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.current_page()` | `usize` | Required | Current page number |
| `.total_pages()` | `usize` | Required | Total page count |
| `.siblings()` | `usize` | `1` | Pages shown on each side |

## Animation Details

- **Trigger**: Click / Keyboard
- **Duration**: 200-300ms
- **Easing**: EaseOut
- **Performance**: 60fps, transform-based

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface`, `on_surface`, `outline`
