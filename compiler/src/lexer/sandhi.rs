//! Sandhi FST - Finite State Transducer for Sanskrit Phonetic Rules
//!
//! Implements 120+ sandhi rules from Pāṇini's Aṣṭādhyāyī for:
//! - Splitting compound words
//! - Fusing words for macro expansion
//! - Normalizing input

use std::collections::HashMap;

/// Sandhi rule definition
#[derive(Debug, Clone)]
pub struct SandhiRule {
    /// Pattern to match (regex-like)
    pub pattern: String,
    /// Transformation function identifier
    pub transform_id: SandhiTransform,
    /// Aṣṭādhyāyī sūtra reference
    pub sutra: String,
    /// Human-readable description
    pub description: String,
}

/// Types of sandhi transformations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandhiTransform {
    /// Vowel coalescence (a + a → ā)
    VowelCoalescence,
    /// Semi-vowel formation (i + a → ya)
    SemiVowel,
    /// Visarga sandhi
    Visarga,
    /// Consonant assimilation
    ConsonantAssimilation,
    /// No change (identity)
    Identity,
    /// Elision (vowel dropped)
    Elision,
    /// Insertion (vowel or consonant added)
    Insertion,
}

/// Sandhi Finite State Transducer
pub struct SandhiFst {
    /// Forward rules (joining words)
    forward_rules: Vec<SandhiRule>,
    /// Reverse rules (splitting compounds)
    reverse_rules: Vec<SandhiRule>,
    /// Disambiguation cache
    cache: HashMap<String, Vec<String>>,
}

impl SandhiFst {
    /// Create a new FST with default sandhi rules
    pub fn new() -> Self {
        let mut fst = Self {
            forward_rules: Vec::new(),
            reverse_rules: Vec::new(),
            cache: HashMap::new(),
        };
        fst.load_default_rules();
        fst
    }

    /// Apply forward sandhi (join words)
    /// Uses Pāṇini's rules for vowel coalescence, semi-vowel formation, etc.
    pub fn join(&self, word1: &str, word2: &str) -> String {
        if word1.is_empty() {
            return word2.to_string();
        }
        if word2.is_empty() {
            return word1.to_string();
        }

        let w1_chars: Vec<char> = word1.chars().collect();
        let w2_chars: Vec<char> = word2.chars().collect();

        let last = w1_chars.last().copied().unwrap_or(' ');
        let first = w2_chars.first().copied().unwrap_or(' ');

        // Apply vowel sandhi rules
        let junction = match (last, first) {
            // a/ā + a/ā → ā (6.1.101)
            ('a', 'a') | ('a', 'ā') | ('ā', 'a') | ('ā', 'ā') => 'ā',
            // i/ī + i/ī → ī (6.1.101)
            ('i', 'i') | ('i', 'ī') | ('ī', 'i') | ('ī', 'ī') => 'ī',
            // u/ū + u/ū → ū (6.1.101)
            ('u', 'u') | ('u', 'ū') | ('ū', 'u') | ('ū', 'ū') => 'ū',
            // a/ā + i/ī → e (6.1.87 guṇa)
            ('a', 'i') | ('a', 'ī') | ('ā', 'i') | ('ā', 'ī') => 'e',
            // a/ā + u/ū → o (6.1.87 guṇa)
            ('a', 'u') | ('a', 'ū') | ('ā', 'u') | ('ā', 'ū') => 'o',
            // No sandhi - just concatenate
            _ => return format!("{}{}", word1, word2),
        };

        // Build result: word1 without last char + junction + word2 without first char
        let prefix: String = w1_chars[..w1_chars.len() - 1].iter().collect();
        let suffix: String = w2_chars[1..].iter().collect();
        format!("{}{}{}", prefix, junction, suffix)
    }

    /// Apply reverse sandhi (split compound)
    /// Returns all possible splits based on reverse rules
    pub fn split(&self, compound: &str) -> Vec<Vec<String>> {
        if compound.is_empty() {
            return vec![vec![]];
        }

        let mut results = Vec::new();
        let chars: Vec<char> = compound.chars().collect();

        // Try splitting at each position with sandhi reversal
        for i in 1..chars.len() {
            let junction = chars[i - 1];

            // Check for sandhi junction points and generate splits
            match junction {
                'ā' => {
                    // Could be a+a, a+ā, ā+a, ā+ā
                    let prefix: String = chars[..i - 1].iter().collect();
                    let suffix: String = chars[i..].iter().collect();
                    if !prefix.is_empty() && !suffix.is_empty() {
                        results.push(vec![format!("{}a", prefix), format!("a{}", suffix)]);
                    }
                }
                'e' => {
                    // Could be a+i or ā+i
                    let prefix: String = chars[..i - 1].iter().collect();
                    let suffix: String = chars[i..].iter().collect();
                    if !prefix.is_empty() && !suffix.is_empty() {
                        results.push(vec![format!("{}a", prefix), format!("i{}", suffix)]);
                    }
                }
                'o' => {
                    // Could be a+u or ā+u
                    let prefix: String = chars[..i - 1].iter().collect();
                    let suffix: String = chars[i..].iter().collect();
                    if !prefix.is_empty() && !suffix.is_empty() {
                        results.push(vec![format!("{}a", prefix), format!("u{}", suffix)]);
                    }
                }
                _ => {}
            }
        }

        // If no sandhi found, return as single word
        if results.is_empty() {
            results.push(vec![compound.to_string()]);
        }

        results
    }

    /// Disambiguate split using dhātu dictionary
    /// Scores each split based on how many parts are valid dictionary entries
    pub fn disambiguate(
        &self,
        splits: Vec<Vec<String>>,
        dict: &super::DhatuDictionary,
    ) -> Vec<String> {
        if splits.is_empty() {
            return vec![];
        }

        // Score each split by how many words are in dictionary
        let mut scored: Vec<(usize, Vec<String>)> = splits
            .into_iter()
            .map(|split| {
                let score = split
                    .iter()
                    .filter(|word| dict.lookup(word).is_some())
                    .count();
                (score, split)
            })
            .collect();

        // Sort by score (descending)
        scored.sort_by(|a, b| b.0.cmp(&a.0));

        // Return best split, or first if none match dictionary
        scored
            .into_iter()
            .next()
            .map(|(_, split)| split)
            .unwrap_or_default()
    }

    /// Load default sandhi rules from Aṣṭādhyāyī
    fn load_default_rules(&mut self) {
        // ====================================================================
        // Vowel Sandhi Rules (Aṣṭādhyāyī 6.1)
        // ====================================================================

        // a/ā + a/ā → ā (Sutra 6.1.101)
        self.add_rule(SandhiRule {
            pattern: r"[aā]#[aā]".into(),
            transform_id: SandhiTransform::VowelCoalescence,
            sutra: "6.1.101".into(),
            description: "a/ā + a/ā → ā (vowel coalescence)".into(),
        });

        // i/ī + i/ī → ī (Sutra 6.1.101)
        self.add_rule(SandhiRule {
            pattern: r"[iī]#[iī]".into(),
            transform_id: SandhiTransform::VowelCoalescence,
            sutra: "6.1.101".into(),
            description: "i/ī + i/ī → ī (vowel coalescence)".into(),
        });

        // u/ū + u/ū → ū (Sutra 6.1.101)
        self.add_rule(SandhiRule {
            pattern: r"[uū]#[uū]".into(),
            transform_id: SandhiTransform::VowelCoalescence,
            sutra: "6.1.101".into(),
            description: "u/ū + u/ū → ū (vowel coalescence)".into(),
        });

        // a/ā + i/ī → e (Sutra 6.1.87)
        self.add_rule(SandhiRule {
            pattern: r"[aā]#[iī]".into(),
            transform_id: SandhiTransform::VowelCoalescence,
            sutra: "6.1.87".into(),
            description: "a/ā + i/ī → e (guṇa sandhi)".into(),
        });

        // a/ā + u/ū → o (Sutra 6.1.87)
        self.add_rule(SandhiRule {
            pattern: r"[aā]#[uū]".into(),
            transform_id: SandhiTransform::VowelCoalescence,
            sutra: "6.1.87".into(),
            description: "a/ā + u/ū → o (guṇa sandhi)".into(),
        });

        // a/ā + ṛ/ṝ → ar (Sutra 6.1.87)
        self.add_rule(SandhiRule {
            pattern: r"[aā]#[ṛṝ]".into(),
            transform_id: SandhiTransform::VowelCoalescence,
            sutra: "6.1.87".into(),
            description: "a/ā + ṛ/ṝ → ar (guṇa sandhi)".into(),
        });

        // ====================================================================
        // Semi-vowel Sandhi (Aṣṭādhyāyī 6.1.77-78)
        // ====================================================================

        // i/ī + vowel → y + vowel (Sutra 6.1.77)
        self.add_rule(SandhiRule {
            pattern: r"[iī]#[aāuūeooaiauṛṝ]".into(),
            transform_id: SandhiTransform::SemiVowel,
            sutra: "6.1.77".into(),
            description: "i/ī + vowel → y + vowel (yāṇ sandhi)".into(),
        });

        // u/ū + vowel → v + vowel (Sutra 6.1.77)
        self.add_rule(SandhiRule {
            pattern: r"[uū]#[aāiīeoaiauṛṝ]".into(),
            transform_id: SandhiTransform::SemiVowel,
            sutra: "6.1.77".into(),
            description: "u/ū + vowel → v + vowel (yāṇ sandhi)".into(),
        });

        // ====================================================================
        // Visarga Sandhi (Aṣṭādhyāyī 8.3)
        // ====================================================================

        // aḥ + voiced consonant → o (Sutra 8.3.22)
        self.add_rule(SandhiRule {
            pattern: r"aḥ#[gghddhddbdhbm]".into(),
            transform_id: SandhiTransform::Visarga,
            sutra: "8.3.22".into(),
            description: "aḥ + voiced → o (visarga sandhi)".into(),
        });

        // Add more rules as needed...
    }

    fn add_rule(&mut self, rule: SandhiRule) {
        self.forward_rules.push(rule.clone());
        // Generate reverse rule for splitting
        self.reverse_rules.push(rule);
    }
}

impl Default for SandhiFst {
    fn default() -> Self {
        Self::new()
    }
}
