//! Visual effects and animations
//!
//! Aceternity-inspired visual effects including aurora, vortex, sparkles,
//! meteors, animated beams, and other atmospheric effects.

pub mod animated_beam;
pub mod aurora;
pub mod background_beams;
pub mod lamp;
pub mod meteor;
pub mod moving_border;
pub mod sparkles;
pub mod spotlight;
pub mod vortex;
pub mod wavy_background;

pub use animated_beam::{AnimatedBeam, AnimatedBeams, BeamLoopMode, PathPoint};
pub use aurora::AuroraBackground;
pub use background_beams::BackgroundBeams;
pub use lamp::LampEffect;
pub use meteor::MeteorShower;
pub use moving_border::MovingBorder;
pub use sparkles::Sparkles;
pub use spotlight::{MultiSpotlight, Spotlight};
pub use vortex::VortexBackground;
pub use wavy_background::WavyBackground;
