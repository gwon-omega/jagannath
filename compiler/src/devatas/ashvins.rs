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
use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};
use tracing::{info, warn};

/// Names of the 2 Ashvins
pub const TWO_ASHVINS: [&str; 2] = [
    "Dasra",   // Error diagnosis
    "Nāsatya", // Error recovery
];

/// An Ashvin (healer deity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ashvin {
    Dasra,   // Skillful - Diagnosis
    Nasatya, // Truthful - Recovery
}

impl Ashvin {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Dasra => "Dasra",
            Self::Nasatya => "Nāsatya",
        }
    }

    pub fn role(&self) -> &'static str {
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

    /// Get all 2 Ashvins in order
    pub fn all() -> &'static [Self] {
        &[Self::Dasra, Self::Nasatya]
    }

    /// Get index (0-based)
    pub fn order(&self) -> usize {
        match self {
            Self::Dasra => 0,
            Self::Nasatya => 1,
        }
    }

    /// Get Sanskrit name (Devanagari)
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Dasra => "दस्र",
            Self::Nasatya => "नासत्य",
        }
    }

    /// Get English meaning
    pub fn english(&self) -> &'static str {
        match self {
            Self::Dasra => "The Skillful One",
            Self::Nasatya => "The Truthful One",
        }
    }
}

// ============================================================================
// v10.0 Trait Implementations
// ============================================================================

impl SanskritNamed for Ashvin {
    fn sanskrit(&self) -> &'static str {
        self.sanskrit_name()
    }

    fn iast(&self) -> &'static str {
        self.name()
    }

    fn english(&self) -> &'static str {
        self.english()
    }
}

impl SanskritDescribed for Ashvin {
    fn meaning(&self) -> &'static str {
        self.function()
    }

    fn explanation(&self) -> &'static str {
        match self {
            Self::Dasra => "First Ashvin: The skillful physician who diagnoses ailments - performs error diagnosis, finds root causes of compilation failures",
            Self::Nasatya => "Second Ashvin: The truthful healer who provides cure - performs error recovery, suggests fixes and auto-corrections",
        }
    }

    fn mantra(&self) -> Option<&'static str> {
        Some(match self {
            Self::Dasra => "ॐ दस्राय नमः (Oṃ Dasrāya Namaḥ)",
            Self::Nasatya => "ॐ नासत्याय नमः (Oṃ Nāsatyāya Namaḥ)",
        })
    }

    fn category(&self) -> &'static str {
        "Ashvins - Divine Physicians (अश्विनौ)"
    }
}

impl PhilosophicalEnum for Ashvin {
    fn all() -> &'static [Self] {
        Ashvin::all()
    }

    fn count() -> usize {
        2
    }

    fn index(&self) -> usize {
        self.order()
    }

    fn ordinal(&self) -> usize {
        self.order() + 1
    }

    fn next(&self) -> Self {
        match self {
            Self::Dasra => Self::Nasatya,
            Self::Nasatya => Self::Dasra, // Twin cycle
        }
    }

    fn prev(&self) -> Self {
        match self {
            Self::Dasra => Self::Nasatya, // Twin cycle
            Self::Nasatya => Self::Dasra,
        }
    }

    fn from_index(index: usize) -> Option<Self> {
        Self::all().get(index).copied()
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

        info!(
            "Ashvin {} attempting healing: {}",
            self.ashvin.name(),
            self.ashvin.function()
        );

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

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};

    #[test]
    fn test_ashvin_count() {
        assert_eq!(Ashvin::count(), 2);
        assert_eq!(Ashvin::all().len(), 2);
    }

    #[test]
    fn test_ashvin_sanskrit_named() {
        let ashvin = Ashvin::Dasra;
        assert_eq!(ashvin.sanskrit(), "दस्र");
        assert_eq!(ashvin.iast(), "Dasra");
        assert_eq!(ashvin.english(), "The Skillful One");
    }

    #[test]
    fn test_ashvin_sanskrit_described() {
        let ashvin = Ashvin::Nasatya;
        assert_eq!(ashvin.meaning(), "Error Recovery");
        assert!(ashvin.explanation().contains("truthful"));
        assert!(ashvin.mantra().is_some());
        assert_eq!(ashvin.category(), "Ashvins - Divine Physicians (अश्विनौ)");
    }

    #[test]
    fn test_ashvin_twin_navigation() {
        // The Ashvins are twins - they cycle between each other
        assert_eq!(Ashvin::Dasra.next(), Ashvin::Nasatya);
        assert_eq!(Ashvin::Nasatya.next(), Ashvin::Dasra);
        assert_eq!(Ashvin::Dasra.prev(), Ashvin::Nasatya);
        assert_eq!(Ashvin::Nasatya.prev(), Ashvin::Dasra);
    }

    #[test]
    fn test_ashvin_from_index() {
        assert_eq!(Ashvin::from_index(0), Some(Ashvin::Dasra));
        assert_eq!(Ashvin::from_index(1), Some(Ashvin::Nasatya));
        assert_eq!(Ashvin::from_index(2), None);
    }

    #[test]
    fn test_ashvin_ordinal_sequence() {
        for (i, ashvin) in Ashvin::all().iter().enumerate() {
            assert_eq!(
                ashvin.ordinal(),
                i + 1,
                "Ashvin {:?} ordinal mismatch",
                ashvin
            );
            assert_eq!(ashvin.index(), i, "Ashvin {:?} index mismatch", ashvin);
        }
    }

    #[test]
    fn test_ashvin_functions() {
        assert_eq!(Ashvin::Dasra.function(), "Error Diagnosis");
        assert_eq!(Ashvin::Nasatya.function(), "Error Recovery");
    }
}
