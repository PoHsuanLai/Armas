//! Prelude module - commonly used types and traits
//!
//! Import this module to get access to the most commonly used armas types:
//!
//! ```rust,no_run
//! # use egui::Ui;
//! # fn example(ui: &mut Ui) {
//! use armas::prelude::*;
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
pub use crate::layout::{AspectRatio, ContentMode, Table, TableStyle};

// Common component enums
pub use crate::components::{
    AlertVariant, BadgeVariant, ButtonSize, ButtonVariant, DialogSize, InputState, InputVariant,
    PopoverPosition, SheetSide, SheetSize, ToastVariant, ToggleSize, ToggleVariant, TooltipPosition,
};

// Essential interactive components
pub use crate::components::{
    Badge, Button, Input, RangeSlider, Select, Slider, Textarea, ThreeValueSlider, Toggle,
};

// Display components
pub use crate::components::{
    Alert, Avatar, AvatarShape, CircularProgress, Kbd, LoadingDots, Separator, Skeleton, Spinner,
};

// Navigation components
pub use crate::components::{Breadcrumbs, Menu, Pagination, Tabs};

// Card components
pub use crate::components::Card;

// Overlay components
pub use crate::components::{Dialog, DialogResponse, Drawer, Popover, Sheet, Tooltip};

// Grouping components
pub use crate::components::Accordion;

// Animation system
pub use crate::animation::{
    Animation, AnimationSequence, AnimationState, EasingFunction, Interpolate, LoopingAnimation,
    StaggeredAnimation,
};
