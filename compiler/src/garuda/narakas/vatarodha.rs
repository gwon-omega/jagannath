//! # Vatarodha - Hell 26: DoS Attack
//!
//! Sin: Persecuting forest animals
//! Code: Denial of service patterns

use super::super::yama::{Violation, ViolationKind};
use crate::parser::ast::{Ast, Block, Expr, Item, LoopKind, Stmt};
use std::collections::HashSet;

/// Checker for Vatarodha violations (DoS)
pub struct VatarodhaChecker {
    /// Functions that consume resources
    resource_intensive: HashSet<&'static str>,
    /// Network/system functions
    network_fns: HashSet<&'static str>,
}

impl VatarodhaChecker {
    pub fn new() -> Self {
        Self {
            resource_intensive: [
                "fork",
                "spawn",
                "clone",
                "thread_create",
                "open",
                "socket",
                "allocate",
                "malloc",
                "mmap",
                "create_file",
            ]
            .into(),
            network_fns: ["connect", "listen", "accept", "send", "recv", "bind"].into(),
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.check_block(&func.body, false, &mut violations);
            }
        }
        violations
    }

    fn check_block(&self, block: &Block, in_loop: bool, violations: &mut Vec<Violation>) {
        for stmt in &block.stmts {
            self.check_stmt(stmt, in_loop, violations);
        }
    }

    fn check_stmt(&self, stmt: &Stmt, in_loop: bool, violations: &mut Vec<Violation>) {
        match stmt {
            Stmt::Expr(e) => self.check_expr(e, in_loop, violations),
            Stmt::Let { value: Some(v), .. } => self.check_expr(v, in_loop, violations),
            Stmt::If {
                condition,
                then_block,
                else_block,
                ..
            } => {
                self.check_expr(condition, in_loop, violations);
                self.check_block(then_block, in_loop, violations);
                if let Some(eb) = else_block {
                    self.check_block(eb, in_loop, violations);
                }
            }
            Stmt::Loop { body, kind, span } => {
                // Check for unbounded loops with resource allocation
                let is_unbounded = matches!(kind, LoopKind::Infinite);
                if is_unbounded {
                    self.check_dos_loop(body, span, violations);
                }
                if let LoopKind::While { condition } = kind {
                    self.check_expr(condition, true, violations);
                }
                self.check_block(body, true, violations);
            }
            _ => {}
        }
    }

    fn check_expr(&self, expr: &Expr, in_loop: bool, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Call { callee, args, span } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    // Resource-intensive operations in loops
                    if in_loop && self.resource_intensive.contains(id.name.as_str()) {
                        violations.push(Violation::full(
                            ViolationKind::DoS,
                            span.clone().into(),
                            format!("Resource-intensive '{}' in loop", id.name),
                            "Persecution: Exhausting system resources",
                            "Entry to Vatarodha (animal persecution hell)",
                            "Add rate limiting or resource cap".to_string(),
                        ));
                    }
                    // Unrestricted network operations
                    if self.network_fns.contains(id.name.as_str()) && !self.has_timeout_arg(args) {
                        violations.push(Violation::full(
                            ViolationKind::DoS,
                            span.clone().into(),
                            format!("Network operation '{}' without timeout", id.name),
                            "Persecution: Potential connection exhaustion",
                            "Entry to Vatarodha",
                            "Add timeout parameter".to_string(),
                        ));
                    }
                }
                for a in args {
                    self.check_expr(a, in_loop, violations);
                }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left, in_loop, violations);
                self.check_expr(right, in_loop, violations);
            }
            _ => {}
        }
    }

    fn check_dos_loop(
        &self,
        body: &Block,
        span: &crate::lexer::token::Span,
        violations: &mut Vec<Violation>,
    ) {
        // Check for fork bombs or resource exhaustion patterns
        for stmt in &body.stmts {
            if let Stmt::Expr(Expr::Call { callee, .. }) = stmt {
                if let Expr::Identifier(id) = callee.as_ref() {
                    if id.name == "fork" || id.name == "spawn" {
                        violations.push(Violation::full(
                            ViolationKind::DoS,
                            span.clone().into(),
                            "Fork/spawn in infinite loop (potential fork bomb)".to_string(),
                            "Persecution: Creating exponential processes",
                            "Entry to Vatarodha",
                            "Add process limit or use thread pool".to_string(),
                        ));
                    }
                }
            }
        }
    }

    fn has_timeout_arg(&self, args: &[Expr]) -> bool {
        args.iter().any(|a| {
            if let Expr::Identifier(id) = a {
                id.name.contains("timeout") || id.name.contains("deadline")
            } else {
                false
            }
        })
    }
}

impl Default for VatarodhaChecker {
    fn default() -> Self {
        Self::new()
    }
}
