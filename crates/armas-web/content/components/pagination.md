# Pagination

Page navigation for paginated content with customizable buttons and ranges.

## Basic Usage

```demo
let mut pagination = Pagination::new(1, 10);
let response = pagination.show(ui);

if let Some(page) = response.page_changed {
    // Handle page change
}
```

## Without First/Last Buttons

```demo
let mut pagination = Pagination::new(3, 10)
    .show_first_last(false);
pagination.show(ui);
```

## Custom Visible Pages

```demo
let mut pagination = Pagination::new(5, 20)
    .max_visible_pages(5);
pagination.show(ui);
```

## Minimal Pagination

```demo
let mut pagination = Pagination::new(2, 5)
    .show_first_last(false)
    .max_visible_pages(3);
pagination.show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `(usize, usize)` | - | Create with current and total pages (1-indexed) |
| `.max_visible_pages()` | `usize` | `7` | Maximum page buttons to show |
| `.show_first_last()` | `bool` | `true` | Show first/last buttons |
| `.show_prev_next()` | `bool` | `true` | Show previous/next buttons |
| `.spacing()` | `f32` | `4.0` | Spacing between buttons |

## Response

| Field | Type | Description |
|-------|------|-------------|
| `page_changed` | `Option<usize>` | New page number if changed |

## Dependencies

- `egui = "0.33"`
- Button component
- Theme colors: `primary`, `surface`
