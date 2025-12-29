//! Module Path Resolution (Mārgadarśana - मार्गदर्शन)
//!
//! Sanskrit: मार्ग (mārga) = path, दर्शन (darśana) = showing/finding
//!
//! Resolves import paths to file system paths following:
//! - Rust-style module resolution (mod.rs or module_name.rs)
//! - Go-style package paths
//! - Standard library vs local module distinction
//!
//! Path resolution order:
//! 1. Standard library (stdlib::*)
//! 2. Local modules (relative to current file)
//! 3. Project root modules
//! 4. External packages (future: kosha packages)

use super::ModuleError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Module path resolver
#[derive(Debug)]
pub struct ModuleResolver {
    /// Cached resolved paths
    cache: HashMap<String, PathBuf>,
    /// Search paths for modules
    search_paths: Vec<PathBuf>,
}

/// Resolution error types
#[derive(Debug, Clone)]
pub enum ResolveError {
    /// Module not found in any search path
    NotFound {
        path: Vec<String>,
        searched: Vec<PathBuf>,
    },
    /// Ambiguous module - found in multiple locations
    Ambiguous {
        path: Vec<String>,
        locations: Vec<PathBuf>,
    },
    /// Invalid module path syntax
    InvalidPath(String),
}

impl std::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolveError::NotFound { path, searched } => {
                write!(f, "Module '{}' not found. Searched:\n", path.join("::"))?;
                for p in searched {
                    writeln!(f, "  - {}", p.display())?;
                }
                Ok(())
            }
            ResolveError::Ambiguous { path, locations } => {
                write!(f, "Ambiguous module '{}' found in:\n", path.join("::"))?;
                for p in locations {
                    writeln!(f, "  - {}", p.display())?;
                }
                Ok(())
            }
            ResolveError::InvalidPath(msg) => {
                write!(f, "Invalid module path: {}", msg)
            }
        }
    }
}

impl std::error::Error for ResolveError {}

impl ModuleResolver {
    /// Create a new resolver
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            search_paths: Vec::new(),
        }
    }

    /// Add a search path
    pub fn add_search_path(&mut self, path: PathBuf) {
        if !self.search_paths.contains(&path) {
            self.search_paths.push(path);
        }
    }

    /// Resolve a module path to file system path
    ///
    /// Resolution algorithm:
    /// 1. Check cache
    /// 2. If starts with "stdlib", resolve to stdlib path
    /// 3. Check relative to project root
    /// 4. Check search paths
    pub fn resolve_path(
        &mut self,
        module_path: &[String],
        project_root: &Path,
        stdlib_path: Option<&Path>,
    ) -> Result<PathBuf, ModuleError> {
        let cache_key = module_path.join("::");

        // Check cache
        if let Some(cached) = self.cache.get(&cache_key) {
            return Ok(cached.clone());
        }

        // Standard library resolution
        if module_path.first().map(|s| s.as_str()) == Some("stdlib") {
            if let Some(stdlib) = stdlib_path {
                let resolved = self.resolve_stdlib_path(&module_path[1..], stdlib)?;
                self.cache.insert(cache_key, resolved.clone());
                return Ok(resolved);
            } else {
                return Err(ModuleError::NotFound(module_path.to_vec()));
            }
        }

        // Local resolution
        let resolved = self.resolve_local_path(module_path, project_root)?;
        self.cache.insert(cache_key, resolved.clone());
        Ok(resolved)
    }

    /// Resolve stdlib path
    fn resolve_stdlib_path(
        &self,
        path: &[String],
        stdlib_root: &Path,
    ) -> Result<PathBuf, ModuleError> {
        let mut base = stdlib_root.join("src");

        for component in path.iter().take(path.len().saturating_sub(1)) {
            base = base.join(component);
        }

        // Try module_name.rs first
        if let Some(last) = path.last() {
            let file_path = base.join(format!("{}.rs", last));
            if file_path.exists() {
                // Note: stdlib is Rust, not .jag, but we'll handle both
                return Ok(file_path);
            }

            // Try module_name/mod.rs
            let mod_path = base.join(last).join("mod.rs");
            if mod_path.exists() {
                return Ok(mod_path);
            }

            // Try as Jagannath file
            let jag_path = base.join(format!("{}.jag", last));
            if jag_path.exists() {
                return Ok(jag_path);
            }
        }

        Err(ModuleError::NotFound(path.to_vec()))
    }

    /// Resolve local module path (relative to project)
    fn resolve_local_path(
        &self,
        path: &[String],
        project_root: &Path,
    ) -> Result<PathBuf, ModuleError> {
        let mut candidates: Vec<PathBuf> = Vec::new();
        let mut searched: Vec<PathBuf> = Vec::new();

        // Build base path from module path components
        let build_path = |root: &Path| -> Vec<PathBuf> {
            let mut results = Vec::new();
            let mut base = root.to_path_buf();

            for component in path.iter().take(path.len().saturating_sub(1)) {
                base = base.join(component);
            }

            if let Some(last) = path.last() {
                // Try: base/name.jag
                let file_path = base.join(format!("{}.jag", last));
                results.push(file_path);

                // Try: base/name/mod.jag
                let mod_path = base.join(last).join("mod.jag");
                results.push(mod_path);

                // Try: base/name/name.jag (Go-style)
                let go_style = base.join(last).join(format!("{}.jag", last));
                results.push(go_style);
            }

            results
        };

        // Check project root
        let src_root = project_root.join("src");
        let roots = if src_root.exists() {
            vec![src_root, project_root.to_path_buf()]
        } else {
            vec![project_root.to_path_buf()]
        };

        for root in &roots {
            let paths = build_path(root);
            for p in paths {
                searched.push(p.clone());
                if p.exists() {
                    candidates.push(p);
                }
            }
        }

        // Check additional search paths
        for search_path in &self.search_paths {
            let paths = build_path(search_path);
            for p in paths {
                searched.push(p.clone());
                if p.exists() {
                    candidates.push(p);
                }
            }
        }

        // Deduplicate candidates
        candidates.sort();
        candidates.dedup();

        match candidates.len() {
            0 => Err(ModuleError::NotFound(path.to_vec())),
            1 => Ok(candidates.remove(0)),
            _ => Err(ModuleError::NotFound(path.to_vec())), // For now, take first
        }
    }

    /// Resolve path relative to importing module
    pub fn resolve_relative(
        &self,
        path: &[String],
        from_file: &Path,
    ) -> Result<PathBuf, ModuleError> {
        let parent = from_file.parent().unwrap_or(Path::new("."));

        // Handle special prefixes
        if path.first().map(|s| s.as_str()) == Some("super") {
            // Go up one directory
            let grandparent = parent.parent().unwrap_or(parent);
            return self.try_resolve_in_dir(&path[1..], grandparent);
        }

        if path.first().map(|s| s.as_str()) == Some("self") {
            // Same directory
            return self.try_resolve_in_dir(&path[1..], parent);
        }

        // Try relative to current file
        self.try_resolve_in_dir(path, parent)
    }

    /// Try to resolve module in a specific directory
    fn try_resolve_in_dir(&self, path: &[String], dir: &Path) -> Result<PathBuf, ModuleError> {
        let mut base = dir.to_path_buf();

        for component in path.iter().take(path.len().saturating_sub(1)) {
            base = base.join(component);
        }

        if let Some(last) = path.last() {
            let file_path = base.join(format!("{}.jag", last));
            if file_path.exists() {
                return Ok(file_path);
            }

            let mod_path = base.join(last).join("mod.jag");
            if mod_path.exists() {
                return Ok(mod_path);
            }
        }

        Err(ModuleError::NotFound(path.to_vec()))
    }

    /// Check if a path looks like a standard library import
    pub fn is_stdlib_import(path: &[String]) -> bool {
        matches!(
            path.first().map(|s| s.as_str()),
            Some("stdlib" | "std" | "core" | "kosha")
        )
    }

    /// Parse an import path string into components
    pub fn parse_import_path(path_str: &str) -> Result<Vec<String>, ResolveError> {
        if path_str.is_empty() {
            return Err(ResolveError::InvalidPath("Empty path".to_string()));
        }

        let components: Vec<String> = path_str
            .split("::")
            .map(|s| s.trim().to_string())
            .collect();

        for component in &components {
            if component.is_empty() {
                return Err(ResolveError::InvalidPath(format!(
                    "Empty component in path: {}", path_str
                )));
            }
            if !is_valid_identifier(component) {
                return Err(ResolveError::InvalidPath(format!(
                    "Invalid identifier '{}' in path: {}", component, path_str
                )));
            }
        }

        Ok(components)
    }

    /// Clear the resolution cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for ModuleResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if a string is a valid Jagannath identifier
fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars();

    // First character must be alphabetic or underscore
    match chars.next() {
        Some(c) if c.is_alphabetic() || c == '_' => {}
        _ => return false,
    }

    // Rest can be alphanumeric, underscore, or Sanskrit characters
    for c in chars {
        if !c.is_alphanumeric() && c != '_' {
            // Allow Unicode letters (for Sanskrit)
            if !c.is_alphabetic() {
                return false;
            }
        }
    }

    true
}

/// Module path manipulation utilities
pub mod path_utils {
    /// Join path components with ::
    pub fn join(components: &[String]) -> String {
        components.join("::")
    }

    /// Get the parent path (all but last component)
    pub fn parent(path: &[String]) -> Vec<String> {
        if path.len() <= 1 {
            Vec::new()
        } else {
            path[..path.len() - 1].to_vec()
        }
    }

    /// Get the last component (module name)
    pub fn name(path: &[String]) -> Option<&String> {
        path.last()
    }

    /// Check if path A is a prefix of path B
    pub fn is_prefix(a: &[String], b: &[String]) -> bool {
        if a.len() > b.len() {
            return false;
        }
        a.iter().zip(b.iter()).all(|(x, y)| x == y)
    }

    /// Make path relative to a base path
    pub fn relative_to(path: &[String], base: &[String]) -> Option<Vec<String>> {
        if is_prefix(base, path) {
            Some(path[base.len()..].to_vec())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_parse_import_path() {
        let path = ModuleResolver::parse_import_path("stdlib::sankhya::prakara").unwrap();
        assert_eq!(path, vec!["stdlib", "sankhya", "prakara"]);
    }

    #[test]
    fn test_invalid_path() {
        let result = ModuleResolver::parse_import_path("");
        assert!(result.is_err());

        let result = ModuleResolver::parse_import_path("foo::::bar");
        assert!(result.is_err());
    }

    #[test]
    fn test_is_stdlib_import() {
        assert!(ModuleResolver::is_stdlib_import(&["stdlib".to_string(), "io".to_string()]));
        assert!(ModuleResolver::is_stdlib_import(&["std".to_string(), "io".to_string()]));
        assert!(!ModuleResolver::is_stdlib_import(&["mymodule".to_string()]));
    }

    #[test]
    fn test_valid_identifier() {
        assert!(is_valid_identifier("foo"));
        assert!(is_valid_identifier("_bar"));
        assert!(is_valid_identifier("foo_bar"));
        assert!(is_valid_identifier("foo123"));
        assert!(is_valid_identifier("saṅkhyā")); // Sanskrit
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("123foo"));
    }

    #[test]
    fn test_path_utils() {
        use path_utils::*;

        let path = vec!["a".to_string(), "b".to_string(), "c".to_string()];

        assert_eq!(join(&path), "a::b::c");
        assert_eq!(parent(&path), vec!["a".to_string(), "b".to_string()]);
        assert_eq!(name(&path), Some(&"c".to_string()));
    }

    #[test]
    fn test_resolve_local() {
        // Create temp directory structure
        let temp = TempDir::new().unwrap();
        let src = temp.path().join("src");
        fs::create_dir_all(&src).unwrap();
        fs::write(src.join("main.jag"), "// main").unwrap();
        fs::create_dir_all(src.join("utils")).unwrap();
        fs::write(src.join("utils").join("mod.jag"), "// utils").unwrap();

        let mut resolver = ModuleResolver::new();

        // Resolve main.jag
        let path = vec!["main".to_string()];
        let result = resolver.resolve_path(&path, temp.path(), None);
        assert!(result.is_ok());

        // Resolve utils/mod.jag
        let path = vec!["utils".to_string()];
        let result = resolver.resolve_path(&path, temp.path(), None);
        assert!(result.is_ok());
    }
}
