//! Sāmānya-Viśeṣa: Generics & Monomorphization System
//!
//! # Philosophy: Sāmānya (सामान्य) & Viśeṣa (विशेष)
//!
//! In Nyāya-Vaiśeṣika philosophy:
//! - **Sāmānya** (generality) = universal/generic - the abstract pattern
//! - **Viśeṣa** (particularity) = specific instance - the concrete instantiation
//!
//! This maps perfectly to generics:
//! - Generic type `T` is Sāmānya (abstract universal)
//! - Concrete `i32` is Viśeṣa (particular instance)
//! - Monomorphization is the process of Viśeṣa-karaṇa (particularization)
//!
//! ## Sanskrit Terminology
//!
//! - sāmānya (सामान्य) = generic/universal
//! - viśeṣa (विशेष) = specific/particular
//! - prakāra-cala (प्रकार-चल) = type parameter ("moving type")
//! - pratisthāpana (प्रतिस्थापन) = substitution ("placing in place of")
//! - mūrti-karaṇa (मूर्ति-करण) = monomorphization ("making concrete form")
//!
//! ## References
//!
//! - Pierce, Benjamin (2002): "Types and Programming Languages"
//! - Harper, Robert (2016): "Practical Foundations for Programming Languages"
//! - Odersky et al. (2004): "A Type-Safe Embedding of Polymorphic Variants"
//! - Rust RFC 2000: "Const generics"

use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::lexer::Span;
use crate::parser::ast::{Block, FunctionDef, GenericParam, Identifier, Type, TypeDef};
use crate::semantics::traits::{TraitBound, TraitId, TraitRef};

// ============================================================================
// PART 1: TYPE PARAMETERS (Prakāra-Cala - Moving Types)
// ============================================================================

/// Unique identifier for type variables
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeVarId(pub u32);

impl TypeVarId {
    pub fn new(id: u32) -> Self {
        TypeVarId(id)
    }
}

/// A type variable/parameter in a generic context
///
/// Sanskrit: prakāra-cala (प्रकार-चल) - "moving type"
#[derive(Debug, Clone)]
pub struct TypeVariable {
    /// Unique identifier
    pub id: TypeVarId,

    /// Variable name (T, U, Item, etc.)
    pub name: String,

    /// Sanskrit name for display
    pub sanskrit_name: Option<String>,

    /// Trait bounds
    pub bounds: Vec<TraitBound>,

    /// Variance (covariant, contravariant, invariant)
    pub variance: Variance,

    /// Default type (if any)
    pub default: Option<Type>,

    /// Is this a lifetime parameter?
    pub is_lifetime: bool,

    /// Is this a const generic?
    pub is_const: bool,

    /// Source span
    pub span: Span,
}

impl TypeVariable {
    pub fn new(name: impl Into<String>, id: TypeVarId, span: Span) -> Self {
        Self {
            id,
            name: name.into(),
            sanskrit_name: None,
            bounds: Vec::new(),
            variance: Variance::Invariant,
            default: None,
            is_lifetime: false,
            is_const: false,
            span,
        }
    }

    pub fn with_bound(mut self, bound: TraitBound) -> Self {
        self.bounds.push(bound);
        self
    }

    pub fn with_default(mut self, ty: Type) -> Self {
        self.default = Some(ty);
        self
    }

    pub fn lifetime(mut self) -> Self {
        self.is_lifetime = true;
        self
    }

    pub fn const_generic(mut self) -> Self {
        self.is_const = true;
        self
    }
}

/// Variance of a type parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variance {
    /// Covariant: if T: U then F<T>: F<U>
    Covariant,
    /// Contravariant: if T: U then F<U>: F<T>
    Contravariant,
    /// Invariant: F<T> and F<U> are unrelated
    Invariant,
    /// Bivariant: F<T> = F<U> regardless of T, U
    Bivariant,
}

// ============================================================================
// PART 2: GENERIC CONTEXT (Sāmānya-Saṃdarbha - Generic Environment)
// ============================================================================

/// A generic context tracks type variables and substitutions
#[derive(Debug, Clone)]
pub struct GenericContext {
    /// Type variables in scope
    pub type_vars: HashMap<String, TypeVariable>,

    /// Current substitutions
    pub substitutions: HashMap<TypeVarId, Type>,

    /// Next type variable ID
    next_id: u32,

    /// Parent context (for nested generics)
    parent: Option<Box<GenericContext>>,
}

impl GenericContext {
    pub fn new() -> Self {
        Self {
            type_vars: HashMap::new(),
            substitutions: HashMap::new(),
            next_id: 1,
            parent: None,
        }
    }

    /// Create a child context
    pub fn child(&self) -> Self {
        Self {
            type_vars: HashMap::new(),
            substitutions: HashMap::new(),
            next_id: self.next_id,
            parent: Some(Box::new(self.clone())),
        }
    }

    /// Add a type variable
    pub fn add_type_var(&mut self, name: impl Into<String>, span: Span) -> TypeVarId {
        let id = TypeVarId(self.next_id);
        self.next_id += 1;

        let name = name.into();
        let var = TypeVariable::new(name.clone(), id, span);
        self.type_vars.insert(name, var);

        id
    }

    /// Add a type variable with bounds
    pub fn add_bounded_type_var(
        &mut self,
        name: impl Into<String>,
        bounds: Vec<TraitBound>,
        span: Span,
    ) -> TypeVarId {
        let id = TypeVarId(self.next_id);
        self.next_id += 1;

        let name = name.into();
        let mut var = TypeVariable::new(name.clone(), id, span);
        var.bounds = bounds;
        self.type_vars.insert(name, var);

        id
    }

    /// Look up a type variable
    pub fn lookup(&self, name: &str) -> Option<&TypeVariable> {
        self.type_vars
            .get(name)
            .or_else(|| self.parent.as_ref().and_then(|p| p.lookup(name)))
    }

    /// Get a type variable by ID
    pub fn get_by_id(&self, id: TypeVarId) -> Option<&TypeVariable> {
        self.type_vars
            .values()
            .find(|v| v.id == id)
            .or_else(|| self.parent.as_ref().and_then(|p| p.get_by_id(id)))
    }

    /// Add a substitution
    pub fn substitute(&mut self, var: TypeVarId, ty: Type) {
        self.substitutions.insert(var, ty);
    }

    /// Get a substitution
    pub fn get_substitution(&self, var: TypeVarId) -> Option<&Type> {
        self.substitutions
            .get(&var)
            .or_else(|| self.parent.as_ref().and_then(|p| p.get_substitution(var)))
    }

    /// Apply all substitutions to a type
    pub fn apply(&self, ty: &Type) -> Type {
        match ty {
            Type::Named {
                name,
                generics,
                affixes,
            } => {
                // Check if this is a type variable
                if generics.is_empty() {
                    if let Some(var) = self.lookup(&name.name) {
                        if let Some(subst) = self.get_substitution(var.id) {
                            return subst.clone();
                        }
                    }
                }

                // Apply to generic arguments
                Type::Named {
                    name: name.clone(),
                    generics: generics.iter().map(|g| self.apply(g)).collect(),
                    affixes: affixes.clone(),
                }
            }
            Type::Function {
                params,
                return_type,
            } => Type::Function {
                params: params.iter().map(|p| self.apply(p)).collect(),
                return_type: Box::new(self.apply(return_type)),
            },
            Type::Array { element, size } => Type::Array {
                element: Box::new(self.apply(element)),
                size: *size,
            },
            Type::Tuple(types) => Type::Tuple(types.iter().map(|t| self.apply(t)).collect()),
            Type::Reference {
                inner,
                mutable,
                lifetime,
            } => Type::Reference {
                inner: Box::new(self.apply(inner)),
                mutable: *mutable,
                lifetime: *lifetime,
            },
            Type::Inferred => Type::Inferred,
        }
    }

    /// Check if all type variables have substitutions
    pub fn is_fully_substituted(&self) -> bool {
        self.type_vars
            .values()
            .all(|v| self.get_substitution(v.id).is_some())
    }

    /// Get all unsubstituted type variables
    pub fn unsubstituted(&self) -> Vec<&TypeVariable> {
        self.type_vars
            .values()
            .filter(|v| self.get_substitution(v.id).is_none())
            .collect()
    }
}

impl Default for GenericContext {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// PART 3: MONOMORPHIZER (Mūrti-Kara - Form Maker)
// ============================================================================

/// Unique identifier for monomorphized instances
#[derive(Debug, Clone)]
pub struct MonoId {
    /// Original item name
    pub name: String,
    /// Type arguments
    pub type_args: Vec<Type>,
    /// Computed hash key for caching
    cache_key: String,
}

impl MonoId {
    pub fn new(name: impl Into<String>, type_args: Vec<Type>) -> Self {
        let name = name.into();
        let cache_key = Self::compute_cache_key(&name, &type_args);
        Self {
            name,
            type_args,
            cache_key,
        }
    }

    fn compute_cache_key(name: &str, type_args: &[Type]) -> String {
        if type_args.is_empty() {
            name.to_string()
        } else {
            let args: Vec<String> = type_args.iter().map(|t| mangle_type(t)).collect();
            format!("{}__{}", name, args.join("_"))
        }
    }

    /// Get a mangled name for code generation
    pub fn mangled_name(&self) -> String {
        self.cache_key.clone()
    }

    /// Get the cache key for HashMap lookups
    pub fn cache_key(&self) -> &str {
        &self.cache_key
    }
}

impl PartialEq for MonoId {
    fn eq(&self, other: &Self) -> bool {
        self.cache_key == other.cache_key
    }
}

impl Eq for MonoId {}

impl std::hash::Hash for MonoId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.cache_key.hash(state);
    }
}

/// Mangle a type into a string for name generation
fn mangle_type(ty: &Type) -> String {
    match ty {
        Type::Named { name, generics, .. } => {
            if generics.is_empty() {
                name.name.clone()
            } else {
                let args: Vec<String> = generics.iter().map(mangle_type).collect();
                format!("{}__{}", name.name, args.join("_"))
            }
        }
        Type::Reference { inner, mutable, .. } => {
            if *mutable {
                format!("MutRef__{}", mangle_type(inner))
            } else {
                format!("Ref__{}", mangle_type(inner))
            }
        }
        Type::Array { element, size } => {
            if let Some(s) = size {
                format!("Array{}__{}", s, mangle_type(element))
            } else {
                format!("Slice__{}", mangle_type(element))
            }
        }
        Type::Tuple(types) => {
            let args: Vec<String> = types.iter().map(mangle_type).collect();
            format!("Tuple__{}", args.join("_"))
        }
        Type::Function {
            params,
            return_type,
        } => {
            let args: Vec<String> = params.iter().map(mangle_type).collect();
            format!("Fn__{}__{}", args.join("_"), mangle_type(return_type))
        }
        Type::Inferred => "Inferred".to_string(),
    }
}

/// The monomorphizer converts generic code to concrete instantiations
///
/// Sanskrit: mūrti-kara (मूर्ति-कर) - "form maker"
///
/// Based on Rust's and C++'s approach: full monomorphization at compile time.
pub struct Monomorphizer {
    /// Cache of already-monomorphized functions
    function_cache: HashMap<MonoId, MonomorphizedFunction>,

    /// Cache of already-monomorphized types
    type_cache: HashMap<MonoId, MonomorphizedType>,

    /// Queue of items to monomorphize
    work_queue: Vec<MonoRequest>,

    /// Statistics
    stats: MonoStats,
}

/// Statistics about monomorphization
#[derive(Debug, Clone, Default)]
pub struct MonoStats {
    pub functions_mono: usize,
    pub types_mono: usize,
    pub cache_hits: usize,
    pub work_items_processed: usize,
}

/// Request to monomorphize an item
#[derive(Debug, Clone)]
pub enum MonoRequest {
    Function {
        name: String,
        type_args: Vec<Type>,
        span: Span,
    },
    Type {
        name: String,
        type_args: Vec<Type>,
        span: Span,
    },
}

/// A monomorphized function
#[derive(Debug, Clone)]
pub struct MonomorphizedFunction {
    /// Unique identifier
    pub id: MonoId,

    /// Mangled name for codegen
    pub mangled_name: String,

    /// Original function
    pub original_name: String,

    /// Type arguments used
    pub type_args: Vec<Type>,

    /// Substituted parameter types
    pub param_types: Vec<Type>,

    /// Substituted return type
    pub return_type: Type,

    /// Body with types substituted
    pub body: Option<Block>,

    /// Source span
    pub span: Span,
}

/// A monomorphized type
#[derive(Debug, Clone)]
pub struct MonomorphizedType {
    /// Unique identifier
    pub id: MonoId,

    /// Mangled name for codegen
    pub mangled_name: String,

    /// Original type name
    pub original_name: String,

    /// Type arguments used
    pub type_args: Vec<Type>,

    /// Size in bytes (if known)
    pub size: Option<usize>,

    /// Alignment (if known)
    pub align: Option<usize>,

    /// Field types (for structs)
    pub fields: Vec<(String, Type)>,

    /// Source span
    pub span: Span,
}

impl Monomorphizer {
    pub fn new() -> Self {
        Self {
            function_cache: HashMap::new(),
            type_cache: HashMap::new(),
            work_queue: Vec::new(),
            stats: MonoStats::default(),
        }
    }

    /// Request monomorphization of a function
    pub fn request_function(&mut self, name: &str, type_args: Vec<Type>, span: Span) -> MonoId {
        let id = MonoId::new(name, type_args.clone());

        // Check cache
        if self.function_cache.contains_key(&id) {
            self.stats.cache_hits += 1;
            return id;
        }

        // Add to work queue
        self.work_queue.push(MonoRequest::Function {
            name: name.to_string(),
            type_args,
            span,
        });

        id
    }

    /// Request monomorphization of a type
    pub fn request_type(&mut self, name: &str, type_args: Vec<Type>, span: Span) -> MonoId {
        let id = MonoId::new(name, type_args.clone());

        // Check cache
        if self.type_cache.contains_key(&id) {
            self.stats.cache_hits += 1;
            return id;
        }

        // Add to work queue
        self.work_queue.push(MonoRequest::Type {
            name: name.to_string(),
            type_args,
            span,
        });

        id
    }

    /// Process all pending work
    pub fn process_work_queue<F, T>(
        &mut self,
        mut get_function: F,
        mut get_type: T,
    ) -> Result<(), MonoError>
    where
        F: FnMut(&str) -> Option<GenericFunction>,
        T: FnMut(&str) -> Option<GenericType>,
    {
        while let Some(request) = self.work_queue.pop() {
            self.stats.work_items_processed += 1;

            match request {
                MonoRequest::Function {
                    name,
                    type_args,
                    span,
                } => {
                    let id = MonoId::new(&name, type_args.clone());

                    if self.function_cache.contains_key(&id) {
                        continue;
                    }

                    let generic =
                        get_function(&name).ok_or_else(|| MonoError::FunctionNotFound {
                            name: name.clone(),
                            span,
                        })?;

                    // Check type argument count
                    if generic.type_params.len() != type_args.len() {
                        return Err(MonoError::WrongTypeArgCount {
                            name: name.clone(),
                            expected: generic.type_params.len(),
                            found: type_args.len(),
                            span,
                        });
                    }

                    // Create substitution context
                    let mut ctx = GenericContext::new();
                    for (param, arg) in generic.type_params.iter().zip(type_args.iter()) {
                        let var_id = ctx.add_type_var(&param.name, span);
                        ctx.substitute(var_id, arg.clone());
                    }

                    // Substitute types
                    let param_types: Vec<Type> =
                        generic.param_types.iter().map(|t| ctx.apply(t)).collect();
                    let return_type = ctx.apply(&generic.return_type);

                    // Create monomorphized function
                    let mono = MonomorphizedFunction {
                        id: id.clone(),
                        mangled_name: id.mangled_name(),
                        original_name: name.clone(),
                        type_args: type_args.clone(),
                        param_types,
                        return_type,
                        body: generic.body.clone(),
                        span: generic.span,
                    };

                    self.function_cache.insert(id, mono);
                    self.stats.functions_mono += 1;
                }
                MonoRequest::Type {
                    name,
                    type_args,
                    span,
                } => {
                    let id = MonoId::new(&name, type_args.clone());

                    if self.type_cache.contains_key(&id) {
                        continue;
                    }

                    let generic = get_type(&name).ok_or_else(|| MonoError::TypeNotFound {
                        name: name.clone(),
                        span,
                    })?;

                    // Check type argument count
                    if generic.type_params.len() != type_args.len() {
                        return Err(MonoError::WrongTypeArgCount {
                            name: name.clone(),
                            expected: generic.type_params.len(),
                            found: type_args.len(),
                            span,
                        });
                    }

                    // Create substitution context
                    let mut ctx = GenericContext::new();
                    for (param, arg) in generic.type_params.iter().zip(type_args.iter()) {
                        let var_id = ctx.add_type_var(&param.name, span);
                        ctx.substitute(var_id, arg.clone());
                    }

                    // Substitute field types
                    let fields: Vec<(String, Type)> = generic
                        .fields
                        .iter()
                        .map(|(name, ty)| (name.clone(), ctx.apply(ty)))
                        .collect();

                    // Create monomorphized type
                    let mono = MonomorphizedType {
                        id: id.clone(),
                        mangled_name: id.mangled_name(),
                        original_name: name.clone(),
                        type_args: type_args.clone(),
                        size: None, // Computed later
                        align: None,
                        fields,
                        span: generic.span,
                    };

                    self.type_cache.insert(id, mono);
                    self.stats.types_mono += 1;
                }
            }
        }

        Ok(())
    }

    /// Get a monomorphized function
    pub fn get_function(&self, id: &MonoId) -> Option<&MonomorphizedFunction> {
        self.function_cache.get(id)
    }

    /// Get a monomorphized type
    pub fn get_type(&self, id: &MonoId) -> Option<&MonomorphizedType> {
        self.type_cache.get(id)
    }

    /// Get all monomorphized functions
    pub fn all_functions(&self) -> impl Iterator<Item = &MonomorphizedFunction> {
        self.function_cache.values()
    }

    /// Get all monomorphized types
    pub fn all_types(&self) -> impl Iterator<Item = &MonomorphizedType> {
        self.type_cache.values()
    }

    /// Get statistics
    pub fn stats(&self) -> &MonoStats {
        &self.stats
    }
}

impl Default for Monomorphizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Generic function definition (before monomorphization)
#[derive(Debug, Clone)]
pub struct GenericFunction {
    pub name: String,
    pub type_params: Vec<TypeVariable>,
    pub param_types: Vec<Type>,
    pub return_type: Type,
    pub body: Option<Block>,
    pub span: Span,
}

/// Generic type definition (before monomorphization)
#[derive(Debug, Clone)]
pub struct GenericType {
    pub name: String,
    pub type_params: Vec<TypeVariable>,
    pub fields: Vec<(String, Type)>,
    pub span: Span,
}

// ============================================================================
// PART 4: TYPE INFERENCE INTEGRATION (Nyāya-Anumāna)
// ============================================================================

/// Constraint for type inference
#[derive(Debug, Clone)]
pub enum TypeConstraint {
    /// Two types must be equal
    Equal { t1: Type, t2: Type, span: Span },

    /// Type must implement trait
    Implements {
        ty: Type,
        trait_id: TraitId,
        span: Span,
    },

    /// Type must be a subtype (for coercion)
    Subtype { sub: Type, sup: Type, span: Span },

    /// Type variable must have certain bounds
    Bounded {
        var: TypeVarId,
        bounds: Vec<TraitBound>,
        span: Span,
    },
}

/// Constraint solver for generic type inference
pub struct ConstraintSolver {
    /// Pending constraints
    constraints: Vec<TypeConstraint>,

    /// Current context
    context: GenericContext,

    /// Errors encountered
    errors: Vec<MonoError>,
}

impl ConstraintSolver {
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            context: GenericContext::new(),
            errors: Vec::new(),
        }
    }

    pub fn with_context(context: GenericContext) -> Self {
        Self {
            constraints: Vec::new(),
            context,
            errors: Vec::new(),
        }
    }

    /// Add a constraint
    pub fn add_constraint(&mut self, constraint: TypeConstraint) {
        self.constraints.push(constraint);
    }

    /// Add equality constraint
    pub fn add_equal(&mut self, t1: Type, t2: Type, span: Span) {
        self.constraints
            .push(TypeConstraint::Equal { t1, t2, span });
    }

    /// Solve all constraints
    pub fn solve(&mut self) -> Result<&GenericContext, Vec<MonoError>> {
        while let Some(constraint) = self.constraints.pop() {
            match constraint {
                TypeConstraint::Equal { t1, t2, span } => {
                    self.unify(&t1, &t2, span)?;
                }
                TypeConstraint::Implements { ty, trait_id, span } => {
                    // Would need TraitSolver integration
                    // For now, just record the constraint
                }
                TypeConstraint::Subtype { sub, sup, span } => {
                    // Simplified: treat as equality for now
                    self.unify(&sub, &sup, span)?;
                }
                TypeConstraint::Bounded { var, bounds, span } => {
                    // Record bounds on the variable
                    if let Some(type_var) = self.context.get_by_id(var) {
                        // Would merge bounds
                    }
                }
            }
        }

        if self.errors.is_empty() {
            Ok(&self.context)
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Unify two types
    fn unify(&mut self, t1: &Type, t2: &Type, span: Span) -> Result<(), Vec<MonoError>> {
        let t1 = self.context.apply(t1);
        let t2 = self.context.apply(t2);

        match (&t1, &t2) {
            // Type variable unification
            (
                Type::Named {
                    name: n1,
                    generics: g1,
                    ..
                },
                _,
            ) if g1.is_empty() => {
                if let Some(var) = self.context.lookup(&n1.name) {
                    // Occurs check
                    if self.occurs(var.id, &t2) {
                        return Err(vec![MonoError::InfiniteType {
                            var_name: n1.name.clone(),
                            span,
                        }]);
                    }
                    self.context.substitute(var.id, t2.clone());
                    return Ok(());
                }
            }
            (
                _,
                Type::Named {
                    name: n2,
                    generics: g2,
                    ..
                },
            ) if g2.is_empty() => {
                if let Some(var) = self.context.lookup(&n2.name) {
                    if self.occurs(var.id, &t1) {
                        return Err(vec![MonoError::InfiniteType {
                            var_name: n2.name.clone(),
                            span,
                        }]);
                    }
                    self.context.substitute(var.id, t1.clone());
                    return Ok(());
                }
            }
            _ => {}
        }

        // Structural unification
        match (&t1, &t2) {
            (
                Type::Named {
                    name: n1,
                    generics: g1,
                    ..
                },
                Type::Named {
                    name: n2,
                    generics: g2,
                    ..
                },
            ) => {
                if n1.name != n2.name || g1.len() != g2.len() {
                    return Err(vec![MonoError::TypeMismatch {
                        expected: format!("{:?}", t1),
                        found: format!("{:?}", t2),
                        span,
                    }]);
                }
                for (a, b) in g1.iter().zip(g2.iter()) {
                    self.unify(a, b, span)?;
                }
            }
            (
                Type::Function {
                    params: p1,
                    return_type: r1,
                },
                Type::Function {
                    params: p2,
                    return_type: r2,
                },
            ) => {
                if p1.len() != p2.len() {
                    return Err(vec![MonoError::TypeMismatch {
                        expected: format!("{:?}", t1),
                        found: format!("{:?}", t2),
                        span,
                    }]);
                }
                for (a, b) in p1.iter().zip(p2.iter()) {
                    self.unify(a, b, span)?;
                }
                self.unify(r1, r2, span)?;
            }
            (
                Type::Array {
                    element: e1,
                    size: s1,
                },
                Type::Array {
                    element: e2,
                    size: s2,
                },
            ) => {
                if s1 != s2 {
                    return Err(vec![MonoError::TypeMismatch {
                        expected: format!("{:?}", t1),
                        found: format!("{:?}", t2),
                        span,
                    }]);
                }
                self.unify(e1, e2, span)?;
            }
            (Type::Tuple(ts1), Type::Tuple(ts2)) => {
                if ts1.len() != ts2.len() {
                    return Err(vec![MonoError::TypeMismatch {
                        expected: format!("{:?}", t1),
                        found: format!("{:?}", t2),
                        span,
                    }]);
                }
                for (a, b) in ts1.iter().zip(ts2.iter()) {
                    self.unify(a, b, span)?;
                }
            }
            (
                Type::Reference {
                    inner: i1,
                    mutable: m1,
                    ..
                },
                Type::Reference {
                    inner: i2,
                    mutable: m2,
                    ..
                },
            ) => {
                if m1 != m2 {
                    return Err(vec![MonoError::TypeMismatch {
                        expected: format!("{:?}", t1),
                        found: format!("{:?}", t2),
                        span,
                    }]);
                }
                self.unify(i1, i2, span)?;
            }
            (Type::Inferred, _) | (_, Type::Inferred) => {
                // Inferred unifies with anything
            }
            _ => {
                return Err(vec![MonoError::TypeMismatch {
                    expected: format!("{:?}", t1),
                    found: format!("{:?}", t2),
                    span,
                }]);
            }
        }

        Ok(())
    }

    /// Check if a type variable occurs in a type
    fn occurs(&self, var: TypeVarId, ty: &Type) -> bool {
        match ty {
            Type::Named { name, generics, .. } => {
                if generics.is_empty() {
                    if let Some(v) = self.context.lookup(&name.name) {
                        return v.id == var;
                    }
                }
                generics.iter().any(|g| self.occurs(var, g))
            }
            Type::Function {
                params,
                return_type,
            } => params.iter().any(|p| self.occurs(var, p)) || self.occurs(var, return_type),
            Type::Array { element, .. } => self.occurs(var, element),
            Type::Tuple(types) => types.iter().any(|t| self.occurs(var, t)),
            Type::Reference { inner, .. } => self.occurs(var, inner),
            Type::Inferred => false,
        }
    }

    /// Get the context (consuming solver)
    pub fn into_context(self) -> GenericContext {
        self.context
    }
}

impl Default for ConstraintSolver {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// PART 5: ERRORS (Doṣa - Faults)
// ============================================================================

#[derive(Debug, Clone)]
pub enum MonoError {
    /// Function not found
    FunctionNotFound { name: String, span: Span },

    /// Type not found
    TypeNotFound { name: String, span: Span },

    /// Wrong number of type arguments
    WrongTypeArgCount {
        name: String,
        expected: usize,
        found: usize,
        span: Span,
    },

    /// Type mismatch during unification
    TypeMismatch {
        expected: String,
        found: String,
        span: Span,
    },

    /// Infinite type (occurs check failed)
    InfiniteType { var_name: String, span: Span },

    /// Bound not satisfied
    BoundNotSatisfied {
        type_name: String,
        trait_name: String,
        span: Span,
    },

    /// Cannot infer type
    CannotInfer { context: String, span: Span },
}

impl fmt::Display for MonoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MonoError::FunctionNotFound { name, .. } => {
                write!(f, "कार्यक्रम अप्राप्य (Function not found): `{}`", name)
            }
            MonoError::TypeNotFound { name, .. } => {
                write!(f, "प्रकार अप्राप्य (Type not found): `{}`", name)
            }
            MonoError::WrongTypeArgCount {
                name,
                expected,
                found,
                ..
            } => {
                write!(
                    f,
                    "गलत प्रकार संख्या (Wrong type args) for `{}`: expected {}, found {}",
                    name, expected, found
                )
            }
            MonoError::TypeMismatch {
                expected, found, ..
            } => {
                write!(
                    f,
                    "प्रकार असंगति (Type mismatch): expected {}, found {}",
                    expected, found
                )
            }
            MonoError::InfiniteType { var_name, .. } => {
                write!(
                    f,
                    "अनन्त प्रकार (Infinite type): `{}` appears in its own definition",
                    var_name
                )
            }
            MonoError::BoundNotSatisfied {
                type_name,
                trait_name,
                ..
            } => {
                write!(
                    f,
                    "अधिकार असंतुष्ट (Bound not satisfied): `{}` doesn't implement `{}`",
                    type_name, trait_name
                )
            }
            MonoError::CannotInfer { context, .. } => {
                write!(f, "अनुमान असंभव (Cannot infer type): {}", context)
            }
        }
    }
}

impl std::error::Error for MonoError {}

// ============================================================================
// PART 6: TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::AffixSequence;

    fn make_type(name: &str) -> Type {
        Type::Named {
            name: Identifier {
                name: name.to_string(),
                affixes: AffixSequence::default(),
                span: Span::dummy(),
            },
            generics: vec![],
            affixes: AffixSequence::default(),
        }
    }

    fn make_generic_type(name: &str, args: Vec<Type>) -> Type {
        Type::Named {
            name: Identifier {
                name: name.to_string(),
                affixes: AffixSequence::default(),
                span: Span::dummy(),
            },
            generics: args,
            affixes: AffixSequence::default(),
        }
    }

    #[test]
    fn test_type_variable_creation() {
        let mut ctx = GenericContext::new();

        let id = ctx.add_type_var("T", Span::dummy());

        let var = ctx.lookup("T").unwrap();
        assert_eq!(var.name, "T");
        assert_eq!(var.id, id);
    }

    #[test]
    fn test_substitution() {
        let mut ctx = GenericContext::new();

        let id = ctx.add_type_var("T", Span::dummy());
        ctx.substitute(id, make_type("i32"));

        let result = ctx.apply(&make_type("T"));
        match result {
            Type::Named { name, .. } => assert_eq!(name.name, "i32"),
            _ => panic!("Expected named type"),
        }
    }

    #[test]
    fn test_generic_substitution() {
        let mut ctx = GenericContext::new();

        let id = ctx.add_type_var("T", Span::dummy());
        ctx.substitute(id, make_type("String"));

        // Apply to Vec<T>
        let vec_t = make_generic_type("Vec", vec![make_type("T")]);
        let result = ctx.apply(&vec_t);

        match result {
            Type::Named { name, generics, .. } => {
                assert_eq!(name.name, "Vec");
                assert_eq!(generics.len(), 1);
                match &generics[0] {
                    Type::Named { name, .. } => assert_eq!(name.name, "String"),
                    _ => panic!("Expected named type"),
                }
            }
            _ => panic!("Expected named type"),
        }
    }

    #[test]
    fn test_mono_id_mangling() {
        let id1 = MonoId::new("foo", vec![]);
        assert_eq!(id1.mangled_name(), "foo");

        let id2 = MonoId::new("bar", vec![make_type("i32")]);
        assert_eq!(id2.mangled_name(), "bar__i32");

        let id3 = MonoId::new("baz", vec![make_type("i32"), make_type("f64")]);
        assert_eq!(id3.mangled_name(), "baz__i32_f64");
    }

    #[test]
    fn test_monomorphizer_function() {
        let mut mono = Monomorphizer::new();

        let id = mono.request_function("identity", vec![make_type("i32")], Span::dummy());

        // Process with mock function provider
        let result = mono.process_work_queue(
            |name| {
                if name == "identity" {
                    Some(GenericFunction {
                        name: "identity".to_string(),
                        type_params: vec![TypeVariable::new("T", TypeVarId(1), Span::dummy())],
                        param_types: vec![make_type("T")],
                        return_type: make_type("T"),
                        body: None,
                        span: Span::dummy(),
                    })
                } else {
                    None
                }
            },
            |_| None,
        );

        assert!(result.is_ok());

        let func = mono.get_function(&id).unwrap();
        assert_eq!(func.mangled_name, "identity__i32");

        // Check param type was substituted
        match &func.param_types[0] {
            Type::Named { name, .. } => assert_eq!(name.name, "i32"),
            _ => panic!("Expected named type"),
        }
    }

    #[test]
    fn test_constraint_solver_basic() {
        let mut ctx = GenericContext::new();
        let id = ctx.add_type_var("T", Span::dummy());

        let mut solver = ConstraintSolver::with_context(ctx);
        solver.add_equal(make_type("T"), make_type("i32"), Span::dummy());

        let result = solver.solve();
        assert!(result.is_ok());

        let ctx = result.unwrap();
        let subst = ctx.get_substitution(id).unwrap();
        match subst {
            Type::Named { name, .. } => assert_eq!(name.name, "i32"),
            _ => panic!("Expected named type"),
        }
    }

    #[test]
    fn test_constraint_solver_chain() {
        let mut ctx = GenericContext::new();
        let t_id = ctx.add_type_var("T", Span::dummy());
        let u_id = ctx.add_type_var("U", Span::dummy());

        let mut solver = ConstraintSolver::with_context(ctx);

        // T = U, U = i32 => T = i32
        solver.add_equal(make_type("T"), make_type("U"), Span::dummy());
        solver.add_equal(make_type("U"), make_type("i32"), Span::dummy());

        let result = solver.solve();
        assert!(result.is_ok());
    }

    #[test]
    fn test_occurs_check() {
        let mut ctx = GenericContext::new();
        ctx.add_type_var("T", Span::dummy());

        let mut solver = ConstraintSolver::with_context(ctx);

        // T = Vec<T> should fail (infinite type)
        solver.add_equal(
            make_type("T"),
            make_generic_type("Vec", vec![make_type("T")]),
            Span::dummy(),
        );

        let result = solver.solve();
        assert!(result.is_err());
    }

    #[test]
    fn test_variance() {
        let var = TypeVariable::new("T", TypeVarId(1), Span::dummy());
        assert_eq!(var.variance, Variance::Invariant);
    }
}
