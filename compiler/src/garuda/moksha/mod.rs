//! # Moksha Module - Redemption Paths
//!
//! Moksha (मोक्ष) = "Liberation" - how to fix errors and redeem code.
//! Provides fix suggestions, redemption paths, and penance actions.

mod path;
mod penance;
mod suggestion;

pub use path::MokshaPath;
pub use penance::Penance;
pub use suggestion::FixSuggestion;

use crate::garuda::narakas::{Naraka, NarakaError};

/// Suggest a fix for a naraka error
pub fn suggest_fix(error: &NarakaError) -> MokshaPath {
    MokshaPath::for_naraka(&error.naraka)
}

/// A redemption offering for a sin (error fix)
#[derive(Debug, Clone)]
pub struct Redemption {
    /// Which naraka this redeems from
    pub naraka: Naraka,
    /// The path to moksha
    pub path: MokshaPath,
    /// Required penances
    pub penances: Vec<Penance>,
    /// Confidence this will work
    pub confidence: f64,
}

impl Redemption {
    /// Create a new redemption
    pub fn new(naraka: Naraka, path: MokshaPath) -> Self {
        Self {
            naraka,
            path,
            penances: Vec::new(),
            confidence: 1.0,
        }
    }

    /// Add a penance
    pub fn with_penance(mut self, penance: Penance) -> Self {
        self.penances.push(penance);
        self
    }

    /// Set confidence
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence;
        self
    }

    /// Get description
    pub fn describe(&self) -> String {
        let mut desc = format!(
            "Redemption from {} via {}:\n",
            self.naraka.sanskrit_name(),
            self.path.name()
        );

        for penance in &self.penances {
            desc.push_str(&format!("  - {}\n", penance.describe()));
        }

        desc
    }
}
