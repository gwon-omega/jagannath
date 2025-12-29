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

// Type checking - split into submodules for better organization
// The main TypeChecker implementation is in typeck_impl.rs (legacy, to be migrated)
// New modular components are in typeck/ directory
#[path = "typeck_impl.rs"]
mod typeck_impl;
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
// Re-export TypeChecker from the impl module
pub use typeck_impl::TypeChecker;
