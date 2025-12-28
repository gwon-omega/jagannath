//! Cross-Module Type Checking Integration (अन्तर-खण्ड प्रकार परीक्षा)
//!
//! This module integrates the module system with the type checker to enable:
//! - Cross-module type resolution (resolving types from imported modules)
//! - Visibility enforcement during type checking
//! - Import validation with type information
//! - Trait resolution across module boundaries
//!
//! # Philosophy: Sarva-Vyāpī Jñāna (सर्वव्यापी ज्ञान - All-pervading Knowledge)
//!
//! Just as Brahman (ब्रह्मन्) is the universal consciousness that pervades all beings,
//! the cross-module type checker acts as the universal knowledge system that
//! connects all modules, allowing types to flow across boundaries while
//! respecting visibility (māyā/illusion creates boundaries in the undivided whole).
//!
//! # Research Foundation
//!
//! Based on:
//! - Cardelli (1997): "Separate Compilation in Object-Oriented Languages"
//! - Leroy (1994): "Manifest Types, Modules with Higher-Order Functions"
//! - Harper & Stone (2000): "A Type-Theoretic Interpretation of Standard ML"

use super::graph::{ModuleGraph, ModuleId};
use super::symbol::{Symbol, SymbolKind, SymbolTable, FunctionSymbol, TypeSymbol, TypeKind};
use super::visibility::Visibility;
use super::{ImportDecl, ImportKind, ModuleContext};
use crate::lexer::Span;
use crate::parser::ast::Type;
use std::collections::HashMap;

// ============================================================================
// Cross-Module Type Environment (अन्तर-खण्ड प्रकार वातावरण)
// ============================================================================

/// Cross-module type environment that tracks types across module boundaries.
///
/// This is the "universal consciousness" (Brahman) of the type system,
/// containing all type information from all loaded modules.
#[derive(Debug)]
pub struct CrossModuleTypeEnv {
    /// Module graph for dependency resolution
    module_graph: ModuleGraph,

    /// Per-module symbol tables (module_id -> symbol_table)
    module_symbols: HashMap<ModuleId, SymbolTable>,

    /// Export maps: what each module publicly exports
    export_maps: HashMap<ModuleId, ExportMap>,

    /// Import resolution cache: (requesting_module, import_path_str) -> resolved_symbol
    import_cache: HashMap<(ModuleId, String), ResolvedImport>,
}

/// Map of exported symbols from a module
#[derive(Debug, Default, Clone)]
pub struct ExportMap {
    /// Exported types (name -> type info)
    pub types: HashMap<String, ExportedType>,

    /// Exported functions (name -> function info)
    pub functions: HashMap<String, ExportedFunction>,

    /// Exported traits (name -> trait info)
    pub traits: HashMap<String, ExportedTrait>,

    /// Re-exports from other modules (pub use)
    pub reexports: HashMap<String, ReexportInfo>,
}

/// Exported type information
#[derive(Debug, Clone)]
pub struct ExportedType {
    pub name: String,
    pub kind: TypeKind,
    pub generics: Vec<String>,
    pub visibility: Visibility,
    pub span: Span,
}

/// Exported function information
#[derive(Debug, Clone)]
pub struct ExportedFunction {
    pub name: String,
    pub param_types: Vec<Type>,
    pub return_type: Option<Type>,
    pub visibility: Visibility,
    pub span: Span,
}

/// Exported trait information
#[derive(Debug, Clone)]
pub struct ExportedTrait {
    pub name: String,
    pub methods: Vec<FunctionSymbol>,
    pub visibility: Visibility,
    pub span: Span,
}

/// Re-export information for `pub use` statements
#[derive(Debug, Clone)]
pub struct ReexportInfo {
    /// Original module where the symbol is defined
    pub source_module: ModuleId,
    /// Original name in source module
    pub source_name: String,
    /// Kind of symbol being re-exported
    pub kind: ReexportKind,
}

/// Kind of re-exported symbol
#[derive(Debug, Clone, PartialEq)]
pub enum ReexportKind {
    Type,
    Function,
    Trait,
    Module,
}

/// Result of resolving an import
#[derive(Debug, Clone)]
pub struct ResolvedImport {
    /// Module where the symbol is defined
    pub source_module: ModuleId,
    /// Resolved symbol information
    pub symbol: ResolvedSymbol,
    /// Whether access is allowed based on visibility
    pub access_allowed: bool,
    /// Reason if access denied
    pub denial_reason: Option<String>,
}

/// Resolved symbol information
#[derive(Debug, Clone)]
pub enum ResolvedSymbol {
    Type(ExportedType),
    Function(ExportedFunction),
    Trait(ExportedTrait),
    Module(ModuleId),
}

impl CrossModuleTypeEnv {
    /// Create a new cross-module type environment
    pub fn new() -> Self {
        Self {
            module_graph: ModuleGraph::new(),
            module_symbols: HashMap::new(),
            export_maps: HashMap::new(),
            import_cache: HashMap::new(),
        }
    }

    /// Create from an existing module context
    pub fn from_module_context(ctx: &ModuleContext) -> Self {
        Self {
            module_graph: ctx.graph.clone(),
            module_symbols: HashMap::new(),
            export_maps: HashMap::new(),
            import_cache: HashMap::new(),
        }
    }

    /// Get the module graph
    pub fn module_graph(&self) -> &ModuleGraph {
        &self.module_graph
    }

    /// Register a module's symbol table
    pub fn register_module_symbols(&mut self, module_id: ModuleId, symbols: SymbolTable) {
        self.module_symbols.insert(module_id, symbols);
    }

    /// Build export map for a module from its symbol table
    pub fn build_export_map(&mut self, module_id: ModuleId) -> Result<(), CrossModuleError> {
        let symbols = self.module_symbols.get(&module_id).ok_or_else(|| {
            CrossModuleError::ModuleNotFound {
                module_id,
                reason: "Module symbols not registered".to_string(),
            }
        })?;

        let mut export_map = ExportMap::default();

        // Collect public exports from symbol table
        // Iterate over all symbols and check if they're public
        for symbol in symbols.all_symbols() {
            if symbol.visibility != Visibility::Public {
                continue;
            }

            let name = symbol.name.clone();

            match &symbol.kind {
                SymbolKind::Type(type_sym) => {
                    export_map.types.insert(
                        name.clone(),
                        ExportedType {
                            name,
                            kind: type_sym.kind.clone(),
                            generics: type_sym.generics.clone(),
                            visibility: Visibility::Public,
                            span: symbol.span,
                        },
                    );
                }
                SymbolKind::Function(func_sym) => {
                    export_map.functions.insert(
                        name.clone(),
                        ExportedFunction {
                            name,
                            param_types: func_sym.param_types.clone(),
                            return_type: func_sym.return_type.clone(),
                            visibility: Visibility::Public,
                            span: symbol.span,
                        },
                    );
                }
                SymbolKind::Trait(trait_sym) => {
                    export_map.traits.insert(
                        name.clone(),
                        ExportedTrait {
                            name,
                            methods: trait_sym.methods.clone(),
                            visibility: Visibility::Public,
                            span: symbol.span,
                        },
                    );
                }
                _ => {}
            }
        }

        self.export_maps.insert(module_id, export_map);
        Ok(())
    }

    /// Resolve an import from one module to another
    ///
    /// # Arguments
    /// * `requesting_module` - The module requesting the import
    /// * `import` - The import declaration to resolve
    ///
    /// # Returns
    /// The resolved import or an error
    pub fn resolve_import(
        &mut self,
        requesting_module: ModuleId,
        import: &ImportDecl,
    ) -> Result<ResolvedImport, CrossModuleError> {
        // Convert path to string for cache key
        let path_str = import.path.join("::");
        let cache_key = (requesting_module, path_str.clone());

        // Check cache first
        if let Some(cached) = self.import_cache.get(&cache_key) {
            return Ok(cached.clone());
        }

        // Find target module
        let target_module = self
            .module_graph
            .find_by_path(&import.path)
            .ok_or_else(|| CrossModuleError::ModuleNotFound {
                module_id: ModuleId(0),
                reason: format!("Module path not found: {}", path_str),
            })?;

        // Get export map for target module
        let export_map = self.export_maps.get(&target_module).ok_or_else(|| {
            CrossModuleError::ModuleNotLoaded {
                path: path_str.clone(),
            }
        })?;

        // Resolve based on import kind
        let resolved = match &import.symbols {
            ImportKind::Selective(names) => {
                // For named imports, resolve each name
                // Return first match (simplified; real impl would return all)
                let name = names.first().ok_or_else(|| CrossModuleError::EmptyImport {
                    path: path_str.clone(),
                })?;

                self.resolve_name_in_export_map(target_module, export_map, name)?
            }
            ImportKind::Glob => {
                // Glob import - return module reference
                ResolvedImport {
                    source_module: target_module,
                    symbol: ResolvedSymbol::Module(target_module),
                    access_allowed: true,
                    denial_reason: None,
                }
            }
            ImportKind::Module => {
                // Module import
                ResolvedImport {
                    source_module: target_module,
                    symbol: ResolvedSymbol::Module(target_module),
                    access_allowed: true,
                    denial_reason: None,
                }
            }
        };

        // Check visibility
        let access_allowed = self.check_visibility_for_import(&resolved.symbol);

        let final_resolved = ResolvedImport {
            access_allowed,
            denial_reason: if !access_allowed {
                Some("Symbol is not visible from requesting module (Pratyāhāra violation)".to_string())
            } else {
                None
            },
            ..resolved
        };

        // Cache result
        self.import_cache.insert(cache_key, final_resolved.clone());

        Ok(final_resolved)
    }

    /// Resolve a name within an export map
    fn resolve_name_in_export_map(
        &self,
        source_module: ModuleId,
        export_map: &ExportMap,
        name: &str,
    ) -> Result<ResolvedImport, CrossModuleError> {
        // Check types
        if let Some(ty) = export_map.types.get(name) {
            return Ok(ResolvedImport {
                source_module,
                symbol: ResolvedSymbol::Type(ty.clone()),
                access_allowed: true,
                denial_reason: None,
            });
        }

        // Check functions
        if let Some(func) = export_map.functions.get(name) {
            return Ok(ResolvedImport {
                source_module,
                symbol: ResolvedSymbol::Function(func.clone()),
                access_allowed: true,
                denial_reason: None,
            });
        }

        // Check traits
        if let Some(tr) = export_map.traits.get(name) {
            return Ok(ResolvedImport {
                source_module,
                symbol: ResolvedSymbol::Trait(tr.clone()),
                access_allowed: true,
                denial_reason: None,
            });
        }

        // Check re-exports
        if let Some(reexport) = export_map.reexports.get(name) {
            // Resolve through re-export chain
            return self.resolve_reexport(reexport);
        }

        Err(CrossModuleError::SymbolNotFound {
            symbol: name.to_string(),
            module_path: format!("module_{}", source_module.0),
        })
    }

    /// Resolve a re-export chain
    fn resolve_reexport(&self, reexport: &ReexportInfo) -> Result<ResolvedImport, CrossModuleError> {
        let source_exports = self.export_maps.get(&reexport.source_module).ok_or_else(|| {
            CrossModuleError::ModuleNotLoaded {
                path: format!("module_{}", reexport.source_module.0),
            }
        })?;

        self.resolve_name_in_export_map(
            reexport.source_module,
            source_exports,
            &reexport.source_name,
        )
    }

    /// Check visibility for an import
    fn check_visibility_for_import(&self, symbol: &ResolvedSymbol) -> bool {
        // Get visibility from symbol
        let visibility = match symbol {
            ResolvedSymbol::Type(ty) => &ty.visibility,
            ResolvedSymbol::Function(func) => &func.visibility,
            ResolvedSymbol::Trait(tr) => &tr.visibility,
            ResolvedSymbol::Module(_) => return true, // Modules are always accessible
        };

        // Check visibility allows access
        matches!(visibility, Visibility::Public)
    }

    /// Resolve a type reference across module boundaries
    ///
    /// This is the key integration point between the module system and type checker.
    pub fn resolve_type(
        &self,
        current_module: ModuleId,
        type_ref: &TypeReference,
    ) -> Result<ResolvedTypeInfo, TypeResolutionError> {
        match type_ref {
            TypeReference::Simple(name) => {
                // First check current module's symbols
                if let Some(symbols) = self.module_symbols.get(&current_module) {
                    if let Some(symbol) = symbols.lookup(name) {
                        if let SymbolKind::Type(type_sym) = &symbol.kind {
                            return Ok(ResolvedTypeInfo {
                                module: current_module,
                                name: name.clone(),
                                generics: type_sym.generics.clone(),
                                kind: ResolvedTypeKind::Local,
                            });
                        }
                    }
                }

                // Not found locally - error
                Err(TypeResolutionError::TypeNotFound {
                    name: name.clone(),
                    searched_module: current_module,
                })
            }

            TypeReference::Qualified { module_path, name } => {
                // Find target module
                let target_module =
                    self.module_graph
                        .find_by_path(module_path)
                        .ok_or_else(|| TypeResolutionError::ModuleNotFound {
                            path: module_path.join("::"),
                        })?;

                // Check exports
                let export_map = self.export_maps.get(&target_module).ok_or_else(|| {
                    TypeResolutionError::ModuleNotLoaded {
                        module: target_module,
                    }
                })?;

                // Find type in exports
                let exported_type = export_map.types.get(name).ok_or_else(|| {
                    TypeResolutionError::TypeNotExported {
                        name: name.clone(),
                        module: target_module,
                    }
                })?;

                // Check visibility
                if !matches!(exported_type.visibility, Visibility::Public) {
                    return Err(TypeResolutionError::VisibilityViolation {
                        name: name.clone(),
                        visibility: exported_type.visibility.clone(),
                    });
                }

                Ok(ResolvedTypeInfo {
                    module: target_module,
                    name: name.clone(),
                    generics: exported_type.generics.clone(),
                    kind: ResolvedTypeKind::Imported,
                })
            }
        }
    }

    /// Get all exported types from a module (for autocomplete, etc.)
    pub fn get_module_exports(&self, module_id: ModuleId) -> Option<&ExportMap> {
        self.export_maps.get(&module_id)
    }

    /// Check if a module has been loaded
    pub fn is_module_loaded(&self, module_id: ModuleId) -> bool {
        self.module_symbols.contains_key(&module_id)
    }
}

impl Default for CrossModuleTypeEnv {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Type Reference (for cross-module resolution)
// ============================================================================

/// Reference to a type that may be in another module
#[derive(Debug, Clone)]
pub enum TypeReference {
    /// Simple name (may be local or imported)
    Simple(String),

    /// Fully qualified: module::path::Type
    Qualified { module_path: Vec<String>, name: String },
}

/// Result of resolving a type reference
#[derive(Debug, Clone)]
pub struct ResolvedTypeInfo {
    /// Module where type is defined
    pub module: ModuleId,
    /// Type name
    pub name: String,
    /// Generic parameters
    pub generics: Vec<String>,
    /// How type was resolved
    pub kind: ResolvedTypeKind,
}

/// How a type was resolved
#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedTypeKind {
    /// Defined in current module
    Local,
    /// Imported from another module
    Imported,
    /// Re-exported through intermediate module
    Reexported,
}

// ============================================================================
// Error Types
// ============================================================================

/// Errors during cross-module operations
#[derive(Debug, Clone)]
pub enum CrossModuleError {
    ModuleNotFound {
        module_id: ModuleId,
        reason: String,
    },
    ModuleNotLoaded {
        path: String,
    },
    SymbolNotFound {
        symbol: String,
        module_path: String,
    },
    EmptyImport {
        path: String,
    },
    VisibilityViolation {
        symbol: String,
        visibility: Visibility,
    },
}

impl std::fmt::Display for CrossModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrossModuleError::ModuleNotFound { module_id, reason } => {
                write!(f, "Module_{} not found: {} (खण्ड अनुपलब्ध)", module_id.0, reason)
            }
            CrossModuleError::ModuleNotLoaded { path } => {
                write!(f, "Module '{}' not loaded (खण्ड अप्रवेशित)", path)
            }
            CrossModuleError::SymbolNotFound { symbol, module_path } => {
                write!(f, "Symbol '{}' not found in '{}' (प्रतीक अनुपलब्ध)", symbol, module_path)
            }
            CrossModuleError::EmptyImport { path } => {
                write!(f, "Empty import for path '{}' (रिक्त आयाति)", path)
            }
            CrossModuleError::VisibilityViolation { symbol, visibility } => {
                write!(
                    f,
                    "Symbol '{}' has {:?} visibility - access denied (प्रत्याहार उल्लंघन)",
                    symbol, visibility
                )
            }
        }
    }
}

impl std::error::Error for CrossModuleError {}

/// Errors during type resolution
#[derive(Debug, Clone)]
pub enum TypeResolutionError {
    TypeNotFound {
        name: String,
        searched_module: ModuleId,
    },
    ModuleNotFound {
        path: String,
    },
    ModuleNotLoaded {
        module: ModuleId,
    },
    TypeNotExported {
        name: String,
        module: ModuleId,
    },
    VisibilityViolation {
        name: String,
        visibility: Visibility,
    },
    CircularDependency {
        cycle: Vec<ModuleId>,
    },
}

impl std::fmt::Display for TypeResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeResolutionError::TypeNotFound { name, searched_module } => {
                write!(
                    f,
                    "Type '{}' not found in module_{} (प्रकार अनुपलब्ध)",
                    name, searched_module.0
                )
            }
            TypeResolutionError::ModuleNotFound { path } => {
                write!(f, "Module '{}' not found (खण्ड अनुपलब्ध)", path)
            }
            TypeResolutionError::ModuleNotLoaded { module } => {
                write!(f, "Module_{} not loaded (खण्ड अप्रवेशित)", module.0)
            }
            TypeResolutionError::TypeNotExported { name, module } => {
                write!(
                    f,
                    "Type '{}' not exported from module_{} (प्रकार अनिर्यात)",
                    name, module.0
                )
            }
            TypeResolutionError::VisibilityViolation { name, visibility } => {
                write!(
                    f,
                    "Type '{}' has {:?} visibility - access denied (प्रत्याहार उल्लंघन)",
                    name, visibility
                )
            }
            TypeResolutionError::CircularDependency { cycle } => {
                let cycle_str: Vec<_> = cycle.iter().map(|m| format!("module_{}", m.0)).collect();
                write!(
                    f,
                    "Circular type dependency: {} (चक्रव्यूह)",
                    cycle_str.join(" → ")
                )
            }
        }
    }
}

impl std::error::Error for TypeResolutionError {}

// ============================================================================
// Import Validator (आयाति परीक्षक)
// ============================================================================

/// Validates imports for type correctness
pub struct ImportValidator<'a> {
    type_env: &'a CrossModuleTypeEnv,
}

impl<'a> ImportValidator<'a> {
    pub fn new(type_env: &'a CrossModuleTypeEnv) -> Self {
        Self { type_env }
    }

    /// Validate all imports in a module
    pub fn validate_imports(
        &self,
        module_id: ModuleId,
        imports: &[ImportDecl],
    ) -> Vec<ImportValidationError> {
        let mut errors = Vec::new();

        for import in imports {
            if let Err(e) = self.validate_single_import(module_id, import) {
                errors.push(e);
            }
        }

        errors
    }

    /// Validate a single import
    fn validate_single_import(
        &self,
        _module_id: ModuleId,
        import: &ImportDecl,
    ) -> Result<(), ImportValidationError> {
        let path_str = import.path.join("::");

        // Find target module
        let target = self
            .type_env
            .module_graph
            .find_by_path(&import.path)
            .ok_or_else(|| ImportValidationError::ModuleNotFound {
                path: path_str.clone(),
                span: import.span,
            })?;

        // Check target module is loaded
        if !self.type_env.is_module_loaded(target) {
            return Err(ImportValidationError::ModuleNotLoaded {
                path: path_str.clone(),
                span: import.span,
            });
        }

        // Validate selective imports
        if let ImportKind::Selective(names) = &import.symbols {
            let export_map = self.type_env.get_module_exports(target).ok_or_else(|| {
                ImportValidationError::ModuleNotLoaded {
                    path: path_str.clone(),
                    span: import.span,
                }
            })?;

            for name in names {
                if !self.name_exists_in_exports(export_map, name) {
                    return Err(ImportValidationError::SymbolNotFound {
                        symbol: name.clone(),
                        module_path: path_str.clone(),
                        span: import.span,
                    });
                }
            }
        }

        Ok(())
    }

    /// Check if a name exists in export map
    fn name_exists_in_exports(&self, export_map: &ExportMap, name: &str) -> bool {
        export_map.types.contains_key(name)
            || export_map.functions.contains_key(name)
            || export_map.traits.contains_key(name)
            || export_map.reexports.contains_key(name)
    }
}

/// Import validation errors
#[derive(Debug, Clone)]
pub enum ImportValidationError {
    ModuleNotFound {
        path: String,
        span: Span,
    },
    ModuleNotLoaded {
        path: String,
        span: Span,
    },
    SymbolNotFound {
        symbol: String,
        module_path: String,
        span: Span,
    },
    VisibilityViolation {
        symbol: String,
        module_path: String,
        visibility: Visibility,
        span: Span,
    },
}

impl std::fmt::Display for ImportValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportValidationError::ModuleNotFound { path, .. } => {
                write!(f, "Cannot find module '{}' (खण्ड अनुपलब्ध)", path)
            }
            ImportValidationError::ModuleNotLoaded { path, .. } => {
                write!(
                    f,
                    "Module '{}' found but not loaded - possible circular dependency (खण्ड अप्रवेशित)",
                    path
                )
            }
            ImportValidationError::SymbolNotFound { symbol, module_path, .. } => {
                write!(
                    f,
                    "Symbol '{}' not found in module '{}' (प्रतीक अनुपलब्ध)",
                    symbol, module_path
                )
            }
            ImportValidationError::VisibilityViolation { symbol, module_path, visibility, .. } => {
                write!(
                    f,
                    "Cannot import '{}' from '{}': {:?} visibility (प्रत्याहार उल्लंघन)",
                    symbol, module_path, visibility
                )
            }
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
    fn test_cross_module_type_env_creation() {
        let env = CrossModuleTypeEnv::new();
        assert!(env.module_symbols.is_empty());
        assert!(env.export_maps.is_empty());
    }

    #[test]
    fn test_export_map_default() {
        let export_map = ExportMap::default();
        assert!(export_map.types.is_empty());
        assert!(export_map.functions.is_empty());
        assert!(export_map.traits.is_empty());
    }

    #[test]
    fn test_type_resolution_error_display() {
        let err = TypeResolutionError::TypeNotFound {
            name: "Point".to_string(),
            searched_module: ModuleId(1),
        };
        let display = format!("{}", err);
        assert!(display.contains("Point"));
        assert!(display.contains("module_1"));
        assert!(display.contains("प्रकार अनुपलब्ध"));
    }

    #[test]
    fn test_type_reference_simple() {
        let env = CrossModuleTypeEnv::new();
        let type_ref = TypeReference::Simple("NonExistent".to_string());
        let result = env.resolve_type(ModuleId(0), &type_ref);
        assert!(result.is_err());
    }

    #[test]
    fn test_visibility_violation_error() {
        let err = TypeResolutionError::VisibilityViolation {
            name: "InternalType".to_string(),
            visibility: Visibility::Private,
        };
        let display = format!("{}", err);
        assert!(display.contains("InternalType"));
        assert!(display.contains("प्रत्याहार उल्लंघन"));
    }

    #[test]
    fn test_cross_module_error_display() {
        let err = CrossModuleError::SymbolNotFound {
            symbol: "Foo".to_string(),
            module_path: "std::collections".to_string(),
        };
        let display = format!("{}", err);
        assert!(display.contains("Foo"));
        assert!(display.contains("std::collections"));
    }
}
