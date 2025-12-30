//! # Pratirupa - Pattern Matching (प्रतिरूप)
//!
//! Pattern matching and string searching utilities.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Match result with position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MilaanPhala {
    /// Start position
    pub shuru: usize,
    /// End position (exclusive)
    pub ant: usize,
}

impl MilaanPhala {
    /// Length of match
    pub fn lambai(&self) -> usize {
        self.ant - self.shuru
    }
}

/// Simple pattern matching (naive)
pub fn saral_khoj(text: &str, pattern: &str) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }

    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();

    if pattern_bytes.len() > text_bytes.len() {
        return None;
    }

    for i in 0..=(text_bytes.len() - pattern_bytes.len()) {
        let mut matched = true;
        for j in 0..pattern_bytes.len() {
            if text_bytes[i + j] != pattern_bytes[j] {
                matched = false;
                break;
            }
        }
        if matched {
            return Some(i);
        }
    }

    None
}

/// Find all occurrences
#[cfg(feature = "alloc")]
pub fn sab_khoj(text: &str, pattern: &str) -> Vec<usize> {
    let mut positions = Vec::new();

    if pattern.is_empty() {
        return positions;
    }

    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();

    if pattern_bytes.len() > text_bytes.len() {
        return positions;
    }

    for i in 0..=(text_bytes.len() - pattern_bytes.len()) {
        let mut matched = true;
        for j in 0..pattern_bytes.len() {
            if text_bytes[i + j] != pattern_bytes[j] {
                matched = false;
                break;
            }
        }
        if matched {
            positions.push(i);
        }
    }

    positions
}

/// KMP failure function
#[cfg(feature = "alloc")]
fn kmp_asafalta(pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let mut failure = vec![0; m];
    let mut j = 0;

    for i in 1..m {
        while j > 0 && pattern[i] != pattern[j] {
            j = failure[j - 1];
        }
        if pattern[i] == pattern[j] {
            j += 1;
        }
        failure[i] = j;
    }

    failure
}

/// KMP pattern search
#[cfg(feature = "alloc")]
pub fn kmp_khoj(text: &str, pattern: &str) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }

    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();

    if pattern_bytes.len() > text_bytes.len() {
        return None;
    }

    let failure = kmp_asafalta(pattern_bytes);
    let mut j = 0;

    for i in 0..text_bytes.len() {
        while j > 0 && text_bytes[i] != pattern_bytes[j] {
            j = failure[j - 1];
        }
        if text_bytes[i] == pattern_bytes[j] {
            j += 1;
        }
        if j == pattern_bytes.len() {
            return Some(i - pattern_bytes.len() + 1);
        }
    }

    None
}

/// KMP find all occurrences
#[cfg(feature = "alloc")]
pub fn kmp_sab_khoj(text: &str, pattern: &str) -> Vec<usize> {
    let mut positions = Vec::new();

    if pattern.is_empty() {
        return positions;
    }

    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();

    if pattern_bytes.len() > text_bytes.len() {
        return positions;
    }

    let failure = kmp_asafalta(pattern_bytes);
    let mut j = 0;

    for i in 0..text_bytes.len() {
        while j > 0 && text_bytes[i] != pattern_bytes[j] {
            j = failure[j - 1];
        }
        if text_bytes[i] == pattern_bytes[j] {
            j += 1;
        }
        if j == pattern_bytes.len() {
            positions.push(i + 1 - pattern_bytes.len());
            // Reset j using failure function, but handle single-char pattern specially
            if pattern_bytes.len() > 1 {
                j = failure[j - 1];
            } else {
                j = 0;
            }
        }
    }

    positions
}

/// Wildcard pattern match (* and ?)
pub fn wildcard_milaan(text: &str, pattern: &str) -> bool {
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();

    let n = text_bytes.len();
    let m = pattern_bytes.len();

    let mut dp = [[false; 256]; 256];
    dp[0][0] = true;

    // Handle leading *s
    for j in 1..=m {
        if pattern_bytes[j - 1] == b'*' {
            dp[0][j] = dp[0][j - 1];
        }
    }

    for i in 1..=n.min(255) {
        for j in 1..=m.min(255) {
            if pattern_bytes[j - 1] == b'*' {
                dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
            } else if pattern_bytes[j - 1] == b'?' || pattern_bytes[j - 1] == text_bytes[i - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }

    dp[n.min(255)][m.min(255)]
}

/// Levenshtein edit distance
#[cfg(feature = "alloc")]
pub fn sampadan_doori(a: &str, b: &str) -> usize {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let m = a_bytes.len();
    let n = b_bytes.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr = vec![0; n + 1];

    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if a_bytes[i - 1] == b_bytes[j - 1] {
                0
            } else {
                1
            };
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        core::mem::swap(&mut prev, &mut curr);
    }

    prev[n]
}

/// Hamming distance (for equal length strings)
pub fn hamming_doori(a: &str, b: &str) -> Option<usize> {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    if a_bytes.len() != b_bytes.len() {
        return None;
    }

    let distance = a_bytes
        .iter()
        .zip(b_bytes.iter())
        .filter(|(x, y)| x != y)
        .count();

    Some(distance)
}

/// Longest common subsequence length
#[cfg(feature = "alloc")]
pub fn dirgh_samaan_upkram(a: &str, b: &str) -> usize {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let m = a_bytes.len();
    let n = b_bytes.len();

    let mut prev = vec![0usize; n + 1];
    let mut curr = vec![0usize; n + 1];

    for i in 1..=m {
        for j in 1..=n {
            if a_bytes[i - 1] == b_bytes[j - 1] {
                curr[j] = prev[j - 1] + 1;
            } else {
                curr[j] = prev[j].max(curr[j - 1]);
            }
        }
        core::mem::swap(&mut prev, &mut curr);
        curr.fill(0);
    }

    prev[n]
}

/// Longest common prefix
pub fn dirgh_samaan_upasarga<'a>(a: &'a str, b: &str) -> &'a str {
    let len = a.bytes().zip(b.bytes()).take_while(|(x, y)| x == y).count();

    &a[..len]
}

/// Longest common suffix
pub fn dirgh_samaan_pratyay<'a>(a: &'a str, b: &str) -> &'a str {
    let len = a
        .bytes()
        .rev()
        .zip(b.bytes().rev())
        .take_while(|(x, y)| x == y)
        .count();

    &a[a.len() - len..]
}

/// Similarity ratio (0.0 to 1.0)
#[cfg(feature = "alloc")]
pub fn samanta_anupaat(a: &str, b: &str) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }

    let max_len = a.len().max(b.len());
    let distance = sampadan_doori(a, b);

    1.0 - (distance as f64 / max_len as f64)
}

/// Check if starts with prefix
pub fn shuru_hai(text: &str, prefix: &str) -> bool {
    text.len() >= prefix.len() && text.as_bytes()[..prefix.len()] == *prefix.as_bytes()
}

/// Check if ends with suffix
pub fn ant_hai(text: &str, suffix: &str) -> bool {
    text.len() >= suffix.len() && text.as_bytes()[text.len() - suffix.len()..] == *suffix.as_bytes()
}

/// Check if contains substring
pub fn shamil_hai(text: &str, pattern: &str) -> bool {
    saral_khoj(text, pattern).is_some()
}

/// Count occurrences of pattern
#[cfg(feature = "alloc")]
pub fn ginti(text: &str, pattern: &str) -> usize {
    sab_khoj(text, pattern).len()
}

/// Replace first occurrence
#[cfg(feature = "alloc")]
pub fn pratham_badlo(text: &str, from: &str, to: &str) -> String {
    if let Some(pos) = saral_khoj(text, from) {
        let mut result = String::with_capacity(text.len() + to.len() - from.len());
        result.push_str(&text[..pos]);
        result.push_str(to);
        result.push_str(&text[pos + from.len()..]);
        result
    } else {
        text.to_string()
    }
}

/// Replace all occurrences
#[cfg(feature = "alloc")]
pub fn sab_badlo(text: &str, from: &str, to: &str) -> String {
    if from.is_empty() {
        return text.to_string();
    }

    let positions = sab_khoj(text, from);
    if positions.is_empty() {
        return text.to_string();
    }

    let mut result =
        String::with_capacity(text.len() + positions.len() * (to.len().saturating_sub(from.len())));

    let mut last_end = 0;
    for pos in positions {
        result.push_str(&text[last_end..pos]);
        result.push_str(to);
        last_end = pos + from.len();
    }
    result.push_str(&text[last_end..]);

    result
}

/// Split by pattern
#[cfg(feature = "alloc")]
pub fn vibhajit<'a>(text: &'a str, delimiter: &str) -> Vec<&'a str> {
    if delimiter.is_empty() {
        return vec![text];
    }

    let mut parts = Vec::new();
    let positions = sab_khoj(text, delimiter);

    let mut last_end = 0;
    for pos in positions {
        parts.push(&text[last_end..pos]);
        last_end = pos + delimiter.len();
    }
    parts.push(&text[last_end..]);

    parts
}

/// Join parts with delimiter
#[cfg(feature = "alloc")]
pub fn jodo(parts: &[&str], delimiter: &str) -> String {
    if parts.is_empty() {
        return String::new();
    }

    let total_len: usize = parts.iter().map(|s| s.len()).sum();
    let delim_len = delimiter.len() * (parts.len() - 1);

    let mut result = String::with_capacity(total_len + delim_len);
    result.push_str(parts[0]);

    for part in &parts[1..] {
        result.push_str(delimiter);
        result.push_str(part);
    }

    result
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_search() {
        assert_eq!(saral_khoj("hello world", "world"), Some(6));
        assert_eq!(saral_khoj("hello world", "xyz"), None);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_kmp_search() {
        assert_eq!(kmp_khoj("hello world", "world"), Some(6));
        assert_eq!(kmp_sab_khoj("aaaa", "aa"), vec![0, 1, 2]);
    }

    #[test]
    fn test_wildcard() {
        assert!(wildcard_milaan("hello", "h*o"));
        assert!(wildcard_milaan("hello", "h?llo"));
        assert!(!wildcard_milaan("hello", "x*"));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_edit_distance() {
        assert_eq!(sampadan_doori("kitten", "sitting"), 3);
        assert_eq!(sampadan_doori("", "abc"), 3);
    }

    #[test]
    fn test_hamming() {
        assert_eq!(hamming_doori("karolin", "kathrin"), Some(3));
        assert_eq!(hamming_doori("ab", "abc"), None);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_lcs() {
        assert_eq!(dirgh_samaan_upkram("ABCDGH", "AEDFHR"), 3);
    }

    #[test]
    fn test_prefix_suffix() {
        assert!(shuru_hai("hello world", "hello"));
        assert!(ant_hai("hello world", "world"));
        assert_eq!(dirgh_samaan_upasarga("hello", "help"), "hel");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_replace() {
        assert_eq!(sab_badlo("hello hello", "hello", "hi"), "hi hi");
    }
}
