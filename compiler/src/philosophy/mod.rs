//! Philosophy Module - Hindu Philosophy Integration (v3.0)
//!
//! Integrates Hindu philosophical systems into the compiler:
//! - Nyāya: 4-pramāṇa type inference
//! - Sāṃkhya: 25-tattva compilation pipeline
//! - Advaita: Unified memory model
//! - Pancha Kosha: 5-tier memory hierarchy
//! - Guṇa: 3 optimization modes
//! - Kāla: Time-budget compilation
//! - Karma: Dependency tracking

pub mod nyaya;
pub mod samkhya;
pub mod advaita;
pub mod pancha_kosha;
pub mod guna;
pub mod kala;
pub mod karma;

// Re-exports
pub use nyaya::{Pramana, NyayaInference};
pub use samkhya::{Tattva, SamkhyaPipeline};
pub use advaita::AdvaitaMemory;
pub use pancha_kosha::{Kosha, PanchaKoshaAllocator};
pub use guna::{Guna, GunaOptimizer};
pub use kala::{Kala, KalaScheduler};
pub use karma::{KarmaGraph, KarmaDependency};
