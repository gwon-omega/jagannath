//! # Vaitarani Naraka - Hell 14: Tainted Data
//!
//! Sin: Abusing power, adultery
//! Code: Tainted data crossing security boundary

use super::super::yama::{Violation, ViolationKind};
use crate::parser::ast::{Ast, Expr, Item, Stmt, Block, LoopKind};
use std::collections::HashSet;

/// Checker for Vaitarani Naraka violations (tainted data)
pub struct VaitaraniNarakaChecker {
    /// Tainted sources
    taint_sources: HashSet<&'static str>,
    /// Security boundary functions (sinks)
    security_sinks: HashSet<&'static str>,
    /// Sanitizer functions
    sanitizers: HashSet<&'static str>,
    /// Currently tainted variables
    tainted: HashSet<String>,
}

impl VaitaraniNarakaChecker {
    pub fn new() -> Self {
        Self {
            taint_sources: ["user_input", "read_input", "request", "query_param", "form_data",
                            "env", "getenv", "stdin", "args", "recv", "read"].into(),
            security_sinks: ["sql_query", "exec", "system", "eval", "render", "respond",
                             "write_file", "send", "privilege_op", "set_cookie"].into(),
            sanitizers: ["escape", "sanitize", "validate", "filter", "encode",
                         "quote", "parameterize", "shuddhi_kri", "purify"].into(),
            tainted: HashSet::new(),
        }
    }

    /// Check for tainted data crossing boundaries
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
            Stmt::Let { name, value: Some(v), .. } => {
                // Track taint propagation
                if self.is_tainted_expr(v) {
                    self.tainted.insert(name.name.clone());
                } else if self.is_sanitized(v) {
                    // Sanitized data is clean
                    self.tainted.remove(&name.name);
                } else if self.is_taint_source(v) {
                    self.tainted.insert(name.name.clone());
                }
                self.check_expr(v, violations);
            }
            Stmt::Expr(e) => self.check_expr(e, violations),
            Stmt::If { condition, then_block, else_block, .. } => {
                self.check_expr(condition, violations);
                // Check for taint-based conditionals (potential info leak)
                if self.is_tainted_expr(condition) {
                    // This could leak information based on tainted data
                }
                self.check_block(then_block, violations);
                if let Some(eb) = else_block { self.check_block(eb, violations); }
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
                    // Check for tainted data reaching security sink
                    if self.security_sinks.contains(id.name.as_str()) {
                        for (i, arg) in args.iter().enumerate() {
                            if self.is_tainted_expr(arg) {
                                violations.push(Violation::full(
                                    ViolationKind::TaintedData, span.clone().into(),
                                    format!("Tainted data in argument {} to security-sensitive '{}'", i+1, id.name),
                                    "Power abuse: Tainted data crossing Vaitarani river",
                                    "Entry to Vaitarani Naraka (filthy river hell)",
                                    "Sanitize with shuddhi_kri() before use".to_string(),
                                ));
                            }
                        }
                    }
                }
                for a in args { self.check_expr(a, violations); }
            }
            Expr::MethodCall { method, receiver, args, span } => {
                // Check method calls as potential sinks
                if self.security_sinks.contains(method.name.as_str()) {
                    if self.is_tainted_expr(receiver) {
                        violations.push(Violation::full(
                            ViolationKind::TaintedData, span.clone().into(),
                            format!("Tainted object calling security method '{}'", method.name),
                            "Power abuse: Contaminated object at boundary",
                            "Entry to Vaitarani Naraka",
                            "Sanitize object before security operation".to_string(),
                        ));
                    }
                    for arg in args {
                        if self.is_tainted_expr(arg) {
                            violations.push(Violation::full(
                                ViolationKind::TaintedData, span.clone().into(),
                                format!("Tainted argument to '{}'", method.name),
                                "Power abuse: Tainted data crossing boundary",
                                "Entry to Vaitarani Naraka",
                                "Sanitize argument before use".to_string(),
                            ));
                        }
                    }
                }
                self.check_expr(receiver, violations);
                for a in args { self.check_expr(a, violations); }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left, violations);
                self.check_expr(right, violations);
            }
            _ => {}
        }
    }

    fn is_tainted_expr(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Identifier(id) => self.tainted.contains(&id.name),
            Expr::Call { callee, .. } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    self.taint_sources.contains(id.name.as_str())
                } else { false }
            }
            Expr::Binary { left, right, .. } => {
                self.is_tainted_expr(left) || self.is_tainted_expr(right)
            }
            Expr::FieldAccess { object, .. } => self.is_tainted_expr(object),
            _ => false,
        }
    }

    fn is_taint_source(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Call { callee, .. } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    self.taint_sources.contains(id.name.as_str())
                } else { false }
            }
            Expr::MethodCall { method, .. } => self.taint_sources.contains(method.name.as_str()),
            _ => false,
        }
    }

    fn is_sanitized(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Call { callee, .. } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    self.sanitizers.contains(id.name.as_str())
                } else { false }
            }
            Expr::MethodCall { method, .. } => self.sanitizers.contains(method.name.as_str()),
            _ => false,
        }
    }
}

impl Default for VaitaraniNarakaChecker {
    fn default() -> Self { Self::new() }
}
