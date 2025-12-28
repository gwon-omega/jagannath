//! Symbol Table (Pratīka Paṭṭikā - प्रतीक पट्टिका)
//!
//! Sanskrit: प्रतीक (pratīka) = symbol, पट्टिका (paṭṭikā) = table, list
//!
//! Implements a hierarchical symbol table with:
//! - Lexical scoping (nested scopes)
//! - Symbol resolution with shadowing
//! - Visibility enforcement (Pratyahara principle)
//! - Cross-module symbol lookup
//!
//! Based on research:
//! - "Efficient Implementation of Symbol Tables" (Aho, Sethi, Ullman)
//! - "Symbol Table Design" from Crafting Interpreters (Nystrom)

use super::visibility::Visibility;
use crate::lexer::Span;
use crate::parser::ast::{Type, FunctionDef, TypeDef};
use std::collections::HashMap;

/// A symbol in the symbol table
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Symbol name
    pub name: String,
    /// Kind of symbol
    pub kind: SymbolKind,
    /// Symbol's type (if applicable)
    pub ty: Option<Type>,
    /// Visibility level
    pub visibility: Visibility,
    /// Module where defined
    pub module: Vec<String>,
    /// Source location
    pub span: Span,
    /// Documentation (if any)
    pub docs: Option<String>,
    /// Is this symbol mutable (for variables)
    pub mutable: bool,
}

/// Kind of symbol
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    /// Variable or constant
    Variable,
    /// Function
    Function(FunctionSymbol),
    /// Type (struct, enum, alias)
    Type(TypeSymbol),
    /// Module
    Module,
    /// Import alias
    ImportAlias {
        original_path: Vec<String>,
    },
    /// Trait/Interface (guṇa)
    Trait(TraitSymbol),
    /// Macro
    Macro,
}

/// Function symbol details
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSymbol {
    /// Parameter types
    pub param_types: Vec<Type>,
    /// Return type
    pub return_type: Option<Type>,
    /// Is this a method (has self parameter)
    pub is_method: bool,
    /// Generic parameters
    pub generics: Vec<String>,
}

/// Type symbol details
#[derive(Debug, Clone, PartialEq)]
pub struct TypeSymbol {
    /// Kind of type definition
    pub kind: TypeKind,
    /// Generic parameters
    pub generics: Vec<String>,
}

/// Kind of type definition
#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Struct,
    Enum,
    Alias,
    Primitive,
}

/// Trait symbol details
#[derive(Debug, Clone, PartialEq)]
pub struct TraitSymbol {
    /// Required method signatures
    pub methods: Vec<FunctionSymbol>,
    /// Associated types
    pub associated_types: Vec<String>,
}

/// Symbol table with hierarchical scopes
#[derive(Debug, Clone)]
pub struct SymbolTable {
    /// Scopes stack (innermost last)
    scopes: Vec<Scope>,
    /// Global symbols (module-level exports)
    globals: HashMap<String, Symbol>,
}

/// A single scope
#[derive(Debug, Clone)]
struct Scope {
    /// Symbols in this scope
    symbols: HashMap<String, Symbol>,
    /// Scope kind for visibility rules
    kind: ScopeKind,
}

/// Kind of scope
#[derive(Debug, Clone, PartialEq)]
pub enum ScopeKind {
    /// Module scope (top-level)
    Module,
    /// Function scope
    Function,
    /// Block scope (if, loop, etc)
    Block,
    /// Impl block scope
    Impl,
}

impl SymbolTable {
    /// Create an empty symbol table with global scope
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope {
                symbols: HashMap::new(),
                kind: ScopeKind::Module,
            }],
            globals: HashMap::new(),
        }
    }

    /// Enter a new scope
    pub fn enter_scope(&mut self, kind: ScopeKind) {
        self.scopes.push(Scope {
            symbols: HashMap::new(),
            kind,
        });
    }

    /// Exit current scope
    pub fn exit_scope(&mut self) -> HashMap<String, Symbol> {
        self.scopes.pop()
            .map(|s| s.symbols)
            .unwrap_or_default()
    }

    /// Current scope depth
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }

    /// Define a symbol in current scope
    pub fn define(&mut self, symbol: Symbol) -> Result<(), SymbolError> {
        let scope = self.scopes.last_mut()
            .ok_or(SymbolError::NoActiveScope)?;

        if scope.symbols.contains_key(&symbol.name) {
            return Err(SymbolError::AlreadyDefined {
                name: symbol.name.clone(),
                span: symbol.span.clone(),
            });
        }

        // If public, also add to globals
        if symbol.visibility == Visibility::Public {
            self.globals.insert(symbol.name.clone(), symbol.clone());
        }

        scope.symbols.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    /// Define a global/exported symbol
    pub fn define_global(&mut self, symbol: Symbol) -> Result<(), SymbolError> {
        if self.globals.contains_key(&symbol.name) {
            return Err(SymbolError::AlreadyDefined {
                name: symbol.name.clone(),
                span: symbol.span.clone(),
            });
        }
        self.globals.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    /// Lookup symbol, searching from innermost scope outward
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        // Search scopes from innermost to outermost
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.symbols.get(name) {
                return Some(sym);
            }
        }

        // Check globals
        self.globals.get(name)
    }

    /// Lookup symbol only in current scope
    pub fn lookup_current(&self, name: &str) -> Option<&Symbol> {
        self.scopes.last()?.symbols.get(name)
    }

    /// Lookup global/exported symbol
    pub fn lookup_global(&self, name: &str) -> Option<&Symbol> {
        self.globals.get(name)
    }

    /// Get all exported symbols
    pub fn get_exports(&self) -> impl Iterator<Item = &Symbol> {
        self.globals.values()
    }

    /// Get all symbols (both global and in current scope)
    /// Used for building export maps and cross-module type checking
    pub fn all_symbols(&self) -> impl Iterator<Item = &Symbol> {
        // Combine globals with current scope symbols
        self.globals.values().chain(
            self.scopes.iter().flat_map(|s| s.symbols.values())
        )
    }

    /// Check if symbol is defined anywhere
    pub fn is_defined(&self, name: &str) -> bool {
        self.lookup(name).is_some()
    }

    /// Get all symbols in current scope
    pub fn current_symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.scopes.last()
            .map(|s| s.symbols.values())
            .into_iter()
            .flatten()
    }

    /// Merge another symbol table's exports into this one
    pub fn merge_exports(&mut self, other: &SymbolTable, alias: Option<&str>) {
        for (name, symbol) in &other.globals {
            let key = if let Some(alias) = alias {
                format!("{}::{}", alias, name)
            } else {
                name.clone()
            };
            self.globals.insert(key, symbol.clone());
        }
    }

    /// Import specific symbols from another table
    pub fn import_symbols(&mut self, other: &SymbolTable, names: &[String]) -> Result<(), SymbolError> {
        for name in names {
            if let Some(symbol) = other.globals.get(name) {
                if symbol.visibility != Visibility::Public {
                    return Err(SymbolError::NotExported {
                        name: name.clone(),
                        module: symbol.module.clone(),
                    });
                }
                self.globals.insert(name.clone(), symbol.clone());
            } else {
                return Err(SymbolError::NotFound(name.clone()));
            }
        }
        Ok(())
    }

    /// Get all function symbols
    pub fn get_functions(&self) -> impl Iterator<Item = &Symbol> {
        self.globals.values().filter(|s| matches!(s.kind, SymbolKind::Function(_)))
    }

    /// Get all type symbols
    pub fn get_types(&self) -> impl Iterator<Item = &Symbol> {
        self.globals.values().filter(|s| matches!(s.kind, SymbolKind::Type(_)))
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Symbol table errors
#[derive(Debug, Clone)]
pub enum SymbolError {
    /// Symbol already defined in current scope
    AlreadyDefined {
        name: String,
        span: Span,
    },
    /// Symbol not found
    NotFound(String),
    /// Symbol not exported from module
    NotExported {
        name: String,
        module: Vec<String>,
    },
    /// No active scope
    NoActiveScope,
    /// Type mismatch during lookup
    TypeMismatch {
        name: String,
        expected: String,
        found: String,
    },
}

impl std::fmt::Display for SymbolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolError::AlreadyDefined { name, span } => {
                write!(f, "Symbol '{}' already defined at {:?}", name, span)
            }
            SymbolError::NotFound(name) => {
                write!(f, "Symbol '{}' not found", name)
            }
            SymbolError::NotExported { name, module } => {
                write!(f, "Symbol '{}' not exported from module '{}'", name, module.join("::"))
            }
            SymbolError::NoActiveScope => {
                write!(f, "No active scope")
            }
            SymbolError::TypeMismatch { name, expected, found } => {
                write!(f, "Type mismatch for '{}': expected {}, found {}", name, expected, found)
            }
        }
    }
}

impl std::error::Error for SymbolError {}

/// Helper to create symbols
impl Symbol {
    /// Create a variable symbol
    pub fn variable(name: String, ty: Type, mutable: bool, span: Span) -> Self {
        Self {
            name,
            kind: SymbolKind::Variable,
            ty: Some(ty),
            visibility: Visibility::Private,
            module: Vec::new(),
            span,
            docs: None,
            mutable,
        }
    }

    /// Create a function symbol
    pub fn function(name: String, params: Vec<Type>, ret: Option<Type>, span: Span) -> Self {
        Self {
            name,
            kind: SymbolKind::Function(FunctionSymbol {
                param_types: params,
                return_type: ret,
                is_method: false,
                generics: Vec::new(),
            }),
            ty: None,
            visibility: Visibility::Private,
            module: Vec::new(),
            span,
            docs: None,
            mutable: false,
        }
    }

    /// Create a type symbol
    pub fn type_def(name: String, kind: TypeKind, generics: Vec<String>, span: Span) -> Self {
        Self {
            name,
            kind: SymbolKind::Type(TypeSymbol { kind, generics }),
            ty: None,
            visibility: Visibility::Private,
            module: Vec::new(),
            span,
            docs: None,
            mutable: false,
        }
    }

    /// Make symbol public (prakāśita)
    pub fn public(mut self) -> Self {
        self.visibility = Visibility::Public;
        self
    }

    /// Add documentation
    pub fn with_docs(mut self, docs: String) -> Self {
        self.docs = Some(docs);
        self
    }

    /// Set module path
    pub fn in_module(mut self, module: Vec<String>) -> Self {
        self.module = module;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_and_lookup() {
        let mut table = SymbolTable::new();
        let sym = Symbol::variable(
            "x".to_string(),
            Type::Inferred,
            false,
            Span::dummy(),
        );
        table.define(sym).unwrap();

        assert!(table.lookup("x").is_some());
        assert!(table.lookup("y").is_none());
    }

    #[test]
    fn test_scope_shadowing() {
        let mut table = SymbolTable::new();

        // Define x in outer scope
        let outer = Symbol::variable("x".to_string(), Type::Inferred, false, Span::dummy());
        table.define(outer).unwrap();

        // Enter inner scope
        table.enter_scope(ScopeKind::Block);

        // Define x in inner scope (shadowing)
        let inner = Symbol::variable("x".to_string(), Type::Inferred, true, Span::dummy());
        table.define(inner).unwrap();

        // Lookup should find inner
        let found = table.lookup("x").unwrap();
        assert!(found.mutable);

        // Exit inner scope
        table.exit_scope();

        // Lookup should find outer
        let found = table.lookup("x").unwrap();
        assert!(!found.mutable);
    }

    #[test]
    fn test_public_exports() {
        let mut table = SymbolTable::new();

        let private = Symbol::variable("x".to_string(), Type::Inferred, false, Span::dummy());
        let public = Symbol::variable("y".to_string(), Type::Inferred, false, Span::dummy()).public();

        table.define(private).unwrap();
        table.define(public).unwrap();

        // Private not in globals
        assert!(table.lookup_global("x").is_none());
        // Public in globals
        assert!(table.lookup_global("y").is_some());
    }

    #[test]
    fn test_already_defined_error() {
        let mut table = SymbolTable::new();
        let sym1 = Symbol::variable("x".to_string(), Type::Inferred, false, Span::dummy());
        let sym2 = Symbol::variable("x".to_string(), Type::Inferred, false, Span::dummy());

        table.define(sym1).unwrap();
        let result = table.define(sym2);

        assert!(matches!(result, Err(SymbolError::AlreadyDefined { .. })));
    }
}
