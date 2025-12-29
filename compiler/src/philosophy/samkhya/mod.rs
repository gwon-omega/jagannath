//! Sāṃkhya Compilation Pipeline
//!
//! Organizes compilation stages according to the 25 tattvas
//! (principles of manifestation) from Sāṃkhya philosophy.

use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};

/// The 25 Sāṃkhya tattvas as compilation stages
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    /// Get all 25 Tattvas in order
    pub fn all() -> [Tattva; 25] {
        [
            Tattva::Purusha,
            Tattva::Prakriti,
            Tattva::Buddhi,
            Tattva::Ahamkara,
            Tattva::Manas,
            Tattva::Shrotra,
            Tattva::Tvak,
            Tattva::Cakshu,
            Tattva::Rasana,
            Tattva::Ghrana,
            Tattva::Vak,
            Tattva::Pani,
            Tattva::Pada,
            Tattva::Payu,
            Tattva::Upastha,
            Tattva::ShabdaTanmatra,
            Tattva::Sparsha,
            Tattva::Rupa,
            Tattva::Rasa,
            Tattva::Gandha,
            Tattva::Akasha,
            Tattva::Vayu,
            Tattva::Tejas,
            Tattva::Apas,
            Tattva::Prithvi,
        ]
    }

    /// Get IAST transliteration
    pub fn iast(&self) -> &'static str {
        match self {
            Self::Purusha => "Puruṣa",
            Self::Prakriti => "Prakṛti",
            Self::Buddhi => "Buddhi",
            Self::Ahamkara => "Ahaṃkāra",
            Self::Manas => "Manas",
            Self::Shrotra => "Śrotra",
            Self::Tvak => "Tvak",
            Self::Cakshu => "Cakṣu",
            Self::Rasana => "Rasanā",
            Self::Ghrana => "Ghrāṇa",
            Self::Vak => "Vāk",
            Self::Pani => "Pāṇi",
            Self::Pada => "Pāda",
            Self::Payu => "Pāyu",
            Self::Upastha => "Upastha",
            Self::ShabdaTanmatra => "Śabda",
            Self::Sparsha => "Sparśa",
            Self::Rupa => "Rūpa",
            Self::Rasa => "Rasa",
            Self::Gandha => "Gandha",
            Self::Akasha => "Ākāśa",
            Self::Vayu => "Vāyu",
            Self::Tejas => "Tejas",
            Self::Apas => "Āpas",
            Self::Prithvi => "Pṛthvī",
        }
    }

    /// Get English name
    pub fn english(&self) -> &'static str {
        match self {
            Self::Purusha => "Pure Consciousness",
            Self::Prakriti => "Primordial Nature",
            Self::Buddhi => "Intellect",
            Self::Ahamkara => "Ego-Sense",
            Self::Manas => "Mind",
            Self::Shrotra => "Hearing",
            Self::Tvak => "Touch",
            Self::Cakshu => "Sight",
            Self::Rasana => "Taste",
            Self::Ghrana => "Smell",
            Self::Vak => "Speech",
            Self::Pani => "Hands",
            Self::Pada => "Feet",
            Self::Payu => "Excretion",
            Self::Upastha => "Generation",
            Self::ShabdaTanmatra => "Sound-Essence",
            Self::Sparsha => "Touch-Essence",
            Self::Rupa => "Form-Essence",
            Self::Rasa => "Taste-Essence",
            Self::Gandha => "Smell-Essence",
            Self::Akasha => "Space/Ether",
            Self::Vayu => "Air/Wind",
            Self::Tejas => "Fire/Light",
            Self::Apas => "Water",
            Self::Prithvi => "Earth",
        }
    }

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
            Self::Shrotra | Self::Tvak | Self::Cakshu | Self::Rasana | Self::Ghrana => {
                CompilationPhase::Frontend
            }
            Self::Vak | Self::Pani | Self::Pada | Self::Payu | Self::Upastha => {
                CompilationPhase::Backend
            }
            Self::ShabdaTanmatra | Self::Sparsha | Self::Rupa | Self::Rasa | Self::Gandha => {
                CompilationPhase::Representation
            }
            Self::Akasha | Self::Vayu | Self::Tejas | Self::Apas | Self::Prithvi => {
                CompilationPhase::Output
            }
        }
    }

    /// Get tattva group
    pub fn group(&self) -> TattvaGroup {
        match self {
            Self::Purusha => TattvaGroup::Purusha,
            Self::Prakriti => TattvaGroup::Prakriti,
            Self::Buddhi | Self::Ahamkara | Self::Manas => TattvaGroup::Antahkarana,
            Self::Shrotra | Self::Tvak | Self::Cakshu | Self::Rasana | Self::Ghrana => {
                TattvaGroup::Jnanendriya
            }
            Self::Vak | Self::Pani | Self::Pada | Self::Payu | Self::Upastha => {
                TattvaGroup::Karmendriya
            }
            Self::ShabdaTanmatra | Self::Sparsha | Self::Rupa | Self::Rasa | Self::Gandha => {
                TattvaGroup::Tanmatra
            }
            Self::Akasha | Self::Vayu | Self::Tejas | Self::Apas | Self::Prithvi => {
                TattvaGroup::Mahabhuta
            }
        }
    }
}

/// Tattva groupings in Sāṃkhya
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TattvaGroup {
    /// Puruṣa - Pure consciousness (1)
    Purusha,
    /// Prakṛti - Primordial nature (1)
    Prakriti,
    /// Antaḥkaraṇa - Internal organs (3)
    Antahkarana,
    /// Jñānendriyas - Knowledge senses (5)
    Jnanendriya,
    /// Karmendriyas - Action organs (5)
    Karmendriya,
    /// Tanmātras - Subtle elements (5)
    Tanmatra,
    /// Mahābhūtas - Gross elements (5)
    Mahabhuta,
}

// ============================================================================
// v10.0 Trait Implementations
// ============================================================================

impl SanskritNamed for Tattva {
    fn sanskrit(&self) -> &'static str {
        self.sanskrit_name()
    }

    fn iast(&self) -> &'static str {
        self.iast()
    }

    fn english(&self) -> &'static str {
        self.english()
    }
}

impl SanskritDescribed for Tattva {
    fn meaning(&self) -> &'static str {
        self.description()
    }

    fn explanation(&self) -> &'static str {
        match self.group() {
            TattvaGroup::Purusha => {
                "The unchanging witness consciousness - source code before compilation"
            }
            TattvaGroup::Prakriti => "The creative matrix - raw source material to be transformed",
            TattvaGroup::Antahkarana => {
                "Internal instruments of cognition - analysis and understanding"
            }
            TattvaGroup::Jnanendriya => "Sense organs that perceive - compiler frontend stages",
            TattvaGroup::Karmendriya => "Action organs that transform - compiler backend stages",
            TattvaGroup::Tanmatra => "Subtle essences - intermediate representations",
            TattvaGroup::Mahabhuta => "Gross elements - final manifestation as executable",
        }
    }

    fn mantra(&self) -> Option<&'static str> {
        Some(match self.group() {
            TattvaGroup::Purusha => "पुरुषः सन्निधिमात्रेण (By mere presence of Puruṣa)",
            TattvaGroup::Prakriti => "प्रकृतेः क्रियमाणानि (Actions done by Prakṛti)",
            TattvaGroup::Antahkarana => "बुद्धिर्ज्ञानं असंमोहः (Intellect, knowledge, clarity)",
            TattvaGroup::Jnanendriya => "इन्द्रियाणां पृथग्भावम् (Distinctness of senses)",
            TattvaGroup::Karmendriya => "कर्मण्येवाधिकारस्ते (Your right is to action alone)",
            TattvaGroup::Tanmatra => "सूक्ष्मं च तद्विज्ञेयम् (That subtle is to be known)",
            TattvaGroup::Mahabhuta => "पृथिव्यप्तेजोवायुः (Earth, water, fire, air)",
        })
    }

    fn category(&self) -> &'static str {
        "Sāṃkhya Philosophy (सांख्य तत्त्व)"
    }
}

impl PhilosophicalEnum for Tattva {
    fn all_variants() -> &'static [Self] {
        const TATTVAS: [Tattva; 25] = [
            Tattva::Purusha,
            Tattva::Prakriti,
            Tattva::Buddhi,
            Tattva::Ahamkara,
            Tattva::Manas,
            Tattva::Shrotra,
            Tattva::Tvak,
            Tattva::Cakshu,
            Tattva::Rasana,
            Tattva::Ghrana,
            Tattva::Vak,
            Tattva::Pani,
            Tattva::Pada,
            Tattva::Payu,
            Tattva::Upastha,
            Tattva::ShabdaTanmatra,
            Tattva::Sparsha,
            Tattva::Rupa,
            Tattva::Rasa,
            Tattva::Gandha,
            Tattva::Akasha,
            Tattva::Vayu,
            Tattva::Tejas,
            Tattva::Apas,
            Tattva::Prithvi,
        ];
        &TATTVAS
    }

    fn count() -> usize {
        25
    }

    fn index(&self) -> usize {
        *self as usize - 1
    }

    fn ordinal(&self) -> usize {
        *self as usize
    }

    fn next(&self) -> Self {
        let next_val = (*self as u8) % 25 + 1;
        // Safety: values 1-25 are all valid
        unsafe { std::mem::transmute(next_val) }
    }

    fn prev(&self) -> Self {
        let prev_val = if *self as u8 == 1 {
            25
        } else {
            *self as u8 - 1
        };
        // Safety: values 1-25 are all valid
        unsafe { std::mem::transmute(prev_val) }
    }

    fn from_index(index: usize) -> Option<Self> {
        if index < 25 {
            // Safety: index 0-24 maps to values 1-25
            Some(unsafe { std::mem::transmute((index + 1) as u8) })
        } else {
            None
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

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};

    #[test]
    fn test_tattva_count() {
        assert_eq!(Tattva::count(), 25);
        assert_eq!(Tattva::all_variants().len(), 25);
        assert_eq!(Tattva::all().len(), 25);
    }

    #[test]
    fn test_tattva_sanskrit_named_trait() {
        let tattva = Tattva::Purusha;
        assert_eq!(tattva.sanskrit(), "पुरुष");
        assert_eq!(tattva.iast(), "Puruṣa");
        assert_eq!(tattva.english(), "Pure Consciousness");
    }

    #[test]
    fn test_tattva_sanskrit_described_trait() {
        let tattva = Tattva::Buddhi;
        assert_eq!(tattva.meaning(), "High-level analysis");
        assert!(tattva.explanation().contains("Internal instruments"));
        assert!(tattva.mantra().is_some());
        assert_eq!(tattva.category(), "Sāṃkhya Philosophy (सांख्य तत्त्व)");
    }

    #[test]
    fn test_tattva_groups() {
        assert_eq!(Tattva::Purusha.group(), TattvaGroup::Purusha);
        assert_eq!(Tattva::Buddhi.group(), TattvaGroup::Antahkarana);
        assert_eq!(Tattva::Shrotra.group(), TattvaGroup::Jnanendriya);
        assert_eq!(Tattva::Vak.group(), TattvaGroup::Karmendriya);
        assert_eq!(Tattva::ShabdaTanmatra.group(), TattvaGroup::Tanmatra);
        assert_eq!(Tattva::Akasha.group(), TattvaGroup::Mahabhuta);
    }

    #[test]
    fn test_tattva_navigation_cycle() {
        // Forward
        assert_eq!(Tattva::Purusha.next(), Tattva::Prakriti);
        assert_eq!(Tattva::Prithvi.next(), Tattva::Purusha); // Wrap from 25 to 1

        // Backward
        assert_eq!(Tattva::Prakriti.prev(), Tattva::Purusha);
        assert_eq!(Tattva::Purusha.prev(), Tattva::Prithvi); // Wrap from 1 to 25
    }

    #[test]
    fn test_tattva_from_index() {
        assert_eq!(Tattva::from_index(0), Some(Tattva::Purusha));
        assert_eq!(Tattva::from_index(24), Some(Tattva::Prithvi));
        assert_eq!(Tattva::from_index(25), None);
    }

    #[test]
    fn test_tattva_ordinal_sequence() {
        for (i, tattva) in Tattva::all().iter().enumerate() {
            assert_eq!(tattva.ordinal(), i + 1, "Tattva {:?} ordinal mismatch", tattva);
            assert_eq!(tattva.index(), i, "Tattva {:?} index mismatch", tattva);
        }
    }

    #[test]
    fn test_tattva_compilation_phases() {
        assert_eq!(Tattva::Purusha.phase(), CompilationPhase::Input);
        assert_eq!(Tattva::Buddhi.phase(), CompilationPhase::Analysis);
        assert_eq!(Tattva::Shrotra.phase(), CompilationPhase::Frontend);
        assert_eq!(Tattva::Vak.phase(), CompilationPhase::Backend);
        assert_eq!(Tattva::ShabdaTanmatra.phase(), CompilationPhase::Representation);
        assert_eq!(Tattva::Prithvi.phase(), CompilationPhase::Output);
    }

    #[test]
    fn test_samkhya_pipeline() {
        let mut pipeline = SamkhyaPipeline::new();
        assert_eq!(pipeline.current_tattva(), Tattva::Purusha);
        assert!(!pipeline.is_complete());

        // Advance through all 25 stages
        for i in 2..=25 {
            let next = pipeline.advance();
            assert!(next.is_some(), "Stage {} should return Some", i);
        }

        // Should complete after 25th
        assert!(pipeline.advance().is_none());
        assert!(pipeline.is_complete());
    }
}

