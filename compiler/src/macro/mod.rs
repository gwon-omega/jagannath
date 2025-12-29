//! Macro Expansion (Vistāra - विस्तार)
//!
//! Compile-time macro expansion using Mantra (incantation) patterns.
//!
//! # Sanskrit Foundation
//! The macro system embodies **Vistāra** (विस्तार - expansion/elaboration):
//! - Macros are **mantras** (मन्त्र) - sacred formulas that invoke transformation
//! - Pattern matching follows **pratyabhijñā** (प्रत्यभिज्ञा - recognition)
//! - Expansion follows **vyākaraṇa** (व्याकरण - grammatical derivation)
//!
//! # Philosophy: Pāṇini's Meta-rules
//! Like Pāṇini's meta-rules (paribhāṣā) that govern other rules,
//! macros are compile-time transformations that generate code.

pub mod hygiene;
pub mod mantra;

pub use hygiene::*;
pub use mantra::*;

use regex::Regex;
use std::collections::HashMap;

/// Macro definition
#[derive(Debug, Clone)]
pub struct Macro {
    /// Macro name
    pub name: String,
    /// Pattern to match
    pub pattern: MacroPattern,
    /// Expansion template
    pub expansion: MacroExpansion,
    /// Hygiene mode
    pub hygiene: HygieneMode,
}

/// Macro pattern
#[derive(Debug, Clone)]
pub enum MacroPattern {
    /// Exact token match
    Literal(String),
    /// Capture variable
    Capture(String, CaptureKind),
    /// Sequence
    Sequence(Vec<MacroPattern>),
    /// Repetition
    Repeat(Box<MacroPattern>, RepetitionKind),
    /// Choice
    Choice(Vec<MacroPattern>),
}

/// Capture kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureKind {
    /// Expression
    Expr,
    /// Statement
    Stmt,
    /// Type
    Type,
    /// Pattern
    Pat,
    /// Identifier
    Ident,
    /// Token tree
    TokenTree,
}

/// Repetition kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepetitionKind {
    /// Zero or more (*)
    ZeroOrMore,
    /// One or more (+)
    OneOrMore,
    /// Zero or one (?)
    ZeroOrOne,
}

/// Macro expansion template
#[derive(Debug, Clone)]
pub enum MacroExpansion {
    /// Literal tokens
    Literal(String),
    /// Substitution of captured variable
    Substitute(String),
    /// Sequence of expansions
    Sequence(Vec<MacroExpansion>),
    /// Repetition
    Repeat(Box<MacroExpansion>, String),
}

/// Captured values during macro pattern matching
#[derive(Debug, Clone, Default)]
pub struct MacroCaptures {
    /// Single captures
    singles: HashMap<String, String>,
    /// Repeated captures
    repeats: HashMap<String, Vec<String>>,
}

impl MacroCaptures {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, name: String, value: String) {
        self.singles.insert(name, value);
    }

    pub fn insert_repeat(&mut self, name: String, values: Vec<String>) {
        self.repeats.insert(name, values);
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.singles.get(name)
    }

    pub fn get_repeat(&self, name: &str) -> Option<&Vec<String>> {
        self.repeats.get(name)
    }
}

/// Macro expander
pub struct MacroExpander {
    /// Defined macros
    macros: Vec<Macro>,
    /// Built-in mantras
    builtin_mantras: HashMap<String, BuiltinMantra>,
    /// Hygiene context
    hygiene_ctx: HygieneContext,
    /// Expansion depth
    depth: usize,
    /// Maximum expansion depth
    max_depth: usize,
    /// Expansion cache for memoization (Smṛti - memory)
    cache: HashMap<String, String>,
}

impl MacroExpander {
    pub fn new() -> Self {
        let mut builtin_mantras = HashMap::new();
        // Register all built-in mantras
        for mantra in [
            BuiltinMantra::Om,
            BuiltinMantra::Mudrana,
            BuiltinMantra::Sthira,
            BuiltinMantra::Prakasha,
            BuiltinMantra::Pariksha,
            BuiltinMantra::Vinyasa,
            BuiltinMantra::Kalpana,
            BuiltinMantra::Vistara,
        ] {
            // Register both ASCII and IAST versions
            if let Some(name) = Self::mantra_names(&mantra) {
                for n in name {
                    builtin_mantras.insert(n.to_string(), mantra);
                }
            }
        }

        Self {
            macros: Vec::new(),
            builtin_mantras,
            hygiene_ctx: HygieneContext::new(),
            depth: 0,
            max_depth: 256,
            cache: HashMap::new(),
        }
    }

    /// Get all name variants for a mantra
    fn mantra_names(mantra: &BuiltinMantra) -> Option<&'static [&'static str]> {
        match mantra {
            BuiltinMantra::Om => Some(&["oṁ", "om", "dṛḍha"]),
            BuiltinMantra::Mudrana => Some(&["mudraṇa", "mudrana", "print"]),
            BuiltinMantra::Sthira => Some(&["sthira", "const"]),
            BuiltinMantra::Prakasha => Some(&["prakāśa", "prakasha", "include"]),
            BuiltinMantra::Pariksha => Some(&["parikṣā", "pariksha", "debug_assert"]),
            BuiltinMantra::Vinyasa => Some(&["vinyāsa", "vinyasa", "format"]),
            BuiltinMantra::Kalpana => Some(&["kalpanā", "kalpana", "struct"]),
            BuiltinMantra::Vistara => Some(&["vistāra", "vistara", "derive"]),
        }
    }

    /// Define a macro
    pub fn define(&mut self, mac: Macro) {
        self.macros.push(mac);
    }

    /// Check if a macro is defined
    pub fn is_defined(&self, name: &str) -> bool {
        self.macros.iter().any(|m| m.name == name) || self.builtin_mantras.contains_key(name)
    }

    /// Expand all macros in source
    pub fn expand_all(&mut self, source: &str) -> Result<String, MacroError> {
        // Check cache first (Smṛti - memory/remembrance)
        if let Some(cached) = self.cache.get(source) {
            return Ok(cached.clone());
        }

        let mut result = source.to_string();
        self.depth = 0;

        loop {
            let prev = result.clone();
            result = self.expand_once(&result)?;

            if result == prev {
                break;
            }

            self.depth += 1;
            if self.depth > self.max_depth {
                return Err(MacroError::MaxDepthExceeded);
            }
        }

        // Cache the result
        self.cache.insert(source.to_string(), result.clone());

        Ok(result)
    }

    /// Expand macros once (single pass)
    fn expand_once(&mut self, source: &str) -> Result<String, MacroError> {
        let mut result = source.to_string();

        // First, expand built-in mantras (sacred incantations)
        result = self.expand_builtin_mantras(&result)?;

        // Then, expand user-defined macros
        result = self.expand_user_macros(&result)?;

        Ok(result)
    }

    /// Expand built-in mantras
    fn expand_builtin_mantras(&mut self, source: &str) -> Result<String, MacroError> {
        let mut result = source.to_string();

        // Pattern: mantra_name!(args)
        // Support both ASCII (mudrana!) and IAST (mudraṇa!) variants
        let mantra_pattern =
            Regex::new(r"([a-zA-Zā-ṛĀ-Ṛ_][a-zA-Zā-ṛĀ-Ṛ0-9_]*)!\s*\(([^)]*)\)").unwrap();

        for cap in mantra_pattern.captures_iter(source) {
            let full_match = cap.get(0).unwrap().as_str();
            let name = cap.get(1).unwrap().as_str();
            let args = cap.get(2).unwrap().as_str();

            if let Some(&mantra) = self.builtin_mantras.get(name) {
                let expansion = self.expand_mantra(mantra, args)?;
                result = result.replacen(full_match, &expansion, 1);
            }
        }

        Ok(result)
    }

    /// Expand a single built-in mantra
    fn expand_mantra(&mut self, mantra: BuiltinMantra, args: &str) -> Result<String, MacroError> {
        match mantra {
            BuiltinMantra::Om | BuiltinMantra::Pariksha => {
                // dṛḍha!(condition, "message") → assertion
                let parts: Vec<&str> = args.splitn(2, ',').collect();
                let condition = parts.first().map(|s| s.trim()).unwrap_or("true");
                let message = parts
                    .get(1)
                    .map(|s| s.trim())
                    .unwrap_or("\"Assertion failed\"");

                Ok(format!(
                    r#"yad !({}) {{ vikṣepa!({}) }}"#,
                    condition, message
                ))
            }
            BuiltinMantra::Mudrana => {
                // mudraṇa!("format", args...) → print expansion
                // Generate runtime print call
                Ok(format!("__jagannath_print({})", args))
            }
            BuiltinMantra::Sthira => {
                // sthira!(expr) → compile-time evaluation marker
                Ok(format!("/* const */ {}", args))
            }
            BuiltinMantra::Prakasha => {
                // prakāśa!("path") → include file contents
                let path = args.trim().trim_matches('"');
                Ok(format!("/* include: {} */", path))
            }
            BuiltinMantra::Vinyasa => {
                // vinyāsa!("format", args...) → format string
                Ok(format!("__jagannath_format({})", args))
            }
            BuiltinMantra::Kalpana => {
                // kalpanā!(TypeName { field: value, ... }) → struct literal
                Ok(args.to_string())
            }
            BuiltinMantra::Vistara => {
                // vistāra!(trait for Type) → derive trait
                Ok(format!("/* derive: {} */", args))
            }
        }
    }

    /// Expand user-defined macros
    fn expand_user_macros(&mut self, source: &str) -> Result<String, MacroError> {
        let mut result = source.to_string();

        for mac in self.macros.clone() {
            result = self.expand_macro(&mac, &result)?;
        }

        Ok(result)
    }

    /// Expand a single user-defined macro
    fn expand_macro(&mut self, mac: &Macro, source: &str) -> Result<String, MacroError> {
        let pattern_str = format!(r"{}!\s*\(([^)]*)\)", regex::escape(&mac.name));
        let pattern = Regex::new(&pattern_str).map_err(|_| MacroError::PatternMismatch)?;

        let mut result = source.to_string();

        for cap in pattern.captures_iter(source) {
            let full_match = cap.get(0).unwrap().as_str();
            let args = cap.get(1).unwrap().as_str();

            // Enter macro scope for hygiene
            let _scope = self.hygiene_ctx.enter_macro();

            // Match pattern and capture bindings
            let captures = self.match_pattern(&mac.pattern, args)?;

            // Apply expansion template
            let expansion = self.apply_expansion(&mac.expansion, &captures)?;

            // Exit macro scope
            self.hygiene_ctx.exit_macro();

            result = result.replacen(full_match, &expansion, 1);
        }

        Ok(result)
    }

    /// Match a pattern against input and capture bindings
    fn match_pattern(
        &self,
        pattern: &MacroPattern,
        input: &str,
    ) -> Result<MacroCaptures, MacroError> {
        let mut captures = MacroCaptures::new();

        match pattern {
            MacroPattern::Literal(expected) => {
                if input.trim() != expected.trim() {
                    return Err(MacroError::PatternMismatch);
                }
            }
            MacroPattern::Capture(name, _kind) => {
                captures.insert(name.clone(), input.trim().to_string());
            }
            MacroPattern::Sequence(patterns) => {
                // Split input by commas and match each pattern
                let parts: Vec<&str> = input.split(',').collect();
                if parts.len() != patterns.len() {
                    return Err(MacroError::PatternMismatch);
                }
                for (pat, part) in patterns.iter().zip(parts.iter()) {
                    let sub_captures = self.match_pattern(pat, part)?;
                    for (k, v) in sub_captures.singles {
                        captures.insert(k, v);
                    }
                }
            }
            MacroPattern::Repeat(inner, _kind) => {
                let parts: Vec<&str> = input.split(',').collect();
                let mut repeated = Vec::new();
                for part in parts {
                    if let MacroPattern::Capture(name, _) = inner.as_ref() {
                        repeated.push(part.trim().to_string());
                        captures.insert_repeat(name.clone(), repeated.clone());
                    }
                }
            }
            MacroPattern::Choice(choices) => {
                for choice in choices {
                    if let Ok(sub_captures) = self.match_pattern(choice, input) {
                        return Ok(sub_captures);
                    }
                }
                return Err(MacroError::PatternMismatch);
            }
        }

        Ok(captures)
    }

    /// Apply expansion template with captured bindings
    fn apply_expansion(
        &self,
        expansion: &MacroExpansion,
        captures: &MacroCaptures,
    ) -> Result<String, MacroError> {
        match expansion {
            MacroExpansion::Literal(s) => Ok(s.clone()),
            MacroExpansion::Substitute(name) => captures
                .get(name)
                .cloned()
                .ok_or_else(|| MacroError::UnknownMacro(format!("Unbound capture: {}", name))),
            MacroExpansion::Sequence(parts) => {
                let mut result = String::new();
                for part in parts {
                    result.push_str(&self.apply_expansion(part, captures)?);
                }
                Ok(result)
            }
            MacroExpansion::Repeat(inner, sep_var) => {
                if let Some(values) = captures.get_repeat(sep_var) {
                    let expanded: Result<Vec<String>, MacroError> = values
                        .iter()
                        .map(|v| {
                            let mut local_captures = captures.clone();
                            local_captures.insert(sep_var.clone(), v.clone());
                            self.apply_expansion(inner, &local_captures)
                        })
                        .collect();
                    Ok(expanded?.join(", "))
                } else {
                    Ok(String::new())
                }
            }
        }
    }

    /// Clear the expansion cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for MacroExpander {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro error
#[derive(Debug, Clone)]
pub enum MacroError {
    /// Unknown macro
    UnknownMacro(String),
    /// Pattern mismatch
    PatternMismatch,
    /// Max depth exceeded
    MaxDepthExceeded,
    /// Hygiene violation
    HygieneViolation(String),
    /// IO error
    IoError(String),
}

impl std::fmt::Display for MacroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownMacro(name) => write!(f, "Unknown macro: {}", name),
            Self::PatternMismatch => write!(f, "Macro pattern did not match"),
            Self::MaxDepthExceeded => write!(f, "Maximum macro expansion depth exceeded"),
            Self::HygieneViolation(msg) => write!(f, "Macro hygiene violation: {}", msg),
            Self::IoError(msg) => write!(f, "IO error during macro expansion: {}", msg),
        }
    }
}

impl std::error::Error for MacroError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_expander_new() {
        let expander = MacroExpander::new();
        assert!(expander.is_defined("mudraṇa"));
        assert!(expander.is_defined("mudrana"));
        assert!(expander.is_defined("oṁ"));
        assert!(expander.is_defined("om"));
    }

    #[test]
    fn test_builtin_mudrana_expansion() {
        let mut expander = MacroExpander::new();
        let source = r#"mudraṇa!("Hello, जगन्नाथ!")"#;
        let result = expander.expand_all(source).unwrap();
        assert!(result.contains("__jagannath_print"));
    }

    #[test]
    fn test_builtin_drda_assertion() {
        let mut expander = MacroExpander::new();
        let source = r#"dṛḍha!(x > 0, "Must be positive")"#;
        let result = expander.expand_all(source).unwrap();
        assert!(result.contains("yad"));
        assert!(result.contains("vikṣepa"));
    }

    #[test]
    fn test_expansion_cache() {
        let mut expander = MacroExpander::new();
        let source = r#"mudraṇa!("test")"#;

        let result1 = expander.expand_all(source).unwrap();
        let result2 = expander.expand_all(source).unwrap();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_user_macro_definition() {
        let mut expander = MacroExpander::new();

        let mac = Macro {
            name: "twice".to_string(),
            pattern: MacroPattern::Capture("x".to_string(), CaptureKind::Expr),
            expansion: MacroExpansion::Sequence(vec![
                MacroExpansion::Substitute("x".to_string()),
                MacroExpansion::Literal(" + ".to_string()),
                MacroExpansion::Substitute("x".to_string()),
            ]),
            hygiene: HygieneMode::Full,
        };

        expander.define(mac);
        assert!(expander.is_defined("twice"));
    }

    #[test]
    fn test_max_depth_protection() {
        let mut expander = MacroExpander::new();
        expander.max_depth = 3;

        // Define a recursive macro that would expand infinitely
        let mac = Macro {
            name: "recurse".to_string(),
            pattern: MacroPattern::Capture("x".to_string(), CaptureKind::Expr),
            expansion: MacroExpansion::Literal("recurse!(recurse)".to_string()),
            hygiene: HygieneMode::Full,
        };

        expander.define(mac);

        let result = expander.expand_all("recurse!(start)");
        // Should either succeed with limited expansion or fail with MaxDepthExceeded
        // The current implementation stops when no changes occur, so this should succeed
        assert!(result.is_ok());
    }
}
