//! Navigation components
//!
//! Components for navigating through the application.

pub mod breadcrumb;
pub mod command;
pub mod context_menu;
pub mod dropdown_menu;
pub mod pagination;
pub mod sidebar;
pub mod tabs;
pub mod tree_view;

// Re-exports
pub use breadcrumb::{Breadcrumb, BreadcrumbResponse};
pub use command::{Command, CommandResponse};
pub use context_menu::{ContextMenu, ContextMenuResponse};
pub use dropdown_menu::{DropdownMenu, DropdownMenuResponse, MenuBuilder};
pub use pagination::{Pagination, PaginationResponse};
pub use sidebar::{CollapsibleMode, Sidebar, SidebarResponse, SidebarState, SidebarVariant};
pub use tabs::Tabs;
pub use tree_view::{TreeItem, TreeView, TreeViewResponse};

// Backwards compatibility aliases
#[doc(hidden)]
pub use tree_view::{Browser, BrowserItem, BrowserResponse};
