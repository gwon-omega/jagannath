//! # Vaitarani Boundary
//!
//! Security boundary between untrusted and trusted realms.

use super::{TaintLevel, VaitaraniViolation};
use crate::garuda::taint::TaintAnalyzer;
use crate::parser::ast::Ast;
use std::collections::{HashMap, HashSet};

/// Vaitarani - Filthy river boundary enforcement
pub struct VaitaraniBoundary {
    /// Untrusted sources (external input)
    untrusted_sources: HashSet<String>,

    /// Purifiers available
    purifiers: HashMap<String, PurifierSpec>,

    /// Sensitive sinks that require clean data
    sensitive_sinks: HashSet<String>,
}

/// Specification for a purifier function
#[derive(Debug, Clone)]
pub struct PurifierSpec {
    /// Name of the purifier
    pub name: String,
    /// What taint types it cleans
    pub cleans: Vec<String>,
    /// Sanskrit name (śuddhi-kri variant)
    pub sanskrit_name: String,
}

impl VaitaraniBoundary {
    pub fn new() -> Self {
        let mut boundary = Self {
            untrusted_sources: HashSet::new(),
            purifiers: HashMap::new(),
            sensitive_sinks: HashSet::new(),
        };

        // Register default untrusted sources
        boundary.untrusted_sources.insert("http_input".to_string());
        boundary.untrusted_sources.insert("file_read".to_string());
        boundary.untrusted_sources.insert("env_var".to_string());
        boundary
            .untrusted_sources
            .insert("command_line".to_string());
        boundary
            .untrusted_sources
            .insert("network_recv".to_string());

        // Register default sensitive sinks
        boundary.sensitive_sinks.insert("sql_query".to_string());
        boundary.sensitive_sinks.insert("shell_exec".to_string());
        boundary.sensitive_sinks.insert("html_output".to_string());
        boundary.sensitive_sinks.insert("file_write".to_string());
        boundary.sensitive_sinks.insert("eval".to_string());

        // Register default purifiers
        boundary.register_purifier(PurifierSpec {
            name: "sql_escape".to_string(),
            cleans: vec!["sql_injection".to_string()],
            sanskrit_name: "śuddhi-kri-sql".to_string(),
        });
        boundary.register_purifier(PurifierSpec {
            name: "html_escape".to_string(),
            cleans: vec!["xss".to_string()],
            sanskrit_name: "śuddhi-kri-html".to_string(),
        });
        boundary.register_purifier(PurifierSpec {
            name: "shell_escape".to_string(),
            cleans: vec!["command_injection".to_string()],
            sanskrit_name: "śuddhi-kri-shell".to_string(),
        });

        boundary
    }

    /// Register a purifier
    pub fn register_purifier(&mut self, spec: PurifierSpec) {
        self.purifiers.insert(spec.name.clone(), spec);
    }

    /// Mark a source as untrusted
    pub fn mark_untrusted(&mut self, source: &str) {
        self.untrusted_sources.insert(source.to_string());
    }

    /// Check all Vaitarani crossings in code
    pub fn check_crossings(&self, _ast: &Ast, _taint: &TaintAnalyzer) -> Vec<VaitaraniViolation> {
        // TODO: Implement proper crossing detection
        Vec::new()
    }

    /// Get required purifier for a sink
    #[allow(dead_code)]
    fn get_required_purifier(&self, sink: &str) -> Option<String> {
        match sink {
            "sql_query" => Some("śuddhi-kri-sql".to_string()),
            "html_output" => Some("śuddhi-kri-html".to_string()),
            "shell_exec" => Some("śuddhi-kri-shell".to_string()),
            _ => None,
        }
    }

    /// Check if a source is untrusted
    pub fn is_untrusted(&self, source: &str) -> bool {
        self.untrusted_sources.contains(source)
    }
}

impl Default for VaitaraniBoundary {
    fn default() -> Self {
        Self::new()
    }
}
