# Pagination

Page navigation for paginated content. Styled to match shadcn/ui pagination.

## Basic Usage

```demo
let (_, page) = Pagination::new(1, 10)
    .id(ui.id().with("basic"))
    .show(ui);
```

## Icons Only (No Labels)

```demo
let (_, page) = Pagination::new(1, 10)
    .id(ui.id().with("icons"))
    .show_labels(false)
    .show(ui);
```

## Custom Visible Pages

```demo
let (_, page) = Pagination::new(1, 20)
    .id(ui.id().with("custom"))
    .max_visible_pages(5)
    .show(ui);
```

## Custom Sibling Count

```demo
let (_, page) = Pagination::new(1, 20)
    .id(ui.id().with("siblings"))
    .sibling_count(2)
    .show(ui);
```

## Without Previous/Next

```demo
let (_, page) = Pagination::new(1, 5)
    .id(ui.id().with("no_nav"))
    .show_prev_next(false)
    .show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `(usize, usize)` | - | Create with initial page and total pages |
| `.id()` | `impl Into<Id>` | - | Set ID for state persistence |
| `.max_visible_pages()` | `usize` | `7` | Maximum page buttons to show |
| `.sibling_count()` | `usize` | `1` | Pages on each side of current |
| `.show_prev_next()` | `bool` | `true` | Show previous/next buttons |
| `.show_labels()` | `bool` | `true` | Show text labels on prev/next |

## Return Value

Returns `(Response, usize)` - the egui Response and current page number.
