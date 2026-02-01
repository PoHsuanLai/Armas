//! Tree View Component
//!
//! A hierarchical tree view for displaying nested items like files/folders.

use crate::ext::ArmasContextExt;
use egui::{Pos2, Response, Sense, Ui, Vec2};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

// ============================================================================
// Constants
// ============================================================================

const ITEM_HEIGHT: f32 = 24.0;
const ITEM_GAP: f32 = 0.0;
const ITEM_PADDING_X: f32 = 8.0;
const CORNER_RADIUS: f32 = 4.0;
const INDENT_WIDTH: f32 = 16.0;

// ============================================================================
// Data Structures
// ============================================================================

/// A tree view item (file or folder)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeItem {
    /// Item name
    pub name: String,
    /// Item path
    pub path: PathBuf,
    /// Whether this is a directory/folder
    pub is_directory: bool,
}

impl TreeItem {
    /// Create a leaf item (file)
    pub fn leaf(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            is_directory: false,
        }
    }

    /// Create a branch item (folder/directory)
    pub fn branch(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            is_directory: true,
        }
    }

    /// Create a file item (alias for leaf)
    pub fn file(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self::leaf(name, path)
    }

    /// Create a folder item (alias for branch)
    pub fn folder(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self::branch(name, path)
    }
}

/// Response from the tree view
#[derive(Debug, Clone)]
pub struct TreeViewResponse {
    /// Base egui response
    pub response: Response,
    /// Item selected this frame
    pub selected: Option<PathBuf>,
    /// Branch expanded/collapsed this frame
    pub toggled: Option<PathBuf>,
}

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for `show_level` function
struct ShowLevelParams<'a> {
    parent: Option<&'a PathBuf>,
    width: f32,
    depth: usize,
    levels_last: &'a mut Vec<bool>,
    selected: &'a mut Option<PathBuf>,
    toggled: &'a mut Option<PathBuf>,
}

/// Parameters for `show_item` function
struct ShowItemParams<'a> {
    item: &'a TreeItem,
    width: f32,
    depth: usize,
    is_last: bool,
    levels_last: &'a [bool],
    selected: &'a mut Option<PathBuf>,
    toggled: &'a mut Option<PathBuf>,
    theme: &'a crate::Theme,
}

// ============================================================================
// TreeView Component
// ============================================================================

/// Hierarchical tree view component
///
/// Displays nested items in a collapsible tree structure.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas_basic::{TreeView, TreeItem};
/// use armas_basic::ext::ArmasContextExt;
///
/// let items = vec![
///     TreeItem::folder("src", "/src"),
///     TreeItem::file("main.rs", "/src/main.rs"),
///     TreeItem::file("lib.rs", "/src/lib.rs"),
/// ];
///
/// let theme = ui.ctx().armas_theme();
/// let mut tree = TreeView::new()
///     .items(items)
///     .root_path("/");
///
/// let response = tree.show(ui, &theme);
/// if let Some(path) = response.selected {
///     // Handle file selection
/// }
/// # }
/// ```
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TreeView {
    // State
    selected: Option<PathBuf>,
    #[serde(skip)]
    expanded: HashSet<String>,

    // Config
    width: f32,
    height: f32,
    items: Vec<TreeItem>,
    root_path: String,
    show_lines: bool,
}

impl TreeView {
    /// Create a new tree view
    #[must_use] 
    pub fn new() -> Self {
        Self {
            root_path: "/".to_string(),
            ..Default::default()
        }
    }

    /// Set width (0 = fill available)
    #[must_use] 
    pub const fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set height (0 = fill available)
    #[must_use] 
    pub const fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set items
    #[must_use] 
    pub fn items(mut self, items: Vec<TreeItem>) -> Self {
        self.items = items;
        self
    }

    /// Set root path for filtering
    #[must_use]
    pub fn root_path(mut self, path: impl Into<String>) -> Self {
        self.root_path = path.into();
        self
    }

    /// Show tree connection lines
    #[must_use] 
    pub const fn show_lines(mut self, show: bool) -> Self {
        self.show_lines = show;
        self
    }

    /// Get selected item
    #[must_use] 
    pub const fn selected(&self) -> Option<&PathBuf> {
        self.selected.as_ref()
    }

    /// Check if branch is expanded
    #[must_use] 
    pub fn is_expanded(&self, path: &Path) -> bool {
        self.expanded.contains(&path.to_string_lossy().to_string())
    }

    /// Toggle branch expanded state
    fn toggle(&mut self, path: &Path) {
        let key = path.to_string_lossy().to_string();
        if !self.expanded.remove(&key) {
            self.expanded.insert(key);
        }
    }

    /// Show the tree view
    pub fn show(&mut self, ui: &mut Ui, theme: &crate::Theme) -> TreeViewResponse {
        let available = ui.available_size();
        let width = if self.width > 0.0 {
            self.width
        } else {
            available.x
        };
        let height = if self.height > 0.0 {
            self.height
        } else {
            available.y
        };

        let mut selected_this_frame = None;
        let mut toggled_this_frame = None;

        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            egui::ScrollArea::vertical()
                .id_salt("tree_view_scroll")
                .show(ui, |ui| {
                    ui.add_space(theme.spacing.xs);
                    let mut levels_last = Vec::new();
                    let params = ShowLevelParams {
                        parent: None,
                        width,
                        depth: 0,
                        levels_last: &mut levels_last,
                        selected: &mut selected_this_frame,
                        toggled: &mut toggled_this_frame,
                    };
                    self.show_level(ui, params);
                });
        });

        TreeViewResponse {
            response,
            selected: selected_this_frame,
            toggled: toggled_this_frame,
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn show_level(&mut self, ui: &mut Ui, params: ShowLevelParams) {
        let theme = ui.ctx().armas_theme();
        let items = self.get_children(params.parent);
        let count = items.len();

        for (i, item) in items.into_iter().enumerate() {
            let is_expanded = item.is_directory && self.is_expanded(&item.path);
            let is_last = i == count - 1;

            let show_item_params = ShowItemParams {
                item: &item,
                width: params.width,
                depth: params.depth,
                is_last,
                levels_last: params.levels_last,
                selected: params.selected,
                toggled: params.toggled,
                theme: &theme,
            };
            self.show_item(ui, show_item_params);
            ui.add_space(ITEM_GAP);

            if is_expanded {
                let path = item.path.clone();
                params.levels_last.push(is_last);
                let nested_params = ShowLevelParams {
                    parent: Some(&path),
                    width: params.width,
                    depth: params.depth + 1,
                    levels_last: params.levels_last,
                    selected: params.selected,
                    toggled: params.toggled,
                };
                self.show_level(ui, nested_params);
                params.levels_last.pop();
            }
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn show_item(&mut self, ui: &mut Ui, params: ShowItemParams) {
        let is_selected = self.selected.as_ref() == Some(&params.item.path);
        let indent = params.depth as f32 * INDENT_WIDTH;
        let show_lines = self.show_lines && params.depth > 0;

        ui.horizontal(|ui| {
            // Tree lines prefix
            if show_lines {
                let prefix_width = indent;
                let (prefix_rect, _) =
                    ui.allocate_exact_size(Vec2::new(prefix_width, ITEM_HEIGHT), Sense::hover());

                let line_color = params.theme.border();

                // Draw vertical lines for each ancestor level
                for level in 0..params.depth {
                    let level_x =
                        prefix_rect.left() + (level as f32 * INDENT_WIDTH) + INDENT_WIDTH / 2.0;

                    // Check if ancestor at this level was the last item
                    let ancestor_is_last =
                        level < params.levels_last.len() && params.levels_last[level];

                    if !ancestor_is_last {
                        // Draw continuing vertical line
                        ui.painter().line_segment(
                            [
                                Pos2::new(level_x, prefix_rect.top()),
                                Pos2::new(level_x, prefix_rect.bottom()),
                            ],
                            egui::Stroke::new(1.0, line_color),
                        );
                    }
                }

                // Draw the connector for current item
                let line_x = prefix_rect.right() - INDENT_WIDTH / 2.0;
                let center_y = prefix_rect.center().y;

                // Vertical line segment for current level
                ui.painter().line_segment(
                    [
                        Pos2::new(line_x, prefix_rect.top()),
                        Pos2::new(
                            line_x,
                            if params.is_last {
                                center_y
                            } else {
                                prefix_rect.bottom()
                            },
                        ),
                    ],
                    egui::Stroke::new(1.0, line_color),
                );

                // Horizontal line to item
                ui.painter().line_segment(
                    [
                        Pos2::new(line_x, center_y),
                        Pos2::new(prefix_rect.right(), center_y),
                    ],
                    egui::Stroke::new(1.0, line_color),
                );
            } else if indent > 0.0 {
                ui.add_space(indent);
            }

            let item_width = (params.width - indent - ITEM_PADDING_X).max(40.0);
            let (rect, response) =
                ui.allocate_exact_size(Vec2::new(item_width, ITEM_HEIGHT), Sense::click());
            let hovered = response.hovered();

            // Background
            if is_selected || hovered {
                let color = if is_selected {
                    params.theme.accent()
                } else {
                    params.theme.accent().gamma_multiply(0.3)
                };
                ui.painter().rect_filled(rect, CORNER_RADIUS, color);
            }

            // Text color
            let text_color = if is_selected {
                params.theme.accent_foreground()
            } else {
                params.theme.foreground()
            };

            let x = rect.left() + ITEM_PADDING_X;

            // Name (with folder indicator if directory)
            let display_name = if params.item.is_directory {
                format!("{}/", params.item.name)
            } else {
                params.item.name.clone()
            };

            ui.painter().text(
                Pos2::new(x, rect.center().y),
                egui::Align2::LEFT_CENTER,
                &display_name,
                egui::FontId::proportional(13.0),
                text_color,
            );

            // Handle click
            if response.clicked() {
                if params.item.is_directory {
                    self.toggle(&params.item.path);
                    *params.toggled = Some(params.item.path.clone());
                } else {
                    self.selected = Some(params.item.path.clone());
                    *params.selected = Some(params.item.path.clone());
                }
            }
        });
    }

    fn get_children(&self, parent: Option<&PathBuf>) -> Vec<TreeItem> {
        let root = PathBuf::from(&self.root_path);

        let mut items: Vec<_> = self
            .items
            .iter()
            .filter(|item| {
                let item_parent = item.path.parent();
                match (parent, item_parent) {
                    (None, Some(p)) => p == root,
                    (Some(expected), Some(actual)) => actual == expected.as_path(),
                    _ => false,
                }
            })
            .cloned()
            .collect();

        // Sort: folders first, then by name
        items.sort_by(|a, b| match (a.is_directory, b.is_directory) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        });

        items
    }
}

// Backwards compatibility aliases
#[doc(hidden)]
pub type Browser = TreeView;
#[doc(hidden)]
pub type BrowserItem = TreeItem;
#[doc(hidden)]
pub type BrowserResponse = TreeViewResponse;
