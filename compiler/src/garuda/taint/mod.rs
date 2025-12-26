//! # Taint Analysis Module
//!
//! Tracks sin/taint propagation through data flow.
//! Like pāpa (sin) spreading through contact.

mod analyzer;
mod source;
mod propagation;

pub use analyzer::TaintAnalyzer;
pub use source::TaintSource;
pub use propagation::TaintPropagation;

use crate::errors::Span;
use crate::garuda::vaitarani::TaintLevel;

/// A taint marking on data
#[derive(Debug, Clone)]
pub struct Taint {
    /// Where the taint originated
    pub source: TaintSource,
    /// Current taint level
    pub level: TaintLevel,
    /// What kind of taint
    pub kind: TaintKind,
    /// Path of propagation
    pub propagation_path: Vec<Span>,
}

/// Kinds of taint (types of sin)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaintKind {
    /// User input (potentially malicious)
    UserInput,
    /// SQL injection risk
    SqlInjection,
    /// XSS risk
    CrossSiteScripting,
    /// Command injection risk
    CommandInjection,
    /// Path traversal risk
    PathTraversal,
    /// Sensitive data (should not leak)
    SensitiveData,
    /// Unchecked data
    Unchecked,
}

impl TaintKind {
    /// Get Sanskrit name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            TaintKind::UserInput => "bāhya-āgama",        // external arrival
            TaintKind::SqlInjection => "sāraṇī-viṣa",    // table poison
            TaintKind::CrossSiteScripting => "kūṭa-lipi", // deceptive script
            TaintKind::CommandInjection => "ājñā-viṣa",  // command poison
            TaintKind::PathTraversal => "mārga-bhrama",   // path wandering
            TaintKind::SensitiveData => "guhya-datta",    // secret data
            TaintKind::Unchecked => "aparīkṣita",         // unexamined
        }
    }
}
