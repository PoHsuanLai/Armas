# Sidebar

Collapsible sidebar navigation with spring animations.

```demo
Sidebar::new().show(ui, |sidebar| {
    sidebar.item("H", "Home");
    sidebar.item("P", "Profile");
    sidebar.item("S", "Settings");
});
```

## With Active Item

```demo
Sidebar::new().show(ui, |sidebar| {
    sidebar.item("D", "Dashboard").active(true);
    sidebar.item("A", "Analytics");
});
```

## With Groups

```demo
Sidebar::new().show(ui, |sidebar| {
    sidebar.group_label("Platform");
    sidebar.item("D", "Dashboard").active(true);
    sidebar.item("A", "Analytics");
    sidebar.group_label("Settings");
    sidebar.item("P", "Profile");
    sidebar.item("N", "Notifications");
});
```

## Expandable Groups

```demo
Sidebar::new().show(ui, |sidebar| {
    sidebar.item("H", "Home").active(true);
    sidebar.group("S", "Settings", |group| {
        group.item("P", "Profile");
        group.item("N", "Notifications");
        group.item("X", "Privacy");
    });
    sidebar.group("T", "Tools", |group| {
        group.item("A", "Analytics");
        group.item("L", "Logs");
    });
});
```

## With Badges

```demo
Sidebar::new().show(ui, |sidebar| {
    sidebar.item("H", "Home").active(true);
    sidebar.item("M", "Messages").badge("5");
    sidebar.item("N", "Notifications").badge("12");
});
```

## Variants

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 16.0;
    ui.label("Standard");
    Sidebar::new().variant(SidebarVariant::Sidebar).show(ui, |sidebar| {
        sidebar.item("H", "Home").active(true);
        sidebar.item("S", "Settings");
    });
    ui.label("Floating");
    Sidebar::new().variant(SidebarVariant::Floating).show(ui, |sidebar| {
        sidebar.item("H", "Home").active(true);
        sidebar.item("S", "Settings");
    });
});
```
