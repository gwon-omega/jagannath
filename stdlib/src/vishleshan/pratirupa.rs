//! # Pratirupa - Pattern Matching (प्रतिरूप)
//!
//! Simple pattern matching without full regex complexity.
//!
//! > **"प्रतिरूपं सादृश्यम्"**
//! > *"Pattern is similarity"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// WILDCARD PATTERN
// ============================================================================

/// Simple wildcard pattern (* and ?)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct JokarPratirupa {
    pattern: String,
}

#[cfg(feature = "alloc")]
impl JokarPratirupa {
    /// Create new wildcard pattern
    /// - `*` matches zero or more characters
    /// - `?` matches exactly one character
    pub fn nava(pattern: impl Into<String>) -> Self {
        Self {
            pattern: pattern.into(),
        }
    }

    /// Check if string matches pattern
    pub fn milana(&self, text: &str) -> bool {
        wildcard_match(&self.pattern, text)
    }
}

/// Wildcard matching algorithm
#[cfg(feature = "alloc")]
fn wildcard_match(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();

    let mut dp = vec![vec![false; t.len() + 1]; p.len() + 1];
    dp[0][0] = true;

    // Handle leading *
    for i in 1..=p.len() {
        if p[i - 1] == '*' {
            dp[i][0] = dp[i - 1][0];
        }
    }

    for i in 1..=p.len() {
        for j in 1..=t.len() {
            if p[i - 1] == '*' {
                dp[i][j] = dp[i - 1][j] || dp[i][j - 1];
            } else if p[i - 1] == '?' || p[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }

    dp[p.len()][t.len()]
}

// ============================================================================
// CHARACTER CLASSES
// ============================================================================

/// Character class matcher
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub enum AksharVarga {
    /// Any letter [a-zA-Z]
    Akshara,
    /// Any digit [0-9]
    Anka,
    /// Alphanumeric [a-zA-Z0-9]
    AksharAnka,
    /// Whitespace
    Shvetasthana,
    /// Word character [a-zA-Z0-9_]
    ShabdaAkshara,
    /// Any of given characters
    KoiBhi(Vec<char>),
    /// Range of characters
    Sima(char, char),
    /// Negated class
    Niṣedha(Box<AksharVarga>),
}

#[cfg(feature = "alloc")]
impl AksharVarga {
    /// Check if character matches class
    pub fn milana(&self, c: char) -> bool {
        match self {
            AksharVarga::Akshara => c.is_alphabetic(),
            AksharVarga::Anka => c.is_ascii_digit(),
            AksharVarga::AksharAnka => c.is_alphanumeric(),
            AksharVarga::Shvetasthana => c.is_whitespace(),
            AksharVarga::ShabdaAkshara => c.is_alphanumeric() || c == '_',
            AksharVarga::KoiBhi(chars) => chars.contains(&c),
            AksharVarga::Sima(start, end) => c >= *start && c <= *end,
            AksharVarga::Niṣedha(inner) => !inner.milana(c),
        }
    }
}

// ============================================================================
// STRING SEARCH
// ============================================================================

/// Find all occurrences of substring
#[cfg(feature = "alloc")]
pub fn sabhi_khoja(text: &str, pattern: &str) -> Vec<usize> {
    if pattern.is_empty() {
        return Vec::new();
    }

    let mut results = Vec::new();
    let mut start = 0;

    while let Some(pos) = text[start..].find(pattern) {
        results.push(start + pos);
        start += pos + 1;
    }

    results
}

/// Count occurrences
#[cfg(feature = "alloc")]
pub fn ganana(text: &str, pattern: &str) -> usize {
    if pattern.is_empty() {
        return 0;
    }
    text.matches(pattern).count()
}

/// Replace all occurrences
#[cfg(feature = "alloc")]
pub fn sabhi_badalna(text: &str, khoja: &str, pratisthapana: &str) -> String {
    text.replace(khoja, pratisthapana)
}

/// Replace first occurrence
#[cfg(feature = "alloc")]
pub fn prathama_badalna(text: &str, khoja: &str, pratisthapana: &str) -> String {
    text.replacen(khoja, pratisthapana, 1)
}

// ============================================================================
// CASE-INSENSITIVE MATCHING
// ============================================================================

/// Case-insensitive contains
pub fn asahishnuta_dhaarana(text: &str, pattern: &str) -> bool {
    text.to_lowercase().contains(&pattern.to_lowercase())
}

/// Case-insensitive equals
pub fn asahishnuta_samana(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

/// Case-insensitive find
#[cfg(feature = "alloc")]
pub fn asahishnuta_khoja(text: &str, pattern: &str) -> Option<usize> {
    let text_lower = text.to_lowercase();
    let pattern_lower = pattern.to_lowercase();
    text_lower.find(&pattern_lower)
}

// ============================================================================
// SPLIT PATTERNS
// ============================================================================

/// Split by multiple delimiters
#[cfg(feature = "alloc")]
pub fn bahu_vibhajana<'a>(text: &'a str, delimiters: &[char]) -> Vec<&'a str> {
    text.split(|c| delimiters.contains(&c))
        .filter(|s| !s.is_empty())
        .collect()
}

/// Split and keep delimiters
#[cfg(feature = "alloc")]
pub fn vibhajana_rakha(text: &str, delimiter: char) -> Vec<&str> {
    let mut result = Vec::new();
    let mut last = 0;

    for (idx, c) in text.char_indices() {
        if c == delimiter {
            if last < idx {
                result.push(&text[last..idx]);
            }
            result.push(&text[idx..idx + c.len_utf8()]);
            last = idx + c.len_utf8();
        }
    }

    if last < text.len() {
        result.push(&text[last..]);
    }

    result
}

/// Split into lines
#[cfg(feature = "alloc")]
pub fn pankti_vibhajana(text: &str) -> Vec<&str> {
    text.lines().collect()
}

/// Split into words
#[cfg(feature = "alloc")]
pub fn shabda_vibhajana(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}

// ============================================================================
// PREFIX/SUFFIX
// ============================================================================

/// Check if starts with any of
#[cfg(feature = "alloc")]
pub fn koi_arambha(text: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|p| text.starts_with(p))
}

/// Check if ends with any of
#[cfg(feature = "alloc")]
pub fn koi_anta(text: &str, suffixes: &[&str]) -> bool {
    suffixes.iter().any(|s| text.ends_with(s))
}

/// Strip prefix if present
pub fn purvapad_chhilna<'a>(text: &'a str, prefix: &str) -> &'a str {
    text.strip_prefix(prefix).unwrap_or(text)
}

/// Strip suffix if present
pub fn uttarapad_chhilna<'a>(text: &'a str, suffix: &str) -> &'a str {
    text.strip_suffix(suffix).unwrap_or(text)
}

// ============================================================================
// EXTRACT PATTERNS
// ============================================================================

/// Extract text between delimiters
#[cfg(feature = "alloc")]
pub fn madhya_nikalna<'a>(text: &'a str, start: &str, end: &str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let mut remaining = text;

    while let Some(start_pos) = remaining.find(start) {
        let after_start = &remaining[start_pos + start.len()..];
        if let Some(end_pos) = after_start.find(end) {
            results.push(&after_start[..end_pos]);
            remaining = &after_start[end_pos + end.len()..];
        } else {
            break;
        }
    }

    results
}

/// Extract all numbers from string
#[cfg(feature = "alloc")]
pub fn sankhya_nikalna(text: &str) -> Vec<i64> {
    let mut results = Vec::new();
    let mut current = String::new();
    let mut in_number = false;
    let mut is_negative = false;

    for c in text.chars() {
        if c.is_ascii_digit() {
            in_number = true;
            current.push(c);
        } else if c == '-' && !in_number && current.is_empty() {
            is_negative = true;
        } else if in_number {
            if let Ok(n) = current.parse::<i64>() {
                results.push(if is_negative { -n } else { n });
            }
            current.clear();
            in_number = false;
            is_negative = false;
        } else {
            is_negative = false;
        }
    }

    if in_number {
        if let Ok(n) = current.parse::<i64>() {
            results.push(if is_negative { -n } else { n });
        }
    }

    results
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_wildcard_exact() {
        let p = JokarPratirupa::nava("hello");
        assert!(p.milana("hello"));
        assert!(!p.milana("world"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_wildcard_star() {
        let p = JokarPratirupa::nava("*.txt");
        assert!(p.milana("file.txt"));
        assert!(p.milana(".txt"));
        assert!(!p.milana("file.pdf"));

        let p2 = JokarPratirupa::nava("hello*");
        assert!(p2.milana("hello"));
        assert!(p2.milana("hello world"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_wildcard_question() {
        let p = JokarPratirupa::nava("?.txt");
        assert!(p.milana("a.txt"));
        assert!(!p.milana("ab.txt"));
        assert!(!p.milana(".txt"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_sabhi_khoja() {
        let text = "ababab";
        let results = sabhi_khoja(text, "ab");
        assert_eq!(results, vec![0, 2, 4]);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_case_insensitive() {
        assert!(asahishnuta_dhaarana("Hello World", "WORLD"));
        assert!(asahishnuta_samana("Hello", "HELLO"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_madhya_nikalna() {
        let text = "<a>hello</a><b>world</b>";
        let results = madhya_nikalna(text, "<a>", "</a>");
        assert_eq!(results, vec!["hello"]);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_sankhya_nikalna() {
        let text = "abc 123 def -456 ghi 789";
        let numbers = sankhya_nikalna(text);
        assert_eq!(numbers, vec![123, -456, 789]);
    }
}
