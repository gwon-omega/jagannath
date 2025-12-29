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

// =============================================================================
// NOTE: Blanket allow directives removed per OPTIMIZATION_PLAN.md Phase 1.3
// Targeted #[allow] should be used only where justified
// =============================================================================

// ============================================================================
// Core Infrastructure
// ============================================================================
pub mod errors;

// ============================================================================
// Unified Diagnostics (Nidāna - Guṇa-based severity)
// ============================================================================
pub mod diagnostics; // Unified diagnostic trait with Sattva/Rajas/Tamas levels

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
// v8.0 - Moksha Journey (Unified Liberation Framework)
// ============================================================================
pub mod moksha; // Jīva→Ātman transformation (source→binary liberation)

// ============================================================================
// v9.0 - Jyotiṣa Śāstra (Temporal Optimization)
// ============================================================================
pub mod jyotisha; // 9 Grahas, 27 Nakṣatras, 12 Rāśis, Muhūrta timing

// ============================================================================
// Shared Traits (Lakṣaṇa - Unified Abstractions)
// ============================================================================
pub mod traits; // SanskritNamed, PhilosophicalEnum, CosmicPattern, Optimization

// ============================================================================
// Module System (Phase 4)
// ============================================================================
pub mod modules; // Khaṇḍa - Module graph, resolver, symbol tables

// ============================================================================
// Query System - Karma-Driven Incremental Computation
// ============================================================================
pub mod queries; // Memoization and incremental compilation (Karma Kosha)

// ============================================================================
// Visitor Pattern - Extensible AST/MIR Traversal (Yātrā)
// ============================================================================
pub mod visitor; // Pilgrimage through code structures

// ============================================================================
// Compiler Driver
// ============================================================================
pub mod driver;

// Re-exports for convenient access
pub use driver::options::CompilerOptions;
pub use driver::session::CompilerSession;
