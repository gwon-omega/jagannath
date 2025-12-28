//! # Ksharakardama - Hell 22: Insecure Storage
//!
//! Sin: Pride, false teaching
//! Code: Insecure credential storage

use crate::parser::ast::{Ast, Expr, Item, Stmt, Block, LoopKind, BinaryOp};
use crate::errors::Span;
use super::super::yama::{Violation, ViolationKind};
use std::collections::HashSet;

/// Checker for Ksharakardama violations (insecure storage)
pub struct KsharakardamaChecker {
    /// Sensitive data patterns
    sensitive_patterns: Vec<&'static str>,
    /// Insecure storage functions
    insecure_storage: HashSet<&'static str>,
    /// Secure alternatives
    secure_alternatives: HashSet<&'static str>,
}

impl KsharakardamaChecker {
    pub fn new() -> Self {
        Self {
            sensitive_patterns: vec!["password", "passwd", "secret", "key", "token", "api_key",
                                     "private_key", "credential", "auth", "guptam"],
            insecure_storage: ["write_file", "print", "log", "save", "store", "serialize",
                               "to_string", "format", "concat"].into(),
            secure_alternatives: ["encrypt", "hash", "secure_store", "vault_store", "keychain"].into(),
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.check_block(&func.body, &mut violations);
            }
            // Check for hardcoded secrets in constants
            if let Item::Constant(constant) = item {
                if self.is_sensitive_name(&constant.name.name) {
                    if let Expr::Literal(_) = &constant.value {
                        violations.push(Violation::full(
                            ViolationKind::InsecureStorage, Span::dummy().into(),
                            format!("Hardcoded sensitive value '{}'", constant.name.name),
                            "Pride: Storing secrets in plain sight",
                            "Entry to Ksharakardama (caustic mud hell)",
                            "Use environment variables or secure vault".to_string(),
                        ));
                    }
                }
            }
        }
        violations
    }

    fn check_block(&self, block: &Block, violations: &mut Vec<Violation>) {
        for stmt in &block.stmts {
            self.check_stmt(stmt, violations);
        }
    }

    fn check_stmt(&self, stmt: &Stmt, violations: &mut Vec<Violation>) {
        match stmt {
            Stmt::Let { name, value: Some(v), span, .. } => {
                // Check for hardcoded secrets
                if self.is_sensitive_name(&name.name) {
                    if self.is_hardcoded_value(v) {
                        violations.push(Violation::full(
                            ViolationKind::InsecureStorage, span.clone().into(),
                            format!("Hardcoded sensitive value for '{}'", name.name),
                            "Pride: Exposing secrets in source code",
                            "Entry to Ksharakardama",
                            "Load from secure configuration or vault".to_string(),
                        ));
                    }
                }
                self.check_expr(v, violations);
            }
            Stmt::Expr(e) => self.check_expr(e, violations),
            Stmt::If { condition, then_block, else_block, .. } => {
                self.check_expr(condition, violations);
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

    fn check_expr(&self, expr: &Expr, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Call { callee, args, span } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    // Check if storing sensitive data insecurely
                    if self.insecure_storage.contains(id.name.as_str()) {
                        for arg in args {
                            if self.contains_sensitive(arg) {
                                violations.push(Violation::full(
                                    ViolationKind::InsecureStorage, span.clone().into(),
                                    format!("Sensitive data passed to '{}'", id.name),
                                    "Pride: Storing credentials insecurely",
                                    "Entry to Ksharakardama",
                                    "Use encrypt() or secure_store()".to_string(),
                                ));
                            }
                        }
                    }
                }
                for a in args { self.check_expr(a, violations); }
            }
            Expr::MethodCall { method, receiver, args, span } => {
                // Check for insecure serialization of sensitive data
                if self.insecure_storage.contains(method.name.as_str()) {
                    if self.contains_sensitive(receiver) {
                        violations.push(Violation::full(
                            ViolationKind::InsecureStorage, span.clone().into(),
                            format!("Sensitive data being serialized via '{}'", method.name),
                            "Pride: Converting secrets to plain text",
                            "Entry to Ksharakardama",
                            "Encrypt before storing or transmitting".to_string(),
                        ));
                    }
                }
                self.check_expr(receiver, violations);
                for a in args { self.check_expr(a, violations); }
            }
            Expr::Binary { left, right, op, span } => {
                // Check for password comparison (should use constant-time)
                if *op == BinaryOp::Eq || *op == BinaryOp::Ne {
                    if self.contains_sensitive(left) || self.contains_sensitive(right) {
                        violations.push(Violation::full(
                            ViolationKind::InsecureStorage, span.clone().into(),
                            "Direct comparison of sensitive data".to_string(),
                            "Pride: Timing attack vulnerability",
                            "Entry to Ksharakardama",
                            "Use constant_time_eq() for password comparison".to_string(),
                        ));
                    }
                }
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

    fn is_hardcoded_value(&self, expr: &Expr) -> bool {
        matches!(expr, Expr::Literal(_))
    }

    fn contains_sensitive(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Identifier(id) => self.is_sensitive_name(&id.name),
            Expr::FieldAccess { field, .. } => self.is_sensitive_name(&field.name),
            Expr::Binary { left, right, .. } => {
                self.contains_sensitive(left) || self.contains_sensitive(right)
            }
            _ => false,
        }
    }
}

impl Default for KsharakardamaChecker {
    fn default() -> Self { Self::new() }
}
