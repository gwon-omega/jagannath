//! Module System (Khaṇḍa - खण्ड)
//!
//! Sanskrit: खण्ड (khaṇḍa) = section, part, module
//!
//! Implements a hierarchical module system with:
//! - Module graph construction and topological sorting
//! - Symbol table management with visibility rules
//! - Cross-module type resolution
//! - Circular dependency detection (Cakravyūha check)
//!
//! Based on research from:
//! - "Separate Compilation in Object-Oriented Languages" (Cardelli, 1997)
//! - Rust Module System (2018 Edition)
//! - Go Module System (Go 1.11+)
//! - Swift Module System
//!
//! Sanskrit mappings:
//! - āyāti (आयाति) = import ("comes to")
//! - niryāti (निर्याति) = export ("goes out")
//! - prakāśita (प्रकाशित) = public ("made visible")
//! - gupya (गुप्य) = private ("hidden")
//! - khaṇḍa (खण्ड) = module ("section")

pub mod graph;
pub mod resolver;
pub mod symbol;
pub mod typeck_integration;
pub mod visibility;

pub use graph::{ModuleGraph, ModuleId};
pub use resolver::{ModuleResolver, ResolveError};
pub use symbol::{Symbol, SymbolKind, SymbolTable};
pub use typeck_integration::{CrossModuleTypeEnv, ResolvedTypeInfo, TypeReference};
pub use visibility::{Visibility, VisibilityScope};

use crate::parser::ast::{Ast, Identifier, ImportStmt, Item};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A compiled module in the module graph
#[derive(Debug, Clone)]
pub struct Module {
    /// Unique identifier
    pub id: ModuleId,
    /// Module name (Sanskrit: नाम nāma)
    pub name: String,
    /// Module path (e.g., "stdlib::sankhya::prakara")
    pub path: Vec<String>,
    /// File system path to source
    pub file_path: PathBuf,
    /// Parsed AST
    pub ast: Option<Ast>,
    /// Exported symbols (niryāti)
    pub exports: SymbolTable,
    /// Imported symbols (āyāti)
    pub imports: Vec<ImportDecl>,
    /// Is this module compiled
    pub compiled: bool,
}

/// Import declaration
#[derive(Debug, Clone)]
pub struct ImportDecl {
    /// Full path of import
    pub path: Vec<String>,
    /// Optional alias
    pub alias: Option<String>,
    /// Symbols imported (if selective)
    pub symbols: ImportKind,
    /// Source span for error reporting
    pub span: crate::lexer::Span,
}

/// Kind of import
#[derive(Debug, Clone)]
pub enum ImportKind {
    /// Import entire module
    Module,
    /// Import all exported symbols (use foo::*)
    Glob,
    /// Import specific symbols (use foo::{a, b})
    Selective(Vec<String>),
}

/// Module compilation context
pub struct ModuleContext {
    /// Module graph
    pub graph: ModuleGraph,
    /// Module resolver
    pub resolver: ModuleResolver,
    /// Root module path
    pub root_path: PathBuf,
    /// Standard library path
    pub stdlib_path: Option<PathBuf>,
}

impl ModuleContext {
    /// Create a new module context
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            graph: ModuleGraph::new(),
            resolver: ModuleResolver::new(),
            root_path,
            stdlib_path: None,
        }
    }

    /// Set standard library path
    pub fn with_stdlib(mut self, path: PathBuf) -> Self {
        self.stdlib_path = Some(path);
        self
    }

    /// Compile all modules starting from entry point
    /// Uses Kahn's algorithm for topological sort
    pub fn compile_all(&mut self, entry_file: &Path) -> Result<Vec<ModuleId>, ModuleError> {
        // 1. Build module graph by parsing all imports
        let entry_id = self.discover_modules(entry_file)?;

        // 2. Check for circular dependencies (Cakravyūha)
        if let Some(cycle) = self.graph.find_cycle() {
            return Err(ModuleError::CircularDependency(cycle));
        }

        // 3. Topological sort for compilation order
        let order = self.graph.topological_order()?;

        Ok(order)
    }

    /// Discover all modules reachable from entry point
    fn discover_modules(&mut self, entry_file: &Path) -> Result<ModuleId, ModuleError> {
        // Parse entry file
        let source = std::fs::read_to_string(entry_file)
            .map_err(|e| ModuleError::IoError(entry_file.to_path_buf(), e.to_string()))?;

        let mut lexer = crate::lexer::Lexer::new(&source);
        let tokens = lexer.tokenize();

        let mut parser = crate::parser::Parser::new(tokens);
        let ast = parser.parse().map_err(|errors| {
            ModuleError::ParseError(
                entry_file.to_path_buf(),
                errors.into_iter().map(|e| e.message).collect(),
            )
        })?;

        // Extract module name from file path
        let module_name = entry_file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("main")
            .to_string();

        // Create module
        let module = Module {
            id: ModuleId(self.graph.modules.len() as u32),
            name: module_name.clone(),
            path: vec![module_name.clone()],
            file_path: entry_file.to_path_buf(),
            ast: Some(ast.clone()),
            exports: SymbolTable::new(),
            imports: Vec::new(),
            compiled: false,
        };

        let id = self.graph.add_module(module);

        // Process imports recursively
        self.process_imports(id, &ast)?;

        Ok(id)
    }

    /// Process all imports in a module
    fn process_imports(&mut self, module_id: ModuleId, ast: &Ast) -> Result<(), ModuleError> {
        for item in &ast.items {
            if let Item::Import(import) = item {
                let dep_id = self.resolve_import(module_id, import)?;
                self.graph.add_dependency(module_id, dep_id);
            }
        }
        Ok(())
    }

    /// Resolve an import statement to a module
    fn resolve_import(
        &mut self,
        from: ModuleId,
        import: &ImportStmt,
    ) -> Result<ModuleId, ModuleError> {
        let path: Vec<String> = import.path.iter().map(|i| i.name.clone()).collect();

        // Check if already loaded
        if let Some(id) = self.graph.find_by_path(&path) {
            return Ok(id);
        }

        // Resolve to file path
        let file_path =
            self.resolver
                .resolve_path(&path, &self.root_path, self.stdlib_path.as_deref())?;

        // Load and add to graph
        self.discover_modules(&file_path)
    }
}

/// Module system errors
#[derive(Debug, Clone)]
pub enum ModuleError {
    /// Circular dependency detected (Cakravyūha - चक्रव्यूह)
    /// Named after the inescapable military formation in Mahabharata
    CircularDependency(Vec<ModuleId>),
    /// Module not found
    NotFound(Vec<String>),
    /// IO error reading module
    IoError(PathBuf, String),
    /// Parse error in module
    ParseError(PathBuf, Vec<String>),
    /// Symbol not exported
    SymbolNotExported { module: Vec<String>, symbol: String },
    /// Visibility violation (Pratyahara)
    VisibilityViolation {
        symbol: String,
        required: Visibility,
        actual: Visibility,
    },
    /// Ambiguous import
    AmbiguousImport {
        symbol: String,
        modules: Vec<Vec<String>>,
    },
}

impl std::fmt::Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleError::CircularDependency(cycle) => {
                write!(f, "चक्रव्यूह (Cakravyūha): Circular dependency detected: ")?;
                for (i, id) in cycle.iter().enumerate() {
                    if i > 0 {
                        write!(f, " → ")?;
                    }
                    write!(f, "{}", id.0)?;
                }
                Ok(())
            }
            ModuleError::NotFound(path) => {
                write!(f, "Module not found: {}", path.join("::"))
            }
            ModuleError::IoError(path, msg) => {
                write!(f, "IO error reading {}: {}", path.display(), msg)
            }
            ModuleError::ParseError(path, errors) => {
                write!(f, "Parse errors in {}:\n", path.display())?;
                for e in errors {
                    writeln!(f, "  - {}", e)?;
                }
                Ok(())
            }
            ModuleError::SymbolNotExported { module, symbol } => {
                write!(
                    f,
                    "Symbol '{}' not exported from module '{}'",
                    symbol,
                    module.join("::")
                )
            }
            ModuleError::VisibilityViolation {
                symbol,
                required,
                actual,
            } => {
                write!(
                    f,
                    "Visibility violation: '{}' requires {:?} but has {:?}",
                    symbol, required, actual
                )
            }
            ModuleError::AmbiguousImport { symbol, modules } => {
                write!(f, "Ambiguous import '{}' found in modules: ", symbol)?;
                for (i, m) in modules.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", m.join("::"))?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for ModuleError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_context_creation() {
        let ctx = ModuleContext::new(PathBuf::from("."));
        assert!(ctx.graph.modules.is_empty());
    }

    #[test]
    fn test_module_error_display() {
        let err = ModuleError::CircularDependency(vec![ModuleId(0), ModuleId(1), ModuleId(0)]);
        let msg = err.to_string();
        assert!(msg.contains("Cakravyūha"));
    }
}
