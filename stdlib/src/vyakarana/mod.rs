//! # Vyākaraṇa - Sanskrit Grammar Computations (व्याकरण)
//!
//! Computational implementation of Pāṇini's Aṣṭādhyāyī principles.
//!
//! > **"व्याकरणं शब्दानुशासनम्"**
//! > *"Grammar is the discipline of words"*
//! > — Patañjali, Mahābhāṣya
//!
//! ## Modules
//!
//! - [`pratyahara`] - Pratyāhāra system (compressed notation like regex)
//! - [`shiva_sutra`] - Māheśvara Sūtrāṇi (14 phoneme organization)
//! - [`sandhi`] - Sound fusion rules
//!
//! ## Innovation
//!
//! This module implements Pāṇini's 2500-year-old formal grammar system
//! as modern computational abstractions. Pāṇini's work predates:
//! - Backus-Naur Form by 2300 years
//! - Regular expressions by 2400 years
//! - Context-free grammars by 2400 years
//!
//! The Pratyāhāra system is particularly innovative - it's a compressed
//! notation for character classes that predates Unicode categories by
//! millennia but achieves similar expressiveness.
//!
//! ## References
//!
//! - Pāṇini, Aṣṭādhyāyī (c. 350 BCE)
//! - Rishi Rajpopat, "In Pāṇini We Trust" (2022) - Rule ordering breakthrough
//! - Frits Staal, "Pāṇini is the Indian Euclid"
//! - Peter Ingerman, "Pāṇini-Backus Form" proposal (1967)

pub mod pratyahara;
pub mod sandhi;
pub mod shiva_sutra;

// Re-exports
pub use pratyahara::{Pratyahara, PratyaharaSet, Varna, VarnaVarga};
pub use sandhi::{SandhiNiyama, SandhiPrakara};
pub use shiva_sutra::{MaheshvaraSutra, ShivaSutraVarna, SutraPada};
