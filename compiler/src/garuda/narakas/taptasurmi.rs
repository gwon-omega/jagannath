//! # Taptasurmi - Hell 25: Hot Oil
//!
//! Sin: Causing burns
//! Code: CPU-intensive tight loops

use super::super::yama::Violation;
use crate::parser::ast::Ast;

/// Checker for Taptasurmi violations (CPU burn)
pub struct TaptasurmiChecker;

impl TaptasurmiChecker {
    pub fn new() -> Self {
        Self
    }

    /// Check for CPU-intensive patterns
    pub fn check(&self, _ast: &Ast) -> Vec<Violation> {
        // TODO: Implement CPU burn detection
        Vec::new()
    }
}

impl Default for TaptasurmiChecker {
    fn default() -> Self {
        Self::new()
    }
}
