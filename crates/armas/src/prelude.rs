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
pub use crate::ext::{ArmasContextExt, PainterExt};
pub use crate::Theme;

// Color utilities
pub use crate::color::{BlendMode, ColorStop, Gradient};

// Layout helpers
pub use crate::layout::{AspectRatio, ContentMode, Table, TableStyle};

// Common component enums
pub use crate::components::{
    AlertVariant, BadgeColor, BadgeVariant, ButtonVariant, DrawerSize, InputState, InputVariant,
    ModalSize, PopoverPosition, ToastVariant, ToggleSize, ToggleVariant, TooltipPosition,
};

// Essential interactive components
pub use crate::components::{Badge, Button, Input, Select, Slider, Textarea, Toggle};

// Display components
pub use crate::components::{
    Alert, Avatar, AvatarShape, AvatarSize, CircularProgress, LoadingDots, Skeleton, Spinner,
};

// Navigation components
pub use crate::components::{Breadcrumbs, Menu, Pagination};

// Card components
pub use crate::components::{Card, GlassPanel, GradientCard};

// Overlay components
pub use crate::components::{Drawer, HoverCard, Modal, Popover, Tooltip};

// DAW-specific components
pub use crate::components::{Fader, RoutingButton, SectionHeader, Slot};

// Grouping components
pub use crate::components::{Accordion, Stepper};

// Animation system
pub use crate::animation::{
    Animation, AnimationSequence, AnimationState, EasingFunction, Interpolate, LoopingAnimation,
    StaggeredAnimation,
};
