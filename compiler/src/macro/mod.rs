//! Macro Expansion
//!
//! Compile-time macro expansion using Mantra (incantation) patterns.

pub mod mantra;
pub mod hygiene;

pub use mantra::*;
pub use hygiene::*;

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

/// Macro expander
pub struct MacroExpander {
    /// Defined macros
    macros: Vec<Macro>,
    /// Hygiene context
    hygiene_ctx: HygieneContext,
    /// Expansion depth
    depth: usize,
    /// Maximum expansion depth
    max_depth: usize,
}

impl MacroExpander {
    pub fn new() -> Self {
        Self {
            macros: Vec::new(),
            hygiene_ctx: HygieneContext::new(),
            depth: 0,
            max_depth: 256,
        }
    }

    /// Define a macro
    pub fn define(&mut self, mac: Macro) {
        self.macros.push(mac);
    }

    /// Expand all macros in source
    pub fn expand_all(&mut self, source: &str) -> Result<String, MacroError> {
        let mut result = source.to_string();

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

        Ok(result)
    }

    fn expand_once(&mut self, source: &str) -> Result<String, MacroError> {
        // TODO: Implement actual macro expansion
        Ok(source.to_string())
    }
}

impl Default for MacroExpander {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro error
#[derive(Debug)]
pub enum MacroError {
    /// Unknown macro
    UnknownMacro(String),
    /// Pattern mismatch
    PatternMismatch,
    /// Max depth exceeded
    MaxDepthExceeded,
    /// Hygiene violation
    HygieneViolation(String),
}
