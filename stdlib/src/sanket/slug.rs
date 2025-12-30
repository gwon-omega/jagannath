//! # Slug - URL-Safe Slugs (स्लग)
//!
//! Generate URL-friendly slugs from text.
//!
//! > **"स्लग पठनीय पथ भवति"**
//! > *"A slug becomes a readable path"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// SLUG GENERATION
// ============================================================================

/// Generate slug from text
#[cfg(feature = "alloc")]
pub fn utpadana(text: &str) -> String {
    utpadana_vinyas(text, SlugVinyas::default())
}

/// Slug configuration
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct SlugVinyas {
    /// Separator character
    pub vibhajaka: char,
    /// Maximum length (0 = unlimited)
    pub adhikatam_lambai: usize,
    /// Convert to lowercase
    pub laghu_akshara: bool,
    /// Allow underscores
    pub rekha_adhah_manya: bool,
    /// Allow dots
    pub bindu_manya: bool,
}

#[cfg(feature = "alloc")]
impl Default for SlugVinyas {
    fn default() -> Self {
        Self {
            vibhajaka: '-',
            adhikatam_lambai: 0,
            laghu_akshara: true,
            rekha_adhah_manya: false,
            bindu_manya: false,
        }
    }
}

/// Generate slug with configuration
#[cfg(feature = "alloc")]
pub fn utpadana_vinyas(text: &str, vinyas: SlugVinyas) -> String {
    let text = if vinyas.laghu_akshara {
        text.to_lowercase()
    } else {
        text.to_string()
    };

    let text = transliterate(&text);

    let mut result = String::new();
    let mut prev_separator = true; // Start as true to skip leading separators

    for c in text.chars() {
        if c.is_ascii_alphanumeric() {
            result.push(c);
            prev_separator = false;
        } else if c == '_' && vinyas.rekha_adhah_manya {
            if !prev_separator {
                result.push('_');
                prev_separator = true;
            }
        } else if c == '.' && vinyas.bindu_manya {
            if !prev_separator {
                result.push('.');
                prev_separator = true;
            }
        } else {
            // Replace with separator
            if !prev_separator {
                result.push(vinyas.vibhajaka);
                prev_separator = true;
            }
        }
    }

    // Remove trailing separator
    while result.ends_with(vinyas.vibhajaka) || result.ends_with('_') || result.ends_with('.') {
        result.pop();
    }

    // Apply max length
    if vinyas.adhikatam_lambai > 0 && result.len() > vinyas.adhikatam_lambai {
        // Try to break at separator
        let mut truncated = &result[..vinyas.adhikatam_lambai];
        if let Some(last_sep) = truncated.rfind(vinyas.vibhajaka) {
            truncated = &truncated[..last_sep];
        }
        return truncated.to_string();
    }

    result
}

/// Transliterate accented characters to ASCII
#[cfg(feature = "alloc")]
fn transliterate(text: &str) -> String {
    let mut result = String::with_capacity(text.len());

    for c in text.chars() {
        let replacement = match c {
            // Latin Extended
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' | 'ą' => 'a',
            'æ' => {
                result.push_str("ae");
                continue;
            }
            'ç' | 'ć' | 'ĉ' | 'ċ' | 'č' => 'c',
            'ð' | 'ď' | 'đ' => 'd',
            'è' | 'é' | 'ê' | 'ë' | 'ē' | 'ĕ' | 'ė' | 'ę' | 'ě' => 'e',
            'ĝ' | 'ğ' | 'ġ' | 'ģ' => 'g',
            'ĥ' | 'ħ' => 'h',
            'ì' | 'í' | 'î' | 'ï' | 'ĩ' | 'ī' | 'ĭ' | 'į' | 'ı' => 'i',
            'ĳ' => {
                result.push_str("ij");
                continue;
            }
            'ĵ' => 'j',
            'ķ' | 'ĸ' => 'k',
            'ĺ' | 'ļ' | 'ľ' | 'ŀ' | 'ł' => 'l',
            'ñ' | 'ń' | 'ņ' | 'ň' | 'ŉ' | 'ŋ' => 'n',
            'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'ō' | 'ŏ' | 'ő' => 'o',
            'œ' => {
                result.push_str("oe");
                continue;
            }
            'ŕ' | 'ŗ' | 'ř' => 'r',
            'ś' | 'ŝ' | 'ş' | 'š' => 's',
            'ß' => {
                result.push_str("ss");
                continue;
            }
            'ţ' | 'ť' | 'ŧ' => 't',
            'þ' => {
                result.push_str("th");
                continue;
            }
            'ù' | 'ú' | 'û' | 'ü' | 'ũ' | 'ū' | 'ŭ' | 'ů' | 'ű' | 'ų' => 'u',
            'ŵ' => 'w',
            'ý' | 'ÿ' | 'ŷ' => 'y',
            'ź' | 'ż' | 'ž' => 'z',

            // Uppercase
            'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' | 'Ā' | 'Ă' | 'Ą' => 'a',
            'Æ' => {
                result.push_str("ae");
                continue;
            }
            'Ç' | 'Ć' | 'Ĉ' | 'Ċ' | 'Č' => 'c',
            'Ð' | 'Ď' | 'Đ' => 'd',
            'È' | 'É' | 'Ê' | 'Ë' | 'Ē' | 'Ĕ' | 'Ė' | 'Ę' | 'Ě' => 'e',
            'Ĝ' | 'Ğ' | 'Ġ' | 'Ģ' => 'g',
            'Ĥ' | 'Ħ' => 'h',
            'Ì' | 'Í' | 'Î' | 'Ï' | 'Ĩ' | 'Ī' | 'Ĭ' | 'Į' => 'i',
            'Ĳ' => {
                result.push_str("ij");
                continue;
            }
            'Ĵ' => 'j',
            'Ķ' => 'k',
            'Ĺ' | 'Ļ' | 'Ľ' | 'Ŀ' | 'Ł' => 'l',
            'Ñ' | 'Ń' | 'Ņ' | 'Ň' | 'Ŋ' => 'n',
            'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' | 'Ō' | 'Ŏ' | 'Ő' => 'o',
            'Œ' => {
                result.push_str("oe");
                continue;
            }
            'Ŕ' | 'Ŗ' | 'Ř' => 'r',
            'Ś' | 'Ŝ' | 'Ş' | 'Š' => 's',
            'Ţ' | 'Ť' | 'Ŧ' => 't',
            'Þ' => {
                result.push_str("th");
                continue;
            }
            'Ù' | 'Ú' | 'Û' | 'Ü' | 'Ũ' | 'Ū' | 'Ŭ' | 'Ů' | 'Ű' | 'Ų' => 'u',
            'Ŵ' => 'w',
            'Ý' | 'Ŷ' | 'Ÿ' => 'y',
            'Ź' | 'Ż' | 'Ž' => 'z',

            _ => c,
        };
        result.push(replacement);
    }

    result
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Generate slug with underscore separator
#[cfg(feature = "alloc")]
pub fn rekha_adhah(text: &str) -> String {
    utpadana_vinyas(
        text,
        SlugVinyas {
            vibhajaka: '_',
            rekha_adhah_manya: true,
            ..Default::default()
        },
    )
}

/// Generate slug preserving case
#[cfg(feature = "alloc")]
pub fn sanskaran_rakha(text: &str) -> String {
    utpadana_vinyas(
        text,
        SlugVinyas {
            laghu_akshara: false,
            ..Default::default()
        },
    )
}

/// Generate slug with max length
#[cfg(feature = "alloc")]
pub fn simita(text: &str, lambai: usize) -> String {
    utpadana_vinyas(
        text,
        SlugVinyas {
            adhikatam_lambai: lambai,
            ..Default::default()
        },
    )
}

/// Check if string is valid slug
pub fn manya(s: &str) -> bool {
    manya_vibhajaka(s, '-')
}

/// Check if string is valid slug with given separator
pub fn manya_vibhajaka(s: &str, sep: char) -> bool {
    if s.is_empty() {
        return false;
    }

    // Check first and last character
    if s.starts_with(sep) || s.ends_with(sep) {
        return false;
    }

    // Check all characters
    let mut prev_sep = false;
    for c in s.chars() {
        if c == sep {
            if prev_sep {
                return false; // No consecutive separators
            }
            prev_sep = true;
        } else if c.is_ascii_alphanumeric() {
            prev_sep = false;
        } else {
            return false; // Invalid character
        }
    }

    true
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_basic_slug() {
        assert_eq!(utpadana("Hello World"), "hello-world");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_multiple_spaces() {
        assert_eq!(utpadana("Hello   World"), "hello-world");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_special_chars() {
        assert_eq!(utpadana("Hello, World!"), "hello-world");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_accents() {
        assert_eq!(utpadana("Café résumé"), "cafe-resume");
        assert_eq!(utpadana("naïve"), "naive");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_german() {
        assert_eq!(utpadana("Größe"), "grosse");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_underscore() {
        assert_eq!(rekha_adhah("Hello World"), "hello_world");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_preserve_case() {
        assert_eq!(sanskaran_rakha("Hello World"), "Hello-World");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_max_length() {
        let slug = simita("This is a very long title that should be truncated", 20);
        assert!(slug.len() <= 20);
        assert!(!slug.ends_with('-'));
    }

    #[test]
    fn test_valid() {
        assert!(manya("hello-world"));
        assert!(manya("hello123"));
        assert!(!manya("-hello"));
        assert!(!manya("hello-"));
        assert!(!manya("hello--world"));
        assert!(!manya("hello world"));
    }
}
