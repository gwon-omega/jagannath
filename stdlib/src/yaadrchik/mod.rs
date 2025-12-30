//! # Yaadrchik - Randomness (यादृच्छिक)
//!
//! Random number generation.

pub mod janaka;     // Generators (जनक)
pub mod vitaran;    // Distributions (वितरण)

// Re-exports
pub use janaka::*;
pub use vitaran::*;
