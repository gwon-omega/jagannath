//! MIR Module - Mid-level Intermediate Representation
//!
//! The MIR is a simplified IR between AST and machine code.
//! Optimizations based on Sāṃkhya tattvas (stages of manifestation)
//! are applied at this level.

pub mod builder;
pub mod nll;
pub mod optimizer;
pub mod passes;
pub mod types;

// Re-exports
pub use builder::MirBuilder;
pub use nll::{compute_liveness, LivenessInfo, NllChecker};
pub use optimizer::MirOptimizer;
pub use types::{MirBasicBlock, MirFunction, MirInstruction, MirType};
