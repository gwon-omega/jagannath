//! Sampūrṇa-Doṣa-Varṇana: Rich Error Reporting System
//!
//! # Philosophy: Doṣa-Nivāraṇa (Error Removal)
//!
//! In Sanskrit grammar, Pāṇini's Aṣṭādhyāyī provides detailed explanations
//! for every rule violation. Similarly, our error messages provide:
//!
//! 1. **Doṣa** (दोष) - The fault itself
//! 2. **Kāraṇa** (कारण) - The cause/reason
//! 3. **Nivāraṇa** (निवारण) - The remedy/fix
//!
//! ## Sanskrit Terminology
//!
//! - doṣa (दोष) = fault/error
//! - varṇana (वर्णन) = description/explanation
//! - kāraṇa (कारण) = cause/reason
//! - nivāraṇa (निवारण) = removal/fix
//! - upadeśa (उपदेश) = teaching/suggestion
//! - udāharaṇa (उदाहरण) = example
//!
//! ## Error Message Style
//!
//! Each error message follows this structure:
//! 1. English description (for clarity)
//! 2. Sanskrit name (for authenticity)
//! 3. Visual code snippet with annotations
//! 4. Suggested fixes with before/after

use std::fmt;

use super::error::{CompilerError, ErrorCode, Severity};
use super::span::{SourceCache, SourceId, Span};

// ============================================================================
// PART 1: SANSKRIT ERROR NAMES (Doṣa-Nāma)
// ============================================================================

/// Sanskrit names and philosophical meanings for errors
#[derive(Debug, Clone)]
pub struct SanskritError {
    /// Sanskrit name
    pub name: &'static str,
    /// Sanskrit in Devanagari
    pub devanagari: &'static str,
    /// Philosophical meaning
    pub meaning: &'static str,
    /// Category (Garuda Purana Naraka, if applicable)
    pub naraka: Option<&'static str>,
}

/// Get Sanskrit error information for an error code
pub fn sanskrit_error(code: ErrorCode) -> SanskritError {
    match code.0 {
        // Lexical errors
        1 => SanskritError {
            name: "apaṭha",
            devanagari: "अपठ",
            meaning: "unreadable/invalid token",
            naraka: None,
        },
        2 => SanskritError {
            name: "anavadhāna-sūtra",
            devanagari: "अनवधान-सूत्र",
            meaning: "unterminated string (endless thread)",
            naraka: None,
        },

        // Type errors
        200 => SanskritError {
            name: "prakāra-virodha",
            devanagari: "प्रकार-विरोध",
            meaning: "type conflict",
            naraka: Some("Kalasutra (कालसूत्र)"),
        },
        201 => SanskritError {
            name: "ajñāta-prakāra",
            devanagari: "अज्ञात-प्रकार",
            meaning: "unknown type",
            naraka: None,
        },
        202 => SanskritError {
            name: "ajñāta-nāman",
            devanagari: "अज्ञात-नामन्",
            meaning: "unknown name/variable",
            naraka: None,
        },

        // Borrow checking errors
        300 => SanskritError {
            name: "mṛta-upayoga",
            devanagari: "मृत-उपयोग",
            meaning: "use after death (use-after-move)",
            naraka: Some("Tamisram (तमिस्रम्)"),
        },
        301 => SanskritError {
            name: "dvitīya-graha",
            devanagari: "द्वितीय-ग्रह",
            meaning: "double grasp (double borrow)",
            naraka: Some("Andhatamisram (अन्धतमिस्रम्)"),
        },
        302 => SanskritError {
            name: "graha-saṅgharṣa",
            devanagari: "ग्रह-संघर्ष",
            meaning: "borrow conflict",
            naraka: Some("Maharaurava (महारौरव)"),
        },
        303 => SanskritError {
            name: "calita-mūlya",
            devanagari: "चलित-मूल्य",
            meaning: "moved value",
            naraka: Some("Tamisram (तमिस्रम्)"),
        },

        // Lifetime errors
        500 => SanskritError {
            name: "āyus-virodha",
            devanagari: "आयुस्-विरोध",
            meaning: "lifetime conflict",
            naraka: Some("Vaitarani (वैतरणी)"),
        },
        501 => SanskritError {
            name: "āyus-palāyana",
            devanagari: "आयुस्-पलायन",
            meaning: "lifetime escape",
            naraka: Some("Vaitarani (वैतरणी)"),
        },
        502 => SanskritError {
            name: "lambamāna-nirdeśa",
            devanagari: "लम्बमान-निर्देश",
            meaning: "dangling reference (hanging pointer)",
            naraka: Some("Andhakupa (अन्धकूप)"),
        },

        // Default
        _ => SanskritError {
            name: "doṣa",
            devanagari: "दोष",
            meaning: "fault/error",
            naraka: None,
        },
    }
}

// ============================================================================
// PART 2: RICH ERROR FORMATTER (Doṣa-Prakāśaka)
// ============================================================================

/// Configuration for error formatting
#[derive(Debug, Clone)]
pub struct FormatConfig {
    /// Use colors in output
    pub colors: bool,
    /// Show Sanskrit names
    pub sanskrit: bool,
    /// Show Naraka classification
    pub naraka: bool,
    /// Show fix suggestions
    pub suggestions: bool,
    /// Maximum context lines to show
    pub context_lines: usize,
    /// Use Unicode box-drawing characters
    pub unicode_boxes: bool,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            colors: true,
            sanskrit: true,
            naraka: true,
            suggestions: true,
            context_lines: 3,
            unicode_boxes: true,
        }
    }
}

/// Rich error formatter
pub struct RichFormatter {
    config: FormatConfig,
    source_cache: SourceCache,
}

impl RichFormatter {
    pub fn new(config: FormatConfig) -> Self {
        Self {
            config,
            source_cache: SourceCache::new(),
        }
    }

    /// Cache source code for a file
    pub fn add_source(&mut self, filename: String, source: String) -> SourceId {
        self.source_cache.add(filename, source)
    }

    /// Format an error into a rich string
    pub fn format(&self, error: &CompilerError) -> String {
        let mut output = String::new();

        // Header with error code and Sanskrit name
        let sanskrit = sanskrit_error(error.code);

        // Severity color/symbol
        let (symbol, color_start, color_end) = if self.config.colors {
            match error.severity {
                Severity::Error => ("✗", "\x1b[31m", "\x1b[0m"), // Red
                Severity::Warning => ("⚠", "\x1b[33m", "\x1b[0m"), // Yellow
                Severity::Hint => ("→", "\x1b[34m", "\x1b[0m"),  // Blue
                Severity::Fatal => ("☠", "\x1b[35m", "\x1b[0m"), // Magenta
            }
        } else {
            match error.severity {
                Severity::Error => ("error:", "", ""),
                Severity::Warning => ("warning:", "", ""),
                Severity::Hint => ("hint:", "", ""),
                Severity::Fatal => ("fatal:", "", ""),
            }
        };

        // Header line
        output.push_str(&format!(
            "{}{} {}{}: {}\n",
            color_start, symbol, error.code, color_end, error.message
        ));

        // Sanskrit name (if enabled)
        if self.config.sanskrit {
            output.push_str(&format!(
                "   {} ({}) - {}\n",
                sanskrit.devanagari, sanskrit.name, sanskrit.meaning
            ));
        }

        // Naraka classification (if enabled and applicable)
        if self.config.naraka {
            if let Some(naraka) = sanskrit.naraka {
                output.push_str(&format!("   ⚰ Naraka: {}\n", naraka));
            }
        }

        // Location from source cache
        let location = if let Some(loc) = self.source_cache.location(&error.span) {
            if let Some(sm) = self.source_cache.get(error.span.source) {
                format!("{}:{}:{}", sm.path, loc.line, loc.column)
            } else {
                format!("{}..{}", error.span.start, error.span.end)
            }
        } else {
            format!("{}..{}", error.span.start, error.span.end)
        };

        output.push_str(&format!("   --> {}\n", location));

        // Source snippet with annotations
        if let Some(source_map) = self.source_cache.get(error.span.source) {
            output.push_str(&self.format_snippet(source_map, &error.span));
        }

        // Help text
        if let Some(ref help) = error.help {
            output.push_str(&format!(
                "   {}help{}: {}\n",
                if self.config.colors { "\x1b[32m" } else { "" },
                if self.config.colors { "\x1b[0m" } else { "" },
                help
            ));
        }

        // Note text
        if let Some(ref note) = error.note {
            output.push_str(&format!(
                "   {}note{}: {}\n",
                if self.config.colors { "\x1b[36m" } else { "" },
                if self.config.colors { "\x1b[0m" } else { "" },
                note
            ));
        }

        output
    }

    /// Format a code snippet with annotations
    fn format_snippet(&self, source_map: &super::span::SourceMap, span: &Span) -> String {
        let mut output = String::new();

        let loc = source_map.location(span.source, span.start);
        let end_loc = source_map.location(span.source, span.end);

        if loc.line == 0 {
            return output;
        }

        let (pipe, _corner, _dash) = if self.config.unicode_boxes {
            ("│", "╰", "─")
        } else {
            ("|", "`", "-")
        };

        // Calculate line number width
        let line_width = format!("{}", loc.line + self.config.context_lines as u32).len();

        // Show context before
        let start_line = loc.line.saturating_sub(self.config.context_lines as u32);
        let end_line =
            (end_loc.line + self.config.context_lines as u32).min(source_map.lines.len() as u32);

        output.push_str(&format!("   {:>width$} {}\n", "", pipe, width = line_width));

        for line_num in start_line..=end_line {
            if line_num == 0 {
                continue;
            }

            let line = source_map.line(line_num);

            output.push_str(&format!(
                "   {:>width$} {} {}\n",
                line_num,
                pipe,
                line,
                width = line_width
            ));

            // Underline the error span on the main line
            if line_num == loc.line {
                let spaces = " ".repeat((loc.column.saturating_sub(1)) as usize);
                let underline_len = if span.end > span.start {
                    (span.end - span.start) as usize
                } else {
                    1
                }
                .min(line.len());
                let underline = "^".repeat(underline_len.max(1));

                output.push_str(&format!(
                    "   {:>width$} {} {}{}{}",
                    "",
                    pipe,
                    spaces,
                    if self.config.colors { "\x1b[31m" } else { "" },
                    underline,
                    width = line_width
                ));
                output.push_str(&format!(
                    "{}\n",
                    if self.config.colors { "\x1b[0m" } else { "" }
                ));
            }
        }

        output.push_str(&format!("   {:>width$} {}\n", "", pipe, width = line_width));

        output
    }

    /// Format multiple errors
    pub fn format_all(&self, errors: &[CompilerError]) -> String {
        let mut output = String::new();

        for error in errors {
            output.push_str(&self.format(error));
            output.push('\n');
        }

        // Summary
        let error_count = errors
            .iter()
            .filter(|e| e.severity >= Severity::Error)
            .count();
        let warning_count = errors
            .iter()
            .filter(|e| e.severity == Severity::Warning)
            .count();

        if error_count > 0 || warning_count > 0 {
            output.push_str(&format!(
                "{}{}{}",
                if self.config.colors { "\x1b[1m" } else { "" },
                format!(
                    "संकलन विफल (compilation failed): {} error(s), {} warning(s)",
                    error_count, warning_count
                ),
                if self.config.colors { "\x1b[0m" } else { "" }
            ));
        }

        output
    }
}

impl Default for RichFormatter {
    fn default() -> Self {
        Self::new(FormatConfig::default())
    }
}

// ============================================================================
// PART 3: FIX SUGGESTIONS (Upadeśa)
// ============================================================================

/// Suggested fix for an error
#[derive(Debug, Clone)]
pub struct FixSuggestion {
    /// Short description
    pub title: String,
    /// Detailed explanation
    pub explanation: String,
    /// Code changes to apply
    pub changes: Vec<CodeChange>,
    /// Confidence level (0.0 - 1.0)
    pub confidence: f32,
}

/// A single code change
#[derive(Debug, Clone)]
pub struct CodeChange {
    /// Span to replace
    pub span: Span,
    /// New text
    pub replacement: String,
}

/// Generate fix suggestions for an error
pub fn suggest_fix(error: &CompilerError) -> Vec<FixSuggestion> {
    let mut suggestions = Vec::new();

    match error.code.0 {
        // Use after move - suggest clone
        300 => {
            suggestions.push(FixSuggestion {
                title: "Clone the value before moving".to_string(),
                explanation: "The value was moved and cannot be used again. \
                              Consider cloning it if the type implements Clone."
                    .to_string(),
                changes: vec![CodeChange {
                    span: error.span,
                    replacement: format!("{}.clone()", extract_identifier(&error.message)),
                }],
                confidence: 0.7,
            });

            suggestions.push(FixSuggestion {
                title: "Use a reference instead".to_string(),
                explanation:
                    "Instead of moving the value, pass a reference to avoid ownership transfer."
                        .to_string(),
                changes: vec![CodeChange {
                    span: error.span,
                    replacement: format!("&{}", extract_identifier(&error.message)),
                }],
                confidence: 0.8,
            });
        }

        // Double borrow - suggest restructure
        301 => {
            suggestions.push(FixSuggestion {
                title: "Use separate scopes for borrows".to_string(),
                explanation:
                    "Limit the scope of the first borrow so it ends before the second begins."
                        .to_string(),
                changes: vec![],
                confidence: 0.6,
            });
        }

        // Type mismatch
        200 => {
            suggestions.push(FixSuggestion {
                title: "Check your types".to_string(),
                explanation: "Ensure the expression has the expected type. \
                              You may need an explicit conversion."
                    .to_string(),
                changes: vec![],
                confidence: 0.5,
            });
        }

        // Unknown variable - suggest similar names
        202 => {
            suggestions.push(FixSuggestion {
                title: "Did you mean a similar name?".to_string(),
                explanation: "The variable name was not found. Check for typos.".to_string(),
                changes: vec![],
                confidence: 0.4,
            });
        }

        _ => {}
    }

    suggestions
}

/// Extract identifier from error message (helper)
fn extract_identifier(message: &str) -> String {
    // Simple extraction - look for `name` patterns
    if let Some(start) = message.find('`') {
        if let Some(end) = message[start + 1..].find('`') {
            return message[start + 1..start + 1 + end].to_string();
        }
    }
    "value".to_string()
}

// ============================================================================
// PART 4: ERROR EXPLANATIONS (Vyākhyā)
// ============================================================================

/// Get a detailed explanation for an error code
pub fn explain_error(code: ErrorCode) -> Option<&'static str> {
    match code.0 {
        // Type mismatch
        200 => Some(
            r#"
Type Mismatch (प्रकार-विरोध)

This error occurs when an expression has a type that differs from what is expected.

In Sanskrit grammar, this is analogous to "vibhakti-virodha" (case conflict) where
a word is used with the wrong grammatical case.

Common causes:
  • Assigning a value of the wrong type to a variable
  • Passing an argument of the wrong type to a function
  • Returning a value of the wrong type from a function
  • Using an operator with incompatible types

Example:
  kāryakrama add(a: saṃkhyā, b: saṃkhyā) -> saṃkhyā {
      phera a + b;
  }

  mana x: saṃkhyā = "hello";  // ERROR: expected saṃkhyā, found sūtra

Solution: Ensure types match at all usage sites. Use explicit type conversions
if needed, or fix the source of the type mismatch.
"#,
        ),

        // Use after move
        300 => Some(
            r#"
Use After Move (मृत-उपयोग)

This error occurs when you try to use a value after it has been moved.

In Jagannath's ownership system (based on Nyāya's Svāmitva concept), when a value
is moved, the original owner loses access - like giving away a possession.

This maps to Naraka "Tamisram" (तमिस्रम्) - the hell of darkness, because accessing
moved memory leads to undefined, "dark" behavior.

Common causes:
  • Passing a non-Copy value to a function consumes it
  • Assigning a non-Copy value to another variable moves it
  • Moving a value inside a loop

Example:
  mana data = Vec::new();
  process(data);       // data is MOVED here
  println(data.len()); // ERROR: use after move

Solutions:
  1. Clone the value: process(data.clone());
  2. Pass a reference: process(&data);
  3. Restructure code to avoid the move
"#,
        ),

        // Double borrow
        301 => Some(
            r#"
Double Borrow (द्वितीय-ग्रह)

This error occurs when you try to borrow a value while it's already borrowed
in a conflicting way.

Jagannath follows Rust's borrowing rules:
  • You can have multiple immutable borrows (&T)
  • OR one mutable borrow (&mut T)
  • But NOT both at the same time

This prevents data races and maintains referential integrity.

Example:
  mana mut data = vec![1, 2, 3];
  mana r1 = &mut data;     // First mutable borrow
  mana r2 = &mut data;     // ERROR: second mutable borrow
  r1.push(4);

Solutions:
  1. Use separate scopes for borrows
  2. Use interior mutability (RefCell)
  3. Restructure code to avoid simultaneous borrows
"#,
        ),

        // Dangling reference
        502 => Some(
            r#"
Dangling Reference (लम्बमान-निर्देश)

This error occurs when a reference outlives the data it points to.

In Sanskrit, "lambamāna" means "hanging" - like a fruit hanging from a branch.
A dangling reference hangs without support, pointing to deallocated memory.

This maps to Naraka "Andhakūpa" (अन्धकूप) - the dark well, because following
a dangling pointer leads you into dark, undefined territory.

Common causes:
  • Returning a reference to a local variable
  • Storing a reference that outlives its target
  • Creating self-referential structures incorrectly

Example:
  kāryakrama bad() -> &saṃkhyā {
      mana x: saṃkhyā = 42;
      phera &x;  // ERROR: x is deallocated when function returns
  }

Solutions:
  1. Return owned data instead of references
  2. Use 'static lifetime for long-lived data
  3. Use smart pointers (Rc, Arc) for shared ownership
"#,
        ),

        _ => None,
    }
}

// ============================================================================
// PART 5: TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanskrit_error() {
        let err = sanskrit_error(ErrorCode::TYPE_MISMATCH);
        assert_eq!(err.name, "prakāra-virodha");
        assert!(err.naraka.is_some());
    }

    #[test]
    fn test_format_basic() {
        let error = CompilerError::new(
            ErrorCode::TYPE_MISMATCH,
            "expected saṃkhyā, found sūtra",
            Span::new(SourceId(0), 10, 15),
        );

        let formatter = RichFormatter::new(FormatConfig {
            colors: false,
            ..Default::default()
        });

        let output = formatter.format(&error);
        assert!(output.contains("E0200"));
        assert!(output.contains("प्रकार-विरोध"));
    }

    #[test]
    fn test_explain_error() {
        let explanation = explain_error(ErrorCode::TYPE_MISMATCH);
        assert!(explanation.is_some());
        assert!(explanation.unwrap().contains("Type Mismatch"));
    }

    #[test]
    fn test_suggest_fix() {
        let error = CompilerError::new(
            ErrorCode::USE_AFTER_MOVE,
            "use of moved value: `data`",
            Span::new(SourceId(0), 50, 54),
        );

        let suggestions = suggest_fix(&error);
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.title.contains("Clone")));
    }

    #[test]
    fn test_format_config_default() {
        let config = FormatConfig::default();
        assert!(config.colors);
        assert!(config.sanskrit);
        assert!(config.suggestions);
    }

    #[test]
    fn test_format_with_source() {
        let mut formatter = RichFormatter::new(FormatConfig {
            colors: false,
            ..Default::default()
        });

        let source_id = formatter.add_source(
            "test.jag".to_string(),
            "mana x: saṃkhyā = 42;\nmana y = x + 1;".to_string(),
        );

        let error = CompilerError::new(
            ErrorCode::TYPE_MISMATCH,
            "expected saṃkhyā",
            Span::new(source_id, 0, 4),
        );

        let output = formatter.format(&error);
        assert!(output.contains("test.jag"));
    }
}
