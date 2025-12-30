//! # Vasus - 8 Elemental Deities
//!
//! The 8 Vasus represent 8 fundamental elements.
//! In compiler terms: 8 core data structures.
//!
//! ## The 8 Vasus:
//!
//! | Vasu | Element | Data Structure |
//! |------|---------|----------------|
//! | Dyaus | Sky | Symbol table |
//! | Prithvi | Earth | AST nodes |
//! | Vayu | Air | Control flow graph |
//! | Agni | Fire | Type system |
//! | Nakshatra | Stars | Source map |
//! | Varuna | Water | Data flow graph |
//! | Soma | Moon | Constants pool |
//! | Pratyusha | Dawn | Initialization data |

use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};
use std::collections::HashMap;
use tracing::debug;

/// Names of the 8 Vasus
pub const EIGHT_VASUS: [&str; 8] = [
    "Dyaus",    // Symbol table
    "Pṛthivī",  // AST nodes
    "Vāyu",     // Control flow graph
    "Agni",     // Type system
    "Nakṣatra", // Source map
    "Varuṇa",   // Data flow graph
    "Soma",     // Constants pool
    "Pratyūṣa", // Initialization data
];

/// A Vasu (elemental deity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vasu {
    Dyaus,     // Sky - Symbol table
    Prithvi,   // Earth - AST nodes
    Vayu,      // Air - CFG
    Agni,      // Fire - Type system
    Nakshatra, // Stars - Source map
    Varuna,    // Water - DFG
    Soma,      // Moon - Constants
    Pratyusha, // Dawn - Init data
}

impl Vasu {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Dyaus => "Dyaus",
            Self::Prithvi => "Pṛthivī",
            Self::Vayu => "Vāyu",
            Self::Agni => "Agni",
            Self::Nakshatra => "Nakṣatra",
            Self::Varuna => "Varuṇa",
            Self::Soma => "Soma",
            Self::Pratyusha => "Pratyūṣa",
        }
    }

    pub fn element(&self) -> &'static str {
        match self {
            Self::Dyaus => "Sky",
            Self::Prithvi => "Earth",
            Self::Vayu => "Air",
            Self::Agni => "Fire",
            Self::Nakshatra => "Stars",
            Self::Varuna => "Water",
            Self::Soma => "Moon",
            Self::Pratyusha => "Dawn",
        }
    }

    pub fn structure(&self) -> &'static str {
        match self {
            Self::Dyaus => "Symbol Table",
            Self::Prithvi => "AST Nodes",
            Self::Vayu => "Control Flow Graph",
            Self::Agni => "Type System",
            Self::Nakshatra => "Source Map",
            Self::Varuna => "Data Flow Graph",
            Self::Soma => "Constants Pool",
            Self::Pratyusha => "Initialization Data",
        }
    }

    /// Get all 8 Vasus in order
    pub fn all() -> &'static [Self] {
        &[
            Self::Dyaus,
            Self::Prithvi,
            Self::Vayu,
            Self::Agni,
            Self::Nakshatra,
            Self::Varuna,
            Self::Soma,
            Self::Pratyusha,
        ]
    }

    /// Get index (0-based)
    pub fn order(&self) -> usize {
        match self {
            Self::Dyaus => 0,
            Self::Prithvi => 1,
            Self::Vayu => 2,
            Self::Agni => 3,
            Self::Nakshatra => 4,
            Self::Varuna => 5,
            Self::Soma => 6,
            Self::Pratyusha => 7,
        }
    }

    /// Get Sanskrit name (Devanagari)
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Dyaus => "द्यौस्",
            Self::Prithvi => "पृथिवी",
            Self::Vayu => "वायु",
            Self::Agni => "अग्नि",
            Self::Nakshatra => "नक्षत्र",
            Self::Varuna => "वरुण",
            Self::Soma => "सोम",
            Self::Pratyusha => "प्रत्यूष",
        }
    }

    /// Get English meaning
    pub fn english(&self) -> &'static str {
        match self {
            Self::Dyaus => "The Sky/Heaven Father",
            Self::Prithvi => "The Earth Mother",
            Self::Vayu => "The Wind God",
            Self::Agni => "The Fire God",
            Self::Nakshatra => "The Stars/Constellations",
            Self::Varuna => "The Water Lord",
            Self::Soma => "The Moon/Divine Nectar",
            Self::Pratyusha => "The Dawn",
        }
    }
}

// ============================================================================
// v10.0 Trait Implementations
// ============================================================================

impl SanskritNamed for Vasu {
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

impl SanskritDescribed for Vasu {
    fn meaning(&self) -> &'static str {
        self.structure()
    }

    fn explanation(&self) -> &'static str {
        match self {
            Self::Dyaus => {
                "First Vasu: Sky-father holds all symbols like stars in heaven - the symbol table"
            }
            Self::Prithvi => {
                "Second Vasu: Earth-mother grounds all AST nodes - solid foundation of syntax tree"
            }
            Self::Vayu => "Third Vasu: Wind flows through control paths - the control flow graph",
            Self::Agni => "Fourth Vasu: Fire transforms types, burns type errors - the type system",
            Self::Nakshatra => "Fifth Vasu: Stars map positions - source locations in source map",
            Self::Varuna => "Sixth Vasu: Water flows carrying data - the data flow graph",
            Self::Soma => {
                "Seventh Vasu: Moon's nectar is unchanging - constants pool, immutable values"
            }
            Self::Pratyusha => {
                "Eighth Vasu: Dawn initiates the day - initialization data and startup state"
            }
        }
    }

    fn mantra(&self) -> Option<&'static str> {
        Some(match self {
            Self::Dyaus => "ॐ द्यौसे नमः (Oṃ Dyause Namaḥ)",
            Self::Prithvi => "ॐ पृथिव्यै नमः (Oṃ Pṛthivyai Namaḥ)",
            Self::Vayu => "ॐ वायवे नमः (Oṃ Vāyave Namaḥ)",
            Self::Agni => "ॐ अग्नये नमः (Oṃ Agnaye Namaḥ)",
            Self::Nakshatra => "ॐ नक्षत्रेभ्यो नमः (Oṃ Nakṣatrebhyo Namaḥ)",
            Self::Varuna => "ॐ वरुणाय नमः (Oṃ Varuṇāya Namaḥ)",
            Self::Soma => "ॐ सोमाय नमः (Oṃ Somāya Namaḥ)",
            Self::Pratyusha => "ॐ प्रत्यूषाय नमः (Oṃ Pratyūṣāya Namaḥ)",
        })
    }

    fn category(&self) -> &'static str {
        "Vasus - Elemental Deities (वसु)"
    }
}

impl PhilosophicalEnum for Vasu {
    fn all() -> &'static [Self] {
        Vasu::all()
    }

    fn count() -> usize {
        8
    }

    fn index(&self) -> usize {
        self.order()
    }

    fn ordinal(&self) -> usize {
        self.order() + 1
    }

    fn next(&self) -> Self {
        Self::all()[(self.index() + 1) % 8]
    }

    fn prev(&self) -> Self {
        Self::all()[(self.index() + 8 - 1) % 8]
    }

    fn from_index(index: usize) -> Option<Self> {
        Self::all().get(index).copied()
    }
}

/// A data structure managed by a Vasu
pub struct VasuStructure {
    /// Which Vasu
    pub vasu: Vasu,
    /// Internal storage (simplified)
    storage: HashMap<String, String>,
    /// Whether initialized
    initialized: bool,
}

impl VasuStructure {
    pub fn new(vasu: Vasu) -> Self {
        Self {
            vasu,
            storage: HashMap::new(),
            initialized: false,
        }
    }

    /// Initialize the data structure
    pub fn initialize(&mut self) {
        debug!(
            "Initializing Vasu structure: {} ({})",
            self.vasu.name(),
            self.vasu.structure()
        );
        self.storage.clear();
        self.initialized = true;
    }

    /// Store a value
    pub fn store(&mut self, key: &str, value: &str) {
        self.storage.insert(key.to_string(), value.to_string());
    }

    /// Retrieve a value
    pub fn get(&self, key: &str) -> Option<&String> {
        self.storage.get(key)
    }

    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get size
    pub fn size(&self) -> usize {
        self.storage.len()
    }
}

/// Create all 8 Vasu structures
pub fn create_all() -> [VasuStructure; 8] {
    [
        VasuStructure::new(Vasu::Dyaus),
        VasuStructure::new(Vasu::Prithvi),
        VasuStructure::new(Vasu::Vayu),
        VasuStructure::new(Vasu::Agni),
        VasuStructure::new(Vasu::Nakshatra),
        VasuStructure::new(Vasu::Varuna),
        VasuStructure::new(Vasu::Soma),
        VasuStructure::new(Vasu::Pratyusha),
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
    fn test_vasu_count() {
        assert_eq!(Vasu::count(), 8);
        assert_eq!(Vasu::all().len(), 8);
    }

    #[test]
    fn test_vasu_sanskrit_named() {
        let vasu = Vasu::Agni;
        assert_eq!(vasu.sanskrit(), "अग्नि");
        assert_eq!(vasu.iast(), "Agni");
        assert_eq!(vasu.english(), "The Fire God");
    }

    #[test]
    fn test_vasu_sanskrit_described() {
        let vasu = Vasu::Vayu;
        assert_eq!(vasu.meaning(), "Control Flow Graph");
        assert!(vasu.explanation().contains("Wind"));
        assert!(vasu.mantra().is_some());
        assert_eq!(vasu.category(), "Vasus - Elemental Deities (वसु)");
    }

    #[test]
    fn test_vasu_elements() {
        assert_eq!(Vasu::Dyaus.element(), "Sky");
        assert_eq!(Vasu::Prithvi.element(), "Earth");
        assert_eq!(Vasu::Vayu.element(), "Air");
        assert_eq!(Vasu::Agni.element(), "Fire");
        assert_eq!(Vasu::Nakshatra.element(), "Stars");
        assert_eq!(Vasu::Varuna.element(), "Water");
        assert_eq!(Vasu::Soma.element(), "Moon");
        assert_eq!(Vasu::Pratyusha.element(), "Dawn");
    }

    #[test]
    fn test_vasu_navigation() {
        // Forward: Elements cycle
        assert_eq!(Vasu::Dyaus.next(), Vasu::Prithvi);
        assert_eq!(Vasu::Pratyusha.next(), Vasu::Dyaus); // Cycle back

        // Backward
        assert_eq!(Vasu::Prithvi.prev(), Vasu::Dyaus);
        assert_eq!(Vasu::Dyaus.prev(), Vasu::Pratyusha); // Cycle back
    }

    #[test]
    fn test_vasu_from_index() {
        assert_eq!(Vasu::from_index(0), Some(Vasu::Dyaus));
        assert_eq!(Vasu::from_index(7), Some(Vasu::Pratyusha));
        assert_eq!(Vasu::from_index(8), None);
    }

    #[test]
    fn test_vasu_ordinal_sequence() {
        for (i, vasu) in Vasu::all().iter().enumerate() {
            assert_eq!(vasu.ordinal(), i + 1, "Vasu {:?} ordinal mismatch", vasu);
            assert_eq!(vasu.index(), i, "Vasu {:?} index mismatch", vasu);
        }
    }
}
