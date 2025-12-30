//! # Adityas - 12 Solar Deities
//!
//! The 12 Adityas represent 12 months/phases of the solar cycle.
//! In compiler terms: 12 compilation phases from source to binary.
//!
//! ## The 12 Adityas:
//!
//! | Aditya | Month | Compilation Phase |
//! |--------|-------|-------------------|
//! | Dhata | Jan | Source input |
//! | Aryaman | Feb | Lexical analysis |
//! | Mitra | Mar | Parsing |
//! | Varuna | Apr | Semantic analysis |
//! | Indra | May | Type checking |
//! | Vivasvan | Jun | MIR generation |
//! | Pushan | Jul | Optimization |
//! | Parjanya | Aug | Code generation |
//! | Tvashta | Sep | Linking |
//! | Vishnu | Oct | Verification |
//! | Amsha | Nov | Packaging |
//! | Bhaga | Dec | Output |

use super::CompilationState;
use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};
use tracing::debug;

/// Names of the 12 Adityas
pub const TWELVE_ADITYAS: [&str; 12] = [
    "Dhātā",    // 1. Source input
    "Aryaman",  // 2. Lexical analysis
    "Mitra",    // 3. Parsing
    "Varuṇa",   // 4. Semantic analysis
    "Indra",    // 5. Type checking
    "Vivasvān", // 6. MIR generation
    "Pūṣan",    // 7. Optimization
    "Parjanya", // 8. Code generation
    "Tvaṣṭā",   // 9. Linking
    "Viṣṇu",    // 10. Verification
    "Aṃśa",     // 11. Packaging
    "Bhaga",    // 12. Output
];

/// An Aditya (solar deity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Aditya {
    Dhata,    // Source input
    Aryaman,  // Lexical analysis
    Mitra,    // Parsing
    Varuna,   // Semantic analysis
    Indra,    // Type checking
    Vivasvan, // MIR generation
    Pushan,   // Optimization
    Parjanya, // Code generation
    Tvashta,  // Linking
    Vishnu,   // Verification
    Amsha,    // Packaging
    Bhaga,    // Output
}

impl Aditya {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Dhata => "Dhātā",
            Self::Aryaman => "Aryaman",
            Self::Mitra => "Mitra",
            Self::Varuna => "Varuṇa",
            Self::Indra => "Indra",
            Self::Vivasvan => "Vivasvān",
            Self::Pushan => "Pūṣan",
            Self::Parjanya => "Parjanya",
            Self::Tvashta => "Tvaṣṭā",
            Self::Vishnu => "Viṣṇu",
            Self::Amsha => "Aṃśa",
            Self::Bhaga => "Bhaga",
        }
    }

    pub fn phase_name(&self) -> &'static str {
        match self {
            Self::Dhata => "Source Input",
            Self::Aryaman => "Lexical Analysis",
            Self::Mitra => "Parsing",
            Self::Varuna => "Semantic Analysis",
            Self::Indra => "Type Checking",
            Self::Vivasvan => "MIR Generation",
            Self::Pushan => "Optimization",
            Self::Parjanya => "Code Generation",
            Self::Tvashta => "Linking",
            Self::Vishnu => "Verification",
            Self::Amsha => "Packaging",
            Self::Bhaga => "Output",
        }
    }

    pub fn order(&self) -> usize {
        match self {
            Self::Dhata => 0,
            Self::Aryaman => 1,
            Self::Mitra => 2,
            Self::Varuna => 3,
            Self::Indra => 4,
            Self::Vivasvan => 5,
            Self::Pushan => 6,
            Self::Parjanya => 7,
            Self::Tvashta => 8,
            Self::Vishnu => 9,
            Self::Amsha => 10,
            Self::Bhaga => 11,
        }
    }

    /// Get all 12 Adityas in order
    pub fn all() -> &'static [Self] {
        &[
            Self::Dhata,
            Self::Aryaman,
            Self::Mitra,
            Self::Varuna,
            Self::Indra,
            Self::Vivasvan,
            Self::Pushan,
            Self::Parjanya,
            Self::Tvashta,
            Self::Vishnu,
            Self::Amsha,
            Self::Bhaga,
        ]
    }

    /// Get Sanskrit name (Devanagari)
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Dhata => "धाता",
            Self::Aryaman => "अर्यमन्",
            Self::Mitra => "मित्र",
            Self::Varuna => "वरुण",
            Self::Indra => "इन्द्र",
            Self::Vivasvan => "विवस्वान्",
            Self::Pushan => "पूषन्",
            Self::Parjanya => "पर्जन्य",
            Self::Tvashta => "त्वष्टा",
            Self::Vishnu => "विष्णु",
            Self::Amsha => "अंश",
            Self::Bhaga => "भग",
        }
    }

    /// Get IAST transliteration
    pub fn iast(&self) -> &'static str {
        self.name() // Already in IAST format
    }

    /// Get English meaning
    pub fn english(&self) -> &'static str {
        match self {
            Self::Dhata => "The Creator",
            Self::Aryaman => "The Noble Friend",
            Self::Mitra => "The Friend",
            Self::Varuna => "The All-Encompassing",
            Self::Indra => "The King of Gods",
            Self::Vivasvan => "The Shining One",
            Self::Pushan => "The Nourisher",
            Self::Parjanya => "The Rain Cloud",
            Self::Tvashta => "The Divine Craftsman",
            Self::Vishnu => "The Pervader",
            Self::Amsha => "The Share/Portion",
            Self::Bhaga => "The Fortune",
        }
    }
}

// ============================================================================
// v10.0 Trait Implementations
// ============================================================================

impl SanskritNamed for Aditya {
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

impl SanskritDescribed for Aditya {
    fn meaning(&self) -> &'static str {
        self.phase_name()
    }

    fn explanation(&self) -> &'static str {
        match self {
            Self::Dhata => "First Aditya: Creates/receives source code input to begin compilation",
            Self::Aryaman => "Second Aditya: Noble friend performs lexical analysis, tokenization",
            Self::Mitra => "Third Aditya: Friendly parsing builds abstract syntax tree",
            Self::Varuna => "Fourth Aditya: All-encompassing semantic analysis, name resolution",
            Self::Indra => "Fifth Aditya: King performs type checking, enforces type rules",
            Self::Vivasvan => "Sixth Aditya: Shining MIR generation, intermediate representation",
            Self::Pushan => "Seventh Aditya: Nourisher optimizes code, applies transformations",
            Self::Parjanya => "Eighth Aditya: Rain of code generation, produces target code",
            Self::Tvashta => "Ninth Aditya: Divine craftsman links objects into executable",
            Self::Vishnu => "Tenth Aditya: Pervader verifies correctness of generated code",
            Self::Amsha => "Eleventh Aditya: Portion packages artifacts for distribution",
            Self::Bhaga => "Twelfth Aditya: Fortune delivers final output to user",
        }
    }

    fn mantra(&self) -> Option<&'static str> {
        Some(match self {
            Self::Dhata => "ॐ धात्रे नमः (Oṃ Dhātre Namaḥ)",
            Self::Aryaman => "ॐ अर्यम्णे नमः (Oṃ Aryamṇe Namaḥ)",
            Self::Mitra => "ॐ मित्राय नमः (Oṃ Mitrāya Namaḥ)",
            Self::Varuna => "ॐ वरुणाय नमः (Oṃ Varuṇāya Namaḥ)",
            Self::Indra => "ॐ इन्द्राय नमः (Oṃ Indrāya Namaḥ)",
            Self::Vivasvan => "ॐ विवस्वते नमः (Oṃ Vivasvate Namaḥ)",
            Self::Pushan => "ॐ पूष्णे नमः (Oṃ Pūṣṇe Namaḥ)",
            Self::Parjanya => "ॐ पर्जन्याय नमः (Oṃ Parjanyāya Namaḥ)",
            Self::Tvashta => "ॐ त्वष्ट्रे नमः (Oṃ Tvaṣṭre Namaḥ)",
            Self::Vishnu => "ॐ विष्णवे नमः (Oṃ Viṣṇave Namaḥ)",
            Self::Amsha => "ॐ अंशाय नमः (Oṃ Aṃśāya Namaḥ)",
            Self::Bhaga => "ॐ भगाय नमः (Oṃ Bhagāya Namaḥ)",
        })
    }

    fn category(&self) -> &'static str {
        "Adityas - Solar Deities (आदित्य)"
    }
}

impl PhilosophicalEnum for Aditya {
    fn all() -> &'static [Self] {
        Aditya::all()
    }

    fn count() -> usize {
        12
    }

    fn index(&self) -> usize {
        self.order()
    }

    fn ordinal(&self) -> usize {
        self.order() + 1
    }

    fn next(&self) -> Self {
        Self::all()[(self.index() + 1) % 12]
    }

    fn prev(&self) -> Self {
        Self::all()[(self.index() + 12 - 1) % 12]
    }

    fn from_index(index: usize) -> Option<Self> {
        Self::all().get(index).copied()
    }
}

/// A compilation phase represented by an Aditya
pub struct AdityaPhase {
    /// Which Aditya
    pub aditya: Aditya,
    /// Whether this phase is enabled
    pub enabled: bool,
}

impl AdityaPhase {
    pub fn new(aditya: Aditya) -> Self {
        Self {
            aditya,
            enabled: true,
        }
    }

    /// Execute this compilation phase
    pub fn execute(&self, mut state: CompilationState) -> CompilationState {
        if !self.enabled {
            return state;
        }

        debug!(
            "Executing phase {}: {} ({})",
            self.aditya.order() + 1,
            self.aditya.name(),
            self.aditya.phase_name()
        );

        state.current_phase = self.aditya.order();

        // Stub: Each phase would do actual work
        match self.aditya {
            Aditya::Dhata => {
                // Source is already in state
            }
            Aditya::Aryaman => {
                // Lexical analysis stub
                state.tokens = vec!["token1".to_string(), "token2".to_string()];
            }
            Aditya::Mitra => {
                // Parsing stub
                state.ast = Some("(ast)".to_string());
            }
            Aditya::Varuna => {
                // Semantic analysis stub
            }
            Aditya::Indra => {
                // Type checking stub
            }
            Aditya::Vivasvan => {
                // MIR generation stub
                state.mir = Some("(mir)".to_string());
            }
            Aditya::Pushan => {
                // Optimization stub
            }
            Aditya::Parjanya => {
                // Code generation stub
            }
            Aditya::Tvashta => {
                // Linking stub
            }
            Aditya::Vishnu => {
                // Verification stub
            }
            Aditya::Amsha => {
                // Packaging stub
            }
            Aditya::Bhaga => {
                // Output stub
                state.output = Some(Vec::new());
            }
        }

        state
    }
}

/// Create all 12 Aditya phases
pub fn create_all() -> [AdityaPhase; 12] {
    [
        AdityaPhase::new(Aditya::Dhata),
        AdityaPhase::new(Aditya::Aryaman),
        AdityaPhase::new(Aditya::Mitra),
        AdityaPhase::new(Aditya::Varuna),
        AdityaPhase::new(Aditya::Indra),
        AdityaPhase::new(Aditya::Vivasvan),
        AdityaPhase::new(Aditya::Pushan),
        AdityaPhase::new(Aditya::Parjanya),
        AdityaPhase::new(Aditya::Tvashta),
        AdityaPhase::new(Aditya::Vishnu),
        AdityaPhase::new(Aditya::Amsha),
        AdityaPhase::new(Aditya::Bhaga),
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
    fn test_aditya_count() {
        assert_eq!(Aditya::count(), 12);
        assert_eq!(Aditya::all().len(), 12);
    }

    #[test]
    fn test_aditya_sanskrit_named() {
        let aditya = Aditya::Indra;
        assert_eq!(aditya.sanskrit(), "इन्द्र");
        assert_eq!(aditya.iast(), "Indra");
        assert_eq!(aditya.english(), "The King of Gods");
    }

    #[test]
    fn test_aditya_sanskrit_described() {
        let aditya = Aditya::Varuna;
        assert_eq!(aditya.meaning(), "Semantic Analysis");
        assert!(aditya.explanation().contains("All-encompassing"));
        assert!(aditya.mantra().is_some());
        assert_eq!(aditya.category(), "Adityas - Solar Deities (आदित्य)");
    }

    #[test]
    fn test_aditya_navigation_solar_cycle() {
        // Forward: Sun moves through 12 months
        assert_eq!(Aditya::Dhata.next(), Aditya::Aryaman);
        assert_eq!(Aditya::Bhaga.next(), Aditya::Dhata); // Cycle back to January

        // Backward
        assert_eq!(Aditya::Aryaman.prev(), Aditya::Dhata);
        assert_eq!(Aditya::Dhata.prev(), Aditya::Bhaga); // Cycle back to December
    }

    #[test]
    fn test_aditya_from_index() {
        assert_eq!(Aditya::from_index(0), Some(Aditya::Dhata));
        assert_eq!(Aditya::from_index(11), Some(Aditya::Bhaga));
        assert_eq!(Aditya::from_index(12), None);
    }

    #[test]
    fn test_aditya_ordinal_sequence() {
        for (i, aditya) in Aditya::all().iter().enumerate() {
            assert_eq!(
                aditya.ordinal(),
                i + 1,
                "Aditya {:?} ordinal mismatch",
                aditya
            );
            assert_eq!(aditya.index(), i, "Aditya {:?} index mismatch", aditya);
        }
    }

    #[test]
    fn test_aditya_compilation_phases() {
        // Each Aditya maps to a compilation phase
        assert_eq!(Aditya::Dhata.phase_name(), "Source Input");
        assert_eq!(Aditya::Aryaman.phase_name(), "Lexical Analysis");
        assert_eq!(Aditya::Mitra.phase_name(), "Parsing");
        assert_eq!(Aditya::Varuna.phase_name(), "Semantic Analysis");
        assert_eq!(Aditya::Indra.phase_name(), "Type Checking");
        assert_eq!(Aditya::Vivasvan.phase_name(), "MIR Generation");
        assert_eq!(Aditya::Pushan.phase_name(), "Optimization");
        assert_eq!(Aditya::Parjanya.phase_name(), "Code Generation");
        assert_eq!(Aditya::Tvashta.phase_name(), "Linking");
        assert_eq!(Aditya::Vishnu.phase_name(), "Verification");
        assert_eq!(Aditya::Amsha.phase_name(), "Packaging");
        assert_eq!(Aditya::Bhaga.phase_name(), "Output");
    }

    #[test]
    fn test_aditya_all_have_mantras() {
        for aditya in Aditya::all() {
            assert!(
                aditya.mantra().is_some(),
                "{:?} should have a mantra",
                aditya
            );
            let mantra = aditya.mantra().unwrap();
            assert!(
                mantra.contains("ॐ"),
                "{:?} mantra should start with ॐ",
                aditya
            );
            assert!(
                mantra.contains("नमः"),
                "{:?} mantra should end with नमः",
                aditya
            );
        }
    }
}
