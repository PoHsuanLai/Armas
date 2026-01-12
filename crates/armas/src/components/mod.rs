//! Reusable UI components for egui
//!
//! This module contains general-purpose UI components with a clean,
//! professional design language.

pub mod accordion;
pub mod alert;
pub mod aurora;
pub mod avatar;
pub mod badge;
pub mod breadcrumbs;
pub mod button;
pub mod card;
pub mod card_stack;
pub mod command_menu;
pub mod date_picker;
pub mod dot_pattern;
pub mod drawer;
pub mod fader;
pub mod flip_words;
pub mod floating_navbar;
pub mod focus_cards;
pub mod glass_panel;
pub mod glowing_border;
pub mod gradient_card;
pub mod gradient_text;
pub mod grid_pattern;
pub mod hover_card;
pub mod infinite_moving_cards;
pub mod input;
pub mod loading;
pub mod menu;
pub mod meteor;
pub mod modal;
pub mod moving_border;
pub mod pagination;
pub mod popover;
pub mod progress;
pub mod retro_grid;
pub mod routing_button;
pub mod scramble_text;
pub mod scrolling_banner;
pub mod section_header;
pub mod select;
pub mod sidebar;
pub mod slider;
pub mod slot;
pub mod sparkles;
pub mod spotlight;
pub mod stepper;
pub mod tabs;
pub mod testimonial;
pub mod text_reveal_card;
pub mod textarea;
pub mod tilt_card;
pub mod timeline;
pub mod toast;
pub mod toggle;
pub mod tooltip;
pub mod typewriter;
pub mod vortex;
pub mod wobble_card;

// Re-exports
pub use accordion::{Accordion, AccordionItem};
pub use alert::{
    alert, alert_error, alert_info, alert_success, alert_warning, Alert, AlertResponse,
    AlertVariant,
};
pub use aurora::AuroraBackground;
pub use avatar::{Avatar, AvatarShape, AvatarSize};
pub use badge::{Badge, BadgeColor, BadgeVariant, NotificationBadge};
pub use breadcrumbs::{BreadcrumbItem, Breadcrumbs, BreadcrumbsResponse};
pub use button::{
    BrutalButton, Button, ButtonVariant, FigmaButton, InvertButton, ShimmerButton, SimpleButton,
    SketchButton, SpotifyButton,
};
pub use card::{Card, CardResponse};
pub use card_stack::{CardStack, StackCard};
pub use command_menu::{Command, CommandMenu, CommandMenuResponse};
pub use date_picker::{Date, DatePicker, DatePickerResponse};
pub use dot_pattern::DotPattern;
pub use drawer::{Drawer, DrawerPosition, DrawerResponse, DrawerSize};
pub use fader::{Fader, FaderStrip};
pub use flip_words::{FlipStyle, FlipWords};
pub use floating_navbar::{FloatingNavbar, NavItem, NavbarPosition, NavbarResponse};
pub use focus_cards::{FocusCard, FocusCardResponse, FocusCards};
pub use glass_panel::{GlassPanel, GlassPanelResponse};
pub use glowing_border::GlowingBorder;
pub use gradient_card::GradientCard;
pub use gradient_text::GradientText;
pub use grid_pattern::GridPattern;
pub use hover_card::HoverCard;
pub use infinite_moving_cards::{InfiniteMovingCards, MovingCard, ScrollSpeed};
pub use input::{Input, InputState, InputVariant, SearchInput};
pub use loading::{CircularProgress, LoadingDots, Skeleton, Spinner};
pub use menu::{Menu, MenuItem, MenuResponse};
pub use meteor::MeteorShower;
pub use modal::{confirm_dialog, dialog, ConfirmResponse, Modal, ModalResponse, ModalSize};
pub use moving_border::MovingBorder;
pub use pagination::{Pagination, PaginationResponse};
pub use popover::{Popover, PopoverColor, PopoverPosition, PopoverResponse, PopoverStyle};
pub use progress::{CircularProgressBar, LinearProgress, RingProgress};
pub use retro_grid::RetroGrid;
pub use routing_button::RoutingButton;
pub use scramble_text::ScrambleText;
pub use scrolling_banner::{ScrollDirection, ScrollingBanner};
pub use section_header::SectionHeader;
pub use select::{Select, SelectOption, SelectResponse};
pub use sidebar::{Sidebar, SidebarItem, SidebarResponse};
pub use slider::{Slider, SliderResponse};
pub use slot::Slot;
pub use sparkles::Sparkles;
pub use spotlight::{MultiSpotlight, Spotlight};
pub use stepper::{Step, Stepper, StepperOrientation, StepperResponse};
pub use tabs::{AnimatedTabs, TabStyle};
pub use testimonial::{TestimonialCard, TestimonialGrid, TestimonialItem};
pub use text_reveal_card::TextRevealCard;
pub use textarea::Textarea;
pub use tilt_card::TiltCard;
pub use timeline::{Timeline, TimelineItem};
pub use toast::{ToastManager, ToastPosition, ToastVariant};
pub use toggle::{
    Toggle, ToggleGroup, ToggleGroupResponse, ToggleResponse, ToggleSize, ToggleVariant,
};
pub use tooltip::{tooltip, tooltip_with, Tooltip, TooltipPosition};
pub use typewriter::{Typewriter, WordTypewriter};
pub use vortex::VortexBackground;
pub use wobble_card::WobbleCard;
