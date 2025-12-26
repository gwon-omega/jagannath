//! # Narayanastra - Preservation Weapon
//!
//! The weapon of Lord Vishnu (Narayana).
//! In compiler terms: Preservation pass that protects critical code.
//!
//! ## Characteristics:
//! - Preserver protects (prevents over-optimization)
//! - Marks critical sections
//! - Ensures correctness over performance
//! - Power Level: 9/10

use crate::mir::types::MirFunction;
use super::{DivyaAstra, AstraDeity, AstraResult, PowerLevel};
use super::mantra::Mantra;
use tracing::info;

/// Narayanastra - The preserver's weapon
pub struct Narayanastra {
    /// Protected regions (by name/label)
    protected_regions: Vec<String>,
}

impl Narayanastra {
    pub fn new() -> Self {
        Self {
            protected_regions: Vec::new(),
        }
    }

    pub fn protect(mut self, region: &str) -> Self {
        self.protected_regions.push(region.to_string());
        self
    }

    /// Mark critical code that should not be optimized away
    fn mark_critical(&self, _func: &mut MirFunction) -> usize {
        // Stub: Would add preservation markers
        0
    }

    /// Verify no critical code was lost
    fn verify_preservation(&self, _func: &MirFunction) -> bool {
        // Stub: Would check critical code still exists
        true
    }
}

impl DivyaAstra for Narayanastra {
    fn name(&self) -> &'static str {
        "Narayanastra"
    }

    fn sanskrit_name(&self) -> &'static str {
        "नारायणास्त्र"
    }

    fn deity(&self) -> AstraDeity {
        AstraDeity::Vishnu
    }

    fn power_level(&self) -> PowerLevel {
        9
    }

    fn invoke(&self, target: &mut MirFunction) -> AstraResult {
        info!("Invoking Narayanastra: {}", self.mantra().text());

        let marked = self.mark_critical(target);

        if !self.verify_preservation(target) {
            return AstraResult::Failed {
                reason: "Critical code was not preserved".to_string(),
            };
        }

        AstraResult::Deployed {
            power_level: self.power_level(),
            transformations: marked,
            mantra: self.mantra().text().to_string(),
        }
    }

    fn mantra(&self) -> Mantra {
        Mantra::narayanastra()
    }
}

impl Default for Narayanastra {
    fn default() -> Self {
        Self::new()
    }
}
