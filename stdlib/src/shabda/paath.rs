//! # Paath - Text Utilities (पाठ)
//!
//! General text processing utilities.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Case conversion types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AksharPrakara {
    /// lowercase
    ChhotaAkshar,
    /// UPPERCASE
    BadaAkshar,
    /// Title Case
    ShirshakAkshar,
    /// camelCase
    UshtraPrakara,
    /// snake_case
    SarpaPrakara,
    /// kebab-case
    DandaPrakara,
    /// PascalCase
    PascalPrakara,
    /// SCREAMING_SNAKE
    ChillaPrakara,
}

/// Check if char is whitespace
pub fn shwet_sthaan(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r')
}

/// Check if char is digit
pub fn ank_hai(c: char) -> bool {
    c >= '0' && c <= '9'
}

/// Check if char is letter (ASCII)
pub fn akshar_hai(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

/// Check if char is alphanumeric
pub fn aksharank_hai(c: char) -> bool {
    akshar_hai(c) || ank_hai(c)
}

/// Check if uppercase
pub fn bada_hai(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

/// Check if lowercase
pub fn chhota_hai(c: char) -> bool {
    c >= 'a' && c <= 'z'
}

/// Convert to lowercase
pub fn chhota_karo(c: char) -> char {
    if bada_hai(c) {
        (c as u8 + 32) as char
    } else {
        c
    }
}

/// Convert to uppercase
pub fn bada_karo(c: char) -> char {
    if chhota_hai(c) {
        (c as u8 - 32) as char
    } else {
        c
    }
}

/// Check if string is empty or whitespace
#[cfg(feature = "alloc")]
pub fn khali_hai(s: &str) -> bool {
    s.chars().all(shwet_sthaan)
}

/// Trim whitespace from both ends
pub fn chhanto<'a>(s: &'a str) -> &'a str {
    let start = s.find(|c| !shwet_sthaan(c)).unwrap_or(s.len());
    let end = s.rfind(|c| !shwet_sthaan(c)).map(|i| i + 1).unwrap_or(0);
    if start >= end {
        ""
    } else {
        &s[start..end]
    }
}

/// Trim from start
pub fn chhanto_shuru<'a>(s: &'a str) -> &'a str {
    let start = s.find(|c| !shwet_sthaan(c)).unwrap_or(s.len());
    &s[start..]
}

/// Trim from end
pub fn chhanto_ant<'a>(s: &'a str) -> &'a str {
    let end = s.rfind(|c| !shwet_sthaan(c)).map(|i| i + 1).unwrap_or(0);
    &s[..end]
}

/// Convert to lowercase
#[cfg(feature = "alloc")]
pub fn chhota_paath(s: &str) -> String {
    s.chars().map(chhota_karo).collect()
}

/// Convert to uppercase
#[cfg(feature = "alloc")]
pub fn bada_paath(s: &str) -> String {
    s.chars().map(bada_karo).collect()
}

/// Capitalize first letter
#[cfg(feature = "alloc")]
pub fn pratham_bada(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let mut result = String::new();
            result.push(bada_karo(first));
            for c in chars {
                result.push(chhota_karo(c));
            }
            result
        }
    }
}

/// Title case
#[cfg(feature = "alloc")]
pub fn shirshak_paath(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut capitalize_next = true;

    for c in s.chars() {
        if shwet_sthaan(c) {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
            result.push(bada_karo(c));
            capitalize_next = false;
        } else {
            result.push(chhota_karo(c));
        }
    }

    result
}

/// Convert to camelCase
#[cfg(feature = "alloc")]
pub fn ushtra_paath(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut capitalize_next = false;

    for c in s.chars() {
        if c == '_' || c == '-' || c == ' ' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(bada_karo(c));
            capitalize_next = false;
        } else {
            result.push(chhota_karo(c));
        }
    }

    result
}

/// Convert to PascalCase
#[cfg(feature = "alloc")]
pub fn pascal_paath(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' || c == '-' || c == ' ' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(bada_karo(c));
            capitalize_next = false;
        } else {
            result.push(chhota_karo(c));
        }
    }

    result
}

/// Convert to snake_case
#[cfg(feature = "alloc")]
pub fn sarpa_paath(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 5);
    let mut prev_lower = false;

    for c in s.chars() {
        if c == '-' || c == ' ' {
            result.push('_');
            prev_lower = false;
        } else if bada_hai(c) {
            if prev_lower {
                result.push('_');
            }
            result.push(chhota_karo(c));
            prev_lower = false;
        } else {
            result.push(c);
            prev_lower = chhota_hai(c);
        }
    }

    result
}

/// Convert to kebab-case
#[cfg(feature = "alloc")]
pub fn danda_paath(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 5);
    let mut prev_lower = false;

    for c in s.chars() {
        if c == '_' || c == ' ' {
            result.push('-');
            prev_lower = false;
        } else if bada_hai(c) {
            if prev_lower {
                result.push('-');
            }
            result.push(chhota_karo(c));
            prev_lower = false;
        } else {
            result.push(c);
            prev_lower = chhota_hai(c);
        }
    }

    result
}

/// Reverse string
#[cfg(feature = "alloc")]
pub fn ulat(s: &str) -> String {
    s.chars().rev().collect()
}

/// Repeat string n times
#[cfg(feature = "alloc")]
pub fn dohrao(s: &str, n: usize) -> String {
    s.repeat(n)
}

/// Pad left to width
#[cfg(feature = "alloc")]
pub fn vaama_pad(s: &str, width: usize, pad_char: char) -> String {
    let len = s.chars().count();
    if len >= width {
        s.to_string()
    } else {
        let padding: String = core::iter::repeat(pad_char).take(width - len).collect();
        padding + s
    }
}

/// Pad right to width
#[cfg(feature = "alloc")]
pub fn dakshina_pad(s: &str, width: usize, pad_char: char) -> String {
    let len = s.chars().count();
    if len >= width {
        s.to_string()
    } else {
        let padding: String = core::iter::repeat(pad_char).take(width - len).collect();
        s.to_string() + &padding
    }
}

/// Center string
#[cfg(feature = "alloc")]
pub fn kendra_pad(s: &str, width: usize, pad_char: char) -> String {
    let len = s.chars().count();
    if len >= width {
        s.to_string()
    } else {
        let total_pad = width - len;
        let left_pad = total_pad / 2;
        let right_pad = total_pad - left_pad;

        let left: String = core::iter::repeat(pad_char).take(left_pad).collect();
        let right: String = core::iter::repeat(pad_char).take(right_pad).collect();

        left + s + &right
    }
}

/// Word count
pub fn shabd_ginti(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Line count
pub fn panki_ginti(s: &str) -> usize {
    if s.is_empty() {
        0
    } else {
        s.lines().count()
    }
}

/// Character count (excluding whitespace)
pub fn akshar_ginti(s: &str) -> usize {
    s.chars().filter(|c| !shwet_sthaan(*c)).count()
}

/// Check if palindrome
pub fn palindrome_hai(s: &str) -> bool {
    let chars: Vec<char> = s
        .chars()
        .filter(|c| aksharank_hai(*c))
        .map(chhota_karo)
        .collect();

    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

/// Check if anagram
#[cfg(feature = "alloc")]
pub fn anagram_hai(a: &str, b: &str) -> bool {
    let mut chars_a: Vec<char> = a
        .chars()
        .filter(|c| aksharank_hai(*c))
        .map(chhota_karo)
        .collect();
    let mut chars_b: Vec<char> = b
        .chars()
        .filter(|c| aksharank_hai(*c))
        .map(chhota_karo)
        .collect();

    chars_a.sort();
    chars_b.sort();

    chars_a == chars_b
}

/// Wrap text to width
#[cfg(feature = "alloc")]
pub fn lapetao(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + 1 + word.len() <= width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

/// Truncate with ellipsis
#[cfg(feature = "alloc")]
pub fn kato(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        s.chars().take(max_len).collect()
    } else {
        let truncated: String = s.chars().take(max_len - 3).collect();
        truncated + "..."
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_conversions() {
        assert!(bada_hai('A'));
        assert!(!bada_hai('a'));
        assert_eq!(bada_karo('a'), 'A');
        assert_eq!(chhota_karo('A'), 'a');
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_snake_case() {
        assert_eq!(sarpa_paath("camelCase"), "camel_case");
        assert_eq!(sarpa_paath("PascalCase"), "pascal_case");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_camel_case() {
        assert_eq!(ushtra_paath("snake_case"), "snakeCase");
        assert_eq!(ushtra_paath("kebab-case"), "kebabCase");
    }

    #[test]
    fn test_palindrome() {
        assert!(palindrome_hai("racecar"));
        assert!(palindrome_hai("A man a plan a canal Panama"));
        assert!(!palindrome_hai("hello"));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_anagram() {
        assert!(anagram_hai("listen", "silent"));
        assert!(!anagram_hai("hello", "world"));
    }
}
