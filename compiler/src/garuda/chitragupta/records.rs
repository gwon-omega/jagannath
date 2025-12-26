//! # Chitragupta Records
//!
//! Complete build and violation records.

use crate::garuda::narakas::NarakaError;
use super::karma_ledger::{KarmaRecord, KarmaLedger};
use super::audit_trail::AuditTrail;
use std::time::{SystemTime, Duration};

/// Chitragupta - Keeper of all records
pub struct ChitraguptaRecords {
    /// Build session ID
    pub session_id: String,
    
    /// Start time
    pub start_time: SystemTime,
    
    /// Karma ledger (all violations)
    pub karma_ledger: KarmaLedger,
    
    /// Audit trail
    pub audit_trail: AuditTrail,
    
    /// Build outcome
    pub outcome: Option<BuildOutcome>,
}

/// Outcome of a build
#[derive(Debug, Clone)]
pub enum BuildOutcome {
    /// Build succeeded (moksha achieved)
    Moksha,
    /// Build failed with errors
    Naraka { error_count: usize },
    /// Build succeeded with warnings
    Samsara { warning_count: usize },
}

impl ChitraguptaRecords {
    /// Create new records for a build session
    pub fn new() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            start_time: SystemTime::now(),
            karma_ledger: KarmaLedger::new(),
            audit_trail: AuditTrail::new(),
            outcome: None,
        }
    }
    
    /// Record a violation
    pub fn record(&mut self, error: &NarakaError) {
        let record = KarmaRecord {
            timestamp: SystemTime::now(),
            naraka: error.naraka,
            location: error.location.clone(),
            sin: error.sin.clone(),
            penance: error.penance.clone(),
        };
        
        self.karma_ledger.add(record.clone());
        self.audit_trail.log_violation(&record);
    }
    
    /// Finalize the build
    pub fn finalize(&mut self) {
        let errors = self.karma_ledger.error_count();
        let warnings = self.karma_ledger.warning_count();
        
        self.outcome = Some(if errors > 0 {
            BuildOutcome::Naraka { error_count: errors }
        } else if warnings > 0 {
            BuildOutcome::Samsara { warning_count: warnings }
        } else {
            BuildOutcome::Moksha
        });
        
        self.audit_trail.log_build_complete(self.outcome.as_ref().unwrap());
    }
    
    /// Get build duration
    pub fn duration(&self) -> Duration {
        SystemTime::now()
            .duration_since(self.start_time)
            .unwrap_or_default()
    }
    
    /// Generate summary report
    pub fn summary(&self) -> String {
        let mut report = String::new();
        
        report.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
        report.push_str("║           CHITRAGUPTA BUILD RECORD                            ║\n");
        report.push_str("╠═══════════════════════════════════════════════════════════════╣\n");
        report.push_str(&format!("║ Session: {}\n", self.session_id));
        report.push_str(&format!("║ Duration: {:?}\n", self.duration()));
        report.push_str(&format!("║ Violations: {}\n", self.karma_ledger.len()));
        report.push_str(&format!("║ Errors: {}\n", self.karma_ledger.error_count()));
        report.push_str(&format!("║ Warnings: {}\n", self.karma_ledger.warning_count()));
        
        if let Some(outcome) = &self.outcome {
            report.push_str(&format!("║ Outcome: {:?}\n", outcome));
        }
        
        report.push_str("╚═══════════════════════════════════════════════════════════════╝\n");
        
        report
    }
}

impl Default for ChitraguptaRecords {
    fn default() -> Self {
        Self::new()
    }
}

// Simple UUID generation (in practice, use uuid crate)
mod uuid {
    pub struct Uuid;
    impl Uuid {
        pub fn new_v4() -> Self { Uuid }
        pub fn to_string(&self) -> String {
            format!("{:016x}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos())
        }
    }
}
