//! # Sulaprota - Hell 24: Code Injection
//!
//! Sin: Killing animals for pleasure
//! Code: SQL/LDAP/XPath/Command injection

use super::super::yama::{Violation, ViolationKind};
use crate::errors::Span;
use crate::parser::ast::{Ast, BinaryOp, Block, Expr, Item, LoopKind, Stmt};
use std::collections::HashSet;

/// Checker for Sulaprota violations (injection attacks)
pub struct SulaprotaChecker {
    /// Tainted variables (user input)
    tainted: HashSet<String>,
    /// Dangerous sink functions
    sink_fns: HashSet<&'static str>,
    /// Source functions (user input)
    source_fns: HashSet<&'static str>,
    /// Sanitizer functions
    sanitizers: HashSet<&'static str>,
}

impl SulaprotaChecker {
    pub fn new() -> Self {
        Self {
            tainted: HashSet::new(),
            sink_fns: [
                "exec",
                "execute",
                "query",
                "system",
                "eval",
                "shell",
                "popen",
                "sql_query",
                "run_command",
                "exec_sql",
                "ldap_search",
                "xpath_query",
            ]
            .into(),
            source_fns: [
                "read_input",
                "get_param",
                "request",
                "stdin",
                "env",
                "args",
                "user_input",
                "form_data",
                "query_string",
                "read_line",
            ]
            .into(),
            sanitizers: [
                "escape",
                "sanitize",
                "quote",
                "parameterize",
                "prepare",
                "html_escape",
                "sql_escape",
                "encode",
                "shuddhi_kri",
            ]
            .into(),
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.tainted.clear();
                // Mark parameters as potentially tainted
                for param in &func.params {
                    self.tainted.insert(param.name.name.clone());
                }
                self.check_block(&func.body, &mut violations);
            }
        }
        violations
    }

    fn check_block(&mut self, block: &Block, violations: &mut Vec<Violation>) {
        for stmt in &block.stmts {
            self.check_stmt(stmt, violations);
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt, violations: &mut Vec<Violation>) {
        match stmt {
            Stmt::Let {
                name,
                value: Some(v),
                ..
            } => {
                // Track tainted values
                if self.is_tainted_expr(v) {
                    self.tainted.insert(name.name.clone());
                } else if self.is_sanitized(v) {
                    self.tainted.remove(&name.name);
                }
                self.check_expr(v, violations);
            }
            Stmt::Expr(e) => self.check_expr(e, violations),
            Stmt::If {
                condition,
                then_block,
                else_block,
                ..
            } => {
                self.check_expr(condition, violations);
                self.check_block(then_block, violations);
                if let Some(eb) = else_block {
                    self.check_block(eb, violations);
                }
            }
            Stmt::Loop { body, kind, .. } => {
                if let LoopKind::While { condition } = kind {
                    self.check_expr(condition, violations);
                }
                self.check_block(body, violations);
            }
            _ => {}
        }
    }

    fn check_expr(&mut self, expr: &Expr, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Call { callee, args, span } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    // Check if calling a dangerous sink with tainted data
                    if self.sink_fns.contains(id.name.as_str()) {
                        for arg in args {
                            if self.is_tainted_expr(arg) {
                                violations.push(Violation::full(
                                    ViolationKind::InjectionAttack, span.clone().into(),
                                    format!("Tainted data passed to dangerous function '{}'", id.name),
                                    "Killing for pleasure: Injecting malicious code",
                                    "Entry to Sulaprota (impaled hell)",
                                    "Sanitize input with shuddhi_kri() or use parameterized queries".to_string(),
                                ));
                            }
                        }
                    }
                    // Track taint from sources
                    if self.source_fns.contains(id.name.as_str()) {
                        // Return value will be tainted
                    }
                }
                for a in args {
                    self.check_expr(a, violations);
                }
            }
            Expr::Binary {
                left,
                right,
                op,
                span,
            } => {
                // String concatenation with taint is suspicious
                if *op == BinaryOp::Add {
                    let left_taint = self.is_tainted_expr(left);
                    let right_taint = self.is_tainted_expr(right);
                    if left_taint || right_taint {
                        // Propagate taint through concatenation
                        // (This is tracked in assignment handling)
                    }
                }
                self.check_expr(left, violations);
                self.check_expr(right, violations);
            }
            Expr::MethodCall {
                receiver,
                method,
                args,
                span,
            } => {
                // Check format!/format strings with tainted data
                if method.name == "format" || method.name == "interpolate" {
                    for arg in args {
                        if self.is_tainted_expr(arg) {
                            violations.push(Violation::full(
                                ViolationKind::InjectionAttack,
                                span.clone().into(),
                                "Tainted data in format string".to_string(),
                                "Killing for pleasure: Format string injection",
                                "Entry to Sulaprota",
                                "Use parameterized formatting".to_string(),
                            ));
                        }
                    }
                }
                self.check_expr(receiver, violations);
                for a in args {
                    self.check_expr(a, violations);
                }
            }
            _ => {}
        }
    }

    fn is_tainted_expr(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Identifier(id) => self.tainted.contains(&id.name),
            Expr::Call { callee, .. } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    self.source_fns.contains(id.name.as_str())
                } else {
                    false
                }
            }
            Expr::Binary { left, right, .. } => {
                self.is_tainted_expr(left) || self.is_tainted_expr(right)
            }
            _ => false,
        }
    }

    fn is_sanitized(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Call { callee, .. } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    self.sanitizers.contains(id.name.as_str())
                } else {
                    false
                }
            }
            Expr::MethodCall { method, .. } => self.sanitizers.contains(method.name.as_str()),
            _ => false,
        }
    }
}

impl Default for SulaprotaChecker {
    fn default() -> Self {
        Self::new()
    }
}
