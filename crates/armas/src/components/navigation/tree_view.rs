//! Tree View Component
//!
//! A hierarchical tree view for displaying nested items like files/folders.

use crate::ext::ArmasContextExt;
use egui::{Pos2, Response, Sense, Ui, Vec2};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

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
    pub name: String,
    pub path: PathBuf,
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
    pub response: Response,
    /// Item selected this frame
    pub selected: Option<PathBuf>,
    /// Branch expanded/collapsed this frame
    pub toggled: Option<PathBuf>,
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
/// use armas::{TreeView, TreeItem};
///
/// let items = vec![
///     TreeItem::folder("src", "/src"),
///     TreeItem::file("main.rs", "/src/main.rs"),
///     TreeItem::file("lib.rs", "/src/lib.rs"),
/// ];
///
/// let mut tree = TreeView::new()
///     .items(items)
///     .root_path("/");
///
/// let response = tree.show(ui);
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
    pub fn new() -> Self {
        Self {
            root_path: "/".to_string(),
            ..Default::default()
        }
    }

    /// Set width (0 = fill available)
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set height (0 = fill available)
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set items
    pub fn items(mut self, items: Vec<TreeItem>) -> Self {
        self.items = items;
        self
    }

    /// Set root path for filtering
    pub fn root_path(mut self, path: impl Into<String>) -> Self {
        self.root_path = path.into();
        self
    }

    /// Show tree connection lines
    pub fn show_lines(mut self, show: bool) -> Self {
        self.show_lines = show;
        self
    }

    /// Get selected item
    pub fn selected(&self) -> Option<&PathBuf> {
        self.selected.as_ref()
    }

    /// Check if branch is expanded
    pub fn is_expanded(&self, path: &PathBuf) -> bool {
        self.expanded.contains(&path.to_string_lossy().to_string())
    }

    /// Toggle branch expanded state
    fn toggle(&mut self, path: &PathBuf) {
        let key = path.to_string_lossy().to_string();
        if !self.expanded.remove(&key) {
            self.expanded.insert(key);
        }
    }

    /// Show the tree view
    pub fn show(&mut self, ui: &mut Ui, theme: &crate::Theme) -> TreeViewResponse {

        let available = ui.available_size();
        let width = if self.width > 0.0 { self.width } else { available.x };
        let height = if self.height > 0.0 { self.height } else { available.y };

        let mut selected_this_frame = None;
        let mut toggled_this_frame = None;

        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            egui::ScrollArea::vertical()
                .id_salt("tree_view_scroll")
                .show(ui, |ui| {
                    ui.add_space(theme.spacing.xs);
                    let mut levels_last = Vec::new();
                    self.show_level(
                        ui,
                        None,
                        width,
                        0,
                        &mut levels_last,
                        &mut selected_this_frame,
                        &mut toggled_this_frame,
                    );
                });
        });

        TreeViewResponse {
            response,
            selected: selected_this_frame,
            toggled: toggled_this_frame,
        }
    }

    fn show_level(
        &mut self,
        ui: &mut Ui,
        parent: Option<&PathBuf>,
        width: f32,
        depth: usize,
        levels_last: &mut Vec<bool>,
        selected: &mut Option<PathBuf>,
        toggled: &mut Option<PathBuf>,
    ) {
        let theme = ui.ctx().armas_theme();
        let items = self.get_children(parent);
        let count = items.len();

        for (i, item) in items.into_iter().enumerate() {
            let is_expanded = item.is_directory && self.is_expanded(&item.path);
            let is_last = i == count - 1;

            self.show_item(ui, &item, width, depth, is_last, levels_last, selected, toggled, &theme);
            ui.add_space(ITEM_GAP);

            if is_expanded {
                let path = item.path.clone();
                levels_last.push(is_last);
                self.show_level(ui, Some(&path), width, depth + 1, levels_last, selected, toggled);
                levels_last.pop();
            }
        }
    }

    fn show_item(
        &mut self,
        ui: &mut Ui,
        item: &TreeItem,
        width: f32,
        depth: usize,
        is_last: bool,
        levels_last: &[bool],
        selected: &mut Option<PathBuf>,
        toggled: &mut Option<PathBuf>,
        theme: &crate::Theme,
    ) {
        let is_selected = self.selected.as_ref() == Some(&item.path);
        let indent = depth as f32 * INDENT_WIDTH;
        let show_lines = self.show_lines && depth > 0;

        ui.horizontal(|ui| {
            // Tree lines prefix
            if show_lines {
                let prefix_width = indent;
                let (prefix_rect, _) = ui.allocate_exact_size(
                    Vec2::new(prefix_width, ITEM_HEIGHT),
                    Sense::hover(),
                );

                let line_color = theme.border();

                // Draw vertical lines for each ancestor level
                for level in 0..depth {
                    let level_x = prefix_rect.left() + (level as f32 * INDENT_WIDTH) + INDENT_WIDTH / 2.0;

                    // Check if ancestor at this level was the last item
                    let ancestor_is_last = level < levels_last.len() && levels_last[level];

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
                        Pos2::new(line_x, if is_last { center_y } else { prefix_rect.bottom() }),
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

            let item_width = (width - indent - ITEM_PADDING_X).max(40.0);
            let (rect, response) = ui.allocate_exact_size(
                Vec2::new(item_width, ITEM_HEIGHT),
                Sense::click(),
            );
            let hovered = response.hovered();

            // Background
            if is_selected || hovered {
                let color = if is_selected {
                    theme.accent()
                } else {
                    theme.accent().gamma_multiply(0.3)
                };
                ui.painter().rect_filled(rect, CORNER_RADIUS, color);
            }

            // Text color
            let text_color = if is_selected {
                theme.accent_foreground()
            } else {
                theme.foreground()
            };

            let x = rect.left() + ITEM_PADDING_X;

            // Name (with folder indicator if directory)
            let display_name = if item.is_directory {
                format!("{}/", item.name)
            } else {
                item.name.clone()
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
                if item.is_directory {
                    self.toggle(&item.path);
                    *toggled = Some(item.path.clone());
                } else {
                    self.selected = Some(item.path.clone());
                    *selected = Some(item.path.clone());
                }
            }
        });
    }

    fn get_children(&self, parent: Option<&PathBuf>) -> Vec<TreeItem> {
        let root = PathBuf::from(&self.root_path);

        let mut items: Vec<_> = self.items.iter().filter(|item| {
            let item_parent = item.path.parent();
            match (parent, item_parent) {
                (None, Some(p)) => p == root,
                (Some(expected), Some(actual)) => actual == expected.as_path(),
                _ => false,
            }
        }).cloned().collect();

        // Sort: folders first, then by name
        items.sort_by(|a, b| {
            match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            }
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
