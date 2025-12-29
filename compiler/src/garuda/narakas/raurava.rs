//! # Raurava - Hell 3: Panic/Crash Violence
//!
//! Sin: Violence causing suffering
//! Code: Panics, crashes, unhandled errors

use super::super::yama::{Violation, ViolationKind};
use crate::parser::ast::{Ast, Block, Expr, Item, LoopKind, Stmt};
use std::collections::HashSet;

/// Checker for Raurava violations (crash violence)
pub struct RauravaChecker {
    panic_functions: HashSet<String>,
    result_vars: HashSet<String>, // Variables that need error handling
}

impl RauravaChecker {
    pub fn new() -> Self {
        let mut panic_functions = HashSet::new();
        // Functions that panic/crash
        panic_functions.insert("panic".to_string());
        panic_functions.insert("unwrap".to_string());
        panic_functions.insert("expect".to_string());
        panic_functions.insert("abort".to_string());
        panic_functions.insert("exit".to_string());
        panic_functions.insert("unreachable".to_string());
        panic_functions.insert("assert".to_string());
        panic_functions.insert("todo".to_string());
        panic_functions.insert("unimplemented".to_string());

        Self {
            panic_functions,
            result_vars: HashSet::new(),
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.result_vars.clear();
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
                if let Some(val) = value {
                    // Check if assigning a Result/Option type
                    if self.may_return_result(val) {
                        self.result_vars.insert(name.name.clone());
                    }
                    self.check_expr(val, violations);
                }
            }
            Stmt::Expr(expr) => self.check_expr(expr, violations),
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
            Expr::Call { callee, args, span } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    // Direct panic call
                    if self.panic_functions.contains(&id.name) {
                        violations.push(Violation::full(
                            ViolationKind::Panic,
                            span.clone().into(),
                            format!("Call to panic function '{}'", id.name),
                            "Violence through crashing: Panic causes suffering",
                            "Entry to Raurava (screaming hell)",
                            "Use proper error handling instead of panic".to_string(),
                        ));
                    }
                }
                for arg in args {
                    self.check_expr(arg, violations);
                }
            }
            Expr::MethodCall {
                method,
                receiver,
                args,
                span,
            } => {
                // Check for .unwrap(), .expect() on Result/Option
                if matches!(method.name.as_str(), "unwrap" | "expect") {
                    if let Expr::Identifier(id) = receiver.as_ref() {
                        if self.result_vars.contains(&id.name) {
                            violations.push(Violation::full(
                                ViolationKind::Panic,
                                span.clone().into(),
                                format!("Calling .{}() on Result/Option", method.name),
                                "Violence by assumption: Unwrap may panic",
                                "Raurava warning",
                                "Use match, if-let, or '?' operator for error handling".to_string(),
                            ));
                        }
                    }
                }
                self.check_expr(receiver, violations);
                for arg in args {
                    self.check_expr(arg, violations);
                }
            }
            Expr::Binary {
                left,
                right,
                op,
                span,
            } => {
                // Check for division by zero potential
                if let crate::parser::ast::BinaryOp::Div | crate::parser::ast::BinaryOp::Mod = op {
                    if let Expr::Literal(crate::parser::ast::Literal::Int(0)) = right.as_ref() {
                        violations.push(Violation::full(
                            ViolationKind::Panic,
                            span.clone().into(),
                            "Division by zero".to_string(),
                            "Mathematical violence: Division by zero crashes",
                            "Entry to Raurava",
                            "Check divisor is non-zero before dividing".to_string(),
                        ));
                    }
                }
                self.check_expr(left, violations);
                self.check_expr(right, violations);
            }
            Expr::Index {
                object,
                index,
                span: _span,
            } => {
                // Unchecked array access may panic
                self.check_expr(object, violations);
                self.check_expr(index, violations);
            }
            _ => {}
        }
    }

    fn may_return_result(&self, expr: &Expr) -> bool {
        // Heuristic: functions with certain names return Result/Option
        if let Expr::Call { callee, .. } = expr {
            if let Expr::Identifier(id) = callee.as_ref() {
                let name_lower = id.name.to_lowercase();
                return name_lower.contains("try")
                    || name_lower.contains("parse")
                    || name_lower.contains("read")
                    || name_lower.contains("open")
                    || name_lower.contains("find")
                    || name_lower.contains("get");
            }
        }
        false
    }
}

impl Default for RauravaChecker {
    fn default() -> Self {
        Self::new()
    }
}
