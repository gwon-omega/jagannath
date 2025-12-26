//! # Taint Propagation
//!
//! Rules for how taint spreads through operations.

use super::TaintKind;
use crate::garuda::vaitarani::TaintLevel;

/// Taint propagation rules
pub struct TaintPropagation;

impl TaintPropagation {
    /// Combine taint levels (returns worse of two)
    pub fn combine_levels(a: TaintLevel, b: TaintLevel) -> TaintLevel {
        std::cmp::max(a, b)
    }

    /// Combine taint kinds
    pub fn combine_kinds(a: TaintKind, b: TaintKind) -> TaintKind {
        // Prioritize more dangerous taints
        match (a, b) {
            (TaintKind::CommandInjection, _) | (_, TaintKind::CommandInjection) => TaintKind::CommandInjection,
            (TaintKind::SqlInjection, _) | (_, TaintKind::SqlInjection) => TaintKind::SqlInjection,
            (TaintKind::CrossSiteScripting, _) | (_, TaintKind::CrossSiteScripting) => TaintKind::CrossSiteScripting,
            (TaintKind::PathTraversal, _) | (_, TaintKind::PathTraversal) => TaintKind::PathTraversal,
            (TaintKind::SensitiveData, _) | (_, TaintKind::SensitiveData) => TaintKind::SensitiveData,
            (TaintKind::UserInput, _) | (_, TaintKind::UserInput) => TaintKind::UserInput,
            _ => TaintKind::Unchecked,
        }
    }

    /// Check if operation sanitizes taint
    pub fn is_sanitizer(operation: &str, taint_kind: TaintKind) -> bool {
        match (operation, taint_kind) {
            ("sql_escape" | "śuddhi-kri-sql", TaintKind::SqlInjection) => true,
            ("html_escape" | "śuddhi-kri-html", TaintKind::CrossSiteScripting) => true,
            ("shell_escape" | "śuddhi-kri-shell", TaintKind::CommandInjection) => true,
            ("path_validate" | "śuddhi-kri-mārga", TaintKind::PathTraversal) => true,
            _ => false,
        }
    }
}
