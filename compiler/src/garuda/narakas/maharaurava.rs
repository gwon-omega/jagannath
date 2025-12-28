//! # Maharaurava - Hell 4: Process Killing
//!
//! Sin: Killing living beings
//! Code: Killing child processes, forced termination

use super::super::yama::{Violation, ViolationKind};
use crate::parser::ast::{Ast, Expr, Item, Stmt, Block, LoopKind};
use crate::errors::Span;
use std::collections::HashSet;

/// Checker for Maharaurava violations (process killing)
pub struct MaharauravaChecker {
    /// Process killing functions
    kill_fns: HashSet<&'static str>,
    /// Graceful termination functions
    graceful_fns: HashSet<&'static str>,
    /// Track spawned processes
    spawned: HashSet<String>,
}

impl MaharauravaChecker {
    pub fn new() -> Self {
        Self {
            kill_fns: ["kill", "terminate", "abort", "exit", "_exit", "sigkill",
                       "process_kill", "force_stop", "destroy"].into(),
            graceful_fns: ["shutdown", "stop", "sigterm", "request_stop", "graceful_stop"].into(),
            spawned: HashSet::new(),
        }
    }

    /// Check for forced process termination
    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.spawned.clear();
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
                // Track spawned processes
                if self.is_spawn(v) {
                    self.spawned.insert(name.name.clone());
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

    fn check_expr(&mut self, expr: &Expr, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Call { callee, args, span } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    // Check for kill operations
                    if self.kill_fns.contains(id.name.as_str()) {
                        let is_sigkill = args.iter().any(|a| {
                            if let Expr::Identifier(arg_id) = a {
                                arg_id.name.contains("SIGKILL") || arg_id.name == "9"
                            } else { false }
                        });

                        if is_sigkill || id.name == "kill" {
                            violations.push(Violation::full(
                                ViolationKind::ForcedTermination, span.clone().into(),
                                format!("Forced process termination via '{}'", id.name),
                                "Killing beings: Denying graceful shutdown",
                                "Entry to Maharaurava (great screaming hell)",
                                "Use graceful_stop() or SIGTERM first".to_string(),
                            ));
                        }
                    }

                    // Check for abort without cleanup
                    if id.name == "abort" || id.name == "_exit" {
                        violations.push(Violation::full(
                            ViolationKind::ForcedTermination, span.clone().into(),
                            format!("Immediate termination via '{}' skips cleanup", id.name),
                            "Killing beings: No chance for final rites",
                            "Entry to Maharaurava",
                            "Use exit() with cleanup handlers".to_string(),
                        ));
                    }
                }
                for a in args { self.check_expr(a, violations); }
            }
            Expr::MethodCall { method, receiver, args, span } => {
                // Check for process.kill() pattern
                if self.kill_fns.contains(method.name.as_str()) {
                    if let Expr::Identifier(obj_id) = receiver.as_ref() {
                        if self.spawned.contains(&obj_id.name) {
                            violations.push(Violation::full(
                                ViolationKind::ForcedTermination, span.clone().into(),
                                format!("Killing spawned process '{}'", obj_id.name),
                                "Killing beings: Terminating child without warning",
                                "Entry to Maharaurava",
                                "Request graceful shutdown first".to_string(),
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

    fn is_spawn(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Call { callee, .. } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    ["spawn", "fork", "exec", "process", "create_process", "thread_create"]
                        .contains(&id.name.as_str())
                } else { false }
            }
            Expr::MethodCall { method, .. } => {
                ["spawn", "fork", "start"].contains(&method.name.as_str())
            }
            _ => false,
        }
    }
}

impl Default for MaharauravaChecker {
    fn default() -> Self { Self::new() }
}
