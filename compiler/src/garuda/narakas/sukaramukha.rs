//! # Sukaramukha - Hell 24: Pig-Face
//!
//! Sin: Torturing others
//! Code: Blocking operations without timeout

use crate::parser::ast::{Ast, Expr, Item, Stmt, Block, LoopKind};
use crate::errors::Span;
use super::super::yama::{Violation, ViolationKind};
use std::collections::HashSet;

/// Checker for Sukaramukha violations (blocking torture)
pub struct SukaramukhaChecker {
    /// Blocking operations
    blocking_ops: HashSet<&'static str>,
    /// Async alternatives
    async_alts: HashSet<&'static str>,
}

impl SukaramukhaChecker {
    pub fn new() -> Self {
        Self {
            blocking_ops: ["read", "write", "recv", "send", "accept", "connect",
                           "wait", "join", "lock", "acquire", "sleep", "block",
                           "getaddrinfo", "gethostbyname"].into(),
            async_alts: ["read_async", "write_async", "recv_async", "poll",
                         "select", "epoll", "try_lock", "try_recv"].into(),
        }
    }

    /// Check for blocking operations without timeout
    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                // Check function name for async indicators
                let is_async = func.name.name.starts_with("async_")
                    || func.name.name.contains("_async");
                self.check_block(&func.body, is_async, &mut violations);
            }
        }
        violations
    }

    fn check_block(&self, block: &Block, is_async: bool, violations: &mut Vec<Violation>) {
        for stmt in &block.stmts {
            self.check_stmt(stmt, is_async, violations);
        }
    }

    fn check_stmt(&self, stmt: &Stmt, is_async: bool, violations: &mut Vec<Violation>) {
        match stmt {
            Stmt::Expr(e) => self.check_expr(e, is_async, violations),
            Stmt::Let { value: Some(v), .. } => self.check_expr(v, is_async, violations),
            Stmt::If { condition, then_block, else_block, .. } => {
                self.check_expr(condition, is_async, violations);
                self.check_block(then_block, is_async, violations);
                if let Some(eb) = else_block { self.check_block(eb, is_async, violations); }
            }
            Stmt::Loop { body, kind, .. } => {
                if let LoopKind::While { condition } = kind {
                    self.check_expr(condition, is_async, violations);
                }
                self.check_block(body, is_async, violations);
            }
            _ => {}
        }
    }

    fn check_expr(&self, expr: &Expr, is_async: bool, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Call { callee, args, span } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    if self.blocking_ops.contains(id.name.as_str()) {
                        // Check if there's a timeout argument
                        let has_timeout = args.iter().any(|a| {
                            if let Expr::Identifier(arg_id) = a {
                                arg_id.name.contains("timeout") || arg_id.name.contains("deadline")
                            } else { false }
                        });

                        if !has_timeout {
                            let severity = if is_async { "CRITICAL" } else { "WARNING" };
                            violations.push(Violation::full(
                                ViolationKind::ThreadUnsafe, span.clone().into(),
                                format!("{}: Blocking '{}' without timeout", severity, id.name),
                                "Torture: Potentially hanging indefinitely",
                                "Entry to Sukaramukha (pig-face hell)",
                                format!("Use {}_timeout() or async alternative", id.name),
                            ));
                        }

                        // Extra warning for blocking in async context
                        if is_async {
                            violations.push(Violation::full(
                                ViolationKind::ThreadUnsafe, span.clone().into(),
                                format!("Blocking '{}' in async function", id.name),
                                "Torture: Blocking async runtime",
                                "Entry to Sukaramukha",
                                "Use async version or spawn_blocking()".to_string(),
                            ));
                        }
                    }
                }
                for a in args { self.check_expr(a, is_async, violations); }
            }
            Expr::MethodCall { method, receiver, args, span } => {
                if self.blocking_ops.contains(method.name.as_str()) {
                    let has_timeout = self.has_timeout_chain(receiver) || args.iter().any(|a| {
                        if let Expr::Identifier(id) = a {
                            id.name.contains("timeout")
                        } else { false }
                    });

                    if !has_timeout {
                        violations.push(Violation::full(
                            ViolationKind::ThreadUnsafe, span.clone().into(),
                            format!("Method '{}' may block indefinitely", method.name),
                            "Torture: No escape from waiting",
                            "Entry to Sukaramukha",
                            "Add timeout or use try_ variant".to_string(),
                        ));
                    }
                }
                self.check_expr(receiver, is_async, violations);
                for a in args { self.check_expr(a, is_async, violations); }
            }
            Expr::Await { expr, span } => {
                // Check for awaiting blocking operations
                if self.is_blocking_future(expr) {
                    violations.push(Violation::full(
                        ViolationKind::ThreadUnsafe, span.clone().into(),
                        "Awaiting potentially blocking future".to_string(),
                        "Torture: Async task stuck waiting",
                        "Entry to Sukaramukha",
                        "Wrap blocking code in spawn_blocking()".to_string(),
                    ));
                }
                self.check_expr(expr, is_async, violations);
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left, is_async, violations);
                self.check_expr(right, is_async, violations);
            }
            _ => {}
        }
    }

    fn has_timeout_chain(&self, expr: &Expr) -> bool {
        match expr {
            Expr::MethodCall { method, receiver, .. } => {
                if method.name.contains("timeout") || method.name.contains("deadline") {
                    return true;
                }
                self.has_timeout_chain(receiver)
            }
            _ => false,
        }
    }

    fn is_blocking_future(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Call { callee, .. } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    self.blocking_ops.contains(id.name.as_str())
                } else { false }
            }
            _ => false,
        }
    }
}

impl Default for SukaramukhaChecker {
    fn default() -> Self { Self::new() }
}
