//! # Suchimukha - Hell 23: Needle-Face
//!
//! Sin: Miserliness (hoarding)
//! Code: Excessive caching without eviction / memory leaks

use super::super::yama::{Violation, ViolationKind};
use crate::errors::Span;
use crate::parser::ast::{Ast, Block, Expr, Item, LoopKind, Stmt};
use std::collections::{HashMap, HashSet};

/// Track allocation state
#[derive(Clone, Debug, PartialEq)]
enum AllocState {
    Allocated,
    AddedToCollection,
    Freed,
}

/// Checker for Suchimukha violations (memory hoarding)
pub struct SuchimukhaChecker {
    /// Track allocations never freed
    allocations: HashMap<String, AllocState>,
    /// Collection types that grow indefinitely
    collection_types: HashSet<&'static str>,
    /// Allocation functions
    alloc_fns: HashSet<&'static str>,
    /// Deallocation functions
    free_fns: HashSet<&'static str>,
    /// Functions that add to collections
    add_fns: HashSet<&'static str>,
}

impl SuchimukhaChecker {
    pub fn new() -> Self {
        Self {
            allocations: HashMap::new(),
            collection_types: ["Vec", "HashMap", "HashSet", "BTreeMap", "Cache", "LruCache"].into(),
            alloc_fns: ["malloc", "calloc", "alloc", "new", "nirmā", "Box::new"].into(),
            free_fns: ["free", "dealloc", "mukta", "drop", "release"].into(),
            add_fns: ["push", "insert", "add", "put", "append", "extend"].into(),
        }
    }

    /// Check for excessive caching / memory leaks
    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();

        for item in &ast.items {
            if let Item::Function(func) = item {
                self.allocations.clear();
                self.check_block(&func.body, &mut violations);

                // Check for leaked allocations
                for (name, state) in &self.allocations {
                    if *state == AllocState::Allocated {
                        violations.push(Violation::full(
                            ViolationKind::MemoryLeak,
                            Span::dummy().into(),
                            format!("Memory '{}' allocated but never freed", name),
                            "Miserliness: Hoarding memory resources",
                            "Entry to Suchimukha (needle-face hell)",
                            format!("Call mukta({}) or drop({}) before scope ends", name, name),
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
            Stmt::Let {
                name,
                value: Some(v),
                ..
            } => {
                if self.is_allocation(v) {
                    self.allocations
                        .insert(name.name.clone(), AllocState::Allocated);
                }
                self.check_expr(v, violations);
            }
            Stmt::Expr(e) => self.check_expr(e, violations),
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
            Stmt::Loop { body, kind, span } => {
                // Check for unbounded growth in loops
                if let LoopKind::Infinite = kind {
                    self.check_loop_growth(body, span, violations);
                }
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
            Expr::Call { callee, args, .. } => {
                // Check for free calls
                if let Expr::Identifier(id) = callee.as_ref() {
                    if self.free_fns.contains(id.name.as_str()) {
                        if let Some(Expr::Identifier(arg_id)) = args.first() {
                            self.allocations
                                .insert(arg_id.name.clone(), AllocState::Freed);
                        }
                    }
                }
                for a in args {
                    self.check_expr(a, violations);
                }
            }
            Expr::MethodCall {
                receiver,
                method,
                args,
                ..
            } => {
                if self.add_fns.contains(method.name.as_str()) {
                    // Mark as added to collection
                    for arg in args {
                        if let Expr::Identifier(id) = arg {
                            if let Some(state) = self.allocations.get_mut(&id.name) {
                                *state = AllocState::AddedToCollection;
                            }
                        }
                    }
                } else if self.free_fns.contains(method.name.as_str()) {
                    if let Expr::Identifier(id) = receiver.as_ref() {
                        self.allocations.insert(id.name.clone(), AllocState::Freed);
                    }
                }
                self.check_expr(receiver, violations);
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

    fn is_allocation(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Call { callee, .. } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    self.alloc_fns.contains(id.name.as_str())
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn check_loop_growth(
        &self,
        body: &Block,
        span: &crate::lexer::token::Span,
        violations: &mut Vec<Violation>,
    ) {
        // Check for allocations inside infinite loops without cleanup
        for stmt in &body.stmts {
            if let Stmt::Let {
                name,
                value: Some(v),
                ..
            } = stmt
            {
                if self.is_allocation(v) {
                    violations.push(Violation::full(
                        ViolationKind::MemoryLeak,
                        span.clone().into(),
                        format!("Allocation '{}' inside infinite loop", name.name),
                        "Extreme miserliness: Infinite hoarding",
                        "Entry to Suchimukha",
                        "Free memory before loop iteration ends".to_string(),
                    ));
                }
            }
        }
    }
}

impl Default for SuchimukhaChecker {
    fn default() -> Self {
        Self::new()
    }
}
