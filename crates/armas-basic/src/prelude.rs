//! Prelude module - commonly used types and traits
//!
//! Import this module to get access to the most commonly used armas types:
//!
//! ```rust,no_run
//! # use egui::Ui;
//! # fn example(ui: &mut Ui) {
//! use armas_basic::prelude::*;
//!
//! // Now you can use components directly
//! Badge::new("Success").show(ui);
//! Button::new("Click me").show(ui);
//! # }
//! ```

// Core theme system
pub use crate::ext::{ArmasContextExt, PainterExt};
pub use crate::Theme;

// Color utilities
pub use crate::color::{BlendMode, ColorStop, Gradient};

// Layout helpers
pub use crate::layout::{
    cell, cell_ui, header_row, row, table, AspectRatio, ContentMode, Resizable, ResizableDirection,
    ResizablePanel,
};

// Components
pub use crate::components::{
    dialog_footer, tooltip, tooltip_with, Accordion, Alert, AlertVariant, Avatar, AvatarShape,
    Badge, BadgeVariant, Breadcrumb, Button, ButtonGroup, ButtonGroupOrientation, ButtonSize,
    ButtonVariant, Calendar, Card, Carousel, CarouselOrientation, Checkbox, CircularProgressBar,
    CollapsibleMode, Command, ContextMenu, Date, DatePicker, Dialog, DialogResponse, DialogSize,
    Drawer, DropdownMenu, HoverCard, IconButton, Input, InputGroup, InputState, InputVariant, Kbd,
    Menubar, NumberField, Pagination, Popover, PopoverPosition, Progress, Radio, RadioGroup,
    RangeSlider, SearchInput, Select, SelectOption, Separator, Sheet, SheetSide, SheetSize,
    Sidebar, SidebarVariant, Skeleton, Slider, Spinner, Switch, SwitchSize, Tabs, Textarea,
    ToastId, ToastManager, ToastPosition, ToastVariant, Toggle, ToggleGroup, ToggleGroupSize,
    ToggleGroupType, ToggleGroupVariant, ToggleSize, ToggleVariant, Tooltip, TooltipPosition,
    TreeItem, TreeView,
};

// Animation system
pub use crate::animation::{
    Animation, AnimationSequence, AnimationState, EasingFunction, Interpolate, LoopingAnimation,
    StaggeredAnimation,
};
