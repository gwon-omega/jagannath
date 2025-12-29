//! # Andhakupa - Hell 9: Null Pointer Dereference
//!
//! Sin: Oppressing good people
//! Code: Null pointer dereference (falling into the dark well)

use super::super::yama::{Violation, ViolationKind};
use crate::parser::ast::{Ast, Block, Expr, Item, LoopKind, Stmt};
use std::collections::{HashMap, HashSet};

/// Nullability state
#[derive(Debug, Clone, PartialEq)]
enum NullState {
    NonNull,
    MaybeNull,
    Null,
    Checked, // After null check
}

/// Checker for Andhakupa violations (null dereference)
pub struct AndhakupaChecker {
    null_states: HashMap<String, NullState>,
    nullable_sources: HashSet<String>,
}

impl AndhakupaChecker {
    pub fn new() -> Self {
        let mut nullable_sources = HashSet::new();
        // Functions that may return null
        nullable_sources.insert("find".to_string());
        nullable_sources.insert("get".to_string());
        nullable_sources.insert("lookup".to_string());
        nullable_sources.insert("search".to_string());
        nullable_sources.insert("query".to_string());
        nullable_sources.insert("fetch".to_string());
        nullable_sources.insert("malloc".to_string());
        nullable_sources.insert("alloc".to_string());

        Self {
            null_states: HashMap::new(),
            nullable_sources,
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.null_states.clear();
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
                let state = if let Some(val) = value {
                    self.infer_null_state(val)
                } else {
                    NullState::MaybeNull // Uninitialized
                };
                self.null_states.insert(name.name.clone(), state);
            }
            Stmt::Expr(expr) => self.check_expr(expr, violations),
            Stmt::If {
                condition,
                then_block,
                else_block,
                ..
            } => {
                // Check if this is a null check
                if let Some(checked_var) = self.extract_null_check(condition) {
                    // Save state
                    let old_state = self.null_states.get(&checked_var).cloned();

                    // In then-block, variable is non-null (after check)
                    self.null_states
                        .insert(checked_var.clone(), NullState::Checked);
                    self.check_block(then_block, violations);

                    // Restore for else
                    if let Some(st) = old_state {
                        self.null_states.insert(checked_var, st);
                    }
                    if let Some(eb) = else_block {
                        self.check_block(eb, violations);
                    }
                } else {
                    self.check_expr(condition, violations);
                    self.check_block(then_block, violations);
                    if let Some(eb) = else_block {
                        self.check_block(eb, violations);
                    }
                }
            }
            Stmt::Loop { body, kind, .. } => {
                if let LoopKind::While { condition } = kind {
                    self.check_expr(condition, violations);
                }
                self.check_block(body, violations);
            }
            Stmt::Return { value: Some(v), .. } => self.check_expr(v, violations),
            _ => {}
        }
    }

    fn check_expr(&self, expr: &Expr, violations: &mut Vec<Violation>) {
        match expr {
            Expr::FieldAccess { object, span, .. } => {
                // Dereferencing - check if object may be null
                if let Expr::Identifier(id) = object.as_ref() {
                    if let Some(state) = self.null_states.get(&id.name) {
                        if matches!(state, NullState::MaybeNull | NullState::Null) {
                            violations.push(Violation::full(
                                ViolationKind::NullDeref,
                                span.clone().into(),
                                format!("Potential null dereference of '{}'", id.name),
                                "Falling into dark well: Null pointer access",
                                "Entry to Andhakupa",
                                "Check for null before dereferencing".to_string(),
                            ));
                        }
                    }
                }
                self.check_expr(object, violations);
            }
            Expr::MethodCall { receiver, span, .. } => {
                // Method call on potentially null
                if let Expr::Identifier(id) = receiver.as_ref() {
                    if let Some(state) = self.null_states.get(&id.name) {
                        if matches!(state, NullState::MaybeNull | NullState::Null) {
                            violations.push(Violation::full(
                                ViolationKind::NullDeref,
                                span.clone().into(),
                                format!("Method call on potentially null '{}'", id.name),
                                "Calling into the void: Method on null",
                                "Andhakupa warning",
                                "Add null check before method call".to_string(),
                            ));
                        }
                    }
                }
            }
            Expr::Unary { operand, span, .. } => {
                // Dereference operator
                if let Expr::Identifier(id) = operand.as_ref() {
                    if let Some(state) = self.null_states.get(&id.name) {
                        if matches!(state, NullState::MaybeNull | NullState::Null) {
                            violations.push(Violation::full(
                                ViolationKind::NullDeref,
                                span.clone().into(),
                                format!("Dereferencing potentially null '{}'", id.name),
                                "Reaching into darkness: Deref of null",
                                "Entry to Andhakupa",
                                "Ensure pointer is non-null".to_string(),
                            ));
                        }
                    }
                }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left, violations);
                self.check_expr(right, violations);
            }
            Expr::Call { args, .. } => {
                for arg in args {
                    self.check_expr(arg, violations);
                }
            }
            _ => {}
        }
    }

    fn infer_null_state(&self, expr: &Expr) -> NullState {
        match expr {
            Expr::Literal(_) => NullState::NonNull,
            Expr::Call { callee, .. } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    if self.nullable_sources.contains(&id.name) {
                        return NullState::MaybeNull;
                    }
                }
                NullState::NonNull
            }
            Expr::Identifier(id) => {
                if id.name == "null" || id.name == "nil" || id.name == "None" {
                    NullState::Null
                } else {
                    self.null_states
                        .get(&id.name)
                        .cloned()
                        .unwrap_or(NullState::NonNull)
                }
            }
            _ => NullState::NonNull,
        }
    }

    fn extract_null_check(&self, expr: &Expr) -> Option<String> {
        // Pattern: x != null, x != nil, x.is_some()
        if let Expr::Binary {
            left, right, op: _op, ..
        } = expr
        {
            if let Expr::Identifier(id) = left.as_ref() {
                if let Expr::Identifier(null_id) = right.as_ref() {
                    if matches!(null_id.name.as_str(), "null" | "nil" | "None") {
                        return Some(id.name.clone());
                    }
                }
            }
        }
        None
    }
}

impl Default for AndhakupaChecker {
    fn default() -> Self {
        Self::new()
    }
}
