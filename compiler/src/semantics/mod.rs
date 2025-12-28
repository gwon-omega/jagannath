//! Semantics Module - Semantic Analysis for Jagannath
//!
//! Handles semantic analysis including:
//! - Kāraka role analysis
//! - Type checking
//! - Lifetime/region checking
//! - Borrow checking (linear types)
//! - Information flow analysis
//! - Trait/interface system (guṇa)
//! - Generics & monomorphization (sāmānya-viśeṣa)

pub mod borrow;
pub mod generics;
pub mod karaka;
pub mod lifetime;
pub mod philosophy_integration;
pub mod security;
pub mod traits;
pub mod typeck;

// Re-exports
pub use borrow::BorrowChecker;
pub use generics::{
    ConstraintSolver, GenericContext, GenericFunction, GenericType, MonoError, MonoId,
    Monomorphizer, TypeVarId, TypeVariable, Variance,
};
pub use karaka::KarakaAnalyzer;
pub use lifetime::LifetimeChecker;
pub use security::SecurityAnalyzer;
pub use traits::{ImplId, TraitDef, TraitError, TraitId, TraitImpl, TraitSolver};
pub use typeck::TypeChecker;
