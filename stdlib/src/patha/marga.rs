//! # Marga - Path Operations (मार्ग)
//!
//! Core path manipulation functions.
//!
//! > **"मार्गो धर्मस्य साधनम्"**
//! > *"The path is the means of righteousness"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::borrow::ToOwned;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// CONSTANTS
// ============================================================================

/// Unix path separator
pub const UNIX_VIBHAJAKA: char = '/';

/// Windows path separator
pub const WINDOWS_VIBHAJAKA: char = '\\';

/// Current platform separator
#[cfg(target_os = "windows")]
pub const VIBHAJAKA: char = WINDOWS_VIBHAJAKA;
#[cfg(not(target_os = "windows"))]
pub const VIBHAJAKA: char = UNIX_VIBHAJAKA;

// ============================================================================
// PATH COMPONENT OPERATIONS
// ============================================================================

/// Get file name from path (नाम)
#[cfg(feature = "alloc")]
pub fn nama(path: &str) -> Option<&str> {
    if path.is_empty() {
        return None;
    }

    let path = path.trim_end_matches(|c| c == '/' || c == '\\');
    if path.is_empty() {
        return None;
    }

    let last_sep = path.rfind(|c| c == '/' || c == '\\');
    match last_sep {
        Some(pos) => {
            let name = &path[pos + 1..];
            if name.is_empty() {
                None
            } else {
                Some(name)
            }
        }
        None => Some(path),
    }
}

/// Get file stem (name without extension) (मूल)
#[cfg(feature = "alloc")]
pub fn mula(path: &str) -> Option<&str> {
    let name = nama(path)?;

    // Handle dotfiles
    if name.starts_with('.') {
        if let Some(pos) = name[1..].rfind('.') {
            return Some(&name[..pos + 1]);
        }
        return Some(name);
    }

    match name.rfind('.') {
        Some(0) => Some(name),
        Some(pos) => Some(&name[..pos]),
        None => Some(name),
    }
}

/// Get file extension (विस्तार)
#[cfg(feature = "alloc")]
pub fn vistar(path: &str) -> Option<&str> {
    let name = nama(path)?;

    // Handle dotfiles
    if name.starts_with('.') {
        if let Some(pos) = name[1..].rfind('.') {
            let ext = &name[pos + 2..];
            return if ext.is_empty() { None } else { Some(ext) };
        }
        return None;
    }

    match name.rfind('.') {
        Some(0) => None,
        Some(pos) if pos < name.len() - 1 => Some(&name[pos + 1..]),
        _ => None,
    }
}

/// Get parent directory (janaka)
#[cfg(feature = "alloc")]
pub fn janaka(path: &str) -> Option<&str> {
    if path.is_empty() {
        return None;
    }

    let path = path.trim_end_matches(|c| c == '/' || c == '\\');
    if path.is_empty() {
        return None;
    }

    let last_sep = path.rfind(|c| c == '/' || c == '\\');
    match last_sep {
        Some(0) => Some(&path[..1]),
        Some(pos) => Some(&path[..pos]),
        None => None,
    }
}

/// Get path components (घटक)
#[cfg(feature = "alloc")]
pub fn ghataka(path: &str) -> Vec<&str> {
    path.split(|c| c == '/' || c == '\\')
        .filter(|s| !s.is_empty())
        .collect()
}

// ============================================================================
// PATH MANIPULATION
// ============================================================================

/// Join paths (संयोजन)
#[cfg(feature = "alloc")]
pub fn samyojana(base: &str, path: &str) -> String {
    if path.is_empty() {
        return base.to_owned();
    }

    if base.is_empty() {
        return path.to_owned();
    }

    // Check if path is absolute
    if path.starts_with('/') || path.starts_with('\\') {
        return path.to_owned();
    }

    #[cfg(target_os = "windows")]
    if path.len() >= 2 && path.chars().nth(1) == Some(':') {
        return path.to_owned();
    }

    let mut result = base.to_owned();

    // Ensure base ends with separator
    if !result.ends_with('/') && !result.ends_with('\\') {
        result.push(VIBHAJAKA);
    }

    result.push_str(path);
    result
}

/// Normalize path (samanya)
#[cfg(feature = "alloc")]
pub fn samanya(path: &str) -> String {
    let components: Vec<&str> = path
        .split(|c| c == '/' || c == '\\')
        .filter(|s| !s.is_empty() && *s != ".")
        .collect();

    let mut result: Vec<&str> = Vec::new();

    for component in components {
        if component == ".." {
            if !result.is_empty() && result.last() != Some(&"..") {
                result.pop();
            } else if !path.starts_with('/') && !path.starts_with('\\') {
                result.push("..");
            }
        } else {
            result.push(component);
        }
    }

    let joined = result.join(&VIBHAJAKA.to_string());

    // Preserve root
    if path.starts_with('/') || path.starts_with('\\') {
        let mut root = String::new();
        root.push(VIBHAJAKA);
        root.push_str(&joined);
        root
    } else if joined.is_empty() {
        ".".to_owned()
    } else {
        joined
    }
}

/// Get relative path (साधारण)
#[cfg(feature = "alloc")]
pub fn sadharana(from: &str, to: &str) -> String {
    let from_components: Vec<&str> = ghataka(from);
    let to_components: Vec<&str> = ghataka(to);

    // Find common prefix
    let mut common_length = 0;
    for (a, b) in from_components.iter().zip(to_components.iter()) {
        if a == b {
            common_length += 1;
        } else {
            break;
        }
    }

    let mut result = Vec::new();

    // Add ".." for each remaining component in from
    for _ in common_length..from_components.len() {
        result.push("..");
    }

    // Add remaining components from to
    for component in to_components.iter().skip(common_length) {
        result.push(*component);
    }

    if result.is_empty() {
        ".".to_owned()
    } else {
        result.join(&VIBHAJAKA.to_string())
    }
}

/// Check if path is absolute (निरपेक्ष)
pub fn nirapeksha_hai(path: &str) -> bool {
    if path.is_empty() {
        return false;
    }

    if path.starts_with('/') || path.starts_with('\\') {
        return true;
    }

    // Windows absolute path (e.g., C:\)
    #[cfg(target_os = "windows")]
    {
        if path.len() >= 2 {
            let chars: Vec<char> = path.chars().collect();
            if chars[0].is_ascii_alphabetic() && chars[1] == ':' {
                return true;
            }
        }
    }

    false
}

/// Check if path is relative (सापेक्ष)
pub fn sapeksha_hai(path: &str) -> bool {
    !nirapeksha_hai(path)
}

/// Replace extension (विस्तार बदलना)
#[cfg(feature = "alloc")]
pub fn vistar_badalna(path: &str, new_ext: &str) -> String {
    let parent = janaka(path).unwrap_or("");
    let stem = mula(path).unwrap_or("");

    let mut result = String::new();

    if !parent.is_empty() {
        result.push_str(parent);
        result.push(VIBHAJAKA);
    }

    result.push_str(stem);

    if !new_ext.is_empty() {
        result.push('.');
        result.push_str(new_ext);
    }

    result
}

/// Add extension (विस्तार जोड़ना)
#[cfg(feature = "alloc")]
pub fn vistar_jodna(path: &str, ext: &str) -> String {
    let mut result = path.to_owned();
    if !ext.is_empty() {
        result.push('.');
        result.push_str(ext);
    }
    result
}

// ============================================================================
// PATH CHECKS
// ============================================================================

/// Check if path has extension (विस्तार है)
pub fn vistar_hai(path: &str) -> bool {
    vistar(path).is_some()
}

/// Check if extension matches (विस्तार मिलान)
#[cfg(feature = "alloc")]
pub fn vistar_milaan(path: &str, ext: &str) -> bool {
    vistar(path)
        .map(|e| e.eq_ignore_ascii_case(ext))
        .unwrap_or(false)
}

/// Check if path starts with prefix (उपसर्ग से शुरू)
pub fn upasarga_se_shuru(path: &str, prefix: &str) -> bool {
    let path_components: Vec<&str> = path
        .split(|c| c == '/' || c == '\\')
        .filter(|s| !s.is_empty())
        .collect();
    let prefix_components: Vec<&str> = prefix
        .split(|c| c == '/' || c == '\\')
        .filter(|s| !s.is_empty())
        .collect();

    if prefix_components.len() > path_components.len() {
        return false;
    }

    path_components
        .iter()
        .zip(prefix_components.iter())
        .all(|(a, b)| a == b)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_nama() {
        assert_eq!(nama("/foo/bar/baz.txt"), Some("baz.txt"));
        assert_eq!(nama("/foo/bar/"), Some("bar"));
        assert_eq!(nama("baz.txt"), Some("baz.txt"));
        assert_eq!(nama("/"), None);
        assert_eq!(nama(""), None);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_mula() {
        assert_eq!(mula("baz.txt"), Some("baz"));
        assert_eq!(mula(".hidden"), Some(".hidden"));
        assert_eq!(mula(".hidden.txt"), Some(".hidden"));
        assert_eq!(mula("noext"), Some("noext"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_vistar() {
        assert_eq!(vistar("file.txt"), Some("txt"));
        assert_eq!(vistar("file.tar.gz"), Some("gz"));
        assert_eq!(vistar(".hidden"), None);
        assert_eq!(vistar(".hidden.txt"), Some("txt"));
        assert_eq!(vistar("noext"), None);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_janaka() {
        assert_eq!(janaka("/foo/bar"), Some("/foo"));
        assert_eq!(janaka("/foo"), Some("/"));
        assert_eq!(janaka("foo"), None);
        assert_eq!(janaka("/"), None);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_ghataka() {
        assert_eq!(ghataka("/foo/bar/baz"), vec!["foo", "bar", "baz"]);
        assert_eq!(ghataka("a/b/c"), vec!["a", "b", "c"]);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_samyojana() {
        // Note: samyojana uses std::path separator which varies by OS
        let result = samyojana("/foo", "bar");
        assert!(result == "/foo/bar" || result == "/foo\\bar");
        let result2 = samyojana("/foo/", "bar");
        assert!(result2 == "/foo/bar" || result2 == "/foo\\bar");
        assert_eq!(samyojana("", "bar"), "bar");
        let result3 = samyojana("/foo", "/bar");
        assert!(result3 == "/bar" || result3 == "\\bar"); // absolute path
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_samanya() {
        // Note: samanya uses std::path separator which varies by OS
        let result = samanya("/foo/./bar");
        assert!(result == "/foo/bar" || result == "\\foo\\bar");
        let result2 = samanya("/foo/bar/../baz");
        assert!(result2 == "/foo/baz" || result2 == "\\foo\\baz");
        let result3 = samanya("./foo/bar");
        assert!(result3 == "foo/bar" || result3 == "foo\\bar");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_sadharana() {
        // Note: Uses OS-specific separator
        let result1 = sadharana("/a/b/c", "/a/b/d");
        assert!(result1 == "../d" || result1 == "..\\d");
        let result2 = sadharana("/a/b", "/a/b/c/d");
        assert!(result2 == "c/d" || result2 == "c\\d");
    }

    #[test]
    fn test_nirapeksha_hai() {
        assert!(nirapeksha_hai("/foo"));
        assert!(!nirapeksha_hai("foo"));
        assert!(!nirapeksha_hai("./foo"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_vistar_badalna() {
        assert_eq!(vistar_badalna("file.txt", "md"), "file.md");
        assert_eq!(vistar_badalna("/path/file.txt", "rs"), "/path\\file.rs");
    }
}
