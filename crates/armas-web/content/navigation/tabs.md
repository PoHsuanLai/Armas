# Tabs

Tab navigation with animated indicator.

```demo
let mut tabs = Tabs::new(vec!["Account", "Password"]);
let response = tabs.show(ui, &theme);
if response.changed {
    // Tab changed to response.selected
}
```

## Pre-selected Tab

```demo
let mut tabs = Tabs::new(vec!["Overview", "Analytics", "Reports"]).active(1);
tabs.show(ui, &theme);
```

## Without Animation

```demo
let mut tabs = Tabs::new(vec!["Tab 1", "Tab 2", "Tab 3"]).animate(false);
tabs.show(ui, &theme);
```
