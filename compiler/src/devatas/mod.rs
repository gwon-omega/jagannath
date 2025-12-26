//! # Devatas Module - 33 Cosmic Deities (v6.0)
//!
//! This module implements the 33 Koti Devatas as compiler subsystems.
//! Each deity group serves a specific purpose in the compilation process.
//!
//! ## The 33 Devatas:
//!
//! | Group | Count | Compiler Role |
//! |-------|-------|---------------|
//! | Adityas | 12 | Solar deities → 12 compilation phases |
//! | Rudras | 11 | Storm deities → 11 transformation passes |
//! | Vasus | 8 | Elemental deities → 8 core data structures |
//! | Ashvins | 2 | Healing deities → 2 diagnostic tools |
//!
//! ## Cosmic Order (Rta)
//!
//! The devatas work in harmony (Rta) to compile code:
//! 1. Adityas execute phases in solar order
//! 2. Rudras transform between phases
//! 3. Vasus store and manage data
//! 4. Ashvins heal/diagnose issues

pub mod adityas;
pub mod rudras;
pub mod vasus;
pub mod ashvins;
pub mod rta;

pub use adityas::{Aditya, AdityaPhase, TWELVE_ADITYAS};
pub use rudras::{Rudra, RudraPass, ELEVEN_RUDRAS};
pub use vasus::{Vasu, VasuStructure, EIGHT_VASUS};
pub use ashvins::{Ashvin, AshvinTool, TWO_ASHVINS};
pub use rta::{Rta, CosmicOrder};

/// Complete system of 33 devatas
pub struct DevataSystem {
    /// 12 Adityas - compilation phases
    pub adityas: [AdityaPhase; 12],
    /// 11 Rudras - transformation passes
    pub rudras: [RudraPass; 11],
    /// 8 Vasus - core data structures
    pub vasus: [VasuStructure; 8],
    /// 2 Ashvins - diagnostic tools
    pub ashvins: [AshvinTool; 2],
    /// Cosmic order coordinator
    pub rta: Rta,
}

impl DevataSystem {
    /// Create new system with all 33 devatas
    pub fn new() -> Self {
        Self {
            adityas: adityas::create_all(),
            rudras: rudras::create_all(),
            vasus: vasus::create_all(),
            ashvins: ashvins::create_all(),
            rta: Rta::new(),
        }
    }

    /// Get total devata count
    pub fn count(&self) -> usize {
        12 + 11 + 8 + 2 // = 33
    }

    /// Execute compilation with cosmic order
    pub fn compile_with_rta(&mut self, source: &str) -> CompilationResult {
        // Phase 1: Initialize Vasus (data structures)
        for vasu in &mut self.vasus {
            vasu.initialize();
        }

        // Phase 2: Execute 12 Aditya phases in solar order
        let mut state = CompilationState::new(source);
        for aditya in &self.adityas {
            state = aditya.execute(state);
            if state.has_errors() {
                // Ashvins attempt healing
                for ashvin in &mut self.ashvins {
                    if ashvin.can_heal(&state) {
                        state = ashvin.heal(state);
                    }
                }
            }
        }

        // Phase 3: Apply 11 Rudra transformations
        for rudra in &self.rudras {
            state = rudra.transform(state);
        }

        // Final: Check cosmic harmony
        if self.rta.verify_harmony(&state) {
            CompilationResult::Success(state)
        } else {
            CompilationResult::Failure(state.errors)
        }
    }
}

impl Default for DevataSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// State during compilation
#[derive(Debug, Clone)]
pub struct CompilationState {
    pub source: String,
    pub tokens: Vec<String>,
    pub ast: Option<String>,
    pub mir: Option<String>,
    pub output: Option<Vec<u8>>,
    pub errors: Vec<String>,
    pub current_phase: usize,
}

impl CompilationState {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: Vec::new(),
            ast: None,
            mir: None,
            output: None,
            errors: Vec::new(),
            current_phase: 0,
        }
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

/// Result of compilation
pub enum CompilationResult {
    Success(CompilationState),
    Failure(Vec<String>),
}
