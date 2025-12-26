//! # Katyayani - Input Validation Layer
//!
//! Layer 6: The Daughter of Katya
//! Input sanitization and validation.
//!
//! - SQL injection prevention
//! - XSS prevention
//! - Command injection prevention

use super::{DurgaLayer, DurgaDefense, SecurityContext, VulnerabilityKind};

/// Katyayani - Input Validation
pub struct Katyayani {
    /// Enable strict validation
    pub strict_validation: bool,
}

impl Katyayani {
    pub fn new() -> Self {
        Self { strict_validation: true }
    }

    /// Check for input validation patterns
    fn check_input_validation(&self, ctx: &SecurityContext) -> bool {
        // Check for injection vulnerabilities
        !ctx.vulnerabilities.iter().any(|v| v.kind == VulnerabilityKind::Injection)
    }
}

impl DurgaLayer for Katyayani {
    fn name(&self) -> &'static str {
        "Katyayani"
    }

    fn sanskrit_name(&self) -> &'static str {
        "कात्यायनी"
    }

    fn security_function(&self) -> &'static str {
        "Input Validation"
    }

    fn layer(&self) -> u8 {
        6
    }

    fn defend(&self, ctx: &SecurityContext) -> DurgaDefense {
        if self.strict_validation && !self.check_input_validation(ctx) {
            return DurgaDefense::Blocked {
                reason: "Input validation vulnerability detected".to_string(),
            };
        }

        DurgaDefense::Passed
    }
}

impl Default for Katyayani {
    fn default() -> Self {
        Self::new()
    }
}
