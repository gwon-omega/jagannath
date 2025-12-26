//! # Vaitarani - Security Boundary Module
//!
//! The filthy river that must be crossed to enter trusted realm.
//! Enforces sanitization when crossing from untrusted to trusted.

mod boundary;
mod purification;
mod crossing;

pub use boundary::VaitaraniBoundary;
pub use purification::Purifier;
pub use crossing::CrossingCheck;

use crate::errors::Span;

/// Level of taint on data
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaintLevel {
    /// Fully trusted (crossed Vaitarani successfully)
    Trusted,
    /// Partially trusted (some validation applied)
    PartiallyTrusted,
    /// Untrusted (from external source)
    Untrusted,
    /// Poisoned (known malicious)
    Poisoned,
}

/// A Vaitarani crossing violation
#[derive(Debug, Clone)]
pub struct VaitaraniViolation {
    /// Location in source
    pub location: Span,
    /// Description
    pub message: String,
    /// What purifier is needed
    pub required_purifier: Option<String>,
    /// Source of tainted data
    pub taint_source: String,
}
