//! # Rudras - 11 Storm Deities
//!
//! The 11 Rudras represent 11 forms of destruction/transformation.
//! In compiler terms: 11 transformation passes between phases.
//!
//! ## The 11 Rudras:
//!
//! | Rudra | Aspect | Transformation Pass |
//! |-------|--------|---------------------|
//! | Aja | Unborn | Initial transform |
//! | Ekapada | One-footed | Single-pass |
//! | Ahirbudhnya | Serpent of depths | Deep analysis |
//! | Tvashta | Creator | Construction |
//! | Rudra | Howler | Error detection |
//! | Hara | Remover | Dead code removal |
//! | Shambhu | Source of happiness | Optimization |
//! | Tryambaka | Three-eyed | Multi-view analysis |
//! | Aparajita | Unconquered | Verification |
//! | Isana | Ruler | Control flow |
//! | Tribhuvana | Three worlds | Cross-module |

use super::CompilationState;
use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};
use tracing::debug;

/// Names of the 11 Rudras
pub const ELEVEN_RUDRAS: [&str; 11] = [
    "Aja",         // Initial transform
    "Ekapāda",     // Single-pass
    "Ahirbudhnya", // Deep analysis
    "Tvaṣṭā",      // Construction
    "Rudra",       // Error detection
    "Hara",        // Dead code removal
    "Śambhu",      // Optimization
    "Tryambaka",   // Multi-view analysis
    "Aparājita",   // Verification
    "Īśāna",       // Control flow
    "Tribhuvana",  // Cross-module
];

/// A Rudra (storm deity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rudra {
    Aja,         // Initial transform
    Ekapada,     // Single-pass
    Ahirbudhnya, // Deep analysis
    Tvashta,     // Construction
    RudraHowler, // Error detection
    Hara,        // Dead code removal
    Shambhu,     // Optimization
    Tryambaka,   // Multi-view analysis
    Aparajita,   // Verification
    Isana,       // Control flow
    Tribhuvana,  // Cross-module
}

impl Rudra {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Aja => "Aja",
            Self::Ekapada => "Ekapāda",
            Self::Ahirbudhnya => "Ahirbudhnya",
            Self::Tvashta => "Tvaṣṭā",
            Self::RudraHowler => "Rudra",
            Self::Hara => "Hara",
            Self::Shambhu => "Śambhu",
            Self::Tryambaka => "Tryambaka",
            Self::Aparajita => "Aparājita",
            Self::Isana => "Īśāna",
            Self::Tribhuvana => "Tribhuvana",
        }
    }

    pub fn transformation(&self) -> &'static str {
        match self {
            Self::Aja => "Initial Transform",
            Self::Ekapada => "Single-Pass Transform",
            Self::Ahirbudhnya => "Deep Analysis",
            Self::Tvashta => "Construction Pass",
            Self::RudraHowler => "Error Detection",
            Self::Hara => "Dead Code Removal",
            Self::Shambhu => "Optimization Pass",
            Self::Tryambaka => "Multi-View Analysis",
            Self::Aparajita => "Verification Pass",
            Self::Isana => "Control Flow Transform",
            Self::Tribhuvana => "Cross-Module Transform",
        }
    }

    pub fn order(&self) -> usize {
        match self {
            Self::Aja => 0,
            Self::Ekapada => 1,
            Self::Ahirbudhnya => 2,
            Self::Tvashta => 3,
            Self::RudraHowler => 4,
            Self::Hara => 5,
            Self::Shambhu => 6,
            Self::Tryambaka => 7,
            Self::Aparajita => 8,
            Self::Isana => 9,
            Self::Tribhuvana => 10,
        }
    }

    /// Get all 11 Rudras in order
    pub fn all() -> &'static [Self] {
        &[
            Self::Aja,
            Self::Ekapada,
            Self::Ahirbudhnya,
            Self::Tvashta,
            Self::RudraHowler,
            Self::Hara,
            Self::Shambhu,
            Self::Tryambaka,
            Self::Aparajita,
            Self::Isana,
            Self::Tribhuvana,
        ]
    }

    /// Get Sanskrit name (Devanagari)
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Aja => "अज",
            Self::Ekapada => "एकपाद",
            Self::Ahirbudhnya => "अहिर्बुध्न्य",
            Self::Tvashta => "त्वष्टा",
            Self::RudraHowler => "रुद्र",
            Self::Hara => "हर",
            Self::Shambhu => "शम्भु",
            Self::Tryambaka => "त्र्यम्बक",
            Self::Aparajita => "अपराजित",
            Self::Isana => "ईशान",
            Self::Tribhuvana => "त्रिभुवन",
        }
    }

    /// Get English meaning
    pub fn english(&self) -> &'static str {
        match self {
            Self::Aja => "The Unborn",
            Self::Ekapada => "The One-Footed",
            Self::Ahirbudhnya => "Serpent of the Depths",
            Self::Tvashta => "The Creator/Craftsman",
            Self::RudraHowler => "The Howler/Roarer",
            Self::Hara => "The Remover",
            Self::Shambhu => "Source of Happiness",
            Self::Tryambaka => "The Three-Eyed",
            Self::Aparajita => "The Unconquered",
            Self::Isana => "The Ruler",
            Self::Tribhuvana => "Lord of Three Worlds",
        }
    }
}

// ============================================================================
// v10.0 Trait Implementations
// ============================================================================

impl SanskritNamed for Rudra {
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

impl SanskritDescribed for Rudra {
    fn meaning(&self) -> &'static str {
        self.transformation()
    }

    fn explanation(&self) -> &'static str {
        match self {
            Self::Aja => {
                "First Rudra: Unborn - initial transformation before any pass, pristine state"
            }
            Self::Ekapada => {
                "Second Rudra: One-footed - single-pass transformations, linear processing"
            }
            Self::Ahirbudhnya => {
                "Third Rudra: Serpent of depths - deep analysis into nested structures"
            }
            Self::Tvashta => "Fourth Rudra: Divine craftsman - constructs IR from analyzed code",
            Self::RudraHowler => "Fifth Rudra: The howler - error detection and warning generation",
            Self::Hara => "Sixth Rudra: The remover - dead code elimination, unused symbol removal",
            Self::Shambhu => {
                "Seventh Rudra: Source of happiness - optimization brings joy through speed"
            }
            Self::Tryambaka => {
                "Eighth Rudra: Three-eyed - multi-view analysis (past, present, future)"
            }
            Self::Aparajita => {
                "Ninth Rudra: Unconquered - verification that cannot be defeated by bugs"
            }
            Self::Isana => "Tenth Rudra: The ruler - control flow analysis and transformation",
            Self::Tribhuvana => {
                "Eleventh Rudra: Three worlds - cross-module linking (source, IR, binary)"
            }
        }
    }

    fn mantra(&self) -> Option<&'static str> {
        Some(match self {
            Self::Aja => "ॐ अजाय नमः (Oṃ Ajāya Namaḥ)",
            Self::Ekapada => "ॐ एकपादाय नमः (Oṃ Ekapādāya Namaḥ)",
            Self::Ahirbudhnya => "ॐ अहिर्बुध्न्याय नमः (Oṃ Ahirbudhnyāya Namaḥ)",
            Self::Tvashta => "ॐ त्वष्ट्रे नमः (Oṃ Tvaṣṭre Namaḥ)",
            Self::RudraHowler => "ॐ रुद्राय नमः (Oṃ Rudrāya Namaḥ)",
            Self::Hara => "ॐ हराय नमः (Oṃ Harāya Namaḥ)",
            Self::Shambhu => "ॐ शम्भवे नमः (Oṃ Śambhave Namaḥ)",
            Self::Tryambaka => "ॐ त्र्यम्बकाय नमः (Oṃ Tryambakāya Namaḥ)",
            Self::Aparajita => "ॐ अपराजिताय नमः (Oṃ Aparājitāya Namaḥ)",
            Self::Isana => "ॐ ईशानाय नमः (Oṃ Īśānāya Namaḥ)",
            Self::Tribhuvana => "ॐ त्रिभुवनाय नमः (Oṃ Tribhuvanāya Namaḥ)",
        })
    }

    fn category(&self) -> &'static str {
        "Rudras - Storm Deities (रुद्र)"
    }
}

impl PhilosophicalEnum for Rudra {
    fn all() -> &'static [Self] {
        Rudra::all()
    }

    fn count() -> usize {
        11
    }

    fn index(&self) -> usize {
        self.order()
    }

    fn ordinal(&self) -> usize {
        self.order() + 1
    }

    fn next(&self) -> Self {
        Self::all()[(self.index() + 1) % 11]
    }

    fn prev(&self) -> Self {
        Self::all()[(self.index() + 11 - 1) % 11]
    }

    fn from_index(index: usize) -> Option<Self> {
        Self::all().get(index).copied()
    }
}

/// A transformation pass represented by a Rudra
pub struct RudraPass {
    /// Which Rudra
    pub rudra: Rudra,
    /// Whether this pass is enabled
    pub enabled: bool,
}

impl RudraPass {
    pub fn new(rudra: Rudra) -> Self {
        Self {
            rudra,
            enabled: true,
        }
    }

    /// Execute this transformation pass
    pub fn transform(&self, state: CompilationState) -> CompilationState {
        if !self.enabled {
            return state;
        }

        debug!(
            "Applying Rudra pass {}: {} ({})",
            self.rudra.order() + 1,
            self.rudra.name(),
            self.rudra.transformation()
        );

        // Stub: Each Rudra would do actual transformation
        match self.rudra {
            Rudra::Aja => {
                // Initial transform
            }
            Rudra::Ekapada => {
                // Single-pass transform
            }
            Rudra::Ahirbudhnya => {
                // Deep analysis
            }
            Rudra::Tvashta => {
                // Construction
            }
            Rudra::RudraHowler => {
                // Error detection (howling = warning)
            }
            Rudra::Hara => {
                // Dead code removal
            }
            Rudra::Shambhu => {
                // Optimization (brings happiness)
            }
            Rudra::Tryambaka => {
                // Multi-view analysis (three eyes)
            }
            Rudra::Aparajita => {
                // Verification (unconquerable correctness)
            }
            Rudra::Isana => {
                // Control flow transform
            }
            Rudra::Tribhuvana => {
                // Cross-module transform (three worlds)
            }
        }

        state
    }
}

/// Create all 11 Rudra passes
pub fn create_all() -> [RudraPass; 11] {
    [
        RudraPass::new(Rudra::Aja),
        RudraPass::new(Rudra::Ekapada),
        RudraPass::new(Rudra::Ahirbudhnya),
        RudraPass::new(Rudra::Tvashta),
        RudraPass::new(Rudra::RudraHowler),
        RudraPass::new(Rudra::Hara),
        RudraPass::new(Rudra::Shambhu),
        RudraPass::new(Rudra::Tryambaka),
        RudraPass::new(Rudra::Aparajita),
        RudraPass::new(Rudra::Isana),
        RudraPass::new(Rudra::Tribhuvana),
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
    fn test_rudra_count() {
        assert_eq!(Rudra::count(), 11);
        assert_eq!(Rudra::all().len(), 11);
    }

    #[test]
    fn test_rudra_sanskrit_named() {
        let rudra = Rudra::Hara;
        assert_eq!(rudra.sanskrit(), "हर");
        assert_eq!(rudra.iast(), "Hara");
        assert_eq!(rudra.english(), "The Remover");
    }

    #[test]
    fn test_rudra_sanskrit_described() {
        let rudra = Rudra::Tryambaka;
        assert_eq!(rudra.meaning(), "Multi-View Analysis");
        assert!(rudra.explanation().contains("Three-eyed"));
        assert!(rudra.mantra().is_some());
        assert_eq!(rudra.category(), "Rudras - Storm Deities (रुद्र)");
    }

    #[test]
    fn test_rudra_navigation() {
        // Forward: Storm passes through
        assert_eq!(Rudra::Aja.next(), Rudra::Ekapada);
        assert_eq!(Rudra::Tribhuvana.next(), Rudra::Aja); // Cycle back

        // Backward
        assert_eq!(Rudra::Ekapada.prev(), Rudra::Aja);
        assert_eq!(Rudra::Aja.prev(), Rudra::Tribhuvana); // Cycle back
    }

    #[test]
    fn test_rudra_from_index() {
        assert_eq!(Rudra::from_index(0), Some(Rudra::Aja));
        assert_eq!(Rudra::from_index(10), Some(Rudra::Tribhuvana));
        assert_eq!(Rudra::from_index(11), None);
    }

    #[test]
    fn test_rudra_ordinal_sequence() {
        for (i, rudra) in Rudra::all().iter().enumerate() {
            assert_eq!(rudra.ordinal(), i + 1, "Rudra {:?} ordinal mismatch", rudra);
            assert_eq!(rudra.index(), i, "Rudra {:?} index mismatch", rudra);
        }
    }

    #[test]
    fn test_rudra_all_have_mantras() {
        for rudra in Rudra::all() {
            assert!(rudra.mantra().is_some(), "{:?} should have a mantra", rudra);
            let mantra = rudra.mantra().unwrap();
            assert!(
                mantra.contains("ॐ"),
                "{:?} mantra should start with ॐ",
                rudra
            );
        }
    }
}
