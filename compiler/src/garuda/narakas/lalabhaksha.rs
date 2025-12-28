//! # Lalabhaksha - Hell 18: Data Exposure
//!
//! Sin: Lustful acts
//! Code: Inappropriate data exposure

use super::super::yama::{Violation, ViolationKind};
use crate::errors::Span;
use crate::parser::ast::{Ast, Block, Expr, Item, LoopKind, Stmt};
use std::collections::HashSet;

/// Checker for Lalabhaksha violations (data exposure)
pub struct LalabhakshaChecker {
    /// Sensitive variable patterns
    sensitive_patterns: Vec<&'static str>,
    /// Output/logging functions
    output_fns: HashSet<&'static str>,
    /// Sensitive variables detected
    sensitive_vars: HashSet<String>,
}

impl LalabhakshaChecker {
    pub fn new() -> Self {
        Self {
            sensitive_patterns: vec![
                "password",
                "passwd",
                "secret",
                "key",
                "token",
                "api_key",
                "private",
                "credential",
                "auth",
                "ssn",
                "credit_card",
                "guptam",
                "rahasya",
            ], // Sanskrit: hidden, secret
            output_fns: [
                "print", "println", "log", "debug", "info", "warn", "error", "console", "write",
                "send", "respond", "vadati",
            ]
            .into(),
            sensitive_vars: HashSet::new(),
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.sensitive_vars.clear();
                // Mark sensitive parameters
                for param in &func.params {
                    if self.is_sensitive_name(&param.name.name) {
                        self.sensitive_vars.insert(param.name.name.clone());
                    }
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
            Stmt::Let { name, value, .. } => {
                if self.is_sensitive_name(&name.name) {
                    self.sensitive_vars.insert(name.name.clone());
                }
                if let Some(v) = value {
                    self.check_expr(v, violations);
                }
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
            Stmt::Return {
                value: Some(v),
                span,
            } => {
                // Check if returning sensitive data
                if self.contains_sensitive(v) {
                    violations.push(Violation::full(
                        ViolationKind::DataExposure,
                        span.clone().into(),
                        "Returning sensitive data from function".to_string(),
                        "Lustful exposure: Revealing private data",
                        "Entry to Lalabhaksha (saliva-eating hell)",
                        "Return sanitized data or remove sensitive fields".to_string(),
                    ));
                }
                self.check_expr(v, violations);
            }
            _ => {}
        }
    }

    fn check_expr(&mut self, expr: &Expr, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Call { callee, args, span } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    if self.output_fns.contains(id.name.as_str()) {
                        // Check if logging sensitive data
                        for arg in args {
                            if self.contains_sensitive(arg) {
                                violations.push(Violation::full(
                                    ViolationKind::DataExposure,
                                    span.clone().into(),
                                    format!("Sensitive data passed to '{}'()", id.name),
                                    "Lustful exposure: Logging secrets",
                                    "Entry to Lalabhaksha",
                                    "Mask or redact sensitive data before logging".to_string(),
                                ));
                            }
                        }
                    }
                }
                for a in args {
                    self.check_expr(a, violations);
                }
            }
            Expr::MethodCall {
                receiver,
                method,
                args,
                span,
            } => {
                // Check for serialization of sensitive data
                if method.name == "to_string"
                    || method.name == "serialize"
                    || method.name == "to_json"
                {
                    if self.contains_sensitive(receiver) {
                        violations.push(Violation::full(
                            ViolationKind::DataExposure,
                            span.clone().into(),
                            "Serializing sensitive data".to_string(),
                            "Lustful exposure: Converting secrets to plain text",
                            "Entry to Lalabhaksha",
                            "Implement Redact trait or use #[sensitive] field annotation"
                                .to_string(),
                        ));
                    }
                }
                self.check_expr(receiver, violations);
                for a in args {
                    self.check_expr(a, violations);
                }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left, violations);
                self.check_expr(right, violations);
            }
            _ => {}
        }
    }

    fn is_sensitive_name(&self, name: &str) -> bool {
        let lower = name.to_lowercase();
        self.sensitive_patterns.iter().any(|p| lower.contains(p))
    }

    fn contains_sensitive(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Identifier(id) => {
                self.sensitive_vars.contains(&id.name) || self.is_sensitive_name(&id.name)
            }
            Expr::FieldAccess { field, .. } => self.is_sensitive_name(&field.name),
            Expr::Binary { left, right, .. } => {
                self.contains_sensitive(left) || self.contains_sensitive(right)
            }
            _ => false,
        }
    }
}

impl Default for LalabhakshaChecker {
    fn default() -> Self {
        Self::new()
    }
}
