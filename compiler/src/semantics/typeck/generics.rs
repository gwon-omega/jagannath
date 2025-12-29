//! Polymorphism System (Sāmānya-Viśeṣa Vyavasthā)
//!
//! Implements parametric polymorphism using the Nyāya-Vaiśeṣika philosophy.
//!
//! ## Philosophical Foundation
//!
//! The Vaiśeṣika school's ontology provides the framework:
//!
//! | Sanskrit | Meaning | Type System Mapping |
//! |----------|---------|---------------------|
//! | Sāmānya | Universal | Type scheme (∀α. T) |
//! | Viśeṣa | Particular | Concrete type instance |
//! | Samavāya | Inherence | Instantiation relation |
//! | Dravya | Substance | Value carrying type |
//! | Guṇa | Quality | Type property/constraint |
//!
//! ## Operations
//!
//! - **Sāmānyīkaraṇa** (Generalization): Finding the universal form
//! - **Viśeṣīkaraṇa** (Instantiation): Deriving particular from universal
//! - **Anumāna** (Inference): Determining type variables to quantify

use super::types::{ResolvedType, TypeVar};
use std::collections::{HashMap, HashSet};

// ============================================================================
// Type Scheme (Sāmānya - Universal Form)
// ============================================================================

/// A type scheme representing universally quantified type (Sāmānya Prakāra)
///
/// In Vaiśeṣika philosophy, sāmānya is the universal that inheres in
/// particulars. Similarly, a type scheme is a universal form from which
/// particular types can be instantiated.
///
/// Example: `∀α. α → α` is the type scheme for identity function
#[derive(Debug, Clone, PartialEq)]
pub struct TypeScheme {
    /// Quantified type variables (bound variables)
    /// In Sanskrit: Baddha Anirdhārita (बद्ध अनिर्धारित)
    pub quantified: Vec<TypeVar>,
    /// The body type containing quantified variables
    /// In Sanskrit: Śarīra Prakāra (शरीर प्रकार)
    pub body: ResolvedType,
}

impl TypeScheme {
    /// Create a monomorphic type scheme (no quantifiers)
    pub fn mono(ty: ResolvedType) -> Self {
        Self {
            quantified: Vec::new(),
            body: ty,
        }
    }

    /// Create a polymorphic type scheme
    pub fn poly(quantified: Vec<TypeVar>, body: ResolvedType) -> Self {
        Self { quantified, body }
    }

    /// Check if this scheme is monomorphic (no quantifiers)
    pub fn is_mono(&self) -> bool {
        self.quantified.is_empty()
    }

    /// Check if this scheme is polymorphic (has quantifiers)
    pub fn is_poly(&self) -> bool {
        !self.quantified.is_empty()
    }

    /// Get the arity (number of type parameters)
    pub fn arity(&self) -> usize {
        self.quantified.len()
    }
}

impl std::fmt::Display for TypeScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.quantified.is_empty() {
            write!(f, "{}", self.body)
        } else {
            write!(f, "∀")?;
            for (i, var) in self.quantified.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", var)?;
            }
            write!(f, ". {}", self.body)
        }
    }
}

// ============================================================================
// Type Environment (Prakāra Parisara)
// ============================================================================

/// Type environment mapping names to type schemes
///
/// In Nyāya, the context of knowledge (jñāna-viṣaya) determines
/// how we interpret symbols. The type environment provides this context.
#[derive(Debug, Clone, Default)]
pub struct TypeEnvironment {
    /// Bindings from names to type schemes
    bindings: HashMap<String, TypeScheme>,
    /// Free type variables in this environment
    free_vars: HashSet<TypeVar>,
}

impl TypeEnvironment {
    pub fn new() -> Self {
        Self::default()
    }

    /// Bind a name to a type scheme
    pub fn bind(&mut self, name: String, scheme: TypeScheme) {
        // Update free vars
        for var in &scheme.quantified {
            self.free_vars.remove(var);
        }
        self.free_vars.extend(free_type_vars(&scheme.body));
        self.bindings.insert(name, scheme);
    }

    /// Look up a name in the environment
    pub fn lookup(&self, name: &str) -> Option<&TypeScheme> {
        self.bindings.get(name)
    }

    /// Get free type variables in the environment
    pub fn free_vars(&self) -> &HashSet<TypeVar> {
        &self.free_vars
    }

    /// Remove a binding
    pub fn unbind(&mut self, name: &str) {
        self.bindings.remove(name);
    }

    /// Extend with another environment
    pub fn extend(&mut self, other: TypeEnvironment) {
        for (name, scheme) in other.bindings {
            self.bind(name, scheme);
        }
    }
}

// ============================================================================
// Polymorphism Operations
// ============================================================================

/// Polymorphism engine (Sāmānya-Viśeṣa Yantra)
pub struct PolymorphismEngine {
    /// Next fresh type variable ID
    next_var: usize,
    /// Substitution map for instantiation
    substitutions: HashMap<TypeVar, ResolvedType>,
}

impl PolymorphismEngine {
    pub fn new() -> Self {
        Self {
            next_var: 0,
            substitutions: HashMap::new(),
        }
    }

    /// Generate a fresh type variable
    pub fn fresh_var(&mut self) -> TypeVar {
        let var = TypeVar(self.next_var);
        self.next_var += 1;
        var
    }

    /// Generate a fresh type variable as ResolvedType
    pub fn fresh_type_var(&mut self) -> ResolvedType {
        ResolvedType::TypeVar(self.fresh_var())
    }

    // ========================================================================
    // Sāmānyīkaraṇa (Generalization)
    // ========================================================================

    /// Generalize a type to a type scheme (Sāmānyīkaraṇa)
    ///
    /// Finds all free type variables in `ty` that are not free in `env`,
    /// and quantifies over them. This creates a universal (sāmānya) form.
    ///
    /// In Vaiśeṣika terms: extracting the sāmānya (universal) from
    /// the viśeṣa (particular) by identifying what varies.
    ///
    /// Example:
    /// - ty = τ0 → τ0
    /// - env has no free vars
    /// - Result: ∀τ0. τ0 → τ0
    pub fn generalize(&self, ty: &ResolvedType, env: &TypeEnvironment) -> TypeScheme {
        let ty_free = free_type_vars(ty);
        let env_free = env.free_vars();

        // Quantify over type vars free in ty but not in env
        let quantified: Vec<TypeVar> = ty_free
            .difference(env_free)
            .copied()
            .collect();

        if quantified.is_empty() {
            TypeScheme::mono(ty.clone())
        } else {
            TypeScheme::poly(quantified, ty.clone())
        }
    }

    // ========================================================================
    // Viśeṣīkaraṇa (Instantiation)
    // ========================================================================

    /// Instantiate a type scheme with fresh variables (Viśeṣīkaraṇa)
    ///
    /// Replaces all quantified (bound) type variables with fresh ones,
    /// creating a particular (viśeṣa) instance of the universal (sāmānya).
    ///
    /// In Vaiśeṣika terms: the samavāya (inherence) relation connects
    /// the sāmānya to a new viśeṣa.
    ///
    /// Example:
    /// - scheme = ∀τ0. τ0 → τ0
    /// - Result = τ5 → τ5 (with τ5 being a fresh variable)
    pub fn instantiate(&mut self, scheme: &TypeScheme) -> ResolvedType {
        if scheme.is_mono() {
            return scheme.body.clone();
        }

        // Create fresh variables for each quantified variable
        let substitution: HashMap<TypeVar, ResolvedType> = scheme
            .quantified
            .iter()
            .map(|&v| (v, self.fresh_type_var()))
            .collect();

        // Apply substitution to body
        self.apply_substitution(&scheme.body, &substitution)
    }

    /// Apply a substitution to a type
    fn apply_substitution(
        &self,
        ty: &ResolvedType,
        subst: &HashMap<TypeVar, ResolvedType>,
    ) -> ResolvedType {
        match ty {
            ResolvedType::TypeVar(var) => {
                subst.get(var).cloned().unwrap_or_else(|| ty.clone())
            }
            ResolvedType::Function { params, return_type } => ResolvedType::Function {
                params: params
                    .iter()
                    .map(|p| self.apply_substitution(p, subst))
                    .collect(),
                return_type: Box::new(self.apply_substitution(return_type, subst)),
            },
            ResolvedType::Reference { inner, mutable, lifetime } => ResolvedType::Reference {
                inner: Box::new(self.apply_substitution(inner, subst)),
                mutable: *mutable,
                lifetime: *lifetime,
            },
            ResolvedType::Array { element, size } => ResolvedType::Array {
                element: Box::new(self.apply_substitution(element, subst)),
                size: *size,
            },
            ResolvedType::Tuple(elems) => ResolvedType::Tuple(
                elems
                    .iter()
                    .map(|e| self.apply_substitution(e, subst))
                    .collect(),
            ),
            ResolvedType::Named { name, generics } => ResolvedType::Named {
                name: name.clone(),
                generics: generics
                    .iter()
                    .map(|g| self.apply_substitution(g, subst))
                    .collect(),
            },
            // Primitives and special types don't need substitution
            _ => ty.clone(),
        }
    }

    // ========================================================================
    // Type Variable Analysis
    // ========================================================================

    /// Check if a type contains a specific type variable
    pub fn contains_var(&self, ty: &ResolvedType, var: TypeVar) -> bool {
        free_type_vars(ty).contains(&var)
    }

    /// Rename type variables to canonical form (α, β, γ, ...)
    pub fn canonicalize(&mut self, scheme: &TypeScheme) -> TypeScheme {
        let mut subst = HashMap::new();
        let mut new_quantified = Vec::new();

        for (i, &old_var) in scheme.quantified.iter().enumerate() {
            let new_var = TypeVar(i);
            subst.insert(old_var, ResolvedType::TypeVar(new_var));
            new_quantified.push(new_var);
        }

        TypeScheme {
            quantified: new_quantified,
            body: self.apply_substitution(&scheme.body, &subst),
        }
    }
}

impl Default for PolymorphismEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Free Type Variables (Mukta Anirdhārita)
// ============================================================================

/// Find all free type variables in a type
///
/// In Sanskrit: Mukta (मुक्त) means free/liberated.
/// These are the type variables that are not bound by any quantifier.
pub fn free_type_vars(ty: &ResolvedType) -> HashSet<TypeVar> {
    let mut vars = HashSet::new();
    collect_free_vars(ty, &mut vars);
    vars
}

fn collect_free_vars(ty: &ResolvedType, vars: &mut HashSet<TypeVar>) {
    match ty {
        ResolvedType::TypeVar(var) => {
            vars.insert(*var);
        }
        ResolvedType::Function { params, return_type } => {
            for p in params {
                collect_free_vars(p, vars);
            }
            collect_free_vars(return_type, vars);
        }
        ResolvedType::Reference { inner, .. } => {
            collect_free_vars(inner, vars);
        }
        ResolvedType::Array { element, .. } => {
            collect_free_vars(element, vars);
        }
        ResolvedType::Tuple(elems) => {
            for e in elems {
                collect_free_vars(e, vars);
            }
        }
        ResolvedType::Named { generics, .. } => {
            for g in generics {
                collect_free_vars(g, vars);
            }
        }
        // Primitives have no free variables
        _ => {}
    }
}

// ============================================================================
// Let-Polymorphism (Vara Bahurupatā)
// ============================================================================

/// Let-polymorphism context for handling `let` bindings
///
/// In Hindley-Milner, let-polymorphism allows:
/// ```text
/// let id = λx. x in (id 5, id "hello")
/// ```
/// Here `id` can be used at different types because it's generalized.
///
/// The Sanskrit term Bahurupatā (बहुरूपता) means "many-formedness"
/// - the ability to take multiple forms.
pub struct LetPolymorphism {
    /// The polymorphism engine
    engine: PolymorphismEngine,
    /// Type environment with let-bound variables
    env: TypeEnvironment,
}

impl LetPolymorphism {
    pub fn new() -> Self {
        Self {
            engine: PolymorphismEngine::new(),
            env: TypeEnvironment::new(),
        }
    }

    /// Process a let binding (Vara Bandhana)
    ///
    /// Given `let x = e`, we:
    /// 1. Infer the type of e
    /// 2. Generalize to get a type scheme
    /// 3. Bind x to the scheme in the environment
    pub fn process_let_binding(
        &mut self,
        name: String,
        inferred_type: ResolvedType,
    ) -> TypeScheme {
        let scheme = self.engine.generalize(&inferred_type, &self.env);
        self.env.bind(name, scheme.clone());
        scheme
    }

    /// Use a let-bound variable (Vara Prayoga)
    ///
    /// When using a let-bound variable, we instantiate its scheme
    /// with fresh type variables.
    pub fn use_variable(&mut self, name: &str) -> Option<ResolvedType> {
        self.env
            .lookup(name)
            .map(|scheme| self.engine.instantiate(scheme))
    }

    /// Get the type scheme for a variable
    pub fn get_scheme(&self, name: &str) -> Option<&TypeScheme> {
        self.env.lookup(name)
    }

    /// Get a fresh type variable
    pub fn fresh_var(&mut self) -> ResolvedType {
        self.engine.fresh_type_var()
    }
}

impl Default for LetPolymorphism {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Higher-Rank Polymorphism (Ucca Śreṇī Bahurupatā)
// ============================================================================

/// Rank of a type (how deeply nested are quantifiers?)
///
/// - Rank 0: No quantifiers (monomorphic)
/// - Rank 1: Quantifiers at top level only (∀α. T where T is rank 0)
/// - Rank 2: Quantifiers can appear in function arguments
/// - Rank n: Quantifiers can nest up to depth n
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeRank(pub u8);

impl TypeRank {
    /// Monomorphic type (no quantifiers)
    pub const MONO: Self = Self(0);
    /// Standard ML/Hindley-Milner rank
    pub const RANK1: Self = Self(1);
    /// Rank-2 polymorphism (System F2)
    pub const RANK2: Self = Self(2);

    /// Calculate the rank of a type scheme
    pub fn of_scheme(scheme: &TypeScheme) -> Self {
        if scheme.is_mono() {
            Self::MONO
        } else {
            Self(1 + Self::of_type(&scheme.body).0)
        }
    }

    /// Calculate the rank of a type (counting nested quantifiers)
    pub fn of_type(ty: &ResolvedType) -> Self {
        match ty {
            ResolvedType::Function { params, return_type } => {
                let param_ranks: Vec<u8> = params.iter().map(|p| Self::of_type(p).0).collect();
                let ret_rank = Self::of_type(return_type).0;
                let max_param = param_ranks.into_iter().max().unwrap_or(0);
                Self(max_param.max(ret_rank))
            }
            ResolvedType::Reference { inner, .. } => Self::of_type(inner),
            ResolvedType::Array { element, .. } => Self::of_type(element),
            ResolvedType::Tuple(elems) => {
                Self(elems.iter().map(|e| Self::of_type(e).0).max().unwrap_or(0))
            }
            ResolvedType::Named { generics, .. } => {
                Self(generics.iter().map(|g| Self::of_type(g).0).max().unwrap_or(0))
            }
            _ => Self::MONO,
        }
    }
}

// ============================================================================
// Semantic Terminology (Arthaśāstra Paribhāṣā)
// ============================================================================

/// Sanskrit terms for polymorphism concepts
pub mod terminology {
    /// Sāmānya (सामान्य) - Universal/General
    /// The abstract form that can be instantiated to many particulars
    pub const UNIVERSAL: &str = "सामान्य";

    /// Viśeṣa (विशेष) - Particular/Specific
    /// A concrete instance of a universal
    pub const PARTICULAR: &str = "विशेष";

    /// Samavāya (समवाय) - Inherence
    /// The inseparable relation between universal and particular
    pub const INHERENCE: &str = "समवाय";

    /// Dravya (द्रव्य) - Substance
    /// The carrier of qualities (value types)
    pub const SUBSTANCE: &str = "द्रव्य";

    /// Guṇa (गुण) - Quality
    /// Properties that inhere in substances (type constraints)
    pub const QUALITY: &str = "गुण";

    /// Sāmānyīkaraṇa (सामान्यीकरण) - Generalization
    /// The process of creating a universal from particular
    pub const GENERALIZATION: &str = "सामान्यीकरण";

    /// Viśeṣīkaraṇa (विशेषीकरण) - Instantiation/Specialization
    /// The process of deriving a particular from universal
    pub const INSTANTIATION: &str = "विशेषीकरण";

    /// Bahurupatā (बहुरूपता) - Polymorphism
    /// Literally "many-formedness"
    pub const POLYMORPHISM: &str = "बहुरूपता";

    /// Mukta (मुक्त) - Free
    /// As in free type variables (not bound)
    pub const FREE: &str = "मुक्त";

    /// Baddha (बद्ध) - Bound
    /// As in bound/quantified type variables
    pub const BOUND: &str = "बद्ध";
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_scheme_mono() {
        let scheme = TypeScheme::mono(ResolvedType::Int32);
        assert!(scheme.is_mono());
        assert!(!scheme.is_poly());
        assert_eq!(scheme.arity(), 0);
    }

    #[test]
    fn test_type_scheme_poly() {
        let var = TypeVar(0);
        let scheme = TypeScheme::poly(
            vec![var],
            ResolvedType::Function {
                params: vec![ResolvedType::TypeVar(var)],
                return_type: Box::new(ResolvedType::TypeVar(var)),
            },
        );
        assert!(scheme.is_poly());
        assert_eq!(scheme.arity(), 1);
    }

    #[test]
    fn test_free_type_vars() {
        let var0 = TypeVar(0);
        let var1 = TypeVar(1);

        let ty = ResolvedType::Function {
            params: vec![ResolvedType::TypeVar(var0)],
            return_type: Box::new(ResolvedType::TypeVar(var1)),
        };

        let free = free_type_vars(&ty);
        assert!(free.contains(&var0));
        assert!(free.contains(&var1));
        assert_eq!(free.len(), 2);
    }

    #[test]
    fn test_generalization() {
        let engine = PolymorphismEngine::new();
        let env = TypeEnvironment::new();

        let var = TypeVar(0);
        let ty = ResolvedType::Function {
            params: vec![ResolvedType::TypeVar(var)],
            return_type: Box::new(ResolvedType::TypeVar(var)),
        };

        let scheme = engine.generalize(&ty, &env);
        assert!(scheme.is_poly());
        assert!(scheme.quantified.contains(&var));
    }

    #[test]
    fn test_generalization_with_env_vars() {
        let engine = PolymorphismEngine::new();
        let mut env = TypeEnvironment::new();

        let var0 = TypeVar(0);
        let var1 = TypeVar(1);

        // var0 is free in the environment
        env.bind("x".to_string(), TypeScheme::mono(ResolvedType::TypeVar(var0)));

        // Type uses both var0 and var1
        let ty = ResolvedType::Tuple(vec![
            ResolvedType::TypeVar(var0),
            ResolvedType::TypeVar(var1),
        ]);

        let scheme = engine.generalize(&ty, &env);
        // Only var1 should be quantified (var0 is free in env)
        assert!(scheme.quantified.contains(&var1));
        assert!(!scheme.quantified.contains(&var0));
    }

    #[test]
    fn test_instantiation() {
        let mut engine = PolymorphismEngine::new();

        let var = TypeVar(0);
        let scheme = TypeScheme::poly(
            vec![var],
            ResolvedType::Function {
                params: vec![ResolvedType::TypeVar(var)],
                return_type: Box::new(ResolvedType::TypeVar(var)),
            },
        );

        let inst1 = engine.instantiate(&scheme);
        let inst2 = engine.instantiate(&scheme);

        // Each instantiation should produce different fresh variables
        assert_ne!(inst1, inst2);
    }

    #[test]
    fn test_let_polymorphism() {
        let mut let_poly = LetPolymorphism::new();

        // let id = λx. x (identity function)
        let var = let_poly.fresh_var();
        let id_type = ResolvedType::Function {
            params: vec![var.clone()],
            return_type: Box::new(var),
        };

        let scheme = let_poly.process_let_binding("id".to_string(), id_type);
        assert!(scheme.is_poly());

        // Using id twice should give different instances
        let use1 = let_poly.use_variable("id").unwrap();
        let use2 = let_poly.use_variable("id").unwrap();
        assert_ne!(use1, use2);
    }

    #[test]
    fn test_type_rank() {
        // Monomorphic type
        assert_eq!(TypeRank::of_type(&ResolvedType::Int32), TypeRank::MONO);

        // Simple function type (still rank 0)
        let simple_fn = ResolvedType::Function {
            params: vec![ResolvedType::Int32],
            return_type: Box::new(ResolvedType::Bool),
        };
        assert_eq!(TypeRank::of_type(&simple_fn), TypeRank::MONO);
    }

    #[test]
    fn test_type_environment() {
        let mut env = TypeEnvironment::new();

        let scheme = TypeScheme::mono(ResolvedType::Int32);
        env.bind("x".to_string(), scheme);

        assert!(env.lookup("x").is_some());
        assert!(env.lookup("y").is_none());

        env.unbind("x");
        assert!(env.lookup("x").is_none());
    }

    #[test]
    fn test_scheme_display() {
        let var = TypeVar(0);
        let mono = TypeScheme::mono(ResolvedType::Int32);
        let poly = TypeScheme::poly(
            vec![var],
            ResolvedType::TypeVar(var),
        );

        assert_eq!(format!("{}", mono), "i32");
        assert!(format!("{}", poly).contains("∀"));
    }

    #[test]
    fn test_canonicalize() {
        let mut engine = PolymorphismEngine::new();

        // Create scheme with weird variable numbers
        let scheme = TypeScheme::poly(
            vec![TypeVar(42), TypeVar(99)],
            ResolvedType::Tuple(vec![
                ResolvedType::TypeVar(TypeVar(42)),
                ResolvedType::TypeVar(TypeVar(99)),
            ]),
        );

        let canonical = engine.canonicalize(&scheme);
        assert_eq!(canonical.quantified, vec![TypeVar(0), TypeVar(1)]);
    }
}
