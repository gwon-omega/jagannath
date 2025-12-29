//! Constraint Solving System (Nyāya Anumāna Yantra)
//!
//! Implements type constraint solving using the Nyāya philosophical framework.
//! Constraints are generated during type inference and solved to find valid
//! type assignments.
//!
//! ## Nyāya Mapping
//!
//! The Nyāya five-part syllogism (Pañcāvayava) maps to constraint solving:
//!
//! | Nyāya Term | Meaning | Constraint Role |
//! |------------|---------|-----------------|
//! | Pratijñā | Thesis | The type equation to prove |
//! | Hetu | Reason | Why constraint was generated |
//! | Udāharaṇa | Example | Universal typing rule applied |
//! | Upanaya | Application | How rule applies to this case |
//! | Nigamana | Conclusion | Solved type assignment |
//!
//! The key terms in constraint solving:
//! - **Sādhya** (साध्य) - Goal: the type to be proven
//! - **Sādhana** (साधन) - Means: evidence for the type
//! - **Pakṣa** (पक्ष) - Subject: the expression being typed
//! - **Hetu** (हेतु) - Reason: why the constraint exists
//!
//! ## Constraint Types
//!
//! 1. **Sāmya** (Equality): `T₁ = T₂` - types must be equal
//! 2. **Upādi** (Subtype): `T₁ <: T₂` - T₁ is subtype of T₂
//! 3. **Avayava** (Field): `T.field : F` - T has field of type F
//! 4. **Āhvāna** (Call): `T(A₁,...,Aₙ) : R` - T called with args gives R

use super::pramana::Pramana;
use super::types::{ResolvedType, TypeVar};
use crate::lexer::Span;
use std::collections::HashMap;

// ============================================================================
// Core Constraint Types
// ============================================================================

/// A type constraint to be solved (Prabandhana)
#[derive(Debug, Clone)]
pub struct Constraint {
    /// The kind of constraint
    pub kind: ConstraintKind,
    /// Why this constraint was generated (Hetu)
    pub reason: ConstraintReason,
    /// Where in source this constraint originates
    pub span: Option<Span>,
    /// Priority for solving (higher = solve first)
    pub priority: u8,
}

/// Kinds of type constraints (Prabandhana Prakāra)
#[derive(Debug, Clone)]
pub enum ConstraintKind {
    /// Sāmya (समय) - Equality constraint: T₁ = T₂
    /// The most common constraint from unification
    Equality {
        /// Left-hand type (Sādhya - goal)
        lhs: ResolvedType,
        /// Right-hand type (Sādhana - means)
        rhs: ResolvedType,
    },

    /// Upādi (उपाधि) - Subtype constraint: T₁ <: T₂
    /// T₁ must be a subtype of T₂ (covariance)
    Subtype {
        /// The type that must be a subtype (sub)
        sub: ResolvedType,
        /// The supertype (super)
        sup: ResolvedType,
    },

    /// Avayava (अवयव) - Field access constraint: T.field : F
    /// Type T must have a field with type F
    FieldAccess {
        /// The receiver type (Pakṣa)
        receiver: ResolvedType,
        /// The field name
        field: String,
        /// The expected field type
        field_type: ResolvedType,
    },

    /// Āhvāna (आह्वान) - Function call constraint
    /// Calling T with arguments yields return type R
    FunctionCall {
        /// The callee type
        callee: ResolvedType,
        /// Argument types
        args: Vec<ResolvedType>,
        /// Expected return type
        result: ResolvedType,
    },

    /// Sūcī (सूचि) - Index constraint: T[I] : E
    /// Indexing T with I yields element type E
    IndexAccess {
        /// The container type
        container: ResolvedType,
        /// The index type
        index: ResolvedType,
        /// The element type
        element: ResolvedType,
    },

    /// Prakāra Bandha (प्रकार बन्ध) - Type binding
    /// Binds a name to a specific type
    TypeBinding {
        /// The name being bound
        name: String,
        /// The type being bound
        ty: ResolvedType,
    },

    /// Dharma (धर्म) - Trait bound constraint: T : Trait
    /// Type T must implement the specified trait (Dharma in Jagannath)
    ///
    /// In Sanskrit philosophy, Dharma represents duty/righteous conduct.
    /// In the type system, a trait bound is a Dharma that a type must fulfill.
    TraitBound {
        /// The type that must implement the trait
        ty: ResolvedType,
        /// The trait name (Dharma name)
        trait_name: String,
        /// Optional trait type parameters
        trait_params: Vec<ResolvedType>,
    },

    /// Saṃbandha (सम्बन्ध) - Associated type constraint
    /// <T as Trait>::AssocType = U
    /// The associated type of T for Trait must equal U
    AssociatedType {
        /// The implementing type
        impl_type: ResolvedType,
        /// The trait containing the associated type
        trait_name: String,
        /// The name of the associated type
        assoc_name: String,
        /// The expected associated type
        expected_type: ResolvedType,
    },
}

/// Reason a constraint was generated (Hetu)
#[derive(Debug, Clone)]
pub enum ConstraintReason {
    /// From explicit type annotation (Pratyakṣa)
    Annotation,
    /// From let binding initialization
    LetBinding { name: String },
    /// From function argument
    FunctionArg { func: String, arg_index: usize },
    /// From function return
    FunctionReturn { func: String },
    /// From binary operation
    BinaryOp { op: String },
    /// From unary operation
    UnaryOp { op: String },
    /// From field access
    FieldAccess { field: String },
    /// From method call
    MethodCall { method: String },
    /// From array indexing
    ArrayIndex,
    /// From if-else branch unification
    IfElseBranch,
    /// From match arm unification
    MatchArm { arm_index: usize },
    /// From struct literal field
    StructField { struct_name: String, field: String },
    /// From pattern matching
    Pattern { pattern: String },
    /// From trait bound (Dharma requirement)
    TraitBound { trait_name: String },
    /// From associated type projection
    AssociatedType {
        trait_name: String,
        assoc_name: String,
    },
    /// From generic function instantiation
    GenericInstantiation { function: String },
    /// From where clause
    WhereClause,
    /// Custom reason
    Custom(String),
}

impl ConstraintReason {
    /// Get a human-readable description
    pub fn describe(&self) -> String {
        match self {
            Self::Annotation => "explicit type annotation".to_string(),
            Self::LetBinding { name } => format!("let binding '{}'", name),
            Self::FunctionArg { func, arg_index } => {
                format!("argument {} of function '{}'", arg_index + 1, func)
            }
            Self::FunctionReturn { func } => format!("return type of '{}'", func),
            Self::BinaryOp { op } => format!("binary operation '{}'", op),
            Self::UnaryOp { op } => format!("unary operation '{}'", op),
            Self::FieldAccess { field } => format!("field access '.{}'", field),
            Self::MethodCall { method } => format!("method call '.{}()'", method),
            Self::ArrayIndex => "array indexing".to_string(),
            Self::IfElseBranch => "if-else branch unification".to_string(),
            Self::MatchArm { arm_index } => format!("match arm {}", arm_index + 1),
            Self::StructField { struct_name, field } => {
                format!("field '{}' in struct '{}'", field, struct_name)
            }
            Self::Pattern { pattern } => format!("pattern '{}'", pattern),
            Self::TraitBound { trait_name } => format!("trait bound '{}'", trait_name),
            Self::AssociatedType {
                trait_name,
                assoc_name,
            } => {
                format!("associated type '{}::{}'", trait_name, assoc_name)
            }
            Self::GenericInstantiation { function } => {
                format!("generic instantiation of '{}'", function)
            }
            Self::WhereClause => "where clause".to_string(),
            Self::Custom(msg) => msg.clone(),
        }
    }

    /// Get Sanskrit term for this reason
    pub fn sanskrit_term(&self) -> &'static str {
        match self {
            Self::Annotation => "स्पष्टीकरण",       // spaṣṭīkaraṇa (clarification)
            Self::LetBinding { .. } => "चर-बन्धन", // cara-bandhana (variable binding)
            Self::FunctionArg { .. } => "युक्ति",   // yukti (argument)
            Self::FunctionReturn { .. } => "फल",  // phala (result)
            Self::BinaryOp { .. } => "द्वि-क्रिया", // dvi-kriyā (binary action)
            Self::UnaryOp { .. } => "एक-क्रिया",   // eka-kriyā (unary action)
            Self::FieldAccess { .. } => "क्षेत्र",   // kṣetra (field)
            Self::MethodCall { .. } => "विधि",    // vidhi (method)
            Self::ArrayIndex => "सूचकाङ्क",         // sūcakāṅka (index)
            Self::IfElseBranch => "शाखा",         // śākhā (branch)
            Self::MatchArm { .. } => "प्रतिरूप",    // pratirūpa (pattern)
            Self::StructField { .. } => "संरचना",  // saṃracanā (structure)
            Self::Pattern { .. } => "आकृति",       // ākṛti (shape)
            Self::TraitBound { .. } => "धर्म",     // dharma (duty/trait)
            Self::AssociatedType { .. } => "सम्बन्ध-प्रकार", // sambandha-prakāra (related type)
            Self::GenericInstantiation { .. } => "सामान्य-विशेषीकरण", // sāmānya-viśeṣīkaraṇa
            Self::WhereClause => "यत्र-वाक्य",      // yatra-vākya (where clause)
            Self::Custom(_) => "अन्य",             // anya (other)
        }
    }
}

// ============================================================================
// Constraint Solver
// ============================================================================

/// Constraint solver using Nyāya inference (Prabandhana Sādhaka)
pub struct ConstraintSolver {
    /// Pending constraints to solve
    constraints: Vec<Constraint>,
    /// Current substitutions (solved assignments)
    substitutions: HashMap<TypeVar, ResolvedType>,
    /// Next type variable ID
    next_var: usize,
    /// Solving errors encountered
    errors: Vec<ConstraintError>,
}

/// Error during constraint solving
#[derive(Debug, Clone)]
pub struct ConstraintError {
    /// The kind of error
    pub kind: ConstraintErrorKind,
    /// The constraint that failed
    pub constraint: Constraint,
    /// Additional context
    pub context: String,
}

/// Kinds of constraint errors
#[derive(Debug, Clone)]
pub enum ConstraintErrorKind {
    /// Types don't unify
    UnificationFailed {
        expected: ResolvedType,
        found: ResolvedType,
    },
    /// Infinite type (occurs check failed)
    InfiniteType { var: TypeVar, ty: ResolvedType },
    /// Unknown field
    UnknownField { ty: ResolvedType, field: String },
    /// Not callable
    NotCallable { ty: ResolvedType },
    /// Not indexable
    NotIndexable { ty: ResolvedType },
    /// Trait not implemented (Dharma violation)
    TraitNotImplemented {
        ty: ResolvedType,
        trait_name: String,
    },
    /// Associated type mismatch
    AssociatedTypeMismatch {
        trait_name: String,
        assoc_name: String,
        expected: ResolvedType,
        found: ResolvedType,
    },
    /// Trait bound conflict (multiple conflicting implementations)
    TraitBoundConflict {
        ty: ResolvedType,
        trait_name: String,
    },
    /// Unsolvable constraint system
    Unsolvable,
}

impl ConstraintSolver {
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            substitutions: HashMap::new(),
            next_var: 0,
            errors: Vec::new(),
        }
    }

    /// Create a fresh type variable (Nūtana Anirdhārita)
    pub fn fresh_var(&mut self) -> ResolvedType {
        let var = TypeVar(self.next_var);
        self.next_var += 1;
        ResolvedType::TypeVar(var)
    }

    /// Add an equality constraint (Sāmya Prabandhana)
    pub fn add_equality(
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
            priority: 10, // High priority for equality
        });
    }

    /// Add a subtype constraint (Upādi Prabandhana)
    pub fn add_subtype(
        &mut self,
        sub: ResolvedType,
        sup: ResolvedType,
        reason: ConstraintReason,
        span: Option<Span>,
    ) {
        self.constraints.push(Constraint {
            kind: ConstraintKind::Subtype { sub, sup },
            reason,
            span,
            priority: 8,
        });
    }

    /// Add a field access constraint
    pub fn add_field_access(
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
            priority: 5,
        });
    }

    /// Add a function call constraint
    pub fn add_function_call(
        &mut self,
        callee: ResolvedType,
        args: Vec<ResolvedType>,
        result: ResolvedType,
        func_name: String,
        span: Option<Span>,
    ) {
        self.constraints.push(Constraint {
            kind: ConstraintKind::FunctionCall {
                callee,
                args,
                result,
            },
            reason: ConstraintReason::Custom(format!("call to '{}'", func_name)),
            span,
            priority: 7,
        });
    }

    /// Solve all constraints (Prabandhana Samadhāna)
    ///
    /// Uses iterative unification following the Nyāya principle of
    /// establishing identity (sāmānya) between appearances.
    pub fn solve(&mut self) -> Result<(), Vec<ConstraintError>> {
        // Sort by priority (higher first)
        self.constraints.sort_by(|a, b| b.priority.cmp(&a.priority));

        // Iteratively solve constraints
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 100;

        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            iterations += 1;

            let constraints = std::mem::take(&mut self.constraints);
            for constraint in constraints {
                match self.solve_constraint(&constraint) {
                    SolveResult::Solved => {
                        changed = true;
                    }
                    SolveResult::Deferred => {
                        // Put back for later
                        self.constraints.push(constraint);
                    }
                    SolveResult::Failed(error) => {
                        self.errors.push(error);
                    }
                }
            }
        }

        // Check for unsolved constraints
        for constraint in &self.constraints {
            self.errors.push(ConstraintError {
                kind: ConstraintErrorKind::Unsolvable,
                constraint: constraint.clone(),
                context: "Could not solve constraint after maximum iterations".to_string(),
            });
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Solve a single constraint
    fn solve_constraint(&mut self, constraint: &Constraint) -> SolveResult {
        match &constraint.kind {
            ConstraintKind::Equality { lhs, rhs } => self.unify(lhs, rhs, constraint),
            ConstraintKind::Subtype { sub, sup } => {
                // For now, treat subtype as equality
                // TODO: Proper subtyping with variance
                self.unify(sub, sup, constraint)
            }
            ConstraintKind::FieldAccess {
                receiver,
                field,
                field_type,
            } => self.solve_field_access(receiver, field, field_type, constraint),
            ConstraintKind::FunctionCall {
                callee,
                args,
                result,
            } => self.solve_function_call(callee, args, result, constraint),
            ConstraintKind::IndexAccess {
                container,
                index,
                element,
            } => self.solve_index_access(container, index, element, constraint),
            ConstraintKind::TypeBinding { name: _, ty: _ } => {
                // Type bindings are recorded, not solved
                SolveResult::Solved
            }
            ConstraintKind::TraitBound {
                ty,
                trait_name: _trait_name,
                trait_params: _,
            } => {
                // Trait bounds are validated by UnifiedChecker
                // Here we just check if the type is concrete enough
                let resolved = self.apply(ty);
                match &resolved {
                    ResolvedType::TypeVar(_) => SolveResult::Deferred,
                    ResolvedType::Error => SolveResult::Solved,
                    _ => {
                        // Record for later validation by UnifiedChecker
                        // The actual trait implementation check happens there
                        SolveResult::Solved
                    }
                }
            }
            ConstraintKind::AssociatedType {
                impl_type,
                trait_name: _trait_name,
                assoc_name: _assoc_name,
                expected_type,
            } => {
                // Associated type resolution
                let resolved_impl = self.apply(impl_type);
                let _resolved_expected = self.apply(expected_type);

                match &resolved_impl {
                    ResolvedType::TypeVar(_) => SolveResult::Deferred,
                    ResolvedType::Error => SolveResult::Solved,
                    _ => {
                        // For now, defer full associated type resolution to UnifiedChecker
                        // This handles the constraint-level bookkeeping
                        SolveResult::Solved
                    }
                }
            }
        }
    }

    /// Unify two types (Ekīkaraṇa)
    fn unify(
        &mut self,
        t1: &ResolvedType,
        t2: &ResolvedType,
        constraint: &Constraint,
    ) -> SolveResult {
        let t1 = self.apply(t1);
        let t2 = self.apply(t2);

        match (&t1, &t2) {
            // Same type - trivially unifies
            (a, b) if a == b => SolveResult::Solved,

            // Type variable unification
            (ResolvedType::TypeVar(var), ty) | (ty, ResolvedType::TypeVar(var)) => {
                // Occurs check
                if self.occurs_in(*var, ty) {
                    return SolveResult::Failed(ConstraintError {
                        kind: ConstraintErrorKind::InfiniteType {
                            var: *var,
                            ty: ty.clone(),
                        },
                        constraint: constraint.clone(),
                        context: "Infinite type detected (occurs check failed)".to_string(),
                    });
                }
                self.substitutions.insert(*var, ty.clone());
                SolveResult::Solved
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
                    return SolveResult::Failed(ConstraintError {
                        kind: ConstraintErrorKind::UnificationFailed {
                            expected: t1.clone(),
                            found: t2.clone(),
                        },
                        constraint: constraint.clone(),
                        context: format!(
                            "Function arity mismatch: expected {} params, found {}",
                            p1.len(),
                            p2.len()
                        ),
                    });
                }
                // Add constraints for params and return
                for (a, b) in p1.iter().zip(p2.iter()) {
                    self.add_equality(
                        a.clone(),
                        b.clone(),
                        constraint.reason.clone(),
                        constraint.span,
                    );
                }
                self.add_equality(
                    (**r1).clone(),
                    (**r2).clone(),
                    constraint.reason.clone(),
                    constraint.span,
                );
                SolveResult::Solved
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
                if let (Some(sz1), Some(sz2)) = (s1, s2) {
                    if sz1 != sz2 {
                        return SolveResult::Failed(ConstraintError {
                            kind: ConstraintErrorKind::UnificationFailed {
                                expected: t1.clone(),
                                found: t2.clone(),
                            },
                            constraint: constraint.clone(),
                            context: format!("Array size mismatch: {} vs {}", sz1, sz2),
                        });
                    }
                }
                self.add_equality(
                    (**e1).clone(),
                    (**e2).clone(),
                    constraint.reason.clone(),
                    constraint.span,
                );
                SolveResult::Solved
            }

            // Tuple type unification
            (ResolvedType::Tuple(elems1), ResolvedType::Tuple(elems2)) => {
                if elems1.len() != elems2.len() {
                    return SolveResult::Failed(ConstraintError {
                        kind: ConstraintErrorKind::UnificationFailed {
                            expected: t1.clone(),
                            found: t2.clone(),
                        },
                        constraint: constraint.clone(),
                        context: format!(
                            "Tuple size mismatch: {} vs {}",
                            elems1.len(),
                            elems2.len()
                        ),
                    });
                }
                for (a, b) in elems1.iter().zip(elems2.iter()) {
                    self.add_equality(
                        a.clone(),
                        b.clone(),
                        constraint.reason.clone(),
                        constraint.span,
                    );
                }
                SolveResult::Solved
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
                    return SolveResult::Failed(ConstraintError {
                        kind: ConstraintErrorKind::UnificationFailed {
                            expected: t1.clone(),
                            found: t2.clone(),
                        },
                        constraint: constraint.clone(),
                        context: "Mutability mismatch".to_string(),
                    });
                }
                self.add_equality(
                    (**i1).clone(),
                    (**i2).clone(),
                    constraint.reason.clone(),
                    constraint.span,
                );
                SolveResult::Solved
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
                    return SolveResult::Failed(ConstraintError {
                        kind: ConstraintErrorKind::UnificationFailed {
                            expected: t1.clone(),
                            found: t2.clone(),
                        },
                        constraint: constraint.clone(),
                        context: format!("Type name mismatch: {} vs {}", n1, n2),
                    });
                }
                if g1.len() != g2.len() {
                    return SolveResult::Failed(ConstraintError {
                        kind: ConstraintErrorKind::UnificationFailed {
                            expected: t1.clone(),
                            found: t2.clone(),
                        },
                        constraint: constraint.clone(),
                        context: format!(
                            "Generic arity mismatch for {}: {} vs {}",
                            n1,
                            g1.len(),
                            g2.len()
                        ),
                    });
                }
                for (a, b) in g1.iter().zip(g2.iter()) {
                    self.add_equality(
                        a.clone(),
                        b.clone(),
                        constraint.reason.clone(),
                        constraint.span,
                    );
                }
                SolveResult::Solved
            }

            // Unknown unifies with anything
            (ResolvedType::Unknown, _) | (_, ResolvedType::Unknown) => SolveResult::Solved,

            // Error propagates
            (ResolvedType::Error, _) | (_, ResolvedType::Error) => SolveResult::Solved,

            // Never unifies with anything (diverging)
            (ResolvedType::Never, _) | (_, ResolvedType::Never) => SolveResult::Solved,

            // Mismatch
            _ => SolveResult::Failed(ConstraintError {
                kind: ConstraintErrorKind::UnificationFailed {
                    expected: t1.clone(),
                    found: t2.clone(),
                },
                constraint: constraint.clone(),
                context: "Types do not match".to_string(),
            }),
        }
    }

    /// Solve field access constraint
    fn solve_field_access(
        &mut self,
        receiver: &ResolvedType,
        field: &str,
        field_type: &ResolvedType,
        constraint: &Constraint,
    ) -> SolveResult {
        let receiver = self.apply(receiver);

        match &receiver {
            ResolvedType::TypeVar(_) => {
                // Defer until receiver is known
                SolveResult::Deferred
            }
            ResolvedType::Named {
                name: _,
                generics: _,
            } => {
                // TODO: Look up field type from type definition
                // For now, just accept it
                SolveResult::Solved
            }
            ResolvedType::Tuple(elems) => {
                // Tuple field access (0, 1, 2, ...)
                if let Ok(idx) = field.parse::<usize>() {
                    if idx < elems.len() {
                        self.add_equality(
                            elems[idx].clone(),
                            field_type.clone(),
                            constraint.reason.clone(),
                            constraint.span,
                        );
                        SolveResult::Solved
                    } else {
                        SolveResult::Failed(ConstraintError {
                            kind: ConstraintErrorKind::UnknownField {
                                ty: receiver.clone(),
                                field: field.to_string(),
                            },
                            constraint: constraint.clone(),
                            context: format!("Tuple index {} out of bounds", idx),
                        })
                    }
                } else {
                    SolveResult::Failed(ConstraintError {
                        kind: ConstraintErrorKind::UnknownField {
                            ty: receiver.clone(),
                            field: field.to_string(),
                        },
                        constraint: constraint.clone(),
                        context: "Tuple fields must be numeric".to_string(),
                    })
                }
            }
            _ => SolveResult::Failed(ConstraintError {
                kind: ConstraintErrorKind::UnknownField {
                    ty: receiver.clone(),
                    field: field.to_string(),
                },
                constraint: constraint.clone(),
                context: format!("Type {} has no field {}", receiver, field),
            }),
        }
    }

    /// Solve function call constraint
    fn solve_function_call(
        &mut self,
        callee: &ResolvedType,
        args: &[ResolvedType],
        result: &ResolvedType,
        constraint: &Constraint,
    ) -> SolveResult {
        let callee = self.apply(callee);

        match &callee {
            ResolvedType::TypeVar(_) => {
                // Defer until callee type is known
                SolveResult::Deferred
            }
            ResolvedType::Function {
                params,
                return_type,
            } => {
                if args.len() != params.len() {
                    return SolveResult::Failed(ConstraintError {
                        kind: ConstraintErrorKind::UnificationFailed {
                            expected: callee.clone(),
                            found: ResolvedType::Function {
                                params: args.to_vec(),
                                return_type: Box::new(result.clone()),
                            },
                        },
                        constraint: constraint.clone(),
                        context: format!("Expected {} arguments, got {}", params.len(), args.len()),
                    });
                }
                // Unify each argument
                for (arg, param) in args.iter().zip(params.iter()) {
                    self.add_equality(
                        arg.clone(),
                        param.clone(),
                        constraint.reason.clone(),
                        constraint.span,
                    );
                }
                // Unify return type
                self.add_equality(
                    result.clone(),
                    (**return_type).clone(),
                    constraint.reason.clone(),
                    constraint.span,
                );
                SolveResult::Solved
            }
            _ => SolveResult::Failed(ConstraintError {
                kind: ConstraintErrorKind::NotCallable { ty: callee.clone() },
                constraint: constraint.clone(),
                context: format!("Type {} is not callable", callee),
            }),
        }
    }

    /// Solve index access constraint
    fn solve_index_access(
        &mut self,
        container: &ResolvedType,
        index: &ResolvedType,
        element: &ResolvedType,
        constraint: &Constraint,
    ) -> SolveResult {
        let container = self.apply(container);
        let _index = self.apply(index);

        match &container {
            ResolvedType::TypeVar(_) => {
                // Defer until container type is known
                SolveResult::Deferred
            }
            ResolvedType::Array {
                element: elem_ty, ..
            } => {
                // Index must be integer
                self.add_equality(
                    _index.clone(),
                    ResolvedType::UInt64,
                    constraint.reason.clone(),
                    constraint.span,
                );
                // Element type must match
                self.add_equality(
                    element.clone(),
                    (**elem_ty).clone(),
                    constraint.reason.clone(),
                    constraint.span,
                );
                SolveResult::Solved
            }
            ResolvedType::String => {
                // String indexing returns char
                self.add_equality(
                    element.clone(),
                    ResolvedType::Char,
                    constraint.reason.clone(),
                    constraint.span,
                );
                SolveResult::Solved
            }
            _ => SolveResult::Failed(ConstraintError {
                kind: ConstraintErrorKind::NotIndexable {
                    ty: container.clone(),
                },
                constraint: constraint.clone(),
                context: format!("Type {} is not indexable", container),
            }),
        }
    }

    /// Apply substitutions to a type
    fn apply(&self, ty: &ResolvedType) -> ResolvedType {
        match ty {
            ResolvedType::TypeVar(var) => {
                if let Some(subst) = self.substitutions.get(var) {
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
            _ => ty.clone(),
        }
    }

    /// Occurs check - prevents infinite types (Anyonyāśraya Parīkṣā)
    fn occurs_in(&self, var: TypeVar, ty: &ResolvedType) -> bool {
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

    /// Get the final type after solving
    pub fn finalize(&self, ty: &ResolvedType) -> ResolvedType {
        self.apply(ty)
    }

    /// Get current substitutions
    pub fn substitutions(&self) -> &HashMap<TypeVar, ResolvedType> {
        &self.substitutions
    }
}

impl Default for ConstraintSolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of solving a single constraint
enum SolveResult {
    /// Constraint was solved successfully
    Solved,
    /// Constraint deferred (needs more info)
    Deferred,
    /// Constraint solving failed
    Failed(ConstraintError),
}

// ============================================================================
// Inference Result with Pramāṇa
// ============================================================================

/// Result of type inference with epistemological context
#[derive(Debug, Clone)]
pub struct InferenceResult {
    /// The inferred type
    pub ty: ResolvedType,
    /// How the type was inferred (Pramāṇa)
    pub pramana: Pramana,
    /// Certainty level (0.0 - 1.0)
    pub certainty: f32,
    /// Constraints generated during inference
    pub constraints: Vec<Constraint>,
}

impl InferenceResult {
    /// Create from explicit annotation (Pratyakṣa)
    pub fn explicit(ty: ResolvedType) -> Self {
        Self {
            ty,
            pramana: Pramana::Pratyaksha,
            certainty: 1.0,
            constraints: Vec::new(),
        }
    }

    /// Create from logical inference (Anumāna)
    pub fn inferred(ty: ResolvedType, constraints: Vec<Constraint>) -> Self {
        Self {
            ty,
            pramana: Pramana::Anumana,
            certainty: 0.95,
            constraints,
        }
    }

    /// Create from function signature (Śabda)
    pub fn from_signature(ty: ResolvedType) -> Self {
        Self {
            ty,
            pramana: Pramana::Shabda,
            certainty: 0.90,
            constraints: Vec::new(),
        }
    }

    /// Create from pattern matching (Upamāna)
    pub fn from_pattern(ty: ResolvedType, constraints: Vec<Constraint>) -> Self {
        Self {
            ty,
            pramana: Pramana::Upamana,
            certainty: 0.85,
            constraints,
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
    fn test_fresh_var() {
        let mut solver = ConstraintSolver::new();
        let v1 = solver.fresh_var();
        let v2 = solver.fresh_var();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_simple_equality() {
        let mut solver = ConstraintSolver::new();
        let var = solver.fresh_var();
        solver.add_equality(
            var.clone(),
            ResolvedType::Int32,
            ConstraintReason::Annotation,
            None,
        );
        assert!(solver.solve().is_ok());
        assert_eq!(solver.finalize(&var), ResolvedType::Int32);
    }

    #[test]
    fn test_transitive_equality() {
        let mut solver = ConstraintSolver::new();
        let v1 = solver.fresh_var();
        let v2 = solver.fresh_var();

        solver.add_equality(v1.clone(), v2.clone(), ConstraintReason::Annotation, None);
        solver.add_equality(
            v2.clone(),
            ResolvedType::Bool,
            ConstraintReason::Annotation,
            None,
        );

        assert!(solver.solve().is_ok());
        assert_eq!(solver.finalize(&v1), ResolvedType::Bool);
        assert_eq!(solver.finalize(&v2), ResolvedType::Bool);
    }

    #[test]
    fn test_function_unification() {
        let mut solver = ConstraintSolver::new();
        let ret = solver.fresh_var();

        let f1 = ResolvedType::Function {
            params: vec![ResolvedType::Int32],
            return_type: Box::new(ret.clone()),
        };
        let f2 = ResolvedType::Function {
            params: vec![ResolvedType::Int32],
            return_type: Box::new(ResolvedType::Bool),
        };

        solver.add_equality(f1, f2, ConstraintReason::Annotation, None);
        assert!(solver.solve().is_ok());
        assert_eq!(solver.finalize(&ret), ResolvedType::Bool);
    }

    #[test]
    fn test_occurs_check() {
        let mut solver = ConstraintSolver::new();
        let var = solver.fresh_var();

        // Try to create infinite type: τ0 = List<τ0>
        let infinite = ResolvedType::Named {
            name: "List".to_string(),
            generics: vec![var.clone()],
        };
        solver.add_equality(var, infinite, ConstraintReason::Annotation, None);

        assert!(solver.solve().is_err());
    }

    #[test]
    fn test_constraint_reason_description() {
        let reason = ConstraintReason::FunctionArg {
            func: "add".to_string(),
            arg_index: 0,
        };
        assert!(reason.describe().contains("argument 1"));
        assert!(reason.describe().contains("add"));
    }

    #[test]
    fn test_inference_result_certainty() {
        let explicit = InferenceResult::explicit(ResolvedType::Int32);
        let inferred = InferenceResult::inferred(ResolvedType::Int32, vec![]);
        let from_sig = InferenceResult::from_signature(ResolvedType::Int32);
        let from_pat = InferenceResult::from_pattern(ResolvedType::Int32, vec![]);

        assert_eq!(explicit.certainty, 1.0);
        assert_eq!(inferred.certainty, 0.95);
        assert_eq!(from_sig.certainty, 0.90);
        assert_eq!(from_pat.certainty, 0.85);
    }
}
