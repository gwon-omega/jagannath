//! # Doṣa - Unified Error Handling (दोष)
//!
//! Unified error handling trait system following Āyurveda philosophy.
//!
//! > **"दोषः मूलं रोगाणाम्"**
//! > *"Doṣa (imbalance) is the root of diseases"*
//! > — Charaka Saṃhitā
//!
//! In Āyurveda, doṣas (वात, पित्त, कफ - Vāta, Pitta, Kapha) are the fundamental
//! imbalances that cause disease. Similarly, in software, errors are imbalances
//! that disrupt correct operation.
//!
//! ## Traits
//! - [`Dosha`] - Core error trait (like std::error::Error)
//! - [`DoshaVarga`] - Error categorization (classification)
//! - [`Chikitsa`] - Error recovery/healing
//!
//! ## Usage
//! ```rust,ignore
//! use jagannath_stdlib::dosha::{Dosha, DoshaVarga, Chikitsa};
//!
//! impl Dosha for MyError {
//!     fn sandesh(&self) -> &str { "My error message" }
//!     fn varga(&self) -> DoshaVarga { DoshaVarga::Sāmānya }
//! }
//! ```

#![allow(dead_code)]

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use core::fmt;

// ============================================================================
// DOSHA CATEGORIES (Based on Ayurvedic Tridosha + Extended)
// ============================================================================

/// Error categories based on Āyurvedic doṣas (दोष वर्ग)
///
/// Just as Āyurveda classifies diseases by dominant doṣa,
/// we classify errors by their fundamental nature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DoshaVarga {
    // Primary Tridoṣas (त्रिदोष)
    /// वात (Vāta) - Movement errors: flow, control, communication
    /// Caused by: interrupted flow, disconnection, timeout
    Vata,

    /// पित्त (Pitta) - Transformation errors: conversion, processing
    /// Caused by: invalid data, format mismatch, corruption
    Pitta,

    /// कफ (Kapha) - Structure errors: storage, resources, memory
    /// Caused by: out of memory, full buffer, resource exhaustion
    Kapha,

    // Extended categories
    /// सामान्य (Sāmānya) - General/unclassified errors
    Samanya,

    /// प्राणिक (Prāṇika) - Critical/fatal errors
    /// Life-threatening to the program
    Pranika,

    /// मानसिक (Mānasika) - Logic/reasoning errors
    /// Mental imbalance: invalid state, logic errors
    Manasika,

    /// बाह्य (Bāhya) - External/environmental errors
    /// From outside: I/O, network, filesystem
    Bahya,

    /// अभ्यन्तर (Ābhyantara) - Internal/system errors
    /// From within: assertions, invariants, panics
    Abhyantara,
}

impl DoshaVarga {
    /// Sanskrit name (Devanagari)
    pub const fn sanskrit(&self) -> &'static str {
        match self {
            Self::Vata => "वात",
            Self::Pitta => "पित्त",
            Self::Kapha => "कफ",
            Self::Samanya => "सामान्य",
            Self::Pranika => "प्राणिक",
            Self::Manasika => "मानसिक",
            Self::Bahya => "बाह्य",
            Self::Abhyantara => "आभ्यन्तर",
        }
    }

    /// IAST transliteration
    pub const fn iast(&self) -> &'static str {
        match self {
            Self::Vata => "vāta",
            Self::Pitta => "pitta",
            Self::Kapha => "kapha",
            Self::Samanya => "sāmānya",
            Self::Pranika => "prāṇika",
            Self::Manasika => "mānasika",
            Self::Bahya => "bāhya",
            Self::Abhyantara => "ābhyantara",
        }
    }

    /// English meaning
    pub const fn english(&self) -> &'static str {
        match self {
            Self::Vata => "Movement/Flow",
            Self::Pitta => "Transformation",
            Self::Kapha => "Structure/Storage",
            Self::Samanya => "General",
            Self::Pranika => "Critical/Fatal",
            Self::Manasika => "Logic/Reasoning",
            Self::Bahya => "External",
            Self::Abhyantara => "Internal",
        }
    }

    /// Is this a primary Tridoṣa?
    pub const fn is_tridosha(&self) -> bool {
        matches!(self, Self::Vata | Self::Pitta | Self::Kapha)
    }

    /// Severity level (1-5)
    pub const fn tivrata(&self) -> u8 {
        match self {
            Self::Pranika => 5,    // Critical
            Self::Abhyantara => 4, // System
            Self::Vata => 3,       // Flow
            Self::Pitta => 3,      // Transform
            Self::Kapha => 3,      // Structure
            Self::Bahya => 2,      // External
            Self::Manasika => 2,   // Logic
            Self::Samanya => 1,    // General
        }
    }
}

impl fmt::Display for DoshaVarga {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.sanskrit(), self.english())
    }
}

// ============================================================================
// CORE DOSHA TRAIT
// ============================================================================

/// Core error trait - every error type should implement this (दोष)
///
/// Like `std::error::Error` but with Sanskrit naming and philosophy.
///
/// ## Required Methods
/// - `sandesh` - Error message (संदेश)
/// - `varga` - Error category (वर्ग)
///
/// ## Optional Methods
/// - `mula` - Root cause (मूल)
/// - `karan` - Source/cause (कारण)
/// - `sthana` - Location (स्थान)
pub trait Dosha: fmt::Debug + fmt::Display {
    /// Error message (संदेश - sandēśa)
    fn sandesh(&self) -> &str;

    /// Error category (वर्ग - varga)
    fn varga(&self) -> DoshaVarga {
        DoshaVarga::Samanya
    }

    /// Root cause error, if any (मूल - mūla)
    #[cfg(feature = "alloc")]
    fn mula(&self) -> Option<&(dyn Dosha + 'static)> {
        None
    }

    /// Source location info (स्थान - sthāna)
    fn sthana(&self) -> Option<&str> {
        None
    }

    /// Severity level 1-5 (तीव्रता - tīvratā)
    fn tivrata(&self) -> u8 {
        self.varga().tivrata()
    }

    /// Is this error recoverable? (पुनर्प्राप्य - punarprāpya)
    fn punarprapya(&self) -> bool {
        self.tivrata() < 4
    }

    /// Error code if applicable (संकेत - saṅkēta)
    fn sanketa(&self) -> Option<u32> {
        None
    }
}

// ============================================================================
// CHIKITSA - ERROR RECOVERY TRAIT
// ============================================================================

/// Error recovery/healing trait (चिकित्सा - treatment)
///
/// Just as Āyurveda provides remedies for doṣa imbalances,
/// this trait provides error recovery strategies.
#[cfg(feature = "alloc")]
pub trait Chikitsa: Dosha {
    /// Suggested fix/remedy (औषधि - auṣadhi = medicine)
    fn aushadhi(&self) -> Option<String> {
        None
    }

    /// Alternative actions that might work (विकल्प - vikalpa)
    fn vikalpa(&self) -> Vec<String> {
        Vec::new()
    }

    /// Can this error be retried? (पुनःप्रयास - punaḥprayāsa)
    fn punah_prayas(&self) -> bool {
        self.punarprapya()
    }

    /// Retry delay in milliseconds, if retryable (विलम्ब - vilamba)
    fn vilamba(&self) -> Option<u64> {
        if self.punah_prayas() {
            Some(100) // Default 100ms retry delay
        } else {
            None
        }
    }
}

// ============================================================================
// DOSHA CHAIN - ERROR CHAIN
// ============================================================================

/// Chain of errors (दोष शृंखला - error chain)
#[cfg(feature = "alloc")]
pub struct DoshaSrinkhala {
    errors: Vec<Box<dyn Dosha>>,
}

#[cfg(feature = "alloc")]
impl DoshaSrinkhala {
    /// Create new error chain
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Add error to chain
    pub fn jod<D: Dosha + 'static>(&mut self, dosha: D) {
        self.errors.push(Box::new(dosha));
    }

    /// Get root cause (first error)
    pub fn mula(&self) -> Option<&dyn Dosha> {
        self.errors.first().map(|b| b.as_ref())
    }

    /// Get most recent error (last)
    pub fn antima(&self) -> Option<&dyn Dosha> {
        self.errors.last().map(|b| b.as_ref())
    }

    /// Iterate through chain
    pub fn iter(&self) -> impl Iterator<Item = &dyn Dosha> {
        self.errors.iter().map(|b| b.as_ref())
    }

    /// Chain length
    pub fn lambai(&self) -> usize {
        self.errors.len()
    }

    /// Is chain empty?
    pub fn khali(&self) -> bool {
        self.errors.is_empty()
    }
}

#[cfg(feature = "alloc")]
impl Default for DoshaSrinkhala {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// SIMPLE DOSHA IMPLEMENTATION
// ============================================================================

/// Simple error type for quick error creation (सरल दोष)
#[cfg(feature = "alloc")]
pub struct SaralDosha {
    sandesh: String,
    varga: DoshaVarga,
    sanketa: Option<u32>,
}

#[cfg(feature = "alloc")]
impl SaralDosha {
    /// Create new simple error
    pub fn new<S: Into<String>>(sandesh: S, varga: DoshaVarga) -> Self {
        Self {
            sandesh: sandesh.into(),
            varga,
            sanketa: None,
        }
    }

    /// Create with error code
    pub fn with_sanketa<S: Into<String>>(sandesh: S, varga: DoshaVarga, sanketa: u32) -> Self {
        Self {
            sandesh: sandesh.into(),
            varga,
            sanketa: Some(sanketa),
        }
    }

    /// Create Vāta (flow) error
    pub fn vata<S: Into<String>>(sandesh: S) -> Self {
        Self::new(sandesh, DoshaVarga::Vata)
    }

    /// Create Pitta (transform) error
    pub fn pitta<S: Into<String>>(sandesh: S) -> Self {
        Self::new(sandesh, DoshaVarga::Pitta)
    }

    /// Create Kapha (structure) error
    pub fn kapha<S: Into<String>>(sandesh: S) -> Self {
        Self::new(sandesh, DoshaVarga::Kapha)
    }

    /// Create critical error
    pub fn pranika<S: Into<String>>(sandesh: S) -> Self {
        Self::new(sandesh, DoshaVarga::Pranika)
    }
}

#[cfg(feature = "alloc")]
impl fmt::Debug for SaralDosha {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SaralDosha")
            .field("sandesh", &self.sandesh)
            .field("varga", &self.varga)
            .field("sanketa", &self.sanketa)
            .finish()
    }
}

#[cfg(feature = "alloc")]
impl fmt::Display for SaralDosha {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(code) = self.sanketa {
            write!(f, "[{}] {}: {}", code, self.varga, self.sandesh)
        } else {
            write!(f, "{}: {}", self.varga, self.sandesh)
        }
    }
}

#[cfg(feature = "alloc")]
impl Dosha for SaralDosha {
    fn sandesh(&self) -> &str {
        &self.sandesh
    }

    fn varga(&self) -> DoshaVarga {
        self.varga
    }

    fn sanketa(&self) -> Option<u32> {
        self.sanketa
    }
}

// ============================================================================
// RESULT TYPE ALIAS
// ============================================================================

/// Result type using Dosha error (परिणाम - result)
#[cfg(feature = "alloc")]
pub type Parinam<T> = core::result::Result<T, Box<dyn Dosha>>;

/// Result with specific error type
pub type DoshaParinam<T, E> = core::result::Result<T, E>;

// ============================================================================
// INTEGRATION WITH STD::ERROR
// ============================================================================

// Note: We cannot implement std::error::Error for all Dosha types due to
// Rust's orphan rules. Individual Dosha implementors should implement
// std::error::Error themselves if needed.
//
// Example:
// ```rust
// impl std::error::Error for MyDosha {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         None
//     }
// }
// ```

#[cfg(all(feature = "std", feature = "alloc"))]
impl std::error::Error for SaralDosha {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dosha_varga() {
        assert_eq!(DoshaVarga::Vata.sanskrit(), "वात");
        assert_eq!(DoshaVarga::Pitta.iast(), "pitta");
        assert_eq!(DoshaVarga::Kapha.english(), "Structure/Storage");
    }

    #[test]
    fn test_tridosha() {
        assert!(DoshaVarga::Vata.is_tridosha());
        assert!(DoshaVarga::Pitta.is_tridosha());
        assert!(DoshaVarga::Kapha.is_tridosha());
        assert!(!DoshaVarga::Samanya.is_tridosha());
    }

    #[test]
    fn test_tivrata() {
        assert_eq!(DoshaVarga::Pranika.tivrata(), 5);
        assert_eq!(DoshaVarga::Samanya.tivrata(), 1);
        assert!(DoshaVarga::Vata.tivrata() >= 2);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_saral_dosha() {
        let err = SaralDosha::vata("Connection timeout");
        assert_eq!(err.sandesh(), "Connection timeout");
        assert_eq!(err.varga(), DoshaVarga::Vata);
        assert!(err.punarprapya()); // Recoverable
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_saral_dosha_with_code() {
        let err = SaralDosha::with_sanketa("Invalid format", DoshaVarga::Pitta, 1001);
        assert_eq!(err.sanketa(), Some(1001));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_pranika_not_recoverable() {
        let err = SaralDosha::pranika("Fatal error");
        assert!(!err.punarprapya()); // Not recoverable
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_dosha_srinkhala() {
        let mut chain = DoshaSrinkhala::new();
        chain.jod(SaralDosha::vata("Network error"));
        chain.jod(SaralDosha::pitta("Parse failed"));

        assert_eq!(chain.lambai(), 2);
        assert!(!chain.khali());
    }
}
