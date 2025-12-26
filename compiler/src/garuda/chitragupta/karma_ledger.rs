//! # Karma Ledger
//!
//! Tracks all violations (bad karma) throughout the build.

use crate::garuda::narakas::{Naraka, Severity};
use crate::errors::Span;
use std::time::SystemTime;

/// A record of a single karma event (violation)
#[derive(Debug, Clone)]
pub struct KarmaRecord {
    /// When the violation occurred
    pub timestamp: SystemTime,
    /// Which Naraka this belongs to
    pub naraka: Naraka,
    /// Location in source
    pub location: Span,
    /// The sin (what was done wrong)
    pub sin: String,
    /// The penance (how to fix)
    pub penance: String,
}

/// Complete ledger of all karma
pub struct KarmaLedger {
    /// All records
    records: Vec<KarmaRecord>,
}

impl KarmaLedger {
    pub fn new() -> Self {
        Self { records: Vec::new() }
    }

    /// Add a karma record
    pub fn add(&mut self, record: KarmaRecord) {
        self.records.push(record);
    }

    /// Get all records
    pub fn records(&self) -> &[KarmaRecord] {
        &self.records
    }

    /// Count of records
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Is ledger empty?
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Count errors (Critical + Error severity)
    pub fn error_count(&self) -> usize {
        self.records.iter()
            .filter(|r| matches!(r.naraka.severity(), Severity::Critical | Severity::Error))
            .count()
    }

    /// Count warnings
    pub fn warning_count(&self) -> usize {
        self.records.iter()
            .filter(|r| matches!(r.naraka.severity(), Severity::Warning))
            .count()
    }

    /// Group by Naraka
    pub fn by_naraka(&self) -> std::collections::HashMap<Naraka, Vec<&KarmaRecord>> {
        let mut map = std::collections::HashMap::new();
        for record in &self.records {
            map.entry(record.naraka).or_insert_with(Vec::new).push(record);
        }
        map
    }

    /// Calculate karma score (0.0 = worst, 1.0 = best)
    pub fn karma_score(&self) -> f64 {
        if self.records.is_empty() {
            return 1.0; // Perfect karma (no violations)
        }

        // Weight by severity
        let weighted_sum: f64 = self.records.iter()
            .map(|r| match r.naraka.severity() {
                Severity::Critical => 10.0,
                Severity::Error => 5.0,
                Severity::Warning => 1.0,
                Severity::Hint => 0.1,
            })
            .sum();

        // Score decreases with more/worse violations
        1.0 / (1.0 + weighted_sum / 10.0)
    }
}

impl Default for KarmaLedger {
    fn default() -> Self {
        Self::new()
    }
}
