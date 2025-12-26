//! MIR Module - Mid-level Intermediate Representation
//!
//! The MIR is a simplified IR between AST and machine code.
//! Optimizations based on Sāṃkhya tattvas (stages of manifestation)
//! are applied at this level.

pub mod types;
pub mod builder;
pub mod optimizer;
pub mod passes;

// Re-exports
pub use types::{MirFunction, MirBasicBlock, MirInstruction, MirType};
pub use builder::MirBuilder;
pub use optimizer::MirOptimizer;
