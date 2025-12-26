//! # Crossing Check
//!
//! Validates data crossing the Vaitarani boundary.

use super::TaintLevel;

/// Check result for a crossing attempt
pub struct CrossingCheck {
    /// Whether crossing is allowed
    pub allowed: bool,
    /// Required purification
    pub required_purification: Option<String>,
    /// Current taint level
    pub taint_level: TaintLevel,
}

impl CrossingCheck {
    /// Create a successful crossing
    pub fn allowed() -> Self {
        Self {
            allowed: true,
            required_purification: None,
            taint_level: TaintLevel::Trusted,
        }
    }

    /// Create a blocked crossing
    pub fn blocked(required: &str, level: TaintLevel) -> Self {
        Self {
            allowed: false,
            required_purification: Some(required.to_string()),
            taint_level: level,
        }
    }
}
