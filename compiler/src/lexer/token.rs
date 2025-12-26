//! Token Definitions for Jagannath
//!
//! Defines all token types used in lexical analysis.

use std::ops::Range;

/// Source span representing location in source code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// Start byte offset
    pub start: usize,
    /// End byte offset (exclusive)
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn to_range(self) -> Range<usize> {
        self.start..self.end
    }
}

/// Token produced by the lexer
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// Kind of token
    pub kind: TokenKind,
    /// Source span
    pub span: Span,
    /// Original lexeme text
    pub lexeme: String,
}

/// All token kinds in Jagannath
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // ========================================================================
    // Literals
    // ========================================================================
    /// Integer literal (t8, t16, t32, t64)
    IntLiteral(i64),
    /// Float literal (f32, f64)
    FloatLiteral(f64),
    /// String literal (sūtra)
    StringLiteral(String),
    /// Boolean literal (bīta)
    BoolLiteral(bool),

    // ========================================================================
    // Identifiers & Keywords
    // ========================================================================
    /// Identifier (Sanskrit or English)
    Identifier(String),
    /// Dhātu (root word) with recognized meaning
    Dhatu { root: String, meaning: String },

    // ========================================================================
    // Sanskrit Keywords (v1.0)
    // ========================================================================
    /// kāryakrama - function definition
    Karyakrama,
    /// prakāra - type definition
    Prakara,
    /// yad - if/when conditional
    Yad,
    /// anyathā - else
    Anyatha,
    /// cala - loop/iterate
    Cala,
    /// phera - return
    Phera,
    /// nirmā - construct/new
    Nirma,
    /// mukta - free/destroy
    Mukta,
    /// paṭha - read
    Patha,
    /// likha - write
    Likha,
    /// mudraṇa - print
    Mudrana,
    /// pradhāna - main
    Pradhana,
    /// āśaya - async
    Ashaya,
    /// pratīkṣā - await
    Pratiksha,
    /// saha - together/concurrent
    Saha,

    // ========================================================================
    // Type Keywords
    // ========================================================================
    /// Saṅkhyā - number
    Sankhya,
    /// Sūtra - string
    Sutra,
    /// Sūci - list/vector
    Suci,
    /// Sāraṇī - map/table
    Sarani,
    /// Vikalpa - option
    Vikalpa,
    /// Phala - result
    Phala,
    /// Saphala - success
    Saphala,
    /// Viphala - failure
    Viphala,
    /// Truṭi - error
    Truti,

    // ========================================================================
    // Affixes (Pratyaya)
    // ========================================================================
    /// Affix marker
    Affix(super::Affix),

    // ========================================================================
    // Operators
    // ========================================================================
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Star,
    /// /
    Slash,
    /// %
    Percent,
    /// =
    Equals,
    /// ==
    EqualsEquals,
    /// !=
    NotEquals,
    /// <
    LessThan,
    /// <=
    LessEquals,
    /// >
    GreaterThan,
    /// >=
    GreaterEquals,
    /// &&
    And,
    /// ||
    Or,
    /// !
    Not,
    /// &
    Ampersand,
    /// |
    Pipe,
    /// ^
    Caret,
    /// →
    Arrow,
    /// ?
    Question,
    /// :
    Colon,
    /// ::
    ColonColon,
    /// .
    Dot,
    /// ..
    DotDot,
    /// #
    Hash,
    /// ##
    HashHash,

    // ========================================================================
    // Delimiters
    // ========================================================================
    /// (
    LeftParen,
    /// )
    RightParen,
    /// {
    LeftBrace,
    /// }
    RightBrace,
    /// [
    LeftBracket,
    /// ]
    RightBracket,
    /// <
    LeftAngle,
    /// >
    RightAngle,
    /// ,
    Comma,
    /// ;
    Semicolon,

    // ========================================================================
    // Special
    // ========================================================================
    /// End of file
    Eof,
    /// Newline (significant in some contexts)
    Newline,
    /// Comment
    Comment(String),
    /// Error token
    Error(String),
}
