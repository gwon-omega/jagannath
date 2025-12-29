//! Unified Type Checker (Samgraha Prakāra Parīkṣaka)
//!
//! This module integrates all type checking subsystems into a cohesive
//! whole, following the Sāṃkhya principle of emergence (Pariṇāma):
//!
//! ## Architectural Philosophy
//!
//! **Sāṃkhya Emergent Hierarchy:**
//! Just as Prakṛti evolves through 25 tattvas to manifest the phenomenal
//! world, the type checker evolves from raw source through stages:
//!
//! ```text
//! Source (Prakṛti) → Types (Buddhi) → Constraints (Ahaṃkāra) → Solution (Puruṣa)
//! ```
//!
//! **Component Integration:**
//! - `Context` (Saṃdarbha): Lexical environment - tracks what we know
//! - `Constraints` (Niyama): What must be proven - logical conditions
//! - `Inference` (Anumāna): How we derive knowledge - unification
//! - `Generics` (Sāmānya): Universal forms - polymorphism
//! - `Lifetimes` (Āyus): Temporal qualities - region analysis
//!
//! ## Integration Points
//!
//! ```text
//!                    ┌─────────────────┐
//!                    │  UnifiedChecker │
//!                    │   (Samgraha)    │
//!                    └────────┬────────┘
//!                             │
//!       ┌──────────┬─────────┼─────────┬──────────┐
//!       │          │         │         │          │
//!       ▼          ▼         ▼         ▼          ▼
//!   Context    Inference  Constraints  Generics  Lifetimes
//!  (Scopes)   (Unify)    (Solve)     (∀/∃)    (Regions)
//! ```

use super::constraints::{Constraint, ConstraintKind, ConstraintReason};
use super::context::{ScopeKind, TypeContext};
use super::errors::TypeError;
use super::generics::{PolymorphismEngine, TypeScheme};
use super::inference::{TypeInference, UnificationError};
use super::lifetimes::{LifetimeInference, OutlivesReason, RegionVar};
use super::pramana::Pramana;
use super::types::{FunctionSig, MethodSig, ResolvedType, SelfType, TypeInfo};
use crate::lexer::Span;

// ============================================================================
// Unified Checker State (Samgraha Avasthā)
// ============================================================================

/// Unified type checker that orchestrates all subsystems
///
/// Named Samgraha (संग्रह) meaning "collection" or "synthesis" in Sanskrit.
/// It gathers all aspects of type checking into coherent judgments.
///
/// ## Design Principles
///
/// 1. **Single Source of Truth**: All type information flows through here
/// 2. **Lazy Constraint Collection**: Constraints gathered during traversal
/// 3. **Batched Solving**: Constraints solved at natural boundaries
/// 4. **Error Recovery**: Continue checking after errors when possible
pub struct UnifiedChecker {
    /// Type context with scopes and registries
    context: TypeContext,
    /// Type inference engine (unification)
    inference: TypeInference,
    /// Constraint queue
    constraints: Vec<Constraint>,
    /// Polymorphism engine
    generics: PolymorphismEngine,
    /// Lifetime inference
    lifetimes: LifetimeInference,
    /// Collected errors
    errors: Vec<TypeError>,
    /// Configuration
    config: CheckerConfig,
}

/// Configuration for the unified checker
#[derive(Debug, Clone)]
pub struct CheckerConfig {
    /// Maximum iterations for constraint solving
    pub max_iterations: usize,
    /// Enable strict lifetime checking
    pub strict_lifetimes: bool,
    /// Enable experimental features
    pub experimental: bool,
    /// Verbosity level for diagnostics
    pub verbosity: u8,
}

impl Default for CheckerConfig {
    fn default() -> Self {
        Self {
            max_iterations: 1000,
            strict_lifetimes: true,
            experimental: false,
            verbosity: 1,
        }
    }
}

impl UnifiedChecker {
    /// Create a new unified checker
    pub fn new() -> Self {
        Self::with_config(CheckerConfig::default())
    }

    /// Create with custom configuration
    pub fn with_config(config: CheckerConfig) -> Self {
        let mut checker = Self {
            context: TypeContext::new(),
            inference: TypeInference::new(),
            constraints: Vec::new(),
            generics: PolymorphismEngine::new(),
            lifetimes: LifetimeInference::new(),
            errors: Vec::new(),
            config,
        };
        checker.register_builtins();
        checker
    }

    // ========================================================================
    // Built-in Registration (Ādhāra Pañjīkaraṇa)
    // ========================================================================

    /// Register built-in types and functions
    fn register_builtins(&mut self) {
        // Register print function
        self.context.register_function(FunctionSig {
            name: "print".to_string(),
            params: vec![("value".to_string(), ResolvedType::String)],
            return_type: ResolvedType::Unit,
            span: None,
        });

        // Register println function
        self.context.register_function(FunctionSig {
            name: "println".to_string(),
            params: vec![("value".to_string(), ResolvedType::String)],
            return_type: ResolvedType::Unit,
            span: None,
        });

        // Register len function
        self.context.register_function(FunctionSig {
            name: "len".to_string(),
            params: vec![("value".to_string(), ResolvedType::String)],
            return_type: ResolvedType::UInt64,
            span: None,
        });

        // Register string methods
        let string_methods: Vec<(&str, Vec<(&str, ResolvedType)>, ResolvedType)> = vec![
            ("len", vec![], ResolvedType::UInt64),
            ("is_empty", vec![], ResolvedType::Bool),
            (
                "chars",
                vec![],
                ResolvedType::Array {
                    element: Box::new(ResolvedType::Char),
                    size: None,
                },
            ),
        ];

        for (name, params, ret) in string_methods {
            self.context.register_method(
                "String".to_string(),
                MethodSig {
                    name: name.to_string(),
                    self_type: SelfType::Ref,
                    params: params
                        .into_iter()
                        .map(|(n, t)| (n.to_string(), t))
                        .collect(),
                    return_type: ret,
                    span: None,
                },
            );
        }
    }

    // ========================================================================
    // Scope Management (Viṣaya Prabandha)
    // ========================================================================

    /// Enter a new scope
    pub fn enter_scope(&mut self, kind: ScopeKind) {
        self.context.enter_scope(kind);
    }

    /// Exit current scope
    pub fn exit_scope(&mut self) {
        self.context.exit_scope();
    }

    /// Add a symbol binding
    pub fn bind(&mut self, name: &str, ty: ResolvedType, pramana: Pramana, span: Option<Span>) {
        self.context
            .add_symbol_with_pramana(name.to_string(), ty, pramana, span);
    }

    /// Look up a symbol
    pub fn lookup(&self, name: &str) -> Option<&TypeInfo> {
        self.context.lookup_symbol(name)
    }

    // ========================================================================
    // Type Variable Management (Prakāra Cāla)
    // ========================================================================

    /// Create a fresh type variable
    pub fn fresh_type_var(&mut self) -> ResolvedType {
        self.inference.fresh_type_var()
    }

    /// Create a fresh region variable
    pub fn fresh_region(&mut self) -> RegionVar {
        self.lifetimes.fresh_region()
    }

    // ========================================================================
    // Constraint Generation (Niyama Utpatti)
    // ========================================================================

    /// Add an equality constraint
    pub fn constrain_equal(
        &mut self,
        lhs: ResolvedType,
        rhs: ResolvedType,
        reason: ConstraintReason,
        span: Option<Span>,
    ) {
        self.constraints.push(Constraint {
            kind: ConstraintKind::Equality { lhs, rhs },
            reason,
            span,
            priority: 10,
        });
    }

    /// Add a field access constraint
    pub fn constrain_field(
        &mut self,
        receiver: ResolvedType,
        field: String,
        field_type: ResolvedType,
        span: Option<Span>,
    ) {
        self.constraints.push(Constraint {
            kind: ConstraintKind::FieldAccess {
                receiver,
                field: field.clone(),
                field_type,
            },
            reason: ConstraintReason::FieldAccess { field },
            span,
            priority: 8,
        });
    }

    /// Add a function call constraint
    pub fn constrain_call(
        &mut self,
        callee: ResolvedType,
        args: Vec<ResolvedType>,
        result: ResolvedType,
        span: Option<Span>,
    ) {
        self.constraints.push(Constraint {
            kind: ConstraintKind::FunctionCall {
                callee,
                args,
                result,
            },
            reason: ConstraintReason::Custom("function call".to_string()),
            span,
            priority: 9,
        });
    }

    // ========================================================================
    // Trait Constraints (Dharma Niyama)
    // ========================================================================

    /// Add a trait bound constraint (Dharma requirement)
    ///
    /// In Sanskrit philosophy, Dharma represents duty and righteous conduct.
    /// A trait bound is a Dharma that a type must fulfill to be valid.
    ///
    /// ```text
    /// T : Trait  ⟹  Type T must implement Trait (Dharma)
    /// ```
    pub fn constrain_trait(
        &mut self,
        ty: ResolvedType,
        trait_name: &str,
        span: Option<Span>,
    ) {
        self.constraints.push(Constraint {
            kind: ConstraintKind::TraitBound {
                ty,
                trait_name: trait_name.to_string(),
                trait_params: vec![],
            },
            reason: ConstraintReason::TraitBound {
                trait_name: trait_name.to_string(),
            },
            span,
            priority: 7, // Lower than equality but still important
        });
    }

    /// Add a trait bound with generic parameters
    pub fn constrain_trait_with_params(
        &mut self,
        ty: ResolvedType,
        trait_name: &str,
        params: Vec<ResolvedType>,
        span: Option<Span>,
    ) {
        self.constraints.push(Constraint {
            kind: ConstraintKind::TraitBound {
                ty,
                trait_name: trait_name.to_string(),
                trait_params: params,
            },
            reason: ConstraintReason::TraitBound {
                trait_name: trait_name.to_string(),
            },
            span,
            priority: 7,
        });
    }

    /// Add an associated type constraint
    ///
    /// ```text
    /// <T as Trait>::AssocType = U
    /// ```
    pub fn constrain_associated_type(
        &mut self,
        impl_type: ResolvedType,
        trait_name: &str,
        assoc_name: &str,
        expected_type: ResolvedType,
        span: Option<Span>,
    ) {
        self.constraints.push(Constraint {
            kind: ConstraintKind::AssociatedType {
                impl_type,
                trait_name: trait_name.to_string(),
                assoc_name: assoc_name.to_string(),
                expected_type,
            },
            reason: ConstraintReason::AssociatedType {
                trait_name: trait_name.to_string(),
                assoc_name: assoc_name.to_string(),
            },
            span,
            priority: 6,
        });
    }

    /// Check if a type implements a trait (immediate check, not constraint)
    pub fn implements_trait(&self, ty: &ResolvedType, trait_name: &str) -> bool {
        let resolved = self.apply(ty);
        self.check_trait_impl(&resolved, trait_name)
    }

    // ========================================================================
    // Unification (Ekīkaraṇa)
    // ========================================================================

    /// Unify two types
    pub fn unify(&mut self, t1: &ResolvedType, t2: &ResolvedType) -> Result<(), TypeError> {
        self.inference
            .unify(t1, t2)
            .map_err(|e| self.unification_error_to_type_error(e, None))
    }

    /// Unify with span for error reporting
    pub fn unify_with_span(
        &mut self,
        t1: &ResolvedType,
        t2: &ResolvedType,
        span: Span,
    ) -> Result<(), TypeError> {
        self.inference
            .unify(t1, t2)
            .map_err(|e| self.unification_error_to_type_error(e, Some(span)))
    }

    /// Apply substitutions to a type
    pub fn apply(&self, ty: &ResolvedType) -> ResolvedType {
        self.inference.apply(ty)
    }

    /// Convert unification error to type error
    fn unification_error_to_type_error(
        &self,
        err: UnificationError,
        span: Option<Span>,
    ) -> TypeError {
        TypeError::UnificationFailed { error: err, span }
    }

    // ========================================================================
    // Generalization & Instantiation (Sāmānyīkaraṇa & Viśeṣīkaraṇa)
    // ========================================================================

    /// Generalize a type to a type scheme
    ///
    /// In Vaiśeṣika terms: Extract the sāmānya (universal) from
    /// the viśeṣa (particular) by quantifying over free type variables.
    pub fn generalize(&self, ty: &ResolvedType) -> TypeScheme {
        // For now, create a monomorphic scheme
        // TODO: Proper generalization with environment
        TypeScheme::mono(self.apply(ty))
    }

    /// Instantiate a type scheme with fresh variables
    ///
    /// In Vaiśeṣika terms: Derive a viśeṣa (particular) from
    /// the sāmānya (universal) through samavāya (inherence).
    pub fn instantiate(&mut self, scheme: &TypeScheme) -> ResolvedType {
        self.generics.instantiate(scheme)
    }

    // ========================================================================
    // Algorithm W (Hindley-Milner) - Vara Bahurupatā Gaṇita
    // ========================================================================

    /// Process a let-binding with polymorphism (Algorithm W - Let rule)
    ///
    /// The Let rule is the heart of Hindley-Milner's power:
    /// ```text
    /// Γ ⊢ e₁ : τ₁    Γ, x : Gen(Γ, τ₁) ⊢ e₂ : τ₂
    /// ─────────────────────────────────────────────
    ///            Γ ⊢ let x = e₁ in e₂ : τ₂
    /// ```
    ///
    /// This enables polymorphic reuse:
    /// ```text
    /// let id = λx. x in (id 5, id "hello")  // Works!
    /// ```
    ///
    /// ## Sanskrit Commentary (Vara Niyama)
    ///
    /// In Nyāya-Vaiśeṣika terms:
    /// - We infer the viśeṣa (particular) type of the expression
    /// - We extract the sāmānya (universal) by generalization
    /// - The samavāya (inherence) relation binds name to scheme
    pub fn process_let_binding(
        &mut self,
        name: &str,
        inferred_type: ResolvedType,
        span: Option<Span>,
    ) -> TypeScheme {
        // Apply current substitutions
        let ty = self.apply(&inferred_type);

        // Generalize: find type vars not constrained by environment
        let scheme = self.generalize_with_env(&ty);

        // Bind to context with type scheme info
        self.context.add_symbol_with_pramana(
            name.to_string(),
            ty.clone(),
            Pramana::Anumana,
            span,
        );

        // Store scheme for polymorphic use
        self.context.register_type_scheme(name.to_string(), scheme.clone());

        scheme
    }

    /// Generalize a type with respect to the current environment
    ///
    /// Finds free type variables in `ty` that are NOT free in the
    /// current environment, and quantifies over them.
    fn generalize_with_env(&self, ty: &ResolvedType) -> TypeScheme {
        use super::generics::free_type_vars;

        let ty_free = free_type_vars(ty);
        let env_free = self.collect_env_free_vars();

        // Quantify over variables free in ty but not in env
        let quantified: Vec<_> = ty_free
            .difference(&env_free)
            .copied()
            .collect();

        if quantified.is_empty() {
            TypeScheme::mono(ty.clone())
        } else {
            TypeScheme::poly(quantified, ty.clone())
        }
    }

    /// Collect free type variables from the current environment
    fn collect_env_free_vars(&self) -> std::collections::HashSet<super::types::TypeVar> {
        use super::generics::free_type_vars;
        use super::types::TypeVar;

        let mut vars = std::collections::HashSet::<TypeVar>::new();

        // Collect from all symbols in scope
        for scope in self.context.scopes() {
            for (_name, info) in scope.symbols() {
                vars.extend(free_type_vars(&info.ty));
            }
        }

        vars
    }

    /// Use a let-bound variable (Algorithm W - Var rule)
    ///
    /// When using a variable, we instantiate its type scheme:
    /// ```text
    /// x : σ ∈ Γ
    /// ────────────────
    /// Γ ⊢ x : Inst(σ)
    /// ```
    ///
    /// This allows polymorphic use at different types.
    pub fn use_variable(&mut self, name: &str) -> Option<ResolvedType> {
        // First check for type scheme
        if let Some(scheme) = self.context.lookup_type_scheme(name) {
            return Some(self.instantiate(&scheme.clone()));
        }

        // Fall back to direct lookup
        self.lookup(name).map(|info| info.ty.clone())
    }

    /// Infer function type (Algorithm W - Abs rule)
    ///
    /// For λx. e:
    /// ```text
    /// Γ, x : τ₁ ⊢ e : τ₂
    /// ───────────────────
    /// Γ ⊢ λx. e : τ₁ → τ₂
    /// ```
    pub fn infer_function(
        &mut self,
        params: &[(String, Option<ResolvedType>)],
        body_type: ResolvedType,
    ) -> ResolvedType {
        let param_types: Vec<ResolvedType> = params
            .iter()
            .map(|(_, ty_opt)| {
                ty_opt.clone().unwrap_or_else(|| self.fresh_type_var())
            })
            .collect();

        ResolvedType::Function {
            params: param_types,
            return_type: Box::new(body_type),
        }
    }

    /// Infer application type (Algorithm W - App rule)
    ///
    /// For e₁ e₂:
    /// ```text
    /// Γ ⊢ e₁ : τ₁    Γ ⊢ e₂ : τ₂    τ₁ ~ τ₂ → τ₃
    /// ─────────────────────────────────────────────
    ///                 Γ ⊢ e₁ e₂ : τ₃
    /// ```
    pub fn infer_application(
        &mut self,
        callee_type: ResolvedType,
        arg_types: Vec<ResolvedType>,
        span: Option<Span>,
    ) -> Result<ResolvedType, TypeError> {
        let result_type = self.fresh_type_var();

        let expected_fn = ResolvedType::Function {
            params: arg_types,
            return_type: Box::new(result_type.clone()),
        };

        // Unify callee with expected function type
        self.inference.unify(&callee_type, &expected_fn).map_err(|e| {
            TypeError::UnificationFailed {
                error: e,
                span,
            }
        })?;

        Ok(self.apply(&result_type))
    }

    /// Infer if-expression type (both branches must unify)
    ///
    /// ```text
    /// Γ ⊢ cond : Bool    Γ ⊢ then : τ    Γ ⊢ else : τ
    /// ──────────────────────────────────────────────────
    ///        Γ ⊢ if cond then else : τ
    /// ```
    pub fn infer_if_expression(
        &mut self,
        condition_type: ResolvedType,
        then_type: ResolvedType,
        else_type: ResolvedType,
        span: Option<Span>,
    ) -> Result<ResolvedType, TypeError> {
        // Condition must be Bool
        self.inference.unify(&condition_type, &ResolvedType::Bool).map_err(|e| {
            TypeError::UnificationFailed {
                error: e,
                span,
            }
        })?;

        // Both branches must have same type
        self.inference.unify(&then_type, &else_type).map_err(|e| {
            TypeError::UnificationFailed {
                error: e,
                span,
            }
        })?;

        Ok(self.apply(&then_type))
    }

    // ========================================================================
    // Lifetime Constraints (Āyus Niyama)
    // ========================================================================

    /// Add lifetime outlives constraint
    pub fn constrain_outlives(
        &mut self,
        longer: RegionVar,
        shorter: RegionVar,
        reason: OutlivesReason,
        span: Option<Span>,
    ) {
        self.lifetimes
            .add_outlives_simple(longer, shorter, reason, span);
    }

    /// Check if one region outlives another
    pub fn region_outlives(&self, longer: RegionVar, shorter: RegionVar) -> bool {
        self.lifetimes.outlives(longer, shorter)
    }

    // ========================================================================
    // Constraint Solving (Niyama Samādhāna)
    // ========================================================================

    /// Solve all accumulated constraints
    ///
    /// Uses iterative solving with the Nyāya Pañcāvayava framework:
    /// 1. Select highest priority unsolved constraint
    /// 2. Attempt to solve (establish Nigamana from Pratijñā)
    /// 3. If solved, remove and continue
    /// 4. If blocked, lower priority and try others
    /// 5. Repeat until all solved or no progress
    pub fn solve_constraints(&mut self) -> Result<(), Vec<TypeError>> {
        let mut iterations = 0;
        let mut progress = true;

        while progress && !self.constraints.is_empty() && iterations < self.config.max_iterations {
            progress = false;
            iterations += 1;

            // Sort by priority (highest first)
            self.constraints.sort_by(|a, b| b.priority.cmp(&a.priority));

            let mut remaining = Vec::new();

            for constraint in std::mem::take(&mut self.constraints) {
                match self.solve_single_constraint(&constraint) {
                    Ok(()) => {
                        progress = true;
                    }
                    Err(None) => {
                        // Blocked, try again later
                        let mut c = constraint;
                        c.priority = c.priority.saturating_sub(1);
                        remaining.push(c);
                    }
                    Err(Some(error)) => {
                        self.errors.push(error);
                        progress = true;
                    }
                }
            }

            self.constraints = remaining;
        }

        if !self.constraints.is_empty() {
            // Some constraints couldn't be solved
            for c in &self.constraints {
                self.errors.push(TypeError::Mismatch {
                    expected: ResolvedType::Unknown,
                    found: ResolvedType::Unknown,
                    span: c.span,
                    context: format!("unsolved constraint: {:?}", c.reason),
                });
            }
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Attempt to solve a single constraint
    /// Returns Ok(()) if solved, Err(None) if blocked, Err(Some) if failed
    fn solve_single_constraint(
        &mut self,
        constraint: &Constraint,
    ) -> Result<(), Option<TypeError>> {
        match &constraint.kind {
            ConstraintKind::Equality { lhs, rhs } => {
                let lhs = self.apply(lhs);
                let rhs = self.apply(rhs);
                self.inference.unify(&lhs, &rhs).map_err(|e| {
                    Some(TypeError::UnificationFailed {
                        error: e,
                        span: constraint.span,
                    })
                })
            }

            ConstraintKind::FieldAccess {
                receiver,
                field,
                field_type,
            } => {
                let receiver = self.apply(receiver);

                // Check if receiver is resolved enough
                if matches!(receiver, ResolvedType::TypeVar(_)) {
                    return Err(None); // Blocked
                }

                // Look up field type
                if let Some(actual_field_type) = self.lookup_field_type(&receiver, field) {
                    let field_type = self.apply(field_type);
                    self.inference
                        .unify(&actual_field_type, &field_type)
                        .map_err(|e| {
                            Some(TypeError::UnificationFailed {
                                error: e,
                                span: constraint.span,
                            })
                        })
                } else {
                    Err(Some(TypeError::UnknownIdentifier {
                        name: format!("{}.{}", receiver, field),
                        span: constraint.span,
                    }))
                }
            }

            ConstraintKind::FunctionCall {
                callee,
                args,
                result,
            } => {
                let callee = self.apply(callee);

                // Check if callee is resolved enough
                if matches!(callee, ResolvedType::TypeVar(_)) {
                    return Err(None); // Blocked
                }

                match callee {
                    ResolvedType::Function {
                        params,
                        return_type,
                    } => {
                        if params.len() != args.len() {
                            return Err(Some(TypeError::ArityMismatch {
                                function: "<anonymous>".to_string(),
                                expected: params.len(),
                                found: args.len(),
                                span: constraint.span,
                            }));
                        }

                        // Unify each argument with parameter
                        for (i, (param, arg)) in params.iter().zip(args.iter()).enumerate() {
                            let arg = self.apply(arg);
                            if let Err(_e) = self.inference.unify(param, &arg) {
                                return Err(Some(TypeError::ArgumentMismatch {
                                    function: "<anonymous>".to_string(),
                                    param: format!("arg{}", i),
                                    expected: param.clone(),
                                    found: arg,
                                    span: constraint.span,
                                }));
                            }
                        }

                        // Unify return type
                        let result = self.apply(result);
                        self.inference.unify(&return_type, &result).map_err(|e| {
                            Some(TypeError::UnificationFailed {
                                error: e,
                                span: constraint.span,
                            })
                        })
                    }
                    _ => Err(Some(TypeError::InvalidOperation {
                        op: "call".to_string(),
                        ty: callee,
                        span: constraint.span,
                    })),
                }
            }

            ConstraintKind::IndexAccess {
                container,
                index,
                element,
            } => {
                let container = self.apply(container);
                let index = self.apply(index);

                if matches!(container, ResolvedType::TypeVar(_)) {
                    return Err(None); // Blocked
                }

                match &container {
                    ResolvedType::Array {
                        element: elem_ty, ..
                    } => {
                        // Index must be integer
                        if let Err(_e) = self.inference.unify(&index, &ResolvedType::UInt64) {
                            return Err(Some(TypeError::InvalidOperation {
                                op: "index".to_string(),
                                ty: index,
                                span: constraint.span,
                            }));
                        }

                        let element = self.apply(element);
                        self.inference.unify(elem_ty, &element).map_err(|e| {
                            Some(TypeError::UnificationFailed {
                                error: e,
                                span: constraint.span,
                            })
                        })
                    }
                    _ => Err(Some(TypeError::InvalidOperation {
                        op: "index".to_string(),
                        ty: container,
                        span: constraint.span,
                    })),
                }
            }

            ConstraintKind::Subtype { sub, sup } => {
                // For now, treat subtyping as equality
                // TODO: Implement proper subtyping with variance
                let sub = self.apply(sub);
                let sup = self.apply(sup);
                self.inference.unify(&sub, &sup).map_err(|e| {
                    Some(TypeError::UnificationFailed {
                        error: e,
                        span: constraint.span,
                    })
                })
            }

            ConstraintKind::TypeBinding { name, ty } => {
                let ty = self.apply(ty);
                self.bind(name, ty, Pramana::Anumana, constraint.span);
                Ok(())
            }

            ConstraintKind::TraitBound {
                ty,
                trait_name,
                trait_params: _,
            } => {
                let ty = self.apply(ty);

                // If type is still a type variable, we're blocked
                if matches!(ty, ResolvedType::TypeVar(_)) {
                    return Err(None); // Blocked - defer until type is resolved
                }

                // Check if type implements the trait
                // For now, check built-in traits and registered implementations
                if self.check_trait_impl(&ty, trait_name) {
                    Ok(())
                } else {
                    Err(Some(TypeError::Mismatch {
                        expected: ResolvedType::Named {
                            name: format!("impl {}", trait_name),
                            generics: vec![],
                        },
                        found: ty,
                        span: constraint.span,
                        context: format!("type does not implement trait '{}'", trait_name),
                    }))
                }
            }

            ConstraintKind::AssociatedType {
                impl_type,
                trait_name,
                assoc_name,
                expected_type,
            } => {
                let impl_type = self.apply(impl_type);
                let expected_type = self.apply(expected_type);

                // If impl type is still a type variable, we're blocked
                if matches!(impl_type, ResolvedType::TypeVar(_)) {
                    return Err(None); // Blocked
                }

                // Look up the associated type
                if let Some(actual_assoc) = self.lookup_associated_type(&impl_type, trait_name, assoc_name) {
                    self.inference.unify(&actual_assoc, &expected_type).map_err(|e| {
                        Some(TypeError::UnificationFailed {
                            error: e,
                            span: constraint.span,
                        })
                    })
                } else {
                    Err(Some(TypeError::UnknownIdentifier {
                        name: format!("<{} as {}>::{}", impl_type, trait_name, assoc_name),
                        span: constraint.span,
                    }))
                }
            }
        }
    }

    /// Check if a type implements a trait (Dharma fulfillment)
    ///
    /// Built-in trait implementations follow Sanskrit philosophical principles:
    /// - Copy (Anusaraṇīya): Types that can be freely duplicated
    /// - Clone (Pratilipi): Types that can create explicit copies
    /// - Debug (Nirūpaṇa): Types that can describe themselves
    /// - Eq (Sāmya): Types supporting equality
    /// - Ord (Krama): Types with ordering
    fn check_trait_impl(&self, ty: &ResolvedType, trait_name: &str) -> bool {
        match trait_name {
            // Built-in traits for primitives
            "Copy" | "Anusaraṇīya" => self.is_copy_type(ty),
            "Clone" | "Pratilipi" => self.is_clone_type(ty),
            "Debug" | "Nirūpaṇa" => true, // All types can be debugged
            "Eq" | "Sāmya" => self.is_eq_type(ty),
            "PartialEq" | "Aṃśa-Sāmya" => self.is_eq_type(ty),
            "Ord" | "Krama" => self.is_ord_type(ty),
            "PartialOrd" | "Aṃśa-Krama" => self.is_ord_type(ty),
            "Default" | "Mūla" => self.is_default_type(ty),
            "Send" | "Preṣaṇīya" => self.is_send_type(ty),
            "Sync" | "Samāna-Kālīna" => self.is_sync_type(ty),
            "Sized" | "Parimita" => self.is_sized_type(ty),

            // Check registered trait implementations
            _ => {
                // TODO: Look up in trait registry
                false
            }
        }
    }

    /// Check if type implements Copy (can be bitwise copied)
    fn is_copy_type(&self, ty: &ResolvedType) -> bool {
        matches!(
            ty,
            ResolvedType::Int8
                | ResolvedType::Int16
                | ResolvedType::Int32
                | ResolvedType::Int64
                | ResolvedType::UInt8
                | ResolvedType::UInt16
                | ResolvedType::UInt32
                | ResolvedType::UInt64
                | ResolvedType::Float32
                | ResolvedType::Float64
                | ResolvedType::Bool
                | ResolvedType::Char
                | ResolvedType::Unit
        )
    }

    /// Check if type implements Clone
    fn is_clone_type(&self, ty: &ResolvedType) -> bool {
        // All Copy types are Clone, plus String and some containers
        self.is_copy_type(ty) || matches!(ty, ResolvedType::String)
    }

    /// Check if type implements Eq
    fn is_eq_type(&self, ty: &ResolvedType) -> bool {
        matches!(
            ty,
            ResolvedType::Int8
                | ResolvedType::Int16
                | ResolvedType::Int32
                | ResolvedType::Int64
                | ResolvedType::UInt8
                | ResolvedType::UInt16
                | ResolvedType::UInt32
                | ResolvedType::UInt64
                | ResolvedType::Bool
                | ResolvedType::Char
                | ResolvedType::String
                | ResolvedType::Unit
        )
    }

    /// Check if type implements Ord
    fn is_ord_type(&self, ty: &ResolvedType) -> bool {
        matches!(
            ty,
            ResolvedType::Int8
                | ResolvedType::Int16
                | ResolvedType::Int32
                | ResolvedType::Int64
                | ResolvedType::UInt8
                | ResolvedType::UInt16
                | ResolvedType::UInt32
                | ResolvedType::UInt64
                | ResolvedType::Char
                | ResolvedType::String
        )
    }

    /// Check if type implements Default
    fn is_default_type(&self, ty: &ResolvedType) -> bool {
        matches!(
            ty,
            ResolvedType::Int8
                | ResolvedType::Int16
                | ResolvedType::Int32
                | ResolvedType::Int64
                | ResolvedType::UInt8
                | ResolvedType::UInt16
                | ResolvedType::UInt32
                | ResolvedType::UInt64
                | ResolvedType::Float32
                | ResolvedType::Float64
                | ResolvedType::Bool
                | ResolvedType::String
                | ResolvedType::Unit
        )
    }

    /// Check if type is Send (can be sent across threads)
    fn is_send_type(&self, ty: &ResolvedType) -> bool {
        // Most types are Send unless they contain Rc or raw pointers
        !matches!(ty, ResolvedType::Reference { .. })
    }

    /// Check if type is Sync (can be shared across threads)
    fn is_sync_type(&self, ty: &ResolvedType) -> bool {
        // Similar to Send
        !matches!(ty, ResolvedType::Reference { mutable: true, .. })
    }

    /// Check if type is Sized (has known size at compile time)
    fn is_sized_type(&self, ty: &ResolvedType) -> bool {
        // Most types are Sized except slices and trait objects
        !matches!(ty, ResolvedType::Unknown)
    }

    /// Look up an associated type for a trait implementation
    fn lookup_associated_type(
        &self,
        _impl_type: &ResolvedType,
        _trait_name: &str,
        _assoc_name: &str,
    ) -> Option<ResolvedType> {
        // TODO: Implement associated type lookup from trait registry
        // For now, return None (no associated types)
        None
    }

    /// Look up field type for a resolved type
    fn lookup_field_type(&self, ty: &ResolvedType, field: &str) -> Option<ResolvedType> {
        match ty {
            ResolvedType::Named { name, generics: _ } => {
                // Look up in type definitions
                if let Some(def) = self.context.lookup_type_def(name) {
                    match &def.body {
                        super::types::TypeBodyResolved::Struct(fields) => {
                            for (fname, ftype) in fields {
                                if fname == field {
                                    // TODO: Apply generic substitution
                                    return Some(ftype.clone());
                                }
                            }
                        }
                        _ => {}
                    }
                }
                None
            }
            ResolvedType::Tuple(elems) => {
                // Tuple field access by index
                if let Ok(idx) = field.parse::<usize>() {
                    elems.get(idx).cloned()
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    // ========================================================================
    // Error Collection (Doṣa Saṅgraha)
    // ========================================================================

    /// Add an error
    pub fn add_error(&mut self, error: TypeError) {
        self.errors.push(error);
    }

    /// Get all errors
    pub fn errors(&self) -> &[TypeError] {
        &self.errors
    }

    /// Take all errors (consuming)
    pub fn take_errors(&mut self) -> Vec<TypeError> {
        std::mem::take(&mut self.errors)
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    // ========================================================================
    // Diagnostic Helpers (Nidāna Sahāyaka)
    // ========================================================================

    /// Get diagnostic information about the current state
    pub fn diagnostic_state(&self) -> DiagnosticState {
        DiagnosticState {
            scope_depth: self.context.scope_depth(),
            pending_constraints: self.constraints.len(),
            type_vars_created: self.inference.type_var_count(),
            region_vars_created: self.lifetimes.region_count(),
            errors_collected: self.errors.len(),
        }
    }

    /// Get reference to the type context
    pub fn context(&self) -> &TypeContext {
        &self.context
    }

    /// Get mutable reference to the type context
    pub fn context_mut(&mut self) -> &mut TypeContext {
        &mut self.context
    }
}

impl Default for UnifiedChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Diagnostic state for debugging
#[derive(Debug, Clone)]
pub struct DiagnosticState {
    pub scope_depth: usize,
    pub pending_constraints: usize,
    pub type_vars_created: usize,
    pub region_vars_created: usize,
    pub errors_collected: usize,
}

// ============================================================================
// Type Judgment (Prakāra Nirṇaya)
// ============================================================================

/// A type judgment with epistemological metadata
///
/// In Nyāya, a judgment (nirṇaya) is not just a conclusion but
/// includes the pramāṇa (means) by which it was established.
#[derive(Debug, Clone)]
pub struct TypeJudgment {
    /// The inferred type
    pub ty: ResolvedType,
    /// How certain we are (0.0-1.0)
    pub certainty: f32,
    /// Which pramāṇa established this judgment
    pub pramana: Pramana,
    /// Lifetime annotations if applicable
    pub lifetime: Option<RegionVar>,
    /// Source span
    pub span: Option<Span>,
}

impl TypeJudgment {
    /// Create from explicit annotation (Pratyakṣa)
    pub fn from_annotation(ty: ResolvedType, span: Span) -> Self {
        Self {
            ty,
            certainty: 1.0,
            pramana: Pramana::Pratyaksha,
            lifetime: None,
            span: Some(span),
        }
    }

    /// Create from inference (Anumāna)
    pub fn from_inference(ty: ResolvedType, certainty: f32, span: Option<Span>) -> Self {
        Self {
            ty,
            certainty,
            pramana: Pramana::Anumana,
            lifetime: None,
            span,
        }
    }

    /// Create from function signature (Śabda)
    pub fn from_signature(ty: ResolvedType, span: Option<Span>) -> Self {
        Self {
            ty,
            certainty: 0.90,
            pramana: Pramana::Shabda,
            lifetime: None,
            span,
        }
    }

    /// Create from pattern matching (Upamāna)
    pub fn from_pattern(ty: ResolvedType, span: Option<Span>) -> Self {
        Self {
            ty,
            certainty: 0.85,
            pramana: Pramana::Upamana,
            lifetime: None,
            span,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_checker_creation() {
        let checker = UnifiedChecker::new();
        let state = checker.diagnostic_state();
        assert_eq!(state.scope_depth, 0);
        assert_eq!(state.pending_constraints, 0);
        assert_eq!(state.errors_collected, 0);
    }

    #[test]
    fn test_scope_management() {
        let mut checker = UnifiedChecker::new();
        assert_eq!(checker.diagnostic_state().scope_depth, 0);

        checker.enter_scope(ScopeKind::Function);
        assert_eq!(checker.diagnostic_state().scope_depth, 1);

        checker.enter_scope(ScopeKind::Block);
        assert_eq!(checker.diagnostic_state().scope_depth, 2);

        checker.exit_scope();
        assert_eq!(checker.diagnostic_state().scope_depth, 1);

        checker.exit_scope();
        assert_eq!(checker.diagnostic_state().scope_depth, 0);
    }

    #[test]
    fn test_symbol_binding() {
        let mut checker = UnifiedChecker::new();

        checker.bind("x", ResolvedType::Int32, Pramana::Pratyaksha, None);

        let info = checker.lookup("x");
        assert!(info.is_some());
        assert_eq!(info.unwrap().ty, ResolvedType::Int32);
    }

    #[test]
    fn test_symbol_shadowing() {
        let mut checker = UnifiedChecker::new();

        checker.bind("x", ResolvedType::Int32, Pramana::Pratyaksha, None);

        checker.enter_scope(ScopeKind::Block);
        checker.bind("x", ResolvedType::String, Pramana::Pratyaksha, None);

        // Inner scope sees String
        let info = checker.lookup("x");
        assert_eq!(info.unwrap().ty, ResolvedType::String);

        checker.exit_scope();

        // Outer scope sees Int32
        let info = checker.lookup("x");
        assert_eq!(info.unwrap().ty, ResolvedType::Int32);
    }

    #[test]
    fn test_fresh_type_var() {
        let mut checker = UnifiedChecker::new();

        let t1 = checker.fresh_type_var();
        let t2 = checker.fresh_type_var();

        // Should be different variables
        assert_ne!(t1, t2);
    }

    #[test]
    fn test_basic_unification() {
        let mut checker = UnifiedChecker::new();

        let t = checker.fresh_type_var();

        // Unify type var with concrete type
        assert!(checker.unify(&t, &ResolvedType::Int32).is_ok());

        // After unification, t should resolve to Int32
        let resolved = checker.apply(&t);
        assert_eq!(resolved, ResolvedType::Int32);
    }

    #[test]
    fn test_unification_failure() {
        let mut checker = UnifiedChecker::new();

        let result = checker.unify(&ResolvedType::Int32, &ResolvedType::String);
        assert!(result.is_err());
    }

    #[test]
    fn test_constraint_generation() {
        let mut checker = UnifiedChecker::new();

        let t1 = checker.fresh_type_var();
        let t2 = ResolvedType::Int32;

        checker.constrain_equal(
            t1.clone(),
            t2.clone(),
            ConstraintReason::LetBinding {
                name: "x".to_string(),
            },
            None,
        );

        assert_eq!(checker.diagnostic_state().pending_constraints, 1);
    }

    #[test]
    fn test_constraint_solving() {
        let mut checker = UnifiedChecker::new();

        let t = checker.fresh_type_var();

        checker.constrain_equal(
            t.clone(),
            ResolvedType::Int32,
            ConstraintReason::Annotation,
            None,
        );

        let result = checker.solve_constraints();
        assert!(result.is_ok());

        let resolved = checker.apply(&t);
        assert_eq!(resolved, ResolvedType::Int32);
    }

    #[test]
    fn test_type_judgment_pratyaksha() {
        let judgment = TypeJudgment::from_annotation(ResolvedType::Int32, Span::new(0, 5));
        assert_eq!(judgment.certainty, 1.0);
        assert_eq!(judgment.pramana, Pramana::Pratyaksha);
    }

    #[test]
    fn test_type_judgment_anumana() {
        let judgment = TypeJudgment::from_inference(ResolvedType::Int32, 0.95, None);
        assert_eq!(judgment.certainty, 0.95);
        assert_eq!(judgment.pramana, Pramana::Anumana);
    }

    #[test]
    fn test_type_judgment_shabda() {
        let judgment = TypeJudgment::from_signature(ResolvedType::Int32, None);
        assert_eq!(judgment.certainty, 0.90);
        assert_eq!(judgment.pramana, Pramana::Shabda);
    }

    #[test]
    fn test_type_judgment_upamana() {
        let judgment = TypeJudgment::from_pattern(ResolvedType::Int32, None);
        assert_eq!(judgment.certainty, 0.85);
        assert_eq!(judgment.pramana, Pramana::Upamana);
    }

    #[test]
    fn test_builtin_functions() {
        let checker = UnifiedChecker::new();

        // Check that print is registered
        let print_sig = checker.context.lookup_function("print");
        assert!(print_sig.is_some());

        let sig = print_sig.unwrap();
        assert_eq!(sig.return_type, ResolvedType::Unit);
    }

    #[test]
    fn test_region_creation() {
        let mut checker = UnifiedChecker::new();

        let r1 = checker.fresh_region();
        let r2 = checker.fresh_region();

        // Should be different regions
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_generalize_monomorphic() {
        let checker = UnifiedChecker::new();

        let scheme = checker.generalize(&ResolvedType::Int32);

        // Simple type should generalize to monomorphic scheme
        assert!(scheme.is_mono());
    }

    #[test]
    fn test_checker_config() {
        let config = CheckerConfig {
            max_iterations: 500,
            strict_lifetimes: false,
            experimental: true,
            verbosity: 3,
        };

        let checker = UnifiedChecker::with_config(config.clone());
        assert_eq!(checker.config.max_iterations, 500);
        assert!(!checker.config.strict_lifetimes);
        assert!(checker.config.experimental);
    }

    #[test]
    fn test_diagnostic_state() {
        let mut checker = UnifiedChecker::new();

        checker.enter_scope(ScopeKind::Function);
        checker.fresh_type_var();
        checker.fresh_type_var();
        checker.constrain_equal(
            ResolvedType::Int32,
            ResolvedType::Int32,
            ConstraintReason::Annotation,
            None,
        );

        let state = checker.diagnostic_state();
        assert_eq!(state.scope_depth, 1);
        assert_eq!(state.pending_constraints, 1);
        assert!(state.type_vars_created >= 2);
    }

    #[test]
    fn test_transitive_unification() {
        let mut checker = UnifiedChecker::new();

        let t1 = checker.fresh_type_var();
        let t2 = checker.fresh_type_var();

        // t1 = t2
        assert!(checker.unify(&t1, &t2).is_ok());

        // t2 = Int32
        assert!(checker.unify(&t2, &ResolvedType::Int32).is_ok());

        // t1 should now resolve to Int32
        let resolved = checker.apply(&t1);
        assert_eq!(resolved, ResolvedType::Int32);
    }

    #[test]
    fn test_function_type_unification() {
        let mut checker = UnifiedChecker::new();

        let t_arg = checker.fresh_type_var();
        let t_ret = checker.fresh_type_var();

        let fn_type = ResolvedType::Function {
            params: vec![t_arg.clone()],
            return_type: Box::new(t_ret.clone()),
        };

        let expected = ResolvedType::Function {
            params: vec![ResolvedType::Int32],
            return_type: Box::new(ResolvedType::Bool),
        };

        assert!(checker.unify(&fn_type, &expected).is_ok());

        assert_eq!(checker.apply(&t_arg), ResolvedType::Int32);
        assert_eq!(checker.apply(&t_ret), ResolvedType::Bool);
    }

    #[test]
    fn test_multiple_constraints() {
        let mut checker = UnifiedChecker::new();

        let t1 = checker.fresh_type_var();
        let t2 = checker.fresh_type_var();
        let t3 = checker.fresh_type_var();

        // t1 = t2
        checker.constrain_equal(t1.clone(), t2.clone(), ConstraintReason::Annotation, None);

        // t2 = t3
        checker.constrain_equal(t2.clone(), t3.clone(), ConstraintReason::Annotation, None);

        // t3 = Int32
        checker.constrain_equal(
            t3.clone(),
            ResolvedType::Int32,
            ConstraintReason::Annotation,
            None,
        );

        let result = checker.solve_constraints();
        assert!(result.is_ok());

        // All should resolve to Int32
        assert_eq!(checker.apply(&t1), ResolvedType::Int32);
        assert_eq!(checker.apply(&t2), ResolvedType::Int32);
        assert_eq!(checker.apply(&t3), ResolvedType::Int32);
    }

    // ========================================================================
    // Algorithm W Tests (Hindley-Milner)
    // ========================================================================

    #[test]
    fn test_let_binding_monomorphic() {
        let mut checker = UnifiedChecker::new();

        // let x = 42  (monomorphic - no type vars)
        let scheme = checker.process_let_binding("x", ResolvedType::Int32, None);

        // Should be monomorphic
        assert!(scheme.is_mono());
        assert_eq!(scheme.body, ResolvedType::Int32);
    }

    #[test]
    fn test_use_variable() {
        let mut checker = UnifiedChecker::new();

        // Bind a variable
        checker.bind("x", ResolvedType::Int32, Pramana::Pratyaksha, None);

        // Use it
        let ty = checker.use_variable("x");
        assert!(ty.is_some());
        assert_eq!(ty.unwrap(), ResolvedType::Int32);
    }

    #[test]
    fn test_infer_function() {
        let mut checker = UnifiedChecker::new();

        // fn(x) -> x  (identity function shape)
        let params = vec![("x".to_string(), Some(ResolvedType::Int32))];
        let fn_type = checker.infer_function(&params, ResolvedType::Int32);

        match fn_type {
            ResolvedType::Function { params, return_type } => {
                assert_eq!(params.len(), 1);
                assert_eq!(params[0], ResolvedType::Int32);
                assert_eq!(*return_type, ResolvedType::Int32);
            }
            _ => panic!("Expected function type"),
        }
    }

    #[test]
    fn test_infer_function_with_type_var() {
        let mut checker = UnifiedChecker::new();

        // fn(x) -> x  where x has no annotation
        let params = vec![("x".to_string(), None)];
        let ret = checker.fresh_type_var();
        let fn_type = checker.infer_function(&params, ret.clone());

        match fn_type {
            ResolvedType::Function { params, return_type: _ } => {
                assert_eq!(params.len(), 1);
                // Param should be a type variable
                assert!(matches!(params[0], ResolvedType::TypeVar(_)));
            }
            _ => panic!("Expected function type"),
        }
    }

    #[test]
    fn test_infer_application() {
        let mut checker = UnifiedChecker::new();

        // f: Int32 -> String
        let callee = ResolvedType::Function {
            params: vec![ResolvedType::Int32],
            return_type: Box::new(ResolvedType::String),
        };

        // f(42) : String
        let result = checker.infer_application(callee, vec![ResolvedType::Int32], None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ResolvedType::String);
    }

    #[test]
    fn test_infer_application_type_inference() {
        let mut checker = UnifiedChecker::new();

        // f: ?0 -> ?1
        let t_arg = checker.fresh_type_var();
        let t_ret = checker.fresh_type_var();
        let callee = ResolvedType::Function {
            params: vec![t_arg.clone()],
            return_type: Box::new(t_ret.clone()),
        };

        // f(42) should infer ?0 = Int32
        let result = checker.infer_application(callee, vec![ResolvedType::Int32], None);
        assert!(result.is_ok());

        // Argument type should be inferred
        assert_eq!(checker.apply(&t_arg), ResolvedType::Int32);
    }

    #[test]
    fn test_infer_if_expression() {
        let mut checker = UnifiedChecker::new();

        // if true then 42 else 0
        let result = checker.infer_if_expression(
            ResolvedType::Bool,
            ResolvedType::Int32,
            ResolvedType::Int32,
            None,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ResolvedType::Int32);
    }

    #[test]
    fn test_infer_if_expression_with_type_var() {
        let mut checker = UnifiedChecker::new();

        let then_type = checker.fresh_type_var();

        // if true then ?0 else Int32
        let result = checker.infer_if_expression(
            ResolvedType::Bool,
            then_type.clone(),
            ResolvedType::Int32,
            None,
        );
        assert!(result.is_ok());

        // Then type should be inferred as Int32
        assert_eq!(checker.apply(&then_type), ResolvedType::Int32);
    }

    #[test]
    fn test_infer_if_expression_condition_error() {
        let mut checker = UnifiedChecker::new();

        // if "not bool" then 42 else 0
        let result = checker.infer_if_expression(
            ResolvedType::String,
            ResolvedType::Int32,
            ResolvedType::Int32,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_infer_if_expression_branch_mismatch() {
        let mut checker = UnifiedChecker::new();

        // if true then 42 else "string"
        let result = checker.infer_if_expression(
            ResolvedType::Bool,
            ResolvedType::Int32,
            ResolvedType::String,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_generalize_with_env() {
        let mut checker = UnifiedChecker::new();

        // Add a variable with a type var to the environment
        let t_env = checker.fresh_type_var();
        checker.bind("y", t_env.clone(), Pramana::Anumana, None);

        // Create a type with a different type var
        let t_new = checker.fresh_type_var();

        // Generalize: should quantify over t_new but not t_env
        let scheme = checker.generalize_with_env(&t_new);

        // Should be polymorphic (one quantified var)
        assert!(scheme.is_poly());
        assert_eq!(scheme.arity(), 1);
    }

    // ========================================================================
    // Trait Constraint Tests (Dharma Niyama)
    // ========================================================================

    #[test]
    fn test_copy_trait_primitive() {
        let checker = UnifiedChecker::new();

        // Int32 implements Copy
        assert!(checker.implements_trait(&ResolvedType::Int32, "Copy"));
        assert!(checker.implements_trait(&ResolvedType::Bool, "Copy"));
        assert!(checker.implements_trait(&ResolvedType::Char, "Copy"));
    }

    #[test]
    fn test_copy_trait_string() {
        let checker = UnifiedChecker::new();

        // String does NOT implement Copy
        assert!(!checker.implements_trait(&ResolvedType::String, "Copy"));
    }

    #[test]
    fn test_clone_trait() {
        let checker = UnifiedChecker::new();

        // All Copy types also implement Clone
        assert!(checker.implements_trait(&ResolvedType::Int32, "Clone"));
        // String implements Clone
        assert!(checker.implements_trait(&ResolvedType::String, "Clone"));
    }

    #[test]
    fn test_eq_trait() {
        let checker = UnifiedChecker::new();

        assert!(checker.implements_trait(&ResolvedType::Int32, "Eq"));
        assert!(checker.implements_trait(&ResolvedType::String, "Eq"));
        assert!(checker.implements_trait(&ResolvedType::Bool, "Eq"));
        // Float doesn't implement Eq (only PartialEq)
        assert!(!checker.implements_trait(&ResolvedType::Float64, "Eq"));
    }

    #[test]
    fn test_ord_trait() {
        let checker = UnifiedChecker::new();

        assert!(checker.implements_trait(&ResolvedType::Int32, "Ord"));
        assert!(checker.implements_trait(&ResolvedType::String, "Ord"));
        // Bool doesn't implement Ord
        assert!(!checker.implements_trait(&ResolvedType::Bool, "Ord"));
    }

    #[test]
    fn test_sanskrit_trait_names() {
        let checker = UnifiedChecker::new();

        // Sanskrit names should work too
        assert!(checker.implements_trait(&ResolvedType::Int32, "Anusaraṇīya")); // Copy
        assert!(checker.implements_trait(&ResolvedType::Int32, "Pratilipi")); // Clone
        assert!(checker.implements_trait(&ResolvedType::Int32, "Sāmya")); // Eq
        assert!(checker.implements_trait(&ResolvedType::Int32, "Krama")); // Ord
    }

    #[test]
    fn test_trait_constraint_solving() {
        let mut checker = UnifiedChecker::new();

        // Add trait constraint: ?0 : Copy
        let t = checker.fresh_type_var();
        checker.constrain_trait(t.clone(), "Copy", None);

        // Then constrain ?0 = Int32
        checker.constrain_equal(
            t.clone(),
            ResolvedType::Int32,
            ConstraintReason::Annotation,
            None,
        );

        // Should solve successfully
        let result = checker.solve_constraints();
        assert!(result.is_ok());
    }

    #[test]
    fn test_trait_constraint_failure() {
        let mut checker = UnifiedChecker::new();

        // Add trait constraint: String : Copy (should fail)
        checker.constrain_trait(ResolvedType::String, "Copy", None);

        // Should fail to solve
        let result = checker.solve_constraints();
        assert!(result.is_err());
    }

    #[test]
    fn test_trait_constraint_deferred() {
        let mut checker = UnifiedChecker::new();

        let t = checker.fresh_type_var();

        // Add trait constraint first (will be deferred)
        checker.constrain_trait(t.clone(), "Copy", None);

        // Then constrain type
        checker.constrain_equal(
            t.clone(),
            ResolvedType::Int32,
            ConstraintReason::Annotation,
            None,
        );

        // Should solve successfully with deferral
        let result = checker.solve_constraints();
        assert!(result.is_ok());
        assert_eq!(checker.apply(&t), ResolvedType::Int32);
    }

    #[test]
    fn test_default_trait() {
        let checker = UnifiedChecker::new();

        assert!(checker.implements_trait(&ResolvedType::Int32, "Default"));
        assert!(checker.implements_trait(&ResolvedType::String, "Default"));
        assert!(checker.implements_trait(&ResolvedType::Bool, "Default"));
    }

    #[test]
    fn test_sized_trait() {
        let checker = UnifiedChecker::new();

        // Most types are Sized
        assert!(checker.implements_trait(&ResolvedType::Int32, "Sized"));
        assert!(checker.implements_trait(&ResolvedType::String, "Sized"));
        // Unknown is not Sized
        assert!(!checker.implements_trait(&ResolvedType::Unknown, "Sized"));
    }

    #[test]
    fn test_send_sync_traits() {
        let checker = UnifiedChecker::new();

        assert!(checker.implements_trait(&ResolvedType::Int32, "Send"));
        assert!(checker.implements_trait(&ResolvedType::Int32, "Sync"));

        // References have special rules
        let immut_ref = ResolvedType::Reference {
            inner: Box::new(ResolvedType::Int32),
            mutable: false,
            lifetime: None,
        };
        assert!(checker.implements_trait(&immut_ref, "Sync"));

        let mut_ref = ResolvedType::Reference {
            inner: Box::new(ResolvedType::Int32),
            mutable: true,
            lifetime: None,
        };
        assert!(!checker.implements_trait(&mut_ref, "Sync"));
    }
}
