//! Tantra - Yantras for SIMD/Vectorization
//!
//! Visual/geometric patterns (yantras) for parallel computation.

// Submodules
pub mod sri_yantra;
pub mod cache_alignment;
pub mod mandala_scheduler;

// Re-exports
pub use sri_yantra::{ShriYantra, TilingConfig, MatrixOp, SimdMatrixCode};
pub use cache_alignment::{KundaliniFlow, CacheConfig, DataStructure, AccessPattern, DataTemperature};
pub use mandala_scheduler::{MandalaScheduler, Task, TaskRing, RingPriority, Schedule};

/// Yantra types for vectorization patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Yantra {
    /// Śrī Yantra - Full matrix operations
    ShriYantra,

    /// Bindu - Single element (scalar)
    Bindu,

    /// Trikoṇa - Triangle pattern (reduction)
    Trikona,

    /// Caturkoṇa - Square pattern (2D block)
    Caturkona,

    /// Aṣṭakoṇa - Octagon pattern (8-way SIMD)
    Ashtakona,

    /// Ṣoḍaśakoṇa - 16-point pattern (16-way SIMD)
    Shodashakona,

    /// Cakra - Circular pattern (ring buffer)
    Chakra,
}

impl Yantra {
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::ShriYantra => "श्रीयन्त्र",
            Self::Bindu => "बिन्दु",
            Self::Trikona => "त्रिकोण",
            Self::Caturkona => "चतुर्कोण",
            Self::Ashtakona => "अष्टकोण",
            Self::Shodashakona => "षोडशकोण",
            Self::Chakra => "चक्र",
        }
    }

    /// Get SIMD lane width
    pub fn simd_width(&self) -> usize {
        match self {
            Self::Bindu => 1,
            Self::Trikona => 3,
            Self::Caturkona => 4,
            Self::Ashtakona => 8,
            Self::Shodashakona => 16,
            Self::ShriYantra | Self::Chakra => 0, // Variable
        }
    }
}

/// Yantra optimizer for vectorization
pub struct YantraOptimizer {
    /// Target SIMD width
    target_width: usize,
    /// Available patterns
    patterns: Vec<YantraPattern>,
}

/// Vectorization pattern
#[derive(Debug, Clone)]
pub struct YantraPattern {
    pub yantra: Yantra,
    pub input_shape: Vec<usize>,
    pub output_shape: Vec<usize>,
    pub operation: YantraOperation,
}

/// Operations representable as yantras
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YantraOperation {
    /// Element-wise operation
    Map,
    /// Reduction (fold)
    Reduce,
    /// Scan (prefix sum)
    Scan,
    /// Matrix multiply
    MatMul,
    /// Convolution
    Convolve,
    /// Transpose
    Transpose,
    /// Gather
    Gather,
    /// Scatter
    Scatter,
}

impl YantraOptimizer {
    pub fn new(target_width: usize) -> Self {
        Self {
            target_width,
            patterns: Vec::new(),
        }
    }

    /// Detect vectorizable pattern
    pub fn detect_pattern(&self, operation: &str, shape: &[usize]) -> Option<Yantra> {
        // Simple heuristics for yantra selection
        match (operation, shape.len()) {
            ("map", 1) if shape[0] >= 16 => Some(Yantra::Shodashakona),
            ("map", 1) if shape[0] >= 8 => Some(Yantra::Ashtakona),
            ("map", 1) if shape[0] >= 4 => Some(Yantra::Caturkona),
            ("reduce", _) => Some(Yantra::Trikona),
            ("matmul", 2) => Some(Yantra::ShriYantra),
            ("ring", 1) => Some(Yantra::Chakra),
            _ => Some(Yantra::Bindu),
        }
    }

    /// Generate SIMD code hint
    pub fn generate_hint(&self, yantra: Yantra) -> SimdHint {
        SimdHint {
            yantra,
            width: yantra.simd_width().max(1),
            unroll_factor: match yantra {
                Yantra::Shodashakona => 4,
                Yantra::Ashtakona => 2,
                _ => 1,
            },
            prefetch: matches!(yantra, Yantra::ShriYantra | Yantra::Chakra),
        }
    }

    /// Check if pattern can be vectorized
    pub fn can_vectorize(&self, shape: &[usize]) -> bool {
        shape.iter().any(|&dim| dim >= self.target_width)
    }
}

/// SIMD code generation hint
#[derive(Debug, Clone)]
pub struct SimdHint {
    pub yantra: Yantra,
    pub width: usize,
    pub unroll_factor: usize,
    pub prefetch: bool,
}

impl Default for YantraOptimizer {
    fn default() -> Self {
        Self::new(8) // Default to 256-bit SIMD (AVX)
    }
}
