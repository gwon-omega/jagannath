//! # Catur-Mārga (Four Paths) - Optimization Strategy System
//!
//! "चतुर्भिर्मार्गैर्मोक्षं प्राप्नुयात्"
//! "Through four paths, one attains liberation"
//!
//! v7.0 feature implementing the four spiritual paths as optimization strategies:
//!
//! | Mārga | Sanskrit | Meaning | Optimization Focus |
//! |-------|----------|---------|---------------------|
//! | Karma | कर्म | Action | Imperative/loop optimization |
//! | Jñāna | ज्ञान | Knowledge | Functional/pure optimization |
//! | Bhakti | भक्ति | Devotion | Domain-specific optimization |
//! | Rāja Yoga | राज योग | Royal Path | Balanced hybrid optimization |
//!
//! ## Philosophy
//!
//! Just as devotees reach liberation through different spiritual paths,
//! code reaches optimal performance through different optimization strategies.
//! The compiler analyzes code characteristics to select the best path.
//!
//! Each marga corresponds to a temperament type:
//! - Karma: For the active, action-oriented (active programmers)
//! - Jñāna: For the intellectual, analytical (functional programmers)
//! - Bhakti: For the devoted, focused (domain experts)
//! - Rāja Yoga: For the balanced, integrative (system architects)

pub mod bhakti;
pub mod jnana;
pub mod karma;
pub mod path_selector;
pub mod raja_yoga;

pub use bhakti::BhaktiMarga;
pub use jnana::JnanaMarga;
pub use karma::KarmaMarga;
pub use path_selector::MargaSelector;
pub use raja_yoga::RajaYogaMarga;

use crate::mir::types::MirFunction;
use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};

/// Four spiritual paths mapped to optimization strategies
///
/// The four mārgas represent different temperaments and approaches to
/// both spiritual liberation and code optimization. Just as seekers
/// choose paths suited to their nature, the compiler selects optimization
/// strategies suited to the code's characteristics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Marga {
    /// कर्म मार्ग - Path of Action (imperative optimization)
    /// Best for: loops, state machines, mutations, side effects
    /// Bhagavad Gītā 3.3: "लोकेऽस्मिन् द्विविधा निष्ठा"
    Karma = 1,

    /// ज्ञान मार्ग - Path of Knowledge (functional optimization)
    /// Best for: pure functions, immutability, composition, memoization
    /// Bhagavad Gītā 4.33: "सर्वं कर्माखिलं पार्थ ज्ञाने परिसमाप्यते"
    Jnana = 2,

    /// भक्ति मार्ग - Path of Devotion (domain-specific optimization)
    /// Best for: GPU kernels, embedded systems, DSLs, single-purpose code
    /// Bhagavad Gītā 9.34: "मन्मना भव मद्भक्तो"
    Bhakti = 3,

    /// राज योग मार्ग - Royal Path (balanced hybrid optimization)
    /// Best for: general-purpose applications, mixed paradigms
    /// Yoga Sūtra 1.2: "योगश्चित्तवृत्तिनिरोधः"
    RajaYoga = 4,
}

impl Marga {
    /// Get Sanskrit name in Devanagari
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Marga::Karma => "कर्म मार्ग",
            Marga::Jnana => "ज्ञान मार्ग",
            Marga::Bhakti => "भक्ति मार्ग",
            Marga::RajaYoga => "राज योग मार्ग",
        }
    }

    /// Get IAST transliteration (romanized Sanskrit)
    pub fn iast(&self) -> &'static str {
        match self {
            Marga::Karma => "Karma Mārga",
            Marga::Jnana => "Jñāna Mārga",
            Marga::Bhakti => "Bhakti Mārga",
            Marga::RajaYoga => "Rāja Yoga Mārga",
        }
    }

    /// Get English name
    pub fn english(&self) -> &'static str {
        match self {
            Marga::Karma => "Path of Action",
            Marga::Jnana => "Path of Knowledge",
            Marga::Bhakti => "Path of Devotion",
            Marga::RajaYoga => "Royal Path",
        }
    }

    /// Get the associated mantra from Bhagavad Gītā or Yoga Sūtra
    pub fn mantra(&self) -> &'static str {
        match self {
            // BG 3.19: "Therefore, without attachment, perform actions always"
            Marga::Karma => "तस्मादसक्तः सततं कार्यं कर्म समाचर",
            // BG 4.33: "All actions culminate in knowledge"
            Marga::Jnana => "सर्वं कर्माखिलं पार्थ ज्ञाने परिसमाप्यते",
            // BG 9.34: "Fix your mind on Me, be devoted to Me"
            Marga::Bhakti => "मन्मना भव मद्भक्तो मद्याजी मां नमस्कुरु",
            // YS 1.2: "Yoga is the stilling of mental fluctuations"
            Marga::RajaYoga => "योगश्चित्तवृत्तिनिरोधः",
        }
    }

    /// Get all four margas in traditional order
    pub fn all() -> &'static [Marga] {
        &[Marga::Karma, Marga::Jnana, Marga::Bhakti, Marga::RajaYoga]
    }

    /// Get the temperament type suited for this marga
    pub fn temperament(&self) -> &'static str {
        match self {
            Marga::Karma => "Active/Kinesthetic",
            Marga::Jnana => "Intellectual/Analytical",
            Marga::Bhakti => "Emotional/Devoted",
            Marga::RajaYoga => "Integrative/Balanced",
        }
    }

    /// Get the optimization focus description
    pub fn focus(&self) -> &'static str {
        match self {
            Marga::Karma => "Efficient execution through action",
            Marga::Jnana => "Pure computation through knowledge",
            Marga::Bhakti => "Devoted optimization for single domain",
            Marga::RajaYoga => "Balanced wisdom combining all paths",
        }
    }

    /// What code characteristics favor this path?
    pub fn best_for(&self) -> Vec<&'static str> {
        match self {
            Marga::Karma => vec![
                "Loop-heavy code",
                "State machines",
                "Mutable data structures",
                "Side effects",
                "Imperative algorithms",
            ],
            Marga::Jnana => vec![
                "Pure functions",
                "Immutable data",
                "Function composition",
                "Mathematical operations",
                "Transformations",
            ],
            Marga::Bhakti => vec![
                "GPU kernels",
                "Embedded systems",
                "Domain-specific languages",
                "Single-purpose code",
                "Hardware-specific optimization",
            ],
            Marga::RajaYoga => vec![
                "General applications",
                "Mixed paradigms",
                "Complex systems",
                "Unknown patterns",
                "Balanced requirements",
            ],
        }
    }
}

/// Characteristics of each optimization path
#[derive(Debug, Clone)]
pub struct MargaCharacteristics {
    /// What the path focuses on
    pub focuses_on: &'static str,
    /// What code patterns this is best for
    pub best_for: &'static str,
    /// The optimization strategy
    pub strategy: OptimizationStrategy,
    /// Purity requirement level
    pub purity_requirement: PurityLevel,
    /// Speed priority
    pub speed_priority: SpeedPriority,
}

/// Optimization strategy type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationStrategy {
    /// Focus on efficient execution (Karma)
    EfficientExecution,
    /// Focus on pure computation (Jnana)
    PureComputation,
    /// Focus on domain-specific optimizations (Bhakti)
    DomainSpecific,
    /// Intelligent mix of all strategies (Raja Yoga)
    IntelligentMix,
}

/// Purity level requirement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PurityLevel {
    /// Allows mutations freely (Karma)
    Low,
    /// Requires high purity (Jnana)
    High,
    /// Varies by domain (Bhakti)
    VariesByDomain,
    /// Adapts to code (Raja Yoga)
    Adaptive,
}

/// Speed priority level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeedPriority {
    Medium,
    High,
    VeryHigh,
    Balanced,
}

/// Domain for Bhakti Marga devotion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Domain {
    /// GPU/parallel computing
    GPU,
    /// Embedded systems
    Embedded,
    /// Domain-specific language
    DSL,
    /// Network/web
    Network,
    /// AI/ML
    MachineLearning,
    /// General
    General,
}

/// Code style analysis result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeStyle {
    /// Mostly imperative with mutations
    Imperative,
    /// Mostly functional with pure functions
    Functional,
    /// Domain-specific patterns
    DomainSpecific,
    /// Mixed patterns
    Mixed,
}

/// Trait for all Marga optimizers
pub trait MargaOptimizer {
    /// Get the marga this optimizer implements
    fn marga(&self) -> Marga;

    /// Optimize a function using this path
    fn optimize(&self, func: &mut MirFunction) -> MargaResult;

    /// Check if this marga is suitable for the given function
    fn is_suitable_for(&self, func: &MirFunction) -> bool;

    /// Get the mantra for this optimization path
    fn mantra(&self) -> &'static str;
}

/// Result of Marga optimization
#[derive(Debug)]
pub struct MargaResult {
    /// Which marga was applied
    pub marga: Marga,
    /// Description of improvements made
    pub improvements: String,
    /// Whether optimization was successful
    pub success: bool,
}

impl MargaResult {
    /// Create a successful result
    pub fn success(marga: Marga, improvements: impl Into<String>) -> Self {
        Self {
            marga,
            improvements: improvements.into(),
            success: true,
        }
    }

    /// Create a failed result
    pub fn failed(marga: Marga, reason: impl Into<String>) -> Self {
        Self {
            marga,
            improvements: reason.into(),
            success: false,
        }
    }
}

/// Main coordinator for the four paths
pub struct CaturMarga {
    karma: KarmaMarga,
    jnana: JnanaMarga,
    bhakti: BhaktiMarga,
    raja_yoga: RajaYogaMarga,
    selector: MargaSelector,
}

impl Default for CaturMarga {
    fn default() -> Self {
        Self::new()
    }
}

impl CaturMarga {
    /// Create a new four-path system
    pub fn new() -> Self {
        Self {
            karma: KarmaMarga::new(),
            jnana: JnanaMarga::new(),
            bhakti: BhaktiMarga::new(Domain::General),
            raja_yoga: RajaYogaMarga::new(),
            selector: MargaSelector::new(),
        }
    }

    /// Optimize using automatic path selection
    pub fn optimize_auto(&self, func: &mut MirFunction) -> MargaResult {
        let selected = self.selector.select_path(func);
        self.optimize_with_marga(func, selected)
    }

    /// Optimize using a specific path
    pub fn optimize_with_marga(&self, func: &mut MirFunction, marga: Marga) -> MargaResult {
        match marga {
            Marga::Karma => self.karma.optimize(func),
            Marga::Jnana => self.jnana.optimize(func),
            Marga::Bhakti => self.bhakti.optimize(func),
            Marga::RajaYoga => self.raja_yoga.optimize(func),
        }
    }
}

// =============================================================================
// v10.0 Trait Implementations for Marga
// =============================================================================

impl SanskritNamed for Marga {
    fn sanskrit(&self) -> &'static str {
        self.sanskrit_name()
    }

    fn iast(&self) -> &'static str {
        Marga::iast(self)
    }

    fn english(&self) -> &'static str {
        Marga::english(self)
    }
}

impl SanskritDescribed for Marga {
    fn meaning(&self) -> &'static str {
        match self {
            Marga::Karma => "Selfless action without attachment to results",
            Marga::Jnana => "Discriminative wisdom distinguishing real from unreal",
            Marga::Bhakti => "Single-pointed devotion to the divine",
            Marga::RajaYoga => "Royal path of meditation and self-discipline",
        }
    }

    fn explanation(&self) -> &'static str {
        self.focus()
    }

    fn mantra(&self) -> Option<&'static str> {
        Some(Marga::mantra(self))
    }

    fn category(&self) -> &'static str {
        // All four paths lead to the same goal - moksha
        "Liberation Path (Mokṣa Mārga)"
    }
}

impl PhilosophicalEnum for Marga {
    fn all() -> &'static [Self] {
        Marga::all()
    }

    fn count() -> usize {
        4
    }

    fn index(&self) -> usize {
        (*self as usize) - 1 // Convert 1-based enum to 0-based index
    }

    fn ordinal(&self) -> usize {
        *self as usize // Keep 1-based for traditional ordering
    }

    fn from_index(index: usize) -> Option<Self> {
        Marga::all().get(index).copied()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marga_count() {
        assert_eq!(Marga::all().len(), 4);
        assert_eq!(Marga::count(), 4);
    }

    #[test]
    fn test_marga_sanskrit_named_trait() {
        // Test Karma Mārga
        let karma = Marga::Karma;
        assert_eq!(karma.sanskrit(), "कर्म मार्ग");
        assert_eq!(karma.iast(), "Karma Mārga");
        assert_eq!(karma.english(), "Path of Action");

        // Test Jñāna Mārga
        let jnana = Marga::Jnana;
        assert_eq!(jnana.sanskrit(), "ज्ञान मार्ग");
        assert_eq!(jnana.iast(), "Jñāna Mārga");
        assert_eq!(jnana.english(), "Path of Knowledge");

        // Test Bhakti Mārga
        let bhakti = Marga::Bhakti;
        assert_eq!(bhakti.sanskrit(), "भक्ति मार्ग");
        assert_eq!(bhakti.iast(), "Bhakti Mārga");
        assert_eq!(bhakti.english(), "Path of Devotion");

        // Test Rāja Yoga Mārga
        let raja = Marga::RajaYoga;
        assert_eq!(raja.sanskrit(), "राज योग मार्ग");
        assert_eq!(raja.iast(), "Rāja Yoga Mārga");
        assert_eq!(raja.english(), "Royal Path");
    }

    #[test]
    fn test_marga_sanskrit_described_trait() {
        use crate::traits::SanskritDescribed;

        let karma = Marga::Karma;
        assert!(karma.meaning().contains("Selfless action"));
        assert!(karma.explanation().contains("Efficient execution"));
        assert!(SanskritDescribed::mantra(&karma).is_some());
        assert_eq!(karma.category(), "Liberation Path (Mokṣa Mārga)");

        let jnana = Marga::Jnana;
        assert!(jnana.meaning().contains("Discriminative wisdom"));
        assert!(jnana.explanation().contains("Pure computation"));

        // All margas have mantras from Bhagavad Gītā or Yoga Sūtra
        for marga in Marga::all() {
            assert!(
                SanskritDescribed::mantra(marga).is_some(),
                "Marga {:?} should have a mantra",
                marga
            );
        }
    }

    #[test]
    fn test_marga_philosophical_enum_trait() {
        // Test indexing (0-based)
        assert_eq!(Marga::Karma.index(), 0);
        assert_eq!(Marga::Jnana.index(), 1);
        assert_eq!(Marga::Bhakti.index(), 2);
        assert_eq!(Marga::RajaYoga.index(), 3);

        // Test ordinal (1-based traditional)
        assert_eq!(Marga::Karma.ordinal(), 1);
        assert_eq!(Marga::Jnana.ordinal(), 2);
        assert_eq!(Marga::Bhakti.ordinal(), 3);
        assert_eq!(Marga::RajaYoga.ordinal(), 4);

        // Test from_index
        assert_eq!(Marga::from_index(0), Some(Marga::Karma));
        assert_eq!(Marga::from_index(1), Some(Marga::Jnana));
        assert_eq!(Marga::from_index(2), Some(Marga::Bhakti));
        assert_eq!(Marga::from_index(3), Some(Marga::RajaYoga));
        assert_eq!(Marga::from_index(4), None);

        // Test navigation (wraps around)
        assert_eq!(Marga::Karma.next(), Marga::Jnana);
        assert_eq!(Marga::RajaYoga.next(), Marga::Karma); // wraps to first
        assert_eq!(Marga::Karma.prev(), Marga::RajaYoga); // wraps to last
        assert_eq!(Marga::Jnana.prev(), Marga::Karma);
    }

    #[test]
    fn test_marga_mantras() {
        // Karma Mārga - Bhagavad Gītā 3.19
        let karma = Marga::Karma;
        assert!(karma.mantra().contains("कार्यं कर्म समाचर"));

        // Jñāna Mārga - Bhagavad Gītā 4.33
        let jnana = Marga::Jnana;
        assert!(jnana.mantra().contains("ज्ञाने परिसमाप्यते"));

        // Bhakti Mārga - Bhagavad Gītā 9.34
        let bhakti = Marga::Bhakti;
        assert!(bhakti.mantra().contains("मन्मना भव मद्भक्तो"));

        // Rāja Yoga - Yoga Sūtra 1.2
        let raja = Marga::RajaYoga;
        assert!(raja.mantra().contains("योगश्चित्तवृत्तिनिरोधः"));
    }

    #[test]
    fn test_marga_temperaments() {
        assert_eq!(Marga::Karma.temperament(), "Active/Kinesthetic");
        assert_eq!(Marga::Jnana.temperament(), "Intellectual/Analytical");
        assert_eq!(Marga::Bhakti.temperament(), "Emotional/Devoted");
        assert_eq!(Marga::RajaYoga.temperament(), "Integrative/Balanced");
    }

    #[test]
    fn test_marga_best_for() {
        let karma = Marga::Karma;
        assert!(karma.best_for().contains(&"Loop-heavy code"));
        assert!(karma.best_for().contains(&"State machines"));

        let jnana = Marga::Jnana;
        assert!(jnana.best_for().contains(&"Pure functions"));
        assert!(jnana.best_for().contains(&"Immutable data"));

        let bhakti = Marga::Bhakti;
        assert!(bhakti.best_for().contains(&"GPU kernels"));
        assert!(bhakti.best_for().contains(&"Embedded systems"));

        let raja = Marga::RajaYoga;
        assert!(raja.best_for().contains(&"General applications"));
        assert!(raja.best_for().contains(&"Mixed paradigms"));
    }

    #[test]
    fn test_marga_all_lead_to_moksha() {
        // All four paths lead to the same goal - liberation (moksha)
        for marga in Marga::all() {
            assert_eq!(
                marga.category(),
                "Liberation Path (Mokṣa Mārga)",
                "All margas should lead to moksha"
            );
        }
    }

    #[test]
    fn test_marga_result_constructors() {
        let success = MargaResult::success(Marga::Karma, "Optimized loops");
        assert!(success.success);
        assert_eq!(success.marga, Marga::Karma);
        assert_eq!(success.improvements, "Optimized loops");

        let failed = MargaResult::failed(Marga::Jnana, "No pure functions found");
        assert!(!failed.success);
        assert_eq!(failed.marga, Marga::Jnana);
        assert_eq!(failed.improvements, "No pure functions found");
    }
}
