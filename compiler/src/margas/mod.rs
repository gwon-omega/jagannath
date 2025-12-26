//! # Catur-Mārga (Four Paths) - Optimization Strategy System
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

/// Four spiritual paths mapped to optimization strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Marga {
    /// कर्म मार्ग - Path of Action (imperative optimization)
    /// Best for: loops, state machines, mutations, side effects
    Karma,

    /// ज्ञान मार्ग - Path of Knowledge (functional optimization)
    /// Best for: pure functions, immutability, composition, memoization
    Jnana,

    /// भक्ति मार्ग - Path of Devotion (domain-specific optimization)
    /// Best for: GPU kernels, embedded systems, DSLs, single-purpose code
    Bhakti,

    /// राज योग मार्ग - Royal Path (balanced hybrid optimization)
    /// Best for: general-purpose applications, mixed paradigms
    RajaYoga,
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
