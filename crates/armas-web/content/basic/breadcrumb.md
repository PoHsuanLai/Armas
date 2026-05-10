# Breadcrumb

Navigation path indicator with chevron separators.

```demo
Breadcrumb::new().show(ui, |breadcrumb| {
    breadcrumb.item("Home", None);
    breadcrumb.item("Projects", None);
    breadcrumb.item("Armas", None).current();
});
```

## With Icons

```demo
Breadcrumb::new().show(ui, |breadcrumb| {
    breadcrumb.item("Home", Some("🏠"));
    breadcrumb.item("Documents", Some("📁"));
    breadcrumb.item("Report.pdf", Some("📄")).current();
});
```

## Handling Clicks

```demo
let response = Breadcrumb::new().show(ui, |breadcrumb| {
    breadcrumb.item("Home", None);
    breadcrumb.item("Settings", None);
    breadcrumb.item("Profile", None).current();
});
if let Some(index) = response.clicked {
    // Navigate to clicked breadcrumb
}
```
