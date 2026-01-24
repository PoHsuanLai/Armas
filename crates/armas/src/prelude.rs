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
    AlertVariant, BadgeVariant, ButtonVariant, DialogSize, DrawerSize, InputState, InputVariant,
    PopoverPosition, ToastVariant, ToggleSize, ToggleVariant, TooltipPosition,
};

// Essential interactive components
pub use crate::components::{
    Badge, Button, Input, RangeSlider, Select, Slider, Textarea, ThreeValueSlider, Toggle,
};

// Display components
pub use crate::components::{
    Alert, Avatar, AvatarShape, AvatarSize, CircularProgress, Kbd, LoadingDots, Separator,
    Skeleton, Spinner,
};

// Navigation components
pub use crate::components::{Breadcrumbs, Menu, Pagination};

// Card components
pub use crate::components::{Card, GradientCard};

// Overlay components
pub use crate::components::{Dialog, DialogResponse, Drawer, HoverCard, Popover, Tooltip};

// Grouping components
pub use crate::components::{Accordion, Stepper};

// Animation system
pub use crate::animation::{
    Animation, AnimationSequence, AnimationState, EasingFunction, Interpolate, LoopingAnimation,
    StaggeredAnimation,
};
