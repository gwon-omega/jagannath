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
use tracing::debug;

/// Names of the 12 Adityas
pub const TWELVE_ADITYAS: [&str; 12] = [
    "Dhātā",      // 1. Source input
    "Aryaman",    // 2. Lexical analysis
    "Mitra",      // 3. Parsing
    "Varuṇa",     // 4. Semantic analysis
    "Indra",      // 5. Type checking
    "Vivasvān",   // 6. MIR generation
    "Pūṣan",      // 7. Optimization
    "Parjanya",   // 8. Code generation
    "Tvaṣṭā",     // 9. Linking
    "Viṣṇu",      // 10. Verification
    "Aṃśa",       // 11. Packaging
    "Bhaga",      // 12. Output
];

/// An Aditya (solar deity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Aditya {
    Dhata,      // Source input
    Aryaman,    // Lexical analysis
    Mitra,      // Parsing
    Varuna,     // Semantic analysis
    Indra,      // Type checking
    Vivasvan,   // MIR generation
    Pushan,     // Optimization
    Parjanya,   // Code generation
    Tvashta,    // Linking
    Vishnu,     // Verification
    Amsha,      // Packaging
    Bhaga,      // Output
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

        debug!("Executing phase {}: {} ({})",
                   self.aditya.order() + 1,
                   self.aditya.name(),
                   self.aditya.phase_name());

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
