//! Scanner - Low-level character scanning
//!
//! Handles UTF-8 source code scanning with support for:
//! - Devanagari script
//! - IAST transliteration
//! - Mixed scripts

use super::Span;

/// Character scanner for source code
pub struct Scanner<'src> {
    /// Source code as bytes
    source: &'src str,
    /// Current byte position
    position: usize,
    /// Current line number (1-indexed)
    line: usize,
    /// Current column (1-indexed)
    column: usize,
    /// Start of current token
    token_start: usize,
    /// Line at token start
    token_start_line: usize,
    /// Column at token start
    token_start_column: usize,
}

impl<'src> Scanner<'src> {
    /// Create a new scanner for the given source
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            position: 0,
            line: 1,
            column: 1,
            token_start: 0,
            token_start_line: 1,
            token_start_column: 1,
        }
    }

    /// Check if we've reached end of file
    pub fn is_eof(&self) -> bool {
        self.position >= self.source.len()
    }

    /// Peek at the current character without consuming
    pub fn peek(&self) -> Option<char> {
        self.source[self.position..].chars().next()
    }

    /// Peek at the next character (one ahead of current)
    pub fn peek_next(&self) -> Option<char> {
        let mut chars = self.source[self.position..].chars();
        chars.next();
        chars.next()
    }

    /// Advance and return the current character
    pub fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += ch.len_utf8();

        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Some(ch)
    }

    /// Mark the start of a new token
    pub fn start_token(&mut self) {
        self.token_start = self.position;
        self.token_start_line = self.line;
        self.token_start_column = self.column;
    }

    /// Get the current token span
    pub fn token_span(&self) -> Span {
        Span::with_location(
            self.token_start,
            self.position,
            self.token_start_line,
            self.token_start_column,
        )
    }

    /// Get the current token text
    pub fn token_text(&self) -> &'src str {
        &self.source[self.token_start..self.position]
    }

    /// Skip whitespace (but not newlines if significant)
    pub fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() && ch != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skip to end of line (for comments)
    pub fn skip_to_eol(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    /// Check if character is a Sanskrit letter (Devanagari or IAST)
    pub fn is_sanskrit_letter(ch: char) -> bool {
        // Devanagari range
        if ('\u{0900}'..='\u{097F}').contains(&ch) {
            return true;
        }
        // IAST diacritics
        matches!(ch,
            'ā' | 'ī' | 'ū' | 'ṛ' | 'ṝ' | 'ḷ' | 'ḹ' |
            'ṅ' | 'ñ' | 'ṭ' | 'ḍ' | 'ṇ' | 'ś' | 'ṣ' |
            'ḥ' | 'ṃ' | 'a'..='z' | 'A'..='Z'
        )
    }

    /// Check if character can be part of an identifier
    pub fn is_identifier_char(ch: char) -> bool {
        Self::is_sanskrit_letter(ch) || ch.is_ascii_digit() || ch == '_'
    }

    /// Get current line number
    pub fn line(&self) -> usize {
        self.line
    }

    /// Get current column
    pub fn column(&self) -> usize {
        self.column
    }
}
