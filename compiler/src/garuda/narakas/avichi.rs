//! # Avichi - Hell 20: Stack Overflow
//!
//! Sin: False witness, perjury
//! Code: Stack overflow from unbounded recursion

use super::super::yama::{Violation, ViolationKind};
use crate::parser::ast::{Ast, Expr, Item, Stmt, Block, LoopKind};
use crate::errors::Span;
use std::collections::{HashMap, HashSet};

/// Checker for Avichi violations (stack overflow)
pub struct AvichiChecker {
    pub max_recursion_depth: usize,
    call_graph: HashMap<String, HashSet<String>>,
    current_function: Option<String>,
}

impl AvichiChecker {
    pub fn new() -> Self {
        Self {
            max_recursion_depth: 1000,
            call_graph: HashMap::new(),
            current_function: None,
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();

        // Build call graph
        self.build_call_graph(ast);

        // Check for direct recursion without base case
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.current_function = Some(func.name.name.clone());
                self.check_function_recursion(&func.name.name, &func.body, &mut violations);
            }
        }

        // Check for mutual recursion cycles
        self.detect_recursion_cycles(&mut violations);

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
                Stmt::If { condition, then_block, else_block, .. } => {
                    self.collect_calls_expr(condition, callees);
                    self.collect_calls(then_block, callees);
                    if let Some(eb) = else_block { self.collect_calls(eb, callees); }
                }
                Stmt::Loop { body, kind, .. } => {
                    if let LoopKind::While { condition } = kind {
                        self.collect_calls_expr(condition, callees);
                    }
                    self.collect_calls(body, callees);
                }
                Stmt::Return { value: Some(v), .. } => self.collect_calls_expr(v, callees),
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
                for a in args { self.collect_calls_expr(a, callees); }
            }
            Expr::Binary { left, right, .. } => {
                self.collect_calls_expr(left, callees);
                self.collect_calls_expr(right, callees);
            }
            Expr::Unary { operand, .. } => self.collect_calls_expr(operand, callees),
            _ => {}
        }
    }

    fn check_function_recursion(&self, fn_name: &str, body: &Block, violations: &mut Vec<Violation>) {
        // Check if function calls itself
        if let Some(callees) = self.call_graph.get(fn_name) {
            if callees.contains(fn_name) {
                // Direct recursion - check for base case
                if !self.has_base_case(body) {
                    violations.push(Violation::full(
                        ViolationKind::StackOverflow, Span::dummy().into(),
                        format!("Recursive function '{}' without clear base case", fn_name),
                        "False witness: Claiming termination without proof",
                        "Entry to Avichi (waveless hell)",
                        "Add explicit base case with early return".to_string(),
                    ));
                }
            }
        }
    }

    fn has_base_case(&self, block: &Block) -> bool {
        // Check for early return in conditional (typical base case pattern)
        for stmt in &block.stmts {
            match stmt {
                Stmt::If { then_block, .. } => {
                    // Check if then-block has a return
                    for s in &then_block.stmts {
                        if matches!(s, Stmt::Return { .. }) {
                            return true;
                        }
                    }
                }
                Stmt::Match { arms, .. } => {
                    // Check if any arm returns without recursion
                    for arm in arms {
                        if self.is_non_recursive_return(&arm.body) {
                            return true;
                        }
                    }
                }
                Stmt::Return { value: Some(val), .. } => {
                    // Check if return is at start (before any calls)
                    if !self.expr_contains_call(val) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn is_non_recursive_return(&self, expr: &Expr) -> bool {
        !self.expr_contains_call(expr)
    }

    fn expr_contains_call(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Call { .. } => true,
            Expr::Binary { left, right, .. } => {
                self.expr_contains_call(left) || self.expr_contains_call(right)
            }
            Expr::Unary { operand, .. } => self.expr_contains_call(operand),
            _ => false,
        }
    }

    fn detect_recursion_cycles(&self, violations: &mut Vec<Violation>) {
        // Simple cycle detection for mutual recursion
        for (func_a, callees_a) in &self.call_graph {
            for func_b in callees_a {
                if func_a == func_b { continue; }  // Already handled direct recursion

                if let Some(callees_b) = self.call_graph.get(func_b) {
                    if callees_b.contains(func_a) {
                        // Mutual recursion: A calls B, B calls A
                        violations.push(Violation::full(
                            ViolationKind::StackOverflow, Span::dummy().into(),
                            format!("Mutual recursion between '{}' and '{}'", func_a, func_b),
                            "Conspiracy of lies: Functions calling each other infinitely",
                            "Entry to Avichi",
                            "Break the cycle with iterative approach or trampoline".to_string(),
                        ));
                    }
                }
            }
        }
    }
}

impl Default for AvichiChecker {
    fn default() -> Self { Self::new() }
}
