//! # Taint Analyzer
//!
//! Main taint analysis engine.

use super::{Taint, TaintKind, TaintSource};
use crate::garuda::vaitarani::TaintLevel;
use crate::parser::ast::{Ast, Expr};
use std::collections::HashMap;

/// Taint analyzer - tracks data flow tainting
pub struct TaintAnalyzer {
    /// Taint status of variables
    taints: HashMap<String, Taint>,
    /// Known taint sources
    sources: Vec<TaintSource>,
}

impl TaintAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            taints: HashMap::new(),
            sources: Vec::new(),
        };

        // Register default taint sources
        analyzer.register_source(TaintSource {
            name: "http_request".to_string(),
            kind: TaintKind::UserInput,
            level: TaintLevel::Untrusted,
            location: None,
        });
        analyzer.register_source(TaintSource {
            name: "env_var".to_string(),
            kind: TaintKind::UserInput,
            level: TaintLevel::Untrusted,
            location: None,
        });
        analyzer.register_source(TaintSource {
            name: "file_contents".to_string(),
            kind: TaintKind::Unchecked,
            level: TaintLevel::PartiallyTrusted,
            location: None,
        });

        analyzer
    }

    /// Register a taint source
    pub fn register_source(&mut self, source: TaintSource) {
        self.sources.push(source);
    }

    /// Mark a variable as tainted
    pub fn mark_tainted(&mut self, var: &str, taint: Taint) {
        self.taints.insert(var.to_string(), taint);
    }

    /// Get taint level for an expression
    pub fn get_level(&self, _expr: &Expr) -> TaintLevel {
        // TODO: Implement proper expression analysis
        TaintLevel::Trusted
    }

    /// Get taint source for an expression
    pub fn get_taint_source(&self, _expr: &Expr) -> Option<String> {
        // TODO: Implement proper source tracking
        None
    }

    /// Analyze AST for taint flow
    pub fn analyze(&mut self, _ast: &Ast) {
        // TODO: Implement full taint flow analysis
    }

    /// Check if a name represents an external source
    #[allow(dead_code)]
    fn is_external_source(&self, name: &str) -> bool {
        let external_patterns = [
            "request", "input", "param", "query", "body", "args", "env", "http", "socket", "file",
        ];

        let name_lower = name.to_lowercase();
        external_patterns.iter().any(|p| name_lower.contains(p))
    }

    /// Check if variable is tainted
    pub fn is_tainted(&self, var: &str) -> bool {
        self.taints.contains_key(var)
    }

    /// Get all tainted variables
    pub fn tainted_variables(&self) -> Vec<&str> {
        self.taints.keys().map(|s| s.as_str()).collect()
    }

    /// Clear taint from variable (after sanitization)
    pub fn clear_taint(&mut self, var: &str) {
        self.taints.remove(var);
    }
}

impl Default for TaintAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
