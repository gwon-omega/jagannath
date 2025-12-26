//! Security Analysis - Information Flow Control
//!
//! Implements taint tracking using:
//! - -guhya (secret/tainted) - sensitive data
//! - -sarvajnika (public/clean) - safe to expose

use crate::parser::ast::*;
use std::collections::HashMap;

/// Security analyzer for information flow
pub struct SecurityAnalyzer {
    /// Security labels for variables
    labels: HashMap<String, SecurityLabel>,
    /// Information flows detected
    flows: Vec<InformationFlow>,
}

/// Security label
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLabel {
    /// Guhya - Secret/tainted (high security)
    Guhya,
    /// Sarvajnika - Public/clean (low security)
    Sarvajnika,
}

/// Information flow record
#[derive(Debug, Clone)]
pub struct InformationFlow {
    pub from: String,
    pub to: String,
    pub from_label: SecurityLabel,
    pub to_label: SecurityLabel,
}

impl SecurityAnalyzer {
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
            flows: Vec::new(),
        }
    }

    /// Analyze a function for information flow violations
    pub fn analyze_function(&mut self, func: &FunctionDef) -> Result<(), SecurityError> {
        todo!("Implement information flow analysis")
    }

    /// Set security label for a variable
    pub fn set_label(&mut self, name: String, label: SecurityLabel) {
        self.labels.insert(name, label);
    }

    /// Get security label for a variable
    pub fn get_label(&self, name: &str) -> SecurityLabel {
        self.labels.get(name).copied().unwrap_or(SecurityLabel::Sarvajnika)
    }

    /// Record an information flow
    pub fn record_flow(&mut self, from: &str, to: &str) -> Result<(), SecurityError> {
        let from_label = self.get_label(from);
        let to_label = self.get_label(to);

        self.flows.push(InformationFlow {
            from: from.to_string(),
            to: to.to_string(),
            from_label,
            to_label,
        });

        // Check for illegal flow: guhya → sarvajnika
        if from_label == SecurityLabel::Guhya && to_label == SecurityLabel::Sarvajnika {
            return Err(SecurityError::IllegalFlow {
                from: from.to_string(),
                to: to.to_string(),
                reason: "Secret data cannot flow to public output".to_string(),
            });
        }

        Ok(())
    }

    /// Check if a value can be returned/exposed
    pub fn can_expose(&self, name: &str) -> bool {
        self.get_label(name) == SecurityLabel::Sarvajnika
    }

    /// Sanitize a value (downgrade from guhya to sarvajnika)
    /// Only allowed with explicit sanitization function
    pub fn sanitize(&mut self, name: &str) {
        self.labels.insert(name.to_string(), SecurityLabel::Sarvajnika);
    }
}

/// Security error
#[derive(Debug)]
pub enum SecurityError {
    /// Illegal information flow
    IllegalFlow {
        from: String,
        to: String,
        reason: String,
    },
    /// Exposing secret data
    ExposingSecret { name: String },
}

impl Default for SecurityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
