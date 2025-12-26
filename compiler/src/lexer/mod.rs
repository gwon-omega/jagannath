//! Lexer Module - Lexical Analysis for Jagannath
//!
//! Handles Sanskrit morphological tokenization including:
//! - Dhātu (root word) recognition
//! - Sandhi (phonetic combination) splitting
//! - Affix (pratyaya) extraction
//! - Token generation

pub mod token;
pub mod dhatu;
pub mod sandhi;
pub mod scanner;
pub mod affixes;

// Re-exports
pub use token::{Token, TokenKind, Span};
pub use dhatu::DhatuDictionary;
pub use sandhi::SandhiFst;
pub use scanner::Scanner;
pub use affixes::{Affix, AffixSequence};

/// Main lexer structure
pub struct Lexer<'src> {
    /// Source code being lexed
    source: &'src str,
    /// Current position in source
    position: usize,
    /// Dhātu dictionary for root recognition (optional)
    dhatu_dict: Option<&'src DhatuDictionary>,
    /// Sandhi FST for phonetic splitting (optional)
    sandhi_fst: Option<&'src SandhiFst>,
}

impl<'src> Lexer<'src> {
    /// Create a new lexer for the given source (simple mode)
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            position: 0,
            dhatu_dict: None,
            sandhi_fst: None,
        }
    }

    /// Create a new lexer with full Sanskrit morphology support
    pub fn with_morphology(
        source: &'src str,
        dhatu_dict: &'src DhatuDictionary,
        sandhi_fst: &'src SandhiFst,
    ) -> Self {
        Self {
            source,
            position: 0,
            dhatu_dict: Some(dhatu_dict),
            sandhi_fst: Some(sandhi_fst),
        }
    }

    /// Tokenize the entire source
    pub fn tokenize(&mut self) -> Vec<Token> {
        todo!("Implement tokenization with Sanskrit morphology")
    }

    /// Apply sandhi rules to normalize input
    fn apply_sandhi_rules(&self, input: &str) -> String {
        todo!("Implement sandhi rule application")
    }

    /// Match input against dhātu dictionary
    fn match_dhatu(&self, normalized: &str) -> Option<&str> {
        todo!("Implement dhātu matching")
    }

    /// Extract affixes from morphological form
    fn extract_affixes(&self, form: &str) -> Option<AffixSequence> {
        todo!("Implement affix extraction")
    }

    /// Validate affix sequence (e.g., -l-b is invalid)
    fn validate_affix_sequence(&self, affixes: &AffixSequence) -> Result<(), String> {
        todo!("Implement affix sequence validation")
    }
}
