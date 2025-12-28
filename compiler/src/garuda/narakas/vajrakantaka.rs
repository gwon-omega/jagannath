//! # Vajrakantaka - Hell 13: FFI Violations
//!
//! Sin: Intercourse with animals
//! Code: Accessing foreign memory unsafely

use super::super::yama::{Violation, ViolationKind};
use crate::parser::ast::{Ast, Expr, Item, Stmt, Block, LoopKind};
use std::collections::HashSet;

/// Checker for Vajrakantaka violations (FFI violations)
pub struct VajrakantakaChecker {
    /// FFI call patterns
    ffi_calls: HashSet<&'static str>,
    /// Unsafe FFI types
    unsafe_types: HashSet<&'static str>,
}

impl VajrakantakaChecker {
    pub fn new() -> Self {
        Self {
            ffi_calls: ["extern_call", "c_call", "ffi_invoke", "call_c", "dlsym",
                        "GetProcAddress", "invoke_foreign"].into(),
            unsafe_types: ["*mut", "*const", "c_void", "c_char", "c_int"].into(),
        }
    }

    /// Check for FFI safety violations
    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.check_block(&func.body, &mut violations);
            }
            // Note: Item::Extern not in AST - FFI detected via call patterns
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
            Stmt::Let { value: Some(v), ty, span, .. } => {
                // Check for unsafe FFI types in declarations
                if let Some(t) = ty {
                    let ty_str = format!("{:?}", t);
                    if self.unsafe_types.iter().any(|ut| ty_str.contains(ut)) {
                        violations.push(Violation::full(
                            ViolationKind::FfiViolation, span.clone().into(),
                            "Unsafe FFI type in declaration".to_string(),
                            "Foreign intercourse: Raw pointer from foreign land",
                            "Entry to Vajrakantaka",
                            "Use safe wrapper type".to_string(),
                        ));
                    }
                }
                self.check_expr(v, violations);
            }
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
                    if self.ffi_calls.contains(id.name.as_str()) {
                        // FFI call detected
                        if !self.args_validated(args) {
                            violations.push(Violation::full(
                                ViolationKind::FfiViolation, span.clone().into(),
                                format!("FFI call '{}' with potentially unvalidated arguments", id.name),
                                "Foreign intercourse: Sending unverified data",
                                "Entry to Vajrakantaka",
                                "Validate and sanitize arguments before FFI call".to_string(),
                            ));
                        }
                    }
                }
                for a in args { self.check_expr(a, violations); }
            }
            Expr::Cast { expr: inner, ty, span } => {
                // Casting to raw pointer for FFI
                let ty_str = format!("{:?}", ty);
                if self.unsafe_types.iter().any(|ut| ty_str.contains(ut)) {
                    violations.push(Violation::full(
                        ViolationKind::FfiViolation, span.clone().into(),
                        "Casting to unsafe FFI type".to_string(),
                        "Foreign intercourse: Creating raw pointer",
                        "Entry to Vajrakantaka",
                        "Ensure lifetime and validity of cast pointer".to_string(),
                    ));
                }
                self.check_expr(inner, violations);
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left, violations);
                self.check_expr(right, violations);
            }
            _ => {}
        }
    }

    fn args_validated(&self, _args: &[Expr]) -> bool {
        // Would need dataflow analysis - conservatively return false
        false
    }
}

impl Default for VajrakantakaChecker {
    fn default() -> Self { Self::new() }
}
