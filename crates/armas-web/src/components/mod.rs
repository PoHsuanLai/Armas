//! Showcase-specific components for armas-web

pub mod code_display;
pub mod demo_card;
pub mod header;
pub mod hero_section;
pub mod section_header;
pub mod variant_card;

pub use code_display::{code_inline, CodeDisplayCard};
pub use demo_card::ComponentDemoCard;
pub use header::Header;
pub use hero_section::{FeatureShowcase, FeatureShowcaseItem, HeroSection};
pub use section_header::{QuickInstall, RelatedComponentCard, ShowcaseSectionHeader};
pub use variant_card::{grid_row, variant_grid, VariantCard};
