//! # Chandraghanta - Encryption Layer
//!
//! Layer 3: The Moon Bell
//! Data encryption and cryptographic safety.
//!
//! - Encryption at rest
//! - Encryption in transit
//! - Key management

use super::{DurgaLayer, DurgaDefense, SecurityContext};

/// Chandraghanta - Encryption
pub struct Chandraghanta {
    /// Require encryption for sensitive data
    pub require_encryption: bool,
}

impl Chandraghanta {
    pub fn new() -> Self {
        Self { require_encryption: true }
    }

    /// Check for proper encryption usage
    fn check_encryption(&self, _ctx: &SecurityContext) -> bool {
        // Stub: Would verify encryption patterns
        true
    }
}

impl DurgaLayer for Chandraghanta {
    fn name(&self) -> &'static str {
        "Chandraghanta"
    }

    fn sanskrit_name(&self) -> &'static str {
        "चन्द्रघण्टा"
    }

    fn security_function(&self) -> &'static str {
        "Encryption"
    }

    fn layer(&self) -> u8 {
        3
    }

    fn defend(&self, ctx: &SecurityContext) -> DurgaDefense {
        if self.require_encryption && !self.check_encryption(ctx) {
            return DurgaDefense::Warning {
                message: "Sensitive data may not be encrypted".to_string(),
            };
        }

        DurgaDefense::Passed
    }
}

impl Default for Chandraghanta {
    fn default() -> Self {
        Self::new()
    }
}
