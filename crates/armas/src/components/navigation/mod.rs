//! Navigation components
//!
//! Components for navigating through the application.

pub mod breadcrumbs;
pub mod command;
pub mod menu;
pub mod pagination;
pub mod sidebar;
pub mod tabs;
pub mod tree_view;

// Re-exports
pub use breadcrumbs::{Breadcrumbs, BreadcrumbsResponse};
pub use command::{Command, CommandResponse};
pub use menu::{Menu, MenuResponse};
pub use pagination::Pagination;
pub use sidebar::{CollapsibleMode, Sidebar, SidebarResponse, SidebarState, SidebarVariant};
pub use tabs::Tabs;

// Backwards compatibility
#[doc(hidden)]
pub use tabs::{AnimatedTabs, TabStyle};
pub use tree_view::{TreeItem, TreeView, TreeViewResponse};

// Backwards compatibility aliases
#[doc(hidden)]
pub use tree_view::{Browser, BrowserItem, BrowserResponse};
