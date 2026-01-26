# Pagination

Page navigation with numbered buttons and arrows.

```demo
let (_, page) = Pagination::new(1, 10).id(ui.id().with("basic")).show(ui);
```

## Many Pages

```demo
let (_, page) = Pagination::new(5, 20).id(ui.id().with("many")).show(ui);
```

## Custom Sibling Count

```demo
let (_, page) = Pagination::new(10, 20).id(ui.id().with("siblings")).sibling_count(2).show(ui);
```

## Without Previous/Next

```demo
let (_, page) = Pagination::new(1, 5).id(ui.id().with("no_nav")).show_prev_next(false).show(ui);
```
