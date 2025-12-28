//! # Dandasuka - Hell 25: Starvation
//!
//! Sin: Imprisoning/starving people
//! Code: Resource starvation, unfair scheduling

use crate::parser::ast::{Ast, Expr, Item, Stmt, Block, LoopKind};
use crate::errors::Span;
use super::super::yama::{Violation, ViolationKind};
use std::collections::{HashMap, HashSet};

/// Resource usage tracking
#[derive(Clone, Debug)]
struct ResourceUsage {
    acquired: bool,
    released: bool,
    #[allow(dead_code)]
    in_loop: bool,
}

/// Checker for Dandasuka violations (starvation)
pub struct DandasukaChecker {
    /// Track resource acquisition/release
    resources: HashMap<String, ResourceUsage>,
    /// Resource acquisition functions
    acquire_fns: HashSet<&'static str>,
    /// Resource release functions
    release_fns: HashSet<&'static str>,
    /// Priority modification functions
    priority_fns: HashSet<&'static str>,
    /// Currently in loop
    in_loop: bool,
}

impl DandasukaChecker {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            acquire_fns: ["acquire", "lock", "take", "get_exclusive", "claim", "reserve"].into(),
            release_fns: ["release", "unlock", "give", "free", "yield_resource"].into(),
            priority_fns: ["set_priority", "boost_priority", "lower_priority", "nice"].into(),
            in_loop: false,
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.resources.clear();
                self.in_loop = false;
                self.check_block(&func.body, &mut violations);

                // Check for resources held indefinitely
                for (name, usage) in &self.resources {
                    if usage.acquired && !usage.released {
                        violations.push(Violation::full(
                            ViolationKind::Starvation, Span::dummy().into(),
                            format!("Resource '{}' acquired but never released", name),
                            "Imprisonment: Starving others of resources",
                            "Entry to Dandasuka (crocodile hell)",
                            format!("Release '{}' after use", name),
                        ));
                    }
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
            Stmt::Let { name, value: Some(v), .. } => {
                if self.is_resource_acquisition(v) {
                    self.resources.insert(name.name.clone(), ResourceUsage {
                        acquired: true,
                        released: false,
                        in_loop: self.in_loop,
                    });
                }
                self.check_expr(v, violations);
            }
            Stmt::If { condition, then_block, else_block, .. } => {
                self.check_expr(condition, violations);
                self.check_block(then_block, violations);
                if let Some(eb) = else_block { self.check_block(eb, violations); }
            }
            Stmt::Loop { body, kind, span } => {
                let was_in_loop = self.in_loop;
                self.in_loop = true;

                // Check for busy waiting
                if self.is_busy_wait(body) {
                    violations.push(Violation::full(
                        ViolationKind::Starvation, span.clone().into(),
                        "Busy waiting detected (spin lock without yield)".to_string(),
                        "Imprisonment: CPU starvation through spinning",
                        "Entry to Dandasuka",
                        "Add yield() or use blocking wait".to_string(),
                    ));
                }

                // Check for resource acquisition in infinite loop
                if matches!(kind, LoopKind::Infinite) {
                    self.check_loop_starvation(body, span, violations);
                }

                if let LoopKind::While { condition } = kind {
                    self.check_expr(condition, violations);
                }
                self.check_block(body, violations);
                self.in_loop = was_in_loop;
            }
            _ => {}
        }
    }

    fn check_expr(&mut self, expr: &Expr, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Call { callee, args, span } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    // Check for resource release
                    if self.release_fns.contains(id.name.as_str()) {
                        if let Some(Expr::Identifier(res_id)) = args.first() {
                            if let Some(usage) = self.resources.get_mut(&res_id.name) {
                                usage.released = true;
                            }
                        }
                    }
                    // Check for priority manipulation
                    if self.priority_fns.contains(id.name.as_str()) {
                        violations.push(Violation::full(
                            ViolationKind::Starvation, span.clone().into(),
                            format!("Priority manipulation via '{}'", id.name),
                            "Imprisonment: Potentially starving lower-priority threads",
                            "Entry to Dandasuka",
                            "Ensure fair scheduling with priority inheritance".to_string(),
                        ));
                    }
                }
                for a in args { self.check_expr(a, violations); }
            }
            Expr::MethodCall { receiver, method, args, .. } => {
                if self.release_fns.contains(method.name.as_str()) {
                    if let Expr::Identifier(id) = receiver.as_ref() {
                        if let Some(usage) = self.resources.get_mut(&id.name) {
                            usage.released = true;
                        }
                    }
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

    fn is_resource_acquisition(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Call { callee, .. } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    self.acquire_fns.contains(id.name.as_str())
                } else { false }
            }
            Expr::MethodCall { method, .. } => self.acquire_fns.contains(method.name.as_str()),
            _ => false,
        }
    }

    fn is_busy_wait(&self, body: &Block) -> bool {
        // Check for empty loop or loop with only check condition
        if body.stmts.is_empty() { return true; }
        if body.stmts.len() == 1 {
            if let Stmt::If { then_block, else_block: None, .. } = &body.stmts[0] {
                // Just a condition check, no yielding
                return !self.contains_yield(then_block);
            }
        }
        false
    }

    fn contains_yield(&self, block: &Block) -> bool {
        for stmt in &block.stmts {
            if let Stmt::Expr(Expr::Call { callee, .. }) = stmt {
                if let Expr::Identifier(id) = callee.as_ref() {
                    if ["yield", "sleep", "wait", "park"].contains(&id.name.as_str()) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn check_loop_starvation(&self, body: &Block, span: &crate::lexer::token::Span, violations: &mut Vec<Violation>) {
        // Check for resource acquisition without release inside infinite loop
        for stmt in &body.stmts {
            if let Stmt::Let { name, value: Some(v), .. } = stmt {
                if self.is_resource_acquisition(v) {
                    // Check if there's a matching release
                    if !self.has_release_in_block(body, &name.name) {
                        violations.push(Violation::full(
                            ViolationKind::Starvation, span.clone().into(),
                            format!("Resource '{}' acquired in loop without release", name.name),
                            "Imprisonment: Accumulating resources without release",
                            "Entry to Dandasuka",
                            "Release resource before loop iteration ends".to_string(),
                        ));
                    }
                }
            }
        }
    }

    fn has_release_in_block(&self, block: &Block, resource_name: &str) -> bool {
        for stmt in &block.stmts {
            if let Stmt::Expr(Expr::Call { callee, args, .. }) = stmt {
                if let Expr::Identifier(id) = callee.as_ref() {
                    if self.release_fns.contains(id.name.as_str()) {
                        if let Some(Expr::Identifier(arg_id)) = args.first() {
                            if arg_id.name == resource_name {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }
}

impl Default for DandasukaChecker {
    fn default() -> Self { Self::new() }
}
