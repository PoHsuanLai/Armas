# Pagination

Page navigation styled like shadcn/ui Pagination.

## Basic Usage

```demo
let (_, page) = Pagination::new(1, 10)
    .id(ui.id().with("basic"))
    .show(ui);
```

## Many Pages

```demo
let (_, page) = Pagination::new(5, 20)
    .id(ui.id().with("many"))
    .show(ui);
```

## Custom Sibling Count

Show more pages around the current page:

```demo
let (_, page) = Pagination::new(10, 20)
    .id(ui.id().with("siblings"))
    .sibling_count(2)
    .show(ui);
```

## Without Previous/Next Buttons

```demo
let (_, page) = Pagination::new(1, 5)
    .id(ui.id().with("no_nav"))
    .show_prev_next(false)
    .show(ui);
```

## API Reference

### Pagination

#### Constructor

```rust
Pagination::new(initial_page: usize, total_pages: usize) -> Self
```

#### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.id()` | `impl Into<Id>` | None | Set ID for state persistence |
| `.sibling_count()` | `usize` | `1` | Pages to show on each side of current |
| `.show_prev_next()` | `bool` | `true` | Show previous/next buttons |

#### Show Method

```rust
pub fn show(self, ui: &mut Ui) -> (Response, usize)
```

Returns the egui Response and current page number (1-indexed).

## shadcn/ui Styling

The Pagination follows shadcn/ui conventions:

- **Gap**: `gap-1` (4px) between items
- **Button size**: `size-9` (36px) for page numbers
- **Page buttons**: Ghost variant, Outline variant for current
- **Previous/Next**: Ghost buttons with chevron icons
- **Ellipsis**: Three dots (MoreHorizontal icon style)
- **Disabled state**: Muted foreground color
