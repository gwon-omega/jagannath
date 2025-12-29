//! # Saṃsāra - The Cycle of Rebirth (Compilation State)
//!
//! > **"पुनरपि जननं पुनरपि मरणं पुनरपि जननी जठरे शयनम्"**
//! > *"Again birth, again death, again lying in a mother's womb"*
//! > — Bhaja Govindam
//!
//! In the Moksha framework, **Saṃsāra** represents the cycle that
//! code may go through during development - repeated compilation,
//! modification, and recompilation until liberation is achieved.
//!
//! ## Saṃskāra (Impressions)
//! Each compilation leaves impressions (cached state) that influence
//! future compilations:
//! - Incremental compilation leverages Saṃskāras
//! - Type inference caches are Saṃskāras
//! - Optimization hints are Saṃskāras
//!
//! ## Breaking the Cycle
//! Liberation (Moksha) breaks free from Saṃsāra when:
//! - No errors remain
//! - All optimizations applied
//! - Binary is stable and correct

use super::avidya::Avidya;
use super::jiva::Jiva;

/// Saṃskāra - An impression from previous compilation state
///
/// These are the cached artifacts that influence future compilations,
/// like karmic impressions influencing future lives.
#[derive(Debug, Clone)]
pub struct Samskara {
    /// Identifier for this impression
    pub id: String,

    /// Type of impression
    pub kind: SamskaraKind,

    /// The cached state
    pub state: SamskaraState,

    /// When this impression was formed
    pub formed_at: std::time::Instant,

    /// Is this impression still valid?
    pub valid: bool,

    /// Strength of the impression (affects reuse priority)
    pub strength: f32,
}

/// Types of Saṃskāras
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamskaraKind {
    /// Type inference cache
    TypeInference,

    /// Parsed AST cache
    ParsedAst,

    /// Optimized MIR cache
    OptimizedMir,

    /// Generated assembly cache
    GeneratedAsm,

    /// Symbol table cache
    SymbolTable,

    /// Dependency graph cache
    DependencyGraph,

    /// Error state cache (for incremental error recovery)
    ErrorState,
}

impl SamskaraKind {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            SamskaraKind::TypeInference => "प्रकारस्मृति",
            SamskaraKind::ParsedAst => "वृक्षस्मृति",
            SamskaraKind::OptimizedMir => "मध्यस्मृति",
            SamskaraKind::GeneratedAsm => "यन्त्रस्मृति",
            SamskaraKind::SymbolTable => "नामस्मृति",
            SamskaraKind::DependencyGraph => "सम्बन्धस्मृति",
            SamskaraKind::ErrorState => "दोषस्मृति",
        }
    }

    /// IAST transliteration
    pub fn iast(&self) -> &'static str {
        match self {
            SamskaraKind::TypeInference => "Prakārasmṛti",
            SamskaraKind::ParsedAst => "Vṛkṣasmṛti",
            SamskaraKind::OptimizedMir => "Madhyasmṛti",
            SamskaraKind::GeneratedAsm => "Yantrasmṛti",
            SamskaraKind::SymbolTable => "Nāmasmṛti",
            SamskaraKind::DependencyGraph => "Sambandhasmṛti",
            SamskaraKind::ErrorState => "Doṣasmṛti",
        }
    }
}

/// The cached state in a Saṃskāra
#[derive(Debug, Clone)]
pub enum SamskaraState {
    /// Raw bytes
    Bytes(Vec<u8>),

    /// Serialized data
    Serialized(String),

    /// Hash reference to stored state
    HashRef(String),

    /// In-memory reference (not persistent)
    InMemory,
}

impl Samskara {
    /// Create from current Jīva and Avidyā state
    pub fn from_state(jiva: &Jiva, avidyas: &[Avidya]) -> Self {
        Self {
            id: format!("samskara_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()),
            kind: SamskaraKind::ErrorState,
            state: SamskaraState::Serialized(format!(
                "jiva_state={:?}, avidya_count={}",
                jiva.state, avidyas.len()
            )),
            formed_at: std::time::Instant::now(),
            valid: true,
            strength: 1.0,
        }
    }

    /// Create type inference cache
    pub fn type_cache(id: &str, data: Vec<u8>) -> Self {
        Self {
            id: id.to_string(),
            kind: SamskaraKind::TypeInference,
            state: SamskaraState::Bytes(data),
            formed_at: std::time::Instant::now(),
            valid: true,
            strength: 1.0,
        }
    }

    /// Create AST cache
    pub fn ast_cache(id: &str, hash: String) -> Self {
        Self {
            id: id.to_string(),
            kind: SamskaraKind::ParsedAst,
            state: SamskaraState::HashRef(hash),
            formed_at: std::time::Instant::now(),
            valid: true,
            strength: 1.0,
        }
    }

    /// Invalidate this Saṃskāra
    pub fn invalidate(&mut self) {
        self.valid = false;
        self.strength = 0.0;
    }

    /// Weaken the impression (used for LRU-style eviction)
    pub fn weaken(&mut self, factor: f32) {
        self.strength *= factor;
        if self.strength < 0.1 {
            self.valid = false;
        }
    }

    /// Strengthen the impression (recently used)
    pub fn strengthen(&mut self) {
        self.strength = (self.strength * 1.5).min(1.0);
    }
}

/// The wheel of Saṃsāra - compilation cycle tracker
pub struct SamsaraWheel {
    /// Compilation cycle count
    pub cycle_count: usize,

    /// Current position in wheel (phase)
    pub current_phase: SamsaraPhase,

    /// Accumulated Saṃskāras from all cycles
    pub samskaras: Vec<Samskara>,

    /// Is the wheel still turning? (are we still compiling?)
    pub turning: bool,
}

/// Phases of the Saṃsāra wheel (compilation cycle)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamsaraPhase {
    /// Birth - Source code enters
    Janma,

    /// Growth - Parsing and analysis
    Vrddhi,

    /// Maturity - Optimization
    Purnatva,

    /// Decay - Obsolescence (code needs update)
    Kshaya,

    /// Death - Binary replacement
    Mrtyu,

    /// Between - Waiting for next cycle
    Antarabhava,
}

impl SamsaraPhase {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            SamsaraPhase::Janma => "जन्म",
            SamsaraPhase::Vrddhi => "वृद्धि",
            SamsaraPhase::Purnatva => "पूर्णत्व",
            SamsaraPhase::Kshaya => "क्षय",
            SamsaraPhase::Mrtyu => "मृत्यु",
            SamsaraPhase::Antarabhava => "अन्तरभाव",
        }
    }

    /// Compilation phase
    pub fn compilation_phase(&self) -> &'static str {
        match self {
            SamsaraPhase::Janma => "Source loading",
            SamsaraPhase::Vrddhi => "Parsing and analysis",
            SamsaraPhase::Purnatva => "Optimization",
            SamsaraPhase::Kshaya => "Code aging/deprecation",
            SamsaraPhase::Mrtyu => "Binary replacement",
            SamsaraPhase::Antarabhava => "Waiting for changes",
        }
    }

    /// Next phase in the wheel
    pub fn next(&self) -> Self {
        match self {
            SamsaraPhase::Janma => SamsaraPhase::Vrddhi,
            SamsaraPhase::Vrddhi => SamsaraPhase::Purnatva,
            SamsaraPhase::Purnatva => SamsaraPhase::Kshaya,
            SamsaraPhase::Kshaya => SamsaraPhase::Mrtyu,
            SamsaraPhase::Mrtyu => SamsaraPhase::Antarabhava,
            SamsaraPhase::Antarabhava => SamsaraPhase::Janma,
        }
    }
}

impl Default for SamsaraWheel {
    fn default() -> Self {
        Self::new()
    }
}

impl SamsaraWheel {
    /// Create new wheel
    pub fn new() -> Self {
        Self {
            cycle_count: 0,
            current_phase: SamsaraPhase::Antarabhava,
            samskaras: Vec::new(),
            turning: false,
        }
    }

    /// Start a new cycle
    pub fn begin_cycle(&mut self) {
        self.cycle_count += 1;
        self.current_phase = SamsaraPhase::Janma;
        self.turning = true;
    }

    /// Advance to next phase
    pub fn advance(&mut self) {
        self.current_phase = self.current_phase.next();
        if self.current_phase == SamsaraPhase::Antarabhava {
            self.turning = false;
        }
    }

    /// Add a Saṃskāra
    pub fn add_samskara(&mut self, samskara: Samskara) {
        self.samskaras.push(samskara);
    }

    /// Get valid Saṃskāras of a kind
    pub fn get_samskaras(&self, kind: SamskaraKind) -> Vec<&Samskara> {
        self.samskaras
            .iter()
            .filter(|s| s.kind == kind && s.valid)
            .collect()
    }

    /// Invalidate all Saṃskāras (full rebuild)
    pub fn invalidate_all(&mut self) {
        for samskara in &mut self.samskaras {
            samskara.invalidate();
        }
    }

    /// Prune weak Saṃskāras
    pub fn prune_weak(&mut self, threshold: f32) {
        self.samskaras.retain(|s| s.strength >= threshold);
    }

    /// Break free from Saṃsāra (achieve Moksha)
    pub fn achieve_moksha(&mut self) {
        self.turning = false;
        self.current_phase = SamsaraPhase::Antarabhava;
        // Saṃskāras are preserved but wheel stops
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samskara_kinds() {
        assert_eq!(SamskaraKind::TypeInference.sanskrit(), "प्रकारस्मृति");
        assert_eq!(SamskaraKind::ParsedAst.iast(), "Vṛkṣasmṛti");
    }

    #[test]
    fn test_samsara_phases() {
        assert_eq!(SamsaraPhase::Janma.sanskrit(), "जन्म");
        assert_eq!(SamsaraPhase::Janma.next(), SamsaraPhase::Vrddhi);
    }

    #[test]
    fn test_samsara_wheel() {
        let mut wheel = SamsaraWheel::new();
        assert!(!wheel.turning);

        wheel.begin_cycle();
        assert!(wheel.turning);
        assert_eq!(wheel.current_phase, SamsaraPhase::Janma);

        wheel.advance();
        assert_eq!(wheel.current_phase, SamsaraPhase::Vrddhi);
    }

    #[test]
    fn test_samskara_strength() {
        let mut samskara = Samskara::type_cache("test", vec![1, 2, 3]);
        assert_eq!(samskara.strength, 1.0);

        samskara.weaken(0.5);
        assert_eq!(samskara.strength, 0.5);

        samskara.strengthen();
        assert_eq!(samskara.strength, 0.75);
    }
}
