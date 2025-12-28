//! # Sarameyadana - Hell 19: Wild Pointer
//!
//! Sin: Poisoning food
//! Code: Wild/dangling pointer

use super::super::yama::{Violation, ViolationKind};
use crate::lexer::token::Span as TokenSpan;
use crate::parser::ast::{Ast, Block, Expr, Item, Literal, LoopKind, Stmt, UnaryOp};
use std::collections::{HashMap, HashSet};

/// Pointer state tracking
#[derive(Clone, Debug, PartialEq)]
enum PointerState {
    Valid,
    Dangling,
    Uninitialized,
    Returned, // Pointer to local returned
}

/// Checker for Sarameyadana violations (wild/dangling pointer)
pub struct SarameyaDanaChecker {
    /// Track pointer states
    pointers: HashMap<String, PointerState>,
    /// Local variables (for detecting returned locals)
    locals: HashSet<String>,
    /// Address-of expressions
    ref_fns: HashSet<&'static str>,
}

impl SarameyaDanaChecker {
    pub fn new() -> Self {
        Self {
            pointers: HashMap::new(),
            locals: HashSet::new(),
            ref_fns: ["ref", "addr_of", "ptr", "as_ptr", "as_mut_ptr"].into(),
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.pointers.clear();
                self.locals.clear();
                self.check_block(&func.body, &mut violations);
            }
        }
        violations
    }

    fn check_block(&mut self, block: &Block, violations: &mut Vec<Violation>) {
        // Collect locals in this scope
        for stmt in &block.stmts {
            if let Stmt::Let { name, .. } = stmt {
                self.locals.insert(name.name.clone());
            }
        }

        for stmt in &block.stmts {
            self.check_stmt(stmt, violations);
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt, violations: &mut Vec<Violation>) {
        match stmt {
            Stmt::Let {
                name, value, span, ..
            } => {
                if let Some(v) = value {
                    // Check if taking address of something
                    if self.is_pointer_creation(v) {
                        self.pointers.insert(name.name.clone(), PointerState::Valid);
                    }
                    // Check for uninitialized pointer
                    if self.is_null_init(v) {
                        self.pointers
                            .insert(name.name.clone(), PointerState::Uninitialized);
                    }
                    self.check_expr(v, span, violations);
                } else {
                    // Uninitialized variable
                    if self.looks_like_pointer(&name.name) {
                        self.pointers
                            .insert(name.name.clone(), PointerState::Uninitialized);
                    }
                }
            }
            Stmt::Expr(e) => self.check_expr(e, &TokenSpan::dummy(), violations),
            Stmt::If {
                condition,
                then_block,
                else_block,
                span,
            } => {
                self.check_expr(condition, span, violations);
                self.check_block(then_block, violations);
                if let Some(eb) = else_block {
                    self.check_block(eb, violations);
                }
            }
            Stmt::Loop { body, kind, span } => {
                if let LoopKind::While { condition } = kind {
                    self.check_expr(condition, span, violations);
                }
                self.check_block(body, violations);
            }
            Stmt::Return {
                value: Some(v),
                span,
            } => {
                // Check for returning address of local
                if let Some(local_name) = self.get_referenced_local(v) {
                    violations.push(Violation::full(
                        ViolationKind::DanglingPointer,
                        span.clone().into(),
                        format!("Returning reference to local variable '{}'", local_name),
                        "Poisoning food: Serving corrupted memory",
                        "Entry to Sarameyadana (wild dog hell)",
                        "Return owned data or use longer lifetime".to_string(),
                    ));
                }
            }
            _ => {}
        }
    }

    fn check_expr(
        &mut self,
        expr: &Expr,
        loc: &crate::lexer::token::Span,
        violations: &mut Vec<Violation>,
    ) {
        match expr {
            Expr::Identifier(id) => {
                // Check for use of dangling pointer
                if let Some(state) = self.pointers.get(&id.name) {
                    match state {
                        PointerState::Dangling => {
                            violations.push(Violation::full(
                                ViolationKind::DanglingPointer,
                                loc.clone().into(),
                                format!("Use of dangling pointer '{}'", id.name),
                                "Poisoning food: Using corrupted pointer",
                                "Entry to Sarameyadana",
                                "Ensure pointer is valid before use".to_string(),
                            ));
                        }
                        PointerState::Uninitialized => {
                            violations.push(Violation::full(
                                ViolationKind::MemoryCorruption,
                                loc.clone().into(),
                                format!("Use of uninitialized pointer '{}'", id.name),
                                "Poisoning food: Using wild pointer",
                                "Entry to Sarameyadana",
                                "Initialize pointer before use".to_string(),
                            ));
                        }
                        _ => {}
                    }
                }
            }
            Expr::Unary { op, operand, .. } => {
                // Dereference of pointer
                if *op == UnaryOp::Deref {
                    if let Expr::Identifier(id) = operand.as_ref() {
                        if let Some(state) = self.pointers.get(&id.name) {
                            if *state != PointerState::Valid {
                                violations.push(Violation::full(
                                    ViolationKind::DanglingPointer,
                                    loc.clone().into(),
                                    format!("Dereference of invalid pointer '{}'", id.name),
                                    "Poisoning food: Dereferencing wild pointer",
                                    "Entry to Sarameyadana",
                                    "Validate pointer before dereference".to_string(),
                                ));
                            }
                        }
                    }
                }
                self.check_expr(operand, loc, violations);
            }
            Expr::Call { callee, args, .. } => {
                // Check for free/drop that invalidates pointers
                if let Expr::Identifier(id) = callee.as_ref() {
                    if ["free", "drop", "mukta", "dealloc"].contains(&id.name.as_str()) {
                        for arg in args {
                            if let Expr::Identifier(arg_id) = arg {
                                self.pointers
                                    .insert(arg_id.name.clone(), PointerState::Dangling);
                            }
                        }
                    }
                }
                for a in args {
                    self.check_expr(a, loc, violations);
                }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left, loc, violations);
                self.check_expr(right, loc, violations);
            }
            _ => {}
        }
    }

    fn is_pointer_creation(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Unary { op, .. } if *op == UnaryOp::Ref => true,
            Expr::Call { callee, .. } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    self.ref_fns.contains(id.name.as_str())
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn is_null_init(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Literal(Literal::Int(0)) => true,
            Expr::Identifier(id) => id.name == "null" || id.name == "nullptr",
            _ => false,
        }
    }

    fn looks_like_pointer(&self, name: &str) -> bool {
        name.ends_with("_ptr") || name.ends_with("_p") || name.starts_with("p_")
    }

    fn get_referenced_local(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::Unary { op, operand, .. } if *op == UnaryOp::Ref => {
                if let Expr::Identifier(id) = operand.as_ref() {
                    if self.locals.contains(&id.name) {
                        return Some(id.name.clone());
                    }
                }
                None
            }
            _ => None,
        }
    }
}

impl Default for SarameyaDanaChecker {
    fn default() -> Self {
        Self::new()
    }
}
