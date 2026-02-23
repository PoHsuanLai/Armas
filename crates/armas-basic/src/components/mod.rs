//! UI components
//!
//! All Armas components in a flat module structure.

pub mod accordion;
pub mod alert;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod button_group;
pub mod calendar;
pub mod card;
pub mod carousel;
pub mod checkbox;
pub mod command;
pub mod content;
pub mod context_menu;
pub mod date_picker;
pub mod dialog;
pub mod drawer;
pub mod dropdown_menu;
pub mod hover_card;
pub mod icon_button;
pub mod input;
pub mod input_group;
pub mod kbd;
pub mod loading;
pub mod menubar;
pub mod number_field;
pub mod pagination;
pub mod popover;
pub mod progress;
pub mod radio;
pub mod range_slider;
pub mod select;
pub mod separator;
pub mod sheet;
pub mod sidebar;
pub mod slider;
pub mod switch;
pub mod tabs;
pub mod textarea;
pub mod toast;
pub mod toggle;
pub mod tooltip;
pub mod tree_view;

// Re-exports
pub use accordion::{Accordion, AccordionResponse};
pub use alert::{alert, alert_destructive, Alert, AlertResponse, AlertVariant};
pub use avatar::{Avatar, AvatarShape, AvatarSize};
pub use badge::{Badge, BadgeResponse, BadgeVariant, NotificationBadge};
pub use breadcrumb::{Breadcrumb, BreadcrumbResponse};
pub use button::{Button, ButtonSize, ButtonVariant};
pub use button_group::{ButtonGroup, ButtonGroupOrientation, ButtonGroupResponse};
pub use calendar::{Calendar, CalendarResponse};
pub use card::{Card, CardResponse, CardVariant};
pub use carousel::{Carousel, CarouselOrientation, CarouselResponse};
pub use checkbox::{Checkbox, CheckboxResponse};
pub use command::{Command, CommandResponse};
pub use content::ContentContext;
pub use context_menu::{ContextMenu, ContextMenuResponse};
pub use date_picker::{Date, DatePicker, DatePickerResponse};
pub use dialog::{dialog_footer, Dialog, DialogResponse, DialogSize};
pub use drawer::{Drawer, DrawerResponse, DrawerSnapPoint};
pub use dropdown_menu::{DropdownMenu, DropdownMenuResponse, MenuBuilder};
pub use hover_card::{HoverCard, HoverCardResponse};
pub use icon_button::IconButton;
pub use input::{Input, InputState, InputVariant, SearchInput};
pub use input_group::{InputGroup, InputGroupResponse};
pub use kbd::Kbd;
pub use loading::{Skeleton, Spinner};
pub use menubar::{Menubar, MenubarResponse};
pub use number_field::{NumberField, NumberFieldResponse};
pub use pagination::{Pagination, PaginationResponse};
pub use popover::{Popover, PopoverColor, PopoverPosition, PopoverResponse, PopoverStyle};
pub use progress::{CircularProgressBar, Progress};
pub use radio::{Radio, RadioGroup, RadioGroupResponse, RadioResponse, RadioSize};
pub use range_slider::{RangeSlider, RangeSliderResponse};
pub use select::{Select, SelectOption, SelectResponse};
pub use separator::{Separator, SeparatorOrientation};
pub use sheet::{Sheet, SheetResponse, SheetSide, SheetSize};
pub use sidebar::{CollapsibleMode, Sidebar, SidebarResponse, SidebarState, SidebarVariant};
pub use slider::{Slider, SliderResponse};
pub use switch::{Switch, SwitchResponse, SwitchSize};
pub use tabs::{Tabs, TabsResponse};
pub use textarea::Textarea;
pub use toast::{ToastId, ToastManager, ToastPosition, ToastVariant};
pub use toggle::{
    Toggle, ToggleGroup, ToggleGroupResponse, ToggleGroupSize, ToggleGroupType, ToggleGroupVariant,
    ToggleResponse, ToggleSize, ToggleVariant,
};
pub use tooltip::{tooltip, tooltip_with, Tooltip, TooltipPosition};
pub use tree_view::{TreeItem, TreeView, TreeViewResponse};

// Backwards compatibility aliases
#[doc(hidden)]
pub use tree_view::{Browser, BrowserItem, BrowserResponse};
