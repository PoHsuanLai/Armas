//! Prelude module - commonly used types and traits
//!
//! Import this module to get access to the most commonly used armas types:
//!
//! ```rust
//! use armas::prelude::*;
//!
//! // Now you can use components directly
//! Badge::new("Success").show(ui);
//! Button::new("Click me").show(ui);
//! ```

// Core theme system
pub use crate::Theme;
pub use crate::ext::ArmasContextExt;

// Color utilities
pub use crate::color::{Gradient, ColorStop, BlendMode};

// Layout helpers
pub use crate::layout::{AspectRatio, ContentMode, Table, TableStyle};

// Common component enums
pub use crate::components::{
    BadgeVariant, BadgeColor,
    ButtonVariant,
    AlertVariant,
    InputVariant, InputState,
    ToggleVariant, ToggleSize,
    ToastVariant,
    ModalSize,
    DrawerSize,
    PopoverPosition,
    TooltipPosition,
};

// Essential interactive components
pub use crate::components::{
    Badge,
    Button,
    Input,
    Toggle,
    Slider,
    Select,
    Textarea,
};

// Display components
pub use crate::components::{
    Alert,
    Avatar, AvatarShape, AvatarSize,
    CircularProgress,
    Spinner,
    LoadingDots,
    Skeleton,
};

// Navigation components
pub use crate::components::{
    Breadcrumbs,
    Pagination,
    Menu,
};

// Card components
pub use crate::components::{
    Card,
    GlassPanel,
    GradientCard,
};

// Overlay components
pub use crate::components::{
    Modal,
    Drawer,
    Popover,
    Tooltip,
    HoverCard,
};

// DAW-specific components
pub use crate::components::{
    Slot,
    Fader,
    RoutingButton,
    SectionHeader,
};

// Grouping components
pub use crate::components::{
    Accordion,
    Stepper,
};

// Animation system
pub use crate::animation::{
    Animation,
    AnimationState,
    EasingFunction,
    Interpolate,
    StaggeredAnimation,
    AnimationSequence,
    LoopingAnimation,
};

// Painter extensions
pub use crate::painter_ext::PainterExt;
