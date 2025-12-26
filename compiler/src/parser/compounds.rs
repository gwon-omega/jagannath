//! Samāsa (Compound) Resolver
//!
//! Handles Sanskrit compound word resolution:
//! - Tatpuruṣa (A's B) → namespace::type
//! - Dvandva (A and B) → sum/product type
//! - Bahuvrīhi (having A) → type constraint
//! - Karmadhāraya (A which is B) → modifier chain

use crate::parser::ast::Identifier;

/// Types of Sanskrit compounds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamasaKind {
    /// Tatpuruṣa - Possessive compound (A's B)
    /// Example: dattakoṣa:saṃyoga:pūla → database::connection::pool
    Tatpurusha,

    /// Dvandva - Copulative compound (A and B)
    /// Example: phala-truṭi → Result<T, Error>
    Dvandva,

    /// Bahuvrīhi - Attributive compound (having A)
    /// Example: mahā-buddhi → having great intelligence (constraint)
    Bahuvrihi,

    /// Karmadhāraya - Descriptive compound (A which is B)
    /// Example: nīla-kamala → blue lotus (modified type)
    Karmadharaya,
}

/// Resolved namespace path
#[derive(Debug, Clone)]
pub struct NamespacePath {
    /// Path segments
    pub segments: Vec<String>,
    /// Kind of compound
    pub kind: SamasaKind,
}

/// Samāsa resolver
pub struct SamasaResolver {
    /// Separator characters for different compound types
    separators: Vec<char>,
}

impl SamasaResolver {
    pub fn new() -> Self {
        Self {
            separators: vec![':', '-', '_'],
        }
    }

    /// Resolve a compound word into a namespace path
    pub fn resolve(&self, compound: &str) -> NamespacePath {
        // Split by tatpuruṣa separator (:)
        let parts: Vec<_> = compound.split(':').collect();

        if parts.len() > 1 {
            return NamespacePath {
                segments: parts.iter().map(|s| s.to_string()).collect(),
                kind: SamasaKind::Tatpurusha,
            };
        }

        // Split by dvandva separator (-)
        let parts: Vec<_> = compound.split('-').collect();

        if parts.len() > 1 {
            // Check if it's dvandva (union) or karmadhāraya (modifier)
            // Heuristic: if parts are similar length, likely dvandva
            let kind = if parts.iter().all(|p| p.len() > 2) {
                SamasaKind::Dvandva
            } else {
                SamasaKind::Karmadharaya
            };

            return NamespacePath {
                segments: parts.iter().map(|s| s.to_string()).collect(),
                kind,
            };
        }

        // Single word, no compound
        NamespacePath {
            segments: vec![compound.to_string()],
            kind: SamasaKind::Tatpurusha,
        }
    }

    /// Check if identifier is a compound
    pub fn is_compound(&self, ident: &str) -> bool {
        self.separators.iter().any(|&sep| ident.contains(sep))
    }

    /// Get the head (final element) of a compound
    pub fn head<'a>(&self, compound: &'a str) -> &'a str {
        compound.rsplit(&['-', ':', '_']).next().unwrap_or(compound)
    }

    /// Get the modifier (non-final elements) of a compound
    pub fn modifier<'a>(&self, compound: &'a str) -> Option<&'a str> {
        let head_start = compound.rfind(&['-', ':', '_'])?;
        Some(&compound[..head_start])
    }
}

impl Default for SamasaResolver {
    fn default() -> Self {
        Self::new()
    }
}
