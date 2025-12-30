//! # Sankhyapan - Tokenization (सांख्यपन)
//!
//! Text tokenization and parsing utilities.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Token type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenPrakara {
    /// Word
    Shabd,
    /// Number
    Ank,
    /// Punctuation
    Virama,
    /// Whitespace
    ShwetSthaan,
    /// Symbol
    Chinh,
    /// Unknown
    Agyaat,
}

/// A token with position
#[derive(Debug, Clone)]
pub struct Token<'a> {
    /// Token text
    pub paath: &'a str,
    /// Token type
    pub prakara: TokenPrakara,
    /// Start position
    pub shuru: usize,
    /// End position
    pub ant: usize,
}

impl<'a> Token<'a> {
    pub fn naya(paath: &'a str, prakara: TokenPrakara, shuru: usize, ant: usize) -> Self {
        Self {
            paath,
            prakara,
            shuru,
            ant,
        }
    }
}

/// Check if punctuation
fn virama_hai(c: char) -> bool {
    matches!(
        c,
        '.' | ',' | '!' | '?' | ':' | ';' | '\'' | '"' | '(' | ')' | '[' | ']' | '{' | '}'
    )
}

/// Check if symbol
fn chinh_hai(c: char) -> bool {
    matches!(
        c,
        '@' | '#'
            | '$'
            | '%'
            | '^'
            | '&'
            | '*'
            | '+'
            | '-'
            | '='
            | '<'
            | '>'
            | '/'
            | '\\'
            | '|'
            | '~'
            | '`'
    )
}

/// Simple word tokenizer
#[cfg(feature = "alloc")]
pub fn shabd_token(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}

/// Full tokenizer with types
#[cfg(feature = "alloc")]
pub fn purn_token(text: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut pos = 0;
    let chars: Vec<char> = text.chars().collect();

    while pos < chars.len() {
        let start = pos;
        let c = chars[pos];

        let prakara = if c.is_whitespace() {
            while pos < chars.len() && chars[pos].is_whitespace() {
                pos += 1;
            }
            TokenPrakara::ShwetSthaan
        } else if c.is_alphabetic() || c == '_' {
            while pos < chars.len() && (chars[pos].is_alphanumeric() || chars[pos] == '_') {
                pos += 1;
            }
            TokenPrakara::Shabd
        } else if c.is_numeric()
            || (c == '-' && pos + 1 < chars.len() && chars[pos + 1].is_numeric())
        {
            if c == '-' {
                pos += 1;
            }
            while pos < chars.len() && (chars[pos].is_numeric() || chars[pos] == '.') {
                pos += 1;
            }
            TokenPrakara::Ank
        } else if virama_hai(c) {
            pos += 1;
            TokenPrakara::Virama
        } else if chinh_hai(c) {
            pos += 1;
            TokenPrakara::Chinh
        } else {
            pos += 1;
            TokenPrakara::Agyaat
        };

        // Calculate byte positions
        let byte_start: usize = chars[..start].iter().map(|c| c.len_utf8()).sum();
        let byte_end: usize = chars[..pos].iter().map(|c| c.len_utf8()).sum();

        tokens.push(Token::naya(
            &text[byte_start..byte_end],
            prakara,
            byte_start,
            byte_end,
        ));
    }

    tokens
}

/// Sentence splitter
#[cfg(feature = "alloc")]
pub fn vakya_vibhajan(text: &str) -> Vec<&str> {
    let mut sentences = Vec::new();
    let mut start = 0;
    let bytes = text.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if b == b'.' || b == b'!' || b == b'?' {
            // Check for end of sentence
            let is_end = if i + 1 < bytes.len() {
                let next = bytes[i + 1];
                next == b' ' || next == b'\n' || next == b'\r'
            } else {
                true
            };

            if is_end {
                let sentence = &text[start..=i];
                let trimmed = sentence.trim();
                if !trimmed.is_empty() {
                    sentences.push(trimmed);
                }
                start = i + 1;
            }
        }
    }

    // Handle remaining text
    let remaining = text[start..].trim();
    if !remaining.is_empty() {
        sentences.push(remaining);
    }

    sentences
}

/// N-gram generator
#[cfg(feature = "alloc")]
pub fn n_gram(text: &str, n: usize) -> Vec<&str> {
    if n == 0 {
        return Vec::new();
    }

    let bytes = text.as_bytes();
    if bytes.len() < n {
        return Vec::new();
    }

    (0..=bytes.len() - n).map(|i| &text[i..i + n]).collect()
}

/// Word n-grams
#[cfg(feature = "alloc")]
pub fn shabd_n_gram(text: &str, n: usize) -> Vec<Vec<&str>> {
    if n == 0 {
        return Vec::new();
    }

    let words: Vec<&str> = text.split_whitespace().collect();
    if words.len() < n {
        return Vec::new();
    }

    (0..=words.len() - n)
        .map(|i| words[i..i + n].to_vec())
        .collect()
}

/// Paragraph splitter
#[cfg(feature = "alloc")]
pub fn anuched_vibhajan(text: &str) -> Vec<&str> {
    text.split("\n\n")
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect()
}

/// CSV parser (simple)
#[cfg(feature = "alloc")]
pub fn csv_padho(line: &str, delimiter: char) -> Vec<&str> {
    let mut fields = Vec::new();
    let mut start = 0;
    let mut in_quotes = false;

    for (i, c) in line.char_indices() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == delimiter && !in_quotes {
            fields.push(line[start..i].trim());
            start = i + 1;
        }
    }

    fields.push(line[start..].trim());
    fields
}

/// Word frequency counter
#[cfg(feature = "alloc")]
pub fn shabd_aavriti(text: &str) -> Vec<(&str, usize)> {
    let mut counts: Vec<(&str, usize)> = Vec::new();

    for word in text.split_whitespace() {
        let word_lower = word;
        if let Some(entry) = counts.iter_mut().find(|(w, _)| *w == word_lower) {
            entry.1 += 1;
        } else {
            counts.push((word, 1));
        }
    }

    counts.sort_by(|a, b| b.1.cmp(&a.1));
    counts
}

/// Extract numbers from text
#[cfg(feature = "alloc")]
pub fn ank_nikaalo(text: &str) -> Vec<f64> {
    let mut numbers = Vec::new();
    let mut current = String::new();
    let mut has_decimal = false;
    let mut has_sign = false;

    for c in text.chars() {
        if c.is_numeric() {
            current.push(c);
        } else if c == '.' && !has_decimal && !current.is_empty() {
            current.push(c);
            has_decimal = true;
        } else if (c == '-' || c == '+') && current.is_empty() && !has_sign {
            current.push(c);
            has_sign = true;
        } else if !current.is_empty() && current != "-" && current != "+" {
            if let Ok(num) = current.parse::<f64>() {
                numbers.push(num);
            }
            current.clear();
            has_decimal = false;
            has_sign = false;
        } else {
            current.clear();
            has_decimal = false;
            has_sign = false;
        }
    }

    if !current.is_empty() && current != "-" && current != "+" {
        if let Ok(num) = current.parse::<f64>() {
            numbers.push(num);
        }
    }

    numbers
}

/// Extract words matching pattern
#[cfg(feature = "alloc")]
pub fn shabd_nikaalo<F>(text: &str, predicate: F) -> Vec<&str>
where
    F: Fn(&str) -> bool,
{
    text.split_whitespace().filter(|w| predicate(w)).collect()
}

/// Remove stop words
#[cfg(feature = "alloc")]
pub fn stop_shabd_hatao<'a>(text: &'a str, stop_words: &[&str]) -> Vec<&'a str> {
    text.split_whitespace()
        .filter(|word| {
            let lower: String = word
                .chars()
                .map(|c| {
                    if c >= 'A' && c <= 'Z' {
                        (c as u8 + 32) as char
                    } else {
                        c
                    }
                })
                .collect();
            !stop_words.iter().any(|sw| sw.eq_ignore_ascii_case(&lower))
        })
        .collect()
}

/// Common English stop words
pub const ANGREZI_STOP_SHABD: &[&str] = &[
    "a", "an", "the", "and", "or", "but", "is", "are", "was", "were", "be", "been", "being",
    "have", "has", "had", "do", "does", "did", "will", "would", "could", "should", "may", "might",
    "must", "can", "this", "that", "these", "those", "i", "you", "he", "she", "it", "we", "they",
    "me", "him", "her", "us", "them", "my", "your", "his", "its", "our", "their", "what", "which",
    "who", "whom", "when", "where", "why", "how", "all", "each", "every", "both", "few", "more",
    "most", "other", "some", "such", "no", "not", "only", "same", "so", "than", "too", "very",
    "just", "also", "now", "here", "there", "then", "if", "for", "of", "to", "in", "on", "at",
    "by", "with", "about", "as", "into", "through", "during", "before", "after", "above", "below",
];

/// Stem word (simple suffix stripping)
#[cfg(feature = "alloc")]
pub fn mul_shabd(word: &str) -> String {
    let mut result = word.to_string();

    // Simple English suffix removal
    let suffixes = [
        "ing", "ed", "er", "est", "ly", "ness", "ment", "tion", "ous", "ive", "able", "ible",
    ];

    for suffix in suffixes {
        if result.len() > suffix.len() + 2 && result.ends_with(suffix) {
            result.truncate(result.len() - suffix.len());
            break;
        }
    }

    result
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_word_tokenize() {
        let tokens = shabd_token("hello world test");
        assert_eq!(tokens, vec!["hello", "world", "test"]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_full_tokenize() {
        let tokens = purn_token("hello 123!");
        assert_eq!(tokens.len(), 4); // hello, space, 123, !
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_sentence_split() {
        let sentences = vakya_vibhajan("Hello. World! How are you?");
        assert_eq!(sentences.len(), 3);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_n_gram() {
        let ngrams = n_gram("hello", 2);
        assert_eq!(ngrams, vec!["he", "el", "ll", "lo"]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_word_ngram() {
        let ngrams = shabd_n_gram("a b c d", 2);
        assert_eq!(ngrams.len(), 3);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_csv() {
        let fields = csv_padho("a,b,c", ',');
        assert_eq!(fields, vec!["a", "b", "c"]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_extract_numbers() {
        let nums = ank_nikaalo("price: $123.45, qty: 10");
        assert_eq!(nums, vec![123.45, 10.0]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_stem() {
        assert_eq!(mul_shabd("running"), "runn");
        assert_eq!(mul_shabd("walked"), "walk");
    }
}
