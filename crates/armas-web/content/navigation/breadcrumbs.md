# Breadcrumbs

Navigation path indicator styled like shadcn/ui Breadcrumb.

## Basic Usage

```demo
Breadcrumbs::new()
    .show(ui, |breadcrumbs| {
        breadcrumbs.item("Home", None);
        breadcrumbs.item("Projects", None);
        breadcrumbs.item("Armas", None).current();
    });
```

## With Icons

```demo
Breadcrumbs::new()
    .show(ui, |breadcrumbs| {
        breadcrumbs.item("Home", Some("🏠"));
        breadcrumbs.item("Documents", Some("📁"));
        breadcrumbs.item("Report.pdf", Some("📄")).current();
    });
```

## Handling Clicks

```demo
let response = Breadcrumbs::new()
    .show(ui, |breadcrumbs| {
        breadcrumbs.item("Home", None);
        breadcrumbs.item("Settings", None);
        breadcrumbs.item("Profile", None).current();
    });

if let Some(index) = response.clicked {
    // Navigate to clicked breadcrumb
}
```

## Custom Spacing

```demo
Breadcrumbs::new()
    .spacing(12.0)
    .show(ui, |breadcrumbs| {
        breadcrumbs.item("Level 1", None);
        breadcrumbs.item("Level 2", None);
        breadcrumbs.item("Level 3", None).current();
    });
```

## API Reference

### Breadcrumbs

#### Constructor

```rust
Breadcrumbs::new() -> Self
```

#### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.spacing()` | `f32` | `6.0` | Gap between items (shadcn gap-1.5) |

#### Show Method

```rust
pub fn show<R>(
    self,
    ui: &mut Ui,
    content: impl FnOnce(&mut BreadcrumbsBuilder) -> R,
) -> BreadcrumbsResponse
```

### BreadcrumbsBuilder

| Method | Description |
|--------|-------------|
| `.item(label, icon)` | Add a breadcrumb item with optional icon |

### ItemBuilder (chainable from .item())

| Method | Description |
|--------|-------------|
| `.current()` | Mark as current/active item (non-clickable) |

### BreadcrumbsResponse

| Field | Type | Description |
|-------|------|-------------|
| `clicked` | `Option<usize>` | Index of clicked breadcrumb |

## shadcn/ui Styling

The Breadcrumbs follows shadcn/ui conventions:

- **List**: `text-muted-foreground`, `gap-1.5` (6px), `text-sm` (14px)
- **Link**: `hover:text-foreground transition-colors`
- **Current page**: `text-foreground font-normal`
- **Separator**: ChevronRight icon at `size-3.5` (14px)
