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

// Common component enums
pub use crate::components::{
    AlertVariant, BadgeVariant, ButtonGroupOrientation, ButtonSize, ButtonVariant,
    CarouselOrientation, DialogSize, InputState, InputVariant, PopoverPosition, SheetSide,
    SheetSize, SwitchSize, ToastVariant, ToggleGroupSize, ToggleGroupType, ToggleGroupVariant,
    ToggleSize, ToggleVariant, TooltipPosition,
};

// Essential interactive components
pub use crate::components::{
    Badge, Button, ButtonGroup, Calendar, Carousel, Checkbox, Input, InputGroup, NumberField,
    RangeSlider, Select, Slider, Switch, Textarea, Toggle, ToggleGroup,
};

// Display components
pub use crate::components::{Alert, Avatar, AvatarShape, Kbd, Separator, Skeleton, Spinner};

// Navigation components
pub use crate::components::{Breadcrumb, ContextMenu, DropdownMenu, Menubar, Pagination, Tabs};

// Card components
pub use crate::components::Card;

// Overlay components
pub use crate::components::{
    dialog_footer, Dialog, DialogResponse, Drawer, HoverCard, Popover, Sheet, Tooltip,
};

// Grouping components
pub use crate::components::Accordion;

// Animation system
pub use crate::animation::{
    Animation, AnimationSequence, AnimationState, EasingFunction, Interpolate, LoopingAnimation,
    StaggeredAnimation,
};
