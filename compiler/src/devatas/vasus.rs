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

use std::collections::HashMap;
use tracing::debug;

/// Names of the 8 Vasus
pub const EIGHT_VASUS: [&str; 8] = [
    "Dyaus",      // Symbol table
    "Pṛthivī",    // AST nodes
    "Vāyu",       // Control flow graph
    "Agni",       // Type system
    "Nakṣatra",   // Source map
    "Varuṇa",     // Data flow graph
    "Soma",       // Constants pool
    "Pratyūṣa",   // Initialization data
];

/// A Vasu (elemental deity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vasu {
    Dyaus,      // Sky - Symbol table
    Prithvi,    // Earth - AST nodes
    Vayu,       // Air - CFG
    Agni,       // Fire - Type system
    Nakshatra,  // Stars - Source map
    Varuna,     // Water - DFG
    Soma,       // Moon - Constants
    Pratyusha,  // Dawn - Init data
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
        debug!("Initializing Vasu structure: {} ({})",
                   self.vasu.name(), self.vasu.structure());
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
