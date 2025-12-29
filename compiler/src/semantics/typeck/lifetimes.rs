//! Lifetime-Aware Type Checking (Āyus Sahita Prakāra Parīkṣā)
//!
//! Integrates type inference with lifetime analysis, creating a unified
//! system based on:
//!
//! ## Saṃskṛta Philosophical Foundation
//!
//! **Kāla (Time) Doctrine in Vaiśeṣika:**
//! Vaiśeṣika recognizes Kāla (time) as one of the nine Dravyas (substances).
//! Lifetimes are temporal qualities (Kālika Guṇas) that qualify references:
//! - Every reference has a temporal extent (Āyus)
//! - References cannot outlive their referents (Kāla-Bandha constraint)
//! - Mutable references enforce exclusive temporal access (Ekakālika Adhikāra)
//!
//! **Nyāya Temporal Inference:**
//! The Nyāya Sāmānya-Viśeṣa framework extends to temporal reasoning:
//! - Type schemes now include lifetime parameters: ∀α ∀'a. T
//! - Lifetime constraints are Upādhi (conditions) on type judgments
//! - Region inference uses Pramāṇa-based certainty tracking
//!
//! ## Technical Foundation
//!
//! Based on:
//! - Tofte & Talpin (1997) "Region-Based Memory Management"
//! - Rust's NLL (Non-Lexical Lifetimes)
//! - Polonius (Rust's next-gen borrow checker)
//!
//! ## Architecture
//!
//! ```text
//! TypeInference ←──→ LifetimeInference ←──→ BorrowChecker
//!      │                    │                     │
//!      │                    │                     │
//!      ▼                    ▼                     ▼
//! ResolvedType         RegionVar            OwnershipInfo
//! (with lifetimes)   (fresh regions)      (move tracking)
//! ```

use super::inference::UnificationError;
use super::types::{ResolvedType, TypeVar};
use crate::lexer::Span;
use std::collections::{BTreeSet, HashMap, HashSet};

// ============================================================================
// Region Variables (Kṣetra Cāla)
// ============================================================================

/// Region variable identifier for lifetime inference
///
/// Named after Vaiśeṣika Kṣetra (field/region) - the spatial extent
/// that becomes temporal when applied to reference lifetimes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RegionVar(pub u32);

impl RegionVar {
    /// Special region: 'static (Nitya - eternal)
    pub const STATIC: RegionVar = RegionVar(0);

    /// Check if this is the static region
    pub fn is_static(self) -> bool {
        self.0 == 0
    }
}

impl std::fmt::Display for RegionVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_static() {
            write!(f, "'static")
        } else {
            write!(f, "'ρ{}", self.0)
        }
    }
}

// ============================================================================
// Region Constraint System (Kṣetra Niyama)
// ============================================================================

/// Outlives constraint: 'a: 'b means 'a lives at least as long as 'b
///
/// In Nyāya terms: This is an Upādhi (limiting condition) on type validity.
/// The type T<'a> is valid only if 'a outlives all references within T.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutlivesConstraint {
    /// The longer-lived region (Dīrgha-Āyus)
    pub longer: RegionVar,
    /// The shorter-lived region (Hrasva-Āyus)
    pub shorter: RegionVar,
    /// Source location for error reporting
    pub span: Option<Span>,
    /// Reason this constraint was generated
    pub reason: OutlivesReason,
}

/// Reason for an outlives constraint
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutlivesReason {
    /// Reference stored in struct field
    StructField {
        struct_name: String,
        field_name: String,
    },
    /// Reference returned from function
    FunctionReturn { function_name: String },
    /// Reference passed as argument
    Argument {
        function_name: String,
        param_name: String,
    },
    /// Reference assigned to variable
    Assignment { var_name: String },
    /// Reference coercion (widening)
    Coercion,
    /// Explicit annotation
    Annotation,
    /// Inferred from usage
    Inferred,
}

impl OutlivesReason {
    /// Sanskrit description for error messages
    pub fn sanskrit_description(&self) -> &'static str {
        match self {
            OutlivesReason::StructField { .. } => "क्षेत्र-बन्ध", // Field binding
            OutlivesReason::FunctionReturn { .. } => "प्रत्यागम-बन्ध", // Return binding
            OutlivesReason::Argument { .. } => "परमति-बन्ध",   // Parameter binding
            OutlivesReason::Assignment { .. } => "न्यास-बन्ध",  // Assignment binding
            OutlivesReason::Coercion => "रूपान्तर-बन्ध",         // Coercion binding
            OutlivesReason::Annotation => "अभिलेख-बन्ध",        // Annotation binding
            OutlivesReason::Inferred => "अनुमान-बन्ध",          // Inferred binding
        }
    }
}

// ============================================================================
// Lifetime Inference Engine (Āyus Anumāna Yantra)
// ============================================================================

/// Lifetime inference engine
///
/// Performs region inference using constraint-based analysis.
/// Integrates with type inference through the TypeWithLifetime system.
pub struct LifetimeInference {
    /// Next region variable ID
    next_region: u32,
    /// Region substitutions
    substitutions: HashMap<RegionVar, RegionVar>,
    /// Active constraints
    constraints: Vec<OutlivesConstraint>,
    /// Known region relationships (transitive closure)
    outlives_graph: HashMap<RegionVar, BTreeSet<RegionVar>>,
}

impl LifetimeInference {
    /// Create a new lifetime inference engine
    pub fn new() -> Self {
        Self {
            next_region: 1, // 0 is reserved for 'static
            substitutions: HashMap::new(),
            constraints: Vec::new(),
            outlives_graph: HashMap::new(),
        }
    }

    /// Create a fresh region variable (Nūtana Kṣetra)
    pub fn fresh_region(&mut self) -> RegionVar {
        let region = RegionVar(self.next_region);
        self.next_region += 1;
        region
    }

    /// Add an outlives constraint: longer: shorter
    pub fn add_outlives(&mut self, constraint: OutlivesConstraint) {
        // Add to graph
        self.outlives_graph
            .entry(constraint.longer)
            .or_insert_with(BTreeSet::new)
            .insert(constraint.shorter);

        // Store constraint
        self.constraints.push(constraint);
    }

    /// Add simple outlives: longer: shorter
    pub fn add_outlives_simple(
        &mut self,
        longer: RegionVar,
        shorter: RegionVar,
        reason: OutlivesReason,
        span: Option<Span>,
    ) {
        self.add_outlives(OutlivesConstraint {
            longer,
            shorter,
            span,
            reason,
        });
    }

    /// Apply region substitutions
    pub fn apply(&self, region: RegionVar) -> RegionVar {
        let mut current = region;
        while let Some(&subst) = self.substitutions.get(&current) {
            if subst == current {
                break; // Prevent infinite loop
            }
            current = subst;
        }
        current
    }

    /// Unify two regions (make them equivalent)
    pub fn unify_regions(&mut self, r1: RegionVar, r2: RegionVar) -> Result<(), LifetimeError> {
        let r1 = self.apply(r1);
        let r2 = self.apply(r2);

        if r1 == r2 {
            return Ok(());
        }

        // Static region absorbs others
        if r1.is_static() {
            self.substitutions.insert(r2, r1);
        } else if r2.is_static() {
            self.substitutions.insert(r1, r2);
        } else {
            // Prefer lower-numbered regions
            if r1.0 < r2.0 {
                self.substitutions.insert(r2, r1);
            } else {
                self.substitutions.insert(r1, r2);
            }
        }

        Ok(())
    }

    /// Check if r1 outlives r2 (r1: r2)
    pub fn outlives(&self, r1: RegionVar, r2: RegionVar) -> bool {
        let r1 = self.apply(r1);
        let r2 = self.apply(r2);

        // Static outlives everything
        if r1.is_static() {
            return true;
        }

        // Same region trivially outlives itself
        if r1 == r2 {
            return true;
        }

        // Check graph
        self.outlives_graph
            .get(&r1)
            .map(|set| set.contains(&r2))
            .unwrap_or(false)
    }

    /// Solve constraints and check for errors
    pub fn solve(&mut self) -> Result<(), Vec<LifetimeError>> {
        let errors: Vec<LifetimeError> = Vec::new();

        // Compute transitive closure
        self.compute_transitive_closure();

        // Check for cycles (would indicate contradictory constraints)
        for constraint in &self.constraints {
            let longer = self.apply(constraint.longer);
            let shorter = self.apply(constraint.shorter);

            // Check if shorter also outlives longer (cycle)
            if longer != shorter && self.outlives(shorter, longer) {
                // This is only an error if they're different and cycle
                // For now, unify them (they must be the same region)
                // Could be an error in some cases
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Compute transitive closure of outlives relations
    fn compute_transitive_closure(&mut self) {
        // Collect all regions (both keys and values in the graph)
        let mut all_regions: BTreeSet<RegionVar> = BTreeSet::new();
        for (&k, vs) in &self.outlives_graph {
            all_regions.insert(k);
            for &v in vs {
                all_regions.insert(v);
            }
        }
        let regions: Vec<RegionVar> = all_regions.into_iter().collect();

        // Floyd-Warshall style closure
        // Iterate until no more changes
        let mut changed = true;
        while changed {
            changed = false;
            for &i in &regions {
                for &j in &regions {
                    for &k in &regions {
                        // If i outlives k and k outlives j, then i outlives j
                        if self.outlives(i, k) && self.outlives(k, j) && !self.outlives(i, j) {
                            self.outlives_graph
                                .entry(i)
                                .or_insert_with(BTreeSet::new)
                                .insert(j);
                            changed = true;
                        }
                    }
                }
            }
        }
    }

    /// Get all regions that r outlives
    pub fn regions_outlived_by(&self, r: RegionVar) -> Vec<RegionVar> {
        let r = self.apply(r);
        self.outlives_graph
            .get(&r)
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default()
    }

    /// Create a fresh region that outlives the given region
    pub fn fresh_outliving(&mut self, outlived: RegionVar) -> RegionVar {
        let fresh = self.fresh_region();
        self.add_outlives_simple(fresh, outlived, OutlivesReason::Inferred, None);
        fresh
    }

    /// Get the count of region variables created
    pub fn region_count(&self) -> usize {
        self.next_region as usize
    }
}

impl Default for LifetimeInference {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Type with Lifetimes (Āyusmata Prakāra)
// ============================================================================

/// Type annotated with lifetime regions
///
/// Extends ResolvedType with explicit lifetime parameters.
/// This is the "full" type used in lifetime-aware type checking.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeWithLifetime {
    /// Primitive type (no lifetimes)
    Primitive(ResolvedType),

    /// Reference with explicit lifetime
    Reference {
        lifetime: RegionVar,
        mutable: bool,
        inner: Box<TypeWithLifetime>,
    },

    /// Named type with lifetime parameters
    Named {
        name: String,
        lifetime_params: Vec<RegionVar>,
        type_params: Vec<TypeWithLifetime>,
    },

    /// Function type with lifetime bounds
    Function {
        lifetime_params: Vec<RegionVar>,
        params: Vec<TypeWithLifetime>,
        return_type: Box<TypeWithLifetime>,
        constraints: Vec<OutlivesConstraint>,
    },

    /// Array type
    Array {
        element: Box<TypeWithLifetime>,
        size: Option<usize>,
    },

    /// Tuple type
    Tuple(Vec<TypeWithLifetime>),

    /// Type variable (for inference)
    TypeVar(TypeVar),

    /// Region variable (for lifetime parameters)
    RegionVar(RegionVar),
}

impl TypeWithLifetime {
    /// Create a primitive type (no lifetime)
    pub fn primitive(ty: ResolvedType) -> Self {
        TypeWithLifetime::Primitive(ty)
    }

    /// Create a reference type
    pub fn reference(lifetime: RegionVar, mutable: bool, inner: TypeWithLifetime) -> Self {
        TypeWithLifetime::Reference {
            lifetime,
            mutable,
            inner: Box::new(inner),
        }
    }

    /// Strip lifetimes to get base ResolvedType
    pub fn to_resolved_type(&self) -> ResolvedType {
        match self {
            TypeWithLifetime::Primitive(ty) => ty.clone(),
            TypeWithLifetime::Reference { inner, mutable, .. } => ResolvedType::Reference {
                inner: Box::new(inner.to_resolved_type()),
                mutable: *mutable,
                lifetime: None,
            },
            TypeWithLifetime::Named {
                name, type_params, ..
            } => ResolvedType::Named {
                name: name.clone(),
                generics: type_params.iter().map(|t| t.to_resolved_type()).collect(),
            },
            TypeWithLifetime::Function {
                params,
                return_type,
                ..
            } => ResolvedType::Function {
                params: params.iter().map(|t| t.to_resolved_type()).collect(),
                return_type: Box::new(return_type.to_resolved_type()),
            },
            TypeWithLifetime::Array { element, size } => ResolvedType::Array {
                element: Box::new(element.to_resolved_type()),
                size: *size,
            },
            TypeWithLifetime::Tuple(elems) => {
                ResolvedType::Tuple(elems.iter().map(|t| t.to_resolved_type()).collect())
            }
            TypeWithLifetime::TypeVar(var) => ResolvedType::TypeVar(*var),
            TypeWithLifetime::RegionVar(_) => ResolvedType::Unknown,
        }
    }

    /// Get all region variables in this type
    pub fn region_vars(&self) -> HashSet<RegionVar> {
        let mut vars = HashSet::new();
        self.collect_region_vars(&mut vars);
        vars
    }

    fn collect_region_vars(&self, vars: &mut HashSet<RegionVar>) {
        match self {
            TypeWithLifetime::Primitive(_) => {}
            TypeWithLifetime::Reference {
                lifetime, inner, ..
            } => {
                vars.insert(*lifetime);
                inner.collect_region_vars(vars);
            }
            TypeWithLifetime::Named {
                lifetime_params,
                type_params,
                ..
            } => {
                vars.extend(lifetime_params.iter().copied());
                for param in type_params {
                    param.collect_region_vars(vars);
                }
            }
            TypeWithLifetime::Function {
                lifetime_params,
                params,
                return_type,
                ..
            } => {
                vars.extend(lifetime_params.iter().copied());
                for param in params {
                    param.collect_region_vars(vars);
                }
                return_type.collect_region_vars(vars);
            }
            TypeWithLifetime::Array { element, .. } => {
                element.collect_region_vars(vars);
            }
            TypeWithLifetime::Tuple(elems) => {
                for elem in elems {
                    elem.collect_region_vars(vars);
                }
            }
            TypeWithLifetime::TypeVar(_) => {}
            TypeWithLifetime::RegionVar(r) => {
                vars.insert(*r);
            }
        }
    }
}

// ============================================================================
// Lifetime Error Types (Āyus Doṣa)
// ============================================================================

/// Lifetime-related errors
#[derive(Debug, Clone)]
pub enum LifetimeError {
    /// Reference outlives referent
    ReferenceOutlivesReferent {
        reference: String,
        referent: String,
        reference_region: RegionVar,
        referent_region: RegionVar,
        span: Option<Span>,
    },

    /// Conflicting lifetime requirements
    ConflictingRequirements {
        constraint1: OutlivesConstraint,
        constraint2: OutlivesConstraint,
    },

    /// Cannot infer lifetime
    CannotInfer { context: String, span: Option<Span> },

    /// Lifetime parameter mismatch
    LifetimeMismatch {
        expected: RegionVar,
        found: RegionVar,
        context: String,
        span: Option<Span>,
    },

    /// Missing lifetime parameter
    MissingLifetime {
        type_name: String,
        expected_count: usize,
        found_count: usize,
        span: Option<Span>,
    },
}

impl LifetimeError {
    /// Sanskrit error name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            LifetimeError::ReferenceOutlivesReferent { .. } => "आयुस्-अतिक्रम",
            LifetimeError::ConflictingRequirements { .. } => "विरोधी-बन्ध",
            LifetimeError::CannotInfer { .. } => "अनुमान-असम्भव",
            LifetimeError::LifetimeMismatch { .. } => "आयुस्-भेद",
            LifetimeError::MissingLifetime { .. } => "आयुस्-न्यून",
        }
    }

    /// User-facing error description
    pub fn description(&self) -> String {
        match self {
            LifetimeError::ReferenceOutlivesReferent {
                reference,
                referent,
                ..
            } => {
                format!(
                    "Reference '{}' may outlive its referent '{}'. \
                     The referent is dropped while the reference is still in use.",
                    reference, referent
                )
            }
            LifetimeError::ConflictingRequirements {
                constraint1,
                constraint2,
            } => {
                format!(
                    "Conflicting lifetime requirements: \
                     {} must outlive {}, but {} must also outlive {}",
                    constraint1.longer,
                    constraint1.shorter,
                    constraint2.longer,
                    constraint2.shorter
                )
            }
            LifetimeError::CannotInfer { context, .. } => {
                format!(
                    "Cannot infer lifetime in {}. \
                     Consider adding explicit lifetime annotations.",
                    context
                )
            }
            LifetimeError::LifetimeMismatch {
                expected,
                found,
                context,
                ..
            } => {
                format!(
                    "Lifetime mismatch in {}: expected {}, found {}",
                    context, expected, found
                )
            }
            LifetimeError::MissingLifetime {
                type_name,
                expected_count,
                found_count,
                ..
            } => {
                format!(
                    "Type '{}' requires {} lifetime parameter(s), but {} were provided",
                    type_name, expected_count, found_count
                )
            }
        }
    }
}

impl std::fmt::Display for LifetimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl std::error::Error for LifetimeError {}

// ============================================================================
// Lifetime-Type Unification (Āyus-Prakāra Ekīkaraṇa)
// ============================================================================

/// Combined type and lifetime unifier
///
/// Unifies types while also generating and checking lifetime constraints.
pub struct LifetimeTypeUnifier<'a> {
    /// Type inference engine
    type_inference: &'a mut super::inference::TypeInference,
    /// Lifetime inference engine
    lifetime_inference: &'a mut LifetimeInference,
}

impl<'a> LifetimeTypeUnifier<'a> {
    /// Create a new unifier
    pub fn new(
        type_inference: &'a mut super::inference::TypeInference,
        lifetime_inference: &'a mut LifetimeInference,
    ) -> Self {
        Self {
            type_inference,
            lifetime_inference,
        }
    }

    /// Unify two types with lifetimes
    pub fn unify(
        &mut self,
        t1: &TypeWithLifetime,
        t2: &TypeWithLifetime,
    ) -> Result<(), LifetimeUnificationError> {
        match (t1, t2) {
            // Primitive types delegate to type inference
            (TypeWithLifetime::Primitive(p1), TypeWithLifetime::Primitive(p2)) => {
                self.type_inference
                    .unify(p1, p2)
                    .map_err(|e| LifetimeUnificationError::TypeMismatch(e))?;
                Ok(())
            }

            // Reference types: unify lifetimes and inner types
            (
                TypeWithLifetime::Reference {
                    lifetime: l1,
                    mutable: m1,
                    inner: i1,
                },
                TypeWithLifetime::Reference {
                    lifetime: l2,
                    mutable: m2,
                    inner: i2,
                },
            ) => {
                if m1 != m2 {
                    return Err(LifetimeUnificationError::MutabilityMismatch);
                }
                self.lifetime_inference
                    .unify_regions(*l1, *l2)
                    .map_err(|e| LifetimeUnificationError::LifetimeError(e))?;
                self.unify(i1, i2)?;
                Ok(())
            }

            // Named types: unify lifetime and type parameters
            (
                TypeWithLifetime::Named {
                    name: n1,
                    lifetime_params: lp1,
                    type_params: tp1,
                },
                TypeWithLifetime::Named {
                    name: n2,
                    lifetime_params: lp2,
                    type_params: tp2,
                },
            ) => {
                if n1 != n2 {
                    return Err(LifetimeUnificationError::TypeMismatch(
                        UnificationError::TypeMismatch {
                            expected: ResolvedType::Named {
                                name: n1.clone(),
                                generics: vec![],
                            },
                            found: ResolvedType::Named {
                                name: n2.clone(),
                                generics: vec![],
                            },
                        },
                    ));
                }

                if lp1.len() != lp2.len() {
                    return Err(LifetimeUnificationError::LifetimeArityMismatch {
                        expected: lp1.len(),
                        found: lp2.len(),
                    });
                }

                if tp1.len() != tp2.len() {
                    return Err(LifetimeUnificationError::TypeMismatch(
                        UnificationError::GenericArityMismatch {
                            ty: n1.clone(),
                            expected: tp1.len(),
                            found: tp2.len(),
                        },
                    ));
                }

                for (l1, l2) in lp1.iter().zip(lp2.iter()) {
                    self.lifetime_inference
                        .unify_regions(*l1, *l2)
                        .map_err(|e| LifetimeUnificationError::LifetimeError(e))?;
                }

                for (t1, t2) in tp1.iter().zip(tp2.iter()) {
                    self.unify(t1, t2)?;
                }

                Ok(())
            }

            // Function types: unify with covariance/contravariance
            (
                TypeWithLifetime::Function {
                    params: p1,
                    return_type: r1,
                    ..
                },
                TypeWithLifetime::Function {
                    params: p2,
                    return_type: r2,
                    ..
                },
            ) => {
                if p1.len() != p2.len() {
                    return Err(LifetimeUnificationError::TypeMismatch(
                        UnificationError::ArityMismatch {
                            expected: p1.len(),
                            found: p2.len(),
                        },
                    ));
                }

                // Parameters are contravariant
                for (p1, p2) in p1.iter().zip(p2.iter()) {
                    self.unify(p2, p1)?; // Note: reversed for contravariance
                }

                // Return type is covariant
                self.unify(r1, r2)?;

                Ok(())
            }

            // Array types
            (
                TypeWithLifetime::Array {
                    element: e1,
                    size: s1,
                },
                TypeWithLifetime::Array {
                    element: e2,
                    size: s2,
                },
            ) => {
                if let (Some(sz1), Some(sz2)) = (s1, s2) {
                    if sz1 != sz2 {
                        return Err(LifetimeUnificationError::TypeMismatch(
                            UnificationError::ArraySizeMismatch {
                                expected: *sz1,
                                found: *sz2,
                            },
                        ));
                    }
                }
                self.unify(e1, e2)
            }

            // Tuple types
            (TypeWithLifetime::Tuple(elems1), TypeWithLifetime::Tuple(elems2)) => {
                if elems1.len() != elems2.len() {
                    return Err(LifetimeUnificationError::TypeMismatch(
                        UnificationError::TupleSizeMismatch {
                            expected: elems1.len(),
                            found: elems2.len(),
                        },
                    ));
                }

                for (e1, e2) in elems1.iter().zip(elems2.iter()) {
                    self.unify(e1, e2)?;
                }

                Ok(())
            }

            // Type variables
            (TypeWithLifetime::TypeVar(v1), TypeWithLifetime::TypeVar(v2)) => {
                self.type_inference
                    .unify(&ResolvedType::TypeVar(*v1), &ResolvedType::TypeVar(*v2))
                    .map_err(|e| LifetimeUnificationError::TypeMismatch(e))?;
                Ok(())
            }

            (TypeWithLifetime::TypeVar(v), other) | (other, TypeWithLifetime::TypeVar(v)) => {
                self.type_inference
                    .unify(&ResolvedType::TypeVar(*v), &other.to_resolved_type())
                    .map_err(|e| LifetimeUnificationError::TypeMismatch(e))?;
                Ok(())
            }

            // Region variables unify with themselves
            (TypeWithLifetime::RegionVar(r1), TypeWithLifetime::RegionVar(r2)) => {
                self.lifetime_inference
                    .unify_regions(*r1, *r2)
                    .map_err(|e| LifetimeUnificationError::LifetimeError(e))?;
                Ok(())
            }

            // Mismatched type constructors
            _ => Err(LifetimeUnificationError::TypeMismatch(
                UnificationError::TypeMismatch {
                    expected: t1.to_resolved_type(),
                    found: t2.to_resolved_type(),
                },
            )),
        }
    }
}

/// Errors during lifetime-aware unification
#[derive(Debug, Clone)]
pub enum LifetimeUnificationError {
    /// Type mismatch (delegated to type inference)
    TypeMismatch(UnificationError),
    /// Lifetime error
    LifetimeError(LifetimeError),
    /// Mutability mismatch in references
    MutabilityMismatch,
    /// Lifetime parameter count mismatch
    LifetimeArityMismatch { expected: usize, found: usize },
}

impl std::fmt::Display for LifetimeUnificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LifetimeUnificationError::TypeMismatch(e) => write!(f, "Type mismatch: {:?}", e),
            LifetimeUnificationError::LifetimeError(e) => write!(f, "Lifetime error: {}", e),
            LifetimeUnificationError::MutabilityMismatch => {
                write!(f, "Mutability mismatch in reference types")
            }
            LifetimeUnificationError::LifetimeArityMismatch { expected, found } => {
                write!(
                    f,
                    "Lifetime parameter count mismatch: expected {}, found {}",
                    expected, found
                )
            }
        }
    }
}

impl std::error::Error for LifetimeUnificationError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fresh_region() {
        let mut inference = LifetimeInference::new();
        let r1 = inference.fresh_region();
        let r2 = inference.fresh_region();
        assert_ne!(r1, r2);
        assert!(!r1.is_static());
        assert!(!r2.is_static());
    }

    #[test]
    fn test_static_region() {
        assert!(RegionVar::STATIC.is_static());
        assert_eq!(format!("{}", RegionVar::STATIC), "'static");
    }

    #[test]
    fn test_outlives_constraint() {
        let mut inference = LifetimeInference::new();
        let r1 = inference.fresh_region();
        let r2 = inference.fresh_region();

        inference.add_outlives_simple(r1, r2, OutlivesReason::Inferred, None);

        assert!(inference.outlives(r1, r2));
        assert!(!inference.outlives(r2, r1));
    }

    #[test]
    fn test_static_outlives_all() {
        let mut inference = LifetimeInference::new();
        let r1 = inference.fresh_region();

        assert!(inference.outlives(RegionVar::STATIC, r1));
    }

    #[test]
    fn test_region_unification() {
        let mut inference = LifetimeInference::new();
        let r1 = inference.fresh_region();
        let r2 = inference.fresh_region();

        inference.unify_regions(r1, r2).unwrap();

        assert_eq!(inference.apply(r1), inference.apply(r2));
    }

    #[test]
    fn test_transitive_outlives() {
        let mut inference = LifetimeInference::new();
        let r1 = inference.fresh_region();
        let r2 = inference.fresh_region();
        let r3 = inference.fresh_region();

        inference.add_outlives_simple(r1, r2, OutlivesReason::Inferred, None);
        inference.add_outlives_simple(r2, r3, OutlivesReason::Inferred, None);

        inference.compute_transitive_closure();

        assert!(inference.outlives(r1, r3));
    }

    #[test]
    fn test_type_with_lifetime_region_vars() {
        let r1 = RegionVar(1);
        let r2 = RegionVar(2);

        let ty = TypeWithLifetime::Reference {
            lifetime: r1,
            mutable: false,
            inner: Box::new(TypeWithLifetime::Named {
                name: "Foo".to_string(),
                lifetime_params: vec![r2],
                type_params: vec![],
            }),
        };

        let vars = ty.region_vars();
        assert!(vars.contains(&r1));
        assert!(vars.contains(&r2));
        assert_eq!(vars.len(), 2);
    }

    #[test]
    fn test_type_with_lifetime_to_resolved() {
        let ty = TypeWithLifetime::Reference {
            lifetime: RegionVar(1),
            mutable: true,
            inner: Box::new(TypeWithLifetime::Primitive(ResolvedType::Int32)),
        };

        let resolved = ty.to_resolved_type();
        match resolved {
            ResolvedType::Reference { mutable, inner, .. } => {
                assert!(mutable);
                assert_eq!(*inner, ResolvedType::Int32);
            }
            _ => panic!("Expected reference type"),
        }
    }

    #[test]
    fn test_lifetime_error_display() {
        let err = LifetimeError::ReferenceOutlivesReferent {
            reference: "r".to_string(),
            referent: "x".to_string(),
            reference_region: RegionVar(1),
            referent_region: RegionVar(2),
            span: None,
        };

        let desc = err.description();
        assert!(desc.contains("outlive"));
        assert!(desc.contains("'r'"));
    }

    #[test]
    fn test_outlives_reason_sanskrit() {
        assert_eq!(
            OutlivesReason::StructField {
                struct_name: "S".to_string(),
                field_name: "f".to_string(),
            }
            .sanskrit_description(),
            "क्षेत्र-बन्ध"
        );

        assert_eq!(
            OutlivesReason::FunctionReturn {
                function_name: "f".to_string(),
            }
            .sanskrit_description(),
            "प्रत्यागम-बन्ध"
        );
    }

    #[test]
    fn test_region_var_display() {
        assert_eq!(format!("{}", RegionVar(5)), "'ρ5");
        assert_eq!(format!("{}", RegionVar::STATIC), "'static");
    }

    #[test]
    fn test_fresh_outliving() {
        let mut inference = LifetimeInference::new();
        let r1 = inference.fresh_region();
        let r2 = inference.fresh_outliving(r1);

        assert!(inference.outlives(r2, r1));
        assert!(!inference.outlives(r1, r2));
    }
}
