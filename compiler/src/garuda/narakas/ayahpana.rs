//! # Ayahpana - Hell 21: Poisoned Data
//!
//! Sin: Consuming alcohol (poisoned/tainted data)
//! Code: Unsafe deserialization, unsanitized input

use crate::parser::ast::{Ast, Expr, Item, Stmt, Block, LoopKind};
use super::super::yama::{Violation, ViolationKind};
use std::collections::HashSet;

/// Checker for Ayahpana violations (unsafe deserialization/tainted data)
pub struct AyahpanaChecker {
    tainted: HashSet<String>,
    dangerous_funcs: HashSet<&'static str>,
}

impl AyahpanaChecker {
    pub fn new() -> Self {
        let mut dangerous_funcs = HashSet::new();
        for f in ["pickle_load", "yaml_load", "eval", "exec", "deserialize", "unmarshal"] {
            dangerous_funcs.insert(f);
        }
        Self { tainted: HashSet::new(), dangerous_funcs }
    }

    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.tainted.clear();
                // Mark input params as tainted
                for p in &func.params {
                    let pn = p.name.name.to_lowercase();
                    if pn.contains("input") || pn.contains("data") || pn.contains("request") {
                        self.tainted.insert(p.name.name.clone());
                    }
                }
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
            Stmt::Expr(expr) => self.check_expr(expr, violations),
            Stmt::Let { name, value: Some(v), .. } => {
                if self.is_tainted(v) {
                    self.tainted.insert(name.name.clone());
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
            Stmt::Return { value: Some(v), span } => {
                if self.is_tainted(v) {
                    violations.push(Violation::full(
                        ViolationKind::PoisonedData, span.clone().into(),
                        "Returning unsanitized tainted data".to_string(),
                        "Drinking poison: Spreading tainted data", "Ayahpana warning",
                        "Sanitize before returning".to_string(),
                    ));
                }
            }
            _ => {}
        }
    }

    fn check_expr(&self, expr: &Expr, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Call { callee, args, span } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    // Check dangerous deserializers
                    if self.dangerous_funcs.contains(id.name.as_str()) {
                        if args.iter().any(|a| self.is_tainted(a)) {
                            violations.push(Violation::full(
                                ViolationKind::PoisonedData, span.clone().into(),
                                format!("Deserializing tainted data with '{}'", id.name),
                                "Drinking poison: Deserializing external data",
                                "Entry to Ayahpana: Security vulnerability",
                                "Validate before deserialization".to_string(),
                            ));
                        }
                    }
                    // Check SQL/command injection
                    let fn_lower = id.name.to_lowercase();
                    if (fn_lower.contains("query") || fn_lower.contains("exec") || fn_lower.contains("system"))
                        && args.iter().any(|a| self.is_tainted(a)) {
                        violations.push(Violation::full(
                            ViolationKind::PoisonedData, span.clone().into(),
                            "Tainted data in dangerous sink".to_string(),
                            "Drinking poison: Injection vulnerability",
                            "Ayahpana critical", "Use parameterized queries".to_string(),
                        ));
                    }
                }
                for arg in args { self.check_expr(arg, violations); }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left, violations);
                self.check_expr(right, violations);
            }
            _ => {}
        }
    }

    fn is_tainted(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Identifier(id) => self.tainted.contains(&id.name),
            Expr::Binary { left, right, .. } => self.is_tainted(left) || self.is_tainted(right),
            Expr::Call { args, .. } => args.iter().any(|a| self.is_tainted(a)),
            _ => false,
        }
    }
}

impl Default for AyahpanaChecker {
    fn default() -> Self { Self::new() }
}
