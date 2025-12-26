//! Sāṃkhya Compilation Pipeline
//!
//! Organizes compilation stages according to the 25 tattvas
//! (principles of manifestation) from Sāṃkhya philosophy.

/// The 25 Sāṃkhya tattvas as compilation stages
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Tattva {
    // === Puruṣa (Pure Consciousness) ===
    /// #1: Source code awareness
    Purusha = 1,

    // === Prakṛti (Primordial Nature) ===
    /// #2: Raw source text
    Prakriti = 2,

    // === Antaḥkaraṇa (Internal Instruments) ===
    /// #3: Buddhi (Intellect) - High-level analysis
    Buddhi = 3,
    /// #4: Ahaṃkāra (Ego) - Module/scope boundaries
    Ahamkara = 4,
    /// #5: Manas (Mind) - Control flow analysis
    Manas = 5,

    // === Jñānendriyas (Knowledge Senses) ===
    /// #6: Śrotra (Hearing) - Lexical analysis
    Shrotra = 6,
    /// #7: Tvak (Touch) - Syntax parsing
    Tvak = 7,
    /// #8: Cakṣu (Sight) - Semantic analysis
    Cakshu = 8,
    /// #9: Rasana (Taste) - Type checking
    Rasana = 9,
    /// #10: Ghrāṇa (Smell) - Optimization hints
    Ghrana = 10,

    // === Karmendriyas (Action Organs) ===
    /// #11: Vāk (Speech) - Code generation
    Vak = 11,
    /// #12: Pāṇi (Hands) - Register allocation
    Pani = 12,
    /// #13: Pāda (Feet) - Jump/branch optimization
    Pada = 13,
    /// #14: Pāyu (Excretion) - Dead code elimination
    Payu = 14,
    /// #15: Upastha (Generation) - Code emission
    Upastha = 15,

    // === Tanmātras (Subtle Elements) ===
    /// #16: Śabda (Sound) - Symbol tables
    ShabdaTanmatra = 16,
    /// #17: Sparśa (Touch) - Memory layout
    Sparsha = 17,
    /// #18: Rūpa (Form) - Data representation
    Rupa = 18,
    /// #19: Rasa (Taste) - Value semantics
    Rasa = 19,
    /// #20: Gandha (Smell) - Heap/stack decisions
    Gandha = 20,

    // === Mahābhūtas (Gross Elements) ===
    /// #21: Ākāśa (Space) - Memory allocation
    Akasha = 21,
    /// #22: Vāyu (Air) - Control flow
    Vayu = 22,
    /// #23: Tejas (Fire) - Computation
    Tejas = 23,
    /// #24: Āpas (Water) - Data flow
    Apas = 24,
    /// #25: Pṛthvī (Earth) - Final binary
    Prithvi = 25,
}

impl Tattva {
    /// Get Sanskrit name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Purusha => "पुरुष",
            Self::Prakriti => "प्रकृति",
            Self::Buddhi => "बुद्धि",
            Self::Ahamkara => "अहंकार",
            Self::Manas => "मनस्",
            Self::Shrotra => "श्रोत्र",
            Self::Tvak => "त्वक्",
            Self::Cakshu => "चक्षु",
            Self::Rasana => "रसना",
            Self::Ghrana => "घ्राण",
            Self::Vak => "वाक्",
            Self::Pani => "पाणि",
            Self::Pada => "पाद",
            Self::Payu => "पायु",
            Self::Upastha => "उपस्थ",
            Self::ShabdaTanmatra => "शब्द",
            Self::Sparsha => "स्पर्श",
            Self::Rupa => "रूप",
            Self::Rasa => "रस",
            Self::Gandha => "गन्ध",
            Self::Akasha => "आकाश",
            Self::Vayu => "वायु",
            Self::Tejas => "तेजस्",
            Self::Apas => "आपस्",
            Self::Prithvi => "पृथ्वी",
        }
    }

    /// Get English description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Purusha => "Source code awareness",
            Self::Prakriti => "Raw source text",
            Self::Buddhi => "High-level analysis",
            Self::Ahamkara => "Module/scope boundaries",
            Self::Manas => "Control flow analysis",
            Self::Shrotra => "Lexical analysis",
            Self::Tvak => "Syntax parsing",
            Self::Cakshu => "Semantic analysis",
            Self::Rasana => "Type checking",
            Self::Ghrana => "Optimization hints",
            Self::Vak => "Code generation",
            Self::Pani => "Register allocation",
            Self::Pada => "Jump optimization",
            Self::Payu => "Dead code elimination",
            Self::Upastha => "Code emission",
            Self::ShabdaTanmatra => "Symbol tables",
            Self::Sparsha => "Memory layout",
            Self::Rupa => "Data representation",
            Self::Rasa => "Value semantics",
            Self::Gandha => "Heap/stack decisions",
            Self::Akasha => "Memory allocation",
            Self::Vayu => "Control flow",
            Self::Tejas => "Computation",
            Self::Apas => "Data flow",
            Self::Prithvi => "Final binary",
        }
    }

    /// Get compilation phase
    pub fn phase(&self) -> CompilationPhase {
        match self {
            Self::Purusha | Self::Prakriti => CompilationPhase::Input,
            Self::Buddhi | Self::Ahamkara | Self::Manas => CompilationPhase::Analysis,
            Self::Shrotra | Self::Tvak | Self::Cakshu | Self::Rasana | Self::Ghrana => CompilationPhase::Frontend,
            Self::Vak | Self::Pani | Self::Pada | Self::Payu | Self::Upastha => CompilationPhase::Backend,
            Self::ShabdaTanmatra | Self::Sparsha | Self::Rupa | Self::Rasa | Self::Gandha => CompilationPhase::Representation,
            Self::Akasha | Self::Vayu | Self::Tejas | Self::Apas | Self::Prithvi => CompilationPhase::Output,
        }
    }
}

/// Compilation phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilationPhase {
    Input,
    Analysis,
    Frontend,
    Backend,
    Representation,
    Output,
}

/// Sāṃkhya-organized compilation pipeline
pub struct SamkhyaPipeline {
    /// Current tattva stage
    current: Tattva,
    /// Stage results
    results: Vec<StageResult>,
    /// Is pipeline complete
    complete: bool,
}

/// Result from a pipeline stage
#[derive(Debug, Clone)]
pub struct StageResult {
    pub tattva: Tattva,
    pub success: bool,
    pub duration_us: u64,
    pub artifacts: Vec<String>,
}

impl SamkhyaPipeline {
    pub fn new() -> Self {
        Self {
            current: Tattva::Purusha,
            results: Vec::new(),
            complete: false,
        }
    }

    /// Get current tattva
    pub fn current_tattva(&self) -> Tattva {
        self.current
    }

    /// Advance to next tattva
    pub fn advance(&mut self) -> Option<Tattva> {
        let next_value = (self.current as u8) + 1;
        if next_value > 25 {
            self.complete = true;
            return None;
        }
        // Safety: We've checked the bounds
        self.current = unsafe { std::mem::transmute(next_value) };
        Some(self.current)
    }

    /// Record stage result
    pub fn record_result(&mut self, result: StageResult) {
        self.results.push(result);
    }

    /// Is compilation complete?
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Get all results
    pub fn results(&self) -> &[StageResult] {
        &self.results
    }
}

impl Default for SamkhyaPipeline {
    fn default() -> Self {
        Self::new()
    }
}
