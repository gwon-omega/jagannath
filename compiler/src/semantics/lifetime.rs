//! Lifetime Checker - Āyus (Lifespan) System
//!
//! Implements lifetime analysis for references based on:
//! - Region-based memory management (Tofte & Talpin)
//! - Rust's lifetime system
//! - Sanskrit sūtra (thread) suffix ^N for lifetime regions
//!
//! Key concepts mapped to Sanskrit:
//! - आयुस् (Āyus) - Lifetime/lifespan
//! - क्षेत्र (Kṣetra) - Region
//! - सूत्र (Sūtra) - Thread (lifetime parameter)
//! - बन्ध (Bandha) - Binding/constraint
//!
//! Reference: Tofte & Talpin (1997) "Region-Based Memory Management"
//! Reference: Rust Reference on Lifetimes

use crate::lexer::Span;
use crate::parser::ast::*;
use std::collections::HashMap;

/// Lifetime checker - validates reference lifetimes
pub struct LifetimeChecker {
    /// Lifetime regions (id -> info)
    regions: HashMap<u8, RegionInfo>,
    /// Region stack (inner regions have shorter lifetimes)
    region_stack: Vec<u8>,
    /// Next region ID
    next_region: u8,
    /// Reference tracking (name -> lifetime info)
    references: HashMap<String, ReferenceInfo>,
    /// Lifetime constraints
    constraints: Vec<LifetimeConstraint>,
    /// Errors accumulated during checking
    errors: Vec<LifetimeError>,
    /// Current function context
    function_context: Option<FunctionLifetimeContext>,
}

/// Information about a lifetime region
#[derive(Debug, Clone)]
pub struct RegionInfo {
    /// Region ID
    pub id: u8,
    /// Parent region (if any)
    pub parent: Option<u8>,
    /// Depth in region hierarchy
    pub depth: usize,
    /// Source location where region started
    pub span: Span,
    /// Is this a function parameter region?
    pub is_param: bool,
    /// Named lifetime (e.g., 'a)
    pub name: Option<String>,
}

/// Information about a reference
#[derive(Debug, Clone)]
pub struct ReferenceInfo {
    /// Name of the reference
    pub name: String,
    /// Lifetime region
    pub region: u8,
    /// What is being referenced
    pub referent: String,
    /// Referent's region
    pub referent_region: u8,
    /// Is this a mutable reference?
    pub mutable: bool,
    /// Source location
    pub span: Span,
}

/// Lifetime constraint between regions
#[derive(Debug, Clone)]
pub struct LifetimeConstraint {
    /// Region that must outlive
    pub longer: u8,
    /// Region that must not outlive
    pub shorter: u8,
    /// Reason for constraint
    pub reason: ConstraintReason,
    /// Source location
    pub span: Span,
}

/// Reason for a lifetime constraint
#[derive(Debug, Clone)]
pub enum ConstraintReason {
    /// Reference must not outlive referent
    ReferenceOutlives,
    /// Return value must outlive function
    ReturnOutlives,
    /// Struct field must not outlive struct
    FieldOutlives,
    /// Borrow must not outlive owner
    BorrowOutlives,
    /// Explicit annotation
    Annotated,
}

/// Function lifetime context
#[derive(Debug, Clone)]
struct FunctionLifetimeContext {
    /// Function name
    name: String,
    /// Named lifetime parameters
    lifetime_params: HashMap<String, u8>,
    /// Return lifetime
    return_lifetime: Option<u8>,
    /// Parameter lifetimes
    param_lifetimes: HashMap<String, u8>,
}

impl LifetimeChecker {
    /// Create a new lifetime checker
    pub fn new() -> Self {
        Self {
            regions: HashMap::new(),
            region_stack: vec![0], // Static region
            next_region: 1,
            references: HashMap::new(),
            constraints: Vec::new(),
            errors: Vec::new(),
            function_context: None,
        }
    }

    /// Initialize with static region
    fn init(&mut self) {
        // Region 0 is 'static - lives forever
        self.regions.insert(
            0,
            RegionInfo {
                id: 0,
                parent: None,
                depth: 0,
                span: Span::dummy(),
                is_param: false,
                name: Some("'static".to_string()),
            },
        );
    }

    /// Check lifetimes in a function
    pub fn check_function(&mut self, func: &FunctionDef) -> Result<(), Vec<LifetimeError>> {
        // Reset state
        self.regions.clear();
        self.region_stack.clear();
        self.references.clear();
        self.constraints.clear();
        self.errors.clear();
        self.next_region = 1;

        // Initialize
        self.init();

        // Create function-level region
        let func_region = self.create_region(Some(0), func.span.clone(), false);
        self.region_stack.push(func_region);

        // Set up function context
        let lifetime_params = HashMap::new();
        let mut param_lifetimes = HashMap::new();

        // Process lifetime parameters from function definition
        // In Jagannath, lifetimes are encoded as ^N suffix on types
        for param in &func.params {
            let lifetime = self.extract_lifetime(&param.ty);
            if let Some(region_id) = lifetime {
                param_lifetimes.insert(param.name.name.clone(), region_id);
            } else {
                // Create implicit lifetime for parameter
                let region = self.create_region(Some(func_region), param.span.clone(), true);
                param_lifetimes.insert(param.name.name.clone(), region);
            }
        }

        // Process return type lifetime
        let return_lifetime = func
            .return_type
            .as_ref()
            .and_then(|t| self.extract_lifetime(t));

        self.function_context = Some(FunctionLifetimeContext {
            name: func.name.name.clone(),
            lifetime_params,
            return_lifetime,
            param_lifetimes,
        });

        // Check function body
        self.check_block(&func.body)?;

        // Validate all constraints
        self.validate_constraints()?;

        // Pop function region
        self.region_stack.pop();

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Check a block of statements
    fn check_block(&mut self, block: &Block) -> Result<(), Vec<LifetimeError>> {
        // Create new region for block
        let parent = *self.region_stack.last().unwrap_or(&0);
        let block_region = self.create_region(Some(parent), block.span.clone(), false);
        self.region_stack.push(block_region);

        for stmt in &block.stmts {
            self.check_stmt(stmt)?;
        }

        // Pop block region
        self.region_stack.pop();
        Ok(())
    }

    /// Check a statement
    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), Vec<LifetimeError>> {
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
                span: _,
            } => {
                self.check_expr(condition)?;
                self.check_block(then_block)?;
                if let Some(else_block) = else_block {
                    self.check_block(else_block)?;
                }
            }
            Stmt::Match {
                scrutinee,
                arms,
                span,
            } => {
                self.check_match(scrutinee, arms, span.clone())?;
            }
            Stmt::Loop {
                kind,
                body,
                span: _,
            } => {
                self.check_loop(kind, body)?;
            }
            Stmt::Break { span: _ } => {}
            Stmt::Continue { span: _ } => {}
        }
        Ok(())
    }

    /// Check let binding for lifetime issues
    fn check_let(
        &mut self,
        name: &Identifier,
        ty: Option<&Type>,
        value: Option<&Expr>,
        span: Span,
    ) -> Result<(), Vec<LifetimeError>> {
        // Get lifetime from type annotation
        let declared_lifetime = ty.and_then(|t| self.extract_lifetime(t));

        // Get current region
        let current_region = *self.region_stack.last().unwrap_or(&0);

        if let Some(value) = value {
            // Check if value is a reference
            if let Expr::Unary {
                op: UnaryOp::Ref,
                operand,
                span: ref_span,
            } = value
            {
                if let Expr::Identifier(referent) = operand.as_ref() {
                    // Creating a reference - check lifetimes
                    let referent_region = self
                        .get_value_region(&referent.name)
                        .unwrap_or(current_region);

                    let ref_region = declared_lifetime.unwrap_or(current_region);

                    // Constraint: reference cannot outlive referent
                    self.add_constraint(
                        referent_region,
                        ref_region,
                        ConstraintReason::ReferenceOutlives,
                        ref_span.clone(),
                    );

                    // Record the reference
                    self.references.insert(
                        name.name.clone(),
                        ReferenceInfo {
                            name: name.name.clone(),
                            region: ref_region,
                            referent: referent.name.clone(),
                            referent_region,
                            mutable: false,
                            span,
                        },
                    );
                }
            } else {
                // Check expression
                self.check_expr(value)?;
            }
        }

        Ok(())
    }

    /// Check expression for lifetime issues
    fn check_expr(&mut self, expr: &Expr) -> Result<Option<u8>, Vec<LifetimeError>> {
        match expr {
            Expr::Literal(_) => Ok(Some(0)), // Literals have 'static lifetime

            Expr::Identifier(id) => {
                // Get the lifetime of this identifier
                Ok(self.get_value_region(&id.name))
            }

            Expr::Binary {
                left,
                op: _,
                right,
                span: _,
            } => {
                self.check_expr(left)?;
                self.check_expr(right)?;
                Ok(None)
            }

            Expr::Unary { op, operand, span } => {
                match op {
                    UnaryOp::Ref => {
                        // Taking a reference
                        if let Expr::Identifier(id) = operand.as_ref() {
                            let referent_region = self
                                .get_value_region(&id.name)
                                .unwrap_or(*self.region_stack.last().unwrap_or(&0));

                            // Reference has same region as current scope
                            let ref_region = *self.region_stack.last().unwrap_or(&0);

                            // Constraint: referent must outlive reference
                            self.add_constraint(
                                referent_region,
                                ref_region,
                                ConstraintReason::ReferenceOutlives,
                                span.clone(),
                            );

                            Ok(Some(ref_region))
                        } else {
                            self.check_expr(operand)
                        }
                    }
                    UnaryOp::Deref => {
                        // Dereferencing - must be valid
                        self.check_expr(operand)
                    }
                    _ => self.check_expr(operand),
                }
            }

            Expr::Call {
                callee,
                args,
                span: _,
            } => {
                self.check_expr(callee)?;
                for arg in args {
                    self.check_expr(arg)?;
                }
                Ok(None)
            }

            Expr::MethodCall {
                receiver,
                method: _,
                args,
                span: _,
            } => {
                self.check_expr(receiver)?;
                for arg in args {
                    self.check_expr(arg)?;
                }
                Ok(None)
            }

            Expr::FieldAccess {
                object,
                field: _,
                span: _,
            } => self.check_expr(object),

            Expr::Index {
                object,
                index,
                span: _,
            } => {
                self.check_expr(object)?;
                self.check_expr(index)?;
                Ok(None)
            }

            Expr::StructConstruct {
                name: _,
                fields,
                span: _,
            } => {
                for (_, field_expr) in fields {
                    self.check_expr(field_expr)?;
                }
                Ok(None)
            }

            Expr::Array { elements, span: _ } => {
                for elem in elements {
                    self.check_expr(elem)?;
                }
                Ok(None)
            }

            Expr::Tuple { elements, span: _ } => {
                for elem in elements {
                    self.check_expr(elem)?;
                }
                Ok(None)
            }

            Expr::Lambda {
                params,
                body,
                span: _,
            } => {
                // Lambda creates new region
                let parent = *self.region_stack.last().unwrap_or(&0);
                let lambda_region = self.create_region(Some(parent), Span::dummy(), false);
                self.region_stack.push(lambda_region);

                // Process parameters (may be captured from outer scope)
                for _param in params {
                    // Lambda params live in lambda region
                }

                // Check body expression
                self.check_expr(body)?;

                self.region_stack.pop();
                Ok(Some(parent))
            }

            Expr::If {
                condition,
                then_expr,
                else_expr,
                span: _,
            } => {
                self.check_expr(condition)?;
                let then_lifetime = self.check_expr(then_expr)?;

                if let Some(else_br) = else_expr {
                    let else_lifetime = self.check_expr(else_br)?;
                    // Both branches must have compatible lifetimes
                    Ok(self.intersect_lifetimes(then_lifetime, else_lifetime))
                } else {
                    Ok(then_lifetime)
                }
            }

            Expr::Block(block) => {
                self.check_block(block)?;
                Ok(None)
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

    /// Check return statement
    fn check_return(&mut self, value: Option<&Expr>, span: Span) -> Result<(), Vec<LifetimeError>> {
        if let Some(value) = value {
            let expr_lifetime = self.check_expr(value)?;

            // Check if returning a reference
            if let Expr::Unary {
                op: UnaryOp::Ref,
                operand,
                ..
            } = value
            {
                if let Expr::Identifier(id) = operand.as_ref() {
                    // Check if referent is local
                    let referent_region = self.get_value_region(&id.name);
                    let func_region = self.region_stack.get(1).copied().unwrap_or(0);

                    if let Some(region) = referent_region {
                        // If referent's region is inside function, it's dangling
                        if self.is_region_inside(region, func_region) {
                            self.errors.push(LifetimeError::ReturnLocalReference {
                                name: id.name.clone(),
                                span,
                            });
                        }
                    }
                }
            }

            // If function has return lifetime, check constraint
            if let Some(ref ctx) = self.function_context {
                if let Some(return_lt) = ctx.return_lifetime {
                    if let Some(expr_lt) = expr_lifetime {
                        self.add_constraint(
                            expr_lt,
                            return_lt,
                            ConstraintReason::ReturnOutlives,
                            span,
                        );
                    }
                }
            }
        }

        Ok(())
    }

    /// Check match expression
    fn check_match(
        &mut self,
        scrutinee: &Expr,
        arms: &[MatchArm],
        _span: Span,
    ) -> Result<(), Vec<LifetimeError>> {
        self.check_expr(scrutinee)?;

        for arm in arms {
            // Each arm gets its own region
            let parent = *self.region_stack.last().unwrap_or(&0);
            let arm_region = self.create_region(Some(parent), Span::dummy(), false);
            self.region_stack.push(arm_region);

            // Check pattern bindings
            self.check_pattern(&arm.pattern)?;

            // Check body
            self.check_expr(&arm.body)?;

            self.region_stack.pop();
        }

        Ok(())
    }

    /// Check loop
    fn check_loop(&mut self, kind: &LoopKind, body: &Block) -> Result<(), Vec<LifetimeError>> {
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

        self.check_block(body)
    }

    /// Check pattern
    fn check_pattern(&mut self, pattern: &Pattern) -> Result<(), Vec<LifetimeError>> {
        match pattern {
            Pattern::Identifier(_) => Ok(()),
            Pattern::Constructor { name: _, fields } => {
                for p in fields {
                    self.check_pattern(p)?;
                }
                Ok(())
            }
            Pattern::Binding {
                name: _,
                mutable: _,
                subpattern,
            } => {
                if let Some(sub) = subpattern {
                    self.check_pattern(sub)?;
                }
                Ok(())
            }
            Pattern::Tuple(patterns) => {
                for p in patterns {
                    self.check_pattern(p)?;
                }
                Ok(())
            }
            Pattern::Struct {
                name: _,
                fields,
                rest: _,
            } => {
                for (_, p) in fields {
                    self.check_pattern(p)?;
                }
                Ok(())
            }
            Pattern::Variant {
                enum_name: _,
                variant: _,
                fields,
            } => match fields {
                crate::parser::ast::VariantFields::Unit => Ok(()),
                crate::parser::ast::VariantFields::Tuple(patterns) => {
                    for p in patterns {
                        self.check_pattern(p)?;
                    }
                    Ok(())
                }
                crate::parser::ast::VariantFields::Struct(field_patterns) => {
                    for (_, p) in field_patterns {
                        self.check_pattern(p)?;
                    }
                    Ok(())
                }
            },
            Pattern::Array(patterns) => {
                for p in patterns {
                    self.check_pattern(p)?;
                }
                Ok(())
            }
            Pattern::Slice {
                before,
                middle,
                after,
            } => {
                for p in before {
                    self.check_pattern(p)?;
                }
                if let Some(mid) = middle {
                    self.check_pattern(mid)?;
                }
                for p in after {
                    self.check_pattern(p)?;
                }
                Ok(())
            }
            Pattern::Range { start, end, .. } => {
                if let Some(p) = start {
                    self.check_pattern(p)?;
                }
                if let Some(p) = end {
                    self.check_pattern(p)?;
                }
                Ok(())
            }
            Pattern::Or(patterns) => {
                for p in patterns {
                    self.check_pattern(p)?;
                }
                Ok(())
            }
            Pattern::Guard {
                pattern,
                condition: _,
            } => self.check_pattern(pattern),
            Pattern::Ref {
                mutable: _,
                pattern: inner,
            } => self.check_pattern(inner),
            Pattern::Literal(_) => Ok(()),
            Pattern::Wildcard => Ok(()),
            Pattern::Rest => Ok(()),
        }
    }

    // ========== Helper Methods ==========

    /// Create a new region
    fn create_region(&mut self, parent: Option<u8>, span: Span, is_param: bool) -> u8 {
        let id = self.next_region;
        self.next_region += 1;

        let depth = parent
            .map(|p| self.regions.get(&p).map(|r| r.depth + 1).unwrap_or(1))
            .unwrap_or(1);

        self.regions.insert(
            id,
            RegionInfo {
                id,
                parent,
                depth,
                span,
                is_param,
                name: None,
            },
        );

        id
    }

    /// Extract lifetime from type (^N suffix)
    fn extract_lifetime(&self, ty: &Type) -> Option<u8> {
        match ty {
            Type::Named { affixes, .. } => affixes.lifetime_region(),
            Type::Reference { lifetime, .. } => *lifetime,
            _ => None,
        }
    }

    /// Get region for a value
    fn get_value_region(&self, name: &str) -> Option<u8> {
        // Check references first
        if let Some(ref_info) = self.references.get(name) {
            return Some(ref_info.region);
        }

        // Check function parameters
        if let Some(ref ctx) = self.function_context {
            if let Some(&region) = ctx.param_lifetimes.get(name) {
                return Some(region);
            }
        }

        // Default to current scope
        self.region_stack.last().copied()
    }

    /// Add a lifetime constraint
    fn add_constraint(&mut self, longer: u8, shorter: u8, reason: ConstraintReason, span: Span) {
        self.constraints.push(LifetimeConstraint {
            longer,
            shorter,
            reason,
            span,
        });
    }

    /// Validate all constraints
    fn validate_constraints(&mut self) -> Result<(), Vec<LifetimeError>> {
        for constraint in &self.constraints {
            if !self.outlives(constraint.longer, constraint.shorter) {
                self.errors.push(LifetimeError::LifetimeTooShort {
                    longer: constraint.longer,
                    shorter: constraint.shorter,
                    reason: constraint.reason.clone(),
                    span: constraint.span.clone(),
                });
            }
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Check if region `a` outlives region `b`
    pub fn outlives(&self, a: u8, b: u8) -> bool {
        if a == b {
            return true;
        }

        // 'static outlives everything
        if a == 0 {
            return true;
        }

        // Check parent chain
        let a_info = self.regions.get(&a);
        let b_info = self.regions.get(&b);

        match (a_info, b_info) {
            (Some(a), Some(b)) => {
                // `a` outlives `b` if `a` is an ancestor of `b`
                // or if `a` has smaller depth (outer scope)
                if a.depth < b.depth {
                    return true;
                }

                // Check if `a` is in the parent chain of `b`
                let mut current = Some(b.id);
                while let Some(id) = current {
                    if id == a.id {
                        return true;
                    }
                    current = self.regions.get(&id).and_then(|r| r.parent);
                }

                false
            }
            _ => false,
        }
    }

    /// Check if region `inner` is inside region `outer`
    fn is_region_inside(&self, inner: u8, outer: u8) -> bool {
        if inner == outer {
            return true;
        }

        let mut current = Some(inner);
        while let Some(id) = current {
            if id == outer {
                return true;
            }
            current = self.regions.get(&id).and_then(|r| r.parent);
        }

        false
    }

    /// Find the intersection (shorter) of two lifetimes
    fn intersect_lifetimes(&self, a: Option<u8>, b: Option<u8>) -> Option<u8> {
        match (a, b) {
            (Some(a), Some(b)) => {
                if self.outlives(a, b) {
                    Some(b) // b is shorter
                } else {
                    Some(a) // a is shorter (or incompatible)
                }
            }
            (Some(x), None) | (None, Some(x)) => Some(x),
            (None, None) => None,
        }
    }

    /// Get accumulated errors
    pub fn errors(&self) -> &[LifetimeError] {
        &self.errors
    }
}

/// Lifetime error
#[derive(Debug, Clone)]
pub enum LifetimeError {
    /// Lifetime is too short for the constraint
    LifetimeTooShort {
        longer: u8,
        shorter: u8,
        reason: ConstraintReason,
        span: Span,
    },
    /// Returning reference to local variable
    ReturnLocalReference { name: String, span: Span },
    /// Reference outlives its referent
    ReferenceOutlivesReferent {
        reference: String,
        referent: String,
        span: Span,
    },
    /// Borrowed value does not live long enough
    BorrowedValueTooShort { name: String, span: Span },
    /// Lifetime parameter not found
    UnknownLifetime(String),
    /// Conflicting lifetimes
    ConflictingLifetimes { span: Span },
}

impl std::fmt::Display for LifetimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LifetimeError::LifetimeTooShort {
                longer,
                shorter,
                reason,
                span: _,
            } => {
                write!(
                    f,
                    "lifetime '^{}' does not live long enough (required to outlive '^{}')",
                    shorter, longer
                )?;
                match reason {
                    ConstraintReason::ReferenceOutlives => {
                        write!(f, " because of reference")?;
                    }
                    ConstraintReason::ReturnOutlives => {
                        write!(f, " because of return type")?;
                    }
                    ConstraintReason::FieldOutlives => {
                        write!(f, " because of struct field")?;
                    }
                    ConstraintReason::BorrowOutlives => {
                        write!(f, " because of borrow")?;
                    }
                    ConstraintReason::Annotated => {
                        write!(f, " because of explicit annotation")?;
                    }
                }
                Ok(())
            }
            LifetimeError::ReturnLocalReference { name, span: _ } => {
                write!(f, "cannot return reference to local variable `{}`", name)
            }
            LifetimeError::ReferenceOutlivesReferent {
                reference,
                referent,
                span: _,
            } => {
                write!(
                    f,
                    "reference `{}` outlives its referent `{}`",
                    reference, referent
                )
            }
            LifetimeError::BorrowedValueTooShort { name, span: _ } => {
                write!(f, "borrowed value `{}` does not live long enough", name)
            }
            LifetimeError::UnknownLifetime(name) => {
                write!(f, "unknown lifetime parameter '{}'", name)
            }
            LifetimeError::ConflictingLifetimes { span: _ } => {
                write!(f, "conflicting lifetime requirements")
            }
        }
    }
}

impl std::error::Error for LifetimeError {}

impl Default for LifetimeChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_creation() {
        let mut checker = LifetimeChecker::new();
        checker.init();

        let r1 = checker.create_region(Some(0), Span::dummy(), false);
        let r2 = checker.create_region(Some(r1), Span::dummy(), false);

        assert!(checker.outlives(0, r1)); // static outlives r1
        assert!(checker.outlives(0, r2)); // static outlives r2
        assert!(checker.outlives(r1, r2)); // r1 outlives r2
        assert!(!checker.outlives(r2, r1)); // r2 does not outlive r1
    }

    #[test]
    fn test_static_outlives_all() {
        let mut checker = LifetimeChecker::new();
        checker.init();

        let r1 = checker.create_region(Some(0), Span::dummy(), false);
        let r2 = checker.create_region(Some(r1), Span::dummy(), false);
        let r3 = checker.create_region(Some(r2), Span::dummy(), false);

        assert!(checker.outlives(0, r1));
        assert!(checker.outlives(0, r2));
        assert!(checker.outlives(0, r3));
    }

    #[test]
    fn test_nested_regions() {
        let mut checker = LifetimeChecker::new();
        checker.init();

        let outer = checker.create_region(Some(0), Span::dummy(), false);
        let inner = checker.create_region(Some(outer), Span::dummy(), false);

        // Outer outlives inner
        assert!(checker.outlives(outer, inner));

        // Inner does not outlive outer
        assert!(!checker.outlives(inner, outer));
    }

    #[test]
    fn test_constraint_validation() {
        let mut checker = LifetimeChecker::new();
        checker.init();

        let outer = checker.create_region(Some(0), Span::dummy(), false);
        let inner = checker.create_region(Some(outer), Span::dummy(), false);

        // Valid constraint: inner reference in outer scope
        checker.add_constraint(
            outer,
            inner,
            ConstraintReason::ReferenceOutlives,
            Span::dummy(),
        );

        let result = checker.validate_constraints();
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_constraint() {
        let mut checker = LifetimeChecker::new();
        checker.init();

        let outer = checker.create_region(Some(0), Span::dummy(), false);
        let inner = checker.create_region(Some(outer), Span::dummy(), false);

        // Invalid constraint: outer reference must outlive inner (but inner is shorter)
        checker.add_constraint(
            inner,
            outer,
            ConstraintReason::ReferenceOutlives,
            Span::dummy(),
        );

        let result = checker.validate_constraints();
        assert!(result.is_err());
    }

    #[test]
    fn test_self_outlives() {
        let mut checker = LifetimeChecker::new();
        checker.init();

        let r1 = checker.create_region(Some(0), Span::dummy(), false);

        assert!(checker.outlives(r1, r1)); // Region outlives itself
    }
}
