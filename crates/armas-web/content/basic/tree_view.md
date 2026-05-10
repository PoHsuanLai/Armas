# Tree View

Hierarchical tree view for displaying nested items like files and folders.

```demo
let items = vec![
    TreeItem::folder("src", "/src"),
    TreeItem::folder("components", "/src/components"),
    TreeItem::file("button.rs", "/src/components/button.rs"),
    TreeItem::file("input.rs", "/src/components/input.rs"),
    TreeItem::file("main.rs", "/src/main.rs"),
    TreeItem::folder("tests", "/tests"),
    TreeItem::file("test_button.rs", "/tests/test_button.rs"),
];
let tree_id = ui.id().with("tree_view_demo");
let mut tree: TreeView = ui.ctx().data_mut(|d| {
    d.get_persisted(tree_id).unwrap_or_else(|| {
        TreeView::new().items(items.clone()).show_lines(true).width(280.0).height(300.0)
    })
});
let response = tree.show(ui);
ui.ctx().data_mut(|d| d.insert_persisted(tree_id, tree));
if let Some(path) = response.selected {
    ui.label(format!("Selected: {:?}", path));
}
```

## With Connection Lines

```demo
let items = vec![
    TreeItem::folder("project", "/project"),
    TreeItem::folder("src", "/project/src"),
    TreeItem::file("main.rs", "/project/src/main.rs"),
    TreeItem::folder("docs", "/project/docs"),
    TreeItem::file("README.md", "/project/docs/README.md"),
];
let tree_id = ui.id().with("tree_lines");
let mut tree: TreeView = ui.ctx().data_mut(|d| {
    d.get_persisted(tree_id).unwrap_or_else(|| TreeView::new().items(items.clone()).show_lines(true).width(280.0).height(200.0))
});
tree.show(ui);
ui.ctx().data_mut(|d| d.insert_persisted(tree_id, tree));
```
