//! Visibility System (Pratyahara - प्रत्याहार)
//!
//! Sanskrit: प्रत्याहार (pratyāhāra) = withdrawal, abstraction
//! From Yoga Sutras - the fifth limb of Ashtanga Yoga
//!
//! Just as Pratyahara teaches withdrawal of senses to focus inward,
//! visibility in programming controls what is "seen" from outside.
//!
//! Visibility levels (based on Rust + Go + Sanskrit concepts):
//! - prakāśita (प्रकाशित) = public (illuminated, made visible)
//! - khaṇḍa-gata (खण्डगत) = crate/package-visible
//! - gupya (गुप्य) = private (hidden, protected)
//!
//! This implements encapsulation as a spiritual practice:
//! By hiding implementation details (Gupya), we allow others
//! to interact only with what we choose to reveal (Prakāśita).

use crate::lexer::Span;
use std::collections::HashSet;

/// Visibility level for symbols
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Visibility {
    /// prakāśita - Public to all (fully illuminated)
    Public,
    /// khaṇḍa-gata - Visible within crate/package
    Crate,
    /// mitra-gata - Visible to specified modules (friend access)
    Restricted,
    /// gupya - Private to current module (hidden)
    Private,
}

impl Visibility {
    /// Parse visibility from keyword
    pub fn from_keyword(keyword: &str) -> Option<Self> {
        match keyword {
            "pub" | "prakāśita" => Some(Visibility::Public),
            "pub(crate)" | "khaṇḍa-gata" => Some(Visibility::Crate),
            "pub(super)" | "mitra-gata" => Some(Visibility::Restricted),
            _ => None,
        }
    }

    /// Get Sanskrit name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Visibility::Public => "प्रकाशित (prakāśita)",
            Visibility::Crate => "खण्डगत (khaṇḍa-gata)",
            Visibility::Restricted => "मित्रगत (mitra-gata)",
            Visibility::Private => "गुप्य (gupya)",
        }
    }

    /// Check if this visibility allows access from given scope
    pub fn allows_access(
        &self,
        accessor_scope: &VisibilityScope,
        target_scope: &VisibilityScope,
    ) -> bool {
        match self {
            Visibility::Public => true,
            Visibility::Crate => accessor_scope.crate_name == target_scope.crate_name,
            Visibility::Restricted => {
                // Check if accessor is in allowed modules or is parent
                accessor_scope
                    .module_path
                    .starts_with(&target_scope.parent_module())
                    || target_scope
                        .allowed_modules
                        .contains(&accessor_scope.module_path)
            }
            Visibility::Private => accessor_scope.module_path == target_scope.module_path,
        }
    }

    /// Get the effective "reach" of this visibility
    pub fn reach(&self) -> VisibilityReach {
        match self {
            Visibility::Public => VisibilityReach::Universe,
            Visibility::Crate => VisibilityReach::Crate,
            Visibility::Restricted => VisibilityReach::Module,
            Visibility::Private => VisibilityReach::Local,
        }
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Private
    }
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Public => write!(f, "pub"),
            Visibility::Crate => write!(f, "pub(crate)"),
            Visibility::Restricted => write!(f, "pub(super)"),
            Visibility::Private => write!(f, "private"),
        }
    }
}

/// How far a visibility extends
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VisibilityReach {
    /// Only within current scope/block
    Local,
    /// Within current module
    Module,
    /// Within current crate/package
    Crate,
    /// Everywhere (no restrictions)
    Universe,
}

/// Scope context for visibility checking
#[derive(Debug, Clone)]
pub struct VisibilityScope {
    /// Current crate/package name
    pub crate_name: String,
    /// Current module path
    pub module_path: Vec<String>,
    /// Modules explicitly granted access (for restricted visibility)
    pub allowed_modules: HashSet<Vec<String>>,
}

impl VisibilityScope {
    /// Create a new scope
    pub fn new(crate_name: String, module_path: Vec<String>) -> Self {
        Self {
            crate_name,
            module_path,
            allowed_modules: HashSet::new(),
        }
    }

    /// Get parent module path
    pub fn parent_module(&self) -> Vec<String> {
        if self.module_path.len() <= 1 {
            Vec::new()
        } else {
            self.module_path[..self.module_path.len() - 1].to_vec()
        }
    }

    /// Check if this scope is a child of another
    pub fn is_child_of(&self, other: &VisibilityScope) -> bool {
        if self.crate_name != other.crate_name {
            return false;
        }
        if self.module_path.len() <= other.module_path.len() {
            return false;
        }
        self.module_path.starts_with(&other.module_path)
    }

    /// Create child scope
    pub fn child(&self, name: &str) -> Self {
        let mut path = self.module_path.clone();
        path.push(name.to_string());
        Self {
            crate_name: self.crate_name.clone(),
            module_path: path,
            allowed_modules: HashSet::new(),
        }
    }
}

/// Visibility checker for enforcing access rules
#[derive(Debug)]
pub struct VisibilityChecker {
    /// Current scope
    current_scope: VisibilityScope,
    /// Visibility violations found
    violations: Vec<VisibilityViolation>,
}

/// A visibility violation
#[derive(Debug, Clone)]
pub struct VisibilityViolation {
    /// Symbol being accessed
    pub symbol: String,
    /// Where access was attempted
    pub access_location: Span,
    /// Where symbol is defined
    pub definition_module: Vec<String>,
    /// Required visibility level
    pub required: Visibility,
    /// Actual visibility level
    pub actual: Visibility,
    /// Error message
    pub message: String,
}

impl VisibilityChecker {
    /// Create a new visibility checker
    pub fn new(scope: VisibilityScope) -> Self {
        Self {
            current_scope: scope,
            violations: Vec::new(),
        }
    }

    /// Check if access to a symbol is allowed
    pub fn check_access(
        &mut self,
        symbol_name: &str,
        symbol_visibility: Visibility,
        symbol_scope: &VisibilityScope,
        access_span: Span,
    ) -> bool {
        if symbol_visibility.allows_access(&self.current_scope, symbol_scope) {
            true
        } else {
            self.violations.push(VisibilityViolation {
                symbol: symbol_name.to_string(),
                access_location: access_span,
                definition_module: symbol_scope.module_path.clone(),
                required: Visibility::Public,
                actual: symbol_visibility,
                message: format!(
                    "Cannot access {} symbol '{}' from module '{}'",
                    symbol_visibility.sanskrit_name(),
                    symbol_name,
                    self.current_scope.module_path.join("::")
                ),
            });
            false
        }
    }

    /// Enter a child scope
    pub fn enter_scope(&mut self, name: &str) {
        self.current_scope = self.current_scope.child(name);
    }

    /// Exit to parent scope
    pub fn exit_scope(&mut self) {
        if !self.current_scope.module_path.is_empty() {
            self.current_scope.module_path.pop();
        }
    }

    /// Get all violations
    pub fn violations(&self) -> &[VisibilityViolation] {
        &self.violations
    }

    /// Clear violations
    pub fn clear_violations(&mut self) {
        self.violations.clear();
    }

    /// Check if there are any violations
    pub fn has_violations(&self) -> bool {
        !self.violations.is_empty()
    }
}

/// Visibility modifier for re-exports
#[derive(Debug, Clone)]
pub struct ReExport {
    /// Original symbol path
    pub original: Vec<String>,
    /// New visibility (must be equal or more restrictive)
    pub visibility: Visibility,
    /// Optional alias
    pub alias: Option<String>,
}

impl ReExport {
    /// Create a public re-export
    pub fn public(original: Vec<String>) -> Self {
        Self {
            original,
            visibility: Visibility::Public,
            alias: None,
        }
    }

    /// Create a re-export with alias
    pub fn with_alias(original: Vec<String>, alias: String) -> Self {
        Self {
            original,
            visibility: Visibility::Public,
            alias: Some(alias),
        }
    }
}

/// Extension trait for checking visibility rules
pub trait CheckVisibility {
    /// Get the visibility of this item
    fn visibility(&self) -> Visibility;

    /// Check if this item is accessible from the given scope
    fn is_accessible_from(&self, scope: &VisibilityScope) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility_from_keyword() {
        assert_eq!(Visibility::from_keyword("pub"), Some(Visibility::Public));
        assert_eq!(
            Visibility::from_keyword("prakāśita"),
            Some(Visibility::Public)
        );
        assert_eq!(
            Visibility::from_keyword("pub(crate)"),
            Some(Visibility::Crate)
        );
        assert_eq!(Visibility::from_keyword("invalid"), None);
    }

    #[test]
    fn test_public_access() {
        let vis = Visibility::Public;
        let scope1 = VisibilityScope::new("crate_a".to_string(), vec!["mod_a".to_string()]);
        let scope2 = VisibilityScope::new("crate_b".to_string(), vec!["mod_b".to_string()]);

        assert!(vis.allows_access(&scope2, &scope1));
    }

    #[test]
    fn test_crate_access() {
        let vis = Visibility::Crate;
        let scope1 = VisibilityScope::new("crate_a".to_string(), vec!["mod_a".to_string()]);
        let scope_same = VisibilityScope::new("crate_a".to_string(), vec!["mod_b".to_string()]);
        let scope_diff = VisibilityScope::new("crate_b".to_string(), vec!["mod_c".to_string()]);

        assert!(vis.allows_access(&scope_same, &scope1));
        assert!(!vis.allows_access(&scope_diff, &scope1));
    }

    #[test]
    fn test_private_access() {
        let vis = Visibility::Private;
        let scope = VisibilityScope::new("crate_a".to_string(), vec!["mod_a".to_string()]);
        let scope_same = VisibilityScope::new("crate_a".to_string(), vec!["mod_a".to_string()]);
        let scope_diff = VisibilityScope::new("crate_a".to_string(), vec!["mod_b".to_string()]);

        assert!(vis.allows_access(&scope_same, &scope));
        assert!(!vis.allows_access(&scope_diff, &scope));
    }

    #[test]
    fn test_visibility_checker() {
        let scope = VisibilityScope::new("test".to_string(), vec!["main".to_string()]);
        let mut checker = VisibilityChecker::new(scope);

        let symbol_scope = VisibilityScope::new("test".to_string(), vec!["utils".to_string()]);

        // Public should be accessible
        assert!(checker.check_access("pub_fn", Visibility::Public, &symbol_scope, Span::dummy()));

        // Private should not be accessible from different module
        assert!(!checker.check_access(
            "priv_fn",
            Visibility::Private,
            &symbol_scope,
            Span::dummy()
        ));

        assert_eq!(checker.violations.len(), 1);
    }

    #[test]
    fn test_scope_hierarchy() {
        let parent = VisibilityScope::new("test".to_string(), vec!["parent".to_string()]);
        let child = parent.child("child");

        assert_eq!(
            child.module_path,
            vec!["parent".to_string(), "child".to_string()]
        );
        assert!(child.is_child_of(&parent));
        assert!(!parent.is_child_of(&child));
    }

    #[test]
    fn test_visibility_reach_ordering() {
        assert!(VisibilityReach::Local < VisibilityReach::Module);
        assert!(VisibilityReach::Module < VisibilityReach::Crate);
        assert!(VisibilityReach::Crate < VisibilityReach::Universe);
    }
}
