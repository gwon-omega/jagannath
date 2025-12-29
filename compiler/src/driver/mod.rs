//! Driver Module - Compiler Driver
//!
//! Main entry point for the compiler.
//!
//! Integrates Hindu philosophy into compilation:
//! - Nyāya: Type inference via 4 pramāṇas
//! - Sāṃkhya: 25-tattva pipeline
//! - Advaita: Unified memory model
//! - Mīmāṃsā: Extended inference (6 pramāṇas)
//! - Yoga: Ashtanga SDLC
//! - Vedic Math: Constant folding
//! - Tantra: SIMD optimization
//! - Āyurveda: Health monitoring

pub mod options;
pub mod philosophy_integration;
pub mod session;

pub use options::CompilerOptions;
pub use philosophy_integration::PhilosophyEngine;
pub use session::CompilerSession;

/// Compile source code
pub fn compile(source: &str, options: &CompilerOptions) -> Result<CompileResult, CompileError> {
    let mut session = CompilerSession::new(options.clone());
    session.compile(source)
}

/// Compilation result
#[derive(Debug)]
pub struct CompileResult {
    /// Generated output (assembly, binary, etc.)
    pub output: Vec<u8>,
    /// Warnings
    pub warnings: Vec<CompileWarning>,
    /// Timing information
    pub timing: CompileTiming,
}

/// Compilation error
#[derive(Debug)]
pub struct CompileError {
    pub message: String,
    pub location: Option<SourceLocation>,
    pub notes: Vec<String>,
}

/// Compilation warning
#[derive(Debug)]
pub struct CompileWarning {
    pub message: String,
    pub location: Option<SourceLocation>,
}

/// Source location
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

/// Compilation timing
#[derive(Debug, Default)]
pub struct CompileTiming {
    pub lexing_us: u64,
    pub parsing_us: u64,
    pub type_checking_us: u64,
    pub mir_building_us: u64,
    pub optimization_us: u64,
    pub codegen_us: u64,
    pub total_us: u64,
}
