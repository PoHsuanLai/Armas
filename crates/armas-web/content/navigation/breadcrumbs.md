# Breadcrumbs

Navigation path indicator with chevron separators.

```demo
Breadcrumbs::new().show(ui, |breadcrumbs| {
    breadcrumbs.item("Home", None);
    breadcrumbs.item("Projects", None);
    breadcrumbs.item("Armas", None).current();
});
```

## With Icons

```demo
Breadcrumbs::new().show(ui, |breadcrumbs| {
    breadcrumbs.item("Home", Some("🏠"));
    breadcrumbs.item("Documents", Some("📁"));
    breadcrumbs.item("Report.pdf", Some("📄")).current();
});
```

## Handling Clicks

```demo
let response = Breadcrumbs::new().show(ui, |breadcrumbs| {
    breadcrumbs.item("Home", None);
    breadcrumbs.item("Settings", None);
    breadcrumbs.item("Profile", None).current();
});
if let Some(index) = response.clicked {
    // Navigate to clicked breadcrumb
}
```
