//! # Taptasurmi - Hell 25: Hot Oil
//!
//! Sin: Causing burns
//! Code: CPU-intensive tight loops

use super::super::yama::{Violation, ViolationKind};
use crate::parser::ast::{Ast, Expr, Item, Stmt, Block, LoopKind};
use std::collections::HashSet;

/// Checker for Taptasurmi violations (CPU burn)
pub struct TaptasurmiChecker {
    /// CPU-intensive operations
    cpu_heavy: HashSet<&'static str>,
    /// Yielding functions
    yield_fns: HashSet<&'static str>,
}

impl TaptasurmiChecker {
    pub fn new() -> Self {
        Self {
            cpu_heavy: ["hash", "encrypt", "decrypt", "compress", "decompress",
                        "sort", "search", "compute", "calculate", "process"].into(),
            yield_fns: ["yield", "yield_now", "sleep", "park", "wait", "await"].into(),
        }
    }

    /// Check for CPU-intensive patterns
    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.check_block(&func.body, 0, &mut violations);
            }
        }
        violations
    }

    fn check_block(&self, block: &Block, loop_depth: usize, violations: &mut Vec<Violation>) {
        for stmt in &block.stmts {
            self.check_stmt(stmt, loop_depth, violations);
        }
    }

    fn check_stmt(&self, stmt: &Stmt, loop_depth: usize, violations: &mut Vec<Violation>) {
        match stmt {
            Stmt::Expr(e) => self.check_expr(e, loop_depth, violations),
            Stmt::Let { value: Some(v), .. } => self.check_expr(v, loop_depth, violations),
            Stmt::If { condition, then_block, else_block, .. } => {
                self.check_expr(condition, loop_depth, violations);
                self.check_block(then_block, loop_depth, violations);
                if let Some(eb) = else_block { self.check_block(eb, loop_depth, violations); }
            }
            Stmt::Loop { body, kind, span } => {
                let new_depth = loop_depth + 1;

                // Check for infinite/while loops without yield
                let is_hot = match kind {
                    LoopKind::Infinite => true,
                    LoopKind::While { .. } => true,
                    _ => false,
                };

                if is_hot {
                    if !self.has_yield(body) && !self.has_break(body) {
                        violations.push(Violation::full(
                            ViolationKind::ResourceExhaustion, span.clone().into(),
                            "Tight loop without yield or break".to_string(),
                            "Burning: CPU spinning without relief",
                            "Entry to Taptasurmi (hot oil hell)",
                            "Add yield(), sleep(), or ensure loop terminates".to_string(),
                        ));
                    }

                    // Extra check for nested loops
                    if new_depth >= 3 {
                        violations.push(Violation::full(
                            ViolationKind::ResourceExhaustion, span.clone().into(),
                            format!("Deeply nested loops (depth {})", new_depth),
                            "Burning: Exponential CPU usage",
                            "Entry to Taptasurmi",
                            "Consider algorithmic optimization or batch processing".to_string(),
                        ));
                    }
                }

                if let LoopKind::While { condition } = kind {
                    self.check_expr(condition, new_depth, violations);
                }
                self.check_block(body, new_depth, violations);
            }
            _ => {}
        }
    }

    fn check_expr(&self, expr: &Expr, loop_depth: usize, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Call { callee, args, span } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    // CPU-heavy operations in loops
                    if loop_depth > 0 && self.cpu_heavy.contains(id.name.as_str()) {
                        violations.push(Violation::full(
                            ViolationKind::ResourceExhaustion, span.clone().into(),
                            format!("CPU-intensive '{}' inside loop", id.name),
                            "Burning: Heavy computation in hot path",
                            "Entry to Taptasurmi",
                            "Move outside loop or batch operations".to_string(),
                        ));
                    }
                }
                for a in args { self.check_expr(a, loop_depth, violations); }
            }
            Expr::MethodCall { method, receiver, args, span } => {
                // Check for busy polling
                if loop_depth > 0 && (method.name == "is_ready" || method.name == "poll") {
                    violations.push(Violation::full(
                        ViolationKind::ResourceExhaustion, span.clone().into(),
                        format!("Busy polling with '{}' in loop", method.name),
                        "Burning: Spinning on ready check",
                        "Entry to Taptasurmi",
                        "Use blocking wait or async/await".to_string(),
                    ));
                }
                self.check_expr(receiver, loop_depth, violations);
                for a in args { self.check_expr(a, loop_depth, violations); }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left, loop_depth, violations);
                self.check_expr(right, loop_depth, violations);
            }
            _ => {}
        }
    }

    fn has_yield(&self, block: &Block) -> bool {
        for stmt in &block.stmts {
            match stmt {
                Stmt::Expr(Expr::Call { callee, .. }) => {
                    if let Expr::Identifier(id) = callee.as_ref() {
                        if self.yield_fns.contains(id.name.as_str()) {
                            return true;
                        }
                    }
                }
                Stmt::Expr(Expr::Await { .. }) => return true,
                Stmt::If { then_block, else_block, .. } => {
                    if self.has_yield(then_block) { return true; }
                    if let Some(eb) = else_block {
                        if self.has_yield(eb) { return true; }
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn has_break(&self, block: &Block) -> bool {
        for stmt in &block.stmts {
            match stmt {
                Stmt::Break { .. } => return true,
                Stmt::Return { .. } => return true,
                Stmt::If { then_block, else_block, .. } => {
                    if self.has_break(then_block) { return true; }
                    if let Some(eb) = else_block {
                        if self.has_break(eb) { return true; }
                    }
                }
                _ => {}
            }
        }
        false
    }
}

impl Default for TaptasurmiChecker {
    fn default() -> Self { Self::new() }
}
