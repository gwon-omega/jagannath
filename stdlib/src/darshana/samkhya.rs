//! Sāṃkhya Module (सांख्य) — 25 Tattvas
//!
//! Provides introspection and metadata for the 25 principles of reality.

use core::fmt;

/// The 25 Tattvas (principles of reality)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Tattva {
    // Gross Elements (Mahābhūta) - Levels 1-5
    /// Pṛthivī (Earth) - Solid/binary emission
    Prithivi = 1,
    /// Āpas (Water) - Fluid/data layout
    Apas = 2,
    /// Tejas (Fire) - Energy/peephole optimization
    Tejas = 3,
    /// Vāyu (Air) - Movement/instruction scheduling
    Vayu = 4,
    /// Ākāśa (Space) - Container/register allocation
    Akasha = 5,

    // Subtle Elements (Tanmātra) - Levels 6-10
    /// Gandha (Smell) - Memory optimization
    Gandha = 6,
    /// Rasa (Taste) - Inlining decisions
    Rasa = 7,
    /// Rūpa (Form) - Loop optimization
    Rupa = 8,
    /// Sparśa (Touch) - Common subexpression
    Sparsha = 9,
    /// Śabda (Sound) - Constant propagation
    Shabda = 10,

    // Action Organs (Karmendriya) - Levels 11-15
    /// Upastha (Generation) - Code generation
    Upastha = 11,
    /// Pāyu (Excretion) - Dead code elimination
    Payu = 12,
    /// Pāda (Feet) - Control flow
    Pada = 13,
    /// Pāṇi (Hands) - Code manipulation
    Pani = 14,
    /// Vāk (Speech) - Error reporting
    Vak = 15,

    // Sense Organs (Jñānendriya) - Levels 16-20
    /// Ghrāṇa (Nose) - Borrow checking
    Ghrana = 16,
    /// Rasanā (Tongue) - Semantic validation
    Rasana = 17,
    /// Cakṣus (Eye) - Type checking
    Cakshus = 18,
    /// Tvak (Skin) - Syntax validation
    Tvak = 19,
    /// Śrotra (Ear) - Lexical validation
    Shrotra = 20,

    // Inner Instruments (Antaḥkaraṇa) - Levels 21-23
    /// Manas (Mind) - IR generation
    Manas = 21,
    /// Ahaṃkāra (Ego) - Symbol table
    Ahamkara = 22,
    /// Buddhi (Intellect) - Semantic analysis
    Buddhi = 23,

    // Primordial - Levels 24-25
    /// Prakṛti (Nature) - AST potential
    Prakriti = 24,
    /// Puruṣa (Consciousness) - Source intent
    Purusha = 25,
}

impl Tattva {
    /// Get the level (1-25) of this tattva
    pub fn level(&self) -> u8 {
        *self as u8
    }

    /// Get the Sanskrit name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Prithivi => "पृथिवी",
            Self::Apas => "आपस्",
            Self::Tejas => "तेजस्",
            Self::Vayu => "वायु",
            Self::Akasha => "आकाश",
            Self::Gandha => "गन्ध",
            Self::Rasa => "रस",
            Self::Rupa => "रूप",
            Self::Sparsha => "स्पर्श",
            Self::Shabda => "शब्द",
            Self::Upastha => "उपस्थ",
            Self::Payu => "पायु",
            Self::Pada => "पाद",
            Self::Pani => "पाणि",
            Self::Vak => "वाक्",
            Self::Ghrana => "घ्राण",
            Self::Rasana => "रसना",
            Self::Cakshus => "चक्षुस्",
            Self::Tvak => "त्वक्",
            Self::Shrotra => "श्रोत्र",
            Self::Manas => "मनस्",
            Self::Ahamkara => "अहंकार",
            Self::Buddhi => "बुद्धि",
            Self::Prakriti => "प्रकृति",
            Self::Purusha => "पुरुष",
        }
    }

    /// Get the category of this tattva
    pub fn category(&self) -> TattvaCategory {
        match self.level() {
            1..=5 => TattvaCategory::Mahabhuta,
            6..=10 => TattvaCategory::Tanmatra,
            11..=15 => TattvaCategory::Karmendriya,
            16..=20 => TattvaCategory::Jnanendriya,
            21..=23 => TattvaCategory::Antahkarana,
            24..=25 => TattvaCategory::Primordial,
            _ => unreachable!(),
        }
    }

    /// Get the compilation phase this tattva represents
    pub fn compiler_phase(&self) -> &'static str {
        match self {
            Self::Prithivi => "Binary Emission",
            Self::Apas => "Data Layout",
            Self::Tejas => "Peephole Optimization",
            Self::Vayu => "Instruction Scheduling",
            Self::Akasha => "Register Allocation",
            Self::Gandha => "Memory Optimization",
            Self::Rasa => "Inlining",
            Self::Rupa => "Loop Optimization",
            Self::Sparsha => "CSE",
            Self::Shabda => "Constant Propagation",
            Self::Upastha => "Code Generation",
            Self::Payu => "DCE",
            Self::Pada => "Control Flow",
            Self::Pani => "Code Manipulation",
            Self::Vak => "Error Reporting",
            Self::Ghrana => "Borrow Checking",
            Self::Rasana => "Semantic Validation",
            Self::Cakshus => "Type Checking",
            Self::Tvak => "Syntax Validation",
            Self::Shrotra => "Lexical Validation",
            Self::Manas => "IR Generation",
            Self::Ahamkara => "Symbol Resolution",
            Self::Buddhi => "Semantic Analysis",
            Self::Prakriti => "AST Generation",
            Self::Purusha => "Source Intent",
        }
    }

    /// Check if this tattva is more subtle than another
    pub fn is_subtler_than(&self, other: &Tattva) -> bool {
        self.level() > other.level()
    }

    /// Check if this tattva is more gross than another
    pub fn is_grosser_than(&self, other: &Tattva) -> bool {
        self.level() < other.level()
    }
}

impl fmt::Display for Tattva {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({}) - {}", self, self.sanskrit_name(), self.compiler_phase())
    }
}

/// Categories of tattvas
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TattvaCategory {
    /// Mahābhūta - Gross elements (1-5)
    Mahabhuta,
    /// Tanmātra - Subtle elements (6-10)
    Tanmatra,
    /// Karmendriya - Action organs (11-15)
    Karmendriya,
    /// Jñānendriya - Sense organs (16-20)
    Jnanendriya,
    /// Antaḥkaraṇa - Inner instruments (21-23)
    Antahkarana,
    /// Primordial principles (24-25)
    Primordial,
}

impl TattvaCategory {
    /// Get all tattvas in this category
    pub fn tattvas(&self) -> &'static [Tattva] {
        use Tattva::*;
        match self {
            Self::Mahabhuta => &[Prithivi, Apas, Tejas, Vayu, Akasha],
            Self::Tanmatra => &[Gandha, Rasa, Rupa, Sparsha, Shabda],
            Self::Karmendriya => &[Upastha, Payu, Pada, Pani, Vak],
            Self::Jnanendriya => &[Ghrana, Rasana, Cakshus, Tvak, Shrotra],
            Self::Antahkarana => &[Manas, Ahamkara, Buddhi],
            Self::Primordial => &[Prakriti, Purusha],
        }
    }
}

/// Compilation progress through tattvas
#[derive(Debug, Clone)]
pub struct TattvaProgress {
    current: Tattva,
    completed: Vec<Tattva>,
}

impl TattvaProgress {
    /// Start compilation at Purusha (source intent)
    pub fn new() -> Self {
        Self {
            current: Tattva::Purusha,
            completed: Vec::new(),
        }
    }

    /// Move to the next (grosser) tattva
    pub fn descend(&mut self) -> Option<Tattva> {
        let level = self.current.level();
        if level > 1 {
            self.completed.push(self.current);
            // Find next tattva (level - 1)
            self.current = unsafe { core::mem::transmute(level - 1) };
            Some(self.current)
        } else {
            None
        }
    }

    /// Get current compilation phase
    pub fn current_phase(&self) -> &'static str {
        self.current.compiler_phase()
    }

    /// Check if compilation is complete
    pub fn is_complete(&self) -> bool {
        self.current == Tattva::Prithivi && self.completed.len() == 24
    }
}

impl Default for TattvaProgress {
    fn default() -> Self {
        Self::new()
    }
}
