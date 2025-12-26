//! # Ashvins - 2 Healing Deities
//!
//! The 2 Ashvins are divine physicians/healers.
//! In compiler terms: 2 diagnostic and recovery tools.
//!
//! ## The 2 Ashvins:
//!
//! | Ashvin | Role | Tool Function |
//! |--------|------|---------------|
//! | Dasra | Skillful | Error diagnosis |
//! | Nasatya | Truthful | Error recovery |

use super::CompilationState;
use tracing::{info, warn};

/// Names of the 2 Ashvins
pub const TWO_ASHVINS: [&str; 2] = [
    "Dasra",    // Error diagnosis
    "Nāsatya",  // Error recovery
];

/// An Ashvin (healer deity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ashvin {
    Dasra,    // Skillful - Diagnosis
    Nasatya,  // Truthful - Recovery
}

impl Ashvin {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Dasra => "Dasra",
            Self::Nasatya => "Nāsatya",
        }
    }

    pub fn meaning(&self) -> &'static str {
        match self {
            Self::Dasra => "Skillful",
            Self::Nasatya => "Truthful",
        }
    }

    pub fn function(&self) -> &'static str {
        match self {
            Self::Dasra => "Error Diagnosis",
            Self::Nasatya => "Error Recovery",
        }
    }
}

/// A diagnostic/recovery tool represented by an Ashvin
pub struct AshvinTool {
    /// Which Ashvin
    pub ashvin: Ashvin,
    /// Whether this tool is enabled
    pub enabled: bool,
    /// Healing attempts made
    pub healing_count: usize,
}

impl AshvinTool {
    pub fn new(ashvin: Ashvin) -> Self {
        Self {
            ashvin,
            enabled: true,
            healing_count: 0,
        }
    }

    /// Check if this Ashvin can heal the current state
    pub fn can_heal(&self, state: &CompilationState) -> bool {
        if !self.enabled {
            return false;
        }

        match self.ashvin {
            Ashvin::Dasra => {
                // Dasra can diagnose any errors
                state.has_errors()
            }
            Ashvin::Nasatya => {
                // Nasatya can attempt recovery if errors exist
                state.has_errors() && state.errors.len() <= 10
            }
        }
    }

    /// Attempt to heal/diagnose the compilation state
    pub fn heal(&mut self, mut state: CompilationState) -> CompilationState {
        if !self.can_heal(&state) {
            return state;
        }

        info!("Ashvin {} attempting healing: {}",
                  self.ashvin.name(), self.ashvin.function());

        self.healing_count += 1;

        match self.ashvin {
            Ashvin::Dasra => {
                // Dasra provides detailed diagnosis
                for error in &state.errors {
                    warn!("Dasra diagnosis: {}", error);
                }
            }
            Ashvin::Nasatya => {
                // Nasatya attempts recovery
                // In real impl, would try to fix recoverable errors
                state.errors.retain(|e| !e.contains("recoverable"));
            }
        }

        state
    }

    /// Get healing statistics
    pub fn stats(&self) -> (usize, bool) {
        (self.healing_count, self.enabled)
    }
}

/// Create both Ashvin tools
pub fn create_all() -> [AshvinTool; 2] {
    [
        AshvinTool::new(Ashvin::Dasra),
        AshvinTool::new(Ashvin::Nasatya),
    ]
}
