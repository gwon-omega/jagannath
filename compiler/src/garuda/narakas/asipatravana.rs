//! # Asipatravana - Hell 7: Buffer Overflow
//!
//! Sin: Abandoning dharma (crossing boundaries)
//! Code: Buffer overflow, out-of-bounds access

use super::super::yama::{Violation, ViolationKind};
use crate::parser::ast::{Ast, Block, Expr, Item, Literal, LoopKind, Stmt};
use std::collections::HashMap;

/// Checker for Asipatravana violations (buffer overflow)
pub struct AsipatravanaChecker {
    array_sizes: HashMap<String, Option<usize>>,
}

impl AsipatravanaChecker {
    pub fn new() -> Self {
        Self {
            array_sizes: HashMap::new(),
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.array_sizes.clear();
                self.collect_arrays(&func.body);
                self.check_block(&func.body, &mut violations);
            }
        }
        violations
    }

    fn collect_arrays(&mut self, block: &Block) {
        for stmt in &block.stmts {
            if let Stmt::Let { name, value, .. } = stmt {
                if let Some(Expr::Array { elements, .. }) = value {
                    self.array_sizes
                        .insert(name.name.clone(), Some(elements.len()));
                }
            }
        }
    }

    fn check_block(&self, block: &Block, violations: &mut Vec<Violation>) {
        for stmt in &block.stmts {
            self.check_stmt(stmt, violations);
        }
    }

    fn check_stmt(&self, stmt: &Stmt, violations: &mut Vec<Violation>) {
        match stmt {
            Stmt::Expr(expr) => self.check_expr(expr, violations),
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
            Stmt::Return { value: Some(v), .. } => self.check_expr(v, violations),
            _ => {}
        }
    }

    fn check_expr(&self, expr: &Expr, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Index {
                object,
                index,
                span,
            } => {
                if let (Expr::Identifier(id), Expr::Literal(Literal::Int(idx))) =
                    (object.as_ref(), index.as_ref())
                {
                    if let Some(Some(size)) = self.array_sizes.get(&id.name) {
                        if *idx < 0 || (*idx as usize) >= *size {
                            violations.push(Violation::full(
                                ViolationKind::BufferOverflow,
                                span.clone().into(),
                                format!(
                                    "Index {} out of bounds for array '{}' (size {})",
                                    idx, id.name, size
                                ),
                                "Abandoning dharma: Crossing sacred boundaries",
                                "Entry to Asipatravana blocked",
                                "Ensure index < array size".to_string(),
                            ));
                        }
                    }
                }
                self.check_expr(object, violations);
                self.check_expr(index, violations);
            }
            Expr::Binary {
                left,
                right,
                op: _op,
                span: _span,
            } => {
                self.check_expr(left, violations);
                self.check_expr(right, violations);
            }
            Expr::Call { callee, args, span } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    if matches!(id.name.as_str(), "strcpy" | "strcat" | "gets" | "sprintf") {
                        violations.push(Violation::full(
                            ViolationKind::BufferOverflow,
                            span.clone().into(),
                            format!("Unsafe function '{}' used", id.name),
                            "Abandoning dharma: Using boundary-ignoring functions",
                            "Asipatravana warning",
                            "Use safe alternatives with bounds checking".to_string(),
                        ));
                    }
                }
                for arg in args {
                    self.check_expr(arg, violations);
                }
            }
            Expr::Array { elements, .. } => {
                for e in elements {
                    self.check_expr(e, violations);
                }
            }
            _ => {}
        }
    }
}

impl Default for AsipatravanaChecker {
    fn default() -> Self {
        Self::new()
    }
}
