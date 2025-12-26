//! # Yamadutas - Enforcement Agents
//!
//! Specialized linters that detect specific types of violations.

use super::Violation;
use crate::parser::ast::Ast;

/// Yamaduta - Enforcement agent (specialized linter)
pub trait Yamaduta: Send + Sync {
    /// Name of this Yamaduta
    fn name(&self) -> &str;

    /// Inspect code for violations
    fn inspect(&self, ast: &Ast) -> Vec<Violation>;
}

/// Memory Yamaduta - Detects memory violations (Narakas 1-10)
pub struct MemoryYamaduta;

impl MemoryYamaduta {
    pub fn new() -> Self { Self }
}

impl Yamaduta for MemoryYamaduta {
    fn name(&self) -> &str { "MemoryYamaduta" }

    fn inspect(&self, _ast: &Ast) -> Vec<Violation> {
        // TODO: Implement memory violation detection
        Vec::new()
    }
}

impl Default for MemoryYamaduta {
    fn default() -> Self { Self::new() }
}

/// Security Yamaduta - Detects security violations (Narakas 17-23)
pub struct SecurityYamaduta;

impl SecurityYamaduta {
    pub fn new() -> Self { Self }
}

impl Yamaduta for SecurityYamaduta {
    fn name(&self) -> &str { "SecurityYamaduta" }

    fn inspect(&self, _ast: &Ast) -> Vec<Violation> {
        // TODO: Implement security violation detection
        Vec::new()
    }
}

impl Default for SecurityYamaduta {
    fn default() -> Self { Self::new() }
}

/// Concurrency Yamaduta - Detects thread violations (Narakas 6, 11, 16)
pub struct ConcurrencyYamaduta;

impl ConcurrencyYamaduta {
    pub fn new() -> Self { Self }
}

impl Yamaduta for ConcurrencyYamaduta {
    fn name(&self) -> &str { "ConcurrencyYamaduta" }

    fn inspect(&self, _ast: &Ast) -> Vec<Violation> {
        // TODO: Implement concurrency violation detection
        Vec::new()
    }
}

impl Default for ConcurrencyYamaduta {
    fn default() -> Self { Self::new() }
}

