//! # Sandamsha - Hell 18: Unsafe Pincer
//!
//! Sin: Biting creatures
//! Code: Unsafe raw pointer operations

use super::super::yama::{Violation, ViolationKind};
use crate::parser::ast::{Ast, Expr, Item, Stmt, Block, LoopKind, UnaryOp};
use crate::errors::Span;
use std::collections::HashSet;

/// Checker for Sandamsha violations (unsafe pincer ops)
pub struct SandamshaChecker {
    /// Unsafe operations
    unsafe_ops: HashSet<&'static str>,
    /// Count of pointer dereferences
    deref_count: usize,
}

impl SandamshaChecker {
    pub fn new() -> Self {
        Self {
            unsafe_ops: ["transmute", "read_volatile", "write_volatile", "copy", "copy_nonoverlapping",
                         "offset", "add", "sub", "read", "write", "as_ptr", "as_mut_ptr",
                         "from_raw_parts", "from_raw_parts_mut", "slice_from_raw_parts"].into(),
            deref_count: 0,
        }
    }

    /// Check for unsafe raw pointer operations
    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.deref_count = 0;
                self.check_block(&func.body, &mut violations);

                // Warn if too many pointer dereferences
                if self.deref_count > 10 {
                    violations.push(Violation::full(
                        ViolationKind::FfiViolation, Span::dummy().into(),
                        format!("Function contains {} raw pointer dereferences", self.deref_count),
                        "Biting creatures: Excessive unsafe pointer operations",
                        "Entry to Sandamsha (pincer hell)",
                        "Refactor to minimize raw pointer usage".to_string(),
                    ));
                }
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

    fn check_expr(&mut self, expr: &Expr, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Unary { op, operand, span } => {
                // Raw pointer dereference
                if *op == UnaryOp::Deref {
                    self.deref_count += 1;
                    // Check for validation before dereference
                    if !self.has_null_check_nearby() {
                        violations.push(Violation::full(
                            ViolationKind::NullDeref, span.clone().into(),
                            "Pointer dereference without null check".to_string(),
                            "Biting creatures: No validation before bite",
                            "Entry to Sandamsha",
                            "Add is_null() check before dereference".to_string(),
                        ));
                    }
                }
                self.check_expr(operand, violations);
            }
            Expr::Call { callee, args, span } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    if self.unsafe_ops.contains(id.name.as_str()) {
                        violations.push(Violation::full(
                            ViolationKind::FfiViolation, span.clone().into(),
                            format!("Unsafe operation '{}' requires careful handling", id.name),
                            "Biting creatures: Dangerous operation",
                            "Entry to Sandamsha",
                            "Validate inputs and handle errors properly".to_string(),
                        ));
                    }
                }
                for a in args { self.check_expr(a, violations); }
            }
            Expr::MethodCall { method, receiver, args, span } => {
                if self.unsafe_ops.contains(method.name.as_str()) {
                    violations.push(Violation::full(
                        ViolationKind::FfiViolation, span.clone().into(),
                        format!("Unsafe method '{}' requires careful handling", method.name),
                        "Biting creatures: Dangerous method",
                        "Entry to Sandamsha",
                        "Validate inputs and handle errors properly".to_string(),
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

    fn has_null_check_nearby(&self) -> bool {
        // Would need more context to determine
        // For now, assume no check (safer to warn)
        false
    }
}

impl Default for SandamshaChecker {
    fn default() -> Self { Self::new() }
}
