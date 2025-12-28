//! # Kumbhipaka - Hell 5: Resource Exhaustion
//!
//! Sin: Cooking/boiling sins (excessive consumption)
//! Code: CPU/memory burning, resource exhaustion, unbounded loops

use super::super::yama::{Violation, ViolationKind};
use crate::errors::Span;
use crate::parser::ast::{Ast, Block, Expr, Item, Literal, LoopKind, MatchArm, Stmt};
use std::collections::{HashMap, HashSet};

/// Checker for Kumbhipaka violations (resource exhaustion)
pub struct KumbhipakaChecker {
    pub max_iterations: usize,
    pub max_recursion: usize,
    call_graph: HashMap<String, HashSet<String>>,
    loop_depth: usize,
    current_function: Option<String>,
}

impl KumbhipakaChecker {
    pub fn new() -> Self {
        Self {
            max_iterations: 1_000_000,
            max_recursion: 1000,
            call_graph: HashMap::new(),
            loop_depth: 0,
            current_function: None,
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        // Build call graph
        self.build_call_graph(ast);
        // Analyze functions
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.current_function = Some(func.name.name.clone());
                self.loop_depth = 0;
                self.check_block(&func.body, &mut violations);
                self.current_function = None;
            }
        }
        // Detect recursion
        self.detect_recursion(&mut violations);
        violations
    }

    fn build_call_graph(&mut self, ast: &Ast) {
        for item in &ast.items {
            if let Item::Function(func) = item {
                let mut callees = HashSet::new();
                self.collect_calls(&func.body, &mut callees);
                self.call_graph.insert(func.name.name.clone(), callees);
            }
        }
    }

    fn collect_calls(&self, block: &Block, callees: &mut HashSet<String>) {
        for stmt in &block.stmts {
            match stmt {
                Stmt::Expr(e) => self.collect_calls_expr(e, callees),
                Stmt::Let { value: Some(v), .. } => self.collect_calls_expr(v, callees),
                Stmt::If {
                    condition,
                    then_block,
                    else_block,
                    ..
                } => {
                    self.collect_calls_expr(condition, callees);
                    self.collect_calls(then_block, callees);
                    if let Some(eb) = else_block {
                        self.collect_calls(eb, callees);
                    }
                }
                Stmt::Loop { body, kind, .. } => {
                    if let LoopKind::While { condition } = kind {
                        self.collect_calls_expr(condition, callees);
                    }
                    self.collect_calls(body, callees);
                }
                Stmt::Return { value: Some(v), .. } => self.collect_calls_expr(v, callees),
                Stmt::Match { arms, .. } => {
                    for arm in arms {
                        self.collect_calls_expr(&arm.body, callees);
                    }
                }
                _ => {}
            }
        }
    }

    fn collect_calls_expr(&self, expr: &Expr, callees: &mut HashSet<String>) {
        match expr {
            Expr::Call { callee, args, .. } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    callees.insert(id.name.clone());
                }
                for a in args {
                    self.collect_calls_expr(a, callees);
                }
            }
            Expr::Binary { left, right, .. } => {
                self.collect_calls_expr(left, callees);
                self.collect_calls_expr(right, callees);
            }
            _ => {}
        }
    }

    fn check_block(&mut self, block: &Block, violations: &mut Vec<Violation>) {
        for stmt in &block.stmts {
            self.check_stmt(stmt, violations);
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt, violations: &mut Vec<Violation>) {
        match stmt {
            Stmt::Loop { body, span, kind } => {
                self.loop_depth += 1;
                // Check for unbounded loop
                if matches!(kind, LoopKind::Infinite) && !self.has_exit(body) {
                    violations.push(Violation::full(
                        ViolationKind::ResourceExhaustion,
                        span.clone().into(),
                        "Unbounded loop without exit".to_string(),
                        "Boiling forever: Infinite loop",
                        "Entry to Kumbhipaka",
                        "Add break condition".to_string(),
                    ));
                }
                // Check deep nesting
                if self.loop_depth > 3 {
                    violations.push(Violation::full(
                        ViolationKind::ResourceExhaustion,
                        span.clone().into(),
                        format!("Deeply nested loops ({} levels)", self.loop_depth),
                        "Boiling intensely: O(n^k) complexity",
                        "Kumbhipaka warning",
                        "Optimize algorithm".to_string(),
                    ));
                }
                // Check while(true)
                if let LoopKind::While { condition } = kind {
                    if self.is_always_true(condition) && !self.has_exit(body) {
                        violations.push(Violation::full(
                            ViolationKind::ResourceExhaustion,
                            span.clone().into(),
                            "while(true) without break".to_string(),
                            "Boiling forever: Eternal loop",
                            "Entry to Kumbhipaka",
                            "Add break statement".to_string(),
                        ));
                    }
                }
                self.check_block(body, violations);
                self.loop_depth -= 1;
            }
            Stmt::If {
                then_block,
                else_block,
                ..
            } => {
                self.check_block(then_block, violations);
                if let Some(eb) = else_block {
                    self.check_block(eb, violations);
                }
            }
            Stmt::Expr(e) => self.check_expr(e, violations),
            Stmt::Let { value: Some(v), .. } => self.check_expr(v, violations),
            _ => {}
        }
    }

    fn check_expr(&self, expr: &Expr, violations: &mut Vec<Violation>) {
        if let Expr::Call { callee, span, .. } = expr {
            if let Expr::Identifier(id) = callee.as_ref() {
                let fn_lower = id.name.to_lowercase();
                if (fn_lower == "fork" || fn_lower == "spawn") && self.loop_depth > 0 {
                    violations.push(Violation::full(
                        ViolationKind::ResourceExhaustion,
                        span.clone().into(),
                        "Process spawn inside loop (fork bomb)".to_string(),
                        "Boiling processes: Exponential creation",
                        "Entry to Kumbhipaka",
                        "Move spawn outside loop".to_string(),
                    ));
                }
            }
        }
    }

    fn has_exit(&self, block: &Block) -> bool {
        for stmt in &block.stmts {
            match stmt {
                Stmt::Break { .. } | Stmt::Return { .. } => return true,
                Stmt::If {
                    then_block,
                    else_block,
                    ..
                } => {
                    let then_exit = self.has_exit(then_block);
                    let else_exit = else_block
                        .as_ref()
                        .map(|b| self.has_exit(b))
                        .unwrap_or(false);
                    if then_exit && else_exit {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn is_always_true(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Literal(Literal::Bool(true)) => true,
            Expr::Literal(Literal::Int(n)) => *n != 0,
            _ => false,
        }
    }

    fn detect_recursion(&self, violations: &mut Vec<Violation>) {
        // Direct recursion
        for (func, callees) in &self.call_graph {
            if callees.contains(func) {
                violations.push(Violation::full(
                    ViolationKind::ResourceExhaustion,
                    Span::dummy().into(),
                    format!("Direct recursion in '{}'", func),
                    "Boiling stack: Recursion detected",
                    "Kumbhipaka warning",
                    "Ensure base case exists".to_string(),
                ));
            }
        }
        // Mutual recursion (simple check)
        for (a, a_calls) in &self.call_graph {
            for b in a_calls {
                if let Some(b_calls) = self.call_graph.get(b) {
                    if b_calls.contains(a) && a != b {
                        violations.push(Violation::full(
                            ViolationKind::ResourceExhaustion,
                            Span::dummy().into(),
                            format!("Mutual recursion: {} <-> {}", a, b),
                            "Boiling together: Circular calls",
                            "Kumbhipaka warning",
                            "Break recursion cycle".to_string(),
                        ));
                    }
                }
            }
        }
    }
}

impl Default for KumbhipakaChecker {
    fn default() -> Self {
        Self::new()
    }
}
