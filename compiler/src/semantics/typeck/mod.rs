//! Type Checker Module (Prakāra Parīkṣaka)
//!
//! This module implements type checking using the Nyāya 4-pramāṇa inference:
//! 1. Pratyakṣa (प्रत्यक्ष) - Direct perception (explicit annotation) - 100% certain
//! 2. Anumāna (अनुमान) - Logical inference (deduction) - 95% certain
//! 3. Śabda (शब्द) - Testimony (documentation/contract) - 90% certain
//! 4. Upamāna (उपमान) - Comparison (pattern matching) - 85% certain
//!
//! ## Module Structure
//!
//! - `types` - Core type definitions (ResolvedType, TypeVar, TypeInfo)
//! - `pramana` - Nyāya epistemology (certainty levels, inference sources)
//! - `inference` - Type inference engine (unification, substitution)
//! - `constraints` - Constraint solving system using Nyāya Pañcāvayava
//! - `generics` - Polymorphism using Vaiśeṣika Sāmānya-Viśeṣa
//! - `errors` - Type error definitions
//!
//! ## Philosophy
//!
//! The inference algorithm is based on Hindley-Milner Algorithm W with
//! adaptations for the Nyāya philosophical framework. This provides:
//! - Principled uncertainty tracking
//! - Helpful diagnostics with philosophical context
//! - Clear guidance for users when types can't be inferred
//!
//! ## Nyāya Constraint Solving
//!
//! Constraints follow the Pañcāvayava (five-part syllogism):
//! 1. Pratijñā (Thesis) - The type equation to prove
//! 2. Hetu (Reason) - Why the constraint was generated
//! 3. Udāharaṇa (Example) - Universal typing rule applied
//! 4. Upanaya (Application) - How rule applies to this case
//! 5. Nigamana (Conclusion) - Solved type assignment
//!
//! ## Vaiśeṣika Polymorphism
//!
//! Type polymorphism uses Vaiśeṣika ontology:
//! - Sāmānya (Universal): Type schemes (∀α. T)
//! - Viśeṣa (Particular): Concrete type instances
//! - Samavāya (Inherence): The instantiation relation

pub mod checker;
pub mod constraints;
pub mod context;
pub mod errors;
pub mod generics;
pub mod inference;
pub mod lifetimes;
pub mod pramana;
pub mod types;

// Re-export core types for convenience
pub use checker::{CheckerConfig, DiagnosticState, TypeJudgment, UnifiedChecker};
pub use constraints::{
    Constraint, ConstraintError, ConstraintErrorKind, ConstraintKind, ConstraintReason,
    ConstraintSolver, InferenceResult,
};
pub use context::{register_builtins, ContextSnapshot, Scope, ScopeKind, TypeContext};
pub use errors::TypeError;
pub use generics::{
    free_type_vars, LetPolymorphism, PolymorphismEngine, TypeEnvironment, TypeRank, TypeScheme,
};
pub use inference::{TypeInference, UnificationError};
pub use lifetimes::{
    LifetimeError, LifetimeInference, LifetimeTypeUnifier, LifetimeUnificationError,
    OutlivesConstraint, OutlivesReason, RegionVar, TypeWithLifetime,
};
pub use pramana::Pramana;
pub use types::{
    FunctionSig, MethodSig, ResolvedType, SelfType, TypeBodyResolved, TypeDefInfo, TypeInfo,
    TypeVar,
};

// Note: TypeChecker is still in the parent typeck_impl.rs file
// UnifiedChecker in checker.rs is the new modular integration layer
