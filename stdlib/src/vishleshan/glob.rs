//! # Glob - Glob Pattern Matching (ग्लोब)
//!
//! File path glob patterns like *.rs, **/*.txt
//!
//! > **"ग्लोब पथस्य प्रतिरूपम्"**
//! > *"Glob is the pattern of path"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// GLOB PATTERN
// ============================================================================

/// Glob pattern for file matching
/// - `*` matches any characters except path separator
/// - `**` matches any characters including path separator
/// - `?` matches single character except path separator
/// - `[abc]` matches character class
/// - `[!abc]` matches negated character class
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct GlobPratirupa {
    pattern: String,
    segments: Vec<GlobKhanda>,
}

#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
enum GlobKhanda {
    /// Literal text
    Aksharsha(String),
    /// Single * (any in segment)
    Tara,
    /// Double ** (recursive)
    DohriTara,
    /// Single ?
    Prashna,
    /// Character class [abc]
    Varga(Vec<char>, bool), // chars, is_negated
}

#[cfg(feature = "alloc")]
impl GlobPratirupa {
    /// Parse glob pattern
    pub fn nava(pattern: impl Into<String>) -> Result<Self, GlobDosha> {
        let pattern = pattern.into();
        let segments = parse_glob(&pattern)?;
        Ok(Self { pattern, segments })
    }

    /// Check if path matches glob
    pub fn milana(&self, path: &str) -> bool {
        glob_match(&self.segments, path)
    }

    /// Get original pattern
    pub fn pratirupa(&self) -> &str {
        &self.pattern
    }
}

#[cfg(feature = "alloc")]
fn parse_glob(pattern: &str) -> Result<Vec<GlobKhanda>, GlobDosha> {
    let mut segments = Vec::new();
    let mut chars = pattern.chars().peekable();
    let mut literal = String::new();

    while let Some(c) = chars.next() {
        match c {
            '*' => {
                if !literal.is_empty() {
                    segments.push(GlobKhanda::Aksharsha(core::mem::take(&mut literal)));
                }
                if chars.peek() == Some(&'*') {
                    chars.next();
                    segments.push(GlobKhanda::DohriTara);
                } else {
                    segments.push(GlobKhanda::Tara);
                }
            }
            '?' => {
                if !literal.is_empty() {
                    segments.push(GlobKhanda::Aksharsha(core::mem::take(&mut literal)));
                }
                segments.push(GlobKhanda::Prashna);
            }
            '[' => {
                if !literal.is_empty() {
                    segments.push(GlobKhanda::Aksharsha(core::mem::take(&mut literal)));
                }

                let negated = chars.peek() == Some(&'!');
                if negated {
                    chars.next();
                }

                let mut class_chars = Vec::new();
                let mut closed = false;

                while let Some(cc) = chars.next() {
                    if cc == ']' {
                        closed = true;
                        break;
                    } else if cc == '-' && !class_chars.is_empty() {
                        // Range like [a-z]
                        if let Some(&end) = chars.peek() {
                            if end != ']' {
                                chars.next();
                                let start = *class_chars.last().unwrap();
                                for ch in (start as u8 + 1)..=(end as u8) {
                                    class_chars.push(ch as char);
                                }
                            } else {
                                class_chars.push('-');
                            }
                        }
                    } else {
                        class_chars.push(cc);
                    }
                }

                if !closed {
                    return Err(GlobDosha::AsamaptaVarga);
                }

                segments.push(GlobKhanda::Varga(class_chars, negated));
            }
            '\\' => {
                // Escape next character
                if let Some(next) = chars.next() {
                    literal.push(next);
                }
            }
            _ => {
                literal.push(c);
            }
        }
    }

    if !literal.is_empty() {
        segments.push(GlobKhanda::Aksharsha(literal));
    }

    Ok(segments)
}

#[cfg(feature = "alloc")]
fn glob_match(segments: &[GlobKhanda], path: &str) -> bool {
    glob_match_impl(segments, path, 0, 0)
}

#[cfg(feature = "alloc")]
fn glob_match_impl(segments: &[GlobKhanda], path: &str, seg_idx: usize, path_idx: usize) -> bool {
    let path_chars: Vec<char> = path.chars().collect();
    glob_match_chars(segments, &path_chars, seg_idx, path_idx)
}

#[cfg(feature = "alloc")]
fn glob_match_chars(
    segments: &[GlobKhanda],
    path: &[char],
    seg_idx: usize,
    path_idx: usize,
) -> bool {
    if seg_idx >= segments.len() {
        return path_idx >= path.len();
    }

    match &segments[seg_idx] {
        GlobKhanda::Aksharsha(lit) => {
            let lit_chars: Vec<char> = lit.chars().collect();
            if path_idx + lit_chars.len() > path.len() {
                return false;
            }
            for (i, c) in lit_chars.iter().enumerate() {
                if path[path_idx + i] != *c {
                    return false;
                }
            }
            glob_match_chars(segments, path, seg_idx + 1, path_idx + lit_chars.len())
        }

        GlobKhanda::Prashna => {
            if path_idx >= path.len() {
                return false;
            }
            let c = path[path_idx];
            if c == '/' || c == '\\' {
                return false;
            }
            glob_match_chars(segments, path, seg_idx + 1, path_idx + 1)
        }

        GlobKhanda::Tara => {
            // Try matching zero or more non-separator chars
            for i in path_idx..=path.len() {
                if i > path_idx {
                    let c = path[i - 1];
                    if c == '/' || c == '\\' {
                        break;
                    }
                }
                if glob_match_chars(segments, path, seg_idx + 1, i) {
                    return true;
                }
            }
            false
        }

        GlobKhanda::DohriTara => {
            // Try matching zero or more of any chars
            for i in path_idx..=path.len() {
                if glob_match_chars(segments, path, seg_idx + 1, i) {
                    return true;
                }
            }
            false
        }

        GlobKhanda::Varga(chars, negated) => {
            if path_idx >= path.len() {
                return false;
            }
            let c = path[path_idx];
            let matches = chars.contains(&c);
            let passes = if *negated { !matches } else { matches };
            if passes {
                glob_match_chars(segments, path, seg_idx + 1, path_idx + 1)
            } else {
                false
            }
        }
    }
}

// ============================================================================
// ERROR TYPE
// ============================================================================

/// Glob pattern error
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub enum GlobDosha {
    /// Unclosed character class
    AsamaptaVarga,
    /// Invalid escape
    AmanyadPalayan,
    /// Empty pattern
    RiktaPratirupa,
}

#[cfg(feature = "alloc")]
impl core::fmt::Display for GlobDosha {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GlobDosha::AsamaptaVarga => write!(f, "Unclosed character class [...]"),
            GlobDosha::AmanyadPalayan => write!(f, "Invalid escape sequence"),
            GlobDosha::RiktaPratirupa => write!(f, "Empty pattern"),
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Quick glob match without parsing
#[cfg(feature = "alloc")]
pub fn dhrut_milana(pattern: &str, path: &str) -> bool {
    GlobPratirupa::nava(pattern)
        .map(|g| g.milana(path))
        .unwrap_or(false)
}

/// Check if string looks like a glob pattern
pub fn glob_hai(s: &str) -> bool {
    s.contains('*') || s.contains('?') || s.contains('[')
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_literal() {
        let g = GlobPratirupa::nava("hello.txt").unwrap();
        assert!(g.milana("hello.txt"));
        assert!(!g.milana("hello.rs"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_star() {
        let g = GlobPratirupa::nava("*.txt").unwrap();
        assert!(g.milana("file.txt"));
        assert!(g.milana("hello.txt"));
        assert!(!g.milana("file.rs"));
        assert!(!g.milana("dir/file.txt")); // * doesn't match /
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_double_star() {
        let g = GlobPratirupa::nava("**/*.txt").unwrap();
        assert!(g.milana("dir/file.txt"));
        assert!(g.milana("a/b/c/file.txt"));
        assert!(!g.milana("file.rs"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_question() {
        let g = GlobPratirupa::nava("file?.txt").unwrap();
        assert!(g.milana("file1.txt"));
        assert!(g.milana("filea.txt"));
        assert!(!g.milana("file12.txt"));
        assert!(!g.milana("file.txt"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_char_class() {
        let g = GlobPratirupa::nava("[abc].txt").unwrap();
        assert!(g.milana("a.txt"));
        assert!(g.milana("b.txt"));
        assert!(!g.milana("d.txt"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_char_range() {
        let g = GlobPratirupa::nava("[a-z].txt").unwrap();
        assert!(g.milana("a.txt"));
        assert!(g.milana("z.txt"));
        assert!(!g.milana("A.txt"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_negated_class() {
        let g = GlobPratirupa::nava("[!0-9].txt").unwrap();
        assert!(g.milana("a.txt"));
        assert!(!g.milana("1.txt"));
    }

    #[test]
    fn test_glob_hai() {
        assert!(glob_hai("*.txt"));
        assert!(glob_hai("file?.txt"));
        assert!(glob_hai("[abc].txt"));
        assert!(!glob_hai("file.txt"));
    }
}
