//! # Fix Suggestion
//!
//! Concrete code fix suggestions.

use crate::errors::Span;

/// A concrete fix suggestion
#[derive(Debug, Clone)]
pub struct FixSuggestion {
    /// Location to apply fix
    pub location: Span,
    /// What to replace (if any)
    pub old_code: Option<String>,
    /// New code to insert
    pub new_code: String,
    /// Description of the fix
    pub description: String,
    /// Is this an auto-fix that can be applied automatically?
    pub auto_fixable: bool,
}

impl FixSuggestion {
    /// Create a replacement fix
    pub fn replace(location: Span, old: &str, new: &str, description: &str) -> Self {
        Self {
            location,
            old_code: Some(old.to_string()),
            new_code: new.to_string(),
            description: description.to_string(),
            auto_fixable: true,
        }
    }

    /// Create an insertion fix
    pub fn insert(location: Span, code: &str, description: &str) -> Self {
        Self {
            location,
            old_code: None,
            new_code: code.to_string(),
            description: description.to_string(),
            auto_fixable: true,
        }
    }

    /// Create a manual fix suggestion
    pub fn manual(location: Span, suggestion: &str, description: &str) -> Self {
        Self {
            location,
            old_code: None,
            new_code: suggestion.to_string(),
            description: description.to_string(),
            auto_fixable: false,
        }
    }

    /// Format as display string
    pub fn format(&self) -> String {
        let prefix = if self.auto_fixable { "Auto-fix" } else { "Suggested fix" };

        if let Some(old) = &self.old_code {
            format!(
                "{}: {}\n  Replace: `{}`\n  With: `{}`",
                prefix, self.description, old, self.new_code
            )
        } else {
            format!(
                "{}: {}\n  Insert: `{}`",
                prefix, self.description, self.new_code
            )
        }
    }
}

/// Fix applicator
pub struct FixApplicator;

impl FixApplicator {
    /// Apply fixes to source code
    pub fn apply(source: &str, fixes: &[FixSuggestion]) -> String {
        let mut result = source.to_string();

        // Sort fixes by location in reverse order (apply from end to start)
        let mut sorted_fixes: Vec<_> = fixes.iter()
            .filter(|f| f.auto_fixable)
            .collect();
        sorted_fixes.sort_by(|a, b| b.location.start.cmp(&a.location.start));

        for fix in sorted_fixes {
            if let Some(old) = &fix.old_code {
                // Find and replace
                if let Some(pos) = result.find(old) {
                    result = format!(
                        "{}{}{}",
                        &result[..pos],
                        fix.new_code,
                        &result[pos + old.len()..]
                    );
                }
            }
        }

        result
    }
}
