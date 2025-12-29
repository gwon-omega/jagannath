//! # Puyoda - Hell 15: Data Corruption
//!
//! Sin: Deceiving women, false promises
//! Code: Data corruption, malformed structures

use super::super::yama::{Violation, ViolationKind};
use crate::parser::ast::{Ast, BinaryOp, Block, Expr, Item, LoopKind, Stmt};
use std::collections::HashSet;

/// Checker for Puyoda violations (data corruption)
pub struct PuyodaChecker {
    /// Operations that can corrupt data
    corrupt_ops: HashSet<&'static str>,
    /// Type manipulation functions
    type_manip: HashSet<&'static str>,
}

impl PuyodaChecker {
    pub fn new() -> Self {
        Self {
            corrupt_ops: [
                "memset",
                "bzero",
                "clear",
                "overwrite",
                "truncate",
                "bit_flip",
                "corrupt",
                "mangle",
            ]
            .into(),
            type_manip: ["reinterpret", "union_access", "transmute", "bit_cast"].into(),
        }
    }

    /// Check for data corruption patterns
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

    fn check_expr(&self, expr: &Expr, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Call { callee, args, span } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    // Check for corruption operations
                    if self.corrupt_ops.contains(id.name.as_str()) {
                        violations.push(Violation::full(
                            ViolationKind::DataCorruption,
                            span.clone().into(),
                            format!("Potentially corrupting operation '{}'", id.name),
                            "Deception: Destroying data integrity",
                            "Entry to Puyoda (pus-filled hell)",
                            "Validate data before modification".to_string(),
                        ));
                    }
                    // Check for unsafe type manipulation
                    if self.type_manip.contains(id.name.as_str()) {
                        violations.push(Violation::full(
                            ViolationKind::TypeConfusion,
                            span.clone().into(),
                            format!("Type manipulation via '{}' can corrupt structure", id.name),
                            "Deception: Breaking type safety",
                            "Entry to Puyoda",
                            "Use safe type conversions".to_string(),
                        ));
                    }
                }
                for a in args {
                    self.check_expr(a, violations);
                }
            }
            Expr::Index {
                object,
                index,
                span: _span,
            } => {
                // Writing through unchecked index
                self.check_expr(object, violations);
                self.check_expr(index, violations);
            }
            Expr::FieldAccess {
                object,
                field,
                span,
            } => {
                // Check for union field access (type punning)
                if self.is_union_access(object) {
                    violations.push(Violation::full(
                        ViolationKind::DataCorruption,
                        span.clone().into(),
                        format!(
                            "Union field '{}' access - potential type punning",
                            field.name
                        ),
                        "Deception: Viewing data as different type",
                        "Entry to Puyoda",
                        "Use explicit conversion functions".to_string(),
                    ));
                }
                self.check_expr(object, violations);
            }
            Expr::Binary {
                left,
                right,
                op,
                span,
            } => {
                // Check for bitwise operations on non-integer types
                if matches!(
                    op,
                    BinaryOp::BitOr
                        | BinaryOp::BitAnd
                        | BinaryOp::BitXor
                        | BinaryOp::Shl
                        | BinaryOp::Shr
                ) {
                    if self.is_potentially_non_numeric(left)
                        || self.is_potentially_non_numeric(right)
                    {
                        violations.push(Violation::full(
                            ViolationKind::DataCorruption,
                            span.clone().into(),
                            "Bitwise operation on potentially non-numeric type".to_string(),
                            "Deception: Corrupting structured data",
                            "Entry to Puyoda",
                            "Ensure operands are numeric types".to_string(),
                        ));
                    }
                }
                self.check_expr(left, violations);
                self.check_expr(right, violations);
            }
            _ => {}
        }
    }

    fn is_union_access(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Identifier(id) => id.name.contains("union") || id.name.contains("Union"),
            _ => false,
        }
    }

    fn is_potentially_non_numeric(&self, expr: &Expr) -> bool {
        match expr {
            Expr::FieldAccess { .. } => true, // Struct fields might not be numeric
            Expr::MethodCall { .. } => true,  // Method returns might not be numeric
            _ => false,
        }
    }
}

impl Default for PuyodaChecker {
    fn default() -> Self {
        Self::new()
    }
}
