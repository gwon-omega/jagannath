//! # Visasana - Hell 17: Forced Termination
//!
//! Sin: Selling wife, imprisoning
//! Code: Forced process termination, kill -9

use crate::parser::ast::{Ast, Expr, Item, Stmt, Block, LoopKind, Literal};
use super::super::yama::{Violation, ViolationKind};
use std::collections::HashSet;

/// Checker for Visasana violations (forced termination)
pub struct VisasanaChecker {
    /// Forced termination functions
    force_term: HashSet<&'static str>,
    /// Cleanup bypass functions
    cleanup_bypass: HashSet<&'static str>,
}

impl VisasanaChecker {
    pub fn new() -> Self {
        Self {
            force_term: ["kill", "terminate", "abort", "_exit", "force_exit",
                         "sigkill", "hard_stop", "force_shutdown"].into(),
            cleanup_bypass: ["_exit", "abort", "quick_exit", "terminate_now"].into(),
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.check_block(&func.body, &mut violations);
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
            Stmt::Expr(e) => self.check_expr(e, violations),
            Stmt::Let { value: Some(v), .. } => self.check_expr(v, violations),
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
                    // Check for forced termination
                    if self.force_term.contains(id.name.as_str()) {
                        let is_sigkill = args.iter().any(|a| {
                            match a {
                                Expr::Identifier(arg_id) => {
                                    arg_id.name.contains("SIGKILL") || arg_id.name == "9"
                                }
                                Expr::Literal(Literal::Int(n)) => *n == 9,
                                _ => false,
                            }
                        });

                        if is_sigkill {
                            violations.push(Violation::full(
                                ViolationKind::ForcedTermination, span.clone().into(),
                                "SIGKILL/signal 9 detected - no cleanup possible".to_string(),
                                "Selling/imprisoning: Denying final rites",
                                "Entry to Visasana (torture hell)",
                                "Use SIGTERM first, allow graceful shutdown".to_string(),
                            ));
                        } else {
                            violations.push(Violation::full(
                                ViolationKind::ForcedTermination, span.clone().into(),
                                format!("Forced termination via '{}'", id.name),
                                "Selling/imprisoning: Forceful process death",
                                "Entry to Visasana",
                                "Request graceful shutdown first".to_string(),
                            ));
                        }
                    }

                    // Check for cleanup bypass (also uses ForcedTermination)
                    if self.cleanup_bypass.contains(id.name.as_str()) {
                        violations.push(Violation::full(
                            ViolationKind::ForcedTermination, span.clone().into(),
                            format!("'{}' bypasses cleanup handlers", id.name),
                            "Selling/imprisoning: No funeral rites",
                            "Entry to Visasana",
                            "Use exit() to run cleanup handlers".to_string(),
                        ));
                    }
                }
                for a in args { self.check_expr(a, violations); }
            }
            Expr::MethodCall { method, receiver, args, span } => {
                if self.force_term.contains(method.name.as_str()) {
                    violations.push(Violation::full(
                        ViolationKind::ForcedTermination, span.clone().into(),
                        format!("Forced termination via method '{}'", method.name),
                        "Selling/imprisoning: Forceful object death",
                        "Entry to Visasana",
                        "Implement graceful shutdown method".to_string(),
                    ));
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
}

impl Default for VisasanaChecker {
    fn default() -> Self { Self::new() }
}
