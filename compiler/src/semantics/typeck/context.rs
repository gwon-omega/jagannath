//! Type Checking Context (Parīkṣā Saṃdarbha)
//!
//! Manages the type checking context including:
//! - Symbol tables with lexical scoping (Viṣaya Tālikā)
//! - Type definitions registry (Prakāra Paricaya)
//! - Function signatures (Kārya Pariccheda)
//!
//! ## Nyāya Epistemological Foundation
//!
//! The context tracks not just types but their epistemological status:
//! - How certain are we about each type binding?
//! - Which pramāṇa (means of knowledge) established it?
//! - What span in source code defined it?
//!
//! ## Vaiśeṣika Padārtha Mapping
//!
//! Context elements map to Vaiśeṣika categories:
//! - Dravya (Substance): Variable bindings
//! - Guṇa (Quality): Type annotations
//! - Karma (Action): Function signatures
//! - Sāmānya (Universal): Type definitions
//! - Viśeṣa (Particular): Concrete instances
//! - Samavāya (Inherence): Scope nesting relationships

use super::pramana::Pramana;
use super::types::{FunctionSig, MethodSig, ResolvedType, TypeDefInfo, TypeInfo};
use crate::lexer::Span;
use std::collections::HashMap;

// ============================================================================
// Scope System (Viṣaya Vyavasthā)
// ============================================================================

/// A single scope level in the lexical scoping hierarchy
///
/// Named after Vaiśeṣika Dravya (substance) as it holds
/// the substantial bindings at each level.
#[derive(Debug, Clone)]
pub struct Scope {
    /// Symbol bindings: name -> type information
    symbols: HashMap<String, TypeInfo>,
    /// Scope kind for better error messages
    kind: ScopeKind,
    /// Depth level (0 = global)
    depth: usize,
}

/// Kind of scope for context-aware error messages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    /// Global (module) scope - Viśva Viṣaya
    Global,
    /// Function scope - Kārya Viṣaya
    Function,
    /// Block scope (if, loop, etc.) - Khaṇḍa Viṣaya
    Block,
    /// Match arm scope - Sādṛśya Viṣaya
    MatchArm,
    /// Loop scope (for, while) - Āvṛtti Viṣaya
    Loop,
    /// Impl block scope - Kartrī Viṣaya
    Impl,
}

impl ScopeKind {
    /// Sanskrit name for error messages
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            ScopeKind::Global => "विश्व-विषय",    // Universal scope
            ScopeKind::Function => "कार्य-विषय",  // Function scope
            ScopeKind::Block => "खण्ड-विषय",      // Block scope
            ScopeKind::MatchArm => "सादृश्य-विषय", // Match arm scope
            ScopeKind::Loop => "आवृत्ति-विषय",     // Loop scope
            ScopeKind::Impl => "कर्तृ-विषय",       // Implementation scope
        }
    }

    /// English description
    pub fn description(&self) -> &'static str {
        match self {
            ScopeKind::Global => "global scope",
            ScopeKind::Function => "function scope",
            ScopeKind::Block => "block scope",
            ScopeKind::MatchArm => "match arm scope",
            ScopeKind::Loop => "loop scope",
            ScopeKind::Impl => "impl block scope",
        }
    }
}

impl Scope {
    /// Create a new scope
    pub fn new(kind: ScopeKind, depth: usize) -> Self {
        Self {
            symbols: HashMap::new(),
            kind,
            depth,
        }
    }

    /// Insert a symbol binding
    pub fn insert(&mut self, name: String, info: TypeInfo) {
        self.symbols.insert(name, info);
    }

    /// Look up a symbol in this scope only
    pub fn get(&self, name: &str) -> Option<&TypeInfo> {
        self.symbols.get(name)
    }

    /// Check if symbol exists in this scope
    pub fn contains(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    /// Get scope kind
    pub fn kind(&self) -> ScopeKind {
        self.kind
    }

    /// Get scope depth
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Iterate over all symbols
    pub fn symbols(&self) -> impl Iterator<Item = (&String, &TypeInfo)> {
        self.symbols.iter()
    }
}

// ============================================================================
// Type Context (Prakāra Saṃdarbha)
// ============================================================================

/// Main type checking context
///
/// Tracks all type-related information during compilation:
/// - Lexical scopes with symbol bindings
/// - Type definitions (structs, enums, aliases)
/// - Function and method signatures
///
/// ## Philosophy: Saṃdarbha as Pramāṇa Repository
///
/// The context serves as a repository of established knowledge,
/// combining evidence from multiple pramāṇas (means of knowledge):
/// - Pratyakṣa: Explicit type annotations in source
/// - Śabda: Function signatures as authoritative testimony
/// - Anumāna: Type definitions as logical premises
#[derive(Debug)]
pub struct TypeContext {
    /// Scope stack (innermost is last)
    scopes: Vec<Scope>,

    /// Type definitions: name -> definition
    /// These are Sāmānya (universals) in Vaiśeṣika terms
    type_defs: HashMap<String, TypeDefInfo>,

    /// Function signatures: name -> signature
    /// These represent Śabda (authoritative testimony)
    function_sigs: HashMap<String, FunctionSig>,

    /// Method signatures: type_name -> (method_name -> signature)
    /// Organized by implementing type for efficient lookup
    method_sigs: HashMap<String, HashMap<String, MethodSig>>,

    /// Type schemes for let-polymorphism: name -> scheme
    /// These enable polymorphic variable reuse
    type_schemes: HashMap<String, super::generics::TypeScheme>,
}

impl TypeContext {
    /// Create a new type context with global scope
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new(ScopeKind::Global, 0)],
            type_defs: HashMap::new(),
            function_sigs: HashMap::new(),
            method_sigs: HashMap::new(),
            type_schemes: HashMap::new(),
        }
    }

    // ========================================================================
    // Scope Management (Viṣaya Prabandha)
    // ========================================================================

    /// Enter a new scope
    ///
    /// In Vaiśeṣika terms, this creates a new Dravya (substance)
    /// container for local bindings.
    pub fn enter_scope(&mut self, kind: ScopeKind) {
        let depth = self.scopes.len();
        self.scopes.push(Scope::new(kind, depth));
    }

    /// Exit the current scope
    ///
    /// Releases all bindings in the current scope level.
    /// Returns the exited scope for inspection if needed.
    pub fn exit_scope(&mut self) -> Option<Scope> {
        // Never pop the global scope
        if self.scopes.len() > 1 {
            self.scopes.pop()
        } else {
            None
        }
    }

    /// Get current scope depth
    pub fn scope_depth(&self) -> usize {
        self.scopes.len() - 1
    }

    /// Get current scope kind
    pub fn current_scope_kind(&self) -> ScopeKind {
        self.scopes
            .last()
            .map(|s| s.kind())
            .unwrap_or(ScopeKind::Global)
    }

    // ========================================================================
    // Symbol Management (Pratīka Prabandha)
    // ========================================================================

    /// Add a symbol to the current scope
    pub fn add_symbol(&mut self, name: String, info: TypeInfo) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, info);
        }
    }

    /// Add a symbol with explicit type and pramāṇa
    pub fn add_symbol_with_pramana(
        &mut self,
        name: String,
        ty: ResolvedType,
        pramana: Pramana,
        span: Option<Span>,
    ) {
        let info = TypeInfo {
            ty,
            certainty: pramana.certainty(),
            pramana,
            span,
        };
        self.add_symbol(name, info);
    }

    /// Look up a symbol by name, searching from innermost scope outward
    ///
    /// This implements lexical scoping: inner bindings shadow outer ones.
    pub fn lookup_symbol(&self, name: &str) -> Option<&TypeInfo> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }
        None
    }

    /// Look up a symbol and return both the info and scope depth
    pub fn lookup_symbol_with_depth(&self, name: &str) -> Option<(&TypeInfo, usize)> {
        for (depth, scope) in self.scopes.iter().enumerate().rev() {
            if let Some(info) = scope.get(name) {
                return Some((info, depth));
            }
        }
        None
    }

    /// Check if a symbol is defined in the current scope (not parent scopes)
    pub fn is_defined_in_current_scope(&self, name: &str) -> bool {
        self.scopes
            .last()
            .map(|s| s.contains(name))
            .unwrap_or(false)
    }

    /// Update a symbol's type (for inference refinement)
    pub fn update_symbol_type(&mut self, name: &str, ty: ResolvedType) {
        // Search from innermost to outermost
        for scope in self.scopes.iter_mut().rev() {
            if let Some(info) = scope.symbols.get_mut(name) {
                info.ty = ty;
                return;
            }
        }
    }

    // ========================================================================
    // Type Definition Registry (Prakāra Paricaya)
    // ========================================================================

    /// Register a type definition
    pub fn register_type_def(&mut self, name: String, def: TypeDefInfo) {
        self.type_defs.insert(name, def);
    }

    /// Look up a type definition
    pub fn lookup_type_def(&self, name: &str) -> Option<&TypeDefInfo> {
        self.type_defs.get(name)
    }

    /// Check if a type is defined
    pub fn is_type_defined(&self, name: &str) -> bool {
        self.type_defs.contains_key(name)
    }

    /// Get all type definitions
    pub fn type_definitions(&self) -> impl Iterator<Item = (&String, &TypeDefInfo)> {
        self.type_defs.iter()
    }

    // ========================================================================
    // Function Signature Registry (Kārya Pariccheda)
    // ========================================================================

    /// Register a function signature
    pub fn register_function_sig(&mut self, name: String, sig: FunctionSig) {
        self.function_sigs.insert(name, sig);
    }

    /// Look up a function signature
    pub fn lookup_function_sig(&self, name: &str) -> Option<&FunctionSig> {
        self.function_sigs.get(name)
    }

    /// Check if a function is defined
    pub fn is_function_defined(&self, name: &str) -> bool {
        self.function_sigs.contains_key(name)
    }

    /// Get all function signatures
    pub fn function_signatures(&self) -> impl Iterator<Item = (&String, &FunctionSig)> {
        self.function_sigs.iter()
    }

    /// Alias for register_function_sig (convenience method)
    pub fn register_function(&mut self, sig: FunctionSig) {
        let name = sig.name.clone();
        self.register_function_sig(name, sig);
    }

    /// Alias for lookup_function_sig (convenience method)
    pub fn lookup_function(&self, name: &str) -> Option<&FunctionSig> {
        self.lookup_function_sig(name)
    }

    // ========================================================================
    // Method Signature Registry (Vidhayaḥ Pariccheda)
    // ========================================================================

    /// Register a method signature for a type
    pub fn register_method_sig(&mut self, type_name: String, method_name: String, sig: MethodSig) {
        self.method_sigs
            .entry(type_name)
            .or_insert_with(HashMap::new)
            .insert(method_name, sig);
    }

    /// Look up a method signature
    pub fn lookup_method_sig(&self, type_name: &str, method_name: &str) -> Option<&MethodSig> {
        self.method_sigs
            .get(type_name)
            .and_then(|methods| methods.get(method_name))
    }

    /// Get all methods for a type
    pub fn methods_for_type(&self, type_name: &str) -> Option<&HashMap<String, MethodSig>> {
        self.method_sigs.get(type_name)
    }

    /// Alias for register_method_sig (convenience method)
    pub fn register_method(&mut self, type_name: String, sig: MethodSig) {
        let method_name = sig.name.clone();
        self.register_method_sig(type_name, method_name, sig);
    }

    // ========================================================================
    // Type Scheme Registry (Bahurupatā Paricaya)
    // ========================================================================

    /// Register a type scheme for let-polymorphism
    ///
    /// Type schemes enable polymorphic reuse of let-bound variables:
    /// ```text
    /// let id = λx. x in (id 5, id "hello")
    /// ```
    pub fn register_type_scheme(&mut self, name: String, scheme: super::generics::TypeScheme) {
        self.type_schemes.insert(name, scheme);
    }

    /// Look up a type scheme
    pub fn lookup_type_scheme(&self, name: &str) -> Option<&super::generics::TypeScheme> {
        self.type_schemes.get(name)
    }

    /// Remove a type scheme (when variable goes out of scope)
    pub fn remove_type_scheme(&mut self, name: &str) {
        self.type_schemes.remove(name);
    }

    // ========================================================================
    // Scope Accessors (Viṣaya Prāptika)
    // ========================================================================

    /// Get iterator over all scopes (for environment analysis)
    pub fn scopes(&self) -> impl Iterator<Item = &Scope> {
        self.scopes.iter()
    }

    // ========================================================================
    // Context Utilities (Saṃdarbha Upayogitā)
    // ========================================================================

    /// Create a snapshot of current context state
    ///
    /// Useful for speculative type checking (e.g., in generics)
    pub fn snapshot(&self) -> ContextSnapshot {
        ContextSnapshot {
            scope_depth: self.scopes.len(),
        }
    }

    /// Restore context to a snapshot (by popping scopes)
    pub fn restore(&mut self, snapshot: ContextSnapshot) {
        while self.scopes.len() > snapshot.scope_depth {
            self.scopes.pop();
        }
    }

    /// Get the number of defined symbols across all scopes
    pub fn symbol_count(&self) -> usize {
        self.scopes.iter().map(|s| s.symbols.len()).sum()
    }

    /// Clear all non-global state (for reuse)
    pub fn clear(&mut self) {
        // Keep only global scope, clear its contents
        self.scopes.truncate(1);
        if let Some(global) = self.scopes.first_mut() {
            global.symbols.clear();
        }
        self.type_defs.clear();
        self.function_sigs.clear();
        self.method_sigs.clear();
        self.type_schemes.clear();
    }
}

impl Default for TypeContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot of context state for rollback
#[derive(Debug, Clone, Copy)]
pub struct ContextSnapshot {
    scope_depth: usize,
}

// ============================================================================
// Builtins Registration (Nirmita Paricaya)
// ============================================================================

/// Built-in functions and types
///
/// Registers the standard library primitives that are always available.
/// These represent Āpta Vacana (authoritative testimony) - trustworthy
/// knowledge from the language designers.
pub fn register_builtins(ctx: &mut TypeContext) {
    // mudrā (print) function
    ctx.register_function_sig(
        "mudrā".to_string(),
        FunctionSig {
            name: "mudrā".to_string(),
            params: vec![("value".to_string(), ResolvedType::String)],
            return_type: ResolvedType::Unit,
            span: None,
        },
    );

    // nirgama (exit) function
    ctx.register_function_sig(
        "nirgama".to_string(),
        FunctionSig {
            name: "nirgama".to_string(),
            params: vec![("code".to_string(), ResolvedType::Int32)],
            return_type: ResolvedType::Never,
            span: None,
        },
    );

    // Common string methods
    let string_methods: Vec<(&str, Vec<(&str, ResolvedType)>, ResolvedType)> = vec![
        ("len", vec![], ResolvedType::UInt64),
        ("is_empty", vec![], ResolvedType::Bool),
    ];

    for (name, params, ret) in string_methods {
        ctx.register_method_sig(
            "String".to_string(),
            name.to_string(),
            MethodSig {
                name: name.to_string(),
                self_type: super::types::SelfType::Ref,
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

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_creation() {
        let scope = Scope::new(ScopeKind::Function, 1);
        assert_eq!(scope.kind(), ScopeKind::Function);
        assert_eq!(scope.depth(), 1);
    }

    #[test]
    fn test_scope_insert_lookup() {
        let mut scope = Scope::new(ScopeKind::Block, 0);
        let info = TypeInfo {
            ty: ResolvedType::Int32,
            certainty: 1.0,
            pramana: Pramana::Pratyaksha,
            span: None,
        };
        scope.insert("x".to_string(), info);

        assert!(scope.contains("x"));
        assert!(!scope.contains("y"));
        assert_eq!(scope.get("x").unwrap().ty, ResolvedType::Int32);
    }

    #[test]
    fn test_context_scoping() {
        let mut ctx = TypeContext::new();
        assert_eq!(ctx.scope_depth(), 0);

        ctx.enter_scope(ScopeKind::Function);
        assert_eq!(ctx.scope_depth(), 1);

        ctx.enter_scope(ScopeKind::Block);
        assert_eq!(ctx.scope_depth(), 2);

        ctx.exit_scope();
        assert_eq!(ctx.scope_depth(), 1);

        ctx.exit_scope();
        assert_eq!(ctx.scope_depth(), 0);

        // Cannot exit global scope
        ctx.exit_scope();
        assert_eq!(ctx.scope_depth(), 0);
    }

    #[test]
    fn test_symbol_shadowing() {
        let mut ctx = TypeContext::new();

        // Add x = i32 in global scope
        ctx.add_symbol_with_pramana(
            "x".to_string(),
            ResolvedType::Int32,
            Pramana::Pratyaksha,
            None,
        );

        ctx.enter_scope(ScopeKind::Function);

        // x should still be visible
        assert_eq!(ctx.lookup_symbol("x").unwrap().ty, ResolvedType::Int32);

        // Add x = String in function scope (shadows)
        ctx.add_symbol_with_pramana(
            "x".to_string(),
            ResolvedType::String,
            Pramana::Pratyaksha,
            None,
        );

        // Now x should be String
        assert_eq!(ctx.lookup_symbol("x").unwrap().ty, ResolvedType::String);

        ctx.exit_scope();

        // Back to global, x should be i32 again
        assert_eq!(ctx.lookup_symbol("x").unwrap().ty, ResolvedType::Int32);
    }

    #[test]
    fn test_lookup_with_depth() {
        let mut ctx = TypeContext::new();

        ctx.add_symbol_with_pramana(
            "global".to_string(),
            ResolvedType::Int32,
            Pramana::Pratyaksha,
            None,
        );

        ctx.enter_scope(ScopeKind::Function);
        ctx.add_symbol_with_pramana(
            "local".to_string(),
            ResolvedType::Bool,
            Pramana::Anumana,
            None,
        );

        let (_, depth) = ctx.lookup_symbol_with_depth("global").unwrap();
        assert_eq!(depth, 0);

        let (_, depth) = ctx.lookup_symbol_with_depth("local").unwrap();
        assert_eq!(depth, 1);
    }

    #[test]
    fn test_type_def_registration() {
        let mut ctx = TypeContext::new();

        let def = TypeDefInfo {
            name: "Point".to_string(),
            generics: vec![],
            body: super::super::types::TypeBodyResolved::Struct(vec![
                ("x".to_string(), ResolvedType::Float64),
                ("y".to_string(), ResolvedType::Float64),
            ]),
        };

        ctx.register_type_def("Point".to_string(), def);

        assert!(ctx.is_type_defined("Point"));
        assert!(!ctx.is_type_defined("Circle"));

        let lookup = ctx.lookup_type_def("Point").unwrap();
        assert_eq!(lookup.name, "Point");
    }

    #[test]
    fn test_function_sig_registration() {
        let mut ctx = TypeContext::new();

        let sig = FunctionSig {
            name: "add".to_string(),
            params: vec![
                ("a".to_string(), ResolvedType::Int32),
                ("b".to_string(), ResolvedType::Int32),
            ],
            return_type: ResolvedType::Int32,
            span: None,
        };

        ctx.register_function_sig("add".to_string(), sig);

        assert!(ctx.is_function_defined("add"));

        let lookup = ctx.lookup_function_sig("add").unwrap();
        assert_eq!(lookup.params.len(), 2);
        assert_eq!(lookup.return_type, ResolvedType::Int32);
    }

    #[test]
    fn test_method_sig_registration() {
        let mut ctx = TypeContext::new();

        let sig = MethodSig {
            name: "len".to_string(),
            self_type: super::super::types::SelfType::Ref,
            params: vec![],
            return_type: ResolvedType::UInt64,
            span: None,
        };

        ctx.register_method_sig("String".to_string(), "len".to_string(), sig);

        let lookup = ctx.lookup_method_sig("String", "len").unwrap();
        assert_eq!(lookup.return_type, ResolvedType::UInt64);
    }

    #[test]
    fn test_context_snapshot_restore() {
        let mut ctx = TypeContext::new();
        ctx.enter_scope(ScopeKind::Function);
        ctx.enter_scope(ScopeKind::Block);

        let snapshot = ctx.snapshot();
        assert_eq!(ctx.scope_depth(), 2);

        ctx.enter_scope(ScopeKind::Block);
        ctx.enter_scope(ScopeKind::Block);
        assert_eq!(ctx.scope_depth(), 4);

        ctx.restore(snapshot);
        assert_eq!(ctx.scope_depth(), 2);
    }

    #[test]
    fn test_builtins_registration() {
        let mut ctx = TypeContext::new();
        register_builtins(&mut ctx);

        assert!(ctx.is_function_defined("mudrā"));
        assert!(ctx.is_function_defined("nirgama"));

        let print_sig = ctx.lookup_function_sig("mudrā").unwrap();
        assert_eq!(print_sig.return_type, ResolvedType::Unit);

        let exit_sig = ctx.lookup_function_sig("nirgama").unwrap();
        assert_eq!(exit_sig.return_type, ResolvedType::Never);
    }

    #[test]
    fn test_scope_kind_names() {
        assert_eq!(ScopeKind::Global.sanskrit_name(), "विश्व-विषय");
        assert_eq!(ScopeKind::Function.description(), "function scope");
    }

    #[test]
    fn test_is_defined_in_current_scope() {
        let mut ctx = TypeContext::new();

        ctx.add_symbol_with_pramana(
            "global".to_string(),
            ResolvedType::Int32,
            Pramana::Pratyaksha,
            None,
        );

        ctx.enter_scope(ScopeKind::Function);

        // global is visible but not defined in current scope
        assert!(!ctx.is_defined_in_current_scope("global"));
        assert!(ctx.lookup_symbol("global").is_some());

        ctx.add_symbol_with_pramana(
            "local".to_string(),
            ResolvedType::Bool,
            Pramana::Anumana,
            None,
        );

        // local is defined in current scope
        assert!(ctx.is_defined_in_current_scope("local"));
    }
}
