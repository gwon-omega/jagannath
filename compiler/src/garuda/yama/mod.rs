//! # Yama Dharmaraja Module
//!
//! The judge of code - static analyzer that determines violations
//! and assigns them to appropriate Narakas.

mod dharmaraja;
mod judgment;
mod sentence;
mod yamadutas;

pub use dharmaraja::YamaDharmaraja;
pub use judgment::{Judgment, JudgmentResult};
pub use sentence::Sentence;
pub use yamadutas::{ConcurrencyYamaduta, MemoryYamaduta, SecurityYamaduta, Yamaduta};

use crate::errors::Span;

/// Kind of violation detected
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ViolationKind {
    // Memory violations
    UseAfterFree,
    DoubleFree,
    MemoryLeak,
    BufferOverflow,
    NullDeref,
    MemoryCorruption,
    DanglingPointer,
    StackOverflow,

    // Concurrency violations
    RaceCondition,
    Deadlock,
    ThreadUnsafe,
    Starvation,

    // Security violations
    TaintedData,
    CodeInjection,
    InjectionAttack,
    InsecureStorage,
    DataExposure,
    PoisonedData,
    DoS,
    ResourceDenial,

    // Type violations
    TypeConfusion,
    ContractViolation,
    FfiViolation,
    DataCorruption,

    // Resource violations
    ResourceExhaustion,
    ForcedTermination,
    Panic,

    // Code quality
    CodeSmell,
}

/// A detected violation
#[derive(Debug, Clone)]
pub struct Violation {
    /// What kind of violation
    pub kind: ViolationKind,
    /// Where in source code
    pub location: Span,
    /// Evidence/description
    pub evidence: String,
    /// The sin committed (Garuda Purana description)
    pub sin: String,
    /// The punishment (compiler action)
    pub punishment: String,
    /// The penance (how to fix)
    pub penance: String,
}

impl Violation {
    /// Create a new violation
    pub fn new(kind: ViolationKind, location: Span, evidence: impl Into<String>) -> Self {
        Self {
            kind,
            location,
            evidence: evidence.into(),
            sin: String::new(),
            punishment: String::new(),
            penance: String::new(),
        }
    }

    /// Create a full violation with Garuda Purana fields
    pub fn full(
        kind: ViolationKind,
        location: Span,
        evidence: impl Into<String>,
        sin: impl Into<String>,
        punishment: impl Into<String>,
        penance: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            location,
            evidence: evidence.into(),
            sin: sin.into(),
            punishment: punishment.into(),
            penance: penance.into(),
        }
    }
}
