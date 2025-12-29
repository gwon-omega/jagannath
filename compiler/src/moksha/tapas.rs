//! # Tapas - Disciplined Optimization (Austerity)
//!
//! > **"तपो विद्या च विप्राणां"**
//! > *"Austerity and knowledge are the wealth of the wise"*
//! > — Manusmṛti
//!
//! In the Moksha framework, **Tapas** represents the disciplined process
//! of optimization - burning away Avidyā (ignorance/inefficiency) through
//! focused, sustained effort.
//!
//! ## The Three Types of Tapas
//! From Bhagavad Gītā 17.14-16:
//! - **Śārīra** (शारीर) - Body: Physical optimizations (memory, CPU)
//! - **Vācika** (वाचिक) - Speech: Code clarity optimizations
//! - **Mānasa** (मानस) - Mind: Algorithmic optimizations
//!
//! ## The Three Guṇas of Tapas
//! - **Sāttvika** - Pure: Balanced, appropriate optimization
//! - **Rājasika** - Passionate: Over-optimization, premature
//! - **Tāmasika** - Dull: Optimization that causes harm
//!
//! ## Connection to Astras
//! Each Tapas pass may invoke divine Astras (v6.0) as optimization weapons.

use super::avidya::Avidya;
use super::jiva::Jiva;

/// TapasEngine - The optimization discipline engine
///
/// Performs controlled, focused optimization passes to remove Avidyā.
/// Each pass is a form of austerity that burns away impurities.
pub struct TapasEngine {
    /// Type of Tapas being performed
    pub tapas_type: TapasType,

    /// Guṇa quality of optimization
    pub guna: TapasGuna,

    /// Passes performed
    pub passes: Vec<TapasPass>,

    /// Configuration
    pub config: TapasConfig,
}

/// Types of Tapas (optimization domains)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TapasType {
    /// शारीर - Body: Physical resource optimization
    #[default]
    Sharira,

    /// वाचिक - Speech: Code clarity and readability
    Vachika,

    /// मानस - Mind: Algorithmic intelligence
    Manasa,

    /// समग्र - Integrated: All three combined
    Samagra,
}

impl TapasType {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            TapasType::Sharira => "शारीर",
            TapasType::Vachika => "वाचिक",
            TapasType::Manasa => "मानस",
            TapasType::Samagra => "समग्र",
        }
    }

    /// IAST transliteration
    pub fn iast(&self) -> &'static str {
        match self {
            TapasType::Sharira => "Śārīra",
            TapasType::Vachika => "Vācika",
            TapasType::Manasa => "Mānasa",
            TapasType::Samagra => "Samagra",
        }
    }

    /// Optimization domain
    pub fn domain(&self) -> &'static str {
        match self {
            TapasType::Sharira => "Memory, CPU, I/O optimization",
            TapasType::Vachika => "Code clarity, naming, structure",
            TapasType::Manasa => "Algorithm improvement, complexity reduction",
            TapasType::Samagra => "Full optimization across all domains",
        }
    }
}

/// Guṇa quality of Tapas (from Bhagavad Gītā)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TapasGuna {
    /// सात्त्विक - Pure: Appropriate, balanced optimization
    #[default]
    Sattvika,

    /// राजसिक - Passionate: Aggressive, potentially premature
    Rajasika,

    /// तामसिक - Dull: Harmful optimization (anti-optimization)
    Tamasika,
}

impl TapasGuna {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            TapasGuna::Sattvika => "सात्त्विक",
            TapasGuna::Rajasika => "राजसिक",
            TapasGuna::Tamasika => "तामसिक",
        }
    }

    /// Get characteristics
    pub fn characteristics(&self) -> &'static str {
        match self {
            TapasGuna::Sattvika => "Balanced, measured, appropriate to need",
            TapasGuna::Rajasika => "Aggressive, potentially premature, over-eager",
            TapasGuna::Tamasika => "Harmful, counterproductive, pessimizing",
        }
    }

    /// Get recommended use
    pub fn when_to_use(&self) -> &'static str {
        match self {
            TapasGuna::Sattvika => "Default for production code",
            TapasGuna::Rajasika => "Performance-critical hot paths",
            TapasGuna::Tamasika => "Never intentionally",
        }
    }
}

/// A single Tapas pass (optimization round)
#[derive(Debug, Clone)]
pub struct TapasPass {
    /// Pass identifier
    pub id: usize,

    /// Name of this pass
    pub name: String,

    /// Type of Tapas
    pub tapas_type: TapasType,

    /// Avidyā removed by this pass
    pub avidya_removed: Vec<String>,

    /// Duration of this pass
    pub duration: std::time::Duration,

    /// Was this pass successful?
    pub successful: bool,
}

/// Configuration for TapasEngine
#[derive(Debug, Clone)]
pub struct TapasConfig {
    /// Maximum number of passes
    pub max_passes: usize,

    /// Enable Śārīra (physical) optimizations
    pub enable_sharira: bool,

    /// Enable Vācika (clarity) optimizations
    pub enable_vachika: bool,

    /// Enable Mānasa (algorithmic) optimizations
    pub enable_manasa: bool,

    /// Guṇa quality to maintain
    pub target_guna: TapasGuna,

    /// Stop when no Avidyā removed
    pub stop_on_no_progress: bool,
}

impl Default for TapasConfig {
    fn default() -> Self {
        Self {
            max_passes: 100,
            enable_sharira: true,
            enable_vachika: true,
            enable_manasa: true,
            target_guna: TapasGuna::Sattvika,
            stop_on_no_progress: true,
        }
    }
}

/// Result of a Tapas session
#[derive(Debug)]
pub struct TapasResult {
    /// Total Avidyā removed
    pub avidya_removed: usize,

    /// Passes performed
    pub passes_performed: usize,

    /// Total duration
    pub duration: std::time::Duration,

    /// Final Guṇa quality achieved
    pub final_guna: TapasGuna,

    /// Any remaining Avidyā
    pub remaining_avidya: usize,
}

impl TapasEngine {
    /// Create new TapasEngine with default configuration
    pub fn new() -> Self {
        Self {
            tapas_type: TapasType::Samagra,
            guna: TapasGuna::Sattvika,
            passes: Vec::new(),
            config: TapasConfig::default(),
        }
    }

    /// Create with specific configuration
    pub fn with_config(config: TapasConfig) -> Self {
        Self {
            tapas_type: TapasType::Samagra,
            guna: config.target_guna,
            passes: Vec::new(),
            config,
        }
    }

    /// Set Tapas type
    pub fn with_type(mut self, tapas_type: TapasType) -> Self {
        self.tapas_type = tapas_type;
        self
    }

    /// Burn Avidyā through disciplined optimization
    ///
    /// > **"तपसा ब्रह्म विजिज्ञासस्व"**
    /// > *"Through austerity, seek to know Brahman"*
    pub fn burn_avidya(&mut self, avidyas: &mut Vec<Avidya>, jiva: &mut Jiva) {
        let start = std::time::Instant::now();
        let pass_id = self.passes.len() + 1;

        let mut removed = Vec::new();

        // Perform optimization based on type and guṇa
        match self.tapas_type {
            TapasType::Sharira => {
                if self.config.enable_sharira {
                    removed.extend(self.perform_sharira_tapas(avidyas, jiva));
                }
            }
            TapasType::Vachika => {
                if self.config.enable_vachika {
                    removed.extend(self.perform_vachika_tapas(avidyas, jiva));
                }
            }
            TapasType::Manasa => {
                if self.config.enable_manasa {
                    removed.extend(self.perform_manasa_tapas(avidyas, jiva));
                }
            }
            TapasType::Samagra => {
                // All three types
                if self.config.enable_sharira {
                    removed.extend(self.perform_sharira_tapas(avidyas, jiva));
                }
                if self.config.enable_vachika {
                    removed.extend(self.perform_vachika_tapas(avidyas, jiva));
                }
                if self.config.enable_manasa {
                    removed.extend(self.perform_manasa_tapas(avidyas, jiva));
                }
            }
        }

        // Record the pass
        self.passes.push(TapasPass {
            id: pass_id,
            name: format!("{} Tapas #{}", self.tapas_type.iast(), pass_id),
            tapas_type: self.tapas_type,
            avidya_removed: removed,
            duration: start.elapsed(),
            successful: true,
        });
    }

    /// Śārīra Tapas - Physical resource optimization
    fn perform_sharira_tapas(&self, avidyas: &mut Vec<Avidya>, _jiva: &mut Jiva) -> Vec<String> {
        let mut removed = Vec::new();

        // Remove inefficiency-type Avidyās
        avidyas.retain(|a| {
            if a.kind == super::avidya::AvidyaKind::Inefficiency && a.removable {
                removed.push(a.description.clone());
                false
            } else {
                true
            }
        });

        // Apply physical optimizations:
        // - Dead code elimination
        // - Memory layout optimization
        // - Register allocation improvement
        // - Loop unrolling where beneficial

        removed
    }

    /// Vācika Tapas - Code clarity optimization
    fn perform_vachika_tapas(&self, avidyas: &mut Vec<Avidya>, _jiva: &mut Jiva) -> Vec<String> {
        let mut removed = Vec::new();

        // Remove technical debt Avidyās
        avidyas.retain(|a| {
            if a.kind == super::avidya::AvidyaKind::TechnicalDebt && a.removable {
                removed.push(a.description.clone());
                false
            } else {
                true
            }
        });

        // Apply clarity optimizations:
        // - Simplify complex expressions
        // - Inline trivial functions
        // - Remove unnecessary abstractions

        removed
    }

    /// Mānasa Tapas - Algorithmic optimization
    fn perform_manasa_tapas(&self, avidyas: &mut Vec<Avidya>, _jiva: &mut Jiva) -> Vec<String> {
        let mut removed = Vec::new();

        // Remove anti-pattern Avidyās
        avidyas.retain(|a| {
            if a.kind == super::avidya::AvidyaKind::AntiPattern && a.removable {
                removed.push(a.description.clone());
                false
            } else {
                true
            }
        });

        // Apply algorithmic optimizations:
        // - Strength reduction
        // - Common subexpression elimination
        // - Constant folding
        // - Loop-invariant code motion

        removed
    }

    /// Get summary of Tapas session
    pub fn summary(&self) -> TapasResult {
        let total_removed: usize = self.passes.iter().map(|p| p.avidya_removed.len()).sum();
        let total_duration: std::time::Duration =
            self.passes.iter().map(|p| p.duration).sum();

        TapasResult {
            avidya_removed: total_removed,
            passes_performed: self.passes.len(),
            duration: total_duration,
            final_guna: self.guna,
            remaining_avidya: 0, // Would be calculated from actual state
        }
    }
}

impl Default for TapasEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Specific Tapas techniques (optimization passes)
pub mod techniques {
    /// Dead Code Elimination - Remove unreachable code
    pub struct DeadCodeElimination;

    /// Constant Folding - Evaluate compile-time constants
    pub struct ConstantFolding;

    /// Strength Reduction - Replace expensive ops with cheaper ones
    pub struct StrengthReduction;

    /// Loop Unrolling - Expand loops for better pipelining
    pub struct LoopUnrolling;

    /// Common Subexpression Elimination
    pub struct CommonSubexprElim;

    /// Register Allocation Improvement
    pub struct RegisterOptimization;

    /// Memory Layout Optimization
    pub struct MemoryLayoutOptim;

    /// Inlining - Inline small functions
    pub struct Inlining;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tapas_types() {
        assert_eq!(TapasType::Sharira.sanskrit(), "शारीर");
        assert_eq!(TapasType::Manasa.domain(), "Algorithm improvement, complexity reduction");
    }

    #[test]
    fn test_tapas_guna() {
        assert_eq!(TapasGuna::Sattvika.sanskrit(), "सात्त्विक");
        assert_eq!(
            TapasGuna::Rajasika.characteristics(),
            "Aggressive, potentially premature, over-eager"
        );
    }

    #[test]
    fn test_tapas_engine_creation() {
        let engine = TapasEngine::new();
        assert_eq!(engine.tapas_type, TapasType::Samagra);
        assert_eq!(engine.guna, TapasGuna::Sattvika);
        assert!(engine.passes.is_empty());
    }

    #[test]
    fn test_tapas_config() {
        let config = TapasConfig::default();
        assert_eq!(config.max_passes, 100);
        assert!(config.enable_sharira);
        assert!(config.stop_on_no_progress);
    }
}
