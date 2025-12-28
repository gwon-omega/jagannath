//! # Andhatamisram - Hell 2: API Contract Violation
//!
//! Sin: Betraying spouse/partner
//! Code: Breaking API contracts, unimplemented functions

use super::super::yama::{Violation, ViolationKind};
use crate::parser::ast::{Ast, Expr, Item, Stmt, Block};
use std::collections::HashSet;

/// Checker for Andhatamisram violations (API contract betrayal)
pub struct AndhatamisramChecker {
    /// Panic functions (contract violations)
    panic_fns: HashSet<&'static str>,
}

impl AndhatamisramChecker {
    pub fn new() -> Self {
        Self {
            panic_fns: ["unimplemented", "todo", "unreachable", "panic"].into(),
        }
    }

    /// Check for API contract violations
    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();

        // Check functions for stub implementations
        for item in &ast.items {
            if let Item::Function(func) = item {
                // Check if entire function is just unimplemented!/todo!
                self.check_stub_function(&func.body, &func.name.name, &mut violations);
                // Check for contract violations in body
                self.check_block(&func.body, &func.name.name, &mut violations);
            }
        }

        violations
    }

    fn check_stub_function(&self, body: &Block, fn_name: &str, violations: &mut Vec<Violation>) {
        // Check if entire function is just a stub
        if body.stmts.len() == 1 {
            if let Stmt::Expr(Expr::Call { callee, span, .. }) = &body.stmts[0] {
                if let Expr::Identifier(id) = callee.as_ref() {
                    if id.name == "unimplemented" || id.name == "todo" {
                        violations.push(Violation::full(
                            ViolationKind::ContractViolation, span.clone().into(),
                            format!("Function '{}' is entirely unimplemented", fn_name),
                            "Betrayal: Empty promise to callers",
                            "Entry to Andhatamisram (blinding darkness hell)",
                            "Implement the function body".to_string(),
                        ));
                    }
                }
            }
        }
    }

    fn check_block(&self, block: &Block, fn_name: &str, violations: &mut Vec<Violation>) {
        for stmt in &block.stmts {
            self.check_stmt(stmt, fn_name, violations);
        }
    }

    fn check_stmt(&self, stmt: &Stmt, fn_name: &str, violations: &mut Vec<Violation>) {
        match stmt {
            Stmt::Expr(e) => self.check_expr(e, fn_name, violations),
            Stmt::If { then_block, else_block, .. } => {
                self.check_block(then_block, fn_name, violations);
                if let Some(eb) = else_block { self.check_block(eb, fn_name, violations); }
            }
            Stmt::Loop { body, .. } => {
                self.check_block(body, fn_name, violations);
            }
            _ => {}
        }
    }

    fn check_expr(&self, expr: &Expr, fn_name: &str, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Call { callee, span, .. } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    if self.panic_fns.contains(id.name.as_str()) {
                        violations.push(Violation::full(
                            ViolationKind::ContractViolation, span.clone().into(),
                            format!("Function '{}' contains {}! - contract violation", fn_name, id.name),
                            "Betrayal: Breaking promise to callers",
                            "Entry to Andhatamisram",
                            "Provide actual implementation or proper error handling".to_string(),
                        ));
                    }
                }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left, fn_name, violations);
                self.check_expr(right, fn_name, violations);
            }
            _ => {}
        }
    }
}

impl Default for AndhatamisramChecker {
    fn default() -> Self { Self::new() }
}
