//! Yoga Module - Advanced Systems (v4.0)
//!
//! Implements advanced philosophical systems:
//! - Ashtanga: 8-limb software development lifecycle
//! - Chitta Vritti: Mind-state management (concurrency)
//! - Chakra: 7-layer software architecture
//! - Determinism: Reproducible builds

pub mod ashtanga;
pub mod chakra;
pub mod chitta_vritti;
pub mod determinism;

// Re-exports
pub use ashtanga::{
    asana::AsanaAnalyzer, dharana::DharanaAnalyzer, dhyana::DhyanaReviewer, niyama::NiyamaChecker,
    pranayama::PranayamaManager, pratyahara::PratyaharaAnalyzer, samadhi::SamadhiDeployment,
    yama::YamaChecker, Anga, AshtangaLifecycle,
};
pub use chakra::{Chakra, ChakraArchitecture, ChakraOptimizer, OptimizationPass, OptimizerConfig};
pub use chitta_vritti::{ChittaVritti, VrittiNirodha};
pub use determinism::{DeterministicBuild, ReproducibilityChecker};
