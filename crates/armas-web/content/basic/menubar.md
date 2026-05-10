# Menubar

A horizontal menu bar for desktop-style application menus.

```demo
Menubar::new("menubar_demo").show(ui, |bar| {
    bar.menu("File", |menu| {
        menu.item("New Tab").shortcut("⌘T");
        menu.item("New Window").shortcut("⌘N");
        menu.separator();
        menu.item("Print").shortcut("⌘P");
    });
    bar.menu("Edit", |menu| {
        menu.item("Undo").shortcut("⌘Z");
        menu.item("Redo").shortcut("⇧⌘Z");
        menu.separator();
        menu.item("Cut").shortcut("⌘X");
        menu.item("Copy").shortcut("⌘C");
        menu.item("Paste").shortcut("⌘V");
    });
    bar.menu("View", |menu| {
        menu.checkbox("Always Show Bookmarks Bar", true);
        menu.checkbox("Always Show Full URLs", false);
        menu.separator();
        menu.item("Reload").shortcut("⌘R");
        menu.item("Force Reload").shortcut("⇧⌘R");
    });
});
```
