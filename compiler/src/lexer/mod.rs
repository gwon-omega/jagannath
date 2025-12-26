//! Lexer Module - Lexical Analysis for Jagannath
//!
//! Handles Sanskrit morphological tokenization including:
//! - Dhātu (root word) recognition
//! - Sandhi (phonetic combination) splitting
//! - Affix (pratyaya) extraction
//! - Token generation
//!
//! ## Two-Pass Tokenization
//! 1. **Basic Scan**: Character-level tokenization (like C)
//! 2. **Sanskrit Analysis**: Dhātu/sandhi processing (optional)

pub mod affixes;
pub mod dhatu;
pub mod sandhi;
pub mod scanner;
pub mod token;

// Re-exports
pub use affixes::{Affix, AffixSequence};
pub use dhatu::DhatuDictionary;
pub use sandhi::SandhiFst;
pub use scanner::Scanner;
pub use token::{Span, Token, TokenKind};

/// Main lexer structure
pub struct Lexer<'src> {
    /// Scanner for character-level operations
    scanner: Scanner<'src>,
    /// Source code being lexed
    source: &'src str,
    /// Dhātu dictionary for root recognition (optional)
    dhatu_dict: Option<&'src DhatuDictionary>,
    /// Sandhi FST for phonetic splitting (optional)
    sandhi_fst: Option<&'src SandhiFst>,
}

impl<'src> Lexer<'src> {
    /// Create a new lexer for the given source (simple mode)
    pub fn new(source: &'src str) -> Self {
        Self {
            scanner: Scanner::new(source),
            source,
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
            scanner: Scanner::new(source),
            source,
            dhatu_dict: Some(dhatu_dict),
            sandhi_fst: Some(sandhi_fst),
        }
    }

    /// Tokenize the entire source
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.scanner.is_eof() {
            // Skip whitespace
            self.skip_whitespace_and_comments();

            if self.scanner.is_eof() {
                break;
            }

            // Scan one token
            self.scanner.start_token();
            if let Some(token) = self.scan_token() {
                tokens.push(token);
            }
        }

        // Add EOF token
        tokens.push(Token {
            kind: TokenKind::Eof,
            span: Span::new(self.source.len(), self.source.len()),
            lexeme: String::new(),
        });

        tokens
    }

    /// Skip whitespace and comments
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // Skip whitespace
            while let Some(ch) = self.scanner.peek() {
                if ch.is_whitespace() {
                    self.scanner.advance();
                } else {
                    break;
                }
            }

            // Check for comments
            if let Some(ch) = self.scanner.peek() {
                if ch == '/' {
                    if let Some(next) = self.scanner.peek_next() {
                        if next == '/' {
                            // Line comment
                            self.scanner.skip_to_eol();
                            continue;
                        } else if next == '*' {
                            // Block comment
                            self.skip_block_comment();
                            continue;
                        }
                    }
                } else if ch == '#' {
                    // Shell-style comment
                    self.scanner.skip_to_eol();
                    continue;
                }
            }
            break;
        }
    }

    /// Skip block comment /* ... */
    fn skip_block_comment(&mut self) {
        self.scanner.advance(); // /
        self.scanner.advance(); // *

        let mut depth = 1;
        while depth > 0 && !self.scanner.is_eof() {
            match self.scanner.peek() {
                Some('*') => {
                    self.scanner.advance();
                    if self.scanner.peek() == Some('/') {
                        self.scanner.advance();
                        depth -= 1;
                    }
                }
                Some('/') => {
                    self.scanner.advance();
                    if self.scanner.peek() == Some('*') {
                        self.scanner.advance();
                        depth += 1; // Nested comment
                    }
                }
                Some(_) => {
                    self.scanner.advance();
                }
                None => break,
            }
        }
    }

    /// Scan a single token
    fn scan_token(&mut self) -> Option<Token> {
        let ch = self.scanner.advance()?;

        let kind = match ch {
            // Single-character tokens
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            ':' => {
                if self.scanner.peek() == Some(':') {
                    self.scanner.advance();
                    TokenKind::ColonColon
                } else {
                    TokenKind::Colon
                }
            }
            '.' => {
                if self.scanner.peek() == Some('.') {
                    self.scanner.advance();
                    if self.scanner.peek() == Some('.') {
                        self.scanner.advance();
                        TokenKind::DotDotDot
                    } else if self.scanner.peek() == Some('=') {
                        self.scanner.advance();
                        TokenKind::DotDotEquals
                    } else {
                        TokenKind::DotDot
                    }
                } else {
                    TokenKind::Dot
                }
            }
            '+' => {
                if self.scanner.peek() == Some('=') {
                    self.scanner.advance();
                    TokenKind::PlusEquals
                } else {
                    TokenKind::Plus
                }
            }
            '-' => {
                if self.scanner.peek() == Some('>') {
                    self.scanner.advance();
                    TokenKind::Arrow
                } else if self.scanner.peek() == Some('=') {
                    self.scanner.advance();
                    TokenKind::MinusEquals
                } else {
                    TokenKind::Minus
                }
            }
            '*' => {
                if self.scanner.peek() == Some('=') {
                    self.scanner.advance();
                    TokenKind::StarEquals
                } else {
                    TokenKind::Star
                }
            }
            '/' => {
                if self.scanner.peek() == Some('=') {
                    self.scanner.advance();
                    TokenKind::SlashEquals
                } else {
                    TokenKind::Slash
                }
            }
            '%' => {
                if self.scanner.peek() == Some('=') {
                    self.scanner.advance();
                    TokenKind::PercentEquals
                } else {
                    TokenKind::Percent
                }
            }
            '=' => {
                if self.scanner.peek() == Some('=') {
                    self.scanner.advance();
                    TokenKind::EqualsEquals
                } else if self.scanner.peek() == Some('>') {
                    self.scanner.advance();
                    TokenKind::FatArrow
                } else {
                    TokenKind::Equals
                }
            }
            '!' => {
                if self.scanner.peek() == Some('=') {
                    self.scanner.advance();
                    TokenKind::NotEquals
                } else {
                    TokenKind::Bang
                }
            }
            '<' => {
                if self.scanner.peek() == Some('=') {
                    self.scanner.advance();
                    TokenKind::LessEquals
                } else if self.scanner.peek() == Some('<') {
                    self.scanner.advance();
                    TokenKind::LeftShift
                } else {
                    TokenKind::LessThan
                }
            }
            '>' => {
                if self.scanner.peek() == Some('=') {
                    self.scanner.advance();
                    TokenKind::GreaterEquals
                } else if self.scanner.peek() == Some('>') {
                    self.scanner.advance();
                    TokenKind::RightShift
                } else {
                    TokenKind::GreaterThan
                }
            }
            '&' => {
                if self.scanner.peek() == Some('&') {
                    self.scanner.advance();
                    TokenKind::AmpAmp
                } else {
                    TokenKind::Ampersand
                }
            }
            '|' => {
                if self.scanner.peek() == Some('|') {
                    self.scanner.advance();
                    TokenKind::PipePipe
                } else {
                    TokenKind::Pipe
                }
            }
            '^' => TokenKind::Caret,
            '~' => TokenKind::Tilde,
            '?' => TokenKind::Question,
            '@' => {
                // Check for kāraka markers
                return Some(self.scan_karaka_marker());
            }
            '#' => TokenKind::Hash,
            '$' => TokenKind::Dollar,

            // String literals
            '"' => return Some(self.scan_string()),
            '\'' => return Some(self.scan_char()),

            // Numbers
            '0'..='9' => return Some(self.scan_number(ch)),

            // Identifiers and keywords
            _ if Scanner::is_sanskrit_letter(ch) || ch == '_' => {
                return Some(self.scan_identifier(ch));
            }

            // Unknown character - create error token
            _ => {
                return Some(Token {
                    kind: TokenKind::Error(format!("Unexpected character: '{}'", ch)),
                    span: self.scanner.token_span(),
                    lexeme: ch.to_string(),
                });
            }
        };

        Some(Token {
            kind,
            span: self.scanner.token_span(),
            lexeme: self.scanner.token_text().to_string(),
        })
    }

    /// Scan a string literal
    fn scan_string(&mut self) -> Token {
        let mut value = String::new();

        while let Some(ch) = self.scanner.peek() {
            if ch == '"' {
                self.scanner.advance();
                break;
            } else if ch == '\\' {
                self.scanner.advance();
                if let Some(escaped) = self.scanner.advance() {
                    value.push(match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '"' => '"',
                        '0' => '\0',
                        _ => escaped,
                    });
                }
            } else if ch == '\n' {
                // Unterminated string
                break;
            } else {
                self.scanner.advance();
                value.push(ch);
            }
        }

        Token {
            kind: TokenKind::StringLiteral(value),
            span: self.scanner.token_span(),
            lexeme: self.scanner.token_text().to_string(),
        }
    }

    /// Scan a character literal
    fn scan_char(&mut self) -> Token {
        let ch = if self.scanner.peek() == Some('\\') {
            self.scanner.advance();
            match self.scanner.advance() {
                Some('n') => '\n',
                Some('t') => '\t',
                Some('r') => '\r',
                Some('\\') => '\\',
                Some('\'') => '\'',
                Some('0') => '\0',
                Some(c) => c,
                None => '\0',
            }
        } else {
            self.scanner.advance().unwrap_or('\0')
        };

        // Expect closing quote
        if self.scanner.peek() == Some('\'') {
            self.scanner.advance();
        }

        Token {
            kind: TokenKind::IntLiteral(ch as i64),
            span: self.scanner.token_span(),
            lexeme: self.scanner.token_text().to_string(),
        }
    }

    /// Scan a number literal
    fn scan_number(&mut self, first: char) -> Token {
        let mut is_float = false;

        // Check for hex/binary/octal
        if first == '0' {
            match self.scanner.peek() {
                Some('x') | Some('X') => {
                    self.scanner.advance();
                    return self.scan_hex_number();
                }
                Some('b') | Some('B') => {
                    self.scanner.advance();
                    return self.scan_binary_number();
                }
                Some('o') | Some('O') => {
                    self.scanner.advance();
                    return self.scan_octal_number();
                }
                _ => {}
            }
        }

        // Scan integer part
        while let Some(ch) = self.scanner.peek() {
            if ch.is_ascii_digit() || ch == '_' {
                self.scanner.advance();
            } else {
                break;
            }
        }

        // Check for decimal point
        if self.scanner.peek() == Some('.') {
            if let Some(next) = self.scanner.peek_next() {
                if next.is_ascii_digit() {
                    is_float = true;
                    self.scanner.advance(); // .
                    while let Some(ch) = self.scanner.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            self.scanner.advance();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        // Check for exponent
        if let Some('e') | Some('E') = self.scanner.peek() {
            is_float = true;
            self.scanner.advance();
            if let Some('+') | Some('-') = self.scanner.peek() {
                self.scanner.advance();
            }
            while let Some(ch) = self.scanner.peek() {
                if ch.is_ascii_digit() {
                    self.scanner.advance();
                } else {
                    break;
                }
            }
        }

        let text = self.scanner.token_text().replace('_', "");

        if is_float {
            let value: f64 = text.parse().unwrap_or(0.0);
            Token {
                kind: TokenKind::FloatLiteral(value),
                span: self.scanner.token_span(),
                lexeme: self.scanner.token_text().to_string(),
            }
        } else {
            let value: i64 = text.parse().unwrap_or(0);
            Token {
                kind: TokenKind::IntLiteral(value),
                span: self.scanner.token_span(),
                lexeme: self.scanner.token_text().to_string(),
            }
        }
    }

    fn scan_hex_number(&mut self) -> Token {
        while let Some(ch) = self.scanner.peek() {
            if ch.is_ascii_hexdigit() || ch == '_' {
                self.scanner.advance();
            } else {
                break;
            }
        }
        let text = self.scanner.token_text();
        let hex_part = &text[2..].replace('_', "");
        let value = i64::from_str_radix(hex_part, 16).unwrap_or(0);
        Token {
            kind: TokenKind::IntLiteral(value),
            span: self.scanner.token_span(),
            lexeme: text.to_string(),
        }
    }

    fn scan_binary_number(&mut self) -> Token {
        while let Some(ch) = self.scanner.peek() {
            if ch == '0' || ch == '1' || ch == '_' {
                self.scanner.advance();
            } else {
                break;
            }
        }
        let text = self.scanner.token_text();
        let bin_part = &text[2..].replace('_', "");
        let value = i64::from_str_radix(bin_part, 2).unwrap_or(0);
        Token {
            kind: TokenKind::IntLiteral(value),
            span: self.scanner.token_span(),
            lexeme: text.to_string(),
        }
    }

    fn scan_octal_number(&mut self) -> Token {
        while let Some(ch) = self.scanner.peek() {
            if ('0'..='7').contains(&ch) || ch == '_' {
                self.scanner.advance();
            } else {
                break;
            }
        }
        let text = self.scanner.token_text();
        let oct_part = &text[2..].replace('_', "");
        let value = i64::from_str_radix(oct_part, 8).unwrap_or(0);
        Token {
            kind: TokenKind::IntLiteral(value),
            span: self.scanner.token_span(),
            lexeme: text.to_string(),
        }
    }

    /// Scan kāraka marker (@kartṛ, @karman, etc.)
    fn scan_karaka_marker(&mut self) -> Token {
        // Scan the identifier after @
        while let Some(ch) = self.scanner.peek() {
            if Scanner::is_identifier_char(ch) {
                self.scanner.advance();
            } else {
                break;
            }
        }

        let text = self.scanner.token_text();
        let marker_name = &text[1..]; // Skip the @

        let kind = match marker_name {
            "kartṛ" | "kartr" | "agent" => TokenKind::KarakaKartr,
            "karman" | "patient" | "object" => TokenKind::KarakaKarman,
            "karaṇa" | "karana" | "instrument" => TokenKind::KarakaKarana,
            "sampradāna" | "sampradana" | "recipient" => TokenKind::KarakaSampradana,
            "apādāna" | "apadana" | "source" => TokenKind::KarakaApadana,
            "adhikaraṇa" | "adhikarana" | "location" => TokenKind::KarakaAdhikarana,
            _ => TokenKind::At, // Just @ if not recognized
        };

        Token {
            kind,
            span: self.scanner.token_span(),
            lexeme: text.to_string(),
        }
    }

    /// Scan an identifier or keyword
    fn scan_identifier(&mut self, first: char) -> Token {
        while let Some(ch) = self.scanner.peek() {
            if Scanner::is_identifier_char(ch) {
                self.scanner.advance();
            } else {
                break;
            }
        }

        let text = self.scanner.token_text();
        let kind = self.keyword_or_identifier(text);

        Token {
            kind,
            span: self.scanner.token_span(),
            lexeme: text.to_string(),
        }
    }

    /// Check if identifier is a keyword
    fn keyword_or_identifier(&self, text: &str) -> TokenKind {
        match text {
            // Sanskrit keywords
            "kāryakrama" | "karyakrama" | "fn" | "func" => TokenKind::Karyakrama,
            "prakāra" | "prakara" | "type" | "struct" => TokenKind::Prakara,
            "yad" | "yadi" | "if" => TokenKind::Yad,
            "anyathā" | "anyatha" | "else" => TokenKind::Anyatha,
            "cala" | "while" | "loop" => TokenKind::Cala,
            "phera" | "return" => TokenKind::Phera,
            "nirmā" | "nirma" | "new" => TokenKind::Nirma,
            "mukta" | "free" | "drop" => TokenKind::Mukta,
            "paṭha" | "patha" | "read" => TokenKind::Patha,
            "likha" | "write" => TokenKind::Likha,
            "mudraṇa" | "mudrana" | "print" => TokenKind::Mudrana,
            "pradhāna" | "pradhana" | "main" => TokenKind::Pradhana,
            "āśaya" | "ashaya" | "async" => TokenKind::Ashaya,
            "pratīkṣā" | "pratiksha" | "await" => TokenKind::Pratiksha,
            "saha" | "concurrent" | "par" => TokenKind::Saha,

            // Boolean literals
            "satya" | "true" => TokenKind::BoolLiteral(true),
            "asatya" | "false" => TokenKind::BoolLiteral(false),

            // Type keywords
            "saṅkhyā" | "sankhya" | "int" => TokenKind::Sankhya,
            "sūtra" | "sutra" | "string" | "str" => TokenKind::Sutra,
            "sūci" | "suci" | "vec" | "list" => TokenKind::Suci,
            "sāraṇī" | "sarani" | "map" => TokenKind::Sarani,
            "vikalpa" | "option" => TokenKind::Vikalpa,
            "phala" | "result" => TokenKind::Phala,

            // Control flow
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "match" => TokenKind::Match,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,

            // Modifiers
            "pub" | "sārvajanika" => TokenKind::Pub,
            "mut" | "parivartya" => TokenKind::Mut,
            "const" | "sthira" => TokenKind::Const,
            "let" | "mānaya" | "māna" => TokenKind::Let,
            "static" => TokenKind::Static,

            // Module system
            "mod" | "vibhāga" => TokenKind::Mod,
            "use" | "upayoga" => TokenKind::Use,
            "impl" | "kriyānvaya" => TokenKind::Impl,
            "trait" | "guṇa" => TokenKind::Trait,
            "self" | "sva" => TokenKind::SelfValue,
            "Self" | "Sva" => TokenKind::SelfType,
            "super" | "uttara" => TokenKind::Super,
            "crate" | "saṃkula" => TokenKind::Crate,

            // Other
            "as" => TokenKind::As,
            "ref" => TokenKind::Ref,
            "unsafe" | "asuraksita" => TokenKind::Unsafe,
            "extern" | "bāhya" => TokenKind::Extern,
            "where" | "yatra" => TokenKind::Where,

            // Default: identifier
            _ => TokenKind::Identifier(text.to_string()),
        }
    }
}

// Add missing token kinds
impl TokenKind {
    /// Check if this is a keyword
    pub fn is_keyword(&self) -> bool {
        !matches!(
            self,
            TokenKind::Identifier(_)
                | TokenKind::IntLiteral(_)
                | TokenKind::FloatLiteral(_)
                | TokenKind::StringLiteral(_)
                | TokenKind::BoolLiteral(_)
        )
    }
}

/// Iterator implementation for Lexer
impl<'src> Iterator for Lexer<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip whitespace
        self.skip_whitespace_and_comments();

        if self.scanner.is_eof() {
            return None;
        }

        // Scan one token
        self.scanner.start_token();
        self.scan_token()
    }
}
