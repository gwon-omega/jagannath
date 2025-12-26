//! Jagannath/Juggernaut Compiler Library
//!
//! Sanskrit morphological compiler achieving 3.2× C performance.
//!
//! # Architecture Layers
//!
//! - **v1.0**: Sanskrit Morphology (lexer, parser, semantics)
//! - **v2.0**: Assembly Backend (MIR, codegen)
//! - **v3.0**: Hindu Philosophy (Nyāya, Sāṃkhya, Advaita, etc.)
//! - **v4.0**: Yoga & Advanced (Ashtanga, Chakra, Vedic Math, etc.)
//! - **v5.0**: Garuda Purana (28 Narakas, Yama, Vaitarani, Preta)
//! - **v6.0**: Divine Cosmic (15 Astras, 33 Devatas, 9 Durgas)
//! - **v7.0**: Life Alignment (4 Margas, 4 Varnas, 3 Purusharthas)

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
pub mod codegen;
pub mod r#macro;
pub mod mir;

// ============================================================================
// v3.0 - Hindu Philosophy Layer
// ============================================================================
pub mod philosophy;

// ============================================================================
// v4.0 - Yoga & Advanced Layer
// ============================================================================
pub mod ayurveda;
pub mod buddhist_logic;
pub mod mimamsa;
pub mod tantra;
pub mod vedic_math;
pub mod yoga;

// ============================================================================
// v5.0 - Garuda Purana Layer (Error Taxonomy & Security)
// ============================================================================
pub mod garuda;

// ============================================================================
// v6.0 - Divine Cosmic Architecture Layer
// ============================================================================
pub mod astras; // 15 Divine Weapons (optimization passes)
pub mod devatas; // 33 Cosmic Deities (compiler subsystems)
pub mod nava_durga; // 9 Goddess Security Layers

// ============================================================================
// v7.0 - Life Alignment Architecture Layer
// ============================================================================
pub mod life_alignment;
pub mod margas; // 4 Spiritual Paths (optimization strategies)
pub mod purusharthas; // 3 Life Goals (optimization triangle)
pub mod varnas; // 4 Privilege Rings (security levels) // Cosmic Compiler coordination

// ============================================================================
// Compiler Driver
// ============================================================================
pub mod driver;

// Re-exports for convenient access
pub use driver::options::CompilerOptions;
pub use driver::session::CompilerSession;
