//! # Tamisram - Hell 1: Memory Theft
//!
//! Sin: Stealing others' wealth
//! Code: Use-after-free, double-free

use super::super::yama::{Violation, ViolationKind};
use crate::lexer::token::Span;
use crate::parser::ast::{Ast, Block, Expr, Item, LoopKind, Stmt};
use std::collections::HashMap;

/// Memory state for a variable
#[derive(Debug, Clone, PartialEq)]
enum MemoryState {
    Alive,
    Freed,
    Moved,
}

/// Checker for Tamisram violations (memory theft - use-after-free, double-free)
pub struct TamisramChecker {
    memory_states: HashMap<String, MemoryState>,
    freed_locations: HashMap<String, Span>,
}

impl TamisramChecker {
    pub fn new() -> Self {
        Self {
            memory_states: HashMap::new(),
            freed_locations: HashMap::new(),
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.memory_states.clear();
                self.freed_locations.clear();
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
                self.memory_states
                    .insert(name.name.clone(), MemoryState::Alive);
                if let Some(v) = value {
                    self.check_expr_use(v, violations);
                }
            }
            Stmt::Expr(expr) => {
                if let Expr::Call { callee, args, span } = expr {
                    if let Expr::Identifier(id) = callee.as_ref() {
                        if matches!(
                            id.name.as_str(),
                            "free" | "drop" | "mukta" | "release" | "deallocate"
                        ) {
                            if let Some(Expr::Identifier(arg_id)) = args.first() {
                                self.handle_free(&arg_id.name, span.clone(), violations);
                            }
                        }
                    }
                }
                self.check_expr_use(expr, violations);
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
                ..
            } => {
                self.check_expr_use(condition, violations);
                let state_before = self.memory_states.clone();
                self.check_block(then_block, violations);
                let then_state = self.memory_states.clone();
                self.memory_states = state_before;
                if let Some(eb) = else_block {
                    self.check_block(eb, violations);
                }
                // Merge: if freed in either branch, mark as potentially freed
                for (var, st) in &then_state {
                    if *st == MemoryState::Freed {
                        self.memory_states.insert(var.clone(), MemoryState::Freed);
                    }
                }
            }
            Stmt::Loop { body, kind, .. } => {
                if let LoopKind::While { condition } = kind {
                    self.check_expr_use(condition, violations);
                }
                self.check_block(body, violations);
            }
            Stmt::Return { value: Some(v), .. } => self.check_expr_use(v, violations),
            _ => {}
        }
    }

    fn handle_free(&mut self, var_name: &str, span: Span, violations: &mut Vec<Violation>) {
        if let Some(MemoryState::Freed) = self.memory_states.get(var_name) {
            violations.push(Violation::full(
                ViolationKind::DoubleFree,
                span.clone().into(),
                format!("Variable '{}' freed twice", var_name),
                "Stealing twice: Double-free corrupts memory",
                "Entry to Tamisram (deepest darkness)",
                "Remove duplicate free".to_string(),
            ));
        } else {
            self.memory_states
                .insert(var_name.to_string(), MemoryState::Freed);
            self.freed_locations.insert(var_name.to_string(), span);
        }
    }

    fn check_expr_use(&self, expr: &Expr, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Identifier(id) => {
                if let Some(MemoryState::Freed) = self.memory_states.get(&id.name) {
                    violations.push(Violation::full(
                        ViolationKind::UseAfterFree,
                        id.span.clone().into(),
                        format!("Use of freed variable '{}'", id.name),
                        "Stealing from the dead: Use-after-free",
                        "Entry to Tamisram (memory theft)",
                        "Do not use after freeing".to_string(),
                    ));
                }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr_use(left, violations);
                self.check_expr_use(right, violations);
            }
            Expr::Call { args, .. } => {
                for arg in args {
                    self.check_expr_use(arg, violations);
                }
            }
            Expr::FieldAccess { object, .. } | Expr::Index { object, .. } => {
                self.check_expr_use(object, violations);
            }
            _ => {}
        }
    }
}

impl Default for TamisramChecker {
    fn default() -> Self {
        Self::new()
    }
}
