//! Type Inference Engine (Prakāra Anumāna Yantra)
//!
//! Implements Algorithm W (Hindley-Milner) with adaptations for the
//! Nyāya philosophical framework. This module handles:
//! - Type variable generation
//! - Substitution application
//! - Unification (Robinson's algorithm)
//! - Occurs check (prevents infinite types)
//! - Generalization and instantiation

use super::types::{ResolvedType, TypeVar};
use std::collections::HashMap;

// ============================================================================
// Type Inference Engine
// ============================================================================

/// Type inference engine implementing Algorithm W with Nyāya philosophy
pub struct TypeInference {
    /// Substitution map: TypeVar -> ResolvedType
    substitutions: HashMap<TypeVar, ResolvedType>,
    /// Next type variable ID
    next_var: usize,
}

impl TypeInference {
    pub fn new() -> Self {
        Self {
            substitutions: HashMap::new(),
            next_var: 0,
        }
    }

    /// Create a fresh type variable (Nūtana Anirdhārita)
    ///
    /// Each type variable is given a unique identifier τn.
    /// Fresh variables are created during inference to represent
    /// unknown types that will be resolved through unification.
    pub fn fresh_type_var(&mut self) -> ResolvedType {
        let var = TypeVar(self.next_var);
        self.next_var += 1;
        ResolvedType::TypeVar(var)
    }

    /// Apply substitutions to a type (Pratiyojana)
    ///
    /// Recursively replaces all type variables with their substituted
    /// values. This is the "chase" operation in unification.
    pub fn apply(&self, ty: &ResolvedType) -> ResolvedType {
        match ty {
            ResolvedType::TypeVar(var) => {
                if let Some(subst) = self.substitutions.get(var) {
                    // Recursively apply substitutions
                    self.apply(subst)
                } else {
                    ty.clone()
                }
            }
            ResolvedType::Function {
                params,
                return_type,
            } => ResolvedType::Function {
                params: params.iter().map(|p| self.apply(p)).collect(),
                return_type: Box::new(self.apply(return_type)),
            },
            ResolvedType::Reference {
                inner,
                mutable,
                lifetime,
            } => ResolvedType::Reference {
                inner: Box::new(self.apply(inner)),
                mutable: *mutable,
                lifetime: *lifetime,
            },
            ResolvedType::Array { element, size } => ResolvedType::Array {
                element: Box::new(self.apply(element)),
                size: *size,
            },
            ResolvedType::Tuple(elems) => {
                ResolvedType::Tuple(elems.iter().map(|e| self.apply(e)).collect())
            }
            ResolvedType::Named { name, generics } => ResolvedType::Named {
                name: name.clone(),
                generics: generics.iter().map(|g| self.apply(g)).collect(),
            },
            // Primitive types don't need substitution
            _ => ty.clone(),
        }
    }

    /// Unify two types (Ekīkaraṇa)
    ///
    /// Implementation of Robinson's unification algorithm adapted for Jagannath.
    /// Returns Ok(()) if unification succeeds, Err with details otherwise.
    ///
    /// The unification process follows the Nyāya principle of establishing
    /// identity (sāmānya) between two potentially different appearances.
    pub fn unify(&mut self, t1: &ResolvedType, t2: &ResolvedType) -> Result<(), UnificationError> {
        let t1 = self.apply(t1);
        let t2 = self.apply(t2);

        match (&t1, &t2) {
            // Same type - trivially unifies
            (a, b) if a == b => Ok(()),

            // Type variable unification
            (ResolvedType::TypeVar(var), ty) | (ty, ResolvedType::TypeVar(var)) => {
                // Occurs check (prevents infinite types)
                if self.occurs_in(*var, ty) {
                    return Err(UnificationError::OccursCheck {
                        var: *var,
                        ty: ty.clone(),
                    });
                }
                // Add substitution
                self.substitutions.insert(*var, ty.clone());
                Ok(())
            }

            // Function type unification
            (
                ResolvedType::Function {
                    params: p1,
                    return_type: r1,
                },
                ResolvedType::Function {
                    params: p2,
                    return_type: r2,
                },
            ) => {
                if p1.len() != p2.len() {
                    return Err(UnificationError::ArityMismatch {
                        expected: p1.len(),
                        found: p2.len(),
                    });
                }
                for (a, b) in p1.iter().zip(p2.iter()) {
                    self.unify(a, b)?;
                }
                self.unify(r1, r2)
            }

            // Array type unification
            (
                ResolvedType::Array {
                    element: e1,
                    size: s1,
                },
                ResolvedType::Array {
                    element: e2,
                    size: s2,
                },
            ) => {
                // Size must match if both specified
                if let (Some(sz1), Some(sz2)) = (s1, s2) {
                    if sz1 != sz2 {
                        return Err(UnificationError::ArraySizeMismatch {
                            expected: *sz1,
                            found: *sz2,
                        });
                    }
                }
                self.unify(e1, e2)
            }

            // Tuple type unification
            (ResolvedType::Tuple(elems1), ResolvedType::Tuple(elems2)) => {
                if elems1.len() != elems2.len() {
                    return Err(UnificationError::TupleSizeMismatch {
                        expected: elems1.len(),
                        found: elems2.len(),
                    });
                }
                for (a, b) in elems1.iter().zip(elems2.iter()) {
                    self.unify(a, b)?;
                }
                Ok(())
            }

            // Reference type unification
            (
                ResolvedType::Reference {
                    inner: i1,
                    mutable: m1,
                    ..
                },
                ResolvedType::Reference {
                    inner: i2,
                    mutable: m2,
                    ..
                },
            ) => {
                if m1 != m2 {
                    return Err(UnificationError::MutabilityMismatch {
                        expected_mutable: *m1,
                    });
                }
                self.unify(i1, i2)
            }

            // Named type unification
            (
                ResolvedType::Named {
                    name: n1,
                    generics: g1,
                },
                ResolvedType::Named {
                    name: n2,
                    generics: g2,
                },
            ) => {
                if n1 != n2 {
                    return Err(UnificationError::TypeMismatch {
                        expected: t1.clone(),
                        found: t2.clone(),
                    });
                }
                if g1.len() != g2.len() {
                    return Err(UnificationError::GenericArityMismatch {
                        ty: n1.clone(),
                        expected: g1.len(),
                        found: g2.len(),
                    });
                }
                for (a, b) in g1.iter().zip(g2.iter()) {
                    self.unify(a, b)?;
                }
                Ok(())
            }

            // Unknown can unify with anything
            (ResolvedType::Unknown, _) | (_, ResolvedType::Unknown) => Ok(()),

            // Error type propagates
            (ResolvedType::Error, _) | (_, ResolvedType::Error) => Ok(()),

            // Never type (diverging) unifies with anything
            (ResolvedType::Never, _) | (_, ResolvedType::Never) => Ok(()),

            // Type mismatch
            _ => Err(UnificationError::TypeMismatch {
                expected: t1.clone(),
                found: t2.clone(),
            }),
        }
    }

    /// Occurs check - prevents infinite recursive types
    ///
    /// This check prevents creating infinite types like τ = List<τ>.
    /// In Nyāya terms, this prevents circular definitions (anyonyāśraya).
    pub fn occurs_in(&self, var: TypeVar, ty: &ResolvedType) -> bool {
        let ty = self.apply(ty);
        match ty {
            ResolvedType::TypeVar(v) => v == var,
            ResolvedType::Function {
                params,
                return_type,
            } => params.iter().any(|p| self.occurs_in(var, p)) || self.occurs_in(var, &return_type),
            ResolvedType::Array { element, .. } => self.occurs_in(var, &element),
            ResolvedType::Tuple(elems) => elems.iter().any(|e| self.occurs_in(var, e)),
            ResolvedType::Reference { inner, .. } => self.occurs_in(var, &inner),
            ResolvedType::Named { generics, .. } => generics.iter().any(|g| self.occurs_in(var, g)),
            _ => false,
        }
    }

    /// Generalize a type to a type scheme (for let-polymorphism)
    ///
    /// Finds free type variables in ty that are not in the environment
    /// and quantifies over them. This enables parametric polymorphism.
    #[allow(unused_variables)]
    pub fn generalize(&self, ty: &ResolvedType, env_vars: &[TypeVar]) -> ResolvedType {
        // Find free type variables in ty that are not in environment
        // For now, just return the applied type
        self.apply(ty)
    }

    /// Instantiate a type scheme with fresh variables
    ///
    /// Replaces quantified type variables with fresh ones.
    /// This is used when using a polymorphic function at a specific type.
    #[allow(unused_variables)]
    pub fn instantiate(&mut self, ty: &ResolvedType) -> ResolvedType {
        // For polymorphic types, replace bound variables with fresh ones
        // For now, just clone the type
        ty.clone()
    }

    /// Get the current substitution for a type variable
    pub fn get_substitution(&self, var: TypeVar) -> Option<&ResolvedType> {
        self.substitutions.get(&var)
    }

    /// Clear all substitutions (for debugging/testing)
    pub fn clear(&mut self) {
        self.substitutions.clear();
        self.next_var = 0;
    }

    /// Get the count of type variables created
    pub fn type_var_count(&self) -> usize {
        self.next_var
    }
}

impl Default for TypeInference {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Unification Errors
// ============================================================================

/// Unification error types
#[derive(Debug, Clone)]
pub enum UnificationError {
    /// Types fundamentally don't match
    TypeMismatch {
        expected: ResolvedType,
        found: ResolvedType,
    },
    /// Occurs check failed (infinite type)
    OccursCheck { var: TypeVar, ty: ResolvedType },
    /// Function arity mismatch
    ArityMismatch { expected: usize, found: usize },
    /// Array size mismatch
    ArraySizeMismatch { expected: usize, found: usize },
    /// Tuple size mismatch
    TupleSizeMismatch { expected: usize, found: usize },
    /// Reference mutability mismatch
    MutabilityMismatch { expected_mutable: bool },
    /// Generic type parameter count mismatch
    GenericArityMismatch {
        ty: String,
        expected: usize,
        found: usize,
    },
}

impl std::fmt::Display for UnificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnificationError::TypeMismatch { expected, found } => {
                write!(f, "type mismatch: expected {}, found {}", expected, found)
            }
            UnificationError::OccursCheck { var, ty } => {
                write!(f, "infinite type: {} occurs in {}", var, ty)
            }
            UnificationError::ArityMismatch { expected, found } => {
                write!(
                    f,
                    "arity mismatch: expected {} parameters, found {}",
                    expected, found
                )
            }
            UnificationError::ArraySizeMismatch { expected, found } => {
                write!(
                    f,
                    "array size mismatch: expected [_; {}], found [_; {}]",
                    expected, found
                )
            }
            UnificationError::TupleSizeMismatch { expected, found } => {
                write!(
                    f,
                    "tuple size mismatch: expected {} elements, found {}",
                    expected, found
                )
            }
            UnificationError::MutabilityMismatch { expected_mutable } => {
                if *expected_mutable {
                    write!(f, "mutability mismatch: expected &mut, found &")
                } else {
                    write!(f, "mutability mismatch: expected &, found &mut")
                }
            }
            UnificationError::GenericArityMismatch {
                ty,
                expected,
                found,
            } => {
                write!(
                    f,
                    "generic arity mismatch for {}: expected {} type parameters, found {}",
                    ty, expected, found
                )
            }
        }
    }
}

impl std::error::Error for UnificationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fresh_type_var() {
        let mut inference = TypeInference::new();
        let v1 = inference.fresh_type_var();
        let v2 = inference.fresh_type_var();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_unify_same_types() {
        let mut inference = TypeInference::new();
        assert!(inference
            .unify(&ResolvedType::Int32, &ResolvedType::Int32)
            .is_ok());
    }

    #[test]
    fn test_unify_type_var() {
        let mut inference = TypeInference::new();
        let var = inference.fresh_type_var();
        assert!(inference.unify(&var, &ResolvedType::Int32).is_ok());
        assert_eq!(inference.apply(&var), ResolvedType::Int32);
    }

    #[test]
    fn test_occurs_check() {
        let mut inference = TypeInference::new();
        let var = inference.fresh_type_var();
        // Create a type that contains the variable
        let list_ty = ResolvedType::Named {
            name: "List".to_string(),
            generics: vec![var.clone()],
        };
        if let ResolvedType::TypeVar(v) = var {
            assert!(inference.occurs_in(v, &list_ty));
        }
    }

    #[test]
    fn test_unify_functions() {
        let mut inference = TypeInference::new();
        let f1 = ResolvedType::Function {
            params: vec![ResolvedType::Int32],
            return_type: Box::new(ResolvedType::Bool),
        };
        let f2 = ResolvedType::Function {
            params: vec![ResolvedType::Int32],
            return_type: Box::new(ResolvedType::Bool),
        };
        assert!(inference.unify(&f1, &f2).is_ok());
    }
}
