# Tabs

Tab navigation with animated indicator.

```demo
let mut tabs = Tabs::new(vec!["Account", "Password"]);
if let Some(new_index) = tabs.show(ui) {
    // Tab changed to new_index
}
```

## Pre-selected Tab

```demo
let mut tabs = Tabs::new(vec!["Overview", "Analytics", "Reports"]).active(1);
tabs.show(ui);
```

## Without Animation

```demo
let mut tabs = Tabs::new(vec!["Tab 1", "Tab 2", "Tab 3"]).animate(false);
tabs.show(ui);
```
