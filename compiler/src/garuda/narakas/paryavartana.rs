//! # Paryavartana - Hell 27: Resource Denial
//!
//! Sin: Denying food to hungry
//! Code: Refusing allocation, resource hoarding

use super::super::yama::{Violation, ViolationKind};
use crate::errors::Span;
use crate::parser::ast::{Ast, BinaryOp, Block, Expr, Item, Literal, LoopKind, Stmt};
use std::collections::{HashMap, HashSet};

/// Resource tracking
#[derive(Clone, Debug)]
struct ResourcePool {
    total: Option<usize>,
    held_count: usize,
}

/// Checker for Paryavartana violations (resource denial)
pub struct ParyavartanaChecker {
    /// Track limited resources
    resource_pools: HashMap<String, ResourcePool>,
    /// Resource limiting functions
    limit_fns: HashSet<&'static str>,
}

impl ParyavartanaChecker {
    pub fn new() -> Self {
        Self {
            resource_pools: HashMap::new(),
            limit_fns: ["set_limit", "quota", "max_allowed", "cap"].into(),
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.resource_pools.clear();
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
            Stmt::Expr(e) => self.check_expr(e, violations),
            Stmt::Let {
                name,
                value: Some(v),
                span,
                ..
            } => {
                // Check for acquiring limited resources
                if self.is_limited_resource(v) {
                    let pool = self.get_resource_type(v);
                    if let Some(rp) = self.resource_pools.get_mut(&pool) {
                        if let Some(total) = rp.total {
                            if rp.held_count >= total {
                                violations.push(Violation::full(
                                    ViolationKind::ResourceDenial,
                                    span.clone().into(),
                                    format!("Acquiring '{}' when pool exhausted", name.name),
                                    "Denying food: Hoarding all resources",
                                    "Entry to Paryavartana (reversal hell)",
                                    "Release resources before acquiring more".to_string(),
                                ));
                            }
                            rp.held_count += 1;
                        }
                    }
                }
                self.check_expr(v, violations);
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
                span,
            } => {
                // Check for conditional resource denial
                if self.is_denial_pattern(condition, then_block) {
                    violations.push(Violation::full(
                        ViolationKind::ResourceDenial,
                        span.clone().into(),
                        "Conditional resource denial pattern".to_string(),
                        "Denying food: Refusing resources to requestors",
                        "Entry to Paryavartana",
                        "Provide alternative or queue mechanism".to_string(),
                    ));
                }
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

    fn check_expr(&mut self, expr: &Expr, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Call { callee, args, span } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    // Detect resource limiting
                    if self.limit_fns.contains(id.name.as_str()) {
                        if let Some(Expr::Literal(Literal::Int(limit))) = args.get(1) {
                            if *limit == 0 {
                                violations.push(Violation::full(
                                    ViolationKind::ResourceDenial,
                                    span.clone().into(),
                                    "Setting resource limit to zero".to_string(),
                                    "Denying food: Refusing all allocations",
                                    "Entry to Paryavartana",
                                    "Set reasonable non-zero limit".to_string(),
                                ));
                            }
                        }
                    }
                }
                for a in args {
                    self.check_expr(a, violations);
                }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left, violations);
                self.check_expr(right, violations);
            }
            _ => {}
        }
    }

    fn is_limited_resource(&self, _expr: &Expr) -> bool {
        // Would need type info to determine
        false
    }

    fn get_resource_type(&self, _expr: &Expr) -> String {
        "default_pool".to_string()
    }

    fn is_denial_pattern(&self, condition: &Expr, then_block: &Block) -> bool {
        // Check for pattern: if (no_resources) { return error; }
        if then_block.stmts.len() == 1 {
            if let Stmt::Return {
                value: Some(Expr::Identifier(id)),
                ..
            } = &then_block.stmts[0]
            {
                if id.name.contains("err") || id.name.contains("deny") {
                    return self.is_resource_check(condition);
                }
            }
        }
        false
    }

    fn is_resource_check(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Identifier(id) => {
                id.name.contains("avail") || id.name.contains("quota") || id.name.contains("limit")
            }
            Expr::Binary { op, .. }
                if *op == BinaryOp::Eq || *op == BinaryOp::Lt || *op == BinaryOp::Le =>
            {
                true
            }
            _ => false,
        }
    }
}

impl Default for ParyavartanaChecker {
    fn default() -> Self {
        Self::new()
    }
}
