//! Semantics Module - Semantic Analysis for Jagannath
//!
//! Handles semantic analysis including:
//! - Kāraka role analysis
//! - Type checking
//! - Lifetime/region checking
//! - Borrow checking (linear types)
//! - Information flow analysis

pub mod karaka;
pub mod typeck;
pub mod lifetime;
pub mod borrow;
pub mod security;
pub mod philosophy_integration;

// Re-exports
pub use karaka::KarakaAnalyzer;
pub use typeck::TypeChecker;
pub use lifetime::LifetimeChecker;
pub use borrow::BorrowChecker;
pub use security::SecurityAnalyzer;
