//! # Rta - Cosmic Order
//!
//! Rta (ऋत) is the cosmic order that governs the universe.
//! In compiler terms: The coordination system that ensures all
//! 33 devatas work in harmony.

use super::CompilationState;

/// The cosmic order
pub struct Rta {
    /// Whether strict ordering is enforced
    strict: bool,
    /// Harmony violations detected
    violations: Vec<String>,
}

impl Rta {
    pub fn new() -> Self {
        Self {
            strict: true,
            violations: Vec::new(),
        }
    }

    /// Enable relaxed ordering (for performance)
    pub fn relax(mut self) -> Self {
        self.strict = false;
        self
    }

    /// Verify that the compilation state is in cosmic harmony
    pub fn verify_harmony(&self, state: &CompilationState) -> bool {
        // Check for violations
        if state.has_errors() {
            return false;
        }

        // Check phase ordering
        if self.strict {
            // In strict mode, all phases must have run
            if state.current_phase < 11 {
                return false;
            }
        }

        true
    }

    /// Record a harmony violation
    pub fn record_violation(&mut self, violation: &str) {
        self.violations.push(violation.to_string());
    }

    /// Get all violations
    pub fn violations(&self) -> &[String] {
        &self.violations
    }

    /// Clear violations
    pub fn clear_violations(&mut self) {
        self.violations.clear();
    }
}

impl Default for Rta {
    fn default() -> Self {
        Self::new()
    }
}

/// Cosmic order types
#[derive(Debug, Clone, Copy)]
pub enum CosmicOrder {
    /// Natural order (Rta)
    Rta,
    /// Human order (Dharma)
    Dharma,
    /// Chaotic/inverted (Anrta)
    Anrta,
}

impl CosmicOrder {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Rta => "Ṛta",
            Self::Dharma => "Dharma",
            Self::Anrta => "Anṛta",
        }
    }

    pub fn meaning(&self) -> &'static str {
        match self {
            Self::Rta => "Cosmic Order",
            Self::Dharma => "Righteous Order",
            Self::Anrta => "Chaos/Disorder",
        }
    }
}
