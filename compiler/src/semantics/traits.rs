//! Guṇa Trait System - Trait Definition and Implementation
//!
//! # Philosophy: Guṇa (गुण)
//!
//! In Sanskrit philosophy, "guṇa" means quality, attribute, or characteristic.
//! Just as guṇas define the essential qualities of prakṛti (matter), traits
//! define the essential behaviors that types must implement.
//!
//! ## The Three Aspects of Guṇa
//!
//! 1. **Sattva** (पurity) - The trait contract (method signatures)
//! 2. **Rajas** (Activity) - The implementation (impl blocks)
//! 3. **Tamas** (Inertia) - The constraints (bounds and coherence)
//!
//! ## Sanskrit Terminology
//!
//! - guṇa (गुण) = trait/quality
//! - kārya (कार्य) = method/action
//! - svabhāva (स्वभाव) = associated type ("own-nature")
//! - adhikāra (अधिकार) = bound/constraint ("authority/right")
//! - upādhi (उपाधि) = supertraits ("limiting adjunct")
//!
//! ## References
//!
//! - Wadler, Philip (1989): "Theorems for free!"
//! - Odersky & Wadler (1997): "Pizza into Java"
//! - Rust RFC 0195 (2014): "Associated Items"
//! - Peyton Jones et al. (1997): "Type Classes with Functional Dependencies"

use std::collections::HashMap;
use std::fmt;

use crate::lexer::Span;
use crate::parser::ast::{Block, GenericParam, Identifier, Type};

// ============================================================================
// PART 1: TRAIT DEFINITION (Guṇa Nirdhāraṇa - Quality Specification)
// ============================================================================

/// Unique identifier for traits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraitId(pub u32);

impl TraitId {
    pub fn new(id: u32) -> Self {
        TraitId(id)
    }
}

/// Trait definition (guṇa)
///
/// Represents a trait/interface contract that types can implement.
///
/// Sanskrit:
/// ```text
/// guṇa Tulanīya {
///     kārya tulana(svayam, anya: &Svayam) -> Krama;
/// }
/// ```
#[derive(Debug, Clone)]
pub struct TraitDef {
    /// Unique identifier
    pub id: TraitId,

    /// Trait name (Sanskrit: nāma)
    pub name: String,

    /// Sanskrit name for display
    pub sanskrit_name: Option<String>,

    /// Generic type parameters
    pub generics: Vec<GenericParam>,

    /// Supertraits (upādhi - limiting adjuncts)
    /// A type implementing this trait must also implement all supertraits
    pub supertraits: Vec<TraitBound>,

    /// Method signatures (kārya)
    pub methods: Vec<TraitMethod>,

    /// Associated types (svabhāva - own-nature)
    pub associated_types: Vec<AssociatedType>,

    /// Associated constants
    pub associated_consts: Vec<AssociatedConst>,

    /// Where clauses for complex bounds
    pub where_clauses: Vec<WhereClause>,

    /// Is this an auto-trait? (like Send, Sync)
    pub is_auto: bool,

    /// Is this a marker trait? (no methods)
    pub is_marker: bool,

    /// Is this an unsafe trait?
    pub is_unsafe: bool,

    /// Documentation
    pub doc: Option<String>,

    /// Source location
    pub span: Span,
}

impl TraitDef {
    /// Create a new trait definition
    pub fn new(name: impl Into<String>, id: TraitId, span: Span) -> Self {
        Self {
            id,
            name: name.into(),
            sanskrit_name: None,
            generics: Vec::new(),
            supertraits: Vec::new(),
            methods: Vec::new(),
            associated_types: Vec::new(),
            associated_consts: Vec::new(),
            where_clauses: Vec::new(),
            is_auto: false,
            is_marker: false,
            is_unsafe: false,
            doc: None,
            span,
        }
    }

    /// Add a Sanskrit name
    pub fn with_sanskrit_name(mut self, name: impl Into<String>) -> Self {
        self.sanskrit_name = Some(name.into());
        self
    }

    /// Add a method
    pub fn with_method(mut self, method: TraitMethod) -> Self {
        self.methods.push(method);
        self
    }

    /// Add an associated type
    pub fn with_associated_type(mut self, assoc: AssociatedType) -> Self {
        self.associated_types.push(assoc);
        self
    }

    /// Add a supertrait bound
    pub fn with_supertrait(mut self, bound: TraitBound) -> Self {
        self.supertraits.push(bound);
        self
    }

    /// Check if this is a marker trait (no methods, no associated items)
    pub fn is_marker_trait(&self) -> bool {
        self.methods.is_empty()
            && self.associated_types.is_empty()
            && self.associated_consts.is_empty()
    }

    /// Get all method names
    pub fn method_names(&self) -> Vec<&str> {
        self.methods.iter().map(|m| m.name.as_str()).collect()
    }

    /// Find a method by name
    pub fn find_method(&self, name: &str) -> Option<&TraitMethod> {
        self.methods.iter().find(|m| m.name == name)
    }

    /// Find an associated type by name
    pub fn find_associated_type(&self, name: &str) -> Option<&AssociatedType> {
        self.associated_types.iter().find(|a| a.name == name)
    }
}

/// Method in a trait
#[derive(Debug, Clone)]
pub struct TraitMethod {
    /// Method name (kārya-nāma)
    pub name: String,

    /// Sanskrit name
    pub sanskrit_name: Option<String>,

    /// Generic parameters on the method
    pub generics: Vec<GenericParam>,

    /// Receiver type (self, &self, &mut self)
    pub receiver: MethodReceiver,

    /// Input parameters (excluding receiver)
    pub params: Vec<MethodParam>,

    /// Return type
    pub return_type: Type,

    /// Where clauses
    pub where_clauses: Vec<WhereClause>,

    /// Default implementation (optional)
    pub default_impl: Option<Block>,

    /// Is this an async method?
    pub is_async: bool,

    /// Is this an unsafe method?
    pub is_unsafe: bool,

    /// Documentation
    pub doc: Option<String>,

    /// Source span
    pub span: Span,
}

impl TraitMethod {
    /// Create a new method
    pub fn new(name: impl Into<String>, return_type: Type, span: Span) -> Self {
        Self {
            name: name.into(),
            sanskrit_name: None,
            generics: Vec::new(),
            receiver: MethodReceiver::None,
            params: Vec::new(),
            return_type,
            where_clauses: Vec::new(),
            default_impl: None,
            is_async: false,
            is_unsafe: false,
            doc: None,
            span,
        }
    }

    /// Set receiver type
    pub fn with_receiver(mut self, receiver: MethodReceiver) -> Self {
        self.receiver = receiver;
        self
    }

    /// Add parameter
    pub fn with_param(mut self, param: MethodParam) -> Self {
        self.params.push(param);
        self
    }

    /// Set default implementation
    pub fn with_default(mut self, body: Block) -> Self {
        self.default_impl = Some(body);
        self
    }

    /// Check if this method has a default implementation
    pub fn has_default(&self) -> bool {
        self.default_impl.is_some()
    }
}

/// Method receiver type
#[derive(Debug, Clone, PartialEq)]
pub enum MethodReceiver {
    /// No self parameter (associated function)
    None,
    /// self (takes ownership)
    Value,
    /// &self (shared reference)
    Ref,
    /// &mut self (mutable reference)
    RefMut,
    /// self: Box<Self>
    Box,
    /// self: Rc<Self>
    Rc,
    /// self: Arc<Self>
    Arc,
    /// Custom receiver type
    Custom(Type),
}

impl MethodReceiver {
    /// Check if this is a method (has self) or an associated function
    pub fn is_method(&self) -> bool {
        !matches!(self, MethodReceiver::None)
    }
}

/// Method parameter
#[derive(Debug, Clone)]
pub struct MethodParam {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

/// Associated type in a trait (svabhāva)
#[derive(Debug, Clone)]
pub struct AssociatedType {
    /// Name of the associated type
    pub name: String,

    /// Sanskrit name
    pub sanskrit_name: Option<String>,

    /// Bounds on the associated type
    pub bounds: Vec<TraitBound>,

    /// Default type (if any)
    pub default: Option<Type>,

    /// Where clauses
    pub where_clauses: Vec<WhereClause>,

    /// Documentation
    pub doc: Option<String>,

    /// Source span
    pub span: Span,
}

impl AssociatedType {
    pub fn new(name: impl Into<String>, span: Span) -> Self {
        Self {
            name: name.into(),
            sanskrit_name: None,
            bounds: Vec::new(),
            default: None,
            where_clauses: Vec::new(),
            doc: None,
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
}

/// Associated constant
#[derive(Debug, Clone)]
pub struct AssociatedConst {
    pub name: String,
    pub ty: Type,
    pub default_value: Option<String>, // Expression as string for now
    pub span: Span,
}

// ============================================================================
// PART 2: TRAIT BOUNDS & WHERE CLAUSES (Adhikāra - Constraints)
// ============================================================================

/// Trait bound (adhikāra)
#[derive(Debug, Clone, PartialEq)]
pub struct TraitBound {
    /// The trait being bounded
    pub trait_ref: TraitRef,

    /// Is this a negative bound (!Trait)?
    pub is_negative: bool,

    /// Is this a ? (maybe) bound?
    pub is_maybe: bool,

    /// Higher-ranked type bounds (for<'a>)
    pub higher_ranked: Vec<String>,

    /// Source span
    pub span: Span,
}

impl TraitBound {
    pub fn new(trait_ref: TraitRef, span: Span) -> Self {
        Self {
            trait_ref,
            is_negative: false,
            is_maybe: false,
            higher_ranked: Vec::new(),
            span,
        }
    }

    pub fn negative(mut self) -> Self {
        self.is_negative = true;
        self
    }

    pub fn maybe(mut self) -> Self {
        self.is_maybe = true;
        self
    }
}

/// Reference to a trait (possibly with type arguments)
#[derive(Debug, Clone, PartialEq)]
pub struct TraitRef {
    /// Trait identifier
    pub trait_id: TraitId,

    /// Trait name for display
    pub name: String,

    /// Generic type arguments
    pub type_args: Vec<Type>,

    /// Associated type bindings (Output = i32)
    pub assoc_bindings: Vec<AssocBinding>,
}

impl TraitRef {
    pub fn new(trait_id: TraitId, name: impl Into<String>) -> Self {
        Self {
            trait_id,
            name: name.into(),
            type_args: Vec::new(),
            assoc_bindings: Vec::new(),
        }
    }

    pub fn with_type_arg(mut self, ty: Type) -> Self {
        self.type_args.push(ty);
        self
    }

    pub fn with_assoc_binding(mut self, binding: AssocBinding) -> Self {
        self.assoc_bindings.push(binding);
        self
    }
}

/// Associated type binding (e.g., Output = i32)
#[derive(Debug, Clone, PartialEq)]
pub struct AssocBinding {
    pub name: String,
    pub ty: Type,
}

/// Where clause for complex bounds
#[derive(Debug, Clone)]
pub struct WhereClause {
    /// The type being constrained
    pub ty: Type,

    /// The bounds applied
    pub bounds: Vec<TraitBound>,

    /// Source span
    pub span: Span,
}

// ============================================================================
// PART 3: TRAIT IMPLEMENTATION (Guṇa Upapādana - Quality Substantiation)
// ============================================================================

/// Unique identifier for implementations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImplId(pub u32);

/// Implementation of a trait for a type
#[derive(Debug, Clone)]
pub struct TraitImpl {
    /// Unique identifier
    pub id: ImplId,

    /// The trait being implemented (None for inherent impl)
    pub trait_ref: Option<TraitRef>,

    /// The type implementing the trait
    pub implementing_type: Type,

    /// Generic parameters on the impl
    pub generics: Vec<GenericParam>,

    /// Where clauses
    pub where_clauses: Vec<WhereClause>,

    /// Method implementations
    pub methods: Vec<ImplMethod>,

    /// Associated type values
    pub associated_types: Vec<ImplAssociatedType>,

    /// Associated constant values
    pub associated_consts: Vec<ImplAssociatedConst>,

    /// Is this an unsafe impl?
    pub is_unsafe: bool,

    /// Is this a negative impl (!Trait)?
    pub is_negative: bool,

    /// Source span
    pub span: Span,
}

impl TraitImpl {
    /// Create a new trait implementation
    pub fn new(trait_ref: TraitRef, implementing_type: Type, id: ImplId, span: Span) -> Self {
        Self {
            id,
            trait_ref: Some(trait_ref),
            implementing_type,
            generics: Vec::new(),
            where_clauses: Vec::new(),
            methods: Vec::new(),
            associated_types: Vec::new(),
            associated_consts: Vec::new(),
            is_unsafe: false,
            is_negative: false,
            span,
        }
    }

    /// Create an inherent impl (impl Type { ... })
    pub fn inherent(implementing_type: Type, id: ImplId, span: Span) -> Self {
        Self {
            id,
            trait_ref: None,
            implementing_type,
            generics: Vec::new(),
            where_clauses: Vec::new(),
            methods: Vec::new(),
            associated_types: Vec::new(),
            associated_consts: Vec::new(),
            is_unsafe: false,
            is_negative: false,
            span,
        }
    }

    /// Check if this is an inherent impl (not a trait impl)
    pub fn is_inherent(&self) -> bool {
        self.trait_ref.is_none()
    }

    /// Add a method implementation
    pub fn with_method(mut self, method: ImplMethod) -> Self {
        self.methods.push(method);
        self
    }

    /// Find an implemented method by name
    pub fn find_method(&self, name: &str) -> Option<&ImplMethod> {
        self.methods.iter().find(|m| m.name == name)
    }
}

/// Method implementation in an impl block
#[derive(Debug, Clone)]
pub struct ImplMethod {
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub receiver: MethodReceiver,
    pub params: Vec<MethodParam>,
    pub return_type: Type,
    pub body: Block,
    pub is_async: bool,
    pub is_unsafe: bool,
    pub span: Span,
}

/// Associated type value in impl
#[derive(Debug, Clone)]
pub struct ImplAssociatedType {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

/// Associated constant value in impl
#[derive(Debug, Clone)]
pub struct ImplAssociatedConst {
    pub name: String,
    pub ty: Type,
    pub value: String, // Expression as string for now
    pub span: Span,
}

// ============================================================================
// PART 4: TRAIT SOLVER (Nyāya-Vicāra - Logical Investigation)
// ============================================================================

/// Trait solver for method resolution and impl selection
///
/// The solver uses a combination of:
/// - Unification for generic matching
/// - Specificity ordering for overlapping impls
/// - Coherence checking for impl correctness
///
/// Based on Chalk (Rust's trait solver) and Haskell's type class resolution
pub struct TraitSolver {
    /// All known traits
    traits: HashMap<TraitId, TraitDef>,

    /// All implementations, indexed by trait
    impls_by_trait: HashMap<TraitId, Vec<TraitImpl>>,

    /// Inherent impls indexed by type name
    inherent_impls: HashMap<String, Vec<TraitImpl>>,

    /// Next trait ID
    next_trait_id: u32,

    /// Next impl ID
    next_impl_id: u32,

    /// Built-in trait IDs
    builtin_traits: BuiltinTraits,
}

/// Built-in trait identifiers
#[derive(Debug, Clone)]
pub struct BuiltinTraits {
    pub copy: Option<TraitId>,          // Copy (प्रतिलिपि)
    pub clone: Option<TraitId>,         // Clone (अनुकरण)
    pub drop: Option<TraitId>,          // Drop (त्याग)
    pub sized: Option<TraitId>,         // Sized (परिमित)
    pub send: Option<TraitId>,          // Send (प्रेषण)
    pub sync: Option<TraitId>,          // Sync (समक्रम)
    pub eq: Option<TraitId>,            // Eq (समान)
    pub partial_eq: Option<TraitId>,    // PartialEq (आंशिक-समान)
    pub ord: Option<TraitId>,           // Ord (क्रम)
    pub partial_ord: Option<TraitId>,   // PartialOrd (आंशिक-क्रम)
    pub hash: Option<TraitId>,          // Hash (संक्षेप)
    pub default: Option<TraitId>,       // Default (पूर्वनिर्धारित)
    pub debug: Option<TraitId>,         // Debug (विमर्श)
    pub display: Option<TraitId>,       // Display (प्रदर्शन)
    pub iterator: Option<TraitId>,      // Iterator (पुनरावर्तक)
    pub into_iterator: Option<TraitId>, // IntoIterator
    pub from: Option<TraitId>,          // From (प्राप्त)
    pub into: Option<TraitId>,          // Into (परिवर्त)
    pub fn_once: Option<TraitId>,       // FnOnce
    pub fn_mut: Option<TraitId>,        // FnMut
    pub fn_: Option<TraitId>,           // Fn
}

impl Default for BuiltinTraits {
    fn default() -> Self {
        Self {
            copy: None,
            clone: None,
            drop: None,
            sized: None,
            send: None,
            sync: None,
            eq: None,
            partial_eq: None,
            ord: None,
            partial_ord: None,
            hash: None,
            default: None,
            debug: None,
            display: None,
            iterator: None,
            into_iterator: None,
            from: None,
            into: None,
            fn_once: None,
            fn_mut: None,
            fn_: None,
        }
    }
}

impl TraitSolver {
    /// Create a new trait solver
    pub fn new() -> Self {
        let mut solver = Self {
            traits: HashMap::new(),
            impls_by_trait: HashMap::new(),
            inherent_impls: HashMap::new(),
            next_trait_id: 1,
            next_impl_id: 1,
            builtin_traits: BuiltinTraits::default(),
        };

        // Register built-in traits
        solver.register_builtin_traits();

        solver
    }

    /// Register all built-in traits
    fn register_builtin_traits(&mut self) {
        // Copy trait (प्रतिलिपि - pratilipi)
        let copy_id = self.register_trait(
            TraitDef::new("Copy", TraitId(0), Span::dummy()).with_sanskrit_name("प्रतिलिपि"),
        );
        self.builtin_traits.copy = Some(copy_id);

        // Clone trait (अनुकरण - anukaraṇa)
        let clone_span = Span::dummy();
        let clone_id = self.register_trait(
            TraitDef::new("Clone", TraitId(0), clone_span)
                .with_sanskrit_name("अनुकरण")
                .with_method(
                    TraitMethod::new(
                        "clone",
                        Type::Named {
                            name: Identifier {
                                name: "Self".to_string(),
                                affixes: Default::default(),
                                span: Span::dummy(),
                            },
                            generics: vec![],
                            affixes: Default::default(),
                        },
                        clone_span,
                    )
                    .with_receiver(MethodReceiver::Ref),
                ),
        );
        self.builtin_traits.clone = Some(clone_id);

        // Sized trait (परिमित - parimita)
        let sized_id = self.register_trait(
            TraitDef::new("Sized", TraitId(0), Span::dummy()).with_sanskrit_name("परिमित"),
        );
        self.builtin_traits.sized = Some(sized_id);

        // PartialEq trait (आंशिक-समान)
        let partial_eq_span = Span::dummy();
        let partial_eq_id = self.register_trait(
            TraitDef::new("PartialEq", TraitId(0), partial_eq_span)
                .with_sanskrit_name("आंशिक-समान")
                .with_method(
                    TraitMethod::new(
                        "eq",
                        Type::Named {
                            name: Identifier {
                                name: "satya".to_string(),
                                affixes: Default::default(),
                                span: Span::dummy(),
                            },
                            generics: vec![],
                            affixes: Default::default(),
                        },
                        partial_eq_span,
                    )
                    .with_receiver(MethodReceiver::Ref)
                    .with_param(MethodParam {
                        name: "other".to_string(),
                        ty: Type::Reference {
                            inner: Box::new(Type::Named {
                                name: Identifier {
                                    name: "Self".to_string(),
                                    affixes: Default::default(),
                                    span: Span::dummy(),
                                },
                                generics: vec![],
                                affixes: Default::default(),
                            }),
                            mutable: false,
                            lifetime: None,
                        },
                        span: partial_eq_span,
                    }),
                ),
        );
        self.builtin_traits.partial_eq = Some(partial_eq_id);

        // Eq trait (समान - samāna)
        let eq_id = self.register_trait(
            TraitDef::new("Eq", TraitId(0), Span::dummy())
                .with_sanskrit_name("समान")
                .with_supertrait(TraitBound::new(
                    TraitRef::new(partial_eq_id, "PartialEq"),
                    Span::dummy(),
                )),
        );
        self.builtin_traits.eq = Some(eq_id);

        // Default trait (पूर्वनिर्धारित)
        let default_span = Span::dummy();
        let default_id = self.register_trait(
            TraitDef::new("Default", TraitId(0), default_span)
                .with_sanskrit_name("पूर्वनिर्धारित")
                .with_method(
                    TraitMethod::new(
                        "default",
                        Type::Named {
                            name: Identifier {
                                name: "Self".to_string(),
                                affixes: Default::default(),
                                span: Span::dummy(),
                            },
                            generics: vec![],
                            affixes: Default::default(),
                        },
                        default_span,
                    )
                    .with_receiver(MethodReceiver::None),
                ),
        );
        self.builtin_traits.default = Some(default_id);

        // Iterator trait (पुनरावर्तक)
        let iterator_span = Span::dummy();
        let iterator_id = self.register_trait({
            let mut tr =
                TraitDef::new("Iterator", TraitId(0), iterator_span).with_sanskrit_name("पुनरावर्तक");
            tr.associated_types
                .push(AssociatedType::new("Item", iterator_span));
            tr.methods.push(
                TraitMethod::new(
                    "next",
                    Type::Named {
                        name: Identifier {
                            name: "Option".to_string(),
                            affixes: Default::default(),
                            span: Span::dummy(),
                        },
                        generics: vec![Type::Named {
                            name: Identifier {
                                name: "Self::Item".to_string(),
                                affixes: Default::default(),
                                span: Span::dummy(),
                            },
                            generics: vec![],
                            affixes: Default::default(),
                        }],
                        affixes: Default::default(),
                    },
                    iterator_span,
                )
                .with_receiver(MethodReceiver::RefMut),
            );
            tr
        });
        self.builtin_traits.iterator = Some(iterator_id);
    }

    /// Register a new trait
    pub fn register_trait(&mut self, mut trait_def: TraitDef) -> TraitId {
        let id = TraitId(self.next_trait_id);
        self.next_trait_id += 1;

        trait_def.id = id;
        self.traits.insert(id, trait_def);
        self.impls_by_trait.insert(id, Vec::new());

        id
    }

    /// Register a trait implementation
    pub fn register_impl(&mut self, mut impl_def: TraitImpl) -> Result<ImplId, TraitError> {
        let id = ImplId(self.next_impl_id);
        self.next_impl_id += 1;
        impl_def.id = id;

        // Check coherence rules
        self.check_coherence(&impl_def)?;

        if let Some(ref trait_ref) = impl_def.trait_ref {
            // Trait impl
            self.impls_by_trait
                .entry(trait_ref.trait_id)
                .or_default()
                .push(impl_def);
        } else {
            // Inherent impl
            let type_name = self.type_name(&impl_def.implementing_type);
            self.inherent_impls
                .entry(type_name)
                .or_default()
                .push(impl_def);
        }

        Ok(id)
    }

    /// Get a trait by ID
    pub fn get_trait(&self, id: TraitId) -> Option<&TraitDef> {
        self.traits.get(&id)
    }

    /// Find a trait by name
    pub fn find_trait(&self, name: &str) -> Option<&TraitDef> {
        self.traits.values().find(|t| t.name == name)
    }

    /// Get all implementations of a trait
    pub fn get_impls(&self, trait_id: TraitId) -> &[TraitImpl] {
        self.impls_by_trait
            .get(&trait_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Check if a type implements a trait
    pub fn implements(&self, ty: &Type, trait_id: TraitId) -> bool {
        self.find_impl(ty, trait_id).is_some()
    }

    /// Find an implementation for a type
    pub fn find_impl(&self, ty: &Type, trait_id: TraitId) -> Option<&TraitImpl> {
        let impls = self.impls_by_trait.get(&trait_id)?;

        // Find most specific matching impl
        impls
            .iter()
            .find(|imp| self.types_match(&imp.implementing_type, ty))
    }

    /// Resolve a method call
    pub fn resolve_method(&self, ty: &Type, method_name: &str) -> Option<ResolvedMethod> {
        // First, check inherent impls
        let type_name = self.type_name(ty);
        if let Some(impls) = self.inherent_impls.get(&type_name) {
            for imp in impls {
                if let Some(method) = imp.find_method(method_name) {
                    return Some(ResolvedMethod {
                        impl_id: imp.id,
                        trait_id: None,
                        method_name: method_name.to_string(),
                        receiver: method.receiver.clone(),
                        return_type: method.return_type.clone(),
                    });
                }
            }
        }

        // Then check trait impls
        for (trait_id, impls) in &self.impls_by_trait {
            for imp in impls {
                if self.types_match(&imp.implementing_type, ty) {
                    if let Some(method) = imp.find_method(method_name) {
                        return Some(ResolvedMethod {
                            impl_id: imp.id,
                            trait_id: Some(*trait_id),
                            method_name: method_name.to_string(),
                            receiver: method.receiver.clone(),
                            return_type: method.return_type.clone(),
                        });
                    }
                }
            }
        }

        None
    }

    /// Check coherence rules for an impl
    fn check_coherence(&self, impl_def: &TraitImpl) -> Result<(), TraitError> {
        if let Some(ref trait_ref) = impl_def.trait_ref {
            // Check if trait exists
            if !self.traits.contains_key(&trait_ref.trait_id) {
                return Err(TraitError::UnknownTrait {
                    name: trait_ref.name.clone(),
                    span: impl_def.span,
                });
            }

            // Check for overlapping impls (orphan rule simplified)
            let existing = self.impls_by_trait.get(&trait_ref.trait_id);
            if let Some(impls) = existing {
                for existing_impl in impls {
                    if self.types_overlap(
                        &impl_def.implementing_type,
                        &existing_impl.implementing_type,
                    ) {
                        return Err(TraitError::OverlappingImpl {
                            trait_name: trait_ref.name.clone(),
                            type_name: self.type_name(&impl_def.implementing_type),
                            existing_span: existing_impl.span,
                            new_span: impl_def.span,
                        });
                    }
                }
            }

            // Check that all required methods are implemented
            let trait_def = self.traits.get(&trait_ref.trait_id).unwrap();
            for method in &trait_def.methods {
                if method.default_impl.is_none() && impl_def.find_method(&method.name).is_none() {
                    return Err(TraitError::MissingMethod {
                        trait_name: trait_ref.name.clone(),
                        method_name: method.name.clone(),
                        span: impl_def.span,
                    });
                }
            }

            // Check that all associated types are provided
            for assoc in &trait_def.associated_types {
                if assoc.default.is_none() {
                    let provided = impl_def
                        .associated_types
                        .iter()
                        .any(|a| a.name == assoc.name);
                    if !provided {
                        return Err(TraitError::MissingAssociatedType {
                            trait_name: trait_ref.name.clone(),
                            type_name: assoc.name.clone(),
                            span: impl_def.span,
                        });
                    }
                }
            }
        }

        Ok(())
    }

    /// Check if two types match (simplified unification)
    fn types_match(&self, pattern: &Type, concrete: &Type) -> bool {
        match (pattern, concrete) {
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
                // Handle Self type
                if n1.name == "Self" || n2.name == "Self" {
                    return true;
                }
                n1.name == n2.name
                    && g1.len() == g2.len()
                    && g1
                        .iter()
                        .zip(g2.iter())
                        .all(|(a, b)| self.types_match(a, b))
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
            ) => m1 == m2 && self.types_match(i1, i2),
            (
                Type::Array {
                    element: e1,
                    size: s1,
                },
                Type::Array {
                    element: e2,
                    size: s2,
                },
            ) => s1 == s2 && self.types_match(e1, e2),
            (Type::Tuple(t1), Type::Tuple(t2)) => {
                t1.len() == t2.len()
                    && t1
                        .iter()
                        .zip(t2.iter())
                        .all(|(a, b)| self.types_match(a, b))
            }
            (Type::Inferred, _) | (_, Type::Inferred) => true,
            _ => false,
        }
    }

    /// Check if two type patterns could overlap
    fn types_overlap(&self, t1: &Type, t2: &Type) -> bool {
        // Simplified: check for exact match for now
        self.types_match(t1, t2)
    }

    /// Get a string representation of a type
    fn type_name(&self, ty: &Type) -> String {
        match ty {
            Type::Named { name, generics, .. } => {
                if generics.is_empty() {
                    name.name.clone()
                } else {
                    let args: Vec<_> = generics.iter().map(|g| self.type_name(g)).collect();
                    format!("{}<{}>", name.name, args.join(", "))
                }
            }
            Type::Reference { inner, mutable, .. } => {
                if *mutable {
                    format!("&mut {}", self.type_name(inner))
                } else {
                    format!("&{}", self.type_name(inner))
                }
            }
            Type::Array { element, size } => {
                if let Some(s) = size {
                    format!("[{}; {}]", self.type_name(element), s)
                } else {
                    format!("[{}]", self.type_name(element))
                }
            }
            Type::Tuple(types) => {
                let args: Vec<_> = types.iter().map(|t| self.type_name(t)).collect();
                format!("({})", args.join(", "))
            }
            Type::Function {
                params,
                return_type,
            } => {
                let args: Vec<_> = params.iter().map(|p| self.type_name(p)).collect();
                format!("fn({}) -> {}", args.join(", "), self.type_name(return_type))
            }
            Type::Inferred => "_".to_string(),
        }
    }

    /// Get all traits a type implements
    pub fn implemented_traits(&self, ty: &Type) -> Vec<TraitId> {
        let mut result = Vec::new();
        for (trait_id, impls) in &self.impls_by_trait {
            if impls
                .iter()
                .any(|imp| self.types_match(&imp.implementing_type, ty))
            {
                result.push(*trait_id);
            }
        }
        result
    }

    /// Check trait bounds satisfaction
    pub fn check_bounds(&self, ty: &Type, bounds: &[TraitBound]) -> Result<(), TraitError> {
        for bound in bounds {
            if bound.is_maybe {
                continue; // ?Sized etc. are optional
            }

            if bound.is_negative {
                // Negative bounds: type must NOT implement trait
                if self.implements(ty, bound.trait_ref.trait_id) {
                    return Err(TraitError::NegativeBoundViolation {
                        type_name: self.type_name(ty),
                        trait_name: bound.trait_ref.name.clone(),
                        span: bound.span,
                    });
                }
            } else {
                // Positive bounds: type must implement trait
                if !self.implements(ty, bound.trait_ref.trait_id) {
                    return Err(TraitError::BoundNotSatisfied {
                        type_name: self.type_name(ty),
                        trait_name: bound.trait_ref.name.clone(),
                        span: bound.span,
                    });
                }
            }
        }
        Ok(())
    }
}

impl Default for TraitSolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of method resolution
#[derive(Debug, Clone)]
pub struct ResolvedMethod {
    pub impl_id: ImplId,
    pub trait_id: Option<TraitId>,
    pub method_name: String,
    pub receiver: MethodReceiver,
    pub return_type: Type,
}

// ============================================================================
// PART 5: TRAIT ERRORS (Doṣa - Faults)
// ============================================================================

/// Errors related to traits
#[derive(Debug, Clone)]
pub enum TraitError {
    /// Unknown trait referenced
    UnknownTrait { name: String, span: Span },

    /// Overlapping implementations
    OverlappingImpl {
        trait_name: String,
        type_name: String,
        existing_span: Span,
        new_span: Span,
    },

    /// Missing required method
    MissingMethod {
        trait_name: String,
        method_name: String,
        span: Span,
    },

    /// Missing associated type
    MissingAssociatedType {
        trait_name: String,
        type_name: String,
        span: Span,
    },

    /// Trait bound not satisfied
    BoundNotSatisfied {
        type_name: String,
        trait_name: String,
        span: Span,
    },

    /// Negative bound violated
    NegativeBoundViolation {
        type_name: String,
        trait_name: String,
        span: Span,
    },

    /// Method not found
    MethodNotFound {
        type_name: String,
        method_name: String,
        span: Span,
    },

    /// Wrong number of type arguments
    WrongTypeArgCount {
        trait_name: String,
        expected: usize,
        found: usize,
        span: Span,
    },

    /// Cyclic trait dependency
    CyclicTrait {
        trait_name: String,
        cycle: Vec<String>,
        span: Span,
    },
}

impl fmt::Display for TraitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TraitError::UnknownTrait { name, .. } => {
                write!(f, "अज्ञात गुण (Unknown trait): `{}`", name)
            }
            TraitError::OverlappingImpl {
                trait_name,
                type_name,
                ..
            } => {
                write!(
                    f,
                    "व्याप्त कार्यान्वयन (Overlapping impl): `{}` for `{}`",
                    trait_name, type_name
                )
            }
            TraitError::MissingMethod {
                trait_name,
                method_name,
                ..
            } => {
                write!(
                    f,
                    "अनुपस्थित कार्य (Missing method): `{}` in impl of `{}`",
                    method_name, trait_name
                )
            }
            TraitError::MissingAssociatedType {
                trait_name,
                type_name,
                ..
            } => {
                write!(
                    f,
                    "अनुपस्थित स्वभाव (Missing associated type): `{}` in impl of `{}`",
                    type_name, trait_name
                )
            }
            TraitError::BoundNotSatisfied {
                type_name,
                trait_name,
                ..
            } => {
                write!(
                    f,
                    "असंतुष्ट अधिकार (Unsatisfied bound): `{}` does not implement `{}`",
                    type_name, trait_name
                )
            }
            TraitError::NegativeBoundViolation {
                type_name,
                trait_name,
                ..
            } => {
                write!(
                    f,
                    "निषेधात्मक अधिकार उल्लंघन (Negative bound violation): `{}` implements `{}`",
                    type_name, trait_name
                )
            }
            TraitError::MethodNotFound {
                type_name,
                method_name,
                ..
            } => {
                write!(
                    f,
                    "कार्य अप्राप्य (Method not found): `{}` on type `{}`",
                    method_name, type_name
                )
            }
            TraitError::WrongTypeArgCount {
                trait_name,
                expected,
                found,
                ..
            } => {
                write!(
                    f,
                    "गलत प्रकार तर्क संख्या (Wrong type arg count): `{}` expects {} args, found {}",
                    trait_name, expected, found
                )
            }
            TraitError::CyclicTrait {
                trait_name, cycle, ..
            } => {
                write!(
                    f,
                    "चक्रीय गुण निर्भरता (Cyclic trait): `{}` -> {}",
                    trait_name,
                    cycle.join(" -> ")
                )
            }
        }
    }
}

impl std::error::Error for TraitError {}

// ============================================================================
// PART 6: TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_simple_type(name: &str) -> Type {
        Type::Named {
            name: Identifier {
                name: name.to_string(),
                affixes: Default::default(),
                span: Span::dummy(),
            },
            generics: vec![],
            affixes: Default::default(),
        }
    }

    #[test]
    fn test_trait_creation() {
        let solver = TraitSolver::new();

        // Check built-in traits exist
        assert!(solver.builtin_traits.copy.is_some());
        assert!(solver.builtin_traits.clone.is_some());
        assert!(solver.builtin_traits.partial_eq.is_some());
        assert!(solver.builtin_traits.iterator.is_some());
    }

    #[test]
    fn test_trait_registration() {
        let mut solver = TraitSolver::new();

        // Create a custom trait
        let trait_def = TraitDef::new("Hashable", TraitId(0), Span::dummy())
            .with_sanskrit_name("संक्षेपनीय")
            .with_method(
                TraitMethod::new("hash", make_simple_type("u64"), Span::dummy())
                    .with_receiver(MethodReceiver::Ref),
            );

        let id = solver.register_trait(trait_def);

        let retrieved = solver.get_trait(id).unwrap();
        assert_eq!(retrieved.name, "Hashable");
        assert_eq!(retrieved.sanskrit_name, Some("संक्षेपनीय".to_string()));
        assert_eq!(retrieved.methods.len(), 1);
    }

    #[test]
    fn test_impl_registration() {
        let mut solver = TraitSolver::new();

        // Create a trait
        let trait_def = TraitDef::new("Show", TraitId(0), Span::dummy());
        let trait_id = solver.register_trait(trait_def);

        // Create an impl
        let impl_def = TraitImpl::new(
            TraitRef::new(trait_id, "Show"),
            make_simple_type("saṅkhyā"),
            ImplId(0),
            Span::dummy(),
        );

        let result = solver.register_impl(impl_def);
        assert!(result.is_ok());

        // Check it was registered
        assert!(solver.implements(&make_simple_type("saṅkhyā"), trait_id));
    }

    #[test]
    fn test_method_resolution() {
        let mut solver = TraitSolver::new();

        // Create an inherent impl
        let mut impl_def = TraitImpl::inherent(make_simple_type("Point"), ImplId(0), Span::dummy());

        impl_def.methods.push(ImplMethod {
            name: "distance".to_string(),
            generics: vec![],
            receiver: MethodReceiver::Ref,
            params: vec![],
            return_type: make_simple_type("f64"),
            body: Block {
                stmts: vec![],
                span: Span::dummy(),
            },
            is_async: false,
            is_unsafe: false,
            span: Span::dummy(),
        });

        solver.register_impl(impl_def).unwrap();

        // Resolve the method
        let resolved = solver.resolve_method(&make_simple_type("Point"), "distance");
        assert!(resolved.is_some());

        let method = resolved.unwrap();
        assert_eq!(method.method_name, "distance");
        assert!(method.trait_id.is_none()); // Inherent method
    }

    #[test]
    fn test_missing_method_error() {
        let mut solver = TraitSolver::new();

        // Create a trait with a required method
        let trait_def = TraitDef::new("Required", TraitId(0), Span::dummy()).with_method(
            TraitMethod::new("must_have", make_simple_type("unit"), Span::dummy())
                .with_receiver(MethodReceiver::Ref),
        );

        let trait_id = solver.register_trait(trait_def);

        // Try to impl without the method
        let impl_def = TraitImpl::new(
            TraitRef::new(trait_id, "Required"),
            make_simple_type("Foo"),
            ImplId(0),
            Span::dummy(),
        );

        let result = solver.register_impl(impl_def);
        assert!(matches!(result, Err(TraitError::MissingMethod { .. })));
    }

    #[test]
    fn test_overlapping_impl_error() {
        let mut solver = TraitSolver::new();

        // Create a marker trait
        let trait_def = TraitDef::new("Marker", TraitId(0), Span::dummy());
        let trait_id = solver.register_trait(trait_def);

        // First impl
        let impl1 = TraitImpl::new(
            TraitRef::new(trait_id, "Marker"),
            make_simple_type("Foo"),
            ImplId(0),
            Span::dummy(),
        );
        solver.register_impl(impl1).unwrap();

        // Overlapping impl
        let impl2 = TraitImpl::new(
            TraitRef::new(trait_id, "Marker"),
            make_simple_type("Foo"),
            ImplId(0),
            Span::dummy(),
        );

        let result = solver.register_impl(impl2);
        assert!(matches!(result, Err(TraitError::OverlappingImpl { .. })));
    }

    #[test]
    fn test_bounds_check() {
        let mut solver = TraitSolver::new();

        // Register a trait
        let trait_def = TraitDef::new("Bounded", TraitId(0), Span::dummy());
        let trait_id = solver.register_trait(trait_def);

        // Implement for one type
        let impl_def = TraitImpl::new(
            TraitRef::new(trait_id, "Bounded"),
            make_simple_type("BoundedType"),
            ImplId(0),
            Span::dummy(),
        );
        solver.register_impl(impl_def).unwrap();

        // Check bounds
        let bound = TraitBound::new(TraitRef::new(trait_id, "Bounded"), Span::dummy());

        // Should pass for implemented type
        assert!(solver
            .check_bounds(&make_simple_type("BoundedType"), &[bound.clone()])
            .is_ok());

        // Should fail for non-implemented type
        assert!(solver
            .check_bounds(&make_simple_type("Other"), &[bound])
            .is_err());
    }

    #[test]
    fn test_supertrait() {
        let mut solver = TraitSolver::new();

        // Clone requires Copy-like semantics
        let copy_id = solver.builtin_traits.copy.unwrap();
        let clone_id = solver.builtin_traits.clone.unwrap();

        // Check Clone has methods
        let clone_trait = solver.get_trait(clone_id).unwrap();
        assert!(!clone_trait.methods.is_empty());

        // Check PartialEq is a supertrait of Eq
        let eq_id = solver.builtin_traits.eq.unwrap();
        let eq_trait = solver.get_trait(eq_id).unwrap();
        assert!(!eq_trait.supertraits.is_empty());
    }

    #[test]
    fn test_associated_type() {
        let solver = TraitSolver::new();

        // Iterator has associated type Item
        let iter_id = solver.builtin_traits.iterator.unwrap();
        let iter_trait = solver.get_trait(iter_id).unwrap();

        assert_eq!(iter_trait.associated_types.len(), 1);
        assert_eq!(iter_trait.associated_types[0].name, "Item");
    }
}
