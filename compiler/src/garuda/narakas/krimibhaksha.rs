//! # Krimibhaksha - Hell 10: Memory Corruption
//!
//! Sin: Dishonoring guests
//! Code: Memory corruption, data worms

use crate::parser::ast::{Ast, Expr, Item, Stmt, Block, LoopKind, BinaryOp, UnaryOp};
use crate::errors::Span;
use super::super::yama::{Violation, ViolationKind};
use std::collections::HashSet;

/// Checker for Krimibhaksha violations (memory corruption)
pub struct KrimibhakshaChecker {
    /// Unsafe write functions
    unsafe_write_fns: HashSet<&'static str>,
    /// Type confusion prone operations
    cast_ops: HashSet<&'static str>,
}

impl KrimibhakshaChecker {
    pub fn new() -> Self {
        Self {
            unsafe_write_fns: ["memcpy", "memmove", "strcpy", "strcat", "sprintf", "gets",
                               "write_raw", "copy_bytes", "unsafe_write", "transmute"].into(),
            cast_ops: ["transmute", "reinterpret_cast", "unsafe_cast", "bit_cast"].into(),
        }
    }

    /// Check for memory corruption patterns
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
                    // Check for dangerous memory functions
                    if self.unsafe_write_fns.contains(id.name.as_str()) {
                        let severity = self.get_severity(&id.name);
                        violations.push(Violation::full(
                            ViolationKind::MemoryCorruption, span.clone().into(),
                            format!("Use of dangerous function '{}' - {}", id.name, severity),
                            "Dishonoring guests: Corrupting memory data",
                            "Entry to Krimibhaksha (worm-eating hell)",
                            self.get_safe_alternative(&id.name),
                        ));
                    }
                    // Check for type confusion casts
                    if self.cast_ops.contains(id.name.as_str()) {
                        violations.push(Violation::full(
                            ViolationKind::TypeConfusion, span.clone().into(),
                            format!("Unsafe type conversion '{}'", id.name),
                            "Dishonoring guests: Type confusion attack",
                            "Entry to Krimibhaksha",
                            "Use safe type conversion with validation".to_string(),
                        ));
                    }
                }
                for a in args { self.check_expr(a, violations); }
            }
            Expr::MethodCall { method, receiver, args, span } => {
                // Check for unsafe slice operations
                if method.name == "set_len" || method.name == "from_raw_parts" {
                    violations.push(Violation::full(
                        ViolationKind::MemoryCorruption, span.clone().into(),
                        format!("Unsafe operation '{}' can corrupt memory", method.name),
                        "Dishonoring guests: Manipulating memory unsafely",
                        "Entry to Krimibhaksha",
                        "Validate bounds and use safe wrapper".to_string(),
                    ));
                }
                self.check_expr(receiver, violations);
                for a in args { self.check_expr(a, violations); }
            }
            Expr::Index { object, index, span } => {
                // Check for unchecked indexing
                if !self.is_bounds_checked(object) {
                    violations.push(Violation::full(
                        ViolationKind::MemoryCorruption, span.clone().into(),
                        "Unchecked array indexing".to_string(),
                        "Dishonoring guests: Potential out-of-bounds access",
                        "Entry to Krimibhaksha",
                        "Use .get() with bounds checking or slice patterns".to_string(),
                    ));
                }
                self.check_expr(object, violations);
                self.check_expr(index, violations);
            }
            Expr::Binary { left, right, op, span } => {
                // Check for pointer arithmetic
                if *op == BinaryOp::Add || *op == BinaryOp::Sub {
                    if self.looks_like_pointer(left) || self.looks_like_pointer(right) {
                        violations.push(Violation::full(
                            ViolationKind::MemoryCorruption, span.clone().into(),
                            "Raw pointer arithmetic detected".to_string(),
                            "Dishonoring guests: Uncontrolled memory access",
                            "Entry to Krimibhaksha",
                            "Use slice indexing or iterator methods".to_string(),
                        ));
                    }
                }
                self.check_expr(left, violations);
                self.check_expr(right, violations);
            }
            _ => {}
        }
    }

    fn get_severity(&self, fn_name: &str) -> &'static str {
        match fn_name {
            "gets" => "CRITICAL: unbounded read",
            "strcpy" | "strcat" => "HIGH: no bounds checking",
            "sprintf" => "HIGH: format string vulnerable",
            "memcpy" | "memmove" => "MEDIUM: requires careful size",
            "transmute" => "CRITICAL: arbitrary type conversion",
            _ => "MEDIUM",
        }
    }

    fn get_safe_alternative(&self, fn_name: &str) -> String {
        match fn_name {
            "gets" => "Use fgets() with size limit or readline()".to_string(),
            "strcpy" => "Use strncpy() or snprintf()".to_string(),
            "strcat" => "Use strncat() with size checking".to_string(),
            "sprintf" => "Use snprintf() with size limit".to_string(),
            "memcpy" => "Validate size before copying".to_string(),
            "transmute" => "Use safe conversion traits".to_string(),
            _ => "Use safe alternative".to_string(),
        }
    }

    fn is_bounds_checked(&self, _expr: &Expr) -> bool {
        // Would need type info to determine if bounds checked
        false
    }

    fn looks_like_pointer(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Identifier(id) => id.name.ends_with("_ptr") || id.name.starts_with("p_"),
            Expr::Unary { op, .. } if *op == UnaryOp::Deref || *op == UnaryOp::Ref => true,
            _ => false,
        }
    }
}

impl Default for KrimibhakshaChecker {
    fn default() -> Self { Self::new() }
}
