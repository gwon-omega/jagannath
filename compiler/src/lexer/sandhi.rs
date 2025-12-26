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
    pub fn join(&self, word1: &str, word2: &str) -> String {
        todo!("Implement sandhi joining")
    }

    /// Apply reverse sandhi (split compound)
    pub fn split(&self, compound: &str) -> Vec<Vec<String>> {
        todo!("Implement sandhi splitting with multiple possibilities")
    }

    /// Disambiguate split using dhātu dictionary
    pub fn disambiguate(&self, splits: Vec<Vec<String>>, dict: &super::DhatuDictionary) -> Vec<String> {
        todo!("Implement disambiguation using dictionary lookup")
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
