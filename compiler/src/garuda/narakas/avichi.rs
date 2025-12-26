//! # Avichi - Hell 20: Stack Overflow
//!
//! Sin: False witness, perjury
//! Code: Stack overflow

use crate::parser::ast::Ast;
use super::super::yama::Violation;

/// Checker for Avichi violations (stack overflow)
pub struct AvichiChecker {
    pub max_recursion_depth: usize,
}

impl AvichiChecker {
    pub fn new() -> Self {
        Self { max_recursion_depth: 1000 }
    }

    pub fn check(&self, _ast: &Ast) -> Vec<Violation> {
        // TODO: Implement recursion and stack analysis
        Vec::new()
    }
}

impl Default for AvichiChecker {
    fn default() -> Self { Self::new() }
}
