//! # Audit Trail
//!
//! Forensic logging of all build events.

use super::karma_ledger::KarmaRecord;
use super::records::BuildOutcome;
use std::time::SystemTime;

/// Event types in the audit trail
#[derive(Debug, Clone)]
pub enum AuditEvent {
    /// Build started
    BuildStart { timestamp: SystemTime },
    /// File compiled
    FileCompiled { path: String, timestamp: SystemTime },
    /// Violation detected
    ViolationDetected { record: KarmaRecord },
    /// Build completed
    BuildComplete { outcome: BuildOutcome, timestamp: SystemTime },
}

/// Complete audit trail
pub struct AuditTrail {
    events: Vec<AuditEvent>,
}

impl AuditTrail {
    pub fn new() -> Self {
        let mut trail = Self { events: Vec::new() };
        trail.events.push(AuditEvent::BuildStart {
            timestamp: SystemTime::now(),
        });
        trail
    }

    /// Log a file compilation
    pub fn log_file(&mut self, path: &str) {
        self.events.push(AuditEvent::FileCompiled {
            path: path.to_string(),
            timestamp: SystemTime::now(),
        });
    }

    /// Log a violation
    pub fn log_violation(&mut self, record: &KarmaRecord) {
        self.events.push(AuditEvent::ViolationDetected {
            record: record.clone(),
        });
    }

    /// Log build completion
    pub fn log_build_complete(&mut self, outcome: &BuildOutcome) {
        self.events.push(AuditEvent::BuildComplete {
            outcome: outcome.clone(),
            timestamp: SystemTime::now(),
        });
    }

    /// Get all events
    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }

    /// Export to JSON (for compliance)
    pub fn to_json(&self) -> String {
        // Simplified JSON export
        let mut json = String::from("{\n  \"events\": [\n");

        for (i, event) in self.events.iter().enumerate() {
            if i > 0 { json.push_str(",\n"); }
            json.push_str(&format!("    {:?}", event));
        }

        json.push_str("\n  ]\n}");
        json
    }
}

impl Default for AuditTrail {
    fn default() -> Self {
        Self::new()
    }
}
