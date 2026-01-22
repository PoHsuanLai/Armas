//! Card components
//!
//! Various card variations with different visual effects.

pub mod card;
pub mod card_spotlight;
pub mod card_stack;
pub mod floating_window;
pub mod focus_cards;
pub mod glass_panel;
pub mod gradient_card;
pub mod hover_card;
pub mod infinite_moving_cards;
pub mod testimonial;
pub mod text_reveal_card;
pub mod tilt_card;
pub mod wobble_card;

// Re-exports
pub use card::{Card, CardResponse, CardVariant};
pub use card_spotlight::CardSpotlight;
pub use card_stack::CardStack;
pub use floating_window::{FloatingWindow, FloatingWindowResponse, FloatingWindowStyle};
pub use focus_cards::{FocusCardResponse, FocusCards};
pub use glass_panel::{GlassPanel, GlassPanelResponse};
pub use gradient_card::GradientCard;
pub use hover_card::HoverCard;
pub use infinite_moving_cards::{InfiniteMovingCards, ScrollSpeed};
pub use testimonial::{TestimonialCard, TestimonialGrid};
pub use text_reveal_card::TextRevealCard;
pub use tilt_card::TiltCard;
pub use wobble_card::WobbleCard;
