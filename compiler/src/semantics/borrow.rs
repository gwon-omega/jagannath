//! Borrow Checker - Ṛṇa (Debt) System
//!
//! Implements ownership and borrowing checking based on:
//! - Substructural type theory (linear/affine types)
//! - Rust's borrow checker semantics
//! - Sanskrit affix-encoded ownership (-l linear, -b borrowed, -g global)
//!
//! Key concepts mapped to Sanskrit:
//! - स्वामित्व (Svāmitva) - Ownership
//! - ऋण (Ṛṇa) - Debt/Borrow
//! - गति (Gati) - Move
//! - मोक्ष (Mokṣa) - Liberation/Drop
//!
//! Reference: Walker (2002) "Substructural Type Systems" in ATTAPL
//! Reference: Rust Reference on Ownership and Borrowing

use crate::lexer::{Affix, Span};
use crate::parser::ast::*;
use std::collections::{HashMap, HashSet};

/// Borrow checker - tracks ownership, moves, and borrows
pub struct BorrowChecker {
    /// Ownership states for all values
    owned: HashMap<String, OwnershipInfo>,
    /// Active borrows (owner -> list of borrow info)
    borrows: HashMap<String, Vec<BorrowInfo>>,
    /// Scope stack for tracking lexical regions
    scope_stack: Vec<ScopeInfo>,
    /// Current scope depth
    current_scope: usize,
    /// Errors accumulated during checking
    errors: Vec<BorrowError>,
    /// Function context for return checking
    function_context: Option<FunctionContext>,
}

/// Complete ownership information for a value
#[derive(Debug, Clone)]
pub struct OwnershipInfo {
    /// Current state
    pub state: OwnershipState,
    /// Ownership kind from type affixes
    pub kind: OwnershipKind,
    /// Type of the value
    pub ty: String,
    /// Scope where defined
    pub defined_scope: usize,
    /// Source location
    pub span: Span,
    /// Move history for diagnostics
    pub move_history: Vec<MoveRecord>,
}

/// Ownership state for a value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnershipState {
    /// Value is owned and valid
    Owned,
    /// Value has been moved
    Moved,
    /// Value is borrowed (immutably)
    BorrowedShared,
    /// Value is borrowed (mutably)
    BorrowedMut,
    /// Value has been consumed (linear type used exactly once)
    Consumed,
    /// Value is partially moved (some fields moved)
    PartiallyMoved,
    /// Value is uninitialized
    Uninitialized,
}

/// Ownership kind derived from Sanskrit affixes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnershipKind {
    /// -l suffix: Linear - must be used exactly once
    Linear,
    /// -b suffix: Borrowed - reference, cannot outlive owner
    Borrowed { mutable: bool },
    /// -g suffix: Global/Static - lives for program duration
    Global,
    /// Default: Affine - can be used at most once (Rust default)
    Affine,
    /// Copy type - can be freely copied
    Copy,
}

/// Information about an active borrow
#[derive(Debug, Clone)]
pub struct BorrowInfo {
    /// Name of the borrower variable
    pub borrower: String,
    /// Name of the owner being borrowed from
    pub owner: String,
    /// Is this a mutable borrow?
    pub mutable: bool,
    /// Lifetime region (from ^N suffix)
    pub region: u8,
    /// Scope where borrow started
    pub start_scope: usize,
    /// Source location
    pub span: Span,
    /// What is being borrowed (full value or field path)
    pub path: BorrowPath,
}

/// Path to a borrowed value
#[derive(Debug, Clone)]
pub enum BorrowPath {
    /// Entire value
    Full,
    /// Specific field
    Field(String),
    /// Array element
    Index(usize),
    /// Dynamic index
    DynamicIndex,
}

/// Record of a move for diagnostics
#[derive(Debug, Clone)]
pub struct MoveRecord {
    /// Where the move occurred
    pub span: Span,
    /// What it was moved to
    pub destination: String,
    /// Scope where move occurred
    pub scope: usize,
}

/// Scope information
#[derive(Debug, Clone)]
struct ScopeInfo {
    /// Scope ID
    id: usize,
    /// Parent scope
    parent: Option<usize>,
    /// Values defined in this scope
    defined: HashSet<String>,
    /// Borrows active in this scope
    borrows: HashSet<String>,
    /// Is this a loop scope?
    is_loop: bool,
}

/// Function context for return checking
#[derive(Debug, Clone)]
struct FunctionContext {
    /// Function name
    name: String,
    /// Parameter names and their ownership kinds
    params: HashMap<String, OwnershipKind>,
    /// Return type ownership
    return_ownership: Option<OwnershipKind>,
    /// Output parameter (for linear returns)
    output_param: Option<String>,
}

impl BorrowChecker {
    /// Create a new borrow checker
    pub fn new() -> Self {
        Self {
            owned: HashMap::new(),
            borrows: HashMap::new(),
            scope_stack: vec![ScopeInfo {
                id: 0,
                parent: None,
                defined: HashSet::new(),
                borrows: HashSet::new(),
                is_loop: false,
            }],
            current_scope: 0,
            errors: Vec::new(),
            function_context: None,
        }
    }

    /// Check borrow rules for a function
    pub fn check_function(&mut self, func: &FunctionDef) -> Result<(), Vec<BorrowError>> {
        // Reset state
        self.owned.clear();
        self.borrows.clear();
        self.errors.clear();
        self.scope_stack.clear();
        self.scope_stack.push(ScopeInfo {
            id: 0,
            parent: None,
            defined: HashSet::new(),
            borrows: HashSet::new(),
            is_loop: false,
        });
        self.current_scope = 0;

        // Set up function context
        let mut params = HashMap::new();
        for param in &func.params {
            let kind = self.extract_ownership_kind(&param.ty);
            params.insert(param.name.name.clone(), kind);

            // Register parameter as owned value
            self.record_owned(
                param.name.name.clone(),
                self.type_to_string(&param.ty),
                kind,
                param.span.clone(),
            );
        }

        self.function_context = Some(FunctionContext {
            name: func.name.name.clone(),
            params,
            return_ownership: func
                .return_type
                .as_ref()
                .map(|t| self.extract_ownership_kind(t)),
            output_param: None,
        });

        // Enter function scope
        self.enter_scope(false);

        // Check function body
        self.check_block(&func.body)?;

        // Exit function scope
        self.exit_scope();

        // Check that all linear values have been consumed
        self.check_linear_consumed()?;

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Check a block of statements
    fn check_block(&mut self, block: &Block) -> Result<(), Vec<BorrowError>> {
        for stmt in &block.stmts {
            self.check_stmt(stmt)?;
        }
        Ok(())
    }

    /// Check a single statement
    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), Vec<BorrowError>> {
        match stmt {
            Stmt::Let {
                name,
                ty,
                value,
                span,
            } => {
                self.check_let(name, ty.as_ref(), value.as_ref(), span.clone())?;
            }
            Stmt::Expr(expr) => {
                self.check_expr(expr)?;
            }
            Stmt::Return { value, span } => {
                self.check_return(value.as_ref(), span.clone())?;
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
                span,
            } => {
                self.check_if(condition, then_block, else_block.as_ref(), span.clone())?;
            }
            Stmt::Match {
                scrutinee,
                arms,
                span,
            } => {
                self.check_match(scrutinee, arms, span.clone())?;
            }
            Stmt::Loop { kind, body, span } => {
                self.check_loop(kind, body, span.clone())?;
            }
            Stmt::Break { span: _ } => {
                // Break is valid - no borrow checking needed
            }
            Stmt::Continue { span: _ } => {
                // Continue is valid - no borrow checking needed
            }
        }
        Ok(())
    }

    /// Check let binding
    fn check_let(
        &mut self,
        name: &Identifier,
        ty: Option<&Type>,
        value: Option<&Expr>,
        span: Span,
    ) -> Result<(), Vec<BorrowError>> {
        // Determine ownership kind from type affixes
        let kind = ty
            .map(|t| self.extract_ownership_kind(t))
            .unwrap_or(OwnershipKind::Affine);
        let ty_str = ty
            .map(|t| self.type_to_string(t))
            .unwrap_or_else(|| "unknown".to_string());

        if let Some(value) = value {
            // Check the value expression
            self.check_expr(value)?;

            // If value is an identifier, check if it's a move
            if let Expr::Identifier(id) = value {
                self.handle_move_or_copy(&id.name, &name.name, span.clone())?;
            }
        }

        // Record the new owned value
        self.record_owned(name.name.clone(), ty_str, kind, span);

        Ok(())
    }

    /// Check expression and track ownership
    fn check_expr(&mut self, expr: &Expr) -> Result<OwnershipState, Vec<BorrowError>> {
        match expr {
            Expr::Literal(_) => Ok(OwnershipState::Owned),

            Expr::Identifier(id) => self.check_use(&id.name, id.span.clone()),

            Expr::Binary {
                left,
                op: _,
                right,
                span: _,
            } => {
                self.check_expr(left)?;
                self.check_expr(right)?;
                Ok(OwnershipState::Owned)
            }

            Expr::Unary { op, operand, span } => {
                // Check for address-of (borrow) operations
                match op {
                    UnaryOp::Ref => {
                        if let Expr::Identifier(id) = operand.as_ref() {
                            self.record_borrow_from_expr(&id.name, false, span.clone())?;
                        }
                    }
                    UnaryOp::Deref => {
                        // Deref operations need the value to be valid
                    }
                    _ => {}
                }
                self.check_expr(operand)?;
                Ok(OwnershipState::Owned)
            }

            Expr::Call { callee, args, span } => {
                // Check callee expression
                self.check_expr(callee)?;

                // Check arguments - each may move or borrow
                for arg in args {
                    self.check_expr(arg)?;

                    // If argument is an identifier, it may be moved
                    if let Expr::Identifier(id) = arg {
                        // For simplicity, assume non-Copy types are moved
                        if let Some(info) = self.owned.get(&id.name) {
                            if info.kind != OwnershipKind::Copy {
                                self.handle_move_or_copy(&id.name, "<function_arg>", span.clone())?;
                            }
                        }
                    }
                }
                Ok(OwnershipState::Owned)
            }

            Expr::FieldAccess {
                object,
                field: _,
                span: _,
            } => {
                self.check_expr(object)?;
                // Field access borrows the object
                Ok(OwnershipState::Owned)
            }

            Expr::Index {
                object,
                index,
                span: _,
            } => {
                self.check_expr(object)?;
                self.check_expr(index)?;
                Ok(OwnershipState::Owned)
            }

            Expr::StructConstruct {
                name: _,
                fields,
                span,
            } => {
                for (_, field_expr) in fields {
                    self.check_expr(field_expr)?;

                    // Field values may be moved into struct
                    if let Expr::Identifier(id) = field_expr {
                        if let Some(info) = self.owned.get(&id.name) {
                            if info.kind != OwnershipKind::Copy {
                                self.handle_move_or_copy(&id.name, "<struct_field>", span.clone())?;
                            }
                        }
                    }
                }
                Ok(OwnershipState::Owned)
            }

            Expr::Array { elements, span: _ } => {
                for elem in elements {
                    self.check_expr(elem)?;
                }
                Ok(OwnershipState::Owned)
            }

            Expr::Tuple { elements, span: _ } => {
                for elem in elements {
                    self.check_expr(elem)?;
                }
                Ok(OwnershipState::Owned)
            }

            Expr::Lambda {
                params,
                body,
                span: _,
            } => {
                // Lambda captures - check what's captured
                self.enter_scope(false);

                // Register parameters
                for param in params {
                    let kind = self.extract_ownership_kind(&param.ty);
                    self.record_owned(
                        param.name.name.clone(),
                        self.type_to_string(&param.ty),
                        kind,
                        param.span.clone(),
                    );
                }

                // Lambda body is an expression
                self.check_expr(body)?;
                self.exit_scope();

                Ok(OwnershipState::Owned)
            }

            Expr::MethodCall {
                receiver,
                method: _,
                args,
                span,
            } => {
                self.check_expr(receiver)?;
                for arg in args {
                    self.check_expr(arg)?;

                    // Arguments may be moved
                    if let Expr::Identifier(id) = arg {
                        if let Some(info) = self.owned.get(&id.name) {
                            if info.kind != OwnershipKind::Copy {
                                self.handle_move_or_copy(&id.name, "<method_arg>", span.clone())?;
                            }
                        }
                    }
                }
                Ok(OwnershipState::Owned)
            }

            Expr::If {
                condition,
                then_expr,
                else_expr,
                span: _,
            } => {
                self.check_expr(condition)?;

                // Both branches must be valid
                self.enter_scope(false);
                self.check_expr(then_expr)?;
                self.exit_scope();

                if let Some(else_br) = else_expr {
                    self.enter_scope(false);
                    self.check_expr(else_br)?;
                    self.exit_scope();
                }

                Ok(OwnershipState::Owned)
            }

            Expr::Block(block) => {
                self.enter_scope(false);
                self.check_block(block)?;
                self.exit_scope();
                Ok(OwnershipState::Owned)
            }

            Expr::Cast {
                expr,
                ty: _,
                span: _,
            } => self.check_expr(expr),

            Expr::Try { expr, span: _ } => self.check_expr(expr),

            Expr::Await { expr, span: _ } => self.check_expr(expr),
        }
    }

    /// Check if using a value is valid
    fn check_use(&mut self, name: &str, span: Span) -> Result<OwnershipState, Vec<BorrowError>> {
        if let Some(info) = self.owned.get(name) {
            match info.state {
                OwnershipState::Owned | OwnershipState::BorrowedShared => Ok(info.state),
                OwnershipState::Moved => {
                    let last_move = info.move_history.last();
                    self.errors.push(BorrowError::UseAfterMove {
                        name: name.to_string(),
                        moved_at: last_move.map(|m| m.span.clone()),
                        used_at: span,
                    });
                    Err(std::mem::take(&mut self.errors))
                }
                OwnershipState::Consumed => {
                    self.errors.push(BorrowError::UseAfterConsume {
                        name: name.to_string(),
                    });
                    Err(std::mem::take(&mut self.errors))
                }
                OwnershipState::BorrowedMut => {
                    self.errors.push(BorrowError::UseWhileMutablyBorrowed {
                        name: name.to_string(),
                        span,
                    });
                    Err(std::mem::take(&mut self.errors))
                }
                OwnershipState::PartiallyMoved => {
                    self.errors.push(BorrowError::UseOfPartiallyMoved {
                        name: name.to_string(),
                        span,
                    });
                    Err(std::mem::take(&mut self.errors))
                }
                OwnershipState::Uninitialized => {
                    self.errors.push(BorrowError::UseOfUninitialized {
                        name: name.to_string(),
                        span,
                    });
                    Err(std::mem::take(&mut self.errors))
                }
            }
        } else {
            // Value not found - might be from outer scope
            Ok(OwnershipState::Owned)
        }
    }

    /// Check return statement
    fn check_return(&mut self, value: Option<&Expr>, span: Span) -> Result<(), Vec<BorrowError>> {
        if let Some(value) = value {
            self.check_expr(value)?;

            // Check for returning local reference
            if let Expr::Unary {
                op: UnaryOp::Ref,
                operand,
                ..
            } = value
            {
                if let Expr::Identifier(id) = operand.as_ref() {
                    if self.is_local(&id.name) {
                        self.errors.push(BorrowError::ReturnLocalReference {
                            name: id.name.clone(),
                            span,
                        });
                    }
                }
            }
        }
        Ok(())
    }

    /// Check if statement
    fn check_if(
        &mut self,
        condition: &Expr,
        then_block: &Block,
        else_block: Option<&Block>,
        span: Span,
    ) -> Result<(), Vec<BorrowError>> {
        self.check_expr(condition)?;

        // Save state before branches
        let state_before = self.owned.clone();

        self.enter_scope(false);
        self.check_block(then_block)?;
        let then_state = self.owned.clone();
        self.exit_scope();

        // Restore and check else
        self.owned = state_before.clone();

        if let Some(else_block) = else_block {
            self.enter_scope(false);
            self.check_block(else_block)?;
            let else_state = self.owned.clone();
            self.exit_scope();

            // Merge states - both branches must agree on moved values
            self.merge_branch_states(&then_state, &else_state, span)?;
        } else {
            // Without else, restore original state
            self.owned = state_before;
        }

        Ok(())
    }

    /// Check match statement
    fn check_match(
        &mut self,
        scrutinee: &Expr,
        arms: &[MatchArm],
        span: Span,
    ) -> Result<(), Vec<BorrowError>> {
        self.check_expr(scrutinee)?;

        // Save state before arms
        let state_before = self.owned.clone();
        let mut arm_states = Vec::new();

        for arm in arms {
            self.owned = state_before.clone();

            self.enter_scope(false);
            self.check_pattern(&arm.pattern, span.clone())?;
            self.check_expr(&arm.body)?;
            arm_states.push(self.owned.clone());
            self.exit_scope();
        }

        // All arms must agree on moved values
        if !arm_states.is_empty() {
            let first = &arm_states[0];
            for other in arm_states.iter().skip(1) {
                self.merge_branch_states(first, other, span.clone())?;
            }
        }

        Ok(())
    }

    /// Check loop statement
    fn check_loop(
        &mut self,
        kind: &LoopKind,
        body: &Block,
        _span: Span,
    ) -> Result<(), Vec<BorrowError>> {
        // Check loop condition/iterator if present
        match kind {
            LoopKind::While { condition } => {
                self.check_expr(condition)?;
            }
            LoopKind::ForIn {
                binding: _,
                iterable,
            } => {
                self.check_expr(iterable)?;
            }
            LoopKind::Range {
                binding: _,
                start,
                end,
                inclusive: _,
            } => {
                self.check_expr(start)?;
                self.check_expr(end)?;
            }
            LoopKind::Infinite => {}
        }

        self.enter_scope(true); // Mark as loop scope
        self.check_block(body)?;
        self.exit_scope();

        Ok(())
    }

    /// Check pattern and bind values
    fn check_pattern(&mut self, pattern: &Pattern, span: Span) -> Result<(), Vec<BorrowError>> {
        match pattern {
            Pattern::Identifier(id) => {
                // Pattern binding creates new owned value
                self.record_owned(
                    id.name.clone(),
                    "unknown".to_string(),
                    OwnershipKind::Affine,
                    span,
                );
            }
            Pattern::Constructor { name: _, fields } => {
                for p in fields {
                    self.check_pattern(p, span.clone())?;
                }
            }
            Pattern::Binding {
                name,
                mutable: _,
                subpattern,
            } => {
                // Named binding - record the name as owned
                self.record_owned(
                    name.name.clone(),
                    "unknown".to_string(),
                    OwnershipKind::Affine,
                    span.clone(),
                );
                // Check subpattern if present
                if let Some(sub) = subpattern {
                    self.check_pattern(sub, span)?;
                }
            }
            Pattern::Tuple(patterns) => {
                for p in patterns {
                    self.check_pattern(p, span.clone())?;
                }
            }
            Pattern::Struct {
                name: _,
                fields,
                rest: _,
            } => {
                for (_, p) in fields {
                    self.check_pattern(p, span.clone())?;
                }
            }
            Pattern::Variant {
                enum_name: _,
                variant: _,
                fields,
            } => match fields {
                crate::parser::ast::VariantFields::Unit => {}
                crate::parser::ast::VariantFields::Tuple(patterns) => {
                    for p in patterns {
                        self.check_pattern(p, span.clone())?;
                    }
                }
                crate::parser::ast::VariantFields::Struct(field_patterns) => {
                    for (_, p) in field_patterns {
                        self.check_pattern(p, span.clone())?;
                    }
                }
            },
            Pattern::Array(patterns) => {
                for p in patterns {
                    self.check_pattern(p, span.clone())?;
                }
            }
            Pattern::Slice {
                before,
                middle,
                after,
            } => {
                for p in before {
                    self.check_pattern(p, span.clone())?;
                }
                if let Some(mid) = middle {
                    self.check_pattern(mid, span.clone())?;
                }
                for p in after {
                    self.check_pattern(p, span.clone())?;
                }
            }
            Pattern::Range { start, end, .. } => {
                if let Some(p) = start {
                    self.check_pattern(p, span.clone())?;
                }
                if let Some(p) = end {
                    self.check_pattern(p, span.clone())?;
                }
            }
            Pattern::Or(patterns) => {
                // All branches must bind same names
                for p in patterns {
                    self.check_pattern(p, span.clone())?;
                }
            }
            Pattern::Guard {
                pattern,
                condition: _,
            } => {
                self.check_pattern(pattern, span)?;
            }
            Pattern::Ref {
                mutable: _,
                pattern: inner,
            } => {
                self.check_pattern(inner, span)?;
            }
            Pattern::Literal(_) => {}
            Pattern::Wildcard => {}
            Pattern::Rest => {}
        }
        Ok(())
    }

    // ========== Helper Methods ==========

    /// Record a new owned value
    pub fn record_owned(&mut self, name: String, ty: String, kind: OwnershipKind, span: Span) {
        self.owned.insert(
            name.clone(),
            OwnershipInfo {
                state: OwnershipState::Owned,
                kind,
                ty,
                defined_scope: self.current_scope,
                span,
                move_history: Vec::new(),
            },
        );

        // Add to current scope's defined set
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.defined.insert(name);
        }
    }

    /// Handle move or copy operation
    fn handle_move_or_copy(
        &mut self,
        from: &str,
        to: &str,
        span: Span,
    ) -> Result<(), Vec<BorrowError>> {
        if let Some(info) = self.owned.get(from) {
            // Check current state
            match info.state {
                OwnershipState::Owned => {
                    // Check if Copy type
                    if info.kind == OwnershipKind::Copy {
                        // Copy - original stays valid
                        return Ok(());
                    }

                    // Move - mark as moved
                    let mut new_info = info.clone();
                    new_info.state = OwnershipState::Moved;
                    new_info.move_history.push(MoveRecord {
                        span,
                        destination: to.to_string(),
                        scope: self.current_scope,
                    });
                    self.owned.insert(from.to_string(), new_info);
                    Ok(())
                }
                OwnershipState::Moved => Err(vec![BorrowError::UseAfterMove {
                    name: from.to_string(),
                    moved_at: info.move_history.last().map(|m| m.span.clone()),
                    used_at: span,
                }]),
                OwnershipState::Consumed => Err(vec![BorrowError::UseAfterConsume {
                    name: from.to_string(),
                }]),
                OwnershipState::BorrowedMut => Err(vec![BorrowError::MoveWhileBorrowed {
                    name: from.to_string(),
                }]),
                OwnershipState::BorrowedShared => Err(vec![BorrowError::MoveWhileBorrowed {
                    name: from.to_string(),
                }]),
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }

    /// Record a borrow
    fn record_borrow_from_expr(
        &mut self,
        owner: &str,
        mutable: bool,
        span: Span,
    ) -> Result<(), Vec<BorrowError>> {
        // Check current state
        if let Some(info) = self.owned.get(owner) {
            match info.state {
                OwnershipState::Owned => {
                    // Check for conflicting borrows
                    if let Some(existing_borrows) = self.borrows.get(owner) {
                        for borrow in existing_borrows {
                            if mutable || borrow.mutable {
                                return Err(vec![BorrowError::ConflictingBorrow {
                                    owner: owner.to_string(),
                                    existing: borrow.borrower.clone(),
                                    new: "<expr>".to_string(),
                                }]);
                            }
                        }
                    }

                    // Record the borrow
                    let borrow_info = BorrowInfo {
                        borrower: format!("__borrow_{}", self.borrows.len()),
                        owner: owner.to_string(),
                        mutable,
                        region: 0,
                        start_scope: self.current_scope,
                        span,
                        path: BorrowPath::Full,
                    };

                    self.borrows
                        .entry(owner.to_string())
                        .or_insert_with(Vec::new)
                        .push(borrow_info);

                    // Update owner state
                    let mut new_info = info.clone();
                    new_info.state = if mutable {
                        OwnershipState::BorrowedMut
                    } else {
                        OwnershipState::BorrowedShared
                    };
                    self.owned.insert(owner.to_string(), new_info);

                    Ok(())
                }
                OwnershipState::Moved => Err(vec![BorrowError::BorrowAfterMove {
                    name: owner.to_string(),
                }]),
                OwnershipState::BorrowedMut if mutable => {
                    Err(vec![BorrowError::SecondMutableBorrow {
                        name: owner.to_string(),
                        span,
                    }])
                }
                OwnershipState::BorrowedMut => Err(vec![BorrowError::BorrowWhileMutablyBorrowed {
                    name: owner.to_string(),
                    span,
                }]),
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }

    /// Record a borrow with explicit borrower name
    pub fn record_borrow(
        &mut self,
        borrower: String,
        owner: &str,
        mutable: bool,
        region: u8,
    ) -> Result<(), BorrowError> {
        match self.owned.get(owner) {
            Some(info) if info.state == OwnershipState::Owned => {
                // Check for conflicting borrows
                if let Some(existing_borrows) = self.borrows.get(owner) {
                    for borrow in existing_borrows {
                        if borrow.owner == owner {
                            if mutable || borrow.mutable {
                                return Err(BorrowError::ConflictingBorrow {
                                    owner: owner.to_string(),
                                    existing: borrow.borrower.clone(),
                                    new: borrower,
                                });
                            }
                        }
                    }
                }

                let borrow_info = BorrowInfo {
                    borrower: borrower.clone(),
                    owner: owner.to_string(),
                    mutable,
                    region,
                    start_scope: self.current_scope,
                    span: Span::dummy(),
                    path: BorrowPath::Full,
                };

                self.borrows
                    .entry(owner.to_string())
                    .or_insert_with(Vec::new)
                    .push(borrow_info);

                // Update owner state
                let mut new_info = self.owned.get(owner).unwrap().clone();
                new_info.state = if mutable {
                    OwnershipState::BorrowedMut
                } else {
                    OwnershipState::BorrowedShared
                };
                self.owned.insert(owner.to_string(), new_info);

                Ok(())
            }
            Some(info) if info.state == OwnershipState::Moved => {
                Err(BorrowError::BorrowAfterMove {
                    name: owner.to_string(),
                })
            }
            _ => Err(BorrowError::UnknownValue(owner.to_string())),
        }
    }

    /// Release a borrow
    pub fn release_borrow(&mut self, borrower: &str) {
        // Find and remove the borrow
        for (_owner, borrows) in self.borrows.iter_mut() {
            borrows.retain(|b| b.borrower != borrower);
        }

        // Update owner states
        for (owner, borrows) in &self.borrows {
            if borrows.is_empty() {
                if let Some(info) = self.owned.get_mut(owner) {
                    if matches!(
                        info.state,
                        OwnershipState::BorrowedShared | OwnershipState::BorrowedMut
                    ) {
                        info.state = OwnershipState::Owned;
                    }
                }
            }
        }
    }

    /// Consume a linear value
    pub fn consume_linear(&mut self, name: &str) -> Result<(), BorrowError> {
        match self.owned.get(name) {
            Some(info) if info.state == OwnershipState::Owned => {
                if info.kind == OwnershipKind::Linear {
                    let mut new_info = info.clone();
                    new_info.state = OwnershipState::Consumed;
                    self.owned.insert(name.to_string(), new_info);
                    Ok(())
                } else {
                    // Non-linear types just get marked as moved
                    let mut new_info = info.clone();
                    new_info.state = OwnershipState::Moved;
                    self.owned.insert(name.to_string(), new_info);
                    Ok(())
                }
            }
            Some(info) if info.state == OwnershipState::Consumed => {
                Err(BorrowError::DoubleConsume {
                    name: name.to_string(),
                })
            }
            Some(info) if info.state == OwnershipState::Moved => Err(BorrowError::UseAfterMove {
                name: name.to_string(),
                moved_at: info.move_history.last().map(|m| m.span.clone()),
                used_at: Span::dummy(),
            }),
            _ => Err(BorrowError::UnknownValue(name.to_string())),
        }
    }

    /// Check that all linear values have been consumed
    pub fn check_linear_consumed(&self) -> Result<(), Vec<BorrowError>> {
        let mut errors = Vec::new();

        for (name, info) in &self.owned {
            if info.kind == OwnershipKind::Linear && info.state == OwnershipState::Owned {
                errors.push(BorrowError::LinearNotConsumed {
                    name: name.clone(),
                    span: info.span.clone(),
                });
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Enter a new scope
    fn enter_scope(&mut self, is_loop: bool) {
        let new_id = self.scope_stack.len();
        self.scope_stack.push(ScopeInfo {
            id: new_id,
            parent: Some(self.current_scope),
            defined: HashSet::new(),
            borrows: HashSet::new(),
            is_loop,
        });
        self.current_scope = new_id;
    }

    /// Exit current scope
    fn exit_scope(&mut self) {
        if let Some(scope) = self.scope_stack.pop() {
            // Release borrows from this scope
            for borrower in &scope.borrows {
                self.release_borrow(borrower);
            }

            // Drop values defined in this scope
            for name in &scope.defined {
                if let Some(info) = self.owned.get(name) {
                    // Check linear values were consumed
                    if info.kind == OwnershipKind::Linear && info.state == OwnershipState::Owned {
                        self.errors.push(BorrowError::LinearNotConsumed {
                            name: name.clone(),
                            span: info.span.clone(),
                        });
                    }
                }
                self.owned.remove(name);
            }
        }

        if let Some(scope) = self.scope_stack.last() {
            self.current_scope = scope.id;
        }
    }

    /// Check if a value is local to current function
    fn is_local(&self, name: &str) -> bool {
        if let Some(info) = self.owned.get(name) {
            // Local if defined in any scope > 0 (function scope)
            info.defined_scope > 0
        } else {
            false
        }
    }

    /// Merge states from two branches
    fn merge_branch_states(
        &mut self,
        state1: &HashMap<String, OwnershipInfo>,
        state2: &HashMap<String, OwnershipInfo>,
        span: Span,
    ) -> Result<(), Vec<BorrowError>> {
        // Check that both branches agree on moved/consumed values
        for (name, info1) in state1 {
            if let Some(info2) = state2.get(name) {
                // If moved in one branch but not the other, that's an error
                let moved1 = matches!(
                    info1.state,
                    OwnershipState::Moved | OwnershipState::Consumed
                );
                let moved2 = matches!(
                    info2.state,
                    OwnershipState::Moved | OwnershipState::Consumed
                );

                if moved1 != moved2 {
                    self.errors.push(BorrowError::PartialMoveInBranch {
                        name: name.clone(),
                        span: span.clone(),
                    });
                }
            }
        }

        // Use conservative state (if moved in either, consider moved)
        for (name, info1) in state1 {
            if let Some(info2) = state2.get(name) {
                let state = match (info1.state, info2.state) {
                    (OwnershipState::Moved, _) | (_, OwnershipState::Moved) => {
                        OwnershipState::Moved
                    }
                    (OwnershipState::Consumed, _) | (_, OwnershipState::Consumed) => {
                        OwnershipState::Consumed
                    }
                    _ => info1.state,
                };

                if let Some(current) = self.owned.get_mut(name) {
                    current.state = state;
                }
            }
        }

        Ok(())
    }

    /// Extract ownership kind from type affixes
    fn extract_ownership_kind(&self, ty: &Type) -> OwnershipKind {
        match ty {
            Type::Named { affixes, .. } => {
                // Check for -l (linear)
                if affixes.contains(&Affix::L) {
                    return OwnershipKind::Linear;
                }
                // Check for -b (borrowed)
                if affixes.contains(&Affix::B) {
                    let mutable = affixes.contains(&Affix::Aa); // -ā for mutable
                    return OwnershipKind::Borrowed { mutable };
                }
                // Check for -g (global)
                if affixes.contains(&Affix::G) {
                    return OwnershipKind::Global;
                }
                OwnershipKind::Affine
            }
            Type::Reference { mutable, .. } => OwnershipKind::Borrowed { mutable: *mutable },
            _ => OwnershipKind::Affine,
        }
    }

    /// Convert type to string for diagnostics
    fn type_to_string(&self, ty: &Type) -> String {
        match ty {
            Type::Named { name, .. } => name.name.clone(),
            Type::Function { .. } => "fn".to_string(),
            Type::Array { element, .. } => format!("[{}]", self.type_to_string(element)),
            Type::Tuple(types) => {
                let types_str: Vec<_> = types.iter().map(|t| self.type_to_string(t)).collect();
                format!("({})", types_str.join(", "))
            }
            Type::Reference { inner, mutable, .. } => {
                let prefix = if *mutable { "&mut " } else { "&" };
                format!("{}{}", prefix, self.type_to_string(inner))
            }
            Type::Inferred => "_".to_string(),
        }
    }

    /// Get accumulated errors
    pub fn errors(&self) -> &[BorrowError] {
        &self.errors
    }
}

/// Borrow error
#[derive(Debug, Clone)]
pub enum BorrowError {
    /// Use after the value was moved
    UseAfterMove {
        name: String,
        moved_at: Option<Span>,
        used_at: Span,
    },
    /// Use after the value was consumed (linear type)
    UseAfterConsume { name: String },
    /// Move while borrowed
    MoveWhileBorrowed { name: String },
    /// Borrow after move
    BorrowAfterMove { name: String },
    /// Conflicting borrow (multiple mutable or mutable+shared)
    ConflictingBorrow {
        owner: String,
        existing: String,
        new: String,
    },
    /// Double consumption of linear value
    DoubleConsume { name: String },
    /// Unknown value reference
    UnknownValue(String),
    /// Linear value not consumed
    LinearNotConsumed { name: String, span: Span },
    /// Return of local reference
    ReturnLocalReference { name: String, span: Span },
    /// Use while mutably borrowed
    UseWhileMutablyBorrowed { name: String, span: Span },
    /// Use of partially moved value
    UseOfPartiallyMoved { name: String, span: Span },
    /// Use of uninitialized value
    UseOfUninitialized { name: String, span: Span },
    /// Second mutable borrow
    SecondMutableBorrow { name: String, span: Span },
    /// Borrow while already mutably borrowed
    BorrowWhileMutablyBorrowed { name: String, span: Span },
    /// Partial move in one branch but not another
    PartialMoveInBranch { name: String, span: Span },
}

impl std::fmt::Display for BorrowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BorrowError::UseAfterMove {
                name,
                moved_at,
                used_at: _,
            } => {
                write!(f, "use of moved value `{}`", name)?;
                if let Some(span) = moved_at {
                    write!(f, " (moved at line {})", span.line)?;
                }
                Ok(())
            }
            BorrowError::UseAfterConsume { name } => {
                write!(
                    f,
                    "use of consumed linear value `{}` (must be used exactly once)",
                    name
                )
            }
            BorrowError::MoveWhileBorrowed { name } => {
                write!(f, "cannot move `{}` while it is borrowed", name)
            }
            BorrowError::BorrowAfterMove { name } => {
                write!(f, "cannot borrow `{}` after it has been moved", name)
            }
            BorrowError::ConflictingBorrow {
                owner,
                existing,
                new: _,
            } => {
                write!(
                    f,
                    "cannot borrow `{}` as mutable while already borrowed by `{}`",
                    owner, existing
                )
            }
            BorrowError::DoubleConsume { name } => {
                write!(f, "linear value `{}` consumed more than once", name)
            }
            BorrowError::UnknownValue(name) => {
                write!(f, "unknown value `{}`", name)
            }
            BorrowError::LinearNotConsumed { name, span } => {
                write!(
                    f,
                    "linear value `{}` must be consumed before scope ends (at line {})",
                    name, span.line
                )
            }
            BorrowError::ReturnLocalReference { name, span: _ } => {
                write!(f, "cannot return reference to local variable `{}`", name)
            }
            BorrowError::UseWhileMutablyBorrowed { name, span: _ } => {
                write!(f, "cannot use `{}` while it is mutably borrowed", name)
            }
            BorrowError::UseOfPartiallyMoved { name, span: _ } => {
                write!(f, "use of partially moved value `{}`", name)
            }
            BorrowError::UseOfUninitialized { name, span: _ } => {
                write!(f, "use of possibly uninitialized value `{}`", name)
            }
            BorrowError::SecondMutableBorrow { name, span: _ } => {
                write!(
                    f,
                    "cannot borrow `{}` as mutable more than once at a time",
                    name
                )
            }
            BorrowError::BorrowWhileMutablyBorrowed { name, span: _ } => {
                write!(
                    f,
                    "cannot borrow `{}` as immutable because it is also borrowed as mutable",
                    name
                )
            }
            BorrowError::PartialMoveInBranch { name, span: _ } => {
                write!(
                    f,
                    "value `{}` is moved in one branch but not the other",
                    name
                )
            }
        }
    }
}

impl std::error::Error for BorrowError {}

impl Default for BorrowChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_states() {
        let mut checker = BorrowChecker::new();

        checker.record_owned(
            "x".to_string(),
            "i32".to_string(),
            OwnershipKind::Affine,
            Span::dummy(),
        );

        assert_eq!(checker.owned.get("x").unwrap().state, OwnershipState::Owned);
    }

    #[test]
    fn test_move_tracking() {
        let mut checker = BorrowChecker::new();

        checker.record_owned(
            "x".to_string(),
            "String".to_string(),
            OwnershipKind::Affine,
            Span::dummy(),
        );

        let result = checker.handle_move_or_copy("x", "y", Span::dummy());
        assert!(result.is_ok());
        assert_eq!(checker.owned.get("x").unwrap().state, OwnershipState::Moved);
    }

    #[test]
    fn test_copy_type() {
        let mut checker = BorrowChecker::new();

        checker.record_owned(
            "x".to_string(),
            "i32".to_string(),
            OwnershipKind::Copy,
            Span::dummy(),
        );

        let result = checker.handle_move_or_copy("x", "y", Span::dummy());
        assert!(result.is_ok());
        // Copy types stay owned
        assert_eq!(checker.owned.get("x").unwrap().state, OwnershipState::Owned);
    }

    #[test]
    fn test_borrow_rules() {
        let mut checker = BorrowChecker::new();

        checker.record_owned(
            "x".to_string(),
            "String".to_string(),
            OwnershipKind::Affine,
            Span::dummy(),
        );

        // First immutable borrow
        let result = checker.record_borrow("ref1".to_string(), "x", false, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_linear_consumption() {
        let mut checker = BorrowChecker::new();

        checker.record_owned(
            "resource".to_string(),
            "FileHandle".to_string(),
            OwnershipKind::Linear,
            Span::dummy(),
        );

        // Consume the linear value
        let result = checker.consume_linear("resource");
        assert!(result.is_ok());
        assert_eq!(
            checker.owned.get("resource").unwrap().state,
            OwnershipState::Consumed
        );

        // Double consume should fail
        let result2 = checker.consume_linear("resource");
        assert!(result2.is_err());
    }

    #[test]
    fn test_linear_must_be_consumed() {
        let mut checker = BorrowChecker::new();

        checker.record_owned(
            "resource".to_string(),
            "FileHandle".to_string(),
            OwnershipKind::Linear,
            Span::dummy(),
        );

        // Linear value not consumed - should error
        let result = checker.check_linear_consumed();
        assert!(result.is_err());
    }
}
