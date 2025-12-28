//! # Kalasutra - Hell 6: Thread Violations
//!
//! Sin: Disrespecting elders (violating order)
//! Code: Thread safety violations, deadlocks, data races

use super::super::yama::{Violation, ViolationKind};
use crate::errors::Span;
use crate::parser::ast::{Ast, Block, Expr, Item, LoopKind, Stmt};
use std::collections::{HashMap, HashSet};

/// Checker for Kalasutra violations (thread safety)
pub struct KalasutraChecker {
    lock_order: HashMap<String, Vec<String>>,
    current_locks: Vec<String>,
    shared_vars: HashSet<String>,
}

impl KalasutraChecker {
    pub fn new() -> Self {
        Self {
            lock_order: HashMap::new(),
            current_locks: Vec::new(),
            shared_vars: HashSet::new(),
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        // First pass: identify shared state
        self.identify_shared_state(ast);
        // Second pass: check for violations
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.current_locks.clear();
                self.check_block(&func.body, &mut violations);
            }
        }
        // Check for deadlock cycles
        self.check_deadlock_cycles(&mut violations);
        violations
    }

    fn identify_shared_state(&mut self, ast: &Ast) {
        for item in &ast.items {
            if let Item::Function(func) = item {
                for stmt in &func.body.stmts {
                    if let Stmt::Let { name, ty, .. } = stmt {
                        if let Some(t) = ty {
                            let ts = format!("{:?}", t).to_lowercase();
                            if ts.contains("mutex")
                                || ts.contains("atomic")
                                || ts.contains("arc")
                                || ts.contains("shared")
                            {
                                self.shared_vars.insert(name.name.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    fn check_block(&mut self, block: &Block, violations: &mut Vec<Violation>) {
        for stmt in &block.stmts {
            self.check_stmt(stmt, violations);
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt, violations: &mut Vec<Violation>) {
        match stmt {
            Stmt::Expr(expr) => self.check_expr(expr, violations),
            Stmt::Let { value: Some(v), .. } => {
                self.check_lock_acquisition(v);
                self.check_expr(v, violations);
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
                span,
            } => {
                self.check_expr(condition, violations);
                let locks_before = self.current_locks.clone();
                self.check_block(then_block, violations);
                let locks_after_then = self.current_locks.clone();
                if let Some(eb) = else_block {
                    self.current_locks = locks_before.clone();
                    self.check_block(eb, violations);
                    if self.current_locks != locks_after_then {
                        violations.push(Violation::full(
                            ViolationKind::ThreadUnsafe,
                            span.clone().into(),
                            "Lock imbalance between branches".to_string(),
                            "Disrespecting order: Different lock states",
                            "Kalasutra warning",
                            "Balance locks in branches".to_string(),
                        ));
                    }
                }
            }
            Stmt::Loop { body, span, kind } => {
                let locks_before = self.current_locks.len();
                self.check_block(body, violations);
                if self.current_locks.len() > locks_before {
                    violations.push(Violation::full(
                        ViolationKind::ThreadUnsafe,
                        span.clone().into(),
                        "Lock acquisition in loop without release".to_string(),
                        "Disrespecting order: Lock starvation",
                        "Kalasutra warning",
                        "Release locks before loop iteration".to_string(),
                    ));
                }
            }
            _ => {}
        }
    }

    fn check_expr(&mut self, expr: &Expr, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Call { callee, span, .. } => {
                // Check for try_lock pattern
                if let Expr::Identifier(id) = callee.as_ref() {
                    if id.name == "try_lock" {
                        violations.push(Violation::full(
                            ViolationKind::ThreadUnsafe,
                            span.clone().into(),
                            "try_lock may cause busy-waiting".to_string(),
                            "Disrespecting order: Spinning on lock",
                            "Kalasutra warning",
                            "Use blocking lock or timeout".to_string(),
                        ));
                    }
                }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left, violations);
                self.check_expr(right, violations);
            }
            _ => {}
        }
    }

    fn check_lock_acquisition(&mut self, expr: &Expr) {
        if let Expr::MethodCall {
            receiver, method, ..
        } = expr
        {
            let method_name = method.name.to_lowercase();
            if method_name == "lock" || method_name == "write" || method_name == "read" {
                if let Expr::Identifier(id) = receiver.as_ref() {
                    // Record lock ordering for deadlock detection
                    for held in &self.current_locks {
                        self.lock_order
                            .entry(held.clone())
                            .or_default()
                            .push(id.name.clone());
                    }
                    self.current_locks.push(id.name.clone());
                }
            }
        }
    }

    fn check_deadlock_cycles(&self, violations: &mut Vec<Violation>) {
        // Simple cycle detection in lock graph
        for (lock, successors) in &self.lock_order {
            for succ in successors {
                if let Some(succ_succs) = self.lock_order.get(succ) {
                    if succ_succs.contains(lock) {
                        violations.push(Violation::full(
                            ViolationKind::Deadlock,
                            Span::dummy().into(),
                            format!("Deadlock cycle: {} <-> {}", lock, succ),
                            "Disrespecting order: Circular lock dependency",
                            "Entry to Kalasutra: Deadlock",
                            "Use consistent lock ordering".to_string(),
                        ));
                    }
                }
            }
        }
    }
}

impl Default for KalasutraChecker {
    fn default() -> Self {
        Self::new()
    }
}
