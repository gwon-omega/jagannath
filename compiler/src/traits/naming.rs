//! # Sanskrit Naming Traits (Nāma Lakṣaṇa - नाम लक्षण)
//!
//! Unified interface for all Sanskrit-named types across the compiler.
//!
//! ## Pattern Recognition
//! Researchers have found that Sanskrit's systematic morphology maps naturally
//! to type hierarchies - Pāṇini's grammar essentially defines interfaces for
//! word categories, just as these traits define interfaces for compiler concepts.
//!
//! > **"शब्दस्य स्फोटः अर्थबोधकः"**
//! > *"The sphoṭa (burst) of a word conveys meaning"*
//! — Bhartṛhari's Vākyapadīya
//!
//! ## Usage
//! ```rust
//! use jagannath_compiler::traits::SanskritNamed;
//!
//! // Any type implementing SanskritNamed can be displayed in multiple scripts
//! fn display_info<T: SanskritNamed>(item: &T) {
//!     println!("Sanskrit: {}", item.sanskrit());
//!     println!("IAST: {}", item.iast());
//!     println!("English: {}", item.english());
//! }
//! ```

/// Core trait for Sanskrit-named types
///
/// All philosophical concepts in the compiler implement this trait,
/// ensuring consistent naming across Devanagari, IAST, and English.
///
/// # Sanskrit Linguistic Foundation
/// This trait embodies the concept of **Nāma** (नाम - name) from
/// Vyākaraṇa (grammar), where every entity has multiple representations:
/// - धातु-रूप (dhātu-rūpa): the root form → `sanskrit()`
/// - लिप्यन्तर (lipyantara): transliteration → `iast()`
/// - अर्थ (artha): meaning → `english()`
pub trait SanskritNamed {
    /// Devanagari script representation (देवनागरी)
    ///
    /// Returns the name in original Sanskrit Devanagari script.
    /// This is the most authentic representation.
    ///
    /// # Example
    /// ```
    /// use jagannath_compiler::jyotisha::Graha;
    /// use jagannath_compiler::traits::SanskritNamed;
    ///
    /// let surya = Graha::Surya;
    /// assert_eq!(surya.sanskrit(), "सूर्य");
    /// ```
    fn sanskrit(&self) -> &'static str;

    /// IAST (International Alphabet of Sanskrit Transliteration)
    ///
    /// Returns the romanized form using standard academic transliteration.
    /// This includes diacritical marks for precise pronunciation.
    ///
    /// # Example
    /// ```ignore
    /// let surya = Graha::Surya;
    /// assert_eq!(surya.iast(), "Sūrya");
    /// ```
    fn iast(&self) -> &'static str {
        // Default: derive from english name
        // Override for proper diacritics
        self.english()
    }

    /// English translation or closest equivalent
    ///
    /// Returns the English name or translation for accessibility.
    fn english(&self) -> &'static str;
}

/// Extended trait for types with detailed descriptions
///
/// This extends `SanskritNamed` with additional semantic information,
/// mapping to the concept of **Viśeṣaṇa** (विशेषण - qualifier/attribute).
pub trait SanskritDescribed: SanskritNamed {
    /// Brief meaning or definition (अर्थ - artha)
    ///
    /// A one-line explanation of the concept's meaning.
    fn meaning(&self) -> &'static str;

    /// Detailed philosophical context (व्याख्या - vyākhyā)
    ///
    /// Expanded explanation including:
    /// - Historical/scriptural context
    /// - Compiler science mapping
    /// - Optimization implications
    fn explanation(&self) -> &'static str {
        self.meaning() // Default to brief meaning
    }

    /// Associated mantra or invocation (मन्त्र - mantra)
    ///
    /// For concepts that have traditional invocations.
    /// Returns `None` for concepts without associated mantras.
    fn mantra(&self) -> Option<&'static str> {
        None
    }

    /// Category or domain (वर्ग - varga)
    ///
    /// The broader category this concept belongs to.
    fn category(&self) -> &'static str;
}

/// Trait for types representing compilation domains
///
/// Maps Sanskrit concepts to compiler science domains.
/// Based on the **Adhikāra** (अधिकार - jurisdiction) concept.
pub trait CompilationDomain: SanskritNamed {
    /// The compiler subsystem this affects
    fn affected_subsystem(&self) -> &'static str;

    /// Optimization strategies associated with this domain
    fn optimization_strategies(&self) -> &'static [&'static str];

    /// Performance impact level (1-10)
    fn impact_level(&self) -> u8;
}

/// Marker trait for celestial/cosmic concepts
///
/// Types that represent astronomical or astrological entities.
/// Based on **Jyotiṣa** (ज्योतिष) principles.
pub trait CelestialConcept: SanskritNamed {
    /// Whether this is a benefic (शुभ) influence
    fn is_benefic(&self) -> bool;

    /// Whether this is a malefic (पाप) influence
    fn is_malefic(&self) -> bool {
        !self.is_benefic()
    }

    /// Natural strength (0.0 to 1.0)
    fn natural_strength(&self) -> f32;
}

// ============================================================================
// Helper macros for implementing traits
// ============================================================================

/// Macro for implementing SanskritNamed on an enum
///
/// # Example
/// ```ignore
/// impl_sanskrit_named!(Graha {
///     Surya => ("सूर्य", "Sūrya", "Sun"),
///     Chandra => ("चन्द्र", "Candra", "Moon"),
///     // ... more variants
/// });
/// ```
#[macro_export]
macro_rules! impl_sanskrit_named {
    ($type:ty { $($variant:ident => ($sanskrit:literal, $iast:literal, $english:literal)),* $(,)? }) => {
        impl $crate::traits::SanskritNamed for $type {
            fn sanskrit(&self) -> &'static str {
                match self {
                    $(Self::$variant => $sanskrit,)*
                }
            }

            fn iast(&self) -> &'static str {
                match self {
                    $(Self::$variant => $iast,)*
                }
            }

            fn english(&self) -> &'static str {
                match self {
                    $(Self::$variant => $english,)*
                }
            }
        }
    };
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Test implementation for demonstration
    #[derive(Debug, Clone, Copy)]
    enum TestGraha {
        Surya,
        Chandra,
    }

    impl SanskritNamed for TestGraha {
        fn sanskrit(&self) -> &'static str {
            match self {
                Self::Surya => "सूर्य",
                Self::Chandra => "चन्द्र",
            }
        }

        fn iast(&self) -> &'static str {
            match self {
                Self::Surya => "Sūrya",
                Self::Chandra => "Candra",
            }
        }

        fn english(&self) -> &'static str {
            match self {
                Self::Surya => "Sun",
                Self::Chandra => "Moon",
            }
        }
    }

    #[test]
    fn test_sanskrit_named() {
        let surya = TestGraha::Surya;
        assert_eq!(surya.sanskrit(), "सूर्य");
        assert_eq!(surya.iast(), "Sūrya");
        assert_eq!(surya.english(), "Sun");
    }

    #[test]
    fn test_chandra() {
        let chandra = TestGraha::Chandra;
        assert_eq!(chandra.sanskrit(), "चन्द्र");
        assert_eq!(chandra.english(), "Moon");
    }
}
