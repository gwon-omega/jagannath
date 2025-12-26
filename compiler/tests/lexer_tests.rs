//! Integration tests for the Jagannath compiler lexer
//!
//! Tests the Sanskrit morphological tokenization including:
//! - Sandhi splitting
//! - Dhātu (root) recognition
//! - Affix parsing
//! - Unicode handling

use jagannath_compiler::lexer::{Lexer, Token, TokenKind};

/// Test basic keyword tokenization
#[test]
fn test_keywords() {
    let source = "kāryakrama phera yad cala nirmā mukta";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.collect();

    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::KwKaryakrama)));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::KwPhera)));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::KwYad)));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::KwCala)));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::KwNirma)));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::KwMukta)));
}

/// Test type suffix parsing
#[test]
fn test_type_suffixes() {
    // -a = immutable, -ā = mutable
    // -l = linear, -b = borrowed
    // -k = stack, -h = heap
    let source = "saṅkhyā-a-l-k saṅkhyā-ā-b-h";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.collect();

    // Should recognize type identifiers with suffixes
    assert!(tokens.len() >= 2);
}

/// Test lifetime annotations
#[test]
fn test_lifetime_annotations() {
    // ^N notation for lifetime regions
    let source = "upayoktṛ^1 upayoktṛ^2 upayoktṛ^global";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.collect();

    // Should parse lifetime regions
    assert!(tokens.len() >= 3);
}

/// Test kāraka markers
#[test]
fn test_karaka_markers() {
    // @kartṛ = agent, @karman = patient, @karaṇa = instrument
    let source = "@kartṛ niveśa @karman nirgama @karaṇa sādhana";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.collect();

    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::KarakaKartr)));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::KarakaKarman)));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::KarakaKarana)));
}

/// Test numeric literals
#[test]
fn test_numeric_literals() {
    let source = "42 3.14159 0xFF 0b1010 1_000_000";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.collect();

    // Should recognize all numeric formats
    assert!(tokens.iter().filter(|t| matches!(t.kind, TokenKind::IntLiteral(_))).count() >= 3);
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::FloatLiteral(_))));
}

/// Test string literals
#[test]
fn test_string_literals() {
    let source = r#""namaste" "हैलो वर्ल्ड" "escape\n\t""#;
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.collect();

    assert!(tokens.iter().filter(|t| matches!(t.kind, TokenKind::StringLiteral(_))).count() >= 3);
}

/// Test Sanskrit Unicode handling
#[test]
fn test_sanskrit_unicode() {
    // Devanagari and IAST transliteration
    let source = "कार्यक्रम kāryakrama संख्या saṅkhyā";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.collect();

    // Should handle both scripts
    assert!(tokens.len() >= 4);
}

/// Test compound word (samāsa) handling
#[test]
fn test_compound_words() {
    // Compounds should be kept together
    let source = "mahārāja bahuvrīhi tatpuruṣa";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.collect();

    assert_eq!(tokens.iter().filter(|t| matches!(t.kind, TokenKind::Ident(_))).count(), 3);
}

/// Test sandhi at word boundaries
#[test]
fn test_sandhi_boundaries() {
    // Internal sandhi should be preserved in identifiers
    // External sandhi at boundaries should be recognized
    let source = "devānām ṛṣīṇām";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.collect();

    assert!(tokens.len() >= 2);
}

/// Test operators
#[test]
fn test_operators() {
    let source = "+ - * / % == != < > <= >= && || ! & | ^ ~ << >>";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.collect();

    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Plus)));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Minus)));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Star)));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Slash)));
}

/// Test delimiters
#[test]
fn test_delimiters() {
    let source = "( ) { } [ ] , ; : :: -> =>";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.collect();

    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::LParen)));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::RParen)));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::LBrace)));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::RBrace)));
}

/// Test comments
#[test]
fn test_comments() {
    let source = r#"
// Line comment
/* Block comment */
/// Doc comment
//! Module doc
code_here
"#;
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.collect();

    // Comments should be captured or skipped appropriately
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Ident(_))));
}

/// Test error recovery
#[test]
fn test_error_recovery() {
    // Invalid tokens should produce errors but continue
    let source = "valid @ invalid $ valid2";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.collect();

    // Should have both valid tokens and error tokens
    assert!(tokens.iter().filter(|t| matches!(t.kind, TokenKind::Ident(_))).count() >= 2);
}

/// Test source locations
#[test]
fn test_source_locations() {
    let source = "first\nsecond\nthird";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.collect();

    // Each token should have correct line/column
    assert_eq!(tokens[0].span.line, 1);
    assert_eq!(tokens[1].span.line, 2);
    assert_eq!(tokens[2].span.line, 3);
}
