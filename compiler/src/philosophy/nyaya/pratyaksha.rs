//! Pratyakṣa - Direct Perception (प्रत्यक्ष)
//!
//! The first and most certain pramāṇa: explicit type annotations.
//! "What you see is what you get" - 100% certainty.
//!
//! In Jagannath, pratyakṣa corresponds to:
//! - Explicit type annotations: `saṅkhyā-k: t32 = 42;`
//! - Generic type parameters: `<T: Prakāra>`
//! - Return type declarations: `-> t32-k`

use super::{Pramana, TypeEvidence};
use crate::parser::ast::{Type, TypeAnnotation};
use std::collections::HashMap;

/// Pratyakṣa type resolver - handles explicit annotations
pub struct PratyakshaResolver {
    /// Direct type bindings (name → type)
    bindings: HashMap<String, ExplicitType>,
    /// Generic parameters in scope
    generics: HashMap<String, GenericBound>,
}

/// An explicitly declared type
#[derive(Debug, Clone)]
pub struct ExplicitType {
    /// The type name
    pub type_name: String,
    /// Source location
    pub location: SourceSpan,
    /// Affix modifiers (-k, -l, -b, etc.)
    pub affixes: Vec<String>,
    /// Lifetime region (^1, ^2, etc.)
    pub lifetime: Option<u32>,
}

/// Source span for error reporting
#[derive(Debug, Clone, Copy)]
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
    pub line: u32,
}

/// Generic type bound
#[derive(Debug, Clone)]
pub struct GenericBound {
    /// Generic parameter name (T, K, V, etc.)
    pub name: String,
    /// Trait bounds
    pub bounds: Vec<String>,
    /// Default type (if any)
    pub default: Option<String>,
}

impl PratyakshaResolver {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            generics: HashMap::new(),
        }
    }

    /// Record an explicit type annotation (pratyakṣa)
    /// This is the highest certainty type assignment
    pub fn record(&mut self, name: String, explicit_type: ExplicitType) {
        self.bindings.insert(name, explicit_type);
    }

    /// Record a generic parameter
    pub fn record_generic(&mut self, name: String, bound: GenericBound) {
        self.generics.insert(name, bound);
    }

    /// Look up a pratyakṣa type
    pub fn lookup(&self, name: &str) -> Option<TypeEvidence> {
        self.bindings.get(name).map(|explicit| TypeEvidence {
            type_name: explicit.type_name.clone(),
            pramana: Pramana::Pratyaksha,
            certainty: 1.0,
            evidence: vec![
                format!("प्रत्यक्ष (direct perception): Explicit annotation"),
                format!(
                    "Type '{}' declared at line {}",
                    explicit.type_name, explicit.location.line
                ),
            ],
        })
    }

    /// Check if a generic parameter exists
    pub fn has_generic(&self, name: &str) -> bool {
        self.generics.contains_key(name)
    }

    /// Get generic bound
    pub fn get_generic(&self, name: &str) -> Option<&GenericBound> {
        self.generics.get(name)
    }

    /// Parse type annotation from syntax
    /// Handles: `saṅkhyā-k: t32 = 42;`
    pub fn parse_annotation(&self, annotation: &str) -> Option<ExplicitType> {
        // Format: name-affixes: type
        // Example: x-k: t32
        let parts: Vec<&str> = annotation.split(':').collect();
        if parts.len() != 2 {
            return None;
        }

        let name_part = parts[0].trim();
        let type_part = parts[1].trim();

        // Extract affixes from name
        let name_affixes: Vec<&str> = name_part.split('-').collect();
        let affixes: Vec<String> = name_affixes[1..].iter().map(|s| s.to_string()).collect();

        Some(ExplicitType {
            type_name: type_part.to_string(),
            location: SourceSpan {
                start: 0,
                end: 0,
                line: 0,
            },
            affixes,
            lifetime: self.extract_lifetime(type_part),
        })
    }

    /// Extract lifetime region from type string
    fn extract_lifetime(&self, type_str: &str) -> Option<u32> {
        // Look for ^N pattern
        if let Some(idx) = type_str.find('^') {
            let rest = &type_str[idx + 1..];
            let num_str: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            return num_str.parse().ok();
        }
        None
    }

    /// Validate that explicit types are consistent
    pub fn validate_consistency(&self) -> Vec<String> {
        let mut errors = Vec::new();

        for (name, explicit) in &self.bindings {
            // Check lifetime affixes match declarations
            if explicit.affixes.contains(&"l".to_string()) && explicit.lifetime.is_none() {
                errors.push(format!(
                    "Type '{}' has linear affix (-l) but no lifetime region (^N)",
                    name
                ));
            }

            // Check for conflicting affixes
            if explicit.affixes.contains(&"l".to_string())
                && explicit.affixes.contains(&"b".to_string())
            {
                errors.push(format!(
                    "Type '{}' cannot be both linear (-l) and borrowed (-b)",
                    name
                ));
            }
        }

        errors
    }

    /// Enter a new scope (for generics)
    pub fn push_scope(&mut self) {
        // TODO: Implement scope stack for nested generics
    }

    /// Exit current scope
    pub fn pop_scope(&mut self) {
        // TODO: Implement scope stack for nested generics
    }
}

impl Default for PratyakshaResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pratyaksha_lookup() {
        let mut resolver = PratyakshaResolver::new();

        resolver.record(
            "x".to_string(),
            ExplicitType {
                type_name: "t32".to_string(),
                location: SourceSpan {
                    start: 0,
                    end: 0,
                    line: 1,
                },
                affixes: vec!["k".to_string()],
                lifetime: None,
            },
        );

        let evidence = resolver.lookup("x").unwrap();
        assert_eq!(evidence.type_name, "t32");
        assert_eq!(evidence.pramana, Pramana::Pratyaksha);
        assert_eq!(evidence.certainty, 1.0);
    }

    #[test]
    fn test_conflicting_affixes() {
        let mut resolver = PratyakshaResolver::new();

        resolver.record(
            "bad".to_string(),
            ExplicitType {
                type_name: "Bufara".to_string(),
                location: SourceSpan {
                    start: 0,
                    end: 0,
                    line: 1,
                },
                affixes: vec!["l".to_string(), "b".to_string()],
                lifetime: None,
            },
        );

        let errors = resolver.validate_consistency();
        assert!(!errors.is_empty());
        assert!(errors[0].contains("cannot be both linear"));
    }
}
