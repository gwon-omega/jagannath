//! # Yama Dharmaraja Module
//!
//! The judge of code - static analyzer that determines violations
//! and assigns them to appropriate Narakas.

mod dharmaraja;
mod judgment;
mod yamadutas;
mod sentence;

pub use dharmaraja::YamaDharmaraja;
pub use judgment::{Judgment, JudgmentResult};
pub use yamadutas::{Yamaduta, MemoryYamaduta, SecurityYamaduta, ConcurrencyYamaduta};
pub use sentence::Sentence;

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
}

impl Violation {
    /// Create a new violation
    pub fn new(kind: ViolationKind, location: Span, evidence: impl Into<String>) -> Self {
        Self {
            kind,
            location,
            evidence: evidence.into(),
        }
    }
}
