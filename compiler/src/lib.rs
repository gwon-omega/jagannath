//! Jagannath/Juggernaut Compiler Library
//!
//! Sanskrit morphological compiler achieving 2.7× C performance.
//!
//! # Architecture Layers
//!
//! - **v1.0**: Sanskrit Morphology (lexer, parser, semantics)
//! - **v2.0**: Assembly Backend (MIR, codegen)
//! - **v3.0**: Hindu Philosophy (Nyāya, Sāṃkhya, Advaita, etc.)
//! - **v4.0**: Yoga & Advanced (Ashtanga, Chakra, Vedic Math, etc.)

// ============================================================================
// Core Infrastructure
// ============================================================================
pub mod errors;

// ============================================================================
// v1.0 - Sanskrit Morphology Layer
// ============================================================================
pub mod lexer;
pub mod parser;
pub mod semantics;

// ============================================================================
// v2.0 - Assembly Backend Layer
// ============================================================================
pub mod mir;
pub mod codegen;
pub mod r#macro;

// ============================================================================
// v3.0 - Hindu Philosophy Layer
// ============================================================================
pub mod philosophy;

// ============================================================================
// v4.0 - Yoga & Advanced Layer
// ============================================================================
pub mod yoga;
pub mod vedic_math;
pub mod mimamsa;
pub mod ayurveda;
pub mod tantra;
pub mod buddhist_logic;

// ============================================================================
// Compiler Driver
// ============================================================================
pub mod driver;

// Re-exports for convenient access
pub use driver::session::Session;
pub use driver::options::CompilerOptions;
