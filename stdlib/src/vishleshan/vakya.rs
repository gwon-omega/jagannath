//! # Vakya - Tokenization (वाक्य)
//!
//! Text tokenization and lexical analysis utilities.
//!
//! > **"वाक्यं पदानां समूहः"**
//! > *"A sentence is a collection of words"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// TOKEN TYPES
// ============================================================================

/// Token kind
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PadaPrakara {
    /// Word (alphanumeric)
    Shabda,
    /// Number (integer or float)
    Sankhya,
    /// Whitespace
    Shvetasthana,
    /// Punctuation
    Viram,
    /// Operator (+, -, *, etc.)
    Paricharaka,
    /// String literal
    Sutra,
    /// Single character
    Akshara,
    /// End of input
    Anta,
}

/// A token with position
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct Pada {
    /// Token kind
    pub prakara: PadaPrakara,
    /// Token text
    pub patha: String,
    /// Start position
    pub arambha: usize,
    /// End position
    pub anta: usize,
}

// ============================================================================
// SIMPLE TOKENIZER
// ============================================================================

/// Simple whitespace-based tokenizer
#[cfg(feature = "alloc")]
pub fn shvetasthana_vibhajana(text: &str) -> Vec<Pada> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut start = 0;
    let mut in_token = false;

    for (idx, c) in text.char_indices() {
        if c.is_whitespace() {
            if in_token {
                tokens.push(Pada {
                    prakara: classify_token(&current),
                    patha: core::mem::take(&mut current),
                    arambha: start,
                    anta: idx,
                });
                in_token = false;
            }
        } else {
            if !in_token {
                start = idx;
                in_token = true;
            }
            current.push(c);
        }
    }

    if in_token {
        tokens.push(Pada {
            prakara: classify_token(&current),
            patha: current,
            arambha: start,
            anta: text.len(),
        });
    }

    tokens
}

#[cfg(feature = "alloc")]
fn classify_token(s: &str) -> PadaPrakara {
    if s.chars()
        .all(|c| c.is_ascii_digit() || c == '.' || c == '-')
    {
        PadaPrakara::Sankhya
    } else if s.chars().all(|c| c.is_alphanumeric() || c == '_') {
        PadaPrakara::Shabda
    } else if s.len() == 1 && is_operator(s.chars().next().unwrap()) {
        PadaPrakara::Paricharaka
    } else if s.len() == 1 && is_punctuation(s.chars().next().unwrap()) {
        PadaPrakara::Viram
    } else {
        PadaPrakara::Shabda
    }
}

fn is_operator(c: char) -> bool {
    matches!(
        c,
        '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | '&' | '|' | '^' | '!' | '~'
    )
}

fn is_punctuation(c: char) -> bool {
    matches!(c, ',' | '.' | ';' | ':' | '(' | ')' | '[' | ']' | '{' | '}')
}

// ============================================================================
// ADVANCED TOKENIZER
// ============================================================================

/// Tokenizer configuration
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct PadakaraVinyas {
    /// Include whitespace tokens
    pub shvetasthana_shamil: bool,
    /// String quote character
    pub sutra_uddharan: char,
    /// Comment start
    pub tippani_arambha: Option<String>,
    /// Line comment
    pub pankti_tippani: Option<String>,
}

#[cfg(feature = "alloc")]
impl Default for PadakaraVinyas {
    fn default() -> Self {
        Self {
            shvetasthana_shamil: false,
            sutra_uddharan: '"',
            tippani_arambha: Some("/*".to_string()),
            pankti_tippani: Some("//".to_string()),
        }
    }
}

/// Advanced tokenizer
#[cfg(feature = "alloc")]
pub struct Padakara {
    vinyas: PadakaraVinyas,
}

#[cfg(feature = "alloc")]
impl Padakara {
    /// Create new tokenizer
    pub fn nava() -> Self {
        Self {
            vinyas: PadakaraVinyas::default(),
        }
    }

    /// Create with config
    pub fn nava_vinyas(vinyas: PadakaraVinyas) -> Self {
        Self { vinyas }
    }

    /// Tokenize text
    pub fn padana(&self, text: &str) -> Vec<Pada> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let mut idx = 0;

        while idx < chars.len() {
            let c = chars[idx];

            // Whitespace
            if c.is_whitespace() {
                let start = idx;
                while idx < chars.len() && chars[idx].is_whitespace() {
                    idx += 1;
                }
                if self.vinyas.shvetasthana_shamil {
                    tokens.push(Pada {
                        prakara: PadaPrakara::Shvetasthana,
                        patha: chars[start..idx].iter().collect(),
                        arambha: start,
                        anta: idx,
                    });
                }
                continue;
            }

            // String literal
            if c == self.vinyas.sutra_uddharan {
                let start = idx;
                idx += 1;
                let mut value = String::new();
                while idx < chars.len() && chars[idx] != self.vinyas.sutra_uddharan {
                    if chars[idx] == '\\' && idx + 1 < chars.len() {
                        idx += 1;
                        match chars[idx] {
                            'n' => value.push('\n'),
                            't' => value.push('\t'),
                            'r' => value.push('\r'),
                            c => value.push(c),
                        }
                    } else {
                        value.push(chars[idx]);
                    }
                    idx += 1;
                }
                if idx < chars.len() {
                    idx += 1; // consume closing quote
                }
                tokens.push(Pada {
                    prakara: PadaPrakara::Sutra,
                    patha: value,
                    arambha: start,
                    anta: idx,
                });
                continue;
            }

            // Number
            if c.is_ascii_digit()
                || (c == '-' && idx + 1 < chars.len() && chars[idx + 1].is_ascii_digit())
            {
                let start = idx;
                if c == '-' {
                    idx += 1;
                }
                while idx < chars.len() && (chars[idx].is_ascii_digit() || chars[idx] == '.') {
                    idx += 1;
                }
                tokens.push(Pada {
                    prakara: PadaPrakara::Sankhya,
                    patha: chars[start..idx].iter().collect(),
                    arambha: start,
                    anta: idx,
                });
                continue;
            }

            // Word
            if c.is_alphabetic() || c == '_' {
                let start = idx;
                while idx < chars.len() && (chars[idx].is_alphanumeric() || chars[idx] == '_') {
                    idx += 1;
                }
                tokens.push(Pada {
                    prakara: PadaPrakara::Shabda,
                    patha: chars[start..idx].iter().collect(),
                    arambha: start,
                    anta: idx,
                });
                continue;
            }

            // Operator
            if is_operator(c) {
                tokens.push(Pada {
                    prakara: PadaPrakara::Paricharaka,
                    patha: c.to_string(),
                    arambha: idx,
                    anta: idx + 1,
                });
                idx += 1;
                continue;
            }

            // Punctuation
            if is_punctuation(c) {
                tokens.push(Pada {
                    prakara: PadaPrakara::Viram,
                    patha: c.to_string(),
                    arambha: idx,
                    anta: idx + 1,
                });
                idx += 1;
                continue;
            }

            // Single char
            tokens.push(Pada {
                prakara: PadaPrakara::Akshara,
                patha: c.to_string(),
                arambha: idx,
                anta: idx + 1,
            });
            idx += 1;
        }

        tokens
    }
}

// ============================================================================
// CSV TOKENIZER
// ============================================================================

/// Parse CSV line
#[cfg(feature = "alloc")]
pub fn csv_pankti(line: &str) -> Vec<String> {
    csv_pankti_vibhajaka(line, ',')
}

/// Parse CSV line with custom delimiter
#[cfg(feature = "alloc")]
pub fn csv_pankti_vibhajaka(line: &str, delimiter: char) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let chars: Vec<char> = line.chars().collect();
    let mut idx = 0;

    while idx < chars.len() {
        let c = chars[idx];

        if in_quotes {
            if c == '"' {
                if idx + 1 < chars.len() && chars[idx + 1] == '"' {
                    // Escaped quote
                    current.push('"');
                    idx += 2;
                    continue;
                } else {
                    // End of quoted field
                    in_quotes = false;
                }
            } else {
                current.push(c);
            }
        } else {
            if c == '"' {
                in_quotes = true;
            } else if c == delimiter {
                fields.push(core::mem::take(&mut current));
            } else {
                current.push(c);
            }
        }

        idx += 1;
    }

    fields.push(current);
    fields
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_simple_tokenize() {
        let tokens = shvetasthana_vibhajana("hello world 123");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].patha, "hello");
        assert_eq!(tokens[1].patha, "world");
        assert_eq!(tokens[2].patha, "123");
        assert_eq!(tokens[2].prakara, PadaPrakara::Sankhya);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_advanced_tokenize() {
        let tokenizer = Padakara::nava();
        let tokens = tokenizer.padana("x = 42 + \"hello\"");

        assert_eq!(tokens[0].patha, "x");
        assert_eq!(tokens[0].prakara, PadaPrakara::Shabda);

        assert_eq!(tokens[1].patha, "=");
        assert_eq!(tokens[1].prakara, PadaPrakara::Paricharaka);

        assert_eq!(tokens[2].patha, "42");
        assert_eq!(tokens[2].prakara, PadaPrakara::Sankhya);

        assert_eq!(tokens[3].patha, "+");
        assert_eq!(tokens[3].prakara, PadaPrakara::Paricharaka);

        assert_eq!(tokens[4].patha, "hello");
        assert_eq!(tokens[4].prakara, PadaPrakara::Sutra);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_csv() {
        let fields = csv_pankti("a,b,c");
        assert_eq!(fields, vec!["a", "b", "c"]);

        let fields = csv_pankti("\"hello, world\",b,c");
        assert_eq!(fields, vec!["hello, world", "b", "c"]);

        let fields = csv_pankti("a,\"with \"\"quotes\"\"\",c");
        assert_eq!(fields, vec!["a", "with \"quotes\"", "c"]);
    }
}
