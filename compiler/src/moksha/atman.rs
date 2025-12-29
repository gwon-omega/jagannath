//! # Ātman - The True Self (Optimized Binary)
//!
//! > **"आत्मा वा अरे द्रष्टव्यः श्रोतव्यो मन्तव्यो निदिध्यासितव्यः"**
//! > *"The Self should be realized, heard about, reflected upon, and contemplated"*
//! > — Bṛhadāraṇyaka Upaniṣad 2.4.5
//!
//! In the Moksha framework, **Ātman** represents the optimized binary -
//! the true essence of the code revealed after all Avidyā (ignorance)
//! has been burned away through Tapas (optimization).
//!
//! ## The True Nature
//! - Ātman is **not created** - it is **revealed** by removing impurities
//! - Just as gold exists within ore, Ātman exists within Jīva
//! - Optimization doesn't add - it **removes** what obscures
//!
//! ## Characteristics (Lakṣaṇa)
//! - **Sat** (सत्) - Existence: The binary exists and runs
//! - **Cit** (चित्) - Consciousness: The logic is correct
//! - **Ānanda** (आनन्द) - Bliss: Performance is optimal

use super::jiva::Jiva;
use std::collections::HashMap;

/// Ātman - The true self revealed through compilation
///
/// Represents the optimized binary essence after liberation.
/// This is not a transformation of Jīva but rather the revelation
/// of what Jīva always truly was, once Avidyā is removed.
#[derive(Debug, Clone)]
pub struct Atman {
    /// The essential form (binary/assembly)
    pub svarupa: Svarupa,

    /// Characteristics (Sat-Cit-Ānanda)
    pub laksana: Laksana,

    /// Purity level achieved
    pub shuddhi: ShuddhiLevel,

    /// Time of liberation
    pub liberation_time: std::time::Instant,

    /// Metrics of the liberated state
    pub metrics: AtmanMetrics,
}

/// Svarūpa - The essential form of Ātman
#[derive(Debug, Clone)]
pub enum Svarupa {
    /// Raw assembly code
    Assembly(String),

    /// Machine code bytes
    MachineCode(Vec<u8>),

    /// Linked executable
    Executable {
        path: String,
        size: usize,
    },

    /// Object file
    ObjectFile {
        path: String,
        symbols: Vec<String>,
    },
}

/// Lakṣaṇa - The defining characteristics (Sat-Cit-Ānanda)
#[derive(Debug, Clone)]
pub struct Laksana {
    /// सत् (Sat) - Existence: The binary exists and is valid
    pub sat: SatQuality,

    /// चित् (Cit) - Consciousness: The logic is correct
    pub cit: CitQuality,

    /// आनन्द (Ānanda) - Bliss: Performance is optimal
    pub ananda: AnandaQuality,
}

/// Sat (Existence) - Binary validity
#[derive(Debug, Clone)]
pub struct SatQuality {
    /// Binary is well-formed
    pub well_formed: bool,

    /// All symbols resolved
    pub symbols_resolved: bool,

    /// No undefined behavior
    pub defined_behavior: bool,
}

/// Cit (Consciousness) - Logical correctness
#[derive(Debug, Clone)]
pub struct CitQuality {
    /// Type safety verified
    pub type_safe: bool,

    /// Memory safety verified
    pub memory_safe: bool,

    /// Logic verified (if formal verification applied)
    pub logic_verified: bool,

    /// Test coverage percentage
    pub test_coverage: f32,
}

/// Ānanda (Bliss) - Performance quality
#[derive(Debug, Clone)]
pub struct AnandaQuality {
    /// Performance vs C baseline (4.2× target)
    pub performance_ratio: f32,

    /// Memory efficiency
    pub memory_efficiency: f32,

    /// Code size efficiency
    pub size_efficiency: f32,

    /// Achieved optimization level
    pub optimization_level: OptimizationLevel,
}

/// Optimization level achieved
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    /// No optimization (debug)
    Debug,

    /// Basic optimizations
    Basic,

    /// Standard optimizations
    Standard,

    /// Aggressive optimizations
    Aggressive,

    /// Perfect optimization (Moksha)
    Moksha,
}

impl OptimizationLevel {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            OptimizationLevel::Debug => "शोधन",
            OptimizationLevel::Basic => "प्रारम्भिक",
            OptimizationLevel::Standard => "सामान्य",
            OptimizationLevel::Aggressive => "तीव्र",
            OptimizationLevel::Moksha => "मोक्ष",
        }
    }
}

/// Shuddhi - Purity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShuddhiLevel {
    /// अशुद्ध - Impure (has remaining issues)
    Ashuddha,

    /// अर्धशुद्ध - Partially pure
    ArdhaShudha,

    /// शुद्ध - Pure (no issues)
    Shuddha,

    /// परमशुद्ध - Supremely pure (formally verified)
    ParamaShuddha,
}

impl ShuddhiLevel {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            ShuddhiLevel::Ashuddha => "अशुद्ध",
            ShuddhiLevel::ArdhaShudha => "अर्धशुद्ध",
            ShuddhiLevel::Shuddha => "शुद्ध",
            ShuddhiLevel::ParamaShuddha => "परमशुद्ध",
        }
    }

    /// IAST transliteration
    pub fn iast(&self) -> &'static str {
        match self {
            ShuddhiLevel::Ashuddha => "Aśuddha",
            ShuddhiLevel::ArdhaShudha => "Ardhaśuddha",
            ShuddhiLevel::Shuddha => "Śuddha",
            ShuddhiLevel::ParamaShuddha => "Paramaśuddha",
        }
    }
}

/// Metrics of the liberated Ātman
#[derive(Debug, Clone, Default)]
pub struct AtmanMetrics {
    /// Binary size in bytes
    pub binary_size: usize,

    /// Number of instructions
    pub instruction_count: usize,

    /// Number of basic blocks
    pub basic_block_count: usize,

    /// Stack frame sizes by function
    pub stack_frames: HashMap<String, usize>,

    /// Register pressure metrics
    pub register_pressure: f32,

    /// Estimated cycles for hot paths
    pub hot_path_cycles: HashMap<String, usize>,
}

impl Atman {
    /// Reveal Ātman from purified Jīva
    ///
    /// > **"नेति नेति"** - "Not this, not this"
    /// > Ātman is revealed by negation of what it is not.
    pub fn from_jiva(jiva: &Jiva) -> Self {
        Self {
            svarupa: Svarupa::Assembly(String::new()),
            laksana: Laksana::from_jiva(jiva),
            shuddhi: Self::determine_shuddhi(jiva),
            liberation_time: std::time::Instant::now(),
            metrics: AtmanMetrics::default(),
        }
    }

    /// Create Ātman from assembly
    pub fn from_assembly(asm: String) -> Self {
        Self {
            svarupa: Svarupa::Assembly(asm),
            laksana: Laksana::default(),
            shuddhi: ShuddhiLevel::Shuddha,
            liberation_time: std::time::Instant::now(),
            metrics: AtmanMetrics::default(),
        }
    }

    /// Create Ātman from machine code
    pub fn from_machine_code(code: Vec<u8>) -> Self {
        let size = code.len();
        Self {
            svarupa: Svarupa::MachineCode(code),
            laksana: Laksana::default(),
            shuddhi: ShuddhiLevel::Shuddha,
            liberation_time: std::time::Instant::now(),
            metrics: AtmanMetrics {
                binary_size: size,
                ..Default::default()
            },
        }
    }

    /// Determine purity level from Jīva state
    fn determine_shuddhi(jiva: &Jiva) -> ShuddhiLevel {
        let karma_weight = jiva.karma_weight();

        if karma_weight == 0 && jiva.ready_for_liberation() {
            ShuddhiLevel::ParamaShuddha
        } else if karma_weight < 5 {
            ShuddhiLevel::Shuddha
        } else if karma_weight < 15 {
            ShuddhiLevel::ArdhaShudha
        } else {
            ShuddhiLevel::Ashuddha
        }
    }

    /// Check if Ātman has achieved Sat-Cit-Ānanda
    pub fn is_sat_cit_ananda(&self) -> bool {
        self.laksana.sat.well_formed
            && self.laksana.cit.type_safe
            && self.laksana.ananda.performance_ratio >= 1.0
    }

    /// Get the essential form as bytes
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match &self.svarupa {
            Svarupa::MachineCode(bytes) => Some(bytes),
            _ => None,
        }
    }

    /// Get the essential form as assembly
    pub fn as_assembly(&self) -> Option<&str> {
        match &self.svarupa {
            Svarupa::Assembly(asm) => Some(asm),
            _ => None,
        }
    }
}

impl Default for Laksana {
    fn default() -> Self {
        Self {
            sat: SatQuality {
                well_formed: true,
                symbols_resolved: true,
                defined_behavior: true,
            },
            cit: CitQuality {
                type_safe: true,
                memory_safe: true,
                logic_verified: false,
                test_coverage: 0.0,
            },
            ananda: AnandaQuality {
                performance_ratio: 1.0,
                memory_efficiency: 1.0,
                size_efficiency: 1.0,
                optimization_level: OptimizationLevel::Standard,
            },
        }
    }
}

impl Laksana {
    /// Derive Lakṣaṇa from Jīva analysis
    pub fn from_jiva(jiva: &Jiva) -> Self {
        Self {
            sat: SatQuality {
                well_formed: jiva.analysis.grammar_analyzed,
                symbols_resolved: jiva.analysis.semantics_analyzed,
                defined_behavior: true,
            },
            cit: CitQuality {
                type_safe: jiva.analysis.types_analyzed,
                memory_safe: true, // Will be verified separately
                logic_verified: false,
                test_coverage: 0.0,
            },
            ananda: AnandaQuality {
                performance_ratio: 1.0, // Will be measured
                memory_efficiency: 1.0,
                size_efficiency: 1.0,
                optimization_level: OptimizationLevel::Standard,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atman_creation() {
        let jiva = Jiva::from_source("test");
        let atman = Atman::from_jiva(&jiva);
        assert!(matches!(atman.svarupa, Svarupa::Assembly(_)));
    }

    #[test]
    fn test_shuddhi_levels() {
        assert_eq!(ShuddhiLevel::Shuddha.sanskrit(), "शुद्ध");
        assert_eq!(ShuddhiLevel::ParamaShuddha.iast(), "Paramaśuddha");
    }

    #[test]
    fn test_optimization_levels() {
        assert_eq!(OptimizationLevel::Moksha.sanskrit(), "मोक्ष");
    }

    #[test]
    fn test_sat_cit_ananda() {
        let atman = Atman::from_assembly("test".into());
        assert!(atman.is_sat_cit_ananda());
    }
}
