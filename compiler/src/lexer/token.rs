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
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end, line: 1, column: 1 }
    }

    pub fn with_location(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self { start, end, line, column }
    }

    /// Create a dummy span (for generated code/tests)
    pub fn dummy() -> Self {
        Self { start: 0, end: 0, line: 1, column: 1 }
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
    // Control Flow Keywords
    // ========================================================================
    /// for loop
    For,
    /// in (iteration)
    In,
    /// match (pattern matching)
    Match,
    /// break
    Break,
    /// continue
    Continue,

    // ========================================================================
    // Modifiers
    // ========================================================================
    /// pub - public visibility
    Pub,
    /// mut - mutable
    Mut,
    /// const - constant
    Const,
    /// let - binding
    Let,
    /// static - static lifetime
    Static,

    // ========================================================================
    // Module System
    // ========================================================================
    /// mod - module
    Mod,
    /// use - import
    Use,
    /// impl - implementation
    Impl,
    /// trait - interface
    Trait,
    /// self - current instance
    SelfValue,
    /// Self - current type
    SelfType,
    /// super - parent module
    Super,
    /// crate - root module
    Crate,
    /// as - type cast/rename
    As,
    /// ref - reference
    Ref,
    /// unsafe - unsafe block
    Unsafe,
    /// extern - external linkage
    Extern,
    /// where - bounds clause
    Where,

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
    AmpAmp,
    /// ||
    PipePipe,
    /// !
    Bang,
    /// ~
    Tilde,
    /// &
    Ampersand,
    /// |
    Pipe,
    /// ^
    Caret,
    /// ->
    Arrow,
    /// =>
    FatArrow,
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
    /// ...
    DotDotDot,
    /// ..=
    DotDotEquals,
    /// #
    Hash,
    /// ##
    HashHash,
    /// @
    At,
    /// $
    Dollar,
    /// <<
    LeftShift,
    /// >>
    RightShift,
    /// +=
    PlusEquals,
    /// -=
    MinusEquals,
    /// *=
    StarEquals,
    /// /=
    SlashEquals,
    /// %=
    PercentEquals,

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

    // ========================================================================
    // Legacy Aliases / Kāraka Markers
    // ========================================================================
    /// @kartṛ = agent
    KarakaKartr,
    /// @karman = patient
    KarakaKarman,
    /// @karaṇa = instrument
    KarakaKarana,
    /// @sampradāna = recipient
    KarakaSampradana,
    /// @apādāna = source
    KarakaApadana,
    /// @adhikaraṇa = location
    KarakaAdhikarana,
}

// ============================================================================
// Token Kind Aliases for Backward Compatibility
// ============================================================================
#[allow(non_upper_case_globals)]
impl TokenKind {
    /// Alias for Karyakrama
    pub const KwKaryakrama: TokenKind = TokenKind::Karyakrama;
    /// Alias for Phera
    pub const KwPhera: TokenKind = TokenKind::Phera;
    /// Alias for Yad
    pub const KwYad: TokenKind = TokenKind::Yad;
    /// Alias for Cala
    pub const KwCala: TokenKind = TokenKind::Cala;
    /// Alias for Nirma
    pub const KwNirma: TokenKind = TokenKind::Nirma;
    /// Alias for Mukta
    pub const KwMukta: TokenKind = TokenKind::Mukta;

    /// Alias for LeftParen
    pub const LParen: TokenKind = TokenKind::LeftParen;
    /// Alias for RightParen
    pub const RParen: TokenKind = TokenKind::RightParen;
    /// Alias for LeftBrace
    pub const LBrace: TokenKind = TokenKind::LeftBrace;
    /// Alias for RightBrace
    pub const RBrace: TokenKind = TokenKind::RightBrace;
    /// Alias for LeftBracket
    pub const LBracket: TokenKind = TokenKind::LeftBracket;
    /// Alias for RightBracket
    pub const RBracket: TokenKind = TokenKind::RightBracket;
}

/// Alias Ident to Identifier for tests
#[allow(non_snake_case)]
impl TokenKind {
    /// Create an identifier token
    pub fn Ident(name: String) -> Self {
        TokenKind::Identifier(name)
    }
}
