//! Type Checker (Prakāra Parīkṣaka)
//!
//! Implements type checking using Nyāya 4-pramāṇa inference:
//! 1. Pratyakṣa (प्रत्यक्ष) - Direct perception (explicit annotation) - 100% certain
//! 2. Anumāna (अनुमान) - Logical inference (deduction) - 95% certain
//! 3. Śabda (शब्द) - Testimony (documentation/contract) - 90% certain
//! 4. Upamāna (उपमान) - Comparison (pattern matching) - 85% certain
//!
//! The inference algorithm is based on Hindley-Milner Algorithm W with
//! adaptations for the Nyāya philosophical framework.

use crate::lexer::Span;
use crate::parser::ast::*;
use std::collections::HashMap;

// ============================================================================
// Type Variable System (for Unification)
// ============================================================================

/// Type variable identifier for unification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeVar(pub usize);

impl std::fmt::Display for TypeVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "τ{}", self.0)
    }
}

/// Type checker with Nyāya pramāṇa inference
pub struct TypeChecker {
    /// Symbol table with scoping
    scopes: Vec<HashMap<String, TypeInfo>>,
    /// Type inference engine
    inference: TypeInference,
    /// Current scope depth
    scope_depth: usize,
    /// Type definitions
    type_defs: HashMap<String, TypeDefInfo>,
    /// Function signatures (for śabda inference)
    function_sigs: HashMap<String, FunctionSig>,
    /// Method signatures per type (type_name -> method_name -> signature)
    method_sigs: HashMap<String, HashMap<String, MethodSig>>,
    /// Errors collected during type checking
    errors: Vec<TypeError>,
}

/// Type information for a symbol
#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub ty: ResolvedType,
    pub certainty: f32,
    pub pramana: Pramana,
    pub span: Option<Span>,
}

/// Stored type definition
#[derive(Debug, Clone)]
pub struct TypeDefInfo {
    pub name: String,
    pub generics: Vec<String>,
    pub body: TypeBodyResolved,
}

/// Resolved type body
#[derive(Debug, Clone)]
pub enum TypeBodyResolved {
    Struct(Vec<(String, ResolvedType)>),
    Enum(Vec<(String, Option<Vec<ResolvedType>>)>),
    Alias(ResolvedType),
}

/// Function signature for inference
#[derive(Debug, Clone)]
pub struct FunctionSig {
    pub name: String,
    pub params: Vec<(String, ResolvedType)>,
    pub return_type: ResolvedType,
    pub span: Option<Span>,
}

/// Method signature for inference (Vidhayaḥ Pariccheda)
#[derive(Debug, Clone)]
pub struct MethodSig {
    pub name: String,
    /// Self type for method receiver
    pub self_type: SelfType,
    /// Parameters (excluding self)
    pub params: Vec<(String, ResolvedType)>,
    pub return_type: ResolvedType,
    pub span: Option<Span>,
}

/// Self type for method receiver (Ātmaprakāra)
#[derive(Debug, Clone, PartialEq)]
pub enum SelfType {
    /// Value self (moves ownership)
    Value,
    /// Immutable reference (&self)
    Ref,
    /// Mutable reference (&mut self)
    RefMut,
}

/// Resolved type (after type checking)
#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedType {
    /// Primitive types (Mūla Prakāra)
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Bool,
    Unit,
    Char,
    String,

    /// Type variable (for inference) - Anirdhārita
    TypeVar(TypeVar),

    /// User-defined type - Nāmita Prakāra
    Named {
        name: String,
        generics: Vec<ResolvedType>,
    },

    /// Function type - Kāryakrama Prakāra
    Function {
        params: Vec<ResolvedType>,
        return_type: Box<ResolvedType>,
    },

    /// Reference type - Nirdeśa Prakāra
    Reference {
        inner: Box<ResolvedType>,
        mutable: bool,
        lifetime: Option<u8>,
    },

    /// Array type - Śreṇī Prakāra
    Array {
        element: Box<ResolvedType>,
        size: Option<usize>,
    },

    /// Tuple type - Yugma Prakāra
    Tuple(Vec<ResolvedType>),

    /// Never type (for diverging) - Kadāpi Na
    Never,

    /// Unknown (to be inferred) - Ajñāta
    Unknown,

    /// Error type (for error recovery) - Doṣa
    Error,
}

/// Nyāya pramāṇa (प्रमाण - means of valid knowledge)
///
/// The four valid sources of knowledge in Nyāya philosophy,
/// mapped to type inference certainty levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pramana {
    /// Pratyakṣa (प्रत्यक्ष) - Direct perception (explicit annotation)
    /// Certainty: 100% - The programmer explicitly stated the type
    Pratyaksha,

    /// Anumāna (अनुमान) - Inference (logical deduction)
    /// Certainty: 95% - Type deduced from logical reasoning
    /// Uses 5-step Nyāya syllogism (Pañcāvayava)
    Anumana,

    /// Śabda (शब्द) - Testimony (documentation/contract)
    /// Certainty: 90% - Type from authoritative source (function signature, docs)
    Shabda,

    /// Upamāna (उपमान) - Comparison (analogy/pattern matching)
    /// Certainty: 85% - Type inferred by similarity to known patterns
    Upamana,
}

impl Pramana {
    /// Get certainty level (0.0 - 1.0)
    pub fn certainty(&self) -> f32 {
        match self {
            Pramana::Pratyaksha => 1.0,
            Pramana::Anumana => 0.95,
            Pramana::Shabda => 0.90,
            Pramana::Upamana => 0.85,
        }
    }

    /// Sanskrit name with diacritics
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Pramana::Pratyaksha => "प्रत्यक्ष",
            Pramana::Anumana => "अनुमान",
            Pramana::Shabda => "शब्द",
            Pramana::Upamana => "उपमान",
        }
    }
}

// ============================================================================
// Type Inference Engine (Prakāra Anumāna Yantra)
// ============================================================================

/// Type inference engine implementing Algorithm W with Nyāya philosophy
struct TypeInference {
    /// Substitution map: TypeVar -> ResolvedType
    substitutions: HashMap<TypeVar, ResolvedType>,
    /// Next type variable ID
    next_var: usize,
}

impl TypeInference {
    fn new() -> Self {
        Self {
            substitutions: HashMap::new(),
            next_var: 0,
        }
    }

    /// Create a fresh type variable (Nūtana Anirdhārita)
    fn fresh_type_var(&mut self) -> ResolvedType {
        let var = TypeVar(self.next_var);
        self.next_var += 1;
        ResolvedType::TypeVar(var)
    }

    /// Apply substitutions to a type (Pratiyojana)
    fn apply(&self, ty: &ResolvedType) -> ResolvedType {
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
    /// Returns true if unification succeeds, false otherwise.
    fn unify(&mut self, t1: &ResolvedType, t2: &ResolvedType) -> Result<(), UnificationError> {
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

    /// Generalize a type to a type scheme (for let-polymorphism)
    fn generalize(&self, ty: &ResolvedType, env_vars: &[TypeVar]) -> ResolvedType {
        // Find free type variables in ty that are not in environment
        // For now, just return the applied type
        self.apply(ty)
    }

    /// Instantiate a type scheme with fresh variables
    fn instantiate(&mut self, ty: &ResolvedType) -> ResolvedType {
        // For polymorphic types, replace bound variables with fresh ones
        // For now, just clone the type
        ty.clone()
    }
}

/// Unification error types
#[derive(Debug, Clone)]
pub enum UnificationError {
    TypeMismatch {
        expected: ResolvedType,
        found: ResolvedType,
    },
    OccursCheck {
        var: TypeVar,
        ty: ResolvedType,
    },
    ArityMismatch {
        expected: usize,
        found: usize,
    },
    ArraySizeMismatch {
        expected: usize,
        found: usize,
    },
    TupleSizeMismatch {
        expected: usize,
        found: usize,
    },
    MutabilityMismatch {
        expected_mutable: bool,
    },
    GenericArityMismatch {
        ty: String,
        expected: usize,
        found: usize,
    },
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut checker = Self {
            scopes: vec![HashMap::new()], // Global scope
            inference: TypeInference::new(),
            scope_depth: 0,
            type_defs: HashMap::new(),
            function_sigs: HashMap::new(),
            method_sigs: HashMap::new(),
            errors: Vec::new(),
        };
        // Register builtin types and functions
        checker.register_builtins();
        checker
    }

    /// Register builtin types and functions
    fn register_builtins(&mut self) {
        // Register primitive type constructors
        // print function
        self.function_sigs.insert(
            "mudrā".to_string(),
            FunctionSig {
                name: "mudrā".to_string(),
                params: vec![("value".to_string(), ResolvedType::String)],
                return_type: ResolvedType::Unit,
                span: None,
            },
        );

        // exit function
        self.function_sigs.insert(
            "nirgama".to_string(),
            FunctionSig {
                name: "nirgama".to_string(),
                params: vec![("code".to_string(), ResolvedType::Int32)],
                return_type: ResolvedType::Never,
                span: None,
            },
        );
    }

    // ========================================================================
    // Public API
    // ========================================================================

    /// Check types for an entire AST (Sampūrṇa Parīkṣā)
    pub fn check(&mut self, ast: &Ast) -> Result<(), Vec<TypeError>> {
        // Phase 1: Collect all type definitions
        for item in &ast.items {
            if let Item::TypeDef(typedef) = item {
                self.collect_type_def(typedef);
            }
        }

        // Phase 2: Collect all function signatures (for śabda inference)
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.collect_function_sig(func);
            }
        }

        // Phase 3: Type check all items
        for item in &ast.items {
            self.check_item(item);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Collect type definition
    fn collect_type_def(&mut self, typedef: &TypeDef) {
        let name = typedef.name.name.clone();
        let generics: Vec<String> = typedef
            .generics
            .iter()
            .map(|g| g.name.name.clone())
            .collect();

        let body = match &typedef.body {
            TypeBody::Struct(fields) => {
                let resolved_fields: Vec<(String, ResolvedType)> = fields
                    .iter()
                    .map(|f| (f.name.name.clone(), self.resolve_ast_type(&f.ty)))
                    .collect();
                TypeBodyResolved::Struct(resolved_fields)
            }
            TypeBody::Enum(variants) => {
                let resolved_variants: Vec<(String, Option<Vec<ResolvedType>>)> = variants
                    .iter()
                    .map(|v| {
                        let fields = v
                            .fields
                            .as_ref()
                            .map(|fs| fs.iter().map(|f| self.resolve_ast_type(&f.ty)).collect());
                        (v.name.name.clone(), fields)
                    })
                    .collect();
                TypeBodyResolved::Enum(resolved_variants)
            }
            TypeBody::Alias(ty) => TypeBodyResolved::Alias(self.resolve_ast_type(ty)),
        };

        self.type_defs.insert(
            name.clone(),
            TypeDefInfo {
                name,
                generics,
                body,
            },
        );
    }

    /// Collect function signature
    fn collect_function_sig(&mut self, func: &FunctionDef) {
        let name = func.name.name.clone();
        let params: Vec<(String, ResolvedType)> = func
            .params
            .iter()
            .map(|p| (p.name.name.clone(), self.resolve_ast_type(&p.ty)))
            .collect();
        let return_type = func
            .return_type
            .as_ref()
            .map(|t| self.resolve_ast_type(t))
            .unwrap_or(ResolvedType::Unit);

        self.function_sigs.insert(
            name.clone(),
            FunctionSig {
                name,
                params,
                return_type,
                span: Some(func.span),
            },
        );
    }

    /// Check a single item
    fn check_item(&mut self, item: &Item) {
        match item {
            Item::Function(func) => self.check_function(func),
            Item::TypeDef(_) => { /* Already collected */ }
            Item::Constant(constant) => self.check_constant(constant),
            Item::Import(_) => { /* No type checking needed */ }
            Item::Module(module) => {
                for sub_item in &module.items {
                    self.check_item(sub_item);
                }
            }
        }
    }

    /// Check a function definition
    fn check_function(&mut self, func: &FunctionDef) {
        self.enter_scope();

        // Add parameters to scope with explicit types (Pratyakṣa)
        for param in &func.params {
            let ty = self.resolve_ast_type(&param.ty);
            self.add_symbol(
                param.name.name.clone(),
                TypeInfo {
                    ty,
                    certainty: Pramana::Pratyaksha.certainty(),
                    pramana: Pramana::Pratyaksha,
                    span: Some(param.span),
                },
            );
        }

        // Expected return type
        let expected_return = func
            .return_type
            .as_ref()
            .map(|t| self.resolve_ast_type(t))
            .unwrap_or(ResolvedType::Unit);

        // Check function body
        let body_type = self.check_block(&func.body);

        // Verify return type matches
        if let Err(e) = self.inference.unify(&body_type, &expected_return) {
            self.errors.push(TypeError::Mismatch {
                expected: self.inference.apply(&expected_return),
                found: self.inference.apply(&body_type),
                span: Some(func.span),
                context: format!("function '{}' return type", func.name.name),
            });
        }

        self.exit_scope();
    }

    /// Check a constant definition
    fn check_constant(&mut self, constant: &ConstantDef) {
        let value_type = self.infer_expr(&constant.value);

        if let Some(declared_ty) = &constant.ty {
            let declared = self.resolve_ast_type(declared_ty);
            if let Err(_) = self.inference.unify(&value_type.ty, &declared) {
                self.errors.push(TypeError::Mismatch {
                    expected: declared,
                    found: value_type.ty.clone(),
                    span: Some(constant.value.span()),
                    context: format!("constant '{}'", constant.name.name),
                });
            }
        }

        self.add_symbol(constant.name.name.clone(), value_type);
    }

    /// Check a block and return its type
    fn check_block(&mut self, block: &Block) -> ResolvedType {
        let mut last_type = ResolvedType::Unit;

        for stmt in &block.stmts {
            last_type = self.check_stmt(stmt);
        }

        last_type
    }

    /// Check a statement and return its type
    fn check_stmt(&mut self, stmt: &Stmt) -> ResolvedType {
        match stmt {
            Stmt::Let {
                name,
                ty,
                value,
                span,
            } => {
                let (resolved_ty, pramana) = if let Some(explicit_ty) = ty {
                    // Pratyakṣa: Explicit type annotation
                    let ty = self.resolve_ast_type(explicit_ty);
                    if let Some(val) = value {
                        let val_info = self.infer_expr(val);
                        if let Err(_) = self.inference.unify(&val_info.ty, &ty) {
                            self.errors.push(TypeError::Mismatch {
                                expected: ty.clone(),
                                found: val_info.ty,
                                span: Some(*span),
                                context: format!("let binding '{}'", name.name),
                            });
                        }
                    }
                    (ty, Pramana::Pratyaksha)
                } else if let Some(val) = value {
                    // Anumāna: Infer from value
                    let val_info = self.infer_expr(val);
                    (val_info.ty, val_info.pramana)
                } else {
                    // Unknown type - create fresh type variable
                    (self.inference.fresh_type_var(), Pramana::Anumana)
                };

                self.add_symbol(
                    name.name.clone(),
                    TypeInfo {
                        ty: resolved_ty,
                        certainty: pramana.certainty(),
                        pramana,
                        span: Some(*span),
                    },
                );
                ResolvedType::Unit
            }

            Stmt::Expr(expr) => self.infer_expr(expr).ty,

            Stmt::Return { value, span } => {
                if let Some(val) = value {
                    self.infer_expr(val).ty
                } else {
                    ResolvedType::Unit
                }
            }

            Stmt::If {
                condition,
                then_block,
                else_block,
                span,
            } => {
                // Condition must be Bool
                let cond_type = self.infer_expr(condition);
                if let Err(_) = self.inference.unify(&cond_type.ty, &ResolvedType::Bool) {
                    self.errors.push(TypeError::Mismatch {
                        expected: ResolvedType::Bool,
                        found: cond_type.ty,
                        span: Some(*span),
                        context: "if condition".to_string(),
                    });
                }

                let then_type = self.check_block(then_block);

                if let Some(else_blk) = else_block {
                    let else_type = self.check_block(else_blk);
                    // Both branches must have same type
                    if let Err(_) = self.inference.unify(&then_type, &else_type) {
                        self.errors.push(TypeError::BranchMismatch {
                            then_type: then_type.clone(),
                            else_type: else_type.clone(),
                            span: Some(*span),
                        });
                    }
                    then_type
                } else {
                    ResolvedType::Unit
                }
            }

            Stmt::Match {
                scrutinee,
                arms,
                span,
            } => {
                let scrutinee_type = self.infer_expr(scrutinee);
                let mut arm_type: Option<ResolvedType> = None;

                for arm in arms {
                    // Check pattern matches scrutinee type
                    self.check_pattern(&arm.pattern, &scrutinee_type.ty);

                    // Check arm body
                    let body_type = self.infer_expr(&arm.body);

                    if let Some(ref expected) = arm_type {
                        if let Err(_) = self.inference.unify(&body_type.ty, expected) {
                            self.errors.push(TypeError::BranchMismatch {
                                then_type: expected.clone(),
                                else_type: body_type.ty,
                                span: Some(arm.span),
                            });
                        }
                    } else {
                        arm_type = Some(body_type.ty);
                    }
                }

                arm_type.unwrap_or(ResolvedType::Unit)
            }

            Stmt::Loop { kind, body, span } => {
                match kind {
                    LoopKind::ForIn { binding, iterable } => {
                        let iter_type = self.infer_expr(iterable);
                        // Extract element type from iterable
                        let elem_type = self.extract_element_type(&iter_type.ty);
                        self.enter_scope();
                        self.add_symbol(
                            binding.name.clone(),
                            TypeInfo {
                                ty: elem_type,
                                certainty: Pramana::Anumana.certainty(),
                                pramana: Pramana::Anumana,
                                span: Some(*span),
                            },
                        );
                        self.check_block(body);
                        self.exit_scope();
                    }
                    LoopKind::While { condition } => {
                        let cond_type = self.infer_expr(condition);
                        if let Err(_) = self.inference.unify(&cond_type.ty, &ResolvedType::Bool) {
                            self.errors.push(TypeError::Mismatch {
                                expected: ResolvedType::Bool,
                                found: cond_type.ty,
                                span: Some(*span),
                                context: "while condition".to_string(),
                            });
                        }
                        self.check_block(body);
                    }
                    LoopKind::Range {
                        binding,
                        start,
                        end,
                        ..
                    } => {
                        let start_type = self.infer_expr(start);
                        let end_type = self.infer_expr(end);
                        // Start and end should be integers
                        if let Err(_) = self.inference.unify(&start_type.ty, &ResolvedType::Int64) {
                            // Allow any integer type
                        }
                        self.enter_scope();
                        self.add_symbol(
                            binding.name.clone(),
                            TypeInfo {
                                ty: start_type.ty,
                                certainty: Pramana::Anumana.certainty(),
                                pramana: Pramana::Anumana,
                                span: Some(*span),
                            },
                        );
                        self.check_block(body);
                        self.exit_scope();
                    }
                    LoopKind::Infinite => {
                        self.check_block(body);
                    }
                }
                ResolvedType::Unit
            }

            Stmt::Break { .. } | Stmt::Continue { .. } => ResolvedType::Unit,
        }
    }

    /// Check a pattern against an expected type
    fn check_pattern(&mut self, pattern: &Pattern, expected: &ResolvedType) {
        match pattern {
            Pattern::Identifier(id) => {
                self.add_symbol(
                    id.name.clone(),
                    TypeInfo {
                        ty: expected.clone(),
                        certainty: Pramana::Upamana.certainty(),
                        pramana: Pramana::Upamana,
                        span: Some(id.span),
                    },
                );
            }
            Pattern::Literal(lit) => {
                let lit_type = self.infer_literal(lit);
                if let Err(_) = self.inference.unify(&lit_type, expected) {
                    // Pattern type mismatch - could add error
                }
            }
            Pattern::Constructor { name, fields } => {
                // Check constructor pattern against type definition
                if let Some(typedef) = self.type_defs.get(&name.name).cloned() {
                    if let TypeBodyResolved::Enum(variants) = &typedef.body {
                        // Find matching variant
                        if let Some((_, variant_fields)) =
                            variants.iter().find(|(n, _)| n == &name.name)
                        {
                            if let Some(field_types) = variant_fields {
                                for (pattern, ty) in fields.iter().zip(field_types.iter()) {
                                    self.check_pattern(pattern, ty);
                                }
                            }
                        }
                    }
                }
            }
            Pattern::Wildcard | Pattern::Rest => { /* Match anything */ }
        }
    }

    /// Extract element type from array/iterator type
    fn extract_element_type(&self, ty: &ResolvedType) -> ResolvedType {
        match ty {
            ResolvedType::Array { element, .. } => (**element).clone(),
            ResolvedType::Named { name, generics } if name == "Vec" || name == "Iterator" => {
                generics.first().cloned().unwrap_or(ResolvedType::Unknown)
            }
            _ => ResolvedType::Unknown,
        }
    }

    /// Infer type of an expression using Nyāya pramāṇas
    pub fn infer_type(&self, expr: &Expr) -> Result<TypeInfo, TypeError> {
        // Try pramāṇas in order of certainty:
        // 1. Pratyakṣa (explicit type)
        if let Some(ty) = self.pratyaksha_lookup(expr) {
            return Ok(ty);
        }

        // 2. Anumāna (inference)
        if let Some(ty) = self.anumana_infer(expr) {
            return Ok(ty);
        }

        // 3. Śabda (documentation contract)
        if let Some(ty) = self.shabda_contract(expr) {
            return Ok(ty);
        }

        // 4. Upamāna (pattern match)
        if let Some(ty) = self.upamana_match(expr) {
            return Ok(ty);
        }

        Err(TypeError::CannotInfer {
            pramanas_tried: vec![
                Pramana::Pratyaksha,
                Pramana::Anumana,
                Pramana::Shabda,
                Pramana::Upamana,
            ],
            evidence: self.collect_evidence(expr),
        })
    }

    /// Full expression type inference (mutable version)
    pub fn infer_expr(&mut self, expr: &Expr) -> TypeInfo {
        match expr {
            Expr::Literal(lit) => {
                let ty = self.infer_literal(lit);
                TypeInfo {
                    ty,
                    certainty: Pramana::Pratyaksha.certainty(),
                    pramana: Pramana::Pratyaksha,
                    span: None,
                }
            }

            Expr::Identifier(id) => {
                // Look up in symbol table (Pratyakṣa or previously inferred)
                if let Some(info) = self.lookup(&id.name) {
                    info.clone()
                } else {
                    self.errors.push(TypeError::UnknownIdentifier {
                        name: id.name.clone(),
                        span: Some(id.span),
                    });
                    TypeInfo {
                        ty: ResolvedType::Error,
                        certainty: 0.0,
                        pramana: Pramana::Anumana,
                        span: Some(id.span),
                    }
                }
            }

            Expr::Binary {
                left,
                op,
                right,
                span,
            } => {
                let left_info = self.infer_expr(left);
                let right_info = self.infer_expr(right);
                let result_type = self.infer_binary_op(*op, &left_info.ty, &right_info.ty);

                // Unify operand types for arithmetic
                if is_arithmetic_op(*op) {
                    if let Err(_) = self.inference.unify(&left_info.ty, &right_info.ty) {
                        self.errors.push(TypeError::BinaryOpMismatch {
                            op: format!("{:?}", op),
                            left_type: left_info.ty,
                            right_type: right_info.ty,
                            span: Some(*span),
                        });
                    }
                }

                TypeInfo {
                    ty: result_type,
                    certainty: Pramana::Anumana.certainty(),
                    pramana: Pramana::Anumana,
                    span: Some(*span),
                }
            }

            Expr::Unary { op, operand, span } => {
                let operand_info = self.infer_expr(operand);
                let result_type = self.infer_unary_op(*op, &operand_info.ty);

                TypeInfo {
                    ty: result_type,
                    certainty: Pramana::Anumana.certainty(),
                    pramana: Pramana::Anumana,
                    span: Some(*span),
                }
            }

            Expr::Call { callee, args, span } => {
                // Śabda: Use function signature if available
                if let Expr::Identifier(func_id) = callee.as_ref() {
                    if let Some(sig) = self.function_sigs.get(&func_id.name).cloned() {
                        // Check argument count
                        if args.len() != sig.params.len() {
                            self.errors.push(TypeError::ArityMismatch {
                                function: func_id.name.clone(),
                                expected: sig.params.len(),
                                found: args.len(),
                                span: Some(*span),
                            });
                        }

                        // Check argument types
                        for (arg, (param_name, param_ty)) in args.iter().zip(sig.params.iter()) {
                            let arg_info = self.infer_expr(arg);
                            if let Err(_) = self.inference.unify(&arg_info.ty, param_ty) {
                                self.errors.push(TypeError::ArgumentMismatch {
                                    function: func_id.name.clone(),
                                    param: param_name.clone(),
                                    expected: param_ty.clone(),
                                    found: arg_info.ty,
                                    span: Some(*span),
                                });
                            }
                        }

                        return TypeInfo {
                            ty: sig.return_type,
                            certainty: Pramana::Shabda.certainty(),
                            pramana: Pramana::Shabda,
                            span: Some(*span),
                        };
                    }
                }

                // Fall back to inferring callee type
                let callee_info = self.infer_expr(callee);
                match callee_info.ty {
                    ResolvedType::Function { return_type, .. } => TypeInfo {
                        ty: *return_type,
                        certainty: Pramana::Anumana.certainty(),
                        pramana: Pramana::Anumana,
                        span: Some(*span),
                    },
                    _ => TypeInfo {
                        ty: ResolvedType::Unknown,
                        certainty: 0.5,
                        pramana: Pramana::Upamana,
                        span: Some(*span),
                    },
                }
            }

            Expr::MethodCall {
                receiver,
                method,
                args,
                span,
            } => {
                let receiver_info = self.infer_expr(receiver);

                // Resolve the type name for method lookup
                let type_name = self.get_type_name(&receiver_info.ty);

                // Look up method on receiver type
                if let Some(method_sig) = self.lookup_method(&type_name, &method.name) {
                    // Type check arguments against method signature
                    for (_i, (arg, (param_name, param_type))) in
                        args.iter().zip(method_sig.params.iter()).enumerate()
                    {
                        let arg_info = self.infer_expr(arg);

                        // Unify argument type with parameter type
                        if let Err(_e) = self.inference.unify(&arg_info.ty, param_type) {
                            self.errors.push(TypeError::ArgumentMismatch {
                                function: format!("{}.{}", type_name, method.name),
                                param: param_name.clone(),
                                expected: param_type.clone(),
                                found: arg_info.ty.clone(),
                                span: arg_info.span,
                            });
                        }
                    }

                    // Check argument count
                    if args.len() != method_sig.params.len() {
                        self.errors.push(TypeError::ArityMismatch {
                            function: format!("{}.{}", type_name, method.name),
                            expected: method_sig.params.len(),
                            found: args.len(),
                            span: Some(*span),
                        });
                    }

                    TypeInfo {
                        ty: method_sig.return_type.clone(),
                        certainty: Pramana::Shabda.certainty(),
                        pramana: Pramana::Shabda,
                        span: Some(*span),
                    }
                } else {
                    // Method not found - use UnknownIdentifier for now
                    self.errors.push(TypeError::UnknownIdentifier {
                        name: format!("{}.{}", type_name, method.name),
                        span: Some(*span),
                    });

                    TypeInfo {
                        ty: ResolvedType::Unknown,
                        certainty: Pramana::Upamana.certainty(),
                        pramana: Pramana::Upamana,
                        span: Some(*span),
                    }
                }
            }

            Expr::FieldAccess {
                object,
                field,
                span,
            } => {
                let object_info = self.infer_expr(object);
                // Look up field type in struct definition
                let field_type = self.lookup_field_type(&object_info.ty, &field.name);

                TypeInfo {
                    ty: field_type,
                    certainty: Pramana::Anumana.certainty(),
                    pramana: Pramana::Anumana,
                    span: Some(*span),
                }
            }

            Expr::Index {
                object,
                index,
                span,
            } => {
                let object_info = self.infer_expr(object);
                let index_info = self.infer_expr(index);

                // Index should be integer
                if let Err(_) = self.inference.unify(&index_info.ty, &ResolvedType::Int64) {
                    // Could also be usize - try other integer types
                }

                let elem_type = self.extract_element_type(&object_info.ty);

                TypeInfo {
                    ty: elem_type,
                    certainty: Pramana::Anumana.certainty(),
                    pramana: Pramana::Anumana,
                    span: Some(*span),
                }
            }

            Expr::StructConstruct { name, fields, span } => {
                // Pratyakṣa: Struct name is explicit
                let struct_type = ResolvedType::Named {
                    name: name.name.clone(),
                    generics: vec![],
                };

                // Check field types if struct definition exists
                if let Some(typedef) = self.type_defs.get(&name.name).cloned() {
                    if let TypeBodyResolved::Struct(struct_fields) = &typedef.body {
                        for (field_name, field_expr) in fields {
                            let expr_info = self.infer_expr(field_expr);
                            if let Some((_, expected_ty)) =
                                struct_fields.iter().find(|(n, _)| n == &field_name.name)
                            {
                                if let Err(_) = self.inference.unify(&expr_info.ty, expected_ty) {
                                    self.errors.push(TypeError::FieldTypeMismatch {
                                        struct_name: name.name.clone(),
                                        field: field_name.name.clone(),
                                        expected: expected_ty.clone(),
                                        found: expr_info.ty,
                                        span: Some(*span),
                                    });
                                }
                            }
                        }
                    }
                }

                TypeInfo {
                    ty: struct_type,
                    certainty: Pramana::Pratyaksha.certainty(),
                    pramana: Pramana::Pratyaksha,
                    span: Some(*span),
                }
            }

            Expr::Array { elements, span } => {
                let elem_type = if let Some(first) = elements.first() {
                    let first_info = self.infer_expr(first);
                    // Check all elements have same type
                    for elem in elements.iter().skip(1) {
                        let elem_info = self.infer_expr(elem);
                        if let Err(_) = self.inference.unify(&elem_info.ty, &first_info.ty) {
                            self.errors.push(TypeError::ArrayElementMismatch {
                                expected: first_info.ty.clone(),
                                found: elem_info.ty,
                                span: Some(*span),
                            });
                        }
                    }
                    first_info.ty
                } else {
                    self.inference.fresh_type_var()
                };

                TypeInfo {
                    ty: ResolvedType::Array {
                        element: Box::new(elem_type),
                        size: Some(elements.len()),
                    },
                    certainty: Pramana::Anumana.certainty(),
                    pramana: Pramana::Anumana,
                    span: Some(*span),
                }
            }

            Expr::Tuple { elements, span } => {
                let elem_types: Vec<ResolvedType> =
                    elements.iter().map(|e| self.infer_expr(e).ty).collect();

                TypeInfo {
                    ty: ResolvedType::Tuple(elem_types),
                    certainty: Pramana::Anumana.certainty(),
                    pramana: Pramana::Anumana,
                    span: Some(*span),
                }
            }

            Expr::Lambda { params, body, span } => {
                self.enter_scope();

                // Add parameters to scope
                let param_types: Vec<ResolvedType> = params
                    .iter()
                    .map(|p| {
                        let ty = self.resolve_ast_type(&p.ty);
                        self.add_symbol(
                            p.name.name.clone(),
                            TypeInfo {
                                ty: ty.clone(),
                                certainty: Pramana::Pratyaksha.certainty(),
                                pramana: Pramana::Pratyaksha,
                                span: Some(p.span),
                            },
                        );
                        ty
                    })
                    .collect();

                let body_info = self.infer_expr(body);
                self.exit_scope();

                TypeInfo {
                    ty: ResolvedType::Function {
                        params: param_types,
                        return_type: Box::new(body_info.ty),
                    },
                    certainty: Pramana::Anumana.certainty(),
                    pramana: Pramana::Anumana,
                    span: Some(*span),
                }
            }

            Expr::Block(block) => {
                self.enter_scope();
                let result_type = self.check_block(block);
                self.exit_scope();

                TypeInfo {
                    ty: result_type,
                    certainty: Pramana::Anumana.certainty(),
                    pramana: Pramana::Anumana,
                    span: Some(block.span),
                }
            }

            Expr::If {
                condition,
                then_expr,
                else_expr,
                span,
            } => {
                let cond_info = self.infer_expr(condition);
                if let Err(_) = self.inference.unify(&cond_info.ty, &ResolvedType::Bool) {
                    self.errors.push(TypeError::Mismatch {
                        expected: ResolvedType::Bool,
                        found: cond_info.ty,
                        span: Some(*span),
                        context: "if condition".to_string(),
                    });
                }

                let then_info = self.infer_expr(then_expr);

                if let Some(else_e) = else_expr {
                    let else_info = self.infer_expr(else_e);
                    if let Err(_) = self.inference.unify(&then_info.ty, &else_info.ty) {
                        self.errors.push(TypeError::BranchMismatch {
                            then_type: then_info.ty.clone(),
                            else_type: else_info.ty,
                            span: Some(*span),
                        });
                    }
                    then_info
                } else {
                    TypeInfo {
                        ty: ResolvedType::Unit,
                        certainty: Pramana::Anumana.certainty(),
                        pramana: Pramana::Anumana,
                        span: Some(*span),
                    }
                }
            }

            Expr::Cast { expr, ty, span } => {
                // Pratyakṣa: Cast has explicit target type
                let _expr_info = self.infer_expr(expr);
                let target_type = self.resolve_ast_type(ty);

                TypeInfo {
                    ty: target_type,
                    certainty: Pramana::Pratyaksha.certainty(),
                    pramana: Pramana::Pratyaksha,
                    span: Some(*span),
                }
            }

            Expr::Try { expr, span } => {
                let expr_info = self.infer_expr(expr);
                // Try operator unwraps Result/Option types
                // For now, just return the inner type or Unknown
                TypeInfo {
                    ty: expr_info.ty,
                    certainty: Pramana::Anumana.certainty(),
                    pramana: Pramana::Anumana,
                    span: Some(*span),
                }
            }

            Expr::Await { expr, span } => {
                let expr_info = self.infer_expr(expr);
                // Await unwraps Future types
                TypeInfo {
                    ty: expr_info.ty,
                    certainty: Pramana::Anumana.certainty(),
                    pramana: Pramana::Anumana,
                    span: Some(*span),
                }
            }
        }
    }

    // ========================================================================
    // Pramāṇa Inference Methods (प्रमाण अनुमान विधि)
    // ========================================================================

    /// Pratyakṣa (प्रत्यक्ष) - Direct observation (explicit type annotation)
    ///
    /// This is the highest certainty inference - the programmer explicitly stated the type.
    /// Note: This is the immutable version for use in infer_type
    fn pratyaksha_lookup(&self, expr: &Expr) -> Option<TypeInfo> {
        match expr {
            // Struct construction has explicit type
            Expr::StructConstruct { name, span, .. } => Some(TypeInfo {
                ty: ResolvedType::Named {
                    name: name.name.clone(),
                    generics: vec![],
                },
                certainty: Pramana::Pratyaksha.certainty(),
                pramana: Pramana::Pratyaksha,
                span: Some(*span),
            }),

            // Literals have explicit types
            Expr::Literal(lit) => Some(TypeInfo {
                ty: self.infer_literal(lit),
                certainty: Pramana::Pratyaksha.certainty(),
                pramana: Pramana::Pratyaksha,
                span: None,
            }),

            // Cast expressions - need mutable access to resolve type
            // So skip in immutable version
            _ => None,
        }
    }

    /// Anumāna (अनुमान) - Logical inference using 5-step Nyāya syllogism
    ///
    /// The Nyāya syllogism (Pañcāvayava):
    /// 1. Pratijñā (प्रतिज्ञा) - Proposition: "x has type T"
    /// 2. Hetu (हेतु) - Reason: "because x is used as T"
    /// 3. Udāharaṇa (उदाहरण) - Example: "like other T values"
    /// 4. Upanaya (उपनय) - Application: "x is similar"
    /// 5. Nigamana (निगमन) - Conclusion: "therefore x: T"
    fn anumana_infer(&self, expr: &Expr) -> Option<TypeInfo> {
        match expr {
            // Binary operations - infer from operation semantics
            Expr::Binary {
                op,
                left,
                right,
                span,
            } => {
                // Hetu: The operation determines the result type
                let result_type = match op {
                    // Comparison operators always return Bool
                    BinaryOp::Eq
                    | BinaryOp::Ne
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge => ResolvedType::Bool,

                    // Logical operators work on Bool and return Bool
                    BinaryOp::And | BinaryOp::Or => ResolvedType::Bool,

                    // Arithmetic: result type matches operand type (inferred later)
                    _ => return None, // Let full inference handle this
                };

                Some(TypeInfo {
                    ty: result_type,
                    certainty: Pramana::Anumana.certainty(),
                    pramana: Pramana::Anumana,
                    span: Some(*span),
                })
            }

            // Unary negation on bool returns bool
            Expr::Unary {
                op: UnaryOp::Not,
                span,
                ..
            } => Some(TypeInfo {
                ty: ResolvedType::Bool,
                certainty: Pramana::Anumana.certainty(),
                pramana: Pramana::Anumana,
                span: Some(*span),
            }),

            // Identifier lookup from symbol table
            Expr::Identifier(id) => {
                // Pratijñā: "x has type T"
                // Hetu: "because x was declared with type T"
                self.lookup(&id.name).cloned()
            }

            _ => None,
        }
    }

    /// Śabda (शब्द) - Documentation/contract based inference
    ///
    /// Uses authoritative testimony (function signatures, documentation) to determine types.
    fn shabda_contract(&self, expr: &Expr) -> Option<TypeInfo> {
        match expr {
            // Function call - use signature from contract
            Expr::Call { callee, span, .. } => {
                if let Expr::Identifier(func_id) = callee.as_ref() {
                    if let Some(sig) = self.function_sigs.get(&func_id.name) {
                        return Some(TypeInfo {
                            ty: sig.return_type.clone(),
                            certainty: Pramana::Shabda.certainty(),
                            pramana: Pramana::Shabda,
                            span: Some(*span),
                        });
                    }
                }
                None
            }

            _ => None,
        }
    }

    /// Upamāna (उपमान) - Pattern matching inference by analogy
    ///
    /// Infers type by comparing with similar known expressions.
    fn upamana_match(&self, expr: &Expr) -> Option<TypeInfo> {
        match expr {
            // Array literal - infer element type from first element
            Expr::Array { elements, span } => {
                if let Some(first) = elements.first() {
                    if let Some(first_info) = self
                        .pratyaksha_lookup(first)
                        .or_else(|| self.anumana_infer(first))
                    {
                        return Some(TypeInfo {
                            ty: ResolvedType::Array {
                                element: Box::new(first_info.ty),
                                size: Some(elements.len()),
                            },
                            certainty: Pramana::Upamana.certainty(),
                            pramana: Pramana::Upamana,
                            span: Some(*span),
                        });
                    }
                }
                None
            }

            _ => None,
        }
    }

    /// Collect evidence for error reporting
    fn collect_evidence(&self, expr: &Expr) -> Vec<String> {
        let mut evidence = vec![];

        match expr {
            Expr::Identifier(id) => {
                evidence.push(format!("Identifier '{}' not found in scope", id.name));
            }
            Expr::Call { callee, .. } => {
                if let Expr::Identifier(func_id) = callee.as_ref() {
                    if !self.function_sigs.contains_key(&func_id.name) {
                        evidence.push(format!("Function '{}' signature not found", func_id.name));
                    }
                }
            }
            _ => {}
        }

        evidence
    }

    // ========================================================================
    // Helper Methods (Sahāyaka Vidhayaḥ)
    // ========================================================================

    /// Enter a new scope
    pub fn enter_scope(&mut self) {
        self.scope_depth += 1;
        self.scopes.push(HashMap::new());
    }

    /// Exit current scope
    pub fn exit_scope(&mut self) {
        self.scope_depth = self.scope_depth.saturating_sub(1);
        self.scopes.pop();
    }

    /// Add a symbol to the current scope
    pub fn add_symbol(&mut self, name: String, ty: TypeInfo) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, ty);
        }
    }

    /// Look up a symbol (searches from innermost to outermost scope)
    pub fn lookup(&self, name: &str) -> Option<&TypeInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }
        None
    }

    /// Resolve AST type to ResolvedType
    fn resolve_ast_type(&mut self, ty: &Type) -> ResolvedType {
        match ty {
            Type::Named { name, generics, .. } => {
                // Check for primitive types
                match name.name.as_str() {
                    "i8" | "saṅkhyā8" => ResolvedType::Int8,
                    "i16" | "saṅkhyā16" => ResolvedType::Int16,
                    "i32" | "saṅkhyā32" | "saṅkhyā" => ResolvedType::Int32,
                    "i64" | "saṅkhyā64" => ResolvedType::Int64,
                    "u8" => ResolvedType::UInt8,
                    "u16" => ResolvedType::UInt16,
                    "u32" => ResolvedType::UInt32,
                    "u64" => ResolvedType::UInt64,
                    "f32" | "daśamika32" => ResolvedType::Float32,
                    "f64" | "daśamika64" | "daśamika" => ResolvedType::Float64,
                    "bool" | "satya" => ResolvedType::Bool,
                    "()" | "śūnya" => ResolvedType::Unit,
                    "char" | "akṣara" => ResolvedType::Char,
                    "str" | "sūtra" => ResolvedType::String,
                    "String" | "Sūtra" => ResolvedType::String,
                    "!" | "kadāpi_na" => ResolvedType::Never,
                    _ => {
                        let resolved_generics: Vec<ResolvedType> =
                            generics.iter().map(|g| self.resolve_ast_type(g)).collect();
                        ResolvedType::Named {
                            name: name.name.clone(),
                            generics: resolved_generics,
                        }
                    }
                }
            }
            Type::Function {
                params,
                return_type,
            } => {
                let resolved_params: Vec<ResolvedType> =
                    params.iter().map(|p| self.resolve_ast_type(p)).collect();
                let resolved_return = self.resolve_ast_type(return_type);
                ResolvedType::Function {
                    params: resolved_params,
                    return_type: Box::new(resolved_return),
                }
            }
            Type::Array { element, size } => {
                let resolved_element = self.resolve_ast_type(element);
                ResolvedType::Array {
                    element: Box::new(resolved_element),
                    size: *size,
                }
            }
            Type::Tuple(elems) => {
                let resolved_elems: Vec<ResolvedType> =
                    elems.iter().map(|e| self.resolve_ast_type(e)).collect();
                ResolvedType::Tuple(resolved_elems)
            }
            Type::Reference {
                inner,
                mutable,
                lifetime,
            } => {
                let resolved_inner = self.resolve_ast_type(inner);
                ResolvedType::Reference {
                    inner: Box::new(resolved_inner),
                    mutable: *mutable,
                    lifetime: *lifetime,
                }
            }
            Type::Inferred => self.inference.fresh_type_var(),
        }
    }

    /// Infer type from literal
    fn infer_literal(&self, lit: &Literal) -> ResolvedType {
        match lit {
            Literal::Int(n) => {
                // Choose appropriate integer type based on value
                if *n >= i8::MIN as i64 && *n <= i8::MAX as i64 {
                    ResolvedType::Int32 // Default to i32 for small integers
                } else if *n >= i32::MIN as i64 && *n <= i32::MAX as i64 {
                    ResolvedType::Int32
                } else {
                    ResolvedType::Int64
                }
            }
            Literal::Float(_) => ResolvedType::Float64,
            Literal::String(_) => ResolvedType::String,
            Literal::Bool(_) => ResolvedType::Bool,
            Literal::Char(_) => ResolvedType::Char,
            Literal::Unit => ResolvedType::Unit,
        }
    }

    /// Infer result type of binary operation
    fn infer_binary_op(
        &self,
        op: BinaryOp,
        left: &ResolvedType,
        right: &ResolvedType,
    ) -> ResolvedType {
        match op {
            // Comparison operators return Bool
            BinaryOp::Eq
            | BinaryOp::Ne
            | BinaryOp::Lt
            | BinaryOp::Le
            | BinaryOp::Gt
            | BinaryOp::Ge => ResolvedType::Bool,

            // Logical operators return Bool
            BinaryOp::And | BinaryOp::Or => ResolvedType::Bool,

            // Arithmetic operators return the operand type
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                // Use the more specific type
                if *left != ResolvedType::Unknown {
                    left.clone()
                } else {
                    right.clone()
                }
            }

            // Bitwise operators return integer type
            BinaryOp::BitAnd
            | BinaryOp::BitOr
            | BinaryOp::BitXor
            | BinaryOp::Shl
            | BinaryOp::Shr => left.clone(),

            // Assignment returns Unit
            BinaryOp::Assign
            | BinaryOp::AddAssign
            | BinaryOp::SubAssign
            | BinaryOp::MulAssign
            | BinaryOp::DivAssign => ResolvedType::Unit,
        }
    }

    /// Infer result type of unary operation
    fn infer_unary_op(&self, op: UnaryOp, operand: &ResolvedType) -> ResolvedType {
        match op {
            UnaryOp::Neg => operand.clone(),
            UnaryOp::Not => ResolvedType::Bool,
            UnaryOp::Ref => ResolvedType::Reference {
                inner: Box::new(operand.clone()),
                mutable: false,
                lifetime: None,
            },
            UnaryOp::Deref => {
                if let ResolvedType::Reference { inner, .. } = operand {
                    (**inner).clone()
                } else {
                    ResolvedType::Error
                }
            }
        }
    }

    /// Look up field type in a struct
    fn lookup_field_type(&self, ty: &ResolvedType, field_name: &str) -> ResolvedType {
        if let ResolvedType::Named { name, .. } = ty {
            if let Some(typedef) = self.type_defs.get(name) {
                if let TypeBodyResolved::Struct(fields) = &typedef.body {
                    if let Some((_, field_ty)) = fields.iter().find(|(n, _)| n == field_name) {
                        return field_ty.clone();
                    }
                }
            }
        }
        ResolvedType::Unknown
    }

    /// Get the type name for method lookup (Prakāra Nāma)
    fn get_type_name(&self, ty: &ResolvedType) -> String {
        match self.inference.apply(ty) {
            ResolvedType::Named { name, .. } => name,
            ResolvedType::Int8 => "i8".to_string(),
            ResolvedType::Int16 => "i16".to_string(),
            ResolvedType::Int32 => "i32".to_string(),
            ResolvedType::Int64 => "i64".to_string(),
            ResolvedType::UInt8 => "u8".to_string(),
            ResolvedType::UInt16 => "u16".to_string(),
            ResolvedType::UInt32 => "u32".to_string(),
            ResolvedType::UInt64 => "u64".to_string(),
            ResolvedType::Float32 => "f32".to_string(),
            ResolvedType::Float64 => "f64".to_string(),
            ResolvedType::Bool => "bool".to_string(),
            ResolvedType::Char => "char".to_string(),
            ResolvedType::String => "String".to_string(),
            ResolvedType::Array { .. } => "Array".to_string(),
            ResolvedType::Tuple(_) => "Tuple".to_string(),
            ResolvedType::Reference { inner, .. } => self.get_type_name(&inner),
            _ => "unknown".to_string(),
        }
    }

    /// Look up a method on a type (Vidhayaḥ Anveṣaṇa)
    fn lookup_method(&self, type_name: &str, method_name: &str) -> Option<MethodSig> {
        // First, try direct lookup
        if let Some(methods) = self.method_sigs.get(type_name) {
            if let Some(method) = methods.get(method_name) {
                return Some(method.clone());
            }
        }

        // For primitive types, check builtin methods
        match type_name {
            "String" | "Sūtra" => self.lookup_string_method(method_name),
            "Array" | "Śreṇī" => self.lookup_array_method(method_name),
            "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" => {
                self.lookup_integer_method(method_name)
            }
            "f32" | "f64" => self.lookup_float_method(method_name),
            _ => None,
        }
    }

    /// Builtin String methods
    fn lookup_string_method(&self, method_name: &str) -> Option<MethodSig> {
        match method_name {
            "len" | "dīrghatā" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::Ref,
                params: vec![],
                return_type: ResolvedType::UInt64,
                span: None,
            }),
            "is_empty" | "śūnyam" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::Ref,
                params: vec![],
                return_type: ResolvedType::Bool,
                span: None,
            }),
            "push" | "yojaya" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::RefMut,
                params: vec![("c".to_string(), ResolvedType::Char)],
                return_type: ResolvedType::Unit,
                span: None,
            }),
            "push_str" | "sūtra_yojaya" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::RefMut,
                params: vec![("s".to_string(), ResolvedType::String)],
                return_type: ResolvedType::Unit,
                span: None,
            }),
            "contains" | "antarbhavati" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::Ref,
                params: vec![("pattern".to_string(), ResolvedType::String)],
                return_type: ResolvedType::Bool,
                span: None,
            }),
            _ => None,
        }
    }

    /// Builtin Array methods
    fn lookup_array_method(&self, method_name: &str) -> Option<MethodSig> {
        match method_name {
            "len" | "dīrghatā" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::Ref,
                params: vec![],
                return_type: ResolvedType::UInt64,
                span: None,
            }),
            "is_empty" | "śūnyam" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::Ref,
                params: vec![],
                return_type: ResolvedType::Bool,
                span: None,
            }),
            "push" | "yojaya" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::RefMut,
                params: vec![("elem".to_string(), ResolvedType::Unknown)], // Generic element
                return_type: ResolvedType::Unit,
                span: None,
            }),
            "pop" | "niṣkāsaya" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::RefMut,
                params: vec![],
                return_type: ResolvedType::Unknown, // Option<T>
                span: None,
            }),
            _ => None,
        }
    }

    /// Builtin integer methods
    fn lookup_integer_method(&self, method_name: &str) -> Option<MethodSig> {
        match method_name {
            "abs" | "nirapeṣa" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::Value,
                params: vec![],
                return_type: ResolvedType::Int64, // Will be same as self type
                span: None,
            }),
            "to_string" | "sūtram" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::Value,
                params: vec![],
                return_type: ResolvedType::String,
                span: None,
            }),
            "checked_add" | "surakṣita_yoga" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::Value,
                params: vec![("rhs".to_string(), ResolvedType::Int64)],
                return_type: ResolvedType::Unknown, // Option<Self>
                span: None,
            }),
            "saturating_add" | "paripūrṇa_yoga" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::Value,
                params: vec![("rhs".to_string(), ResolvedType::Int64)],
                return_type: ResolvedType::Int64,
                span: None,
            }),
            _ => None,
        }
    }

    /// Builtin float methods
    fn lookup_float_method(&self, method_name: &str) -> Option<MethodSig> {
        match method_name {
            "abs" | "nirapeṣa" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::Value,
                params: vec![],
                return_type: ResolvedType::Float64,
                span: None,
            }),
            "floor" | "bhūmi" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::Value,
                params: vec![],
                return_type: ResolvedType::Float64,
                span: None,
            }),
            "ceil" | "chatra" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::Value,
                params: vec![],
                return_type: ResolvedType::Float64,
                span: None,
            }),
            "round" | "vartula" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::Value,
                params: vec![],
                return_type: ResolvedType::Float64,
                span: None,
            }),
            "sqrt" | "vargamūla" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::Value,
                params: vec![],
                return_type: ResolvedType::Float64,
                span: None,
            }),
            "sin" | "jyā" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::Value,
                params: vec![],
                return_type: ResolvedType::Float64,
                span: None,
            }),
            "cos" | "koṭijyā" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::Value,
                params: vec![],
                return_type: ResolvedType::Float64,
                span: None,
            }),
            "to_string" | "sūtram" => Some(MethodSig {
                name: method_name.to_string(),
                self_type: SelfType::Value,
                params: vec![],
                return_type: ResolvedType::String,
                span: None,
            }),
            _ => None,
        }
    }

    /// Register a method for a type (Vidhayaḥ Nibandhanam)
    pub fn register_method(&mut self, type_name: String, method: MethodSig) {
        self.method_sigs
            .entry(type_name)
            .or_insert_with(HashMap::new)
            .insert(method.name.clone(), method);
    }

    /// Get the final type after applying all substitutions
    pub fn finalize_type(&self, ty: &ResolvedType) -> ResolvedType {
        self.inference.apply(ty)
    }

    /// Get all type errors
    pub fn errors(&self) -> &[TypeError] {
        &self.errors
    }
}

/// Check if operator is arithmetic
fn is_arithmetic_op(op: BinaryOp) -> bool {
    matches!(
        op,
        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod
    )
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Type error (Prakāra Doṣa)
///
/// Classified according to Garuda Purana error taxonomy where applicable.
#[derive(Debug, Clone)]
pub enum TypeError {
    /// Cannot infer type (Anumāna Asaphala)
    CannotInfer {
        pramanas_tried: Vec<Pramana>,
        evidence: Vec<String>,
    },

    /// Type mismatch (Prakāra Vaiṣamya)
    Mismatch {
        expected: ResolvedType,
        found: ResolvedType,
        span: Option<Span>,
        context: String,
    },

    /// Unknown type name (Ajñāta Prakāra)
    UnknownType { name: String, span: Option<Span> },

    /// Unknown identifier (Ajñāta Nāma)
    UnknownIdentifier { name: String, span: Option<Span> },

    /// Invalid operation for type (Avidhi Kriyā)
    InvalidOperation {
        op: String,
        ty: ResolvedType,
        span: Option<Span>,
    },

    /// Function arity mismatch (Mātrā Vaiṣamya)
    ArityMismatch {
        function: String,
        expected: usize,
        found: usize,
        span: Option<Span>,
    },

    /// Argument type mismatch (Yukti Vaiṣamya)
    ArgumentMismatch {
        function: String,
        param: String,
        expected: ResolvedType,
        found: ResolvedType,
        span: Option<Span>,
    },

    /// Binary operator type mismatch (Dvividha Vaiṣamya)
    BinaryOpMismatch {
        op: String,
        left_type: ResolvedType,
        right_type: ResolvedType,
        span: Option<Span>,
    },

    /// Branch type mismatch in if/match (Śākhā Vaiṣamya)
    BranchMismatch {
        then_type: ResolvedType,
        else_type: ResolvedType,
        span: Option<Span>,
    },

    /// Array element type mismatch (Śreṇī Vaiṣamya)
    ArrayElementMismatch {
        expected: ResolvedType,
        found: ResolvedType,
        span: Option<Span>,
    },

    /// Field type mismatch (Kṣetra Vaiṣamya)
    FieldTypeMismatch {
        struct_name: String,
        field: String,
        expected: ResolvedType,
        found: ResolvedType,
        span: Option<Span>,
    },

    /// Unification error (Ekīkaraṇa Doṣa)
    UnificationFailed {
        error: UnificationError,
        span: Option<Span>,
    },

    /// Recursive type error - infinite type (Ananta Prakāra)
    InfiniteType {
        var: TypeVar,
        ty: ResolvedType,
        span: Option<Span>,
    },
}

impl TypeError {
    /// Get the span of the error
    pub fn span(&self) -> Option<Span> {
        match self {
            TypeError::Mismatch { span, .. } => *span,
            TypeError::UnknownType { span, .. } => *span,
            TypeError::UnknownIdentifier { span, .. } => *span,
            TypeError::InvalidOperation { span, .. } => *span,
            TypeError::ArityMismatch { span, .. } => *span,
            TypeError::ArgumentMismatch { span, .. } => *span,
            TypeError::BinaryOpMismatch { span, .. } => *span,
            TypeError::BranchMismatch { span, .. } => *span,
            TypeError::ArrayElementMismatch { span, .. } => *span,
            TypeError::FieldTypeMismatch { span, .. } => *span,
            TypeError::UnificationFailed { span, .. } => *span,
            TypeError::InfiniteType { span, .. } => *span,
            TypeError::CannotInfer { .. } => None,
        }
    }

    /// Get a human-readable description
    pub fn description(&self) -> String {
        match self {
            TypeError::CannotInfer {
                pramanas_tried,
                evidence,
            } => {
                let pramanas: Vec<&str> =
                    pramanas_tried.iter().map(|p| p.sanskrit_name()).collect();
                format!(
                    "Cannot infer type. Tried pramāṇas: {}. Evidence: {}",
                    pramanas.join(", "),
                    evidence.join("; ")
                )
            }
            TypeError::Mismatch {
                expected,
                found,
                context,
                ..
            } => {
                format!(
                    "Type mismatch in {}: expected {:?}, found {:?}",
                    context, expected, found
                )
            }
            TypeError::UnknownType { name, .. } => {
                format!("Unknown type: '{}'", name)
            }
            TypeError::UnknownIdentifier { name, .. } => {
                format!("Unknown identifier: '{}'", name)
            }
            TypeError::InvalidOperation { op, ty, .. } => {
                format!("Invalid operation '{}' for type {:?}", op, ty)
            }
            TypeError::ArityMismatch {
                function,
                expected,
                found,
                ..
            } => {
                format!(
                    "Function '{}' expects {} arguments, found {}",
                    function, expected, found
                )
            }
            TypeError::ArgumentMismatch {
                function,
                param,
                expected,
                found,
                ..
            } => {
                format!(
                    "Function '{}' parameter '{}': expected {:?}, found {:?}",
                    function, param, expected, found
                )
            }
            TypeError::BinaryOpMismatch {
                op,
                left_type,
                right_type,
                ..
            } => {
                format!(
                    "Binary operator '{}' cannot be applied to {:?} and {:?}",
                    op, left_type, right_type
                )
            }
            TypeError::BranchMismatch {
                then_type,
                else_type,
                ..
            } => {
                format!(
                    "Branch types don't match: 'then' is {:?}, 'else' is {:?}",
                    then_type, else_type
                )
            }
            TypeError::ArrayElementMismatch {
                expected, found, ..
            } => {
                format!(
                    "Array element type mismatch: expected {:?}, found {:?}",
                    expected, found
                )
            }
            TypeError::FieldTypeMismatch {
                struct_name,
                field,
                expected,
                found,
                ..
            } => {
                format!(
                    "Field '{}' of struct '{}': expected {:?}, found {:?}",
                    field, struct_name, expected, found
                )
            }
            TypeError::UnificationFailed { error, .. } => {
                format!("Type unification failed: {:?}", error)
            }
            TypeError::InfiniteType { var, ty, .. } => {
                format!("Infinite type: {} occurs in {:?}", var, ty)
            }
        }
    }
}

// ============================================================================
// Display Implementations
// ============================================================================

impl std::fmt::Display for ResolvedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolvedType::Int8 => write!(f, "i8"),
            ResolvedType::Int16 => write!(f, "i16"),
            ResolvedType::Int32 => write!(f, "i32"),
            ResolvedType::Int64 => write!(f, "i64"),
            ResolvedType::UInt8 => write!(f, "u8"),
            ResolvedType::UInt16 => write!(f, "u16"),
            ResolvedType::UInt32 => write!(f, "u32"),
            ResolvedType::UInt64 => write!(f, "u64"),
            ResolvedType::Float32 => write!(f, "f32"),
            ResolvedType::Float64 => write!(f, "f64"),
            ResolvedType::Bool => write!(f, "bool"),
            ResolvedType::Unit => write!(f, "()"),
            ResolvedType::Char => write!(f, "char"),
            ResolvedType::String => write!(f, "String"),
            ResolvedType::Never => write!(f, "!"),
            ResolvedType::TypeVar(v) => write!(f, "{}", v),
            ResolvedType::Named { name, generics } => {
                write!(f, "{}", name)?;
                if !generics.is_empty() {
                    write!(f, "<")?;
                    for (i, g) in generics.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", g)?;
                    }
                    write!(f, ">")?;
                }
                Ok(())
            }
            ResolvedType::Function {
                params,
                return_type,
            } => {
                write!(f, "fn(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", return_type)
            }
            ResolvedType::Reference { inner, mutable, .. } => {
                if *mutable {
                    write!(f, "&mut {}", inner)
                } else {
                    write!(f, "&{}", inner)
                }
            }
            ResolvedType::Array { element, size } => {
                if let Some(sz) = size {
                    write!(f, "[{}; {}]", element, sz)
                } else {
                    write!(f, "[{}]", element)
                }
            }
            ResolvedType::Tuple(elems) => {
                write!(f, "(")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            ResolvedType::Unknown => write!(f, "_"),
            ResolvedType::Error => write!(f, "!error"),
        }
    }
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl std::error::Error for TypeError {}
