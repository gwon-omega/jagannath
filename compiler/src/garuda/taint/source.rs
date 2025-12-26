//! # Taint Source
//!
//! Representation of where taint originates.

use super::TaintKind;
use crate::garuda::vaitarani::TaintLevel;
use crate::errors::Span;

/// A source of taint
#[derive(Debug, Clone)]
pub struct TaintSource {
    /// Name/identifier of the source
    pub name: String,
    /// What kind of taint
    pub kind: TaintKind,
    /// How tainted
    pub level: TaintLevel,
    /// Location in code
    pub location: Option<Span>,
}

impl TaintSource {
    /// Create a new taint source
    pub fn new(name: &str, kind: TaintKind, level: TaintLevel) -> Self {
        Self {
            name: name.to_string(),
            kind,
            level,
            location: None,
        }
    }

    /// With location
    pub fn with_location(mut self, span: Span) -> Self {
        self.location = Some(span);
        self
    }

    /// Get description
    pub fn describe(&self) -> String {
        format!(
            "Taint source '{}' ({}) with level {:?}",
            self.name,
            self.kind.sanskrit_name(),
            self.level
        )
    }
}
