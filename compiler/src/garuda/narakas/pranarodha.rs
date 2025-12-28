//! # Pranarodha - Hell 16: Deadlock
//!
//! Sin: Imprisoning innocents
//! Code: Deadlock (process cannot breathe/continue)

use super::super::yama::{Violation, ViolationKind};
use crate::errors::Span;
use crate::parser::ast::{Ast, Block, Expr, Item, LoopKind, Stmt};
use std::collections::{HashMap, HashSet};

/// Lock state for tracking
#[derive(Clone, Debug)]
enum LockState {
    Unlocked,
    Locked { order: usize },
}

/// Checker for Pranarodha violations (deadlock)
pub struct PranarodhaChecker {
    /// Wait-for graph for cycle detection
    wait_graph: HashMap<String, HashSet<String>>,
    /// Lock ordering
    lock_order: HashMap<String, usize>,
    /// Current held locks
    held_locks: Vec<String>,
    /// Lock functions
    lock_fns: HashSet<&'static str>,
    /// Unlock functions
    unlock_fns: HashSet<&'static str>,
}

impl PranarodhaChecker {
    pub fn new() -> Self {
        let lock_fns: HashSet<&'static str> = [
            "lock",
            "acquire",
            "mutex_lock",
            "write_lock",
            "read_lock",
            "spin_lock",
        ]
        .into();
        let unlock_fns: HashSet<&'static str> =
            ["unlock", "release", "mutex_unlock", "drop", "spin_unlock"].into();
        Self {
            wait_graph: HashMap::new(),
            lock_order: HashMap::new(),
            held_locks: Vec::new(),
            lock_fns,
            unlock_fns,
        }
    }

    /// Check for deadlock patterns
    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.held_locks.clear();
                self.check_block(&func.body, &mut violations);
            }
        }
        self.detect_deadlock_cycles(&mut violations);
        violations
    }

    fn check_block(&mut self, block: &Block, violations: &mut Vec<Violation>) {
        for stmt in &block.stmts {
            self.check_stmt(stmt, violations);
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt, violations: &mut Vec<Violation>) {
        match stmt {
            Stmt::Expr(e) => self.check_expr(e, violations),
            Stmt::Let { value: Some(v), .. } => self.check_expr(v, violations),
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
                if let Expr::MethodCall { method, .. } = callee.as_ref() {
                    self.handle_lock_call(&method.name, args, span, violations);
                } else if let Expr::Identifier(id) = callee.as_ref() {
                    self.handle_lock_call(&id.name, args, span, violations);
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
                self.handle_lock_call(&method.name, args, span, violations);
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

    fn handle_lock_call(
        &mut self,
        fn_name: &str,
        args: &[Expr],
        span: &crate::lexer::token::Span,
        violations: &mut Vec<Violation>,
    ) {
        if self.lock_fns.contains(fn_name) {
            // Extract lock name from first arg
            if let Some(Expr::Identifier(id)) = args.first() {
                let lock_name = id.name.clone();
                let new_order = self.lock_order.len();
                let order = *self
                    .lock_order
                    .entry(lock_name.clone())
                    .or_insert(new_order);

                // Check lock ordering violation
                for held in &self.held_locks {
                    if let Some(&held_order) = self.lock_order.get(held) {
                        if order < held_order {
                            violations.push(Violation::full(
                                ViolationKind::Deadlock, span.clone().into(),
                                format!("Lock ordering violation: '{}' (order {}) acquired after '{}' (order {})", lock_name, order, held, held_order),
                                "Imprisoning innocents: Locks acquired in wrong order",
                                "Entry to Pranarodha (breathless hell)",
                                "Acquire locks in consistent order".to_string(),
                            ));
                        }
                    }
                }

                // Build wait-for graph
                for held in &self.held_locks {
                    self.wait_graph
                        .entry(lock_name.clone())
                        .or_default()
                        .insert(held.clone());
                }
                self.held_locks.push(lock_name);
            }
        } else if self.unlock_fns.contains(fn_name) {
            if let Some(Expr::Identifier(id)) = args.first() {
                self.held_locks.retain(|l| l != &id.name);
            }
        }
    }

    fn detect_deadlock_cycles(&self, violations: &mut Vec<Violation>) {
        // Simple cycle detection in wait-for graph
        for start in self.wait_graph.keys() {
            let mut visited = HashSet::new();
            let mut path = Vec::new();
            if self.has_cycle(start, &mut visited, &mut path) {
                violations.push(Violation::full(
                    ViolationKind::Deadlock,
                    Span::dummy().into(),
                    format!("Potential deadlock cycle: {:?}", path),
                    "Mutual imprisonment: Locks waiting on each other",
                    "Entry to Pranarodha",
                    "Restructure locking to break cycle".to_string(),
                ));
            }
        }
    }

    fn has_cycle(&self, node: &str, visited: &mut HashSet<String>, path: &mut Vec<String>) -> bool {
        if path.contains(&node.to_string()) {
            return true;
        }
        if visited.contains(node) {
            return false;
        }
        visited.insert(node.to_string());
        path.push(node.to_string());
        if let Some(deps) = self.wait_graph.get(node) {
            for dep in deps {
                if self.has_cycle(dep, visited, path) {
                    return true;
                }
            }
        }
        path.pop();
        false
    }
}

impl Default for PranarodhaChecker {
    fn default() -> Self {
        Self::new()
    }
}
