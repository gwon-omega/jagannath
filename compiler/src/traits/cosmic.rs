//! # Cosmic Pattern Traits (Viśva Ākṛti - विश्व आकृति)
//!
//! Traits for cosmic/celestial concepts used in compilation optimization.
//!
//! ## Pattern Recognition
//! Vedic astronomy identifies recurring cosmic patterns:
//! - 9 Grahas (planetary influences)
//! - 27 Nakṣatras (lunar mansions)
//! - 12 Rāśis (zodiac signs)
//!
//! These patterns map to compilation phases and optimization timing.
//!
//! > **"यथा पिण्डे तथा ब्रह्माण्डे"**
//! > *"As in the microcosm, so in the macrocosm"*
//! — Vedic principle

use super::{PhilosophicalEnum, SanskritNamed};

/// Core trait for cosmic/celestial patterns
///
/// Implemented by types representing astronomical concepts
/// that influence compilation timing and optimization.
pub trait CosmicPattern: SanskritNamed {
    /// The cosmic domain this pattern operates in
    fn cosmic_domain(&self) -> CosmicDomain;

    /// Cycle length (in appropriate units)
    fn cycle_period(&self) -> f32;

    /// Current phase position (0.0 to 1.0)
    fn phase_position(&self, context: &impl CompilationContext) -> f32;

    /// Whether currently in strong position
    fn is_strong(&self, context: &impl CompilationContext) -> bool {
        let pos = self.phase_position(context);
        pos > 0.25 && pos < 0.75
    }
}

/// Compilation context for cosmic calculations
pub trait CompilationContext {
    /// Current compilation timestamp
    fn timestamp(&self) -> u64;

    /// Code complexity measure
    fn complexity(&self) -> f32;

    /// Resource constraints
    fn resource_budget(&self) -> ResourceBudget;

    /// Target platform characteristics
    fn target_characteristics(&self) -> TargetCharacteristics;
}

/// Domains of cosmic influence
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CosmicDomain {
    /// Solar domain (main thread, core execution)
    Solar,
    /// Lunar domain (memory, caching)
    Lunar,
    /// Planetary domain (optimization passes)
    Planetary,
    /// Stellar domain (code patterns)
    Stellar,
    /// Zodiacal domain (lifecycle phases)
    Zodiacal,
}

impl SanskritNamed for CosmicDomain {
    fn sanskrit(&self) -> &'static str {
        match self {
            Self::Solar => "सौर",
            Self::Lunar => "चान्द्र",
            Self::Planetary => "ग्रह",
            Self::Stellar => "नक्षत्र",
            Self::Zodiacal => "राशि",
        }
    }

    fn english(&self) -> &'static str {
        match self {
            Self::Solar => "Solar",
            Self::Lunar => "Lunar",
            Self::Planetary => "Planetary",
            Self::Stellar => "Stellar",
            Self::Zodiacal => "Zodiacal",
        }
    }
}

/// Trait for celestial bodies (Grahas)
///
/// Represents planetary influences on compilation.
pub trait CelestialBody: PhilosophicalEnum {
    /// Natural beneficence (positive influence)
    fn is_natural_benefic(&self) -> bool;

    /// Natural maleficence (challenging influence)
    fn is_natural_malefic(&self) -> bool {
        !self.is_natural_benefic()
    }

    /// Strength in current context (0.0 to 1.0)
    fn strength(&self, context: &impl CompilationContext) -> f32;

    /// Primary influence area
    fn primary_influence(&self) -> InfluenceArea;

    /// Secondary influence areas
    fn secondary_influences(&self) -> &'static [InfluenceArea];

    /// Associated element
    fn element(&self) -> Element;

    /// Friendly bodies
    fn friends(&self) -> &'static [Self]
    where
        Self: Sized;

    /// Enemy bodies
    fn enemies(&self) -> &'static [Self]
    where
        Self: Sized;
}

/// Areas of compilation influenced by celestial bodies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfluenceArea {
    /// Main thread execution
    MainThread,
    /// Memory management
    Memory,
    /// CPU optimization
    CpuIntensive,
    /// Type inference
    TypeSystem,
    /// Code wisdom/optimization
    Optimization,
    /// Code aesthetics
    CodeStyle,
    /// Resource limits
    Resources,
    /// Async/concurrency
    Concurrency,
    /// Dead code detection
    DeadCode,
}

impl SanskritNamed for InfluenceArea {
    fn sanskrit(&self) -> &'static str {
        match self {
            Self::MainThread => "मुख्य-सूत्र",
            Self::Memory => "स्मृति",
            Self::CpuIntensive => "गणन",
            Self::TypeSystem => "प्रकार",
            Self::Optimization => "अनुकूलन",
            Self::CodeStyle => "शैली",
            Self::Resources => "संसाधन",
            Self::Concurrency => "समानता",
            Self::DeadCode => "मृत-कूट",
        }
    }

    fn english(&self) -> &'static str {
        match self {
            Self::MainThread => "Main Thread",
            Self::Memory => "Memory",
            Self::CpuIntensive => "CPU Intensive",
            Self::TypeSystem => "Type System",
            Self::Optimization => "Optimization",
            Self::CodeStyle => "Code Style",
            Self::Resources => "Resources",
            Self::Concurrency => "Concurrency",
            Self::DeadCode => "Dead Code",
        }
    }
}

/// Classical elements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    /// Fire (Agni) - transformation, computation
    Fire,
    /// Water (Jala) - flow, memory
    Water,
    /// Earth (Pṛthvī) - stability, storage
    Earth,
    /// Air (Vāyu) - movement, communication
    Air,
    /// Space (Ākāśa) - capacity, allocation
    Space,
}

impl SanskritNamed for Element {
    fn sanskrit(&self) -> &'static str {
        match self {
            Self::Fire => "अग्नि",
            Self::Water => "जल",
            Self::Earth => "पृथ्वी",
            Self::Air => "वायु",
            Self::Space => "आकाश",
        }
    }

    fn english(&self) -> &'static str {
        match self {
            Self::Fire => "Fire",
            Self::Water => "Water",
            Self::Earth => "Earth",
            Self::Air => "Air",
            Self::Space => "Space",
        }
    }
}

/// Trait for lunar mansion patterns (Nakṣatras)
pub trait LunarMansion: PhilosophicalEnum {
    /// Ruling deity
    fn ruling_deity(&self) -> &'static str;

    /// Symbol description
    fn symbol(&self) -> &'static str;

    /// Code pattern this nakshatra favors
    fn favored_pattern(&self) -> CodePattern;

    /// Span in degrees (each nakshatra = 13°20')
    fn span_degrees(&self) -> (f32, f32) {
        let start = self.index() as f32 * 13.333333;
        let end = start + 13.333333;
        (start, end)
    }
}

/// Code patterns associated with Nakṣatras
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodePattern {
    /// Initialization patterns
    Initialization,
    /// Loop structures
    Loops,
    /// Branching logic
    Branching,
    /// Data transformation
    Transformation,
    /// Resource management
    ResourceManagement,
    /// Error handling
    ErrorHandling,
    /// Concurrency patterns
    Concurrency,
    /// I/O operations
    InputOutput,
    /// Algorithm complexity
    Algorithms,
    /// Memory layout
    MemoryLayout,
}

impl SanskritNamed for CodePattern {
    fn sanskrit(&self) -> &'static str {
        match self {
            Self::Initialization => "आरम्भ",
            Self::Loops => "आवृत्ति",
            Self::Branching => "शाखा",
            Self::Transformation => "परिवर्तन",
            Self::ResourceManagement => "संसाधन",
            Self::ErrorHandling => "दोष-निवारण",
            Self::Concurrency => "समानता",
            Self::InputOutput => "आदान-प्रदान",
            Self::Algorithms => "विधि",
            Self::MemoryLayout => "स्मृति-विन्यास",
        }
    }

    fn english(&self) -> &'static str {
        match self {
            Self::Initialization => "Initialization",
            Self::Loops => "Loops",
            Self::Branching => "Branching",
            Self::Transformation => "Transformation",
            Self::ResourceManagement => "Resource Management",
            Self::ErrorHandling => "Error Handling",
            Self::Concurrency => "Concurrency",
            Self::InputOutput => "I/O",
            Self::Algorithms => "Algorithms",
            Self::MemoryLayout => "Memory Layout",
        }
    }
}

/// Trait for zodiac signs (Rāśis)
pub trait ZodiacSign: PhilosophicalEnum {
    /// Element association
    fn element(&self) -> Element;

    /// Modality (cardinal, fixed, mutable)
    fn modality(&self) -> Modality;

    /// Compilation phase this sign governs
    fn compilation_phase(&self) -> CompilationPhase;

    /// Ruling planet
    fn ruler(&self) -> &'static str;
}

/// Modalities of zodiac signs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modality {
    /// Cardinal (initiating)
    Cardinal,
    /// Fixed (stabilizing)
    Fixed,
    /// Mutable (adapting)
    Mutable,
}

impl SanskritNamed for Modality {
    fn sanskrit(&self) -> &'static str {
        match self {
            Self::Cardinal => "चर",
            Self::Fixed => "स्थिर",
            Self::Mutable => "द्विस्वभाव",
        }
    }

    fn english(&self) -> &'static str {
        match self {
            Self::Cardinal => "Cardinal",
            Self::Fixed => "Fixed",
            Self::Mutable => "Mutable",
        }
    }
}

/// Compilation phases mapped to Rāśis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilationPhase {
    Lexing,
    Parsing,
    SemanticAnalysis,
    TypeChecking,
    MirGeneration,
    Optimization,
    CodeGeneration,
    Linking,
    Testing,
    Benchmarking,
    Documentation,
    Deployment,
}

impl SanskritNamed for CompilationPhase {
    fn sanskrit(&self) -> &'static str {
        match self {
            Self::Lexing => "पदच्छेद",
            Self::Parsing => "वाक्यविश्लेषण",
            Self::SemanticAnalysis => "अर्थविश्लेषण",
            Self::TypeChecking => "प्रकारपरीक्षा",
            Self::MirGeneration => "मध्यनिर्माण",
            Self::Optimization => "अनुकूलन",
            Self::CodeGeneration => "कूटनिर्माण",
            Self::Linking => "संयोजन",
            Self::Testing => "परीक्षण",
            Self::Benchmarking => "मापन",
            Self::Documentation => "प्रलेखन",
            Self::Deployment => "विन्यास",
        }
    }

    fn english(&self) -> &'static str {
        match self {
            Self::Lexing => "Lexing",
            Self::Parsing => "Parsing",
            Self::SemanticAnalysis => "Semantic Analysis",
            Self::TypeChecking => "Type Checking",
            Self::MirGeneration => "MIR Generation",
            Self::Optimization => "Optimization",
            Self::CodeGeneration => "Code Generation",
            Self::Linking => "Linking",
            Self::Testing => "Testing",
            Self::Benchmarking => "Benchmarking",
            Self::Documentation => "Documentation",
            Self::Deployment => "Deployment",
        }
    }
}

/// Resource budget for compilation
#[derive(Debug, Clone, Copy)]
pub struct ResourceBudget {
    /// Maximum memory in bytes
    pub max_memory: usize,
    /// Maximum CPU time in milliseconds
    pub max_cpu_time: u64,
    /// Optimization level (0-3)
    pub optimization_level: u8,
}

impl Default for ResourceBudget {
    fn default() -> Self {
        Self {
            max_memory: 1024 * 1024 * 1024, // 1GB
            max_cpu_time: 60_000,           // 60 seconds
            optimization_level: 2,
        }
    }
}

/// Target platform characteristics
#[derive(Debug, Clone)]
pub struct TargetCharacteristics {
    /// Architecture (x86_64, aarch64, etc.)
    pub arch: String,
    /// OS family
    pub os: String,
    /// Number of CPU cores
    pub cpu_cores: u32,
    /// Cache line size
    pub cache_line_size: u32,
    /// Has SIMD support
    pub has_simd: bool,
}

impl Default for TargetCharacteristics {
    fn default() -> Self {
        Self {
            arch: "x86_64".to_string(),
            os: "linux".to_string(),
            cpu_cores: 4,
            cache_line_size: 64,
            has_simd: true,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosmic_domain_naming() {
        assert_eq!(CosmicDomain::Solar.sanskrit(), "सौर");
        assert_eq!(CosmicDomain::Lunar.english(), "Lunar");
    }

    #[test]
    fn test_influence_area_naming() {
        assert_eq!(InfluenceArea::Memory.sanskrit(), "स्मृति");
        assert_eq!(InfluenceArea::Concurrency.english(), "Concurrency");
    }

    #[test]
    fn test_element_naming() {
        assert_eq!(Element::Fire.sanskrit(), "अग्नि");
        assert_eq!(Element::Space.english(), "Space");
    }

    #[test]
    fn test_compilation_phase_naming() {
        assert_eq!(CompilationPhase::Lexing.sanskrit(), "पदच्छेद");
        assert_eq!(CompilationPhase::Optimization.english(), "Optimization");
    }

    #[test]
    fn test_resource_budget_default() {
        let budget = ResourceBudget::default();
        assert_eq!(budget.optimization_level, 2);
        assert!(budget.max_memory > 0);
    }

    #[test]
    fn test_target_characteristics_default() {
        let target = TargetCharacteristics::default();
        assert!(target.has_simd);
        assert_eq!(target.cache_line_size, 64);
    }
}
