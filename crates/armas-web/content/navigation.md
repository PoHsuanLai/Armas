# Navigation

## Breadcrumbs

```demo
Breadcrumbs::new()
    .items(vec!["Home", "Projects", "Current"])
    .separator("›")
    .show(ui, &theme);
```

## Tabs

```demo
let mut tabs = AnimatedTabs::new(vec![
    "Overview".to_string(),
    "Details".to_string(),
    "Settings".to_string(),
]);
tabs.show(ui, &theme);
```

## Pagination

```demo
Pagination::new()
    .current_page(3)
    .total_pages(10)
    .show(ui, &theme);
```
