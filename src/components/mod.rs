//! Reusable UI components for egui
//!
//! This module contains general-purpose UI components with a clean,
//! professional design language.

pub mod accordion;
pub mod badge;
pub mod button;
pub mod card;
pub mod fader;
pub mod feature_grid;
pub mod glass_panel;
pub mod gradient_card;
pub mod loading;
pub mod progress;
pub mod routing_button;
pub mod scrolling_banner;
pub mod section_header;
pub mod slot;
pub mod spotlight;
pub mod tabs;
pub mod testimonial;
pub mod timeline;

// Re-exports
pub use accordion::{Accordion, AccordionItem};
pub use badge::{Badge, BadgeColor, BadgeVariant, NotificationBadge};
pub use button::{Button, ButtonVariant};
pub use card::{Card, CardResponse};
pub use fader::{Fader, FaderStrip};
pub use feature_grid::{FeatureGrid, FeatureItem};
pub use glass_panel::{GlassPanel, GlassPanelResponse};
pub use gradient_card::GradientCard;
pub use loading::{CircularProgress, LoadingDots, Skeleton, Spinner};
pub use progress::{CircularProgressBar, LinearProgress, RingProgress};
pub use routing_button::RoutingButton;
pub use scrolling_banner::{ScrollDirection, ScrollingBanner};
pub use section_header::SectionHeader;
pub use slot::Slot;
pub use spotlight::{MultiSpotlight, Spotlight};
pub use tabs::{AnimatedTabs, TabStyle};
pub use testimonial::{TestimonialCard, TestimonialGrid, TestimonialItem};
pub use timeline::{Timeline, TimelineItem};
