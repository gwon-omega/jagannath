//! # Judgment System
//!
//! Determines severity and consequences of violations.

use super::Violation;
use crate::garuda::narakas::{Naraka, Severity};

/// Result of a judgment
#[derive(Debug, Clone)]
pub struct JudgmentResult {
    /// The assigned Naraka
    pub naraka: Naraka,
    /// Severity of the violation
    pub severity: Severity,
    /// Whether this blocks compilation
    pub blocks_compilation: bool,
    /// The sentence (error message)
    pub sentence: String,
}

/// Judgment process
pub struct Judgment;

impl Judgment {
    /// Judge a violation and return the result
    pub fn judge(violation: &Violation, naraka: Naraka) -> JudgmentResult {
        let severity = naraka.severity();
        let blocks_compilation = matches!(severity, Severity::Critical | Severity::Error);

        let sentence = format!(
            "{}: {} at {:?}:{}",
            naraka.name(),
            violation.evidence,
            violation.location.source,
            violation.location.start
        );

        JudgmentResult {
            naraka,
            severity,
            blocks_compilation,
            sentence,
        }
    }

    /// Weigh good karma vs bad karma
    pub fn weigh_karma(good_practices: usize, violations: usize) -> f64 {
        if good_practices + violations == 0 {
            return 1.0; // Perfect karma
        }
        good_practices as f64 / (good_practices + violations) as f64
    }
}

