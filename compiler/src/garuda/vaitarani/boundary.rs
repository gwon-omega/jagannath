//! # Vaitarani Boundary
//!
//! Security boundary between untrusted and trusted realms.
//!
//! # Sanskrit Foundation
//! Vaitarani (वैतरणी) is the river of blood and pus in Garuda Purana
//! that souls must cross to reach Yama's realm. Only the righteous
//! (purified data) can cross safely.
//!
//! # Security Model
//! All external input is "tainted" (impure). Before reaching sensitive
//! sinks (SQL, shell, file operations), data must be "purified" (sanitized).
//!
//! # Algorithm
//! Uses forward dataflow taint analysis:
//! - GEN: External sources generate taint
//! - KILL: Sanitizers remove specific taints
//! - SINK CHECK: Error if tainted data reaches sink without purification

use super::VaitaraniViolation;
use crate::garuda::taint::TaintAnalyzer;
use crate::parser::ast::{Ast, Block, Expr, FunctionDef, Item, Stmt};
use std::collections::{HashMap, HashSet};

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

/// Taint information for tracking
#[derive(Debug, Clone, PartialEq)]
struct TaintInfo {
    /// Variable name
    variable: String,
    /// Source of taint
    source: String,
    /// Type of taint (for targeted sanitization)
    taint_type: String,
}

/// Vaitarani - Filthy river boundary enforcement
///
/// Implements taint analysis to ensure external input is sanitized
/// before reaching sensitive operations.
pub struct VaitaraniBoundary {
    /// Untrusted sources (external input)
    untrusted_sources: HashSet<String>,

    /// Purifiers available
    purifiers: HashMap<String, PurifierSpec>,

    /// Sensitive sinks that require clean data
    sensitive_sinks: HashMap<String, Vec<String>>, // sink -> required purification

    /// Current taint state
    tainted: HashMap<String, TaintInfo>,

    /// Detected violations
    violations: Vec<VaitaraniViolation>,
}

impl VaitaraniBoundary {
    pub fn new() -> Self {
        let mut boundary = Self {
            untrusted_sources: HashSet::new(),
            purifiers: HashMap::new(),
            sensitive_sinks: HashMap::new(),
            tainted: HashMap::new(),
            violations: Vec::new(),
        };

        // Register default untrusted sources
        // Sanskrit names
        boundary
            .untrusted_sources
            .insert("bāhya_praveśa".to_string()); // external input
        boundary.untrusted_sources.insert("jāla_prāpti".to_string()); // network receive
        boundary
            .untrusted_sources
            .insert("paryāvaraṇa_paṭha".to_string()); // env read
        boundary.untrusted_sources.insert("koṣa_paṭha".to_string()); // file read

        // English names
        boundary.untrusted_sources.insert("http_input".to_string());
        boundary.untrusted_sources.insert("file_read".to_string());
        boundary.untrusted_sources.insert("env_var".to_string());
        boundary
            .untrusted_sources
            .insert("command_line".to_string());
        boundary
            .untrusted_sources
            .insert("network_recv".to_string());
        boundary.untrusted_sources.insert("user_input".to_string());
        boundary.untrusted_sources.insert("request".to_string());
        boundary.untrusted_sources.insert("query".to_string());
        boundary.untrusted_sources.insert("params".to_string());
        boundary.untrusted_sources.insert("body".to_string());
        boundary.untrusted_sources.insert("stdin".to_string());

        // Register default sensitive sinks with required purification
        boundary
            .sensitive_sinks
            .insert("sql_query".to_string(), vec!["sql_injection".to_string()]);
        boundary.sensitive_sinks.insert(
            "sāraṇī_pṛcchā".to_string(), // Sanskrit SQL query
            vec!["sql_injection".to_string()],
        );
        boundary.sensitive_sinks.insert(
            "shell_exec".to_string(),
            vec!["command_injection".to_string()],
        );
        boundary.sensitive_sinks.insert(
            "ādeśa_kriyā".to_string(), // Sanskrit command execution
            vec!["command_injection".to_string()],
        );
        boundary
            .sensitive_sinks
            .insert("html_output".to_string(), vec!["xss".to_string()]);
        boundary
            .sensitive_sinks
            .insert("file_write".to_string(), vec!["path_traversal".to_string()]);
        boundary
            .sensitive_sinks
            .insert("eval".to_string(), vec!["code_injection".to_string()]);
        boundary.sensitive_sinks.insert(
            "mūlyāṅkana".to_string(), // Sanskrit eval
            vec!["code_injection".to_string()],
        );

        // Register default purifiers (śuddhi-kri = purification action)
        boundary.register_purifier(PurifierSpec {
            name: "sql_escape".to_string(),
            cleans: vec!["sql_injection".to_string()],
            sanskrit_name: "śuddhi-kri-sāraṇī".to_string(),
        });
        boundary.register_purifier(PurifierSpec {
            name: "html_escape".to_string(),
            cleans: vec!["xss".to_string()],
            sanskrit_name: "śuddhi-kri-prakāśa".to_string(),
        });
        boundary.register_purifier(PurifierSpec {
            name: "shell_escape".to_string(),
            cleans: vec!["command_injection".to_string()],
            sanskrit_name: "śuddhi-kri-ādeśa".to_string(),
        });
        boundary.register_purifier(PurifierSpec {
            name: "path_sanitize".to_string(),
            cleans: vec!["path_traversal".to_string()],
            sanskrit_name: "śuddhi-kri-patha".to_string(),
        });
        boundary.register_purifier(PurifierSpec {
            name: "validate_json".to_string(),
            cleans: vec!["sql_injection".to_string(), "xss".to_string()],
            sanskrit_name: "śuddhi-kri-vicāra".to_string(),
        });

        boundary
    }

    /// Register a purifier
    pub fn register_purifier(&mut self, spec: PurifierSpec) {
        // Register both by name and Sanskrit name
        self.purifiers.insert(spec.name.clone(), spec.clone());
        self.purifiers.insert(spec.sanskrit_name.clone(), spec);
    }

    /// Mark a source as untrusted
    pub fn mark_untrusted(&mut self, source: &str) {
        self.untrusted_sources.insert(source.to_string());
    }

    /// Check if a source is untrusted
    pub fn is_untrusted(&self, source: &str) -> bool {
        self.untrusted_sources.contains(source) || self.is_pattern_untrusted(source)
    }

    /// Check if source matches untrusted patterns
    fn is_pattern_untrusted(&self, source: &str) -> bool {
        let source_lower = source.to_lowercase();
        let patterns = [
            "request", "input", "param", "query", "body", "args", "env", "http", "socket", "stdin",
            "user", "external", "praveśa", "prāpti", "paṭha", "bāhya",
        ];
        patterns.iter().any(|p| source_lower.contains(p))
    }

    /// Check all Vaitarani crossings in code
    pub fn check_crossings(&self, ast: &Ast, _taint: &TaintAnalyzer) -> Vec<VaitaraniViolation> {
        let mut checker = VaitaraniChecker::new(self);
        checker.analyze(ast);
        checker.violations
    }

    /// Get the purifier needed for a specific sink
    pub fn get_required_purifier(&self, sink: &str) -> Option<&PurifierSpec> {
        if let Some(required) = self.sensitive_sinks.get(sink) {
            // Find a purifier that cleans the required taints
            for purifier in self.purifiers.values() {
                if required.iter().all(|r| purifier.cleans.contains(r)) {
                    return Some(purifier);
                }
            }
        }
        None
    }

    /// Get Sanskrit fix suggestion
    pub fn get_sanskrit_fix(&self, sink: &str) -> String {
        if let Some(purifier) = self.get_required_purifier(sink) {
            format!(
                "Apply {}() [{}] before passing to {}",
                purifier.name, purifier.sanskrit_name, sink
            )
        } else {
            format!("Validate and sanitize input before {}", sink)
        }
    }
}

impl Default for VaitaraniBoundary {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal checker that performs the actual taint analysis
struct VaitaraniChecker<'a> {
    boundary: &'a VaitaraniBoundary,
    /// Current taint state for variables
    tainted: HashMap<String, TaintInfo>,
    /// Variables that have been purified
    purified: HashMap<String, HashSet<String>>, // var -> set of cleaned taint types
    /// Detected violations
    violations: Vec<VaitaraniViolation>,
}

impl<'a> VaitaraniChecker<'a> {
    fn new(boundary: &'a VaitaraniBoundary) -> Self {
        Self {
            boundary,
            tainted: HashMap::new(),
            purified: HashMap::new(),
            violations: Vec::new(),
        }
    }

    fn analyze(&mut self, ast: &Ast) {
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.analyze_function(func);
            }
        }
    }

    fn analyze_function(&mut self, func: &FunctionDef) {
        // Reset per-function state
        self.tainted.clear();
        self.purified.clear();

        // Mark parameters from external sources as tainted
        for param in &func.params {
            if self.boundary.is_untrusted(&param.name.name) {
                self.tainted.insert(
                    param.name.name.clone(),
                    TaintInfo {
                        variable: param.name.name.clone(),
                        source: "parameter".to_string(),
                        taint_type: "untrusted_input".to_string(),
                    },
                );
            }
        }

        self.analyze_block(&func.body);
    }

    fn analyze_block(&mut self, block: &Block) {
        for stmt in &block.stmts {
            self.analyze_stmt(stmt);
        }
    }

    fn analyze_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let {
                name,
                value: Some(value),
                span: _span,
                ..
            } => {
                // Check if assignment is from untrusted source
                if let Some(taint) = self.check_expr_taint(value) {
                    self.tainted.insert(
                        name.name.clone(),
                        TaintInfo {
                            variable: name.name.clone(),
                            source: taint.source.clone(),
                            taint_type: taint.taint_type.clone(),
                        },
                    );
                } else if self.is_tainted_expr(value) {
                    // Propagate taint through assignment
                    if let Some(source_taint) = self.get_expr_source_taint(value) {
                        self.tainted.insert(name.name.clone(), source_taint);
                    }
                }

                // Check if value is result of purification
                if let Some(cleaned_types) = self.check_purification(value) {
                    self.purified
                        .entry(name.name.clone())
                        .or_default()
                        .extend(cleaned_types);
                }
            }
            Stmt::Expr(expr) => {
                self.check_sink_access(expr);
            }
            Stmt::If {
                then_block,
                else_block,
                ..
            } => {
                self.analyze_block(then_block);
                if let Some(eb) = else_block {
                    self.analyze_block(eb);
                }
            }
            Stmt::Loop { body, .. } => {
                self.analyze_block(body);
            }
            Stmt::Return {
                value: Some(value), ..
            } => {
                self.check_sink_access(value);
            }
            _ => {}
        }
    }

    /// Check if expression originates from untrusted source
    fn check_expr_taint(&self, expr: &Expr) -> Option<TaintInfo> {
        match expr {
            Expr::Call { callee, .. } => {
                if let Expr::Identifier(ident) = callee.as_ref() {
                    if self.boundary.is_untrusted(&ident.name) {
                        return Some(TaintInfo {
                            variable: String::new(),
                            source: ident.name.clone(),
                            taint_type: "untrusted_input".to_string(),
                        });
                    }
                }
            }
            Expr::MethodCall { method, .. } => {
                if self.boundary.is_untrusted(&method.name) {
                    return Some(TaintInfo {
                        variable: String::new(),
                        source: method.name.clone(),
                        taint_type: "untrusted_input".to_string(),
                    });
                }
            }
            _ => {}
        }
        None
    }

    /// Check if expression uses tainted data
    fn is_tainted_expr(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Identifier(ident) => {
                self.tainted.contains_key(&ident.name) && !self.is_fully_purified(&ident.name)
            }
            Expr::Binary { left, right, .. } => {
                self.is_tainted_expr(left) || self.is_tainted_expr(right)
            }
            Expr::Call { args, .. } | Expr::MethodCall { args, .. } => {
                args.iter().any(|a| self.is_tainted_expr(a))
            }
            Expr::FieldAccess { object, .. } => self.is_tainted_expr(object),
            Expr::Index { object, index, .. } => {
                self.is_tainted_expr(object) || self.is_tainted_expr(index)
            }
            _ => false,
        }
    }

    /// Get taint info from expression source
    fn get_expr_source_taint(&self, expr: &Expr) -> Option<TaintInfo> {
        match expr {
            Expr::Identifier(ident) => self.tainted.get(&ident.name).cloned(),
            Expr::Binary { left, right, .. } => self
                .get_expr_source_taint(left)
                .or_else(|| self.get_expr_source_taint(right)),
            _ => None,
        }
    }

    /// Check if expression is a purification call
    fn check_purification(&self, expr: &Expr) -> Option<Vec<String>> {
        match expr {
            Expr::Call { callee, .. } => {
                if let Expr::Identifier(ident) = callee.as_ref() {
                    if let Some(purifier) = self.boundary.purifiers.get(&ident.name) {
                        return Some(purifier.cleans.clone());
                    }
                }
            }
            Expr::MethodCall { method, .. } => {
                if let Some(purifier) = self.boundary.purifiers.get(&method.name) {
                    return Some(purifier.cleans.clone());
                }
            }
            _ => {}
        }
        None
    }

    /// Check if variable has been fully purified
    fn is_fully_purified(&self, var: &str) -> bool {
        if let Some(taint_info) = self.tainted.get(var) {
            if let Some(purified_types) = self.purified.get(var) {
                // Check if all required taint types are purified
                return purified_types.contains(&taint_info.taint_type)
                    || purified_types.contains("untrusted_input");
            }
        }
        false
    }

    /// Check if expression accesses a sensitive sink with tainted data
    fn check_sink_access(&mut self, expr: &Expr) {
        match expr {
            Expr::Call { callee, args, .. } => {
                if let Expr::Identifier(ident) = callee.as_ref() {
                    // Check if this is a sensitive sink
                    if let Some(_required) = self.boundary.sensitive_sinks.get(&ident.name) {
                        // Check if any argument is tainted
                        for arg in args {
                            if self.is_tainted_expr(arg) {
                                let taint_source = self
                                    .get_expr_source_taint(arg)
                                    .map(|t| t.source.clone())
                                    .unwrap_or_else(|| "unknown".to_string());

                                self.violations.push(VaitaraniViolation {
                                    location: ident.span.clone().into(),
                                    message: format!(
                                        "Tainted data from '{}' flows to sensitive sink '{}' without purification - \
                                        Vaitarani crossing denied!",
                                        taint_source, ident.name
                                    ),
                                    required_purifier: self.boundary
                                        .get_required_purifier(&ident.name)
                                        .map(|p| p.sanskrit_name.clone()),
                                    taint_source,
                                });
                            }
                        }
                    }
                }

                // Recursively check nested calls
                for arg in args {
                    self.check_sink_access(arg);
                }
            }
            Expr::MethodCall {
                receiver,
                method,
                args,
                ..
            } => {
                if let Some(_required) = self.boundary.sensitive_sinks.get(&method.name) {
                    // Check receiver and args for taint
                    if self.is_tainted_expr(receiver) {
                        let taint_source = self
                            .get_expr_source_taint(receiver)
                            .map(|t| t.source.clone())
                            .unwrap_or_else(|| "unknown".to_string());

                        self.violations.push(VaitaraniViolation {
                            location: method.span.clone().into(),
                            message: format!(
                                "Tainted receiver from '{}' used in sensitive method '{}' - \
                                Vaitarani crossing denied!",
                                taint_source, method.name
                            ),
                            required_purifier: self
                                .boundary
                                .get_required_purifier(&method.name)
                                .map(|p| p.sanskrit_name.clone()),
                            taint_source,
                        });
                    }

                    for arg in args {
                        if self.is_tainted_expr(arg) {
                            let taint_source = self
                                .get_expr_source_taint(arg)
                                .map(|t| t.source.clone())
                                .unwrap_or_else(|| "unknown".to_string());

                            self.violations.push(VaitaraniViolation {
                                location: method.span.clone().into(),
                                message: format!(
                                    "Tainted argument from '{}' passed to sensitive method '{}' - \
                                    Vaitarani crossing denied!",
                                    taint_source, method.name
                                ),
                                required_purifier: self
                                    .boundary
                                    .get_required_purifier(&method.name)
                                    .map(|p| p.sanskrit_name.clone()),
                                taint_source,
                            });
                        }
                    }
                }

                // Recursively check
                self.check_sink_access(receiver);
                for arg in args {
                    self.check_sink_access(arg);
                }
            }
            Expr::Binary { left, right, .. } => {
                self.check_sink_access(left);
                self.check_sink_access(right);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vaitarani_boundary_new() {
        let boundary = VaitaraniBoundary::new();
        assert!(boundary.is_untrusted("http_input"));
        assert!(boundary.is_untrusted("user_input"));
        assert!(!boundary.is_untrusted("trusted_source"));
    }

    #[test]
    fn test_pattern_matching() {
        let boundary = VaitaraniBoundary::new();
        assert!(boundary.is_untrusted("get_request_body"));
        assert!(boundary.is_untrusted("read_user_input"));
        assert!(boundary.is_untrusted("bāhya_praveśa"));
    }

    #[test]
    fn test_purifier_registration() {
        let boundary = VaitaraniBoundary::new();
        assert!(boundary.purifiers.contains_key("sql_escape"));
        assert!(boundary.purifiers.contains_key("śuddhi-kri-sāraṇī"));
    }

    #[test]
    fn test_sink_detection() {
        let boundary = VaitaraniBoundary::new();
        assert!(boundary.sensitive_sinks.contains_key("sql_query"));
        assert!(boundary.sensitive_sinks.contains_key("shell_exec"));
        assert!(boundary.sensitive_sinks.contains_key("eval"));
    }

    #[test]
    fn test_get_required_purifier() {
        let boundary = VaitaraniBoundary::new();
        let purifier = boundary.get_required_purifier("sql_query");
        assert!(purifier.is_some());
        assert!(purifier
            .unwrap()
            .cleans
            .contains(&"sql_injection".to_string()));
    }

    #[test]
    fn test_sanskrit_fix_suggestion() {
        let boundary = VaitaraniBoundary::new();
        let fix = boundary.get_sanskrit_fix("sql_query");
        assert!(fix.contains("śuddhi-kri"));
    }
}
