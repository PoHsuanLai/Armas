//! Card components
//!
//! Various card variations with different visual effects.

pub mod card;
pub mod card_stack;
pub mod infinite_moving_cards;
/// Testimonial cards and grids
pub mod testimonial;
pub mod text_reveal_card;
pub mod tilt_card;
pub mod wobble_card;

// Re-exports
pub use card::{Card, CardResponse, CardVariant};
pub use card_stack::CardStack;
pub use infinite_moving_cards::{InfiniteMovingCards, ScrollSpeed};
pub use testimonial::{TestimonialCard, TestimonialGrid};
pub use text_reveal_card::TextRevealCard;
pub use tilt_card::TiltCard;
pub use wobble_card::WobbleCard;
