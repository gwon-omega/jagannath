//! # Garuda Purana Module (v5.0)
//!
//! Forensic error classification and security framework based on
//! Garuda Purana's 28 Narakas (Hells).
//!
//! ## Architecture
//!
//! - **Narakas**: 28 error types mapped to specific code violations
//! - **Yama**: Static analyzer that judges code and assigns violations
//! - **Chitragupta**: Audit trail and build record keeping
//! - **Vaitarani**: Security boundary enforcement (untrusted→trusted)
//! - **Preta**: Resource leak detection (hungry ghost state)
//! - **Taint**: Sin propagation tracking through data flow
//! - **Moksha**: Redemption paths and fix suggestions
//! - **Dataflow**: Generic dataflow analysis framework (prāṇa-vāha)

pub mod chitragupta;
pub mod dataflow;
pub mod moksha;
pub mod narakas;
pub mod preta;
pub mod taint;
pub mod vaitarani;
pub mod yama;

pub use chitragupta::{ChitraguptaRecords, KarmaRecord};
pub use moksha::{MokshaPath, Penance};
pub use narakas::{Duration, Naraka, NarakaError, Severity};
pub use preta::{PretaDetector, PretaViolation};
pub use taint::{TaintAnalyzer, TaintSource};
pub use vaitarani::{TaintLevel, VaitaraniBoundary};
pub use yama::{Violation, ViolationKind, YamaDharmaraja, Yamaduta};

use crate::parser::ast::Ast;

/// Main Garuda analyzer - integrates all subsystems
pub struct GarudaAnalyzer {
    /// Yama Dharmaraja - the judge
    pub yama: YamaDharmaraja,

    /// Chitragupta - record keeper
    pub chitragupta: ChitraguptaRecords,

    /// Vaitarani boundary checker
    pub vaitarani: VaitaraniBoundary,

    /// Preta (leak) detector
    pub preta: PretaDetector,

    /// Taint analyzer
    pub taint: TaintAnalyzer,
}

impl GarudaAnalyzer {
    /// Create a new Garuda analyzer
    pub fn new() -> Self {
        Self {
            yama: YamaDharmaraja::new(),
            chitragupta: ChitraguptaRecords::new(),
            vaitarani: VaitaraniBoundary::new(),
            preta: PretaDetector::new(),
            taint: TaintAnalyzer::new(),
        }
    }

    /// Analyze code for all violations
    pub fn analyze(&mut self, ast: &Ast) -> Vec<NarakaError> {
        let mut errors = Vec::new();

        // Phase 1: Yama judges code violations
        let violations = self.yama.judge(ast);
        for violation in &violations {
            let naraka = self.yama.determine_naraka(violation);
            let error = NarakaError::from_violation(violation, naraka);

            // Record in Chitragupta's ledger
            self.chitragupta.record(&error);

            errors.push(error);
        }

        // Phase 2: Detect Preta (resource leaks)
        let pretas = self.preta.detect(ast);
        for preta in pretas {
            // Convert Ghost to NarakaError directly (Ghost contains relevant info)
            let error = NarakaError::from_ghost(&preta);
            self.chitragupta.record(&error);
            errors.push(error);
        }

        // Phase 3: Check Vaitarani crossings (security boundaries)
        let boundary_violations = self.vaitarani.check_crossings(ast, &self.taint);
        for violation in boundary_violations {
            let error = NarakaError::from_vaitarani(&violation);
            self.chitragupta.record(&error);
            errors.push(error);
        }

        errors
    }

    /// Get moksha (redemption) paths for all errors
    pub fn suggest_redemption(&self, errors: &[NarakaError]) -> Vec<MokshaPath> {
        errors.iter().map(|e| moksha::suggest_fix(e)).collect()
    }
}

impl Default for GarudaAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
